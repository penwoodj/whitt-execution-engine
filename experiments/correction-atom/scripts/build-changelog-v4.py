#!/usr/bin/env python3
"""Build v4 change log: per-angle + per-pass diff summary.

Reads pass-N-after-angle-M.txt files, computes diffs across passes/angles,
emits JSON change log capturing which angle mutated text and the final state.

Usage: build-changelog-v4.py <case.yml> <output_dir>
"""
import json
import sys
from pathlib import Path

import yaml


def diff_summary(before: str, after: str) -> dict:
    if before.strip() == after.strip():
        return {"changed": False, "char_delta": 0}
    return {
        "changed": True,
        "char_delta": len(after) - len(before),
        "before_excerpt": before.strip()[:200],
        "after_excerpt": after.strip()[:200],
    }


def main():
    if len(sys.argv) < 3:
        print("usage: build-changelog-v4.py <case.yml> <output_dir>", file=sys.stderr)
        sys.exit(2)
    case_path = Path(sys.argv[1])
    outdir = Path(sys.argv[2])

    with open(case_path) as f:
        case = yaml.safe_load(f)

    broken = case.get("broken_output", "").strip()
    log = {
        "case_id": case.get("case_id"),
        "issue_type": case.get("issue_type"),
        "passes": [],
    }

    for pass_num in [1, 2]:
        pass_entry = {"pass": pass_num, "angles": [], "ran": False}
        prev = broken if pass_num == 1 else None
        for angle in range(1, 6):
            if angle == 5 and pass_num == 1:
                fname = f"pass-{pass_num}-after-angle-{angle}.txt"
            else:
                fname = f"pass-{pass_num}-after-angle-{angle}.txt"
            path = outdir / fname
            if not path.exists():
                continue
            pass_entry["ran"] = True
            current = path.read_text()
            if prev is None:
                prev = current
                continue
            d = diff_summary(prev, current)
            check_path = outdir / f"pass-{pass_num}-check-{angle}.json"
            check_result = None
            if check_path.exists():
                try:
                    check_result = json.loads(check_path.read_text())
                except Exception:
                    pass
            pass_entry["angles"].append({
                "angle": angle,
                "status": "changed" if d["changed"] else "unchanged",
                "diff": d,
                "deterministic_check": check_result,
            })
            prev = current
        if pass_entry["ran"]:
            log["passes"].append(pass_entry)

    final_path = outdir / "final-corrected.txt"
    if final_path.exists():
        log["final_text"] = final_path.read_text().strip()
        log["final_vs_broken_diff"] = diff_summary(broken, final_path.read_text())

    print(json.dumps(log, indent=2))


if __name__ == "__main__":
    main()
