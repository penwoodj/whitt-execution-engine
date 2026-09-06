#!/usr/bin/env python3
"""Batch tally for v4 campaign runs: per-case routelog pass/fail from
check-{pid}-routelog.json files in a run dir, plus stage progress from
outcomes.jsonl. See docs/06-VERSION-ITERATION-RULESET.md §8."""
import argparse
import json
import re
import sys
from pathlib import Path


def tally(run_dir):
    run_dir = Path(run_dir)
    checks = {}
    for f in run_dir.glob("check-*-routelog.json"):
        pid = re.match(r"check-(.+)-routelog\.json", f.name).group(1)
        try:
            d = json.loads(f.read_text())
        except json.JSONDecodeError:
            checks[pid] = "BROKEN-JSON"
            continue
        checks[pid] = "PASS" if d.get("passed") else "FAIL"
    stages = {}
    for line in (run_dir / "outcomes.jsonl").read_text().splitlines():
        if not line.strip():
            continue
        d = json.loads(line)
        stages.setdefault(d["case"], []).append(d["stage"])
    pids = sorted(set(checks) | set(stages))
    rows = []
    for pid in pids:
        rows.append((pid, checks.get(pid, "-"),
                     len(stages.get(pid, [])),
                     ",".join(stages.get(pid, [])[-2:])))
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("run_dir")
    a = ap.parse_args()
    rows = tally(a.run_dir)
    npass = sum(1 for r in rows if r[1] == "PASS")
    nfail = sum(1 for r in rows if r[1] == "FAIL")
    for r in rows:
        print(f"{r[0]:<10} {r[1]:<11} stages={r[2]:<3} last={r[3]}")
    print(f"\nPASS {npass}  FAIL {nfail}  PENDING {len(rows)-npass-nfail}")
    sys.exit(0 if rows and npass == len(rows) else 1)


if __name__ == "__main__":
    main()
