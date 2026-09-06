#!/usr/bin/env python3
"""judge_gate_v2.py — gray-zone gate (script, no LLM).

Exit 0 = clear accept (R >= theta+eps): skip judge.
Exit 1 = gray zone (theta <= R < theta+eps): invoke judge step.
Reads triage_<n>.json written by triage_v2.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sh_lib_v2 import read_json, run_dir_of, trace_append  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--attempt", type=int, required=True)
    args = ap.parse_args()

    run_dir = run_dir_of(args.case, args.run_dir_base)
    triage = read_json(run_dir, f"triage_{args.attempt}.json")
    gray = bool(triage["gray_zone"])
    trace_append(run_dir, "judge_gate", f"judge_gate_{args.attempt}",
                 attempt=args.attempt, gray_zone=gray)
    return 1 if gray else 0


if __name__ == "__main__":
    raise SystemExit(main())
