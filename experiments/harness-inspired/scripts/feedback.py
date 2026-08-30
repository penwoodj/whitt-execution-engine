#!/usr/bin/env python3
"""Leak-safe feedback generator. Reads check-result JSON, emits feedback
that names failing checks and categories but NEVER expected values.

Usage: feedback.py --result FILE [--out FILE]
Exit 0 always (feedback is informational).
"""
import argparse, json, sys
from pathlib import Path

CATEGORY_RULES = [
    ("json_exact", "FORMAT"),
    ("json_valid", "FORMAT"),
    ("yaml_valid", "FORMAT"),
    ("max_words", "FORMAT"),
    ("min_words", "FORMAT"),
    ("max_lines", "FORMAT"),
    ("min_lines", "FORMAT"),
    ("exact_match", "CONTENT"),
    ("contains", "CONTENT"),
    ("regex", "CONTENT"),
    ("forbidden", "LEAK"),
    ("file_exists", "FORMAT"),
]

REDACT = ["expected", "golden", "answer", "correct"]


def classify(check_name):
    for prefix, cat in CATEGORY_RULES:
        if check_name.startswith(prefix):
            return cat
    return "CONTENT"


def build_feedback(result, observed):
    fails = [c for c in result.get("checks", []) if not c.get("pass")]
    if not fails:
        return "ALL CHECKS PASSED. No feedback needed."

    lines = ["Attempt failed checks:"]
    for c in fails:
        name = c.get("name", "unknown")
        cat = classify(name)
        ev = str(c.get("evidence", ""))
        for word in REDACT:
            if word.lower() in ev.lower():
                ev = "details withheld"
                break
        else:
            if len(ev) > 60:
                ev = ev[:57] + "..."
        lines.append(f"- {name} [{cat}]: {ev}")

    if observed:
        lines.append(f"Your output was: {observed}")
    lines.append("Re-derive every value from the task rules. Entities you do not")
    lines.append("manage never count. Output ONLY corrected JSON.")
    return "\n".join(lines)


def observed_summary(artifact_path, limit=200):
    if not artifact_path:
        return None
    try:
        text = Path(artifact_path).read_text().strip()
    except (FileNotFoundError, OSError):
        return None
    return " ".join(text.split())[:limit]


def main():
    p = argparse.ArgumentParser(description="Leak-safe feedback generator")
    p.add_argument("--result", required=True, help="check-result JSON path")
    p.add_argument("--artifact", help="artifact path (adds observed values)")
    p.add_argument("--out", help="write feedback to file")
    args = p.parse_args()

    try:
        result = json.loads(Path(args.result).read_text())
    except (json.JSONDecodeError, FileNotFoundError) as e:
        print(f"ERROR: cannot read result: {e}", file=sys.stderr)
        sys.exit(1)

    feedback = build_feedback(result, observed_summary(args.artifact))
    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(feedback)
    print(feedback)
    sys.exit(0)


if __name__ == "__main__":
    main()
