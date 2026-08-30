#!/usr/bin/env python3
"""report.py — metrics aggregation + oracle (Phase S).

Per-case mode (workflow-invoked):
  report.py --case <case.yml> --run-dir <dir>
  Validates: final outcome, attempts used, classification sequence, heal routing,
  R-gate behavior (clean accepts above θ, failures detected below θ).
  Writes report.json. EXIT 0 = all expectations met, 1 = any mismatch (GWT routes).

Suite mode (manual):
  report.py --suite <results/sh-v1 dir>
  Aggregates TSR/FDA/RSR across case subdirs.
"""
import argparse
import glob
import json
import sys
import os

import yaml

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sh_lib


def classify_events(trace):
    """Pull ordered semantic events from trace."""
    return [t for t in trace if t["event"] in
            ("attempt", "classify", "accept", "final_fail")
            or t["event"].startswith("heal_")]


def expected_semantic_path(case):
    """Translate case expected.path into comparable event list.
    E.g. 'heal_f1_1' -> event 'heal_corrective_prompt' w/ round=1;
         'classify_1' -> 'classify' attempt=1; 'attempt_2' -> 'attempt' attempt=2.
    """
    out = []
    for tok in case["expected"]["path"]:
        if tok.startswith("attempt_"):
            out.append(("attempt", int(tok.split("_")[1])))
        elif tok.startswith("classify_"):
            out.append(("classify", int(tok.split("_")[1])))
        elif tok.startswith("heal_"):
            _, strat, rnd = tok.split("_")
            strat_map = {"f1": "corrective_prompt", "f2": "tool_reselect", "f34": "replan"}
            out.append(("heal_" + strat_map[strat], int(rnd)))
        elif tok == "accept":
            out.append(("accept", None))
        elif tok == "final_fail":
            out.append(("final_fail", None))
        elif tok == "report":
            continue
        else:
            raise ValueError("unknown path token %r" % tok)
    return out


def actual_semantic_path(trace):
    out = []
    for t in classify_events(trace):
        if t["event"] in ("accept", "final_fail"):
            out.append((t["event"], None))
        elif t["event"] == "attempt":
            out.append(("attempt", t["attempt"]))
        elif t["event"] == "classify":
            out.append(("classify", t["attempt"]))
        else:  # heal_*
            out.append((t["event"], t["round"]))
    return out


def run_case(case_path, run_dir):
    with open(case_path) as f:
        case = yaml.safe_load(f)

    trace = sh_lib.read_trace(run_dir)
    problems = []

    # outcome check (workflow vocabulary accept/final_fail vs case pass/fail)
    outcome_path = os.path.join(run_dir, "outcome.json")
    if not os.path.exists(outcome_path):
        return {"case_id": case["case_id"], "error": "no outcome.json"}, ["missing outcome.json"]
    outcome = sh_lib.read_json(outcome_path)
    vocab = {"accept": "pass", "final_fail": "fail"}
    outcome_norm = vocab.get(outcome["outcome"], outcome["outcome"])
    if vocab.get(outcome["outcome"], outcome["outcome"]) != case["expected"]["final"]:
        problems.append("outcome mismatch: got %s want %s" %
                        (outcome["outcome"], case["expected"]["final"]))

    # attempts used
    attempts_used = outcome["at_attempt"]
    want_attempts = case["expected"]["attempts_to_success"]
    if want_attempts is not None and attempts_used != want_attempts:
        problems.append("attempts mismatch: got %d want %d" % (attempts_used, want_attempts))
    if want_attempts is None and attempts_used != sh_lib.MAX_ATTEMPTS:
        problems.append("fail path must exhaust MAX_ATTEMPTS (%d), got %d" %
                        (sh_lib.MAX_ATTEMPTS, attempts_used))

    # classification sequence vs ground truth injection
    cls_events = [t for t in trace if t["event"] == "classify"]
    fail_set = set(case["failure"]["fail_attempts"])
    fda_hits, fda_total = 0, 0
    for t in cls_events:
        n = t["attempt"]
        inj = case["failure"]["class"] if n in fail_set else "clean"
        got = t["classification"]
        if n in fail_set or got != "clean":
            fda_total += 1
            if got == inj:
                fda_hits += 1
            else:
                problems.append("classify@%d: got %s want %s" % (n, got, inj))

    # semantic path check
    want_path = expected_semantic_path(case)
    got_path = actual_semantic_path(trace)
    if got_path != want_path:
        problems.append("path mismatch:\n  want %s\n  got  %s" % (want_path, got_path))

    # R-gate check (H3): clean accepts must be R>=θ; every healed attempt R<θ
    for t in cls_events:
        if t["classification"] == "clean" and t["R"] < sh_lib.THETA:
            problems.append("clean classify with R<θ at attempt %d" % t["attempt"])
        if t["classification"] != "clean" and t["R"] >= sh_lib.THETA:
            problems.append("failure classify with R>=θ at attempt %d" % t["attempt"])

    recovered = outcome_norm == "pass" and case["failure"]["class"] != "none"
    report = {
        "case_id": case["case_id"],
        "outcome": outcome["outcome"],
        "attempts_used": attempts_used,
        "R_per_attempt": {str(t["attempt"]): t["R"] for t in cls_events},
        "classification_per_attempt": {str(t["attempt"]): t["classification"] for t in cls_events},
        "injected_class": case["failure"]["class"],
        "fda": {"hits": fda_hits, "total": fda_total},
        "recovered": recovered,
        "oracle_pass": len(problems) == 0,
        "problems": problems,
    }
    sh_lib.write_json(os.path.join(run_dir, "report.json"), report)
    sh_lib.append_trace(run_dir, "report", oracle_pass=report["oracle_pass"])
    return report, problems


def run_suite(suite_dir):
    reports = []
    for rj in sorted(glob.glob(os.path.join(suite_dir, "*", "report.json"))):
        reports.append(sh_lib.read_json(rj))
    if not reports:
        print("no report.json files under %s" % suite_dir)
        return 1

    n = len(reports)
    vocab = {"accept": "pass", "final_fail": "fail"}
    tsr = sum(1 for r in reports
              if vocab.get(r["outcome"], r["outcome"]) == "pass") / n
    fda_t = sum(r["fda"]["total"] for r in reports)
    fda_h = sum(r["fda"]["hits"] for r in reports)
    detected_cases = [r for r in reports if r["injected_class"] != "none"]
    recovered = sum(1 for r in detected_cases if r["recovered"])

    suite = {
        "cases": n,
        "TSR": round(tsr, 4),
        "FDA": {"hits": fda_h, "total": fda_t,
                "value": round(fda_h / fda_t, 4) if fda_t else None},
        "RSR": {"recovered": recovered, "detected_cases": len(detected_cases),
                "value": round(recovered / len(detected_cases), 4) if detected_cases else None},
        "oracle_all_pass": all(r["oracle_pass"] for r in reports),
        "per_case": [{k: r[k] for k in ("case_id", "outcome", "attempts_used",
                                        "oracle_pass")} for r in reports],
    }
    sh_lib.write_json(os.path.join(suite_dir, "suite-report.json"), suite)
    print(json.dumps(suite, indent=2))
    return 0 if suite["oracle_all_pass"] else 1


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case")
    ap.add_argument("--run-dir")
    ap.add_argument("--suite")
    args = ap.parse_args()

    if args.suite:
        return run_suite(args.suite)
    if not (args.case and args.run_dir):
        ap.error("need --case + --run-dir, or --suite")

    report, problems = run_case(args.case, args.run_dir)
    if problems:
        print("ORACLE FAIL:", json.dumps(problems, indent=2))
        return 1
    print("ORACLE PASS", report["case_id"])
    return 0


if __name__ == "__main__":
    sys.exit(main())
