#!/usr/bin/env python3
"""Suite runner for harness v3 (agentic ha-* cases).

Generates per-case workflow YAML from template, validates, runs against
live engine, collects results. Caveman prompts, plan-then-solve gen,
leak-safe fix feedback, deterministic-first judge gating, full trace.

Usage: python3 scripts/run-ha-suite.py [--cases ha-01,ha-02] [--dry-run]
"""
import argparse, json, os, subprocess, sys, time
import yaml

WHITT = '/home/jon/code/whitt-execution-engine/target/release/whitt'
CASES_DIR = 'experiments/harness-inspired/cases'
WORKFLOW_DIR = 'experiments/harness-inspired/workflows'
OUTPUT_DIR = './docs/benchmarks/outputs/output'
ALL_CASES = [f'ha-{i:02d}' for i in range(1, 11)]

YAML_TEMPLATE = r'''workflow_id: harness_v3___CASE_ID__
name: "Harness v3 - __CASE_ID__"
description: "SCREEN->SOLVE->CHECK->FIX(2, leak-safe)->JUDGE. Case: __CASE_ID__"
version: "3.0.0"
author: "Experiment"
tags: [harness, agentic, leak-safe, v3]
schema_version: "2.0.0"
min_schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

models:
  m_worker:
    name: "Qwen3-5-9B-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  m_judge:
    name: "Qwen3-5-9B-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan

workflow_execution_strategy:
  timing:
    cooldown_after_unload_secs: 0
    min_tmp_space_mb: 50
  memory:
    model_lifecycle:
      unload_unused: false

agentic_workflow:
  when:
    after_workflow:
      - log:
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info
  steps:
    s00_screen:
      generative_entity: "${models.m_worker}"
      prompt: "pre-flight"
      model_overrides: { temperature: 0.0, max_tokens: 1 }
      when:
        before_step_starts:
          - shell:
              command: "python3 experiments/harness-inspired/scripts/screen-validate.py __CASES_DIR__/__CASE_ID__.yml"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{bookmarks.shell_output.exit_code}} != 0'
                then: s06_end_fail
        after_step_succeeds:
          - log:
              event_fields: [step_name, duration_ms]
              level: info

    s01_solve:
      generative_entity: "${models.m_worker}"
      prompt: |
        ROLE: on-call operator. Precise. Zero fluff.

        __CASE_PROMPT__

        METHOD: FIRST list every component the report asks for. THEN
        restate each rule briefly. Walk the scenario step by step in plain
        text, tracking counts as you go. Entities you do not manage NEVER
        count. Check deadline/budget rules BEFORE finalizing. Verify every
        requested component appears in your answer.
        FINAL LINE: the JSON object alone. Nothing after it.
      model_overrides: { temperature: 0.1, max_tokens: 1280 }
      when:
        after_step_succeeds:
          - save_to:
              - "$artifact"
              - "__OUTPUT_DIR__/ha-artifact.txt"
          - shell:
              command: "python3 experiments/harness-inspired/scripts/check-deterministic.py --artifact __OUTPUT_DIR__/ha-artifact.txt --criteria __CASES_DIR__/__CASE_ID__.yml --out __OUTPUT_DIR__/ha-check-result.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{bookmarks.shell_output.exit_code}} == 0'
                then: s05_judge
              - given: '{{bookmarks.shell_output.exit_code}} != 0'
                then: s02_fix_1
          - shell:
              command: "python3 experiments/harness-inspired/scripts/trace-check.py __OUTPUT_DIR__/ha-check-result.json check __OUTPUT_DIR__/ha-trace.jsonl __CASE_ID__"
              working_dir: "."
              fail_on_error: false
        after_step_fails:
          - log:
              event_fields: [step_name, error_message]
              level: error
          - fail:
              message: "Solve step failed"

    s02_fix_1:
      generative_entity: "${models.m_worker}"
      prompt: |
        ROLE: correction specialist. Attempt failed checks. Fix output.

        Failed checks (names only, values withheld):
        __FEEDBACK__

        Prior attempt:
        {{step.s01_solve.output}}

        METHOD: re-derive every value from task rules. Do NOT copy prior
        attempt. Walk rules step by step in plain text. Track counts.
        FINAL LINE: corrected JSON object alone.
      model_overrides: { temperature: 0.0, max_tokens: 1280 }
      when:
        before_step_starts:
          - shell:
              command: "python3 experiments/harness-inspired/scripts/feedback.py --result __OUTPUT_DIR__/ha-check-result.json --artifact __OUTPUT_DIR__/ha-artifact.txt --out __OUTPUT_DIR__/ha-feedback.txt"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$feedback_bookmark"
        after_step_succeeds:
          - save_to:
              - "$artifact"
              - "__OUTPUT_DIR__/ha-artifact.txt"
          - shell:
              command: "python3 experiments/harness-inspired/scripts/check-deterministic.py --artifact __OUTPUT_DIR__/ha-artifact.txt --criteria __CASES_DIR__/__CASE_ID__.yml --out __OUTPUT_DIR__/ha-check-result.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{bookmarks.shell_output.exit_code}} == 0'
                then: s05_judge
              - given: '{{bookmarks.shell_output.exit_code}} != 0'
                then: s03_fix_2
          - shell:
              command: "python3 experiments/harness-inspired/scripts/trace-check.py __OUTPUT_DIR__/ha-check-result.json fix_1 __OUTPUT_DIR__/ha-trace.jsonl __CASE_ID__"
              working_dir: "."
              fail_on_error: false
        after_step_fails:
          - fail:
              message: "Fix attempt 1 failed"

    s03_fix_2:
      generative_entity: "${models.m_worker}"
      prompt: |
        ROLE: escalation solver. Two attempts wrong. Solve from scratch.

        Original task:
        __CASE_PROMPT__

        Failed checks (names only, values withheld):
        __FEEDBACK__

        METHOD: discard prior attempts entirely. Re-read every rule.
        FIRST list every component the report asks for. THEN compute each
        component in plain text. THEN sum or combine them. Verify nothing
        requested is missing from the JSON. Entities you do not manage
        NEVER count.
        FINAL LINE: final JSON object alone.
      model_overrides: { temperature: 0.0, max_tokens: 2400 }
      when:
        before_step_starts:
          - shell:
              command: "python3 experiments/harness-inspired/scripts/feedback.py --result __OUTPUT_DIR__/ha-check-result.json --artifact __OUTPUT_DIR__/ha-artifact.txt --out __OUTPUT_DIR__/ha-feedback.txt"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$feedback_bookmark"
        after_step_succeeds:
          - save_to:
              - "$artifact"
              - "__OUTPUT_DIR__/ha-artifact.txt"
          - shell:
              command: "python3 experiments/harness-inspired/scripts/check-deterministic.py --artifact __OUTPUT_DIR__/ha-artifact.txt --criteria __CASES_DIR__/__CASE_ID__.yml --out __OUTPUT_DIR__/ha-check-result.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{bookmarks.shell_output.exit_code}} == 0'
                then: s05_judge
              - given: '{{bookmarks.shell_output.exit_code}} != 0'
                then: s04_end_fail
          - shell:
              command: "python3 experiments/harness-inspired/scripts/trace-check.py __OUTPUT_DIR__/ha-check-result.json fix_2 __OUTPUT_DIR__/ha-trace.jsonl __CASE_ID__"
              working_dir: "."
              fail_on_error: false
        after_step_fails:
          - fail:
              message: "Fix attempt 2 failed"

    s05_judge:
      generative_entity: "${models.m_judge}"
      prompt: |
        ROLE: blind auditor. See only task + result. No author knowledge.

        Task objective: __CASE_OBJECTIVE__
        Result: {{bookmarks.artifact}}

        Check: does result answer task completely? Numbers plausible?
        Format exact JSON with required keys?
        Output EXACTLY 3 lines:
        VERDICT: PASS or FAIL
        REASON: one sentence
        SCORE: 1-10
      model_overrides: { temperature: 0.0, max_tokens: 128 }
      when:
        after_step_succeeds:
          - save_to:
              - "$verdict"
              - "__OUTPUT_DIR__/ha-verdict.txt"
          - shell:
              command: "python3 experiments/harness-inspired/scripts/trace-judge.py __OUTPUT_DIR__/ha-verdict.txt __OUTPUT_DIR__/ha-trace.jsonl __CASE_ID__"
              working_dir: "."
              fail_on_error: false
          - log:
              event_fields: [step_name, duration_ms, token_count]
              level: info

    s04_end_fail:
      generative_entity: "${models.m_worker}"
      prompt: "FAIL"
      model_overrides: { temperature: 0.0, max_tokens: 8 }
      when:
        before_step_starts:
          - shell:
              command: "python3 experiments/harness-inspired/scripts/trace-append.py __OUTPUT_DIR__/ha-trace.jsonl end_fail fail case=__CASE_ID__"
              working_dir: "."
              fail_on_error: false
          - fail:
              message: "All attempts exhausted"
'''


def load_case(case_id):
    with open(os.path.join(CASES_DIR, f'{case_id}.yml')) as f:
        return yaml.safe_load(f)


def gen_yaml(case, feedback_placeholder='__FEEDBACK__'):
    cid = case['case_id']
    prompt_indented = '\n'.join('        ' + l for l in case['prompt'].strip().split('\n'))
    objective = case.get('objective', '')
    yml = YAML_TEMPLATE
    yml = yml.replace('__CASE_ID__', cid)
    yml = yml.replace('__CASES_DIR__', CASES_DIR)
    yml = yml.replace('__OUTPUT_DIR__', OUTPUT_DIR)
    yml = yml.replace('__CASE_PROMPT__', prompt_indented)
    yml = yml.replace('__CASE_OBJECTIVE__', objective)
    yml = yml.replace(feedback_placeholder, '{{bookmarks.feedback_bookmark}}')
    return yml


def restart_docker():
    subprocess.run(['docker', 'restart', 'whitt-llama-server'], capture_output=True, timeout=90)
    time.sleep(15)


def run_case(case_id, keep_yaml=False):
    case = load_case(case_id)
    yml = gen_yaml(case)
    yml_path = os.path.join(WORKFLOW_DIR, f'run-v3-{case_id}.yml')
    with open(yml_path, 'w') as f:
        f.write(yml)

    for fn in ['ha-artifact.txt', 'ha-verdict.txt', 'ha-trace.jsonl', 'ha-check-result.json', 'ha-feedback.txt']:
        p = os.path.join(OUTPUT_DIR, fn)
        if os.path.exists(p):
            os.remove(p)

    t0 = time.time()
    env = dict(os.environ, WHITT_ZOMBIE_MAX='4')
    r = subprocess.run(
        [WHITT, 'benchmark', '--workflow', yml_path, '--output-dir', './docs/benchmarks/outputs'],
        capture_output=True, text=True, timeout=300, env=env,
    )
    elapsed = time.time() - t0

    artifact = verdict = ''
    trace = []
    ap = os.path.join(OUTPUT_DIR, 'ha-artifact.txt')
    vp = os.path.join(OUTPUT_DIR, 'ha-verdict.txt')
    tp = os.path.join(OUTPUT_DIR, 'ha-trace.jsonl')
    if os.path.exists(ap):
        artifact = Path_read(ap)
    if os.path.exists(vp):
        verdict = Path_read(vp)
    if os.path.exists(tp):
        with open(tp) as fh:
            trace = [json.loads(l) for l in fh if l.strip()]

    if not artifact and r.returncode != 0:
        if not keep_yaml:
            os.remove(yml_path)
        return {'case': case_id, 'verdict': 'ERROR', 'artifact': r.stderr[:150].replace('\n', ' '),
                'elapsed': round(elapsed, 1), 'trace': []}

    last_check = [t for t in trace if t.get('gate', '').endswith(('check', 'fix_1', 'fix_2'))]
    det_pass = any(t.get('verdict') == 'pass' for t in last_check)
    v = 'PASS' if det_pass else 'FAIL'
    if not keep_yaml:
        os.remove(yml_path)
    return {'case': case_id, 'verdict': v, 'artifact': artifact[:100], 'elapsed': round(elapsed, 1), 'trace': trace}


def Path_read(p):
    with open(p) as f:
        return f.read().strip()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--cases', default=','.join(ALL_CASES), help='comma-separated case ids')
    ap.add_argument('--dry-run', action='store_true', help='generate YAMLs only, no execution')
    ap.add_argument('--keep-yaml', action='store_true')
    ap.add_argument('--no-restart', action='store_true', help='skip docker restart between cases')
    args = ap.parse_args()

    cases = [c.strip() for c in args.cases.split(',') if c.strip()]

    if args.dry_run:
        for c in cases:
            case = load_case(c)
            path = os.path.join(WORKFLOW_DIR, f'run-v3-{c}.yml')
            with open(path, 'w') as f:
                f.write(gen_yaml(case))
            print(f'generated {path}')
        return

    results = []
    for i, c in enumerate(cases):
        if i > 0 and not args.no_restart:
            print(f'  [docker restart]', flush=True)
            restart_docker()
        print(f'=== {c} ===', flush=True)
        try:
            r = run_case(c, keep_yaml=args.keep_yaml)
            results.append(r)
            print(f"{r['case']}: {r['verdict']} ({r['elapsed']}s) | {r['artifact'][:70]}", flush=True)
        except Exception as e:
            results.append({'case': c, 'verdict': 'ERROR', 'artifact': str(e)[:100], 'elapsed': 0, 'trace': []})
            print(f"{c}: ERROR - {e}", flush=True)

    print('\n=== SUMMARY ===')
    pass_ct = sum(1 for r in results if r['verdict'] == 'PASS')
    print(f'Pass rate: {pass_ct}/{len(results)}')
    for r in results:
        gates = ' -> '.join(f"{t.get('gate','?')}={t.get('verdict','?')}" for t in r['trace'])
        print(f"  {r['case']}: {r['verdict']} ({r['elapsed']}s) [{gates}]")

    out = os.path.join(OUTPUT_DIR, 'ha-suite-results.json')
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    with open(out, 'w') as f:
        json.dump(results, f, indent=2)
    print(f'Full results: {out}')


if __name__ == '__main__':
    main()
