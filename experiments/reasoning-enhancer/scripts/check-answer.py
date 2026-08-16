#!/usr/bin/env python3
"""REA deterministic answer checker (v1 judge).

Usage:
  check-answer.py --case CASE.yml --text-file FILE --out check.json

Reads success_criteria.deterministic_checks from the case, runs check_lib on
the text file, writes the result JSON. Exit 0 iff all checks passed, else 1 —
the workflow's gwt early-exit routes on this exit code (v8 check-angle.py
semantics; "exit 0 always" broke round-2 escalation in live run 1).
"""

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import yaml

from check_lib import run_checks


def load_case(path):
    with open(path) as fh:
        return yaml.safe_load(fh)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--text-file", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()

    case = load_case(args.case)
    checks = (case.get("success_criteria") or {}).get("deterministic_checks") or {}
    text = Path(args.text_file).read_text()

    result = run_checks(text, checks)
    result["case_id"] = case.get("case_id")
    result["text_file"] = args.text_file

    Path(args.out).write_text(json.dumps(result, indent=2))
    print(json.dumps({"passed": result["passed"], "out": args.out}))
    return 0 if result["passed"] else 1


if __name__ == "__main__":
    sys.exit(main())
