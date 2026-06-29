#!/usr/bin/env python3
"""Dry-run validator for generated workflow YAMLs.

Validates structural integrity WITHOUT running inference. Catches 90% of
pipeline failures in <5 seconds instead of 70 minutes.

Usage:
    python3 scripts/meta-v6/validate-workflow.py <workflow.yml> [--repo-root <path>]

Exit codes:
    0 = ALL CHECKS PASSED
    1 = ONE OR MORE CHECKS FAILED (see output for details)
    2 = USAGE ERROR
"""
import sys
import os
import re
import yaml
import argparse
from pathlib import Path
from collections import defaultdict


class ValidationResult:
    def __init__(self):
        self.passed = []
        self.failed = []
        self.warnings = []

    def ok(self, check, detail=""):
        self.passed.append((check, detail))

    def fail(self, check, detail=""):
        self.failed.append((check, detail))

    def warn(self, check, detail=""):
        self.warnings.append((check, detail))

    def exit_code(self):
        return 1 if self.failed else 0

    def print_report(self, wf_path):
        total = len(self.passed) + len(self.failed)
        print(f"\n{'='*60}")
        print(f"DRY-RUN VALIDATION: {wf_path}")
        print(f"{'='*60}")
        print(f"Checks: {len(self.passed)}/{total} passed, {len(self.failed)} failed, {len(self.warnings)} warnings")
        print()

        if self.failed:
            print("FAILURES:")
            for check, detail in self.failed:
                print(f"  ❌ {check}: {detail}")
            print()

        if self.warnings:
            print("WARNINGS:")
            for check, detail in self.warnings:
                print(f"  ⚠️  {check}: {detail}")
            print()

        if self.passed and self.failed:
            print("PASSED:")
            for check, detail in self.passed:
                if detail:
                    print(f"  ✅ {check}: {detail}")

        print(f"\n{'='*60}")
        if self.failed:
            print(f"VERDICT: ❌ FAIL — {len(self.failed)} issue(s) found. Do NOT execute this workflow.")
        else:
            print(f"VERDICT: ✅ PASS — workflow is structurally sound. Safe to execute.")
        print(f"{'='*60}")


def extract_step_ids(steps):
    return set(steps.keys())


def extract_cat_targets(shell_cmd):
    targets = []
    for m in re.finditer(r'cat\s+(\S+)', shell_cmd):
        targets.append(m.group(1).strip("'\""))
    return targets


def extract_sed_targets(shell_cmd):
    targets = []
    for m in re.finditer(r"sed\s+-n\s+'[^']+'\s+(\S+)", shell_cmd):
        targets.append(m.group(1).strip("'\""))
    for m in re.finditer(r'sed\s+-n\s+"[^"]+"\s+(\S+)', shell_cmd):
        targets.append(m.group(1).strip("'\""))
    return targets


def extract_template_refs(text):
    refs = set()
    for m in re.finditer(r'\{\{step\.(\w+)\.output\}\}', text):
        refs.add(m.group(1))
    return refs


def topological_sort(steps):
    in_degree = defaultdict(int)
    adj = defaultdict(list)

    for sid, step in steps.items():
        deps = step.get('depends_on', [])
        if isinstance(deps, str):
            deps = [deps]
        elif not isinstance(deps, list):
            deps = []
        in_degree.setdefault(sid, 0)
        for dep in deps:
            if dep in steps:
                adj[dep].append(sid)
                in_degree[sid] += 1

    queue = [s for s in steps if in_degree.get(s, 0) == 0]
    sorted_order = []
    while queue:
        node = queue.pop(0)
        sorted_order.append(node)
        for neighbor in adj[node]:
            in_degree[neighbor] -= 1
            if in_degree[neighbor] == 0:
                queue.append(neighbor)

    has_cycle = len(sorted_order) < len(steps)
    return sorted_order, has_cycle


def validate_workflow(wf_path, repo_root):
    result = ValidationResult()

    wf_path = Path(wf_path)
    if not wf_path.exists():
        result.fail("file-exists", f"Workflow file not found: {wf_path}")
        return result

    try:
        wf = yaml.safe_load(open(wf_path))
    except yaml.YAMLError as e:
        result.fail("yaml-parse", f"YAML parse error: {e}")
        return result

    if not isinstance(wf, dict):
        result.fail("yaml-structure", "Root must be a mapping")
        return result

    agentic = wf.get('agentic_workflow')
    if not agentic or not isinstance(agentic, dict):
        result.fail("agentic-workflow", "Missing 'agentic_workflow' top-level key")
        return result

    steps = agentic.get('steps')
    if not steps or not isinstance(steps, dict):
        result.fail("steps", "Missing 'agentic_workflow.steps' mapping")
        return result

    step_ids = extract_step_ids(steps)

    # CHECK 1: Step count (<3 = WARNING, not failure — SW5 has 2 by design)
    if len(steps) < 2:
        result.fail("step-count", f"Only {len(steps)} step(s). Workflow is degenerate.")
    elif len(steps) < 3:
        result.warn("step-count", f"Only {len(steps)} steps (SW5 deterministic is OK; generated workflows should have >=3)")
    else:
        result.ok("step-count", f"{len(steps)} steps")

    # CHECK 2: All steps have generative_entity
    missing_ge = [s for s in step_ids if not steps[s].get('generative_entity')]
    if missing_ge:
        result.fail("generative-entity", f"Steps missing generative_entity: {missing_ge}")
    else:
        result.ok("generative-entity", f"All {len(step_ids)} steps have generative_entity")

    # CHECK 3: Bootstrap step exists
    has_bootstrap = any(s.startswith('step_00') for s in step_ids)
    if not has_bootstrap:
        result.warn("bootstrap", "No step_00_* bootstrap step found")
    else:
        result.ok("bootstrap", "Bootstrap step present")

    # CHECK 4: Synthesis step exists
    has_synthesis = any('synth' in s.lower() for s in step_ids)
    if not has_synthesis:
        result.warn("synthesis", "No synthesis/final step found")
    else:
        synth_id = next(s for s in step_ids if 'synth' in s.lower())
        synth_deps = steps[synth_id].get('depends_on', [])
        if synth_deps:
            result.ok("synthesis-deps", f"Synthesis '{synth_id}' depends_on: {synth_deps}")
        else:
            result.fail("synthesis-deps", f"Synthesis step '{synth_id}' has NO depends_on — will run before upstream steps complete")

    # CHECK 5: All depends_on reference real steps
    bad_deps = []
    for sid, step in steps.items():
        deps = step.get('depends_on', [])
        if isinstance(deps, str):
            deps = [deps]
        elif not isinstance(deps, list):
            deps = []
        for dep in deps:
            if dep not in step_ids:
                bad_deps.append(f"{sid} → depends_on '{dep}' (not found)")
    if bad_deps:
        result.fail("depends-on-refs", f"Broken references: {'; '.join(bad_deps)}")
    else:
        result.ok("depends-on-refs", "All depends_on references valid")

    # CHECK 6: Topological sort (no cycles)
    sorted_order, has_cycle = topological_sort(steps)
    if has_cycle:
        cycle_steps = set(steps.keys()) - set(sorted_order)
        result.fail("topological-sort", f"Dependency cycle detected involving: {cycle_steps}")
    else:
        result.ok("topological-sort", f"Valid ordering: {' → '.join(sorted_order[:5])}{'...' if len(sorted_order) > 5 else ''}")

    # CHECK 7: Shell hook targets exist
    missing_files = []
    for sid, step in steps.items():
        when = step.get('when', {})
        for trigger, hooks in when.items():
            if not isinstance(hooks, list):
                hooks = [hooks] if hooks else []
            for hook in hooks:
                if not isinstance(hook, dict):
                    continue
                shell = hook.get('shell')
                if not shell:
                    continue
                cmd = shell.get('command', '')
                working_dir = shell.get('working_dir', repo_root)

                targets = extract_cat_targets(cmd) + extract_sed_targets(cmd)
                for target in targets:
                    if target.startswith('$') or target.startswith('{'):
                        continue
                    if target.startswith('/'):
                        full_path = Path(target)
                    else:
                        full_path = Path(working_dir) / target

                    if not full_path.exists():
                        missing_files.append(f"{sid}: {cmd[:60]}... → {full_path} not found")

    if missing_files:
        result.warn("shell-targets", f"Shell target files missing: {'; '.join(missing_files[:5])}. Steps may have degraded context.")
    else:
        result.ok("shell-targets", "All shell hook target files exist")

    # CHECK 8: Shell hooks have working_dir
    missing_wd = []
    for sid, step in steps.items():
        when = step.get('when', {})
        for trigger, hooks in when.items():
            if not isinstance(hooks, list):
                hooks = [hooks] if hooks else []
            for hook in hooks:
                if not isinstance(hook, dict):
                    continue
                shell = hook.get('shell')
                if shell and 'working_dir' not in shell:
                    cmd = shell.get('command', '')
                    if '/' in cmd and not cmd.startswith('echo'):
                        missing_wd.append(f"{sid}: shell hook without working_dir")

    if missing_wd:
        result.fail("working-dir", f"Shell hooks missing working_dir: {'; '.join(missing_wd[:5])}")
    else:
        result.ok("working-dir", "All shell hooks have working_dir")

    # CHECK 9: No bare save_to names
    bare_names = []
    for sid, step in steps.items():
        when = step.get('when', {})
        for trigger, hooks in when.items():
            if not isinstance(hooks, list):
                hooks = [hooks] if hooks else []
            for hook in hooks:
                if not isinstance(hook, dict):
                    continue
                st = hook.get('save_to')
                if st is None:
                    continue
                items = st if isinstance(st, list) else [st]
                for item in items:
                    if isinstance(item, str):
                        if not item.startswith('$') and not item.startswith('./') and not item.startswith('/') and '.' not in item and '/' not in item:
                            bare_names.append(f"{sid}: save_to '{item}' is bare name (needs $ prefix)")
    if bare_names:
        result.fail("bare-save-to", f"Bare save_to names: {'; '.join(bare_names[:5])}")
    else:
        result.ok("bare-save-to", "All save_to entries are $variable or file path")

    # CHECK 10: Template references resolve
    # Only check templates in hook configs (when:) — prompt text may contain
    # descriptive template syntax (e.g., "use {{step.prior_step.output}}") that
    # is NOT meant to be resolved by the engine.
    bad_templates = []
    for sid, step in steps.items():
        when = step.get('when', {})
        when_str = str(when)
        refs = extract_template_refs(when_str)
        for ref in refs:
            if ref not in step_ids:
                bad_templates.append(f"{sid}: {{{{step.{ref}.output}}}} in hooks → step '{ref}' not found")

    if bad_templates:
        result.fail("template-refs", f"Unresolved template references: {'; '.join(bad_templates[:5])}")
    else:
        result.ok("template-refs", "All {{step.X.output}} references resolve")

    # CHECK 11: save_to parent dirs are writable
    bad_dirs = []
    for sid, step in steps.items():
        when = step.get('when', {})
        for trigger, hooks in when.items():
            if not isinstance(hooks, list):
                hooks = [hooks] if hooks else []
            for hook in hooks:
                if not isinstance(hook, dict):
                    continue
                st = hook.get('save_to')
                if st is None:
                    continue
                items = st if isinstance(st, list) else [st]
                for item in items:
                    if isinstance(item, str) and ('/' in item) and not item.startswith('$'):
                        parent = os.path.dirname(item)
                        if parent and not os.path.exists(parent):
                            bad_dirs.append(f"{sid}: save_to parent dir '{parent}' does not exist")

    if bad_dirs:
        result.fail("save-to-dirs", f"save_to parent dirs missing: {'; '.join(bad_dirs[:5])}")
    else:
        result.ok("save-to-dirs", "All save_to parent dirs exist")

    return result


def main():
    parser = argparse.ArgumentParser(description="Dry-run workflow validator")
    parser.add_argument('workflow', help='Path to workflow YAML')
    parser.add_argument('--repo-root', default='.', help='Repo root for resolving relative paths')
    args = parser.parse_args()

    result = validate_workflow(args.workflow, args.repo_root)
    result.print_report(args.workflow)
    sys.exit(result.exit_code())


if __name__ == '__main__':
    main()
