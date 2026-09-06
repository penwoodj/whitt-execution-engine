#!/usr/bin/env python3
"""judge_eval_live.py — parse judge verdict from raw output.

Reads the judge model's raw text (save_to artifact), extracts
ACCEPT/REJECT, writes judge_<round>.json plus the judge llm_call ledger
event. Exit 0 = accept, 1 = reject (unparseable counts as reject).
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from lib_live import read_raw  # noqa: E402
from sh_lib_v2 import MODELS, run_dir_of, trace_append, write_json  # noqa: E402

VERDICT_RE = re.compile(r"\b(ACCEPT|REJECT)\b", re.IGNORECASE)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--round", type=int, required=True)
    ap.add_argument("--raw", required=True)
    args = ap.parse_args()

    run_dir = run_dir_of(args.case, args.run_dir_base)
    raw = read_raw(Path(args.raw))

    head = raw[:400]
    match = VERDICT_RE.search(head)
    verdict = match.group(1).upper() if match else "REJECT"
    if match is None:
        basis = "unparseable judge output; conservative reject"
    else:
        basis = raw[:300].replace("\n", " ").strip()

    write_json(run_dir, f"judge_{args.round}.json",
               {"round": args.round, "verdict": verdict, "basis": basis})
    trace_append(run_dir, "judge", f"judge_{args.round}",
                 round=args.round, verdict=verdict)
    trace_append(run_dir, "llm_call", f"judge_{args.round}",
                 site="judge", model=MODELS["judge"])
    return 0 if verdict == "ACCEPT" else 1


if __name__ == "__main__":
    raise SystemExit(main())
