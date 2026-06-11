#!/usr/bin/env python3
"""Post-process LLM-generated YAML to fix common formatting issues.

Fixes:
1. Strips markdown code fences (```yaml / ```)
2. Re-indents loose steps under agentic_workflow.steps
3. Removes template placeholders like <prompt from STEP PROMPTS>
4. Removes empty depends_on: [] from first step
5. Ensures file starts with workflow_id: on line 1
6. Ensures file ends without trailing fences
7. Removes --- STEP: and --- END --- markers
"""
import sys
import re
import yaml

# Top-level YAML keys that should be at 0 indent
TOP_LEVEL_KEYS = {
    'workflow_id:', 'name:', 'description:', 'version:', 'author:',
    'tags:', 'schema_version:', 'min_schema_version:', 'providers:',
    'models:', 'sub_workflows:', 'agentic_workflow:',
    'workflow_execution_strategy:', 'tool_permissions:', 'memory:',
    'workspace:',
}

def is_top_level_key(line: str) -> bool:
    """Check if line is a top-level YAML key at 0 indent."""
    stripped = line.lstrip()
    if line != stripped:  # Has leading spaces, not top-level
        return False
    for key in TOP_LEVEL_KEYS:
        if stripped.startswith(key):
            return True
    return False

def _parse_step_plan(plan_path: str) -> dict[str, dict]:
    """Parse step plan file to extract shell commands per step.
    
    Format 1: STEP_NAME: name | DEPENDS: dep | TYPE: type | SHELL_CMD: "cmd" | GOAL: goal
    Format 2: STEP_N: Title\n  DEPENDS: dep | TYPE: type | SHELL_CMD: `cmd` | GOAL: goal
    """
    steps = {}
    try:
        with open(plan_path, 'r') as f:
            content = f.read()
        
        # Format 1: pipe-delimited single-line entries
        if 'STEP_NAME:' in content:
            for line in content.split('\n'):
                line = line.strip()
                if not line.startswith('STEP_NAME:'):
                    continue
                parts = {}
                for field in ['STEP_NAME', 'DEPENDS', 'TYPE', 'SHELL_CMD', 'GOAL']:
                    pattern = rf'{field}:\s*(.+?)(?=\s*\|\s*[A-Z_]+:|$)'
                    match = re.search(pattern, line)
                    if match:
                        val = match.group(1).strip().strip('"').strip("'").strip('`')
                        if field == 'STEP_NAME':
                            parts['name'] = val
                        elif field == 'TYPE':
                            parts['step_type'] = val.lower().strip()
                        elif field == 'SHELL_CMD':
                            parts['shell_cmd'] = val
                        elif field == 'DEPENDS':
                            parts['depends'] = val
                        elif field == 'GOAL':
                            parts['goal'] = val
                if 'name' in parts:
                    steps[parts['name']] = parts
        
        # Format 2: multi-line STEP_N: Title entries
        elif re.search(r'STEP_\d+:', content):
            blocks = re.split(r'\n(?=STEP_\d+:)', content)
            for block in blocks:
                block = block.strip()
                if not block.startswith('STEP_'):
                    continue
                lines = block.split('\n')
                header = lines[0]
                title_match = re.match(r'STEP_\d+:\s*(.+)', header)
                if not title_match:
                    continue
                title = title_match.group(1).strip()
                name = re.sub(r'[^a-z0-9]+', '_', title.lower()).strip('_')
                rest = '\n'.join(lines[1:])
                parts = {'name': name}
                
                type_match = re.search(r'TYPE:\s*(\w+)', rest, re.IGNORECASE)
                if type_match:
                    parts['step_type'] = type_match.group(1).lower().strip()
                
                cmd_match = re.search(r'SHELL_CMD:\s*`([^`]+)`', rest)
                if not cmd_match:
                    cmd_match = re.search(r'SHELL_CMD:\s*["\']?([^"\':\n]+)["\']?', rest)
                if cmd_match:
                    parts['shell_cmd'] = cmd_match.group(1).strip()
                
                depends_match = re.search(r'DEPENDS:\s*(.+?)(?:\s*\|\s*[A-Z_]+:|$)', rest)
                if depends_match:
                    parts['depends'] = depends_match.group(1).strip()
                
                goal_match = re.search(r'GOAL:\s*(.+?)$', rest, re.MULTILINE)
                if goal_match:
                    parts['goal'] = goal_match.group(1).strip()
                
                if parts.get('step_type') == 'shell' or parts.get('shell_cmd'):
                    steps[name] = parts
    except Exception:
        pass
    return steps

def _inject_shell_hooks(content: str, plan_steps: dict[str, dict]) -> tuple[int, str]:
    """Inject before_step_starts shell hooks for steps that need them based on plan."""
    if not plan_steps:
        return 0, content
    
    hooks_added = 0
    lines = content.split('\n')
    
    yaml_steps_ordered = []
    in_agentic_steps = False
    for line_idx, line in enumerate(lines):
        stripped = line.lstrip()
        if stripped == 'steps:':
            in_agentic_steps = True
            continue
        if in_agentic_steps:
            base_indent = len(line) - len(stripped)
            if base_indent == 0 and stripped:
                in_agentic_steps = False
                continue
            if base_indent == 4 and stripped.endswith(':') and not stripped.startswith('#'):
                yaml_step_name = stripped.rstrip(':').strip()
                if yaml_step_name and not yaml_step_name.startswith('when') and not yaml_step_name.startswith('after'):
                    yaml_steps_ordered.append(yaml_step_name)

    yaml_step_to_plan = {}
    plan_names = list(plan_steps.keys())
    match_count = min(len(plan_names), len(yaml_steps_ordered))
    for idx in range(match_count):
        yaml_step_to_plan[yaml_steps_ordered[idx]] = plan_names[idx]
    
    for yaml_step_name, plan_name in yaml_step_to_plan.items():
        plan_info = plan_steps[plan_name]
        if plan_info.get('step_type') != 'shell' and 'shell_cmd' not in plan_info:
            continue
        
        shell_cmd = plan_info.get('shell_cmd', '')
        if not shell_cmd:
            continue
        
        step_pattern = re.compile(rf'^(\s+){re.escape(yaml_step_name)}:\s*$')
        
        new_lines = []
        i = 0
        step_found = False
        already_has_shell_hook = False
        
        while i < len(lines):
            cur = lines[i]
            new_lines.append(cur)
            
            if not step_found:
                m = step_pattern.match(cur)
                if m:
                    si = len(m.group(1))
                    step_found = True
                    i += 1
                    step_lines = []
                    while i < len(lines):
                        nxt = lines[i]
                        nxt_indent = len(nxt) - len(nxt.lstrip())
                        if nxt.strip() and nxt_indent <= si:
                            break
                        step_lines.append(nxt)
                        if re.search(r'-\s*shell:', nxt):
                            already_has_shell_hook = True
                        i += 1
                    
                    for sl in step_lines:
                        new_lines.append(sl)
                    
                    if not already_has_shell_hook:
                        has_when = any(re.match(rf'^\s{{{si + 2}}}when:\s*$', sl) for sl in step_lines)
                        ind = ' ' * si
                        if not has_when:
                            new_lines.append(f'{ind}  when:')
                        new_lines.append(f'{ind}    before_step_starts:')
                        new_lines.append(f'{ind}      - shell:')
                        safe_cmd = shell_cmd.replace("'", "''")
                        new_lines.append(f"{ind}          command: '{safe_cmd}'")
                        new_lines.append(f'{ind}          args: []')
                        new_lines.append(f"{ind}          working_dir: '/home/jon/code/whitt-execution-engine'")
                        new_lines.append(f'{ind}          fail_on_error: false')
                        hooks_added += 1
                    continue
            
            i += 1
        
        if step_found:
            lines = new_lines
    
    return hooks_added, '\n'.join(lines)

def _fix_prompt_templates(content: str, plan_steps: dict[str, dict]) -> tuple[int, str]:
    if not plan_steps:
        return 0, content

    plan_names = list(plan_steps.keys())
    shell_plan_names = set()
    for plan_name, info in plan_steps.items():
        if info.get('step_type') == 'shell' or info.get('shell_cmd'):
            shell_plan_names.add(plan_name)

    if not shell_plan_names:
        return 0, content

    data_block_pattern = re.compile(r'Here is (?:the |some )?data:\s*\n```[^`]*```', re.DOTALL)

    lines = content.split('\n')
    new_lines = []
    fixes = 0
    i = 0

    while i < len(lines):
        new_lines.append(lines[i])
        i += 1

    result = '\n'.join(new_lines)
    result = data_block_pattern.sub('{{bookmarks.shell_output.stdout}}', result)
    if '{{bookmarks.shell_output.stdout}}' in result and 'Here is' not in result.split('{{bookmarks.shell_output.stdout}}')[0].split('\n')[-1]:
        fixes = 1

    return fixes, result

def _inject_missing_hooks(content: str) -> tuple[int, str]:
    """Add missing when: hooks only to steps under agentic_workflow.steps."""
    import yaml
    try:
        data = yaml.safe_load(content)
    except Exception:
        return 0, content
    
    if not data or 'agentic_workflow' not in data or 'steps' not in data.get('agentic_workflow', {}):
        return 0, content
    
    steps = data['agentic_workflow']['steps']
    steps_needing_hooks = []
    for step_name, step_data in steps.items():
        if not isinstance(step_data, dict):
            continue
        needs_succeeds = 'when' not in step_data or 'after_step_succeeds' not in step_data.get('when', {})
        needs_fails = 'when' not in step_data or 'after_step_fails' not in step_data.get('when', {})
        if needs_succeeds or needs_fails:
            steps_needing_hooks.append((step_name, needs_succeeds, needs_fails))
    
    if not steps_needing_hooks:
        return 0, content
    
    lines = content.split('\n')
    hooks_added = 0
    
    for step_name, needs_succeeds, needs_fails in steps_needing_hooks:
        step_pattern = re.compile(rf'^(\s+){re.escape(step_name)}:\s*$')
        
        new_lines = []
        i = 0
        step_found = False
        while i < len(lines):
            new_lines.append(lines[i])
            
            if not step_found:
                m = step_pattern.match(lines[i])
                if m:
                    step_indent = len(m.group(1))
                    step_found = True
                    i += 1
                    has_when = False
                    step_lines = []
                    while i < len(lines):
                        if re.match(rf'^\s{{{step_indent}}}\w+:\s*$', lines[i]) and not step_pattern.match(lines[i]):
                            break
                        if re.match(rf'^\s{{{step_indent + 2}}}when:\s*$', lines[i]):
                            has_when = True
                        step_lines.append(lines[i])
                        i += 1
                    
                    for sl in step_lines:
                        new_lines.append(sl)
                    
                    si = ' ' * step_indent
                    if not has_when:
                        new_lines.append(f'{si}  when:')
                    
                    if needs_succeeds:
                        new_lines.append(f'{si}    after_step_succeeds:')
                        new_lines.append(f'{si}      - save_to: "./docs/benchmarks/outputs/output/{step_name}-output.txt"')
                        new_lines.append(f'{si}      - log:')
                        new_lines.append(f'{si}          to_file_path: "./docs/benchmarks/outputs/logs/workflow.log"')
                        new_lines.append(f'{si}          event_fields: [step_name, duration_ms, token_count]')
                        new_lines.append(f'{si}          level: info')
                        hooks_added += 1
                    
                    if needs_fails:
                        new_lines.append(f'{si}    after_step_fails:')
                        new_lines.append(f'{si}      - log:')
                        new_lines.append(f'{si}          to_file_path: "./docs/benchmarks/outputs/logs/workflow.log"')
                        new_lines.append(f'{si}          event_fields: [step_name, error_message]')
                        new_lines.append(f'{si}          level: error')
                        hooks_added += 1
            else:
                i += 1
        
        if step_found:
            lines = new_lines
    
    return hooks_added, '\n'.join(lines)

def fix_yaml(content: str) -> tuple[str, list[str]]:
    fixes = []
    
    # 1. Strip markdown fences
    if content.strip().startswith('```'):
        lines = content.strip().split('\n')
        if lines[0].startswith('```'):
            lines = lines[1:]
            fixes.append("Stripped opening markdown fence")
        if lines and lines[-1].strip() == '```':
            lines = lines[:-1]
            fixes.append("Stripped closing markdown fence")
        content = '\n'.join(lines)
    
    # 2. Re-indent loose steps under agentic_workflow.steps
    # After "  steps:", check if first non-empty line is at 0 indent.
    # If so, ALL lines in the steps block need +4. If already at 4+, skip.
    lines = content.split('\n')
    steps_idx = -1
    for i, line in enumerate(lines):
        if line == '  steps:':
            steps_idx = i
            break
    
    needs_reindent = False
    if steps_idx >= 0:
        for i in range(steps_idx + 1, len(lines)):
            if lines[i].strip() and not is_top_level_key(lines[i]):
                first_indent = len(lines[i]) - len(lines[i].lstrip())
                if first_indent == 0:
                    needs_reindent = True
                break
    
    steps_indent_added = 0
    if needs_reindent:
        fixed_lines = []
        after_steps_header = False
        for i, line in enumerate(lines):
            if line == '  steps:':
                after_steps_header = True
                fixed_lines.append(line)
                continue
            if after_steps_header:
                if line.strip() == '':
                    fixed_lines.append(line)
                    continue
                if is_top_level_key(line):
                    after_steps_header = False
                    fixed_lines.append(line)
                    continue
                fixed_lines.append('    ' + line)
                steps_indent_added += 1
                continue
            fixed_lines.append(line)
        lines = fixed_lines
    
    if steps_indent_added > 0:
        fixes.append(f"Re-indented {steps_indent_added} line(s) under agentic_workflow.steps")
    
    content = '\n'.join(lines)
    
    # 2b. Fix pipe block indentation after +4 indent
    # After step 2, lines that were at 0 indent are now at 4 spaces.
    # For prompt: | blocks, content must be at pipe_block_indent = line_indent + 2.
    # If prompt: | is at 8 spaces (under step), content must be at 10+.
    # We find all prompt: | lines and re-indent their content.
    lines = content.split('\n')
    fixed_lines = []
    pipe_fixes = 0
    i = 0
    while i < len(lines):
        line = lines[i]
        fixed_lines.append(line)
        
        stripped = line.rstrip()
        if stripped.endswith('prompt: |') or stripped.endswith('prompt: |-'):
            pipe_indent = len(line) - len(line.lstrip())
            content_indent = pipe_indent + 2
            i += 1
            while i < len(lines):
                next_line = lines[i]
                if next_line.strip() == '':
                    fixed_lines.append(next_line)
                    i += 1
                    continue
                next_indent = len(next_line) - len(next_line.lstrip())
                next_stripped = next_line.lstrip()
                is_yaml_key = bool(re.match(r'^[a-z_][a-z0-9_]*:($|\s)', next_stripped) or re.match(r'^-[a-z_]', next_stripped))
                if next_indent <= pipe_indent and is_yaml_key:
                    break
                if next_line.lstrip().startswith('when:') or next_line.lstrip().startswith('model_overrides:') or next_line.lstrip().startswith('depends_on:'):
                    break
                current_indent = next_indent
                if current_indent < content_indent:
                    fixed_lines.append(' ' * content_indent + next_line.lstrip())
                    pipe_fixes += 1
                else:
                    fixed_lines.append(next_line)
                i += 1
        else:
            i += 1
    
    if pipe_fixes > 0:
        fixes.append(f"Fixed {pipe_fixes} prompt pipe block indentation(s)")
        content = '\n'.join(fixed_lines)
    
    # 3. Remove template placeholders
    placeholder_patterns = [
        (r'<prompt from STEP PROMPTS>', 'the actual prompt content for this step'),
        (r'__TASK_PLACEHOLDER__', ''),
        (r'__RUN_ID__', 'auto-generated'),
        (r'<TASK_DESCRIPTION>', 'the task'),
    ]
    for pattern, replacement in placeholder_patterns:
        count = len(re.findall(pattern, content))
        if count > 0:
            content = re.sub(pattern, replacement, content)
            fixes.append(f"Replaced {count} instance(s) of '{pattern}'")
    
    # 3b. Remove --- STEP: and --- END --- markers
    step_markers = len(re.findall(r'^--- (?:STEP|END) ---\s*$', content, re.MULTILINE))
    if step_markers > 0:
        content = re.sub(r'^--- (?:STEP|END) ---\s*\n?', '', content, flags=re.MULTILINE)
        fixes.append(f"Removed {step_markers} step markers")
    
    # 4. Remove empty depends_on: [] from any step (at any indent level)
    before = content
    content = re.sub(r'^\s+depends_on: \[\]\n', '', content, flags=re.MULTILINE)
    if content != before:
        count = len(re.findall(r'depends_on: \[\]', before)) - len(re.findall(r'depends_on: \[\]', content))
        fixes.append(f"Removed {count} empty depends_on: [] entries")
    
    # 4b. Fix broken dependency references (text-based to preserve pipe blocks)
    try:
        data = yaml.safe_load(content)
        if data and 'agentic_workflow' in data and 'steps' in data.get('agentic_workflow', {}):
            steps = data['agentic_workflow']['steps']
            step_names = list(steps.keys())
            dep_fixes = 0
            for name, step in steps.items():
                deps = step.get('depends_on', [])
                if not deps:
                    continue
                for dep_idx, dep in enumerate(deps):
                    if dep not in step_names:
                        name_idx = step_names.index(name)
                        if name_idx > 0:
                            old_ref = dep
                            new_ref = step_names[name_idx - 1]
                            content = content.replace(f"[{old_ref}]", f"[{new_ref}]")
                            content = content.replace(f"'{old_ref}'", f"'{new_ref}'")
                            content = content.replace(f"- {old_ref}", f"- {new_ref}")
                            dep_fixes += 1
            if dep_fixes > 0:
                fixes.append(f"Fixed {dep_fixes} broken dependency reference(s)")
    except Exception:
        pass
    
    # 5. Ensure starts with workflow_id:
    lines = content.split('\n')
    while lines and lines[0].strip() == '':
        lines.pop(0)
    if lines and not lines[0].startswith('workflow_id:'):
        for i, line in enumerate(lines):
            if line.startswith('workflow_id:'):
                lines = lines[i:]
                fixes.append("Moved workflow_id: to line 1")
                break
    content = '\n'.join(lines)
    
    # 6. Remove trailing fences
    content = content.rstrip()
    if content.endswith('```'):
        content = content[:-3].rstrip()
        fixes.append("Removed trailing markdown fence")
    
    # 7. Ensure every step has when: hooks (text-based insertion to preserve formatting)
    hooks_added = _inject_missing_hooks(content)
    if hooks_added[0] > 0:
        content = hooks_added[1]
        fixes.append(f"Added {hooks_added[0]} missing when: hook(s) to steps")
    
    return content, fixes

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: fix-generated-yaml.py <filepath> [step-plan-path]", file=sys.stderr)
        sys.exit(1)
    
    filepath = sys.argv[1]
    plan_path = sys.argv[2] if len(sys.argv) > 2 else None
    
    with open(filepath, 'r') as f:
        content = f.read()
    
    fixed, fixes = fix_yaml(content)
    
    if plan_path:
        plan_steps = _parse_step_plan(plan_path)
        if plan_steps:
            shell_hooks_added, fixed = _inject_shell_hooks(fixed, plan_steps)
            if shell_hooks_added > 0:
                fixes.append(f"Injected {shell_hooks_added} shell hook(s) from step plan")
            prompt_fixes, fixed = _fix_prompt_templates(fixed, plan_steps)
            if prompt_fixes > 0:
                fixes.append(f"Fixed {prompt_fixes} prompt(s) with template interpolation")
    
    if fixes:
        print(f"Applied {len(fixes)} fix(es):")
        for fix in fixes:
            print(f"  - {fix}")
        
        with open(filepath, 'w') as f:
            f.write(fixed)
        print(f"Saved: {filepath}")
    else:
        print("No fixes needed.")
