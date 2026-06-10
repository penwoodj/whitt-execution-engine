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
    result = []
    hooks_added = 0
    
    for step_name, needs_succeeds, needs_fails in steps_needing_hooks:
        step_pattern = re.compile(rf'^(\s{{4}}){re.escape(step_name)}:\s*$')
        when_pattern = re.compile(rf'^(\s{{6}})when:\s*$')
        
        new_lines = []
        i = 0
        step_found = False
        while i < len(lines):
            new_lines.append(lines[i])
            
            if not step_found and step_pattern.match(lines[i]):
                step_found = True
                i += 1
                has_when = False
                step_lines = []
                while i < len(lines):
                    if re.match(r'^\s{4}\w+:\s*$', lines[i]) and not step_pattern.match(lines[i]):
                        break
                    if when_pattern.match(lines[i]):
                        has_when = True
                    step_lines.append(lines[i])
                    i += 1
                
                for sl in step_lines:
                    new_lines.append(sl)
                
                if not has_when:
                    new_lines.append('      when:')
                
                if needs_succeeds:
                    new_lines.append('        after_step_succeeds:')
                    new_lines.append(f'          - save_to: "./docs/benchmarks/outputs/output/{step_name}-output.txt"')
                    new_lines.append('          - log:')
                    new_lines.append('              to_file_path: "./docs/benchmarks/outputs/logs/workflow.log"')
                    new_lines.append('              event_fields: [step_name, duration_ms, token_count]')
                    new_lines.append('              level: info')
                    hooks_added += 1
                
                if needs_fails:
                    new_lines.append('        after_step_fails:')
                    new_lines.append('          - log:')
                    new_lines.append('              to_file_path: "./docs/benchmarks/outputs/logs/workflow.log"')
                    new_lines.append('              event_fields: [step_name, error_message]')
                    new_lines.append('              level: error')
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
    # After "  steps:", ALL non-empty, non-top-level-key lines get +4 spaces.
    # The LLM outputs valid step YAML at 0 base indent; we shift the whole block +4.
    lines = content.split('\n')
    fixed_lines = []
    after_steps_header = False
    steps_indent_added = 0
    
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
    
    if steps_indent_added > 0:
        fixes.append(f"Re-indented {steps_indent_added} line(s) under agentic_workflow.steps")
    
    content = '\n'.join(fixed_lines)
    
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
    
    # 4. Remove empty depends_on: [] from first step
    before = content
    content = re.sub(r'^      depends_on: \[\]\n', '', content, flags=re.MULTILINE)
    if content != before:
        fixes.append("Removed empty depends_on: [] from first step")
    
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
    if len(sys.argv) != 2:
        print("Usage: fix-generated-yaml.py <filepath>", file=sys.stderr)
        sys.exit(1)
    
    filepath = sys.argv[1]
    
    with open(filepath, 'r') as f:
        content = f.read()
    
    fixed, fixes = fix_yaml(content)
    
    if fixes:
        print(f"Applied {len(fixes)} fix(es):")
        for fix in fixes:
            print(f"  - {fix}")
        
        with open(filepath, 'w') as f:
            f.write(fixed)
        print(f"Saved: {filepath}")
    else:
        print("No fixes needed.")
