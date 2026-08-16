#!/usr/bin/env python3
"""REA ratchet selector.

Usage:
  select-best.py --case CASE.yml --run-dir RUND

Candidates (earlier preferred on ties): input draft, enhanced round 1,
enhanced round 2, enhanced round 3 (targeted rescue). Re-runs deterministic
checks on each candidate text.
Ratchet invariant (SAFETY.md R14): never write a final-answer.txt passing
fewer subchecks than the input draft. Hard-fail candidates are disqualified
unless every candidate hard-fails. Modes: early_exit | selected_rN |
ratchet_input | input_fallback | hard_fail_no_ship. Always exits 0; outcome
lives in select-best.json.
"""

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import yaml

from check_lib import run_checks

CANDIDATE_FILES = [
    ("input", None),
    ("r1", "enhanced-answer-r1.txt"),
    ("r2", "enhanced-answer-r2.txt"),
    ("r3", "enhanced-answer-r3.txt"),
]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir", required=True)
    args = ap.parse_args()

    run = Path(args.run_dir)
    with open(args.case) as fh:
        case = yaml.safe_load(fh)
    checks = (case.get("success_criteria") or {}).get("deterministic_checks") or {}

    final_path = run / "final-answer.txt"
    gate_marker = run / "gate-exit.json"
    if gate_marker.exists() and final_path.exists():
        result = {
            "case_id": case.get("case_id"),
            "mode": "early_exit",
            "selected": "input",
            "passed": True,
            "candidates": [],
        }
        (run / "select-best.json").write_text(json.dumps(result, indent=2))
        print(json.dumps(result))
        return 0

    candidates = []
    for name, fname in CANDIDATE_FILES:
        if fname is None:
            text = case.get("draft_response", "")
        else:
            p = run / fname
            if not p.exists():
                continue
            text = p.read_text()
        outcome = run_checks(text, checks)
        candidates.append({"name": name, "text": text, **outcome})

    clean = [c for c in candidates if not c["hard_fail"]]
    input_cand = next(c for c in candidates if c["name"] == "input")
    enhanced_clean = [c for c in clean if c["name"] != "input"]

    if enhanced_clean:
        order = {c["name"]: i for i, c in enumerate(candidates)}
        best = max(enhanced_clean,
                   key=lambda c: (c["subchecks_passed"], -order[c["name"]]))
        if (not input_cand["hard_fail"]
                and best["subchecks_passed"] < input_cand["subchecks_passed"]):
            best, mode = input_cand, "ratchet_input"
        else:
            mode = {"r1": "selected_r1", "r2": "selected_r2", "r3": "selected_r3"}[best["name"]]
    elif not input_cand["hard_fail"]:
        best, mode = input_cand, "input_fallback"
    else:
        (run / "select-best.json").write_text(json.dumps({
            "case_id": case.get("case_id"),
            "mode": "hard_fail_no_ship",
            "selected": None,
            "passed": False,
            "candidates": [{k: v for k, v in c.items() if k != "text"} for c in candidates],
        }, indent=2))
        print(json.dumps({"mode": "hard_fail_no_ship"}))
        return 0

    final_path.write_text(best["text"])
    result = {
        "case_id": case.get("case_id"),
        "mode": mode,
        "selected": best["name"],
        "passed": best["passed"],
        "subchecks_passed": best["subchecks_passed"],
        "subchecks_total": best["subchecks_total"],
        "candidates": [{k: v for k, v in c.items() if k != "text"} for c in candidates],
    }
    (run / "select-best.json").write_text(json.dumps(result, indent=2))
    print(json.dumps({k: result[k] for k in ("mode", "selected", "passed")}))
    return 0


if __name__ == "__main__":
    sys.exit(main())
