#!/usr/bin/env python3
"""Inject shell hooks for steps whose prompts reference src/ files.

Scans each step's prompt for patterns like:
  - "Read src/foo.rs"
  - "Inspect src/bar/baz.rs"
  - "Open src/types.rs"

For each match, adds a before_step_starts shell hook:
  before_step_starts:
    - shell:
        command: cat
        args: ["src/foo.rs"]

The engine's Cycle 2 auto-inject mechanism will then load file content
into prompt automatically.
"""
import re
import sys
import yaml
from pathlib import Path

READ_PATTERNS = [
    r'(?:Read|Inspect|Open|Examine|Check|Locate|Find|Look at|View|Trace)\s+(?:the\s+)?(?:file\s+)?(src/[a-zA-Z0-9_./-]+\.(?:rs|toml|md|yml|yaml|sh|py))',
    r'(?:in|at|from|of)\s+(src/[a-zA-Z0-9_./-]+\.(?:rs|toml|md|yml|yaml|sh|py))',
]

LARGE_FILE_THRESHOLD = 20_000
LINE_RANGE_PATTERN = re.compile(
    r'(?:lines?|line(?:s)?\s+numbers?)\s*(?:~|around|approximately)?\s*(\d{2,5})\s*[-\u2013]\s*(\d{2,5})',
    re.IGNORECASE,
)


def extract_line_range(prompt: str) -> tuple[int, int] | None:
    for m in LINE_RANGE_PATTERN.finditer(prompt):
        start, end = int(m.group(1)), int(m.group(2))
        if 1 <= start < end <= 100000 and (end - start) <= 500:
            padding = max(20, (end - start) // 4)
            return max(1, start - padding), end + padding
    return None


def build_shell_spec(file_path: str, prompt: str) -> dict:
    """For large files, use sed line-range; otherwise cat entire file."""
    try:
        size = Path(file_path).stat().st_size
    except OSError:
        size = 0

    if size > LARGE_FILE_THRESHOLD:
        line_range = extract_line_range(prompt)
        if line_range:
            start, end = line_range
            return {
                'command': 'bash',
                'args': ['-c', f"sed -n '{start},{end}p' {file_path}"],
            }
        return {
            'command': 'bash',
            'args': ['-c', f"head -n 250 {file_path}"],
        }
    return {'command': 'cat', 'args': [file_path]}


def extract_referenced_files(prompt: str) -> list[str]:
    files = set()
    for pattern in READ_PATTERNS:
        for match in re.finditer(pattern, prompt, re.IGNORECASE):
            files.add(match.group(1))
    return sorted(files)


def inject_shell_hooks(workflow_path: str) -> bool:
    """Inject shell hooks for src/ references. Returns True if modified."""
    path = Path(workflow_path)
    content = path.read_text()

    # Strip markdown fences if present
    content = re.sub(r'^```(?:yaml|yml)?\s*\n', '', content, flags=re.MULTILINE)
    content = re.sub(r'\n```\s*$', '', content, flags=re.MULTILINE)

    try:
        d = yaml.safe_load(content)
    except yaml.YAMLError as e:
        print(f"ERROR: cannot parse YAML: {e}", file=sys.stderr)
        return False

    # Find steps container
    steps_container = None
    for key in ('agentic_workflow', 'workflow'):
        if key in d and isinstance(d[key], dict) and 'steps' in d[key]:
            steps_container = d[key]['steps']
            break
    if steps_container is None and 'steps' in d:
        steps_container = d['steps']
    if steps_container is None:
        print("ERROR: no steps found", file=sys.stderr)
        return False

    modified = False
    for step_name, step in steps_container.items():
        if not isinstance(step, dict):
            continue
        prompt = step.get('prompt', '')
        if not prompt:
            continue

        files = extract_referenced_files(prompt)
        if not files:
            continue

        # Skip if already has shell hook
        when = step.get('when', {})
        if isinstance(when, dict):
            bss = when.get('before_step_starts', [])
            if isinstance(bss, list):
                has_shell = any(
                    isinstance(a, dict) and 'shell' in a for a in bss
                )
                if has_shell:
                    continue

        # Inject before_step_starts shell hook
        first_file = files[0]
        if 'when' not in step or not isinstance(step.get('when'), dict):
            step['when'] = {}
        if 'before_step_starts' not in step['when']:
            step['when']['before_step_starts'] = []

        shell_spec = build_shell_spec(first_file, prompt)

        step['when']['before_step_starts'].append({
            'shell': shell_spec,
        })

        # Also: rewrite prompt to use template var
        new_prompt = prompt
        if '{{bookmarks.shell_output.stdout}}' not in new_prompt:
            new_prompt = (
                f"File content of {first_file}:\n"
                f"```\n{{{{bookmarks.shell_output.stdout}}}}\n```\n\n"
                + new_prompt
            )
            step['prompt'] = new_prompt

        print(f"  {step_name}: injected shell hook for {first_file}")
        modified = True

    if modified:
        # Write back
        backup = path.with_suffix(path.suffix + '.bak')
        if not backup.exists():
            backup.write_text(path.read_text())
        path.write_text(yaml.safe_dump(d, default_flow_style=False, sort_keys=False, width=120))
        print(f"WROTE: {path}")
    else:
        print("No modifications needed")

    return modified


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: inject-shell-hooks.py <workflow.yml>", file=sys.stderr)
        sys.exit(2)
    modified = inject_shell_hooks(sys.argv[1])
    sys.exit(0 if modified else 1)
