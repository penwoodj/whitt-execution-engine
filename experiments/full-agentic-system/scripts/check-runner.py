#!/usr/bin/env python3
"""Deterministic check executor for FAS cases.

--case-yml + --text-file → runs deterministic_checks via check_lib,
writes result JSON to --out, exit 0 iff passed (GWT routes on exit code).
"""
import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import fas_lib as L  # noqa: E402


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case-yml", required=True)
    ap.add_argument("--case", required=True)
    ap.add_argument("--text-file", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--stage", default="check")
    a = ap.parse_args()

    import yaml
    case = yaml.safe_load(Path(a.case_yml).read_text())
    checks = (case.get("success_criteria", {})
                  .get("deterministic_checks", {}))
    scoped = L.scoped_checks(checks, a.stage)
    key = L.scoped_stage_key(a.stage)
    text = ""
    tf = Path(a.text_file)
    if tf.exists():
        raw = tf.read_text()
        for cand in L.stage_candidates(raw, key):
            r = L.run_checks_safe(cand, scoped)
            if r is not None and r.get("passed"):
                text = cand
                break
        if not text:
            text = raw

    result = L.run_checks_safe(text, scoped)
    if result is None:
        result = {"passed": False, "failures": [
            {"check": "internal", "detail": "check execution error"}]}
    result["case_id"] = a.case
    result["stage"] = a.stage
    Path(a.out).parent.mkdir(parents=True, exist_ok=True)
    Path(a.out).write_text(json.dumps(result))
    sys.exit(0 if result.get("passed") else 1)


if __name__ == "__main__":
    main()
