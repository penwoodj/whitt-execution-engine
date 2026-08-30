#!/usr/bin/env python3
"""inject_live.py — after-hook for live attempt steps.

Reads the raw model output saved by save_to, leniently parses the JSON
artifact, applies the case's deterministic failure profile (same
spoof-kit inject() as Phase S2 so detector signatures match exactly),
and writes attempt_<n>.json plus the worker llm_call ledger event.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from lib_live import ROLE_IDS, corrupt, ensure_meta, parse_attempt, read_raw  # noqa: E402
from sh_lib_v2 import (  # noqa: E402
    MODELS,
    failed_attempts_of,
    load_case,
    run_dir_of,
    trace_append,
    write_json,
)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--attempt", type=int, required=True)
    ap.add_argument("--model-role", required=True)
    ap.add_argument("--raw", required=True)
    args = ap.parse_args()

    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)
    raw_path = Path(args.raw)

    attempt, parsed = parse_attempt(read_raw(raw_path))
    attempt = ensure_meta(attempt, parsed)

    failed = failed_attempts_of(case)
    if args.attempt in failed:
        cls = case["failure"]["class"]
        attempt = corrupt(attempt, cls, args.attempt, case["case_id"])
    else:
        attempt.setdefault("_meta", {})
        attempt["_meta"]["injected"] = False

    write_json(run_dir, f"attempt_{args.attempt}.json", attempt)
    trace_append(run_dir, "attempt", f"attempt_{args.attempt}",
                 attempt=args.attempt, model_role=args.model_role,
                 parsed=parsed, injected=args.attempt in failed)
    trace_append(run_dir, "llm_call", f"attempt_{args.attempt}",
                 site="worker", model=MODELS.get(args.model_role, args.model_role),
                 round=args.attempt)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
