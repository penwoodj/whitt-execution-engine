#!/usr/bin/env python3
"""Exit non-zero if markdown file contains non-YAML fenced code blocks.

Used by SW4 (yaml-substructure-translation) to enforce the hard rule that
structs.md contains only ```yaml code blocks (no ```rust, ```python, ```bash).

Usage:
    python3 scripts/check-yaml-only-blocks.py <markdown-file>

Exit codes:
    0 = OK (only YAML blocks, or no code blocks at all)
    1 = VIOLATION (found non-YAML fenced block)
"""
import re
import sys
from pathlib import Path

ALLOWED = {"yaml", "yml", ""}  # empty string = no language specified (allowed)


def find_violations(text: str) -> list[tuple[int, str]]:
    """Return list of (line_number, language) for non-YAML fenced blocks."""
    violations = []
    in_fence = False
    current_lang = ""
    fence_start_line = 0

    for i, line in enumerate(text.splitlines(), 1):
        stripped = line.strip()
        # Match opening or closing fence (3+ backticks)
        m = re.match(r"^(`{3,})([\w+-]*)\s*$", stripped)
        if m:
            if not in_fence:
                # Opening fence
                in_fence = True
                current_lang = m.group(2).lower()
                fence_start_line = i
                # Check language immediately on opening
                if current_lang not in ALLOWED:
                    violations.append((fence_start_line, current_lang))
            else:
                # Closing fence
                in_fence = False
                current_lang = ""

    return violations


def main(path: str) -> int:
    p = Path(path)
    if not p.exists():
        print(f"ERROR: file not found: {path}", file=sys.stderr)
        return 2

    text = p.read_text()
    violations = find_violations(text)

    if violations:
        print(f"VIOLATION: {path} contains non-YAML fenced code blocks:")
        for lineno, lang in violations:
            print(f"  line {lineno}: ```{lang}")
        print()
        print("Only ```yaml code blocks are allowed in this document.")
        return 1

    # Count YAML blocks for confirmation
    yaml_count = len(re.findall(r"^```(?:yaml|yml)\s*$", text, re.MULTILINE))
    print(f"OK: {path}")
    print(f"  YAML code blocks found: {yaml_count}")
    return 0


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: check-yaml-only-blocks.py <markdown-file>", file=sys.stderr)
        sys.exit(2)
    sys.exit(main(sys.argv[1]))
