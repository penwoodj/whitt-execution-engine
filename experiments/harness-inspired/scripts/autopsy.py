#!/usr/bin/env python3
"""Suite autopsy: classify failures per check class from a suite-results JSON.

Classes:
- ERROR      — engine/run failure, no trace
- NO_TABLE   — solve produced 0 parseable rows (format collapse)
- VAL_WRONG  — rows parsed, values wrong, fixes never corrected
- AGG_MISMATCH — table cells passed but final aggregate check failed
- JUDGE_ONLY — det pass but judge fail (informational, two-lane override)

Usage: autopsy.py --results v8-suite-results.json
"""
import argparse
import json
import sys
from collections import Counter
from pathlib import Path


def classify(r):
    if r["verdict"] == "ERROR":
        return "ERROR"
    trace = r.get("trace", [])
    gates = [(t.get("gate"), t.get("verdict")) for t in trace]
    checks = [v for g, v in gates if g in ("check", "fix_1", "fix_2", "fix_3")]
    if r["verdict"] == "PASS":
        if any(g == "judge" and v == "fail" for g, v in gates):
            return "PASS_JUDGE_DISSENT"
        return "PASS"
    if not checks:
        return "NO_TRACE"
    if all(v == "fail" for v in checks):
        final = [v for g, v in gates if g == "final"]
        if final and final[0] == "fail":
            first_check = next((t for t in trace if t.get("gate") == "check"), {})
            score = first_check.get("score", 0)
            if score == 0:
                return "NO_TABLE"
            return "VAL_WRONG"
        return "VAL_WRONG"
    if checks[-1] == "pass" and any(g == "final" and v == "fail" for g, v in gates):
        return "AGG_MISMATCH"
    return "VAL_WRONG"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--results", required=True)
    args = ap.parse_args()
    data = json.loads(Path(args.results).read_text())

    buckets = {}
    for r in data:
        cls = classify(r)
        buckets.setdefault(cls, []).append(r["case"])

    print(f"total: {len(data)}  pass: {sum(1 for r in data if r['verdict'] == 'PASS')}")
    for cls in sorted(buckets):
        cases = buckets[cls]
        print(f"{cls}: {len(cases)}")
        if cls not in ("PASS",):
            print("  " + ",".join(sorted(cases)))

    fails = [r for r in data if r["verdict"] != "PASS"]
    if fails:
        print("\nfail detail:")
        for r in sorted(fails, key=lambda x: x["case"]):
            gates = ",".join(f"{t.get('gate')}={t.get('verdict', '?')}" for t in r.get("trace", []))
            print(f"  {r['case']} [{classify(r)}] ({r.get('elapsed', 0)}s) {gates}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
