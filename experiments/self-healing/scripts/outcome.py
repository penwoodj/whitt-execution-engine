#!/usr/bin/env python3
"""outcome.py — terminal outcome record (accept / final_fail) + trace event.

Derives the deciding attempt + R from the LAST classify event in trace.jsonl
(accept can occur at attempt 1, 2, or 3; final_fail always at MAX_ATTEMPTS).
Exit: 0 always.
"""
import argparse
import sys
import os

import yaml

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sh_lib


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--outcome", required=True, choices=["accept", "final_fail"])
    args = ap.parse_args()

    with open(args.case) as f:
        case = yaml.safe_load(f)

    cls = [t for t in sh_lib.read_trace(args.run_dir) if t["event"] == "classify"]
    if not cls:
        raise SystemExit("no classify events in trace — cannot record outcome")
    last = cls[-1]

    rec = {
        "case_id": case["case_id"],
        "outcome": args.outcome,
        "at_attempt": last["attempt"],
        "R": last["R"],
        "classification": last["classification"],
    }
    sh_lib.write_json(os.path.join(args.run_dir, "outcome.json"), rec)
    sh_lib.append_trace(args.run_dir, args.outcome, attempt=last["attempt"], R=last["R"])
    return 0


if __name__ == "__main__":
    sys.exit(main())
