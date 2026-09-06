#!/usr/bin/env python3
"""Comprehensive checker — runs ALL 5 angle-specific checks against given text.

Used as the gate for v4 backward routing. If ANY check fails, exit non-zero.

Usage: check-all.py <case.yml> <text>
"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_angle_lib import run_checks_for_angle


def main():
    if len(sys.argv) < 3:
        print("usage: check-all.py <case.yml> <text-or-file>", file=sys.stderr)
        sys.exit(2)
    case_path = Path(sys.argv[1])
    text_arg = sys.argv[2]

    text = text_arg
    if Path(text_arg).exists():
        text = Path(text_arg).read_text()

    import yaml
    with open(case_path) as f:
        case = yaml.safe_load(f)

    all_results = {
        "passed": True,
        "angle_checks": {},
    }
    for angle_num in range(1, 6):
        result = run_checks_for_angle(angle_num, case, text)
        all_results["angle_checks"][f"angle_{angle_num}"] = result
        if not result["passed"]:
            all_results["passed"] = False

    print(json.dumps(all_results, indent=2))
    sys.exit(0 if all_results["passed"] else 1)


if __name__ == "__main__":
    main()
