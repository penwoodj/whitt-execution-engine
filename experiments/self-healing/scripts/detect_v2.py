#!/usr/bin/env python3
"""detect_v2.py — deterministic detection suite over one spoofed attempt.

LLM-free first line (guardrails doc 11): schema conformance, key-set
diff, contradiction markers, truncation, tool errors, verbalized
confidence and pseudo-self-consistency extraction.

Writes detection_<n>.json. Exit 0 always; findings feed triage.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sh_lib_v2 import (  # noqa: E402
    CONFIDENCE_FLOOR,
    LENGTH_BOUNDS,
    PSC_UNCERTAIN,
    load_case,
    read_json,
    run_dir_of,
    trace_append,
    write_json,
)


def detect(attempt: dict, case: dict) -> dict:
    schema = case["task"]["expected_schema"]
    required = set(schema["required"])
    optional = set(schema.get("optional", []))
    meta = attempt.get("_meta", {})

    keys = {k for k in attempt if k != "_meta"}
    hallucination_keys = sorted(
        k for k in keys - required - optional if not k.startswith("_")
    )
    missing_keys = sorted(required - keys)
    wrapped = list(attempt.keys()) == ["document", "_meta"] or (
        set(attempt.keys()) - {"_meta"} == {"document"}
    )
    truncated = bool(meta.get("truncated"))
    tool_errors = int(meta.get("tool_errors", 0))
    contradictions = list(meta.get("contradiction_pairs", []))
    confidence = float(meta.get("confidence", 0.75))
    psc = float(meta.get("self_consistency", 0.85))
    length = len(json.dumps(attempt))

    findings = {
        "hallucination_keys": hallucination_keys,
        "schema_missing_keys": missing_keys,
        "schema_nested_wrong": wrapped,
        "truncated_output": truncated,
        "tool_errors": tool_errors,
        "contradiction_pairs": contradictions,
        "confidence": confidence,
        "pseudo_self_consistency": psc,
        "confidence_degraded": confidence < CONFIDENCE_FLOOR,
        "psc_uncertain": psc < PSC_UNCERTAIN,
        "length_out_of_bounds": not (LENGTH_BOUNDS[0] <= length <= LENGTH_BOUNDS[1]),
    }
    return findings


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--attempt", type=int, required=True)
    args = ap.parse_args()

    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)
    attempt = read_json(run_dir, f"attempt_{args.attempt}.json")
    findings = detect(attempt, case)
    write_json(run_dir, f"detection_{args.attempt}.json", findings)
    trace_append(run_dir, "detect", f"detect_{args.attempt}", attempt=args.attempt,
                 flags_true=sum(1 for v in findings.values() if v is True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
