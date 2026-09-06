#!/usr/bin/env python3
"""Collect agentic run results into summary.json per AGENTIC-PHASE R1-R6.

Reads check-{cid}-{stage}.json artifacts (winning stage, pass rate,
win-depth distribution) + outcomes.jsonl (stage ledger). Writes
{run_dir}/summary.json. Exit 1 if no checks found.
"""
import argparse
import json
import sys
from collections import Counter
from pathlib import Path

STAGE_ORDER = ["attempt1", "solve", "attempt2", "repair1", "repair2"]


def taxonomy(fail_checks):
    fmt = {"json_exact", "line_count", "all_caps",
           "bullet_count_min", "bullet_count_max", "yaml_parsable"}
    leak = {"forbidden_phrases"}
    labels = []
    for c in fail_checks:
        if c in fmt:
            labels.append("FORMAT")
        elif c in leak:
            labels.append("LEAK")
        else:
            labels.append("CONTENT")
    return labels


def collect(run_dir):
    run_dir = Path(run_dir)
    checks = list(run_dir.glob("check-*.json"))
    if not checks:
        return None
    shape = {}
    shape_p = run_dir / "shape.json"
    if shape_p.exists():
        try:
            shape = json.loads(shape_p.read_text())
        except json.JSONDecodeError:
            pass
    per_case = {}
    tax = Counter()
    for f in checks:
        try:
            j = json.loads(f.read_text())
        except json.JSONDecodeError:
            continue
        cid = f.name.replace("check-", "").rsplit("-", 1)[0]
        stage = f.name.replace("check-", "").rsplit("-", 1)[1][:-5]
        if "stage_index" in j:
            rank = j["stage_index"]
        elif stage in shape:
            rank = shape[stage]
        elif stage in STAGE_ORDER:
            rank = STAGE_ORDER.index(stage)
        else:
            continue
        if j.get("passed"):
            cur = per_case.get(cid)
            if cur is None or rank < cur[0]:
                per_case[cid] = (rank, stage)
        else:
            for label in taxonomy(
                    [x.get("check", "?") for x in j.get("failures", [])]):
                tax[label] += 1
    def _known(f):
        try:
            j = json.loads(f.read_text())
        except json.JSONDecodeError:
            return False
        stage = f.name.replace("check-", "").rsplit("-", 1)[1][:-5]
        return (stage in STAGE_ORDER or stage in shape
                or j.get("stage_index") is not None)
    cases = sorted({f.name.replace("check-", "").rsplit("-", 1)[0]
                    for f in checks if _known(f)})
    passed = {c: s for c, (r, s) in per_case.items()}
    depth = Counter(r for r, _ in per_case.values())
    outcomes = []
    ol = run_dir / "outcomes.jsonl"
    if ol.exists():
        outcomes = [json.loads(x) for x in ol.read_text().splitlines()
                    if x.strip()]
    return {
        "total": len(cases),
        "passed": len(passed),
        "pass_rate": round(len(passed) / len(cases), 3) if cases else 0.0,
        "win_depths": {str(k): v for k, v in sorted(depth.items())},
        "mean_depth": round(
            sum(r for r, _ in per_case.values())
            / len(passed), 2) if passed else None,
        "winning_stages": dict(Counter(passed.values())),
        "failing_cases": sorted(set(cases) - set(passed)),
        "failure_taxonomy": dict(tax),
        "stage_ledger_entries": len(outcomes),
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    args = ap.parse_args()
    s = collect(args.run_dir)
    if s is None:
        print("no check artifacts", file=sys.stderr)
        return 1
    Path(args.run_dir).joinpath("summary.json").write_text(
        json.dumps(s, indent=1))
    print(json.dumps({k: s[k] for k in
                      ("total", "passed", "pass_rate", "win_depths",
                       "mean_depth", "failure_taxonomy")}))
    return 0


if __name__ == "__main__":
    sys.exit(main())
