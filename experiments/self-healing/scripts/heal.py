#!/usr/bin/env python3
"""heal.py — class-routed healing step (Phase S, zero-LLM spoof).

Paper 2605.06737 healing strategies:
  F1 → corrective_prompt   (role model: Ministral-3-3B)
  F2 → tool_reselect       (role model: Qwen2.5-Coder-3B)
  F3/F4 → replan           (role model: Qwen3-5-9B)

Writes heal_N.json (the corrective context the NEXT attempt consumes —
Reflexion-style bounded episodic memory, research doc 02) + trace event.
Exit: 0 on heal action recorded.
"""
import argparse
import sys
import os

import yaml

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sh_lib

CORRECTIVE_CORE = {
    "corrective_prompt": (
        "ROLE: correction specialist. Prior attempt failed a validity check.\n"
        "Restate the REPORT SPEC keys, drop any field not derivable from FACTS,\n"
        "re-derive each value from the rules alone. No verification claims\n"
        "you cannot derive (e.g. source_verified)."
    ),
    "tool_reselect": (
        "ROLE: tool-repair specialist. A tool call failed (connection refused).\n"
        "Switch to the fallback endpoint cache_api.dupe_scan_v2, re-issue ONLY\n"
        "the failed call, keep prior successful results."
    ),
    "replan": (
        "ROLE: replanner. Rebuild the task plan from the objective.\n"
        "Exclude the failed subtask, decompose into: (1) recompute class deltas,\n"
        "(2) fold duplicates, (3) recompute retained. Priority order: deltas,\n"
        "dupes, retained. Re-execute from the first failed subtask."
    ),
}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--round", type=int, required=True, help="heal round = failed attempt #")
    ap.add_argument("--strategy", required=True,
                    choices=["corrective_prompt", "tool_reselect", "replan"])
    args = ap.parse_args()

    with open(args.case) as f:
        case = yaml.safe_load(f)

    verdict = sh_lib.read_json(
        os.path.join(sh_lib.attempt_dir(args.run_dir, args.round), "verdict.json")
    )

    record = {
        "round": args.round,
        "strategy": args.strategy,
        "failed_classification": verdict["classification"],
        "failed_R": verdict["R"],
        "corrective_instruction": CORRECTIVE_CORE[args.strategy],
        "consumed_by_attempt": args.round + 1,
    }
    sh_lib.write_json(os.path.join(args.run_dir, "heal_%d.json" % args.round), record)
    sh_lib.append_trace(
        args.run_dir, "heal_%s" % args.strategy, round=args.round,
        failed_classification=verdict["classification"],
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
