#!/usr/bin/env python3
"""Deterministic verifier for atom outputs. No model involved.

Checks:
1. Output non-empty (>20 chars)
2. Not a refusal pattern
3. Reasonable length (<5000 chars)
4. Contains expected format markers (if specified via ATOM_TASK_KIND env)

Exit codes:
  0 = pass
  1 = empty/too short
  2 = refusal detected
  3 = too long
  4 = format mismatch

Reads draft from argv[1] (or stdin if empty).
"""
import os
import re
import sys


REFUSAL_PATTERNS = [
    r" I (?:can't|cannot|won't|will not)",
    r"as an ai",
    r"i'm not able to",
    r"i am not able to",
    r"i don't feel comfortable",
    r"is beyond my capabilities",
]


def main():
    draft = sys.argv[1] if len(sys.argv) > 1 else sys.stdin.read()
    draft = draft.strip()

    if len(draft) < 20:
        print("FAIL: too short")
        sys.exit(1)

    lower = draft.lower()
    for pat in REFUSAL_PATTERNS:
        if re.search(pat, lower):
            print(f"FAIL: refusal pattern matched: {pat}")
            sys.exit(2)

    if len(draft) > 5000:
        print("FAIL: too long")
        sys.exit(3)

    task_kind = os.environ.get("ATOM_TASK_KIND", "")
    if task_kind == "bullets":
        if draft.count("\n- ") < 2 and draft.count("\n* ") < 2 and draft.count("\n• ") < 2:
            print("FAIL: expected bullet list (3+ items)")
            sys.exit(4)
    elif task_kind == "yaml":
        if ":" not in draft or "\n" not in draft:
            print("FAIL: expected YAML structure")
            sys.exit(4)
    elif task_kind == "numbered":
        if not re.search(r"^\s*1\.", draft, re.MULTILINE):
            print("FAIL: expected numbered list")
            sys.exit(4)

    print("PASS")
    sys.exit(0)


if __name__ == "__main__":
    main()
