#!/usr/bin/env python3
"""Leak-safe table feedback. Converts check-table result into fixer text:
missing cell ids, wrong cell ids + OBSERVED values + entity question.
Never prints expected values (F8).

Usage: table-feedback.py --result FILE --case FILE
"""
import argparse
import json
import sys
from pathlib import Path

import yaml


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--result", required=True)
    p.add_argument("--case", required=True)
    args = p.parse_args()

    try:
        result = json.loads(Path(args.result).read_text())
    except (json.JSONDecodeError, FileNotFoundError) as e:
        print(f"ERROR: cannot read result: {e}", file=sys.stderr)
        sys.exit(1)

    case = yaml.safe_load(Path(args.case).read_text())
    questions = {e["id"]: e["question"] for e in case["v6"]["entities"]}

    missing = result.get("missing", [])
    wrong = result.get("wrong", [])

    if not missing and not wrong:
        print("ALL TABLE CELLS PASSED. No feedback needed.")
        sys.exit(0)

    lines = []
    for eid in missing:
        lines.append(f"- {eid} [MISSING]: row absent. Question: {questions.get(eid, '?')}")
    for w in wrong:
        lines.append(f"- {w['id']} [WRONG]: your value was {json.dumps(w.get('observed'))}. Question: {questions.get(w['id'], '?')}")
    lines.append("Re-derive these entities from the task rules. Correct values withheld.")
    print("\n".join(lines))
    sys.exit(0)


if __name__ == "__main__":
    main()
