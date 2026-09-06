#!/usr/bin/env python3
"""outcome_v2.py — terminal outcome event (accept / final_fail).

Also stamps outcome.json with attempts_used, llm_calls breakdown,
and budget state for the oracle report.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sh_lib_v2 import (  # noqa: E402
    LLM_CALL_CEILING,
    MODELS,
    load_case,
    llm_calls_used,
    run_dir_of,
    trace_append,
    trace_read,
    write_json,
)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--outcome", choices=["accept", "final_fail"], required=True)
    args = ap.parse_args()

    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)
    events = trace_read(run_dir)

    attempts = [e for e in events if e["event"] == "attempt"]
    calls = [e for e in events if e["event"] == "llm_call"]
    by_site: dict[str, int] = {}
    for c in calls:
        by_site[c["data"]["site"]] = by_site.get(c["data"]["site"], 0) + 1

    outcome = {
        "outcome": args.outcome,
        "attempts_used": len(attempts),
        "llm_calls": {
            "worker": by_site.get("worker", 0),
            "heavy": by_site.get("replan", 0),
            "judge": by_site.get("judge", 0),
        },
        "llm_calls_total": len(calls),
        "llm_call_ceiling": LLM_CALL_CEILING,
        "budget_exhausted": len(calls) >= LLM_CALL_CEILING,
    }
    write_json(run_dir, "outcome.json", outcome)
    trace_append(run_dir, args.outcome, args.outcome, attempts=len(attempts),
                 llm_calls=len(calls))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
