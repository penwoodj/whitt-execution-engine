#!/usr/bin/env python3
"""detect.py — deterministic failure detection suite (Phase S, zero-LLM).

Implements the paper's detection component: execution pattern analysis +
output consistency checking, as LLM-free first-line detectors (research doc 11).

Reads attempt_N/output.json, writes attempt_N/detection.json.
Exit: 0 on successful detection run (regardless of findings).
"""
import argparse
import json
import sys
import os

import yaml

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sh_lib


def detect(output, expected_schema):
    det = {}
    text = output["text"]

    # json_parsable
    try:
        parsed = json.loads(text)
        det["json_parsable"] = True
    except (json.JSONDecodeError, ValueError):
        parsed = None
        det["json_parsable"] = False

    # schema conformance
    if isinstance(parsed, dict):
        det["schema_ok"] = all(k in parsed for k in expected_schema)
        det["schema_missing"] = [k for k in expected_schema if k not in parsed]
    else:
        det["schema_ok"] = False
        det["schema_missing"] = list(expected_schema)

    # refusal keywords
    low = text.lower()
    det["refusal_hits"] = [k for k in sh_lib.REFUSAL_KEYWORDS if k in low]

    # length bounds
    lo, hi = sh_lib.LENGTH_BOUNDS
    det["length_ok"] = lo <= len(text) <= hi

    # execution patterns (F2)
    calls = output.get("tool_calls", [])
    det["tool_error_count"] = sum(1 for c in calls if c.get("status") != "ok")
    det["tool_total"] = len(calls)

    # output consistency (F3)
    markers = output.get("reasoning_markers", [])
    det["contradiction"] = any(
        all(p in markers for p in pair) for pair in sh_lib.CONTRADICTION_PAIRS
    )

    # hallucination (F1): unexpected keys beyond the report spec = unsupported claims
    if isinstance(parsed, dict):
        det["hallucination_keys"] = sorted(set(parsed) - set(expected_schema))
    else:
        det["hallucination_keys"] = []
    det["hallucination_markers"] = [m for m in sh_lib.HALLUCINATION_MARKERS if m in text]

    # upstream propagation (F4)
    det["upstream_errors"] = output.get("upstream_errors", [])

    # soft signals
    det["confidence"] = output.get("confidence", 0.0)
    det["psc"] = output.get("psc", 0.0)
    det["confidence_degraded"] = det["confidence"] < sh_lib.CONFIDENCE_FLOOR
    det["psc_uncertain"] = det["psc"] < sh_lib.PSC_UNCERTAIN

    return det


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--attempt", type=int, required=True)
    args = ap.parse_args()

    with open(args.case) as f:
        case = yaml.safe_load(f)

    output = sh_lib.read_json(
        os.path.join(sh_lib.attempt_dir(args.run_dir, args.attempt), "output.json")
    )
    det = detect(output, case["task"]["expected_schema"])

    adir = sh_lib.attempt_dir(args.run_dir, args.attempt)
    sh_lib.write_json(os.path.join(adir, "detection.json"), det)
    sh_lib.append_trace(args.run_dir, "detect", attempt=args.attempt)
    return 0


if __name__ == "__main__":
    sys.exit(main())
