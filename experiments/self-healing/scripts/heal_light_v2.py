#!/usr/bin/env python3
"""heal_light_v2.py — script-side heal for F1/F2 (no LLM call).

F1 -> corrective_prompt: appends citation/verbatim-discipline corrective
     instruction to the heal record.
F2 -> tool_reselect: schema-focused contract re-read instruction.
Reads last triage classification; writes heal_<round>.json and the
heal trace event. Round = attempts so far.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sh_lib_v2 import (  # noqa: E402
    CLASS_STRATEGY,
    load_case,
    read_json,
    run_dir_of,
    trace_append,
    write_json,
)

INSTRUCTIONS = {
    "corrective_prompt": (
        "CORRECTIVE INSTRUCTION: every citation key, method name, and "
        "number must appear verbatim in the supplied material. Re-derive "
        "each claim; delete anything you cannot trace. Report confidence "
        "only after the sweep."
    ),
    "tool_reselect": (
        "CONTRACT RE-READ: re-read the output contract field by field. "
        "Required top-level keys stay top-level; no wrapper object; "
        "types and formats exactly as specified. Re-emit the full object."
    ),
}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--round", type=int, required=True)
    args = ap.parse_args()

    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)
    triage = read_json(run_dir, f"triage_{args.round}.json")
    cls = triage["class"]
    strategy = CLASS_STRATEGY.get(cls, "corrective_prompt")

    instruction = INSTRUCTIONS[strategy]
    try:
        detection = read_json(run_dir, f"detection_{args.round}.json")
        missing = [k for k in detection.get("schema_missing_keys", []) if k]
        if missing:
            instruction += (
                " Your output is MISSING these required top-level keys: "
                f"{missing}. Emit each as a top-level key with substantive content."
            )
    except FileNotFoundError:
        pass

    heal = {
        "round": args.round,
        "kind": "script",
        "class": cls,
        "strategy": strategy,
        "instruction": instruction,
    }
    write_json(run_dir, f"heal_{args.round}.json", heal)
    trace_append(run_dir, f"heal_{strategy}", f"heal_light_{args.round}",
                 round=args.round, kind="script", strategy=strategy)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
