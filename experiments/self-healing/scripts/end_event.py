#!/usr/bin/env python3
"""end_event.py — terminal trace marker after report oracle.

report.py runs before end steps and validates the trace prefix up to its own
event; this marker appends AFTER, so it never interferes with path matching.
"""
import argparse
import sys
import os

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sh_lib


def reset_run(run_dir):
    """init = fresh workflow start: clear stale artifacts from earlier runs.

    Stale trace/outcome/report from a previous run would corrupt the oracle
    (path matching, attempts count). Remove them so every live run is clean.
    """
    import glob
    import shutil
    for stale in ("trace.jsonl", "outcome.json", "report.json"):
        p = os.path.join(run_dir, stale)
        if os.path.exists(p):
            os.remove(p)
    for d in glob.glob(os.path.join(run_dir, "attempt_*")):
        shutil.rmtree(d, ignore_errors=True)
    for h in glob.glob(os.path.join(run_dir, "heal_*.json")):
        os.remove(h)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--status", required=True, choices=["init", "pass", "fail"])
    args = ap.parse_args()
    if args.status == "init":
        reset_run(args.run_dir)
        sh_lib.append_trace(args.run_dir, "init", status=args.status)
    else:
        sh_lib.append_trace(args.run_dir, "end", status=args.status)
    return 0


if __name__ == "__main__":
    sys.exit(main())
