#!/usr/bin/env python3
"""budget_v2.py — call-budget governor check (script gate, no LLM).

Exit 0 = budget available (at least `reserve` calls of headroom);
exit 1 = exhausted. Called before every LLM-side step (replan heals,
judge) per the call-budget table in 00-V2-OVERVIEW.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sh_lib_v2 import (  # noqa: E402
    LLM_CALL_CEILING,
    budget_ok,
    llm_calls_used,
    run_dir_of,
    trace_append,
)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--reserve", type=int, default=1,
                    help="headroom required for the gated step")
    ap.add_argument("--site", default="gate")
    args = ap.parse_args()

    run_dir = Path(args.run_dir)
    used = llm_calls_used(run_dir)
    ok = used + args.reserve <= LLM_CALL_CEILING
    trace_append(run_dir, "budget_check", args.site, used=used,
                 ceiling=LLM_CALL_CEILING, reserve=args.reserve, ok=ok)
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
