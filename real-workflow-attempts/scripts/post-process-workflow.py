#!/usr/bin/env python3
"""Post-process a generated workflow YAML using yaml library for reliable transforms.

Transformations:
1. Strip markdown fences (```yaml ... ```)
2. Move top-level steps under agentic_workflow: steps:
3. Inject actual prompts from 03-prompts.txt (replaces placeholders)
4. Remove before_step_starts with command: "none"
5. Fix malformed hook formats (mapping→list)
6. Inject missing after_step_succeeds/after_step_fails hooks
7. Remove invalid hosting: gpu_layers field
8. Validate structure
"""
import sys
import re
import os
import glob

try:
    import yaml
except ImportError:
    print("FATAL: PyYAML required. Install: pip install pyyaml", file=sys.stderr)
    sys.exit(1)


# Custom representer to preserve multi-line strings as block scalars
class _MultiLineStr(str):
    """String subclass that yaml.dump will render as literal block (|)."""
    pass


def _repr_multiline_str(dumper, data):
    """Use literal block style for multi-line strings, flow style for single-line."""
    if '\n' in data:
        return dumper.represent_scalar('tag:yaml.org,2002:str', data, style='|')
    return dumper.represent_scalar('tag:yaml.org,2002:str', data)


yaml.add_representer(_MultiLineStr, _repr_multiline_str)


def _preserve_str(value):
    """Wrap multi-line strings so yaml.dump uses block style."""
    if isinstance(value, str) and '\n' in value:
        return _MultiLineStr(value)
    return value


def _preserve_all_strings(obj):
    """Walk data tree, wrapping multi-line strings for block-style YAML output."""
    if isinstance(obj, dict):
        for k, v in list(obj.items()):
            if isinstance(v, str):
                obj[k] = _preserve_str(v)
            elif isinstance(v, dict):
                _preserve_all_strings(v)
            elif isinstance(v, list):
                _preserve_all_strings(v)
    elif isinstance(obj, list):
        for i, v in enumerate(obj):
            if isinstance(v, str):
                obj[i] = _preserve_str(v)
            elif isinstance(v, (dict, list)):
                _preserve_all_strings(v)

# Standard hook templates
LOG_SUCCESS = {
    "log": {
        "to_file_path": "./logs/workflow.log",
        "event_fields": ["step_name", "duration_ms"],
        "level": "info"
    }
}
LOG_FAIL = {
    "log": {
        "to_file_path": "./logs/workflow.log",
        "event_fields": ["step_name", "error_message"],
        "level": "error"
    }
}


def strip_fences(content):
    """Remove ```yaml ... ``` wrappers and non-YAML preamble."""
    if content.strip().startswith('```'):
        lines = content.strip().split('\n')
        if lines[0].startswith('```'):
            lines = lines[1:]
        if lines and lines[-1].strip() == '```':
            lines = lines[:-1]
        return '\n'.join(lines)
    # Strip preamble: find first line starting with a YAML key
    lines = content.split('\n')
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped and not stripped.startswith('#') and ':' in stripped and not stripped.startswith('-'):
            if stripped.split(':')[0].replace('_', '').replace('-', '').isalnum():
                if i > 0:
                    return '\n'.join(lines[i:])
    return content


def parse_prompts_file(prompts_path):
    if not prompts_path or not os.path.exists(prompts_path):
        return {}
    with open(prompts_path, 'r') as f:
        content = f.read()
    
    prompts = {}
    pattern = r'---\s*STEP:\s*(\S+)\s*---\s*\n(.*?)\n---\s*END\s*---'
    for match in re.finditer(pattern, content, re.DOTALL):
        step_name = match.group(1).strip()
        prompt_text = match.group(2).strip()
        prompts[step_name] = prompt_text
    return prompts


def inject_prompts(steps, prompts, changes):
    if not prompts or not isinstance(steps, dict):
        return
    for step_name, step_data in steps.items():
        if not isinstance(step_data, dict):
            continue
        prompt = step_data.get('prompt', '')
        if isinstance(prompt, str) and ('<prompt content' in prompt or 'FILL_IN_LATER' in prompt):
            if step_name in prompts:
                step_data['prompt'] = prompts[step_name]
                changes.append(f"{step_name}: injected actual prompt")
            else:
                for pname, ptext in prompts.items():
                    if pname == step_name or pname.endswith(step_name.split('_', 2)[-1] if '_' in step_name else step_name):
                        step_data['prompt'] = ptext
                        changes.append(f"{step_name}: injected prompt from {pname}")
                        break


def fix_step_hooks(step_name, step_data, changes):
    """Fix hooks for a single step dict (mutates in place)."""
    if not isinstance(step_data, dict):
        return

    # Get/create when block
    when = step_data.get('when')
    if when is None:
        when = {}
        step_data['when'] = when
        changes.append(f"{step_name}: added when: block")
    elif not isinstance(when, dict):
        when = {}
        step_data['when'] = when
        changes.append(f"{step_name}: recreated malformed when:")

    # Remove before_step_starts with command "none"
    before = when.get('before_step_starts')
    if isinstance(before, list):
        for action in before:
            if isinstance(action, dict) and isinstance(action.get('shell'), dict):
                if action['shell'].get('command') in ('none', 'None'):
                    del when['before_step_starts']
                    changes.append(f"{step_name}: removed before_step_starts with command none")
                    break

    # Inject bookmark reference into prompts for steps with shell hooks
    if isinstance(when.get('before_step_starts'), list):
        has_shell = any(
            isinstance(a, dict) and isinstance(a.get('shell'), dict)
            for a in when['before_step_starts']
        )
        if has_shell:
            prompt = step_data.get('prompt', '')
            if isinstance(prompt, str) and 'bookmarks.shell_output' not in prompt:
                step_data['prompt'] = (
                    "Here is the data to process:\n"
                    "{{bookmarks.shell_output.stdout}}\n\n"
                    + prompt
                )
                changes.append(f"{step_name}: injected bookmark reference into prompt")

    # Inject working_dir for shell hooks that lack it
    REPO_DIR = os.environ.get('REPO_DIR', '/home/jon/code/whitt-execution-engine')
    if isinstance(when.get('before_step_starts'), list):
        for action in when['before_step_starts']:
            if isinstance(action, dict) and isinstance(action.get('shell'), dict):
                shell = action['shell']
                if 'working_dir' not in shell:
                    shell['working_dir'] = REPO_DIR
                    changes.append(f"{step_name}: injected working_dir into shell hook")

    # Fix after_step_succeeds
    after_s = when.get('after_step_succeeds')
    if after_s is not None and not isinstance(after_s, list):
        # Convert malformed mapping to list
        if isinstance(after_s, dict):
            fixed = []
            if 'save_to' in after_s:
                fixed.append({'save_to': after_s['save_to']})
            if 'log' in after_s:
                fixed.append({'log': after_s['log']})
            when['after_step_succeeds'] = fixed
            changes.append(f"{step_name}: fixed after_step_succeeds mapping→list")
        else:
            when['after_step_succeeds'] = []

    # Ensure after_step_succeeds exists and has save_to + log
    if 'after_step_succeeds' not in when:
        save_to = {"save_to": f"./outputs/{step_name}-output.txt"}
        when['after_step_succeeds'] = [save_to, dict(LOG_SUCCESS)]
        changes.append(f"{step_name}: injected after_step_succeeds")
    else:
        actions = when['after_step_succeeds']
        if not any(isinstance(a, dict) and 'save_to' in a for a in actions if isinstance(a, dict)):
            actions.insert(0, {"save_to": f"./outputs/{step_name}-output.txt"})
            changes.append(f"{step_name}: injected save_to")
        if not any(isinstance(a, dict) and 'log' in a for a in actions if isinstance(a, dict)):
            actions.append(dict(LOG_SUCCESS))
            changes.append(f"{step_name}: injected log into after_step_succeeds")

    # Fix after_step_fails
    after_f = when.get('after_step_fails')
    if after_f is not None and not isinstance(after_f, list):
        if isinstance(after_f, dict):
            fixed = []
            if 'log' in after_f:
                fixed.append({'log': after_f['log']})
            when['after_step_fails'] = fixed
            changes.append(f"{step_name}: fixed after_step_fails mapping→list")
        else:
            when['after_step_fails'] = []

    # Ensure after_step_fails exists
    if 'after_step_fails' not in when:
        when['after_step_fails'] = [dict(LOG_FAIL)]
        changes.append(f"{step_name}: injected after_step_fails")
    else:
        actions = when['after_step_fails']
        if not any(isinstance(a, dict) and 'log' in a for a in actions if isinstance(a, dict)):
            actions.append(dict(LOG_FAIL))
            changes.append(f"{step_name}: injected log into after_step_fails")


def process(filepath, prompts_path=None):
    """Main processing pipeline."""
    with open(filepath, 'r') as f:
        content = f.read()

    changes = []

    content = strip_fences(content)

    # Fix shell command quoting: wrap complex commands in single quotes
    # Models generate things like: command: for file in $(find ...); do cat "$file"; done
    # which breaks YAML parsing due to nested quotes and special chars
    def fix_command_quoting(text):
        # Replace double-quoted command values containing nested quotes
        # with single-quoted values (YAML-safe for shell commands)
        pattern = re.compile(r'^(\s*command:\s*)"(.*)"$', re.MULTILINE)
        def replacer(match):
            indent_and_key = match.group(1)
            cmd_val = match.group(2)
            if '"' in cmd_val or '$(' in cmd_val or ';' in cmd_val:
                escaped = cmd_val.replace("'", "''")
                return f"{indent_and_key}'{escaped}'"
            return match.group(0)
        return pattern.sub(replacer, text)

    content = fix_command_quoting(content)

    # Remove markdown table separators that break YAML block scalar parsing
    content = re.sub(r'^\s*\|[-|]+\|$', '', content, flags=re.MULTILINE)

    try:
        data = yaml.safe_load(content)
    except yaml.YAMLError as e:
        content = re.sub(r'^```.*$', '', content, flags=re.MULTILINE).strip()
        try:
            data = yaml.safe_load(content)
        except yaml.YAMLError as e2:
            # Try with safe_loader that handles more edge cases
            try:
                # Fix common YAML quoting issues in shell commands
                fixed = re.sub(r'command:\s+"([^"]*)"', lambda m: 'command: "' + m.group(1).replace('\\', '\\\\') + '"', content)
                data = yaml.safe_load(fixed)
            except yaml.YAMLError:
                print(f"FATAL: YAML unparseable: {e2}", file=sys.stderr)
                return False

    if not isinstance(data, dict):
        print(f"FATAL: root is {type(data).__name__}, not mapping", file=sys.stderr)
        return False

    top_steps = {k: v for k, v in data.items()
                 if isinstance(k, str) and k.startswith('step_') and isinstance(v, dict)}
    if top_steps:
        for k in top_steps:
            del data[k]

        if 'agentic_workflow' not in data:
            data['agentic_workflow'] = {}
        aw = data['agentic_workflow']
        if not isinstance(aw, dict):
            aw = {}
            data['agentic_workflow'] = aw
        if 'steps' not in aw:
            aw['steps'] = {}
        steps = aw['steps']
        if not isinstance(steps, dict):
            steps = {}
            aw['steps'] = steps

        for k, v in top_steps.items():
            if k not in steps:
                steps[k] = v
                changes.append(f"moved {k} → agentic_workflow.steps")

    prompts = parse_prompts_file(prompts_path)
    if 'agentic_workflow' in data and isinstance(data['agentic_workflow'], dict):
        steps = data['agentic_workflow'].get('steps', {})
        if isinstance(steps, dict):
            inject_prompts(steps, prompts, changes)

    providers = data.get('providers', {})
    if isinstance(providers, dict):
        for pname, pdata in providers.items():
            if isinstance(pdata, dict) and 'hosting' in pdata:
                hosting = pdata['hosting']
                if isinstance(hosting, dict) and 'gpu_layers' in hosting:
                    del hosting['gpu_layers']
                    if not hosting:
                        del pdata['hosting']
                    changes.append(f"removed invalid hosting: gpu_layers from provider {pname}")

    if 'agentic_workflow' in data and isinstance(data['agentic_workflow'], dict):
        steps = data['agentic_workflow'].get('steps', {})
        if isinstance(steps, dict):
            for sn, sd in steps.items():
                fix_step_hooks(sn, sd, changes)

    with open(filepath, 'w') as f:
        _preserve_all_strings(data)
        yaml.dump(data, f, default_flow_style=False, sort_keys=False, allow_unicode=True, width=200)

    for c in changes:
        print(f"  FIX: {c}", file=sys.stderr)
    print(f"Post-processed: {filepath} ({len(changes)} fixes)")
    return True


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: post-process-workflow.py <filepath> [prompts-file]", file=sys.stderr)
        sys.exit(1)
    prompts_path = sys.argv[2] if len(sys.argv) > 2 else None
    success = process(sys.argv[1], prompts_path)
    sys.exit(0 if success else 1)