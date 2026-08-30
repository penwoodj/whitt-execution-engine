#!/usr/bin/env python3
"""route_v2.py — worker variant selection (model specialization).

Decision table (docs/v2/02-WORKFLOW-V2-DESIGN §failure->strategy->model):
  F1 retry                    -> worker_fast      (exit 0)
  F2 in code-flavored domain  -> worker_coder     (exit 1)
  F4 with cx >= 4 compounds   -> heavy as worker  (exit 3)
  default                     -> worker_general   (exit 2)

cx = compound count from the previous triage's detection findings
(number of distinct failure signatures co-occurring). Writes
routing_<round>.json; exit code selects the attempt variant step via
GWT. Round 1 always routes general.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sh_lib_v2 import (  # noqa: E402
    CODE_DOMAINS,
    MODELS,
    ROUTE_CODER,
    ROUTE_FAST,
    ROUTE_GENERAL,
    ROUTE_HEAVY,
    load_case,
    read_json,
    run_dir_of,
    trace_append,
    write_json,
)

ROLE_OF_EXIT = {ROUTE_FAST: "worker_fast", ROUTE_CODER: "worker_coder",
                ROUTE_GENERAL: "worker_general", ROUTE_HEAVY: "heavy_replanner"}

COMPOUND_FLAGS = (
    "hallucination_keys", "schema_missing_keys", "schema_nested_wrong",
    "truncated_output", "tool_errors", "confidence_degraded",
)


def compound_count(triage: dict) -> int:
    det = triage.get("detection", {})
    return sum(1 for f in COMPOUND_FLAGS if det.get(f))


def decide(case: dict, run_dir: Path, round_no: int) -> int:
    if round_no == 1:
        return ROUTE_GENERAL
    prev = read_json(run_dir, f"triage_{round_no - 1}.json")
    cls = prev["class"]
    domain = case["meta"]["domain"]
    if cls == "F4" and compound_count(prev) >= 4:
        return ROUTE_HEAVY
    if cls == "F1":
        return ROUTE_FAST
    if cls == "F2" and domain in CODE_DOMAINS:
        return ROUTE_CODER
    return ROUTE_GENERAL


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--round", type=int, required=True)
    args = ap.parse_args()

    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)
    route = decide(case, run_dir, args.round)
    role = ROLE_OF_EXIT[route]

    write_json(run_dir, f"routing_{args.round}.json",
               {"round": args.round, "model_role": role, "model": MODELS[role]})
    trace_append(run_dir, "route", f"route_{args.round}",
                 round=args.round, model_role=role)
    return route


if __name__ == "__main__":
    raise SystemExit(main())
