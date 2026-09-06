#!/usr/bin/env python3
"""jgate_prep_live.py — gray-zone check + judge prompt feed.

Exit 0 = not gray, candidate auto-accepts (GWT routes to ACCEPT).
Exit 1 = gray zone: prints the full adjudication prompt to stdout for
the judge step's prompt template to read back.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from sh_lib_v2 import load_case, read_json, run_dir_of  # noqa: E402

CANDIDATE_CHARS = 6000


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--attempt", type=int, required=True)
    args = ap.parse_args()

    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)
    triage = read_json(run_dir, f"triage_{args.attempt}.json")

    if not triage.get("gray_zone"):
        return 0

    attempt = read_json(run_dir, f"attempt_{args.attempt}.json")
    candidate = json.dumps(attempt, indent=1, ensure_ascii=False)
    if len(candidate) > CANDIDATE_CHARS:
        candidate = candidate[:CANDIDATE_CHARS] + "\n... [truncated]"

    sys.stdout.write(
        "ADJUDICATION TASK (gray zone: reliability R=%.4f within epsilon "
        "of threshold).\n\n"
        "TASK CONTRACT (abstract):\n%s\n\n"
        "CANDIDATE ARTIFACT:\n%s\n\n"
        "Decide: does the candidate satisfy the contract well enough to "
        "accept?\n"
        "Reply with exactly ACCEPT or REJECT on the first line, then one "
        "short line of basis.\n"
        % (triage.get("R", 0.0),
           case["task"]["prompt"][:1200],
           candidate)
    )
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
