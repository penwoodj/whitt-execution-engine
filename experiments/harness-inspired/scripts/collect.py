#!/usr/bin/env python3
"""Suite collector. Reads trace.jsonl + verdicts, emits summary JSON.

Usage: collect.py --trace FILE [--out FILE]
Output: {"cases": N, "pass": N, "fail": N, "win_depth": {...}, "wasted": N}
"""
import argparse, json, sys
from pathlib import Path


def collect(trace_path):
    entries = []
    path = Path(trace_path)
    if path.is_file():
        for line in path.read_text().splitlines():
            line = line.strip()
            if line:
                try:
                    entries.append(json.loads(line))
                except json.JSONDecodeError:
                    continue

    cases = {}
    for e in entries:
        cid = e.get("case", "unknown")
        cases.setdefault(cid, {"gates": []})
        cases[cid]["gates"].append(e)

    stats = {"cases": len(cases), "pass": 0, "fail": 0, "win_depth": {}, "wasted": 0, "per_case": {}}
    for cid, info in cases.items():
        gates = info["gates"]
        check_gates = [g for g in gates if g.get("gate", "").startswith(("check", "fix"))]
        passed_gate = next((g for g in check_gates if g.get("verdict") == "pass"), None)
        end_fail = any(g.get("gate") == "end_fail" for g in gates)
        judge = any(g.get("gate") == "judge" and g.get("verdict") == "pass" for g in gates)

        if passed_gate:
            depth = 0
            for g in check_gates:
                if g.get("verdict") == "pass":
                    break
                depth += 1
            stats["win_depth"][str(depth)] = stats["win_depth"].get(str(depth), 0) + 1
            executed = len(check_gates)
            stats["wasted"] += executed - 1
            won = judge or not end_fail
            stats["pass" if won else "fail"] += 1
            stats["per_case"][cid] = {"verdict": "PASS" if won else "FAIL", "depth": depth, "checks": executed}
        else:
            stats["fail"] += 1
            stats["wasted"] += len(check_gates)
            stats["per_case"][cid] = {"verdict": "FAIL", "depth": None, "checks": len(check_gates)}

    return stats


def main():
    p = argparse.ArgumentParser(description="Suite collector")
    p.add_argument("--trace", required=True, help="trace.jsonl path")
    p.add_argument("--out", help="write summary JSON to file")
    args = p.parse_args()

    stats = collect(args.trace)

    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(json.dumps(stats, indent=2) + "\n")
    print(json.dumps(stats, indent=2))
    sys.exit(0)


if __name__ == "__main__":
    main()
