#!/usr/bin/env python3
"""canonicalize-workflow.py — fix broken YAML indent in SW5 outputs."""
import sys
import re
import yaml


def detect_step_indent(lines, steps_idx):
    """Find indent level of step names (lines ending with `_X:` after steps:)."""
    for i in range(steps_idx + 1, len(lines)):
        stripped = lines[i].lstrip()
        if not stripped or stripped.startswith('#'):
            continue
        cur_indent = len(lines[i]) - len(stripped)
        if stripped.endswith(':') and not stripped.startswith('-'):
            tok = stripped[:-1].split('_')
            if len(tok) >= 2 and tok[0] == 'step':
                return cur_indent
        return None
    return None


def extract_step_blocks(lines, steps_line_idx):
    """Walk lines after steps:, group by step_X: pattern. Return list of (step_name, raw_block_lines)."""
    blocks = []
    cur_name = None
    cur_lines = []
    step_re = re.compile(r'^\s+(step_[a-z0-9_]+):\s*$')
    for line in lines[steps_line_idx + 1:]:
        if not line.strip():
            if cur_name:
                cur_lines.append('')
            continue
        m = step_re.match(line)
        if m:
            if cur_name:
                blocks.append((cur_name, cur_lines))
            cur_name = m.group(1)
            cur_lines = []
        else:
            if cur_name:
                cur_lines.append(line)
            else:
                continue
    if cur_name:
        blocks.append((cur_name, cur_lines))
    return blocks


def fix_save_to_nesting(raw_lines):
    """Fix `- save_to:` followed by sibling `- name` and `- path` items.
    Pattern: `- save_to:` then `- <name>` then `- <path>` → nest name+path under save_to.
    Same for `- log:` (single following block)."""
    out = []
    i = 0
    while i < len(raw_lines):
        line = raw_lines[i]
        stripped = line.lstrip()
        if stripped == '- save_to:' and i + 2 < len(raw_lines):
            next1 = raw_lines[i + 1].lstrip()
            next2 = raw_lines[i + 2].lstrip()
            if next1.startswith('- ') and not next1.startswith('- save_to') and not next1.startswith('- log') and not next1.startswith('- shell') and not next1.startswith('- bookmark') and not next1.startswith('- append_to') and not next1.startswith('- gwt') and not next1.startswith('- route_to') and not next1.startswith('- fail') and not next1.startswith('- notify') and not next1.startswith('- skip') and not next1.startswith('- iterate'):
                if next2.startswith('- ') and ('/' in next2 or next2.startswith('- "./') or next2.startswith("- './")):
                    out.append(line)
                    indent = ' ' * 10
                    out.append(indent + next1)
                    out.append(indent + next2)
                    i += 3
                    continue
        out.append(line)
        i += 1
    return out


def reformat_step(name, raw_lines):
    """Given raw block lines (mixed indent), output canonical 4/6/8 indent."""
    raw_lines = fix_save_to_nesting(raw_lines)
    body_keys = {'generative_entity', 'model_overrides', 'when', 'requires', 'step_id', 'step_name'}
    out = [f'    {name}:']
    in_prompt = False
    for line in raw_lines:
        if not line.strip():
            if in_prompt:
                out.append('')
            continue
        stripped = line.lstrip()
        first_word = stripped.split(':')[0].split(' ')[0].split('-')[-1]
        if first_word == 'prompt':
            in_prompt = True
            out.append(f'      {stripped}')
        elif first_word in body_keys:
            in_prompt = False
            out.append(f'      {stripped}')
        elif stripped.startswith('- save_to:'):
            in_prompt = False
            out.append(f'        {stripped}')
        elif stripped.startswith('- '):
            in_prompt = False
            indent_match = re.match(r'^(\s*)', line)
            base_indent = len(indent_match.group(1)) if indent_match else 0
            if base_indent > 8:
                out.append(f'          {stripped}')
            else:
                out.append(f'        {stripped}')
        elif stripped.startswith(('command:', 'args:', 'working_dir:', 'fail_on_error:',
                                    'to_file_path:', 'event_fields:', 'level:', 'message:',
                                    'bookmark:', 'path:', 'host:', 'port:', 'name:', 'type:',
                                    'max_tokens:', 'temperature:', 'top_p:', 'context_size:',
                                    'gpu_layers:', 'threads:', 'parallel:', 'cache_type_k:',
                                    'cache_type_v:', 'no_cache_prompt:', 'cont_batching:',
                                    'flash_attn:', 'description:', 'line_range:')):
            if in_prompt:
                out.append(f'        {stripped}')
            else:
                out.append(f'          {stripped}')
        else:
            if in_prompt:
                out.append(f'        {stripped}')
            else:
                out.append(f'          {stripped}')
    return out


def reindent_steps_block(lines, steps_line_idx, steps_indent, step_indent):
    """Use block extraction instead of indent-based reformat."""
    blocks = extract_step_blocks(lines, steps_line_idx)
    if not blocks:
        return lines[:steps_line_idx + 1]
    out = lines[:steps_line_idx + 1]
    for name, raw_lines in blocks:
        out.extend(reformat_step(name, raw_lines))
    return out


class LenientLoader(yaml.SafeLoader):
    pass


def keep_last(loader, node, deep=False):
    mapping = {}
    for k, v in node.value:
        mapping[loader.construct_object(k, deep=deep)] = loader.construct_object(v, deep=deep)
    return mapping


LenientLoader.add_constructor(yaml.resolver.BaseResolver.DEFAULT_MAPPING_TAG, keep_last)


def canonicalize(content):
    lines = content.split('\n')
    steps_line_idx = None
    steps_indent = 0
    for i, line in enumerate(lines):
        if line.lstrip() == 'steps:':
            steps_line_idx = i
            steps_indent = len(line) - len(line.lstrip())
            break
    if steps_line_idx is None:
        return content
    step_indent = detect_step_indent(lines, steps_line_idx)
    if step_indent is None:
        return content
    reindented = reindent_steps_block(lines, steps_line_idx, steps_indent, step_indent)
    reindented_str = '\n'.join(reindented)
    data = yaml.load(reindented_str, Loader=LenientLoader)
    return yaml.safe_dump(data, sort_keys=False, default_flow_style=False, width=120, allow_unicode=True)


def main():
    if len(sys.argv) < 2:
        print("Usage: canonicalize-workflow.py <workflow.yml>", file=sys.stderr)
        sys.exit(1)
    path = sys.argv[1]
    with open(path) as f:
        content = f.read()
    backup = path + ".canon.bak"
    with open(backup, 'w') as f:
        f.write(content)
    canonical = canonicalize(content)
    with open(path, 'w') as f:
        f.write(canonical)
    print(f"Canonicalized {path} (backup: {backup})", file=sys.stderr)


if __name__ == "__main__":
    main()
