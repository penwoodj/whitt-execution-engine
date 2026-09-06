#!/usr/bin/env python3
"""judge_v2.py — gray-zone acceptance judge (spoofed Hermes call).

Only invoked when triage marks gray_zone (theta <= R < theta+eps).
Spoof policy (Phase S2): accept iff pseudo-self-consistency >= 0.5,
reject otherwise. Writes judge_<n>.json + llm_call ledger event.

Exit 0 = accept; exit 1 = reject (route back to heal path).
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sh_lib_v2 import (  # noqa: E402
    MODELS,
    load_case,
    read_json,
    run_dir_of,
    trace_append,
    write_json,
)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--attempt", type=int, required=True)
    args = ap.parse_args()

    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)
    det = read_json(run_dir, f"detection_{args.attempt}.json")

    psc = det["pseudo_self_consistency"]
    decision = "accept" if psc >= 0.50 else "reject"

    write_json(run_dir, f"judge_{args.attempt}.json",
               {"attempt": args.attempt, "model": MODELS["judge"],
                "psc": psc, "decision": decision})
    trace_append(run_dir, "judge", f"judge_{args.attempt}",
                 attempt=args.attempt, decision=decision,
                 model=MODELS["judge"])
    trace_append(run_dir, "llm_call", f"judge_{args.attempt}",
                 model=MODELS["judge"], site="judge")
    return 0 if decision == "accept" else 1


if __name__ == "__main__":
    raise SystemExit(main())
