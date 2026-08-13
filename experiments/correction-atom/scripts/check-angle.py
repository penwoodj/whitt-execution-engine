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
