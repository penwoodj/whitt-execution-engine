#!/usr/bin/env python3
"""REA judge (v1: deterministic-only).

Usage:
  judge-answer.py --case CASE.yml --text-file FILE --out judge.json

v1 judge = check_lib deterministic checks; judge_method recorded in output so
live-run analysis can distinguish judge versions. v2 may add a Prometheus-
style 4B rubric judge (narrow rubric + reference materials only) — see
DESIGN.md "Known v1 limitations".
"""

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import yaml

from check_lib import run_checks


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--text-file", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()

    with open(args.case) as fh:
        case = yaml.safe_load(fh)
    checks = (case.get("success_criteria") or {}).get("deterministic_checks") or {}
    text = Path(args.text_file).read_text()

    result = run_checks(text, checks)
    result["case_id"] = case.get("case_id")
    result["judge_method"] = "deterministic_v1"
    result["score"] = (
        result["subchecks_passed"] / result["subchecks_total"]
        if result["subchecks_total"]
        else 0.0
    )

    Path(args.out).write_text(json.dumps(result, indent=2))
    print(json.dumps({"passed": result["passed"], "score": result["score"]}))
    return 0 if result["passed"] else 1


if __name__ == "__main__":
    sys.exit(main())
