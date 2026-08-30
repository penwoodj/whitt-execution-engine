#!/usr/bin/env python3
"""heal_replan_live.py — heavy-model replan heal for F3/F4 (LIVE).

--pre  : budget gate + writes heal_<round>.json skeleton + prints the
         replanning prompt to stdout (fed to the heavy step's prompt
         template). Exit 0 = proceed to real inference, 1 = budget
         exhausted (GWT routes to FINAL_FAIL).
--post : finalizes the heal record with the replanner's raw output and
         appends the replan llm_call ledger event. Runs as an
         after_step_succeeds hook; exit 0 always.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from lib_live import read_raw  # noqa: E402
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


def pre(args: argparse.Namespace) -> int:
    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)

    if not budget_ok(run_dir):
        trace_append(run_dir, "budget_block", f"heal_replan_{args.round}",
                     used=llm_calls_used(run_dir), ceiling=LLM_CALL_CEILING)
        print("budget exhausted: replan blocked", file=sys.stderr)
        return 1

    triage = read_json(run_dir, f"triage_{args.round}.json")
    detection = read_json(run_dir, f"detection_{args.round}.json")
    replan = {
        "round": args.round,
        "kind": "replan",
        "class": triage["class"],
        "strategy": "replan",
        "model": MODELS["heavy_replanner"],
        "status": "pending",
    }
    write_json(run_dir, f"heal_{args.round}.json", replan)

    findings = "\n".join(
        "  %s: %s" % (k, v) for k, v in detection.items()
        if v and k not in ("confidence", "pseudo_self_consistency")
    ) or "  (none)"

    sys.stdout.write(
        "RE-PLANNING TASK (round %d, failure class %s, R=%.4f).\n\n"
        "TASK CONTRACT (verbatim):\n%s\n\n"
        "DETECTOR FINDINGS (round %d):\n%s\n\n"
        "Produce two numbered items, max 200 words total:\n"
        "1. Root-cause analysis of why the attempt failed.\n"
        "2. Corrective strategy instruction for the next attempt — "
        "specific, checkable, referencing the contract fields.\n"
        % (args.round, triage["class"], triage.get("R", 0.0),
           case["task"]["prompt"], args.round, findings)
    )
    return 0


def post(args: argparse.Namespace) -> int:
    run_dir = run_dir_of(args.case, args.run_dir_base)
    raw = read_raw(Path(args.raw))

    heal = read_json(run_dir, f"heal_{args.round}.json")
    heal["status"] = "complete"
    heal["strategy_text"] = raw[:2400]
    write_json(run_dir, f"heal_{args.round}.json", heal)

    trace_append(run_dir, "heal_replan", f"heal_replan_{args.round}",
                 round=args.round, kind="replan", strategy="replan",
                 model=MODELS["heavy_replanner"])
    trace_append(run_dir, "llm_call", f"heal_replan_{args.round}",
                 model=MODELS["heavy_replanner"], site="replan")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case")
    ap.add_argument("--run-dir-base")
    ap.add_argument("--round", type=int)
    ap.add_argument("--mode", required=True, choices=["pre", "post"])
    ap.add_argument("--raw")
    args = ap.parse_args()

    if args.mode == "pre":
        if not (args.case and args.run_dir_base and args.round):
            ap.error("--pre requires --case --run-dir-base --round")
        return pre(args)
    if not (args.case and args.run_dir_base and args.round and args.raw):
        ap.error("--post requires --case --run-dir-base --round --raw")
    return post(args)


if __name__ == "__main__":
    raise SystemExit(main())
