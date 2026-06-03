#!/usr/bin/env python3
"""Post-process a generated workflow YAML: strip fences, inject missing hooks, validate."""
import sys
import re

def strip_markdown_fences(content):
    if content.strip().startswith('```'):
        lines = content.strip().split('\n')
        if lines[0].startswith('```'):
            lines = lines[1:]
        if lines and lines[-1].strip() == '```':
            lines = lines[:-1]
        return '\n'.join(lines)
    return content

def inject_hooks(content):
    """Add after_step_succeeds save_to and log hooks to steps missing them."""
    lines = content.split('\n')
    result = []
    i = 0
    steps_fixed = 0
    
    while i < len(lines):
        line = lines[i]
        result.append(line)
        
        # Detect step definition (4+ space indent, ends with :)
        stripped = line.lstrip()
        indent = len(line) - len(stripped)
        if indent >= 4 and stripped.startswith('step_') and stripped.endswith(':') and 'agentic_workflow' not in stripped:
            step_name = stripped.rstrip(':').strip()
            
            # Collect step content until next step at same indent or dedent
            step_lines = []
            i += 1
            while i < len(lines):
                next_line = lines[i]
                next_stripped = next_line.lstrip()
                next_indent = len(next_line) - len(next_stripped)
                
                if next_stripped and next_indent <= indent and not next_stripped.startswith('#'):
                    break
                step_lines.append(next_line)
                result.append(next_line)
                i += 1
            
            # Check if step has when: block
            has_when = any(l.strip().startswith('when:') for l in step_lines)
            has_save_to = any('save_to' in l for l in step_lines)
            
            if not has_when or not has_save_to:
                hook_indent = ' ' * (indent + 2)
                
                if not has_when:
                    result.append(f'{hook_indent}when:')
                
                if not has_save_to:
                    if not has_when:
                        result.append(f'{hook_indent}  after_step_succeeds:')
                        result.append(f'{hook_indent}    - save_to: "./outputs/{step_name}-output.txt"')
                    steps_fixed += 1
            
            continue
        i += 1
    
    if steps_fixed > 0:
        print(f"Injected save_to hooks into {steps_fixed} steps", file=sys.stderr)
    
    return '\n'.join(result)

def process(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    content = strip_markdown_fences(content)
    content = inject_hooks(content)
    
    with open(filepath, 'w') as f:
        f.write(content)
    
    print(f"Post-processed: {filepath}")
    return True

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: post-process-workflow.py <filepath>", file=sys.stderr)
        sys.exit(1)
    success = process(sys.argv[1])
    sys.exit(0 if success else 1)
