#!/usr/bin/env python3
"""Fix common YAML issues in SW5 generated workflow.yml files.

Handles:
- Unquoted GWT expressions with == operator
- Inline array save_to syntax
"""
import re
import sys
from pathlib import Path


def fix_gwt_unquoted_equals(content: str) -> str:
    """Fix `given: "x" == "y"` → `given: '"x" == "y"'`."""
    # Match: given: "..." == "..."  (possibly followed by more comparisons)
    # The issue is the value after given: contains unquoted ==
    lines = content.split('\n')
    fixed = []
    for line in lines:
        # Pattern: leading whitespace + given: <stuff with == outside quotes>
        m = re.match(r'^(\s*given:\s+)(.+)$', line)
        if m and '==' in m.group(2):
            prefix = m.group(1)
            expr = m.group(2)
            # Only wrap if not already wrapped in single quotes
            if not (expr.startswith("'") and expr.endswith("'")):
                # Escape any single quotes in the expression
                escaped = expr.replace("'", "''")
                line = f"{prefix}'{escaped}'"
        fixed.append(line)
    return '\n'.join(fixed)


def fix_inline_save_to(content: str) -> str:
    """Fix `save_to: [bookmark, "path"]` → indented list."""
    # Match: - save_to: [name, "path"]
    def replace_save_to(m):
        indent = m.group(1)
        bookmark = m.group(2)
        path = m.group(3)
        return f"{indent}- save_to:\n{indent}    - {bookmark}\n{indent}    - {path}"

    content = re.sub(
        r'^(\s*)- save_to:\s*\[(\w+),\s*("[^"]+"|\'[^\']+\')\]',
        replace_save_to,
        content,
        flags=re.MULTILINE,
    )
    return content


def main():
    if len(sys.argv) != 2:
        print("Usage: fix-yaml.py <workflow.yml>", file=sys.stderr)
        sys.exit(1)

    path = Path(sys.argv[1])
    if not path.exists():
        print(f"File not found: {path}", file=sys.stderr)
        sys.exit(1)

    content = path.read_text()
    original = content

    content = fix_gwt_unquoted_equals(content)
    content = fix_inline_save_to(content)

    if content != original:
        backup = path.with_suffix('.yml.bak')
        backup.write_text(original)
        path.write_text(content)
        print(f"Fixed {path} (backup: {backup})")
    else:
        print(f"No changes needed in {path}")

    # Validate
    import yaml
    try:
        yaml.safe_load(content)
        print("✅ Valid YAML after fixes")
    except yaml.YAMLError as e:
        print(f"❌ Still invalid YAML: {e}", file=sys.stderr)
        sys.exit(2)


if __name__ == '__main__':
    main()
