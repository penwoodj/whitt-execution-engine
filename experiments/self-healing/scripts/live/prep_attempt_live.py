#!/usr/bin/env python3
"""prep_attempt_live.py — before-hook for live attempt variants.

Checks the router's decision against this step's variant. On match,
prints the full attempt prompt to stdout (the step's own prompt template
reads it back); on mismatch (sibling variant reached by fallthrough)
exits 42 so the GWT guard routes to detect without any inference.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from lib_live import VARIANT_ROLES, worker_prompt  # noqa: E402
from sh_lib_v2 import load_case, read_json, run_dir_of  # noqa: E402

MISMATCH_EXIT = 42


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--round", type=int, required=True)
    ap.add_argument("--variant", required=True,
                    choices=["fast", "coder", "general", "heavy"])
    args = ap.parse_args()

    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)
    routing = read_json(run_dir, f"routing_{args.round}.json")

    if routing.get("model_role") != VARIANT_ROLES[args.variant]:
        return MISMATCH_EXIT

    sys.stdout.write(worker_prompt(case, args.round, run_dir))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
