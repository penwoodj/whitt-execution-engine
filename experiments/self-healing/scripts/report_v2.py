#!/usr/bin/env python3
"""report_v2.py — oracle report for self-healing v2.

Per-case checks:
  O1 verdict: outcome vs case expected.final
  O2 semantic path: trace event sequence vs expected.path
  O3 llm_calls: outcome llm_calls vs case oracle.llm_calls
  O4 budget: calls <= ceiling; budget_exhausted flag matches oracle
Exit 0 = oracle pass; 1 = fail. Suite mode (--suite <results-base>)
adds TSR / FDA / RSR and call-economy (v2 actual vs naive 3x cases).

Path translation: trace events map to semantic tokens
  attempt->attempt, classify->classify, heal_corrective_prompt/
  heal_tool_reselect/heal_replan as-is, judge->judge, accept->accept,
  final_fail->final_fail. judge_gate is implicit (no trace event);
  report appended at end.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

import yaml

TOKEN_OF_EVENT = {
    "attempt": "attempt",
    "classify": "classify",
    "heal_corrective_prompt": "heal_corrective_prompt",
    "heal_tool_reselect": "heal_tool_reselect",
    "heal_replan": "heal_replan",
    "judge": "judge",
    "accept": "accept",
    "final_fail": "final_fail",
}
VOCAB = {"accept": "pass", "final_fail": "fail"}


def load(path: str) -> dict:
    return yaml.safe_load(Path(path).read_text())


def actual_path(events: list[dict]) -> list[str]:
    toks = [TOKEN_OF_EVENT[e["event"]] for e in events if e["event"] in TOKEN_OF_EVENT]
    toks.append("report")
    # collapse judge_gate expectation: expected paths may contain
    # judge_gate; actual never emits it -> drop from expected at compare
    return toks


def check_case(case: dict, run_dir: Path) -> dict:
    problems: list[str] = []
    outcome = json.loads((run_dir / "outcome.json").read_text())
    events = [json.loads(l) for l in (run_dir / "trace.jsonl").read_text().splitlines() if l.strip()]

    # O1 verdict
    if outcome["outcome"] != case["expected"]["final"]:
        problems.append(f"verdict: {outcome['outcome']} != {case['expected']['final']}")

    # O2 path
    exp = [t for t in case["expected"]["path"] if t != "judge_gate"]
    act = actual_path(events)
    if exp != act:
        problems.append(f"path: {act} != {exp}")

    # O3 llm_calls
    want = case["oracle"]["llm_calls"]
    got = outcome["llm_calls"]
    for k in ("worker", "heavy", "judge"):
        if got.get(k, 0) != want.get(k, 0):
            problems.append(f"llm_calls.{k}: {got.get(k, 0)} != {want.get(k, 0)}")

    # O4 budget
    if outcome["llm_calls_total"] > outcome["llm_call_ceiling"]:
        problems.append("budget: ceiling exceeded")
    if outcome["budget_exhausted"] != bool(case["oracle"]["gate_flags"]["budget_exhausted"]):
        problems.append("budget_exhausted flag mismatch")

    verdict = VOCAB[outcome["outcome"]]
    return {
        "case_id": case["case_id"],
        "outcome": outcome["outcome"],
        "attempts_used": outcome["attempts_used"],
        "llm_calls": outcome["llm_calls_total"],
        "oracle_pass": not problems,
        "problems": problems,
        "verdict": verdict,
    }


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", help="per-case mode: case YAML path")
    ap.add_argument("--run-dir", help="per-case mode: run dir")
    ap.add_argument("--suite", help="suite mode: results base dir")
    args = ap.parse_args()

    if args.suite:
        base = Path(args.suite)
        pairs = []
        for case_dir in sorted(p for p in base.iterdir() if p.is_dir()):
            if not (case_dir / "outcome.json").exists():
                continue
            case_yaml = None
            for cand in (base.parent / "cases" / "v2" / "flagship",
                         base.parent / "cases" / "v2" / "matrix"):
                c = cand / f"{case_dir.name}.yml"
                if c.exists():
                    case_yaml = c
                    break
            if case_yaml is None:
                continue
            pairs.append((load(str(case_yaml)), case_dir))

        results = [check_case(case, cd) for case, cd in pairs]
        n = len(results)
        tsr = sum(1 for r in results if r["verdict"] == "pass") / n if n else 0.0

        # FDA: of attempts the case injects as failed, fraction classified
        # non-clean by the workflow trace
        fda_hits = 0
        fda_total = 0
        for case, _cd in pairs:
            failed = case.get("failure", {}).get("injection_profile", {}).get("attempts_failed", [])
            cd = _cd
            events = [json.loads(l) for l in (cd / "trace.jsonl").read_text().splitlines() if l.strip()]
            for e in events:
                if e["event"] == "classify" and e["data"]["attempt"] in failed:
                    fda_total += 1
                    if e["data"]["classification"] != "clean":
                        fda_hits += 1

        rsr_rec = sum(1 for r in results if r["verdict"] == "pass" and r["attempts_used"] > 1)
        rsr_det = sum(1 for r in results if r["attempts_used"] > 1)
        calls = sum(r["llm_calls"] for r in results)
        summary = {
            "cases": n,
            "TSR": round(tsr, 4),
            "FDA": {"hits": fda_hits, "total": fda_total,
                    "value": round(fda_hits / fda_total, 4) if fda_total else None},
            "RSR": {"recovered": rsr_rec, "detected_cases": rsr_det,
                    "value": round(rsr_rec / rsr_det, 4) if rsr_det else None},
            "call_economy": {"v2_calls": calls, "naive_calls": 3 * n,
                             "ratio": round(calls / (3 * n), 4) if n else None},
            "oracle_all_pass": all(r["oracle_pass"] for r in results),
            "per_case": results,
        }
        print(json.dumps(summary, indent=2))
        return 0 if summary["oracle_all_pass"] else 1

    if not (args.case and args.run_dir):
        ap.error("per-case mode needs --case and --run-dir")
    case = load(args.case)
    rep = check_case(case, Path(args.run_dir))
    (Path(args.run_dir) / "report.json").write_text(json.dumps(rep, indent=2) + "\n")
    print(json.dumps(rep, indent=2))
    return 0 if rep["oracle_pass"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
