#!/usr/bin/env python3
"""Build structured retry feedback from a failed deterministic check.

Reads check-angle-<N>.json from a run directory and prints retry-prompt
feedback: every failed check as a numbered item with an actionable fix hint,
plus per-angle lens line and full-rewrite rules.

Evidence base: all-failures-in-feedback + actionable hints + full rewrite
beats prose feedback for <10B models (CRITIC arXiv:2305.11738,
Self-Refine arXiv:2303.17651).

Usage: build-retry-feedback.py <angle> <output_dir>
"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_angle_lib import fix_hint as fmt_hint, format_check_failure as describe


ANGLE_LENSES = {
    1: "format auditor (restructure only, preserve content)",
    2: "fact checker (fix factual mismatches against source)",
    3: "requirements tracer (every spec item present, no scope creep)",
    4: "hallucination hunter (remove fabricated content)",
    5: "consistency auditor (internal coherence, no meta language)",
}


def main():
    if len(sys.argv) < 3:
        print("usage: build-retry-feedback.py <angle> <output_dir>", file=sys.stderr)
        sys.exit(0)
    angle = int(sys.argv[1])
    chk_path = Path(sys.argv[2]) / f"check-angle-{angle}.json"

    if not chk_path.exists():
        print("Deterministic check result is MISSING. Treat the prior attempt as FAILED: "
              "re-emit the deliverable carefully following the task spec.")
        return

    try:
        chk = json.loads(chk_path.read_text())
    except Exception:
        print("Deterministic check result is UNREADABLE. Treat the prior attempt as FAILED: "
              "re-emit the deliverable carefully following the task spec.")
        return

    failed = [(n, c) for n, c in chk.get("checks", {}).items()
              if isinstance(c, dict) and c.get("passed") is not True]

    if not failed:
        print("All checks pass already. Reproduce the TEXT UNDER REVIEW content exactly, "
              "with no other text.")
        return

    lines = [
        "PRIOR ATTEMPT FAILED deterministic verification.",
        "",
        "FAILED CHECKS (fix ALL of these):",
    ]
    for i, (name, c) in enumerate(failed, 1):
        lines.append(f"{i}. {describe(name, c)} → FIX: {fmt_hint(name, c)}")
    lines += [
        "",
        f"LENS: {ANGLE_LENSES.get(angle, 'correct the text')}.",
        "RULES:",
        "- Output the FULL corrected text (complete rewrite, not a diff).",
        "- Copy real content from the task spec. Never invent.",
        "- First character must match the required output format.",
        "- No preamble. No commentary. No instructions.",
    ]
    print("\n".join(lines))


if __name__ == "__main__":
    main()
