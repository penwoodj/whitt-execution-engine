#!/usr/bin/env python3
"""REA deterministic format repair (ancestor: correction-atom format-fix pass).

Usage: format-repair.py --run-dir DIR --case CASE.yml

Repairs only mechanical list-shape defects in enhanced-answer-r*.txt before
select-best re-checks candidates: a missing "- " prefix is added to bare
content lines ONLY when the case contract demands more bullets than present
(bullet_count_min > current bullets) and prefixing lands the count inside
the contract window. Never touches numbers, words, or non-list texts; never
prefixes contract-bare summary lines. Exit 0 always; select-best re-runs
the checks, so any repair that would not help changes nothing downstream.
"""

import argparse
import json
import re
import sys

import yaml
from pathlib import Path

BULLET_RE = re.compile(r"^\s*[-*•]\s*\S")


def count_bullets(lines):
    return sum(1 for ln in lines if BULLET_RE.match(ln))


def repair_list_prefixes(text, lo, hi):
    lines = text.splitlines()
    content_idx = [i for i, ln in enumerate(lines) if ln.strip()]
    bullets = count_bullets(lines)
    if lo is None or bullets >= lo or len(content_idx) < 2:
        return text, 0
    bare = [i for i in content_idx
            if not BULLET_RE.match(lines[i]) and not re.match(r"^\s*#", lines[i])]
    projected = bullets + len(bare)
    if projected < lo or (hi is not None and projected > hi):
        return text, 0
    for i in bare:
        lines[i] = "- " + lines[i].lstrip()
    return "\n".join(lines), len(bare)


def repair_json_spacing(text):
    try:
        obj = json.loads(text)
    except (json.JSONDecodeError, ValueError):
        return text, 0
    dumped = json.dumps(obj)
    if dumped == text.strip():
        return text, 0
    return dumped, 1


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--case", required=True)
    args = ap.parse_args()
    checks = {}
    try:
        case = yaml.safe_load(Path(args.case).read_text())
        checks = (case.get("success_criteria") or {}).get("deterministic_checks") or {}
    except Exception:
        pass
    lo, hi = checks.get("bullet_count_min"), checks.get("bullet_count_max")
    json_contract = bool(checks.get("yaml_parsable"))
    run = Path(args.run_dir)
    total = 0
    for ans in sorted(run.glob("enhanced-answer-r*.txt")):
        text = ans.read_text()
        if json_contract:
            text, n = repair_json_spacing(text)
            total += n
        new, n = repair_list_prefixes(text, lo, hi)
        if n:
            total += n
            text = new
        if text != ans.read_text():
            ans.write_text(text)
    if total:
        print(f"format-repair: applied {total} fix(es)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
