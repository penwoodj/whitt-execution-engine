#!/usr/bin/env python3
"""Fail-flag gate. Exit 1 if the table result file says failed (or is
missing), exit 0 if passed. Lets the end_fail GATE step distinguish
routed-here-because-failed from topological fall-through after a win.

Usage: failflag.py --result FILE
"""
import argparse
import json
import sys
from pathlib import Path


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--result", required=True)
    p.add_argument("--final-check", help="aggregate final-check JSON (checked if present)")
    args = p.parse_args()

    failed = False
    try:
        r = json.loads(Path(args.result).read_text())
        failed = not r.get("pass", False)
    except (FileNotFoundError, json.JSONDecodeError):
        failed = True

    if not failed and args.final_check and Path(args.final_check).is_file():
        try:
            fc = json.loads(Path(args.final_check).read_text())
            failed = not fc.get("pass", False)
        except json.JSONDecodeError:
            failed = True

    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
