#!/usr/bin/env python3
"""classify.py — reliability scoring + failure classification (Phase S, zero-LLM).

Paper 2605.06737 component 2+3:
  R = ω1*C + ω2*S + ω3*E   (θ = 0.65)
  classification priority: F4 > F2 > F3 > F1 (see docs/04-CASE-SUITE-SPEC.md)

Reads attempt_N/detection.json, writes attempt_N/verdict.json + trace event.
EXIT CODE = classification (0 clean, 1 F1, 2 F2, 3 F3, 4 F4) — GWT routes on this.
"""
import argparse
import sys
import os

import yaml

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sh_lib


def score(det):
    """Deterministic C/S/E components from detection signals."""
    # C — consistency: self-consistency (psc), contradiction + confidence degradation caps
    c = det["psc"]
    if det["contradiction"]:
        c = min(c, 0.25)
    if det["confidence_degraded"]:
        c = min(c, 0.45)

    # S — semantic correctness: schema + parseability + hallucination-free
    s = 1.0
    if not det["json_parsable"]:
        s -= 0.5
    if not det["schema_ok"]:
        s -= 0.3
    if det["hallucination_keys"] or det["hallucination_markers"]:
        s = min(s, 0.2)   # unsupported claim = hard semantic violation (paper F1)
    if det["refusal_hits"]:
        s -= 0.5
    s = max(0.0, min(1.0, s))

    # E — execution success rate: tool calls + upstream dependency health
    if det["tool_total"] > 0:
        e = (det["tool_total"] - det["tool_error_count"]) / det["tool_total"]
    else:
        e = 1.0
    if det["upstream_errors"]:
        e = 0.0   # upstream failure = execution base compromised (paper F4)
    if not det["length_ok"]:
        e = min(e, 0.5)

    r = sh_lib.W_C * c + sh_lib.W_S * s + sh_lib.W_E * e
    return round(r, 4), round(c, 4), round(s, 4), round(e, 4)


def classify(det, r):
    if r >= sh_lib.THETA:
        return "clean"
    # priority: structural/execution failures dominate (docs/04)
    if det["upstream_errors"]:
        return "F4"
    if det["tool_error_count"] > 0 or not det["json_parsable"]:
        return "F2"
    if det["contradiction"] or det["psc_uncertain"]:
        return "F3"
    if det["hallucination_keys"] or det["hallucination_markers"] or det["confidence_degraded"] or not det["schema_ok"]:
        return "F1"
    return "F3"  # residual reasoning failure (paper treats inconsistency as residual)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--attempt", type=int, required=True)
    args = ap.parse_args()

    with open(args.case) as f:
        case = yaml.safe_load(f)

    adir = sh_lib.attempt_dir(args.run_dir, args.attempt)
    det = sh_lib.read_json(os.path.join(adir, "detection.json"))

    r, c, s, e = score(det)
    cls = classify(det, r)

    verdict = {
        "attempt": args.attempt,
        "R": r,
        "components": {"C": c, "S": s, "E": e},
        "weights": {"W_C": sh_lib.W_C, "W_S": sh_lib.W_S, "W_E": sh_lib.W_E},
        "theta": sh_lib.THETA,
        "classification": cls,
        "ground_truth_injected": det.get("injected_failure"),
    }
    sh_lib.write_json(os.path.join(adir, "verdict.json"), verdict)
    sh_lib.append_trace(
        args.run_dir, "classify", attempt=args.attempt, R=r,
        classification=cls, C=c, S=s, E=e,
    )
    return sh_lib.CLASS_EXIT[cls]


if __name__ == "__main__":
    sys.exit(main())
