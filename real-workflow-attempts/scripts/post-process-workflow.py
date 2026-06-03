#!/usr/bin/env python3
"""Post-process a generated workflow YAML using yaml library for reliable transforms.

Transformations:
1. Strip markdown fences (```yaml ... ```)
2. Move top-level steps under agentic_workflow: steps:
3. Remove before_step_starts with command: "none"
4. Fix malformed hook formats (mapping→list)
5. Inject missing after_step_succeeds/after_step_fails hooks
6. Validate structure
"""
import sys
import re

try:
    import yaml
except ImportError:
    print("FATAL: PyYAML required. Install: pip install pyyaml", file=sys.stderr)
    sys.exit(1)

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
    """Remove ```yaml ... ``` wrappers."""
    if content.strip().startswith('```'):
        lines = content.strip().split('\n')
        if lines[0].startswith('```'):
            lines = lines[1:]
        if lines and lines[-1].strip() == '```':
            lines = lines[:-1]
        return '\n'.join(lines)
    return content


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


def process(filepath):
    """Main processing pipeline."""
    with open(filepath, 'r') as f:
        content = f.read()

    changes = []

    # Step 1: Strip fences
    content = strip_fences(content)

    # Step 2: Parse YAML
    try:
        data = yaml.safe_load(content)
    except yaml.YAMLError as e:
        # More aggressive cleanup
        content = re.sub(r'^```.*$', '', content, flags=re.MULTILINE).strip()
        try:
            data = yaml.safe_load(content)
        except yaml.YAMLError as e2:
            print(f"FATAL: YAML unparseable: {e2}", file=sys.stderr)
            return False

    if not isinstance(data, dict):
        print(f"FATAL: root is {type(data).__name__}, not mapping", file=sys.stderr)
        return False

    # Step 3: Move top-level steps under agentic_workflow.steps
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

    # Step 4: Fix hooks for all steps
    if 'agentic_workflow' in data and isinstance(data['agentic_workflow'], dict):
        steps = data['agentic_workflow'].get('steps', {})
        if isinstance(steps, dict):
            for sn, sd in steps.items():
                fix_step_hooks(sn, sd, changes)

    # Step 5: Write back
    with open(filepath, 'w') as f:
        yaml.dump(data, f, default_flow_style=False, sort_keys=False, allow_unicode=True)

    # Report
    for c in changes:
        print(f"  FIX: {c}", file=sys.stderr)
    print(f"Post-processed: {filepath} ({len(changes)} fixes)")
    return True


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: post-process-workflow.py <filepath>", file=sys.stderr)
        sys.exit(1)
    success = process(sys.argv[1])
    sys.exit(0 if success else 1)