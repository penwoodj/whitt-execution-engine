#!/usr/bin/env python3
"""heal_replan_v2.py — heavy-model replan heal for F3/F4 (spoofed).

In Phase S2 the LLM call is spoofed: this script writes the replan
record and increments the budget ledger exactly as the live heavy call
would. Phase L replaces the write with a real inference through the
heavy_replanner model; the trace contract is identical.

Exit 0 = replan produced; exit 1 = budget exhausted, no replan.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sh_lib_v2 import (  # noqa: E402
    LLM_CALL_CEILING,
    MODELS,
    budget_ok,
    load_case,
    llm_calls_used,
    read_json,
    run_dir_of,
    trace_append,
    write_json,
)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--round", type=int, required=True)
    args = ap.parse_args()

    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)

    if not budget_ok(run_dir):
        trace_append(run_dir, "budget_block", f"heal_replan_{args.round}",
                     used=llm_calls_used(run_dir), ceiling=LLM_CALL_CEILING)
        print("budget exhausted: replan blocked", file=sys.stderr)
        return 1

    triage = read_json(run_dir, f"triage_{args.round}.json")
    replan = {
        "round": args.round,
        "kind": "replan",
        "class": triage["class"],
        "strategy": "replan",
        "model": MODELS["heavy_replanner"],
        "plan": (
            "Recomputed plan: rederive every verdict from the artifact's "
            "own tables; reconcile cross-section numbers; restore dropped "
            "contract fields; re-emit complete artifact within contract."
        ),
    }
    write_json(run_dir, f"heal_{args.round}.json", replan)
    trace_append(run_dir, "heal_replan", f"heal_replan_{args.round}",
                 round=args.round, kind="replan", strategy="replan",
                 model=MODELS["heavy_replanner"])
    trace_append(run_dir, "llm_call", f"heal_replan_{args.round}",
                 model=MODELS["heavy_replanner"], site="replan")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
