#!/usr/bin/env python3
"""Emit formatted context block from case YAML for use in prompts.

Reads case file, outputs a single text block containing TASK SPEC,
AUXILIARY, and BROKEN OUTPUT sections. This becomes {{bookmarks.shell_output.stdout}}
for the workflow's first angle.

Usage: emit-context.py <case.yml>
"""
import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    print("PyYAML required: pip install pyyaml", file=sys.stderr)
    sys.exit(2)

if len(sys.argv) < 2:
    print("usage: emit-context.py <case.yml>", file=sys.stderr)
    sys.exit(2)

case_path = Path(sys.argv[1])
if not case_path.exists():
    print(f"case file not found: {case_path}", file=sys.stderr)
    sys.exit(2)

with open(case_path) as f:
    case = yaml.safe_load(f)

print("=== TASK SPEC ===")
print(case.get("task_spec", "").strip())
print()
print("=== AUXILIARY (source of truth) ===")
print(case.get("auxiliary", "").strip())
print()
print("=== BROKEN OUTPUT (to correct) ===")
print(case.get("broken_output", "").strip())
