#!/usr/bin/env python3
"""Build JSON change log comparing each angle's output to prior.

Reads after-angle-N.txt files from output dir, computes per-angle diff
summary, writes JSON change log.

Usage: build-changelog.py <case.yml> <output_dir>
"""
import json
import sys
from pathlib import Path

import yaml


def diff_summary(before: str, after: str) -> dict:
    if before.strip() == after.strip():
        return {
            "changed": False,
            "before_excerpt": before.strip()[:200],
            "after_excerpt": after.strip()[:200],
            "char_delta": 0,
        }
    return {
        "changed": True,
        "before_excerpt": before.strip()[:200],
        "after_excerpt": after.strip()[:200],
        "char_delta": len(after) - len(before),
    }


def main():
    if len(sys.argv) < 3:
        print("usage: build-changelog.py <case.yml> <output_dir>", file=sys.stderr)
        sys.exit(2)
    case_path = Path(sys.argv[1])
    outdir = Path(sys.argv[2])

    with open(case_path) as f:
        case = yaml.safe_load(f)

    broken = case.get("broken_output", "").strip()
    angles = [
        ("angle_1_format_auditor", "after-angle-1.txt"),
        ("angle_2_fact_checker", "after-angle-2.txt"),
        ("angle_3_requirements_tracer", "after-angle-3.txt"),
        ("angle_4_hallucination_hunter", "after-angle-4.txt"),
        ("angle_5_consistency_auditor", "final-corrected.txt"),
    ]

    log = {
        "case_id": case.get("case_id"),
        "issue_type": case.get("issue_type"),
        "angles": [],
    }

    prev = broken
    for angle_name, fname in angles:
        path = outdir / fname
        if not path.exists():
            log["angles"].append({
                "angle": angle_name,
                "status": "missing",
                "diff": None,
            })
            continue
        current = path.read_text()
        d = diff_summary(prev, current)
        check_path = outdir / f"check-angle-{angle_name.split('_')[1]}.json"
        check_result = None
        if check_path.exists():
            try:
                check_result = json.loads(check_path.read_text())
            except Exception:
                pass
        log["angles"].append({
            "angle": angle_name,
            "status": "changed" if d["changed"] else "unchanged",
            "diff": d,
            "deterministic_check": check_result,
        })
        prev = current

    print(json.dumps(log, indent=2))


if __name__ == "__main__":
    main()
