#!/usr/bin/env python3
"""Deterministic per-angle checker. Wraps check_angle_lib for CLI use.

Usage: check-angle.py <angle_num> <case.yml> <text>
"""
import json
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).parent))
from check_angle_lib import run_checks_for_angle


def main():
    if len(sys.argv) < 4:
        print("usage: check-angle.py <angle_num> <case.yml> <text-or-file>", file=sys.stderr)
        sys.exit(2)
    angle_num = int(sys.argv[1])
    case_path = Path(sys.argv[2])
    text_arg = sys.argv[3]

    text = text_arg
    if Path(text_arg).exists():
        text = Path(text_arg).read_text()

    with open(case_path) as f:
        case = yaml.safe_load(f)

    result = run_checks_for_angle(angle_num, case, text)
    print(json.dumps(result, indent=2))
    sys.exit(0 if result["passed"] else 1)


if __name__ == "__main__":
    main()
