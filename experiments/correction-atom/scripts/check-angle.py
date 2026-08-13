#!/usr/bin/env python3
"""Deterministic per-angle checker.

Runs all deterministic checks from the case file against given text.
Emits JSON result.

Usage: check-angle.py <angle_num> <case.yml> <text>
"""
import json
import re
import sys
from pathlib import Path

import yaml


def count_bullets(text: str) -> int:
    lines = text.splitlines()
    return sum(1 for ln in lines if re.match(r"^\s*[-*•]\s+\S", ln))


def count_numbered(text: str) -> int:
    lines = text.splitlines()
    return sum(1 for ln in lines if re.match(r"^\s*\d+\.\s+\S", ln))


def word_count(text: str) -> int:
    return len(text.split())


def check_forbidden(text: str, phrases: list) -> list:
    hits = []
    lower = text.lower()
    for p in phrases:
        if p.lower() in lower:
            hits.append(p)
    return hits


def check_contains_required(text: str, required: list) -> list:
    missing = []
    for r in required:
        if r not in text:
            missing.append(r)
    return missing


def check_contains_any(text: str, options: list) -> list:
    found = []
    for o in options:
        if o in text:
            found.append(o)
    return found


def check_yaml_parsable(text: str) -> tuple:
    try:
        import yaml
        # Strip markdown code fences if present
        stripped = text.strip()
        if stripped.startswith("```"):
            lines = stripped.splitlines()
            if lines[0].startswith("```"):
                lines = lines[1:]
            if lines and lines[-1].startswith("```"):
                lines = lines[:-1]
            stripped = "\n".join(lines)
        yaml.safe_load(stripped)
        return True, None
    except Exception as e:
        return False, str(e)


def run_checks(angle_num, case, text):
    checks = case.get("deterministic_checks", {})
    results = {
        "angle": angle_num,
        "passed": True,
        "checks": {},
    }

    bullets = count_bullets(text)
    if "bullet_count_min" in checks or "bullet_count_max" in checks:
        bmin = checks.get("bullet_count_min", 0)
        bmax = checks.get("bullet_count_max", 9999)
        ok = bmin <= bullets <= bmax
        results["checks"]["bullet_count"] = {
            "value": bullets,
            "min": bmin,
            "max": bmax,
            "passed": ok,
        }
        results["passed"] = results["passed"] and ok

    wc = word_count(text)
    if "max_words" in checks or "min_words" in checks:
        wmin = checks.get("min_words", 0)
        wmax = checks.get("max_words", 99999)
        ok = wmin <= wc <= wmax
        results["checks"]["word_count"] = {
            "value": wc,
            "min": wmin,
            "max": wmax,
            "passed": ok,
        }
        results["passed"] = results["passed"] and ok

    forbidden = checks.get("forbidden_phrases", [])
    if forbidden:
        hits = check_forbidden(text, forbidden)
        ok = len(hits) == 0
        results["checks"]["forbidden_phrases"] = {
            "found": hits,
            "passed": ok,
        }
        results["passed"] = results["passed"] and ok

    required = checks.get("contains_required", [])
    if required:
        missing = check_contains_required(text, required)
        ok = len(missing) == 0
        results["checks"]["contains_required"] = {
            "missing": missing,
            "passed": ok,
        }
        results["passed"] = results["passed"] and ok

    any_of = checks.get("contains_any", [])
    if any_of:
        found = check_contains_any(text, any_of)
        ok = len(found) > 0
        results["checks"]["contains_any"] = {
            "found": found,
            "passed": ok,
        }
        results["passed"] = results["passed"] and ok

    if checks.get("yaml_parsable", False):
        ok, err = check_yaml_parsable(text)
        results["checks"]["yaml_parsable"] = {
            "passed": ok,
            "error": err,
        }
        results["passed"] = results["passed"] and ok

    return results


def main():
    if len(sys.argv) < 4:
        print("usage: check-angle.py <angle_num> <case.yml> <text>", file=sys.stderr)
        sys.exit(2)
    angle_num = int(sys.argv[1])
    case_path = Path(sys.argv[2])
    text = sys.argv[3]

    with open(case_path) as f:
        case = yaml.safe_load(f)

    result = run_checks(angle_num, case, text)
    print(json.dumps(result, indent=2))
    sys.exit(0 if result["passed"] else 1)


if __name__ == "__main__":
    main()
