#!/usr/bin/env python3
"""Fix common YAML issues in SW5 generated workflow.yml files.

Handles:
- Unquoted GWT expressions with == operator
- Inline array save_to syntax
- Markdown fence wrappers (```yaml ... ```)
- Mis-indented - log: as sibling of - save_to: (model puts at 12 sp, needs 10 sp)
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


def strip_markdown_fences(content: str) -> str:
    """Remove ```yaml ... ``` wrappers that model sometimes adds despite instructions."""
    lines = content.split('\n')
    out = []
    in_fence = False
    for line in lines:
        stripped = line.strip()
        if stripped.startswith('```yaml') or stripped.startswith('```yml') or stripped == '```yaml' or stripped == '```yml':
            in_fence = True
            continue
        if stripped == '```' and in_fence:
            in_fence = False
            continue
        out.append(line)
    return '\n'.join(out)


def fix_log_after_save_to_indent(content: str) -> str:
    """Fix `- log:` placed at wrong indent after `- save_to:` block.

    Bug pattern (12-space indent — WRONG):
        after_step_succeeds:
          - save_to:
              - "name"
              - "path"
            - log:           <- indent 12, parser treats as save_to sub-item
                ...

    Correct (10-space indent):
        after_step_succeeds:
          - save_to:
              - "name"
              - "path"
          - log:             <- indent 10, sibling of - save_to:
                ...

    Also handles when there's no save_to args (single-line - save_to: then wrong-indent - log:).
    """
    lines = content.split('\n')
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        # Detect "- save_to:" line — capture its indent
        m = re.match(r'^(\s*)- save_to:', line)
        if m:
            save_to_indent = len(m.group(1))
            out.append(line)
            i += 1
            # Skip save_to args (deeper indent) and any nested keys under save_to
            while i < len(lines):
                next_line = lines[i]
                # Empty line: keep, continue
                if next_line.strip() == '':
                    out.append(next_line)
                    i += 1
                    continue
                # Compute leading whitespace
                next_stripped = next_line.lstrip()
                if not next_stripped:
                    out.append(next_line)
                    i += 1
                    continue
                next_indent = len(next_line) - len(next_stripped)
                # Save_to ARGS are quoted strings or bare identifiers as list items:
                #   - "bookmark_name"
                #   - "./path/to/file"
                #   - bookmark_name
                # Distinguish from sibling ACTION items like "- log:", "- shell:" which
                # have a colon (key) and are at deeper indent due to the bug.
                is_string_arg = bool(re.match(r'^- ("[^"]+"|\'[^\']+\'|[A-Za-z_][A-Za-z0-9_]*)\s*$', next_stripped))
                if next_indent > save_to_indent and is_string_arg:
                    out.append(next_line)
                    i += 1
                    continue
                # If next line is a key under save_to (deeper indent, not list, no colon-suffix bug):
                # e.g. nested config
                if next_indent > save_to_indent and not next_stripped.startswith('- '):
                    out.append(next_line)
                    i += 1
                    continue
                # If next line is "- log:" or "- shell:" etc at exactly save_to_indent+2
                # (the bug pattern): rewrite to save_to_indent
                m2 = re.match(r'^(\s*)- (log|shell|gwt|notify|fail|skip_step|skip_remaining|bookmark|append_to|route_to):', next_line)
                if m2:
                    this_indent = len(m2.group(1))
                    expected_bug_indent = save_to_indent + 2
                    if this_indent == expected_bug_indent:
                        # Fix: dedent to save_to_indent
                        fixed_line = ' ' * save_to_indent + next_stripped
                        out.append(fixed_line)
                        i += 1
                        continue
                # Otherwise: this line is at same or lesser indent — end of save_to block
                break
            continue
        out.append(line)
        i += 1
    return '\n'.join(out)


def fix_save_to_map_to_list(content: str) -> str:
    """Convert `save_to: { bookmark: path }` (map form) to `save_to: [bookmark, path]` (list form).

    Schema expects SaveToAction as a 2-element list [bookmark_name, file_path].
    Model sometimes emits as a map with single key:value pair.

    Bug pattern:
        - save_to:
            http_client_output: "./outputs/x.rs"

    Correct:
        - save_to:
            - http_client_output
            - "./outputs/x.rs"
    """
    lines = content.split('\n')
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        m = re.match(r'^(\s*)- save_to:\s*$', line)
        if m:
            indent = len(m.group(1))
            out.append(line)
            i += 1
            # Next non-empty line should be the args
            while i < len(lines) and lines[i].strip() == '':
                out.append(lines[i])
                i += 1
            if i >= len(lines):
                continue
            next_line = lines[i]
            next_stripped = next_line.lstrip()
            next_indent = len(next_line) - len(next_stripped)
            # If next line is at deeper indent AND is a "key: value" (map form):
            m_kv = re.match(r'^(\w[\w-]*)\s*:\s*(.+)$', next_stripped)
            if next_indent > indent and m_kv and not next_stripped.startswith('- '):
                bookmark = m_kv.group(1)
                path = m_kv.group(2).strip()
                # Rewrite as list form
                inner_indent = ' ' * (indent + 4)
                out.append(f"{inner_indent}- {bookmark}")
                out.append(f"{inner_indent}- {path}")
                i += 1
                continue
            # Otherwise leave alone (already list form or other)
            continue
        out.append(line)
        i += 1
    return '\n'.join(out)


def fix_steps_list_to_map(content: str) -> str:
    """Convert `steps: - step_name: ...` (list form) to `steps: step_name: ...` (map form).

    Schema expects `agentic_workflow.steps` as a map keyed by step name.
    Model sometimes emits as a list of single-key maps.

    Bug pattern:
        steps:
          - step_t1_...:
              generative_entity: ...

    Correct:
        steps:
          step_t1_...:
            generative_entity: ...
    """
    lines = content.split('\n')
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        # Match `steps:` line (with leading whitespace)
        m = re.match(r'^(\s*)steps:\s*$', line)
        if m:
            steps_indent = len(m.group(1))
            out.append(line)
            i += 1
            # Now look at following lines — process all step entries
            while i < len(lines):
                cur = lines[i]
                if cur.strip() == '':
                    out.append(cur)
                    i += 1
                    continue
                cur_stripped = cur.lstrip()
                if not cur_stripped:
                    out.append(cur)
                    i += 1
                    continue
                cur_indent = len(cur) - len(cur_stripped)
                # If indent <= steps_indent, we've left the steps block
                if cur_indent <= steps_indent:
                    break
                # Check if this is `- step_name:` (list-form bug)
                m_list = re.match(r'^(\s*)- (step_[a-zA-Z0-9_]+:)\s*$', cur)
                if m_list:
                    list_indent = len(m_list.group(1))
                    step_name = m_list.group(2)
                    # Rewrite: remove "- " prefix, keep step_name
                    new_indent = list_indent
                    out.append(' ' * new_indent + step_name)
                    i += 1
                    # Now dedent all sub-lines of this step by 2 spaces (because we removed `- `)
                    while i < len(lines):
                        sub = lines[i]
                        if sub.strip() == '':
                            out.append(sub)
                            i += 1
                            continue
                        sub_stripped = sub.lstrip()
                        if not sub_stripped:
                            out.append(sub)
                            i += 1
                            continue
                        sub_indent = len(sub) - len(sub_stripped)
                        # If sub_indent <= list_indent, end of this step
                        if sub_indent <= list_indent:
                            break
                        # Dedent by 2 (relative to the `- ` removal)
                        if sub_indent >= 2:
                            new_sub = ' ' * (sub_indent - 2) + sub_stripped
                        else:
                            new_sub = sub_stripped
                        out.append(new_sub)
                        i += 1
                    continue
                # Otherwise: not a list-form step, copy as-is (could be map-form already)
                # But we still need to consume the whole steps block
                # Check: is this a map-form step (no `- `)?
                if cur_indent == steps_indent + 2 and re.match(r'^\s*step_[a-zA-Z0-9_]+:', cur):
                    # Map form (correct). Copy line.
                    out.append(cur)
                    i += 1
                    # Copy all sub-lines
                    while i < len(lines):
                        sub = lines[i]
                        if sub.strip() == '':
                            out.append(sub)
                            i += 1
                            continue
                        sub_stripped = sub.lstrip()
                        sub_indent = len(sub) - len(sub_stripped) if sub_stripped else 0
                        if sub_indent <= steps_indent + 2:
                            break
                        out.append(sub)
                        i += 1
                    continue
                # Not a recognized step format — just copy
                out.append(cur)
                i += 1
            continue
        out.append(line)
        i += 1
    return '\n'.join(out)


def fix_save_to_string_form(content: str) -> str:
    """Convert `- save_to: "path"` (string form) to `- save_to: [bookmark, path]` (list form).

    Bug: model sometimes emits save_to with just a path string.
        - save_to: "src/benchmark/mod.rs"

    Correct:
        - save_to:
            - step_output
            - "./outputs/step.rs"
    """
    def replace(m):
        indent = m.group(1)
        path = m.group(2)
        # Derive bookmark name from path (last segment, sanitized)
        import os
        basename = os.path.basename(path.replace('.rs', '').replace('.yml', '').replace('.md', ''))
        bookmark = f"step_output_{basename}" if basename else "step_output"
        return f"{indent}- save_to:\n{indent}    - {bookmark}\n{indent}    - {path}"

    # Match: - save_to: "..." or - save_to: '...' (string values, not lists/maps)
    # IMPORTANT: use [ \t] not \s, to avoid matching across newlines.
    # The value must be on SAME line as `- save_to:`.
    content = re.sub(
        r'^([ \t]*)- save_to:[ \t]+("[^"]+"|\'[^\']+\'|[^\s\[\{\n].*?)[ \t]*$',
        replace,
        content,
        flags=re.MULTILINE,
    )
    return content


def fix_save_to_outside_when(content: str) -> str:
    """Move `save_to:`/`log:` blocks at step-level INTO `when.after_step_succeeds:`.

    Bug: model sometimes emits hooks directly under step, not nested in when block:
        step_t1_...:
          generative_entity: ...
          prompt: ...
          save_to:           <- WRONG: step-level
            - item
          log:               <- WRONG: step-level
            ...

    Correct:
        step_t1_...:
          generative_entity: ...
          prompt: ...
          when:
            after_step_succeeds:
              - save_to: ...
              - log: ...
    """
    lines = content.split('\n')
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        out.append(line)
        # Detect step-level misplaced hook: line is at step_indent+2 and starts with `save_to:` or `log:`
        # Step indent is 4 typically (under `steps:` at 2)
        m_step = re.match(r'^(\s*)step_[a-z0-9_]+:\s*$', line)
        if m_step:
            step_indent = len(m_step.group(1))
            # Collect all subsequent lines of this step
            step_lines = []
            i += 1
            misplaced_hooks = []
            other_lines = []
            has_when = False
            while i < len(lines):
                cur = lines[i]
                if cur.strip() == '':
                    step_lines.append(('blank', cur))
                    i += 1
                    continue
                cur_stripped = cur.lstrip()
                cur_indent = len(cur) - len(cur_stripped)
                if cur_indent <= step_indent:
                    break
                # Check if this is a misplaced hook (save_to: or log: at step_indent+2)
                if cur_indent == step_indent + 2 and re.match(r'^(save_to|log):\s*$', cur_stripped):
                    misplaced_hooks.append((cur, cur_indent))
                    i += 1
                    # Collect sub-lines of this hook
                    while i < len(lines):
                        sub = lines[i]
                        if sub.strip() == '':
                            misplaced_hooks.append(('blank', sub))
                            i += 1
                            continue
                        sub_stripped = sub.lstrip()
                        sub_indent = len(sub) - len(sub_stripped) if sub_stripped else 0
                        if sub_indent <= cur_indent:
                            break
                        misplaced_hooks.append((sub, sub_indent))
                        i += 1
                    continue
                # Check for when: block (correct location)
                if cur_indent == step_indent + 2 and re.match(r'^when:\s*$', cur_stripped):
                    has_when = True
                step_lines.append(('line', cur))
                i += 1
            # Emit step_lines first, then if misplaced_hooks and no when:, add when block
            for kind, l in step_lines:
                out.append(l)
            if misplaced_hooks and not has_when:
                # Add when block with the misplaced hooks
                when_indent = ' ' * (step_indent + 2)
                hook_indent = ' ' * (step_indent + 4)
                item_indent = ' ' * (step_indent + 6)
                out.append(f"{when_indent}when:")
                out.append(f"{hook_indent}after_step_succeeds:")
                # Re-emit misplaced hooks as list items
                for entry, e_indent in misplaced_hooks:
                    if entry == 'blank' or entry.strip() == '':
                        continue
                    e_stripped = entry.lstrip()
                    # If this is the hook header (save_to: or log:), prepend "- "
                    if re.match(r'^(save_to|log):\s*$', e_stripped):
                        out.append(f"{item_indent}- {e_stripped}")
                    else:
                        # Sub-line of hook — preserve relative indent (add 2 for the "- " prefix)
                        rel_indent = e_indent - (step_indent + 2)
                        new_indent = item_indent + ' ' * rel_indent + '  '
                        out.append(f"{new_indent}{e_stripped}")
            continue
        i += 1
    return '\n'.join(out)


def fix_save_to_null_map_pattern(content: str) -> str:
    """Fix save_to blocks with `- null` followed by map item `path: true`.

    Specific bug pattern from prompt-12 (model produced malformed save_to):
        - save_to:
            - bookmark_name
            - null
            "./path/to/file": true

    Correct:
        - save_to:
            - bookmark_name
            - "./path/to/file"
    """
    lines = content.split('\n')
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        m = re.match(r'^([ \t]*)- save_to:\s*$', line)
        if m:
            save_to_indent = len(m.group(1))
            out.append(line)
            i += 1
            # Collect args (deeper indent)
            args = []
            while i < len(lines):
                cur = lines[i]
                if cur.strip() == '':
                    out.append(cur)
                    i += 1
                    continue
                cur_stripped = cur.lstrip()
                cur_indent = len(cur) - len(cur_stripped)
                if cur_indent <= save_to_indent:
                    break
                args.append((cur, cur_indent, cur_stripped))
                i += 1
            # Check if pattern matches: contains `- null` or `: null` or `: true`
            # indicating malformed save_to from prompt-12 generator output
            is_broken = (
                len(args) >= 2 and
                any(a[2] == '- null' or ': null' in a[2] or ': true' in a[2] for a in args)
            )
            if is_broken:
                # 4-arg pattern: [step_output_null, null, real_bookmark, real_path]
                # → keep only the real pair (args[2], args[3])
                if (
                    len(args) == 4 and
                    args[0][2].startswith('- step_output') and
                    args[1][2] == '- null' and
                    not args[2][2].startswith('- null') and
                    not args[3][2].startswith('- null')
                ):
                    inner_indent = ' ' * (save_to_indent + 4)
                    out.append(f"{inner_indent}{args[2][2]}")
                    out.append(f"{inner_indent}{args[3][2]}")
                    continue
                # Generic: keep first bookmark + extract first path
                bookmark = args[0][2].lstrip('- ').strip()
                path = None
                for a_line, _, a_stripped in args[1:]:
                    m_path = re.match(r'^("[^"]+"|\'[^\']+\')', a_stripped)
                    if m_path:
                        path = m_path.group(1)
                        break
                    m_kv = re.match(r'^("[^"]+"|\'[^\']+\')\s*:\s*true\s*$', a_stripped)
                    if m_kv:
                        path = m_kv.group(1)
                        break
                if path:
                    inner_indent = ' ' * (save_to_indent + 4)
                    out.append(f"{inner_indent}- {bookmark}")
                    out.append(f"{inner_indent}- {path}")
                    continue
            for a_line, _, _ in args:
                out.append(a_line)
            continue
        out.append(line)
        i += 1
    return '\n'.join(out)


def fix_save_to_split_pair(content: str) -> str:
    pattern = re.compile(
        r'([ \t]+)- save_to:\n'
        r'[ \t]+- step_output_null\n'
        r'[ \t]+- null\n'
        r'\1- ([A-Za-z_][A-Za-z0-9_]*)\n'
        r'\1- (\.[^\n]+)',
    )
    return pattern.sub(
        lambda m: f"{m.group(1)}- save_to:\n{m.group(1)}    - {m.group(2)}\n{m.group(1)}    - {m.group(3)}",
        content,
    )


def fix_log_null_with_siblings(content: str) -> str:
    r"""Convert `- <action>: null` + sibling keys to `- <action>:` with nested keys.

    Handles log, shell, save_to, bookmark, notify, fail actions where model
    emitted `<action>: null` then sibling keys (command, args, to_file_path,
    event_fields, etc.) at same indent. Serde rejects: 'expected end of
    mapping after enum variant value' or 'missing field'.

    Pattern: `- log: null\n  to_file_path: X\n  event_fields: [...]`
    Fix: `- log:\n      to_file_path: X\n      event_fields: [...]`
    """
    lines = content.split('\n')
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        m = re.match(r'^([ \t]+)- (log|shell|save_to|bookmark|notify|fail|route_to|skip_step|skip_remaining|append_to): null\s*$', line)
        if m:
            base_indent = m.group(1)
            action = m.group(2)
            item_indent = base_indent + '  '
            nested_indent = base_indent + '      '
            out.append(f"{base_indent}- {action}:")
            i += 1
            while i < len(lines):
                cur = lines[i]
                if cur.strip() == '':
                    out.append(cur)
                    i += 1
                    continue
                cur_stripped = cur.lstrip()
                cur_indent = len(cur) - len(cur_stripped)
                if cur_indent <= len(base_indent):
                    break
                if cur_indent == len(item_indent):
                    out.append(f"{nested_indent}{cur_stripped}")
                else:
                    delta = cur_indent - len(item_indent)
                    out.append(f"{nested_indent}{(' ' * (delta if delta > 0 else 0))}{cur_stripped}")
                i += 1
            continue
        out.append(line)
        i += 1
    return '\n'.join(out)


def fix_save_to_unwritable_paths(content: str) -> str:
    """Rewrite save_to paths that won't be writable in test context.

    Bug patterns:
    - Paths starting with ~/ (home dir, may not expand correctly)
    - Paths to src/ or other nonexistent project dirs
    - Bare identifiers as second arg (should be path string)

    Convert all to ./outputs/<basename> to ensure writability.
    """
    lines = content.split('\n')
    out = []
    in_save_to_args = False
    save_to_indent = 0
    arg_index = 0
    for line in lines:
        m_st = re.match(r'^([ \t]*)- save_to:\s*$', line)
        if m_st:
            in_save_to_args = True
            save_to_indent = len(m_st.group(1))
            arg_index = 0
            out.append(line)
            continue
        if in_save_to_args:
            if line.strip() == '':
                out.append(line)
                continue
            stripped = line.lstrip()
            indent = len(line) - len(stripped)
            if indent <= save_to_indent:
                in_save_to_args = False
                out.append(line)
                continue
            arg_index += 1
            # Second arg (arg_index==2) is the path
            if arg_index == 2:
                m_path = re.match(r'^- (.+)$', stripped)
                if m_path:
                    raw_path = m_path.group(1).strip()
                    if (raw_path.startswith('"') and raw_path.endswith('"')) or \
                       (raw_path.startswith("'") and raw_path.endswith("'")):
                        unquoted = raw_path[1:-1]
                    else:
                        unquoted = raw_path
                    new_path = None
                    if unquoted.startswith('~/') or unquoted.startswith('/'):
                        basename = unquoted.split('/')[-1]
                        if not basename:
                            basename = 'output.txt'
                        new_path = f'./outputs/{basename}'
                    elif unquoted.startswith('src/') or unquoted.startswith('./src/'):
                        basename = unquoted.split('/')[-1]
                        if not basename:
                            basename = 'output.rs'
                        new_path = f'./outputs/{basename}'
                    elif unquoted.startswith('./logs/') or unquoted in ['./logs', './outputs']:
                        basename = unquoted.split('/')[-1]
                        new_path = f'./outputs/{basename}.txt'
                    if new_path:
                        indent_str = ' ' * indent
                        out.append(f'{indent_str}- "{new_path}"')
                        continue
            if arg_index == 1:
                m_arg = re.match(r'^- (.+)$', stripped)
                if m_arg:
                    val = m_arg.group(1).strip()
                    if '/' in val or val.startswith('.') or val.startswith('~'):
                        unquoted = val.strip('"\'')
                        basename = unquoted.split('/')[-1].split('.')[0]
                        bookmark = f'bookmark_{basename}' if basename else 'bookmark_step'
                        indent_str = ' ' * indent
                        out.append(f'{indent_str}- {bookmark}')
                        out.append(f'{indent_str}- "{unquoted}"')
                        continue
            out.append(line)
        else:
            out.append(line)
    return '\n'.join(out)


def fix_unindented_markdown_in_prompt(content: str) -> str:
    """Re-indent lines after 'prompt: |' that lost their indentation.

    YAML literal blocks require consistent indentation. Model output sometimes
    has markdown lines starting at column 1 (e.g. '**bold**' or '# heading')
    which breaks the block scalar.

    Detection of end-of-prompt-body: a line whose indent is LESS THAN OR EQUAL TO
    the prompt's indent AND starts with a known step-block field name (model_overrides,
    when, generative_entity, etc.). Without this, the function would re-indent
    model_overrides/when/etc. as if they were prompt body content, breaking the YAML.
    """
    lines = content.split('\n')
    out = []
    in_prompt_block = False
    prompt_indent = 0
    block_indent = 0
    # Step-block sibling field names (siblings of `prompt:` under a step). Lines
    # at <= prompt_indent starting with any of these END the prompt body.
    step_field_names = (
        'model_overrides:', 'when:', 'generative_entity:', 'requires:',
        'depends_on:', 'step_id:', 'step_name:', 'r#loop:', 'loop:',
        'temperature:', 'max_tokens:', 'top_p:',
    )
    for line in lines:
        stripped = line.lstrip()
        cur_indent = len(line) - len(stripped)
        if stripped.startswith('prompt: |'):
            in_prompt_block = True
            prompt_indent = cur_indent
            block_indent = cur_indent + 2
            out.append(line)
            continue
        if in_prompt_block:
            if stripped == '':
                out.append(line)
                continue
            # End of prompt body: indented at or below prompt's indent AND is a known sibling field
            if cur_indent <= prompt_indent and any(stripped.startswith(k) for k in step_field_names):
                in_prompt_block = False
                out.append(line)
                continue
            # End of prompt body: indented at or below prompt's indent AND starts a new step
            if cur_indent <= prompt_indent and re.match(r'^step_[a-z_0-9]+:', stripped):
                in_prompt_block = False
                out.append(line)
                continue
            # Re-indent only TRULY unindented lines (col 0 or 1)
            if cur_indent < 2:
                out.append(' ' * block_indent + stripped)
            else:
                out.append(line)
        else:
            out.append(line)
    return '\n'.join(out)


def fix_save_to_extra_keys(content: str) -> str:
    """Remove line_range/description keys misplaced in save_to items."""
    content = re.sub(
        r'([ \t]+)(line_range|description):\s*[^\n]+\n',
        '',
        content,
    )
    return content


def fix_missing_newline_after_quote(content: str) -> str:
    """Insert newline between closing quote + next key on same line."""
    return re.sub(
        r'("[^"]*")((?:version|workflow_id|name|schema_version|description|models|steps|provider|config|host|port|when|prompt|load_params|context_size|gpu_layers|threads|parallel|cache_type_k|cache_type_v|temperature|max_tokens|top_p|generative_entity|requires|step_id|step_name|route_to|hook|hooks|shell|command|args|log|level|message|to_file_path|event_fields|step_[a-z_0-9]+):)',
        r'\1\n\2',
        content,
    )


def inject_shell_output_template_var(content: str) -> str:
    # Inject {{bookmarks.shell_output.stdout}} into prompt bodies of steps that
    # have before_step_starts: shell hooks but no template var yet.
    #
    # Only injects into MULTI-LINE prompts (`prompt: |`). Single-line prompts
    # (`prompt: "..."`) are skipped — template var cannot be appended to a quoted
    # string without converting to multi-line, which would change semantics.
    lines = content.split('\n')
    out = []
    in_step = False
    has_multiline_prompt = False
    prompt_line_idx = -1
    prompt_indent = 0
    has_shell_hook = False
    has_template_var = False
    step_lines = []

    def flush_step():
        nonlocal has_multiline_prompt, prompt_line_idx, prompt_indent, has_shell_hook, has_template_var, step_lines
        if has_shell_hook and not has_template_var and has_multiline_prompt and prompt_line_idx >= 0:
            inject_line = ' ' * (prompt_indent + 2) + '{{bookmarks.shell_output.stdout}}'
            step_lines.insert(prompt_line_idx + 1, inject_line)
        out.extend(step_lines)
        has_multiline_prompt = False
        prompt_line_idx = -1
        prompt_indent = 0
        has_shell_hook = False
        has_template_var = False
        step_lines = []

    for line in lines:
        if re.match(r'^[ ]+step_[a-z_0-9]+:', line):
            if in_step:
                flush_step()
            in_step = True
            step_lines = [line]
            continue

        if in_step:
            stripped = line.lstrip()
            cur_indent = len(line) - len(stripped)
            if '- shell:' in line and any('before_step_starts' in sl for sl in step_lines[-10:]):
                has_shell_hook = True
            if '{{bookmarks.shell_output' in line:
                has_template_var = True
            if re.match(r'^[ ]+prompt:\s*\|', line):
                has_multiline_prompt = True
                prompt_indent = cur_indent
                prompt_line_idx = len(step_lines)
            step_lines.append(line)
            continue

        out.append(line)

    if in_step:
        flush_step()

    return '\n'.join(out)


def fix_prose_shell_output(content: str) -> str:
    return re.sub(
        r'(in|from|provided in|via|the|using)\s+shell_output\b(?!\.stdout|\}\})',
        r'{{bookmarks.shell_output.stdout}}',
        content,
    )


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

    content = strip_markdown_fences(content)
    content = fix_missing_newline_after_quote(content)
    content = fix_gwt_unquoted_equals(content)
    content = fix_inline_save_to(content)
    content = fix_save_to_map_to_list(content)
    content = fix_save_to_string_form(content)
    content = fix_save_to_outside_when(content)
    content = fix_save_to_null_map_pattern(content)
    content = fix_save_to_split_pair(content)
    content = fix_log_null_with_siblings(content)
    content = fix_log_after_save_to_indent(content)
    content = fix_steps_list_to_map(content)
    content = fix_save_to_unwritable_paths(content)
    content = fix_save_to_extra_keys(content)
    content = fix_unindented_markdown_in_prompt(content)
    content = fix_prose_shell_output(content)
    content = inject_shell_output_template_var(content)

    if content != original:
        backup = path.with_suffix('.yml.bak')
        backup.write_text(original)
        path.write_text(content)
        print(f"Fixed {path} (backup: {backup})")
    else:
        print(f"No changes needed in {path}")

    import yaml
    try:
        yaml.safe_load(content)
        print("VALID_YAML_AFTER_FIX")
    except yaml.YAMLError as e:
        print(f"STILL_INVALID: {e}", file=sys.stderr)
        sys.exit(2)


if __name__ == '__main__':
    main()

