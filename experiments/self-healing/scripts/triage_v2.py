#!/usr/bin/env python3
"""triage_v2.py — score reliability R, classify failure, decide gate.

R = w_C*C + w_S*S + w_E*E in [0,1] (higher = more reliable).
Class priority F4 > F3 > F2 > F1 (paper doc 01; LARD doc 12 boosts F2
over F3 for equal evidence, but contradiction presence dominates).

Gate decision (exit code):
  0 clean      -> judge_gate (judge only if gray zone)
  1 light heal -> F1/F2 script heal (corrective_prompt / tool_reselect)
  2 replan     -> F3/F4 heavy-model replan, budget-gated upstream
  3 fail       -> round 3 miss or unrecoverable signature

Writes triage_<n>.json with R, class, gate, gray_zone flags.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sh_lib_v2 import (  # noqa: E402
    EPSILON,
    GATE_CLEAN,
    GATE_FAIL,
    GATE_LIGHT,
    GATE_REPLAN,
    MAX_ATTEMPTS,
    OMEGA,
    THETA,
    load_case,
    read_json,
    run_dir_of,
    trace_append,
    write_json,
)


def score(det: dict, attempt_no: int) -> tuple[float, str]:
    C = 1.0
    S = 1.0
    E = 1.0
    cls = "clean"
    if det["hallucination_keys"]:
        C = min(C, 0.45)
        S = min(S, 0.20)
        cls = "F1"
    if det["confidence_degraded"]:
        C = min(C, 0.45)
        cls = cls if cls != "clean" else "F1"
    if det["schema_missing_keys"] or det["schema_nested_wrong"] or det["tool_errors"]:
        E = min(E, 0.45)
        S = min(S, 0.40)
        cls = "F2" if cls == "clean" else cls
    if det["contradiction_pairs"]:
        S = min(S, 0.15)
        cls = "F3"
    if det["truncated_output"] or det.get("upstream_errors"):
        E = min(E, 0.25)
        cls = "F4"

    R = round(OMEGA["C"] * C + OMEGA["S"] * S + OMEGA["E"] * E, 4)
    return R, cls


def gate_of(R: float, cls: str, attempt_no: int) -> int:
    if cls == "clean" and R >= THETA:
        return GATE_CLEAN
    if attempt_no >= MAX_ATTEMPTS:
        return GATE_FAIL
    if cls in ("F3", "F4"):
        return GATE_REPLAN
    return GATE_LIGHT


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--attempt", type=int, required=True)
    args = ap.parse_args()

    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)
    det = read_json(run_dir, f"detection_{args.attempt}.json")

    R, cls = score(det, args.attempt)
    gate = gate_of(R, cls, args.attempt)
    gray = bool(cls == "clean" and THETA <= R < THETA + EPSILON)

    triage = {
        "attempt": args.attempt,
        "R": R,
        "class": cls,
        "gate": gate,
        "gate_name": {0: "clean", 1: "light", 2: "replan", 3: "fail"}[gate],
        "gray_zone": gray,
        "need_judge": gray,
        "need_replan": gate == GATE_REPLAN,
    }
    write_json(run_dir, f"triage_{args.attempt}.json", triage)
    trace_append(run_dir, "classify", f"classify_{args.attempt}",
                 attempt=args.attempt, R=R, classification=cls, gate=gate,
                 gray_zone=gray)
    return gate


if __name__ == "__main__":
    raise SystemExit(main())
