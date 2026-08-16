#!/usr/bin/env python3
"""Deterministic format fixer — replaces the 0.5B angle-1 LLM stage.

Rules (format only; factual fixes belong to the LLM cascade):
- drop lines containing a forbidden phrase when the line is short (<15 words)
  → removes preambles/closings/meta chatter
- bullet cases: when enough bullets survive, keep only bullet lines
  (continuation lines of a bullet stay attached)
- in-bullet forbidden phrases on long content lines are KEPT so the
  LLM angles see and fix the factual defect
- collapse runs of blank lines

Usage: format-fix.py <case.yml> <out_file>
"""
import re
import sys
from pathlib import Path

import yaml

BULLET_RE = re.compile(r"^\s*[-*•]\s+\S")
SHORT_LINE_WORDS = 15


def fix_text(text: str, checks: dict) -> str:
    forbidden = [p.lower() for p in checks.get("forbidden_phrases", [])]

    kept = []
    prev_bullet = False
    for line in text.splitlines():
        is_bullet = bool(BULLET_RE.match(line))
        is_continuation = prev_bullet and line[:1] in (" ", "\t") and line.strip()
        if not is_bullet and not is_continuation:
            low = line.lower()
            if forbidden and any(p in low for p in forbidden) \
                    and len(line.split()) < SHORT_LINE_WORDS:
                prev_bullet = False
                continue
        kept.append(line)
        prev_bullet = is_bullet or is_continuation
    text = "\n".join(kept)

    wants_bullets = "bullet_count_min" in checks or "bullet_count_max" in checks
    if wants_bullets:
        bmin = checks.get("bullet_count_min", 0)
        bullets = [l for l in text.splitlines() if BULLET_RE.match(l)]
        if len(bullets) >= max(bmin, 1):
            out, in_bullet = [], False
            for line in text.splitlines():
                if BULLET_RE.match(line):
                    out.append(line)
                    in_bullet = True
                elif in_bullet and line.strip() and not BULLET_RE.match(line) \
                        and (line.startswith((" ", "\t")) or out and out[-1].endswith((";", ","))):
                    out.append(line)
                else:
                    in_bullet = False
            text = "\n".join(out)

    text = re.sub(r"\n{3,}", "\n\n", text).strip()
    return text


def main():
    if len(sys.argv) < 3:
        print("usage: format-fix.py <case.yml> <out_file>", file=sys.stderr)
        sys.exit(2)
    case = yaml.safe_load(Path(sys.argv[1]).read_text())
    fixed = fix_text(case.get("broken_output", ""), case.get("deterministic_checks", {}))
    Path(sys.argv[2]).write_text(fixed + "\n")


if __name__ == "__main__":
    main()
