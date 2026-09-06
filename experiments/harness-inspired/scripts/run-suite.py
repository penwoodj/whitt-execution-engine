import sys, os, json, subprocess, yaml, time

WHITT = '/home/jon/code/whitt-execution-engine/target/release/whitt'
CASES_DIR = 'experiments/harness-inspired/cases'
WORKFLOW_DIR = 'experiments/harness-inspired/workflows'
OUTPUT_DIR = './docs/benchmarks/outputs/output'
CASES = ['hi-str-01', 'hi-json-01', 'hi-list-01', 'hi-fmt-01', 'hi-code-01']

YAML_TEMPLATE = r'''workflow_id: harness_v2___CASE_ID__
name: "Harness v2 - __CASE_ID__"
description: "SCREEN->GEN->CHECK->FIX(x3)->JUDGE. Case: __CASE_ID__"
version: "2.0.0"
author: "Experiment"
tags: [harness, quality-gate, cascade, v2]
schema_version: "2.0.0"
min_schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

models:
  m_worker:
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan
  m_judge:
    name: "Qwen3-4B-Instruct-2507-Q4_K_M"
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

    s01_gen:
      generative_entity: "${models.m_worker}"
      prompt: |
        You are a precise task executor. Follow instructions exactly.

        __CASE_PROMPT__
        Output ONLY the requested result, nothing else.
      model_overrides: { temperature: 0.1, max_tokens: 128 }
      when:
        after_step_succeeds:
          - save_to:
              - "$artifact"
              - "__OUTPUT_DIR__/harness-artifact.txt"
          - shell:
              command: "python3 experiments/harness-inspired/scripts/check-deterministic.py --artifact __OUTPUT_DIR__/harness-artifact.txt --criteria __CASES_DIR__/__CASE_ID__.yml --out __OUTPUT_DIR__/harness-check-result.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{bookmarks.shell_output.exit_code}} == 0'
                then: s05_judge
              - given: '{{bookmarks.shell_output.exit_code}} != 0'
                then: s02_fix_1
          - shell:
              command: "python3 experiments/harness-inspired/scripts/trace-check.py __OUTPUT_DIR__/harness-check-result.json check __OUTPUT_DIR__/harness-trace.jsonl"
              working_dir: "."
              fail_on_error: false
        after_step_fails:
          - log:
              event_fields: [step_name, error_message]
              level: error
          - fail:
              message: "Generation step failed"

    s02_fix_1:
      generative_entity: "${models.m_worker}"
      prompt: |
        You are a format fixer. The output failed automated checks. Fix it.
        Output ONLY the corrected result, nothing else.

        Original output:
        {{step.s01_gen.output}}

        Check failures:
        {{bookmarks.check_failures}}
      model_overrides: { temperature: 0.0, max_tokens: 128 }
      when:
        before_step_starts:
          - shell:
              command: "python3 experiments/harness-inspired/scripts/get-failures.py __OUTPUT_DIR__/harness-check-result.json"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$check_failures"
        after_step_succeeds:
          - save_to:
              - "$artifact"
              - "__OUTPUT_DIR__/harness-artifact.txt"
          - shell:
              command: "python3 experiments/harness-inspired/scripts/check-deterministic.py --artifact __OUTPUT_DIR__/harness-artifact.txt --criteria __CASES_DIR__/__CASE_ID__.yml --out __OUTPUT_DIR__/harness-check-result.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{bookmarks.shell_output.exit_code}} == 0'
                then: s05_judge
              - given: '{{bookmarks.shell_output.exit_code}} != 0'
                then: s03_fix_2
          - shell:
              command: "python3 experiments/harness-inspired/scripts/trace-check.py __OUTPUT_DIR__/harness-check-result.json fix_1 __OUTPUT_DIR__/harness-trace.jsonl"
              working_dir: "."
              fail_on_error: false
        after_step_fails:
          - fail:
              message: "Fix attempt 1 failed"

    s03_fix_2:
      generative_entity: "${models.m_worker}"
      prompt: |
        You are a format fixer. Output failed AGAIN. Fix attempt 2 of 3.
        Output ONLY the corrected result, nothing else.

        Previous attempt:
        {{step.s02_fix_1.output}}

        Check failures:
        {{bookmarks.check_failures}}
      model_overrides: { temperature: 0.0, max_tokens: 128 }
      when:
        before_step_starts:
          - shell:
              command: "python3 experiments/harness-inspired/scripts/get-failures.py __OUTPUT_DIR__/harness-check-result.json"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$check_failures"
        after_step_succeeds:
          - save_to:
              - "$artifact"
              - "__OUTPUT_DIR__/harness-artifact.txt"
          - shell:
              command: "python3 experiments/harness-inspired/scripts/check-deterministic.py --artifact __OUTPUT_DIR__/harness-artifact.txt --criteria __CASES_DIR__/__CASE_ID__.yml --out __OUTPUT_DIR__/harness-check-result.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{bookmarks.shell_output.exit_code}} == 0'
                then: s05_judge
              - given: '{{bookmarks.shell_output.exit_code}} != 0'
                then: s04_fix_3
          - shell:
              command: "python3 experiments/harness-inspired/scripts/trace-check.py __OUTPUT_DIR__/harness-check-result.json fix_2 __OUTPUT_DIR__/harness-trace.jsonl"
              working_dir: "."
              fail_on_error: false
        after_step_fails:
          - fail:
              message: "Fix attempt 2 failed"

    s04_fix_3:
      generative_entity: "${models.m_worker}"
      prompt: |
        You are a format fixer. Output failed AGAIN. FINAL attempt 3 of 3.
        Output ONLY the corrected result, nothing else.

        Previous attempt:
        {{step.s03_fix_2.output}}

        Check failures:
        {{bookmarks.check_failures}}
      model_overrides: { temperature: 0.0, max_tokens: 128 }
      when:
        before_step_starts:
          - shell:
              command: "python3 experiments/harness-inspired/scripts/get-failures.py __OUTPUT_DIR__/harness-check-result.json"
              working_dir: "."
              fail_on_error: false
          - append_to:
              - "$check_failures"
        after_step_succeeds:
          - save_to:
              - "$artifact"
              - "__OUTPUT_DIR__/harness-artifact.txt"
          - shell:
              command: "python3 experiments/harness-inspired/scripts/check-deterministic.py --artifact __OUTPUT_DIR__/harness-artifact.txt --criteria __CASES_DIR__/__CASE_ID__.yml --out __OUTPUT_DIR__/harness-check-result.json"
              working_dir: "."
              fail_on_error: false
          - gwt:
              - given: '{{bookmarks.shell_output.exit_code}} == 0'
                then: s05_judge
              - given: '{{bookmarks.shell_output.exit_code}} != 0'
                then: s06_end_fail
          - shell:
              command: "python3 experiments/harness-inspired/scripts/trace-check.py __OUTPUT_DIR__/harness-check-result.json fix_3 __OUTPUT_DIR__/harness-trace.jsonl"
              working_dir: "."
              fail_on_error: false
        after_step_fails:
          - fail:
              message: "Fix attempt 3 failed"

    s05_judge:
      generative_entity: "${models.m_judge}"
      prompt: |
        You are a quality judge. Evaluate the response against the task.
        Be specific about what is correct or wrong.

        Task: __CASE_OBJECTIVE__
        Response: {{bookmarks.artifact}}

        Output EXACTLY this format (3 lines):
        VERDICT: PASS or FAIL
        REASON: one sentence explaining why
        SCORE: 1-10
      model_overrides: { temperature: 0.0, max_tokens: 128 }
      when:
        after_step_succeeds:
          - save_to:
              - "$verdict"
              - "__OUTPUT_DIR__/harness-verdict.txt"
          - shell:
              command: "python3 experiments/harness-inspired/scripts/trace-judge.py __OUTPUT_DIR__/harness-verdict.txt __OUTPUT_DIR__/harness-trace.jsonl"
              working_dir: "."
              fail_on_error: false
          - log:
              event_fields: [step_name, duration_ms, token_count]
              level: info

    s06_end_fail:
      generative_entity: "${models.m_worker}"
      prompt: "FAIL"
      model_overrides: { temperature: 0.0, max_tokens: 8 }
      when:
        before_step_starts:
          - shell:
              command: "python3 experiments/harness-inspired/scripts/trace-append.py __OUTPUT_DIR__/harness-trace.jsonl end_fail fail"
              working_dir: "."
              fail_on_error: false
          - fail:
              message: "All fix attempts exhausted"
'''

def load_case(case_id):
    with open(os.path.join(CASES_DIR, f'{case_id}.yml')) as f:
        return yaml.safe_load(f)

def gen_yaml(case):
    cid = case['case_id']
    prompt = case['prompt'].strip()
    objective = case.get('objective', '')
    yml = YAML_TEMPLATE
    prompt_indented = '\n'.join('        ' + l for l in prompt.split('\n'))
    yml = yml.replace('__CASE_ID__', cid)
    yml = yml.replace('__CASES_DIR__', CASES_DIR)
    yml = yml.replace('__OUTPUT_DIR__', OUTPUT_DIR)
    yml = yml.replace('__CASE_PROMPT__', prompt_indented)
    yml = yml.replace('__CASE_OBJECTIVE__', objective)
    return yml

def run_case(case_id):
    case = load_case(case_id)
    yml = gen_yaml(case)
    yml_path = os.path.join(WORKFLOW_DIR, f'run-{case_id}.yml')
    with open(yml_path, 'w') as f:
        f.write(yml)

    for fn in ['harness-artifact.txt', 'harness-verdict.txt', 'harness-trace.jsonl', 'harness-check-result.json']:
        p = os.path.join(OUTPUT_DIR, fn)
        if os.path.exists(p):
            os.remove(p)

    t0 = time.time()
    r = subprocess.run(
        [WHITT, 'benchmark', '--workflow', yml_path, '--output-dir', './docs/benchmarks/outputs'],
        capture_output=True, text=True, timeout=180
    )
    elapsed = time.time() - t0

    if r.returncode != 0 and not os.path.exists(os.path.join(OUTPUT_DIR, 'harness-artifact.txt')):
        os.remove(yml_path)
        return {'case': case_id, 'verdict': 'ERROR', 'artifact': r.stderr[:120], 'elapsed': round(elapsed, 1), 'trace': [], 'stderr': r.stderr[-200:]}

    artifact = ''
    verdict = ''
    trace = []
    for fn in ['harness-artifact.txt', 'harness-verdict.txt', 'harness-trace.jsonl']:
        p = os.path.join(OUTPUT_DIR, fn)
        if os.path.exists(p):
            if fn == 'harness-trace.jsonl':
                with open(p) as fh:
                    trace = [json.loads(l) for l in fh if l.strip()]
            else:
                with open(p) as fh:
                    content = fh.read().strip()
                    if fn == 'harness-artifact.txt':
                        artifact = content
                    else:
                        verdict = content

    v = 'PASS' if 'VERDICT: PASS' in verdict else 'FAIL'
    os.remove(yml_path)
    return {'case': case_id, 'verdict': v, 'artifact': artifact[:80], 'elapsed': round(elapsed, 1), 'trace': trace}

def restart_docker():
    subprocess.run(['docker', 'restart', 'whitt-llama-server'], capture_output=True, timeout=60)
    time.sleep(15)
    subprocess.run(['curl', '-sf', 'http://localhost:8080/health'], capture_output=True, timeout=5)

results = []
for i, c in enumerate(CASES):
    if i > 0:
        print(f'  restarting docker...', flush=True)
        restart_docker()
    print(f'=== {c} ===', flush=True)
    try:
        r = run_case(c)
        results.append(r)
        print(f"{r['case']}: {r['verdict']} ({r['elapsed']}s) | {r['artifact']}")
        if r['verdict'] == 'ERROR':
            print(f"  stderr: {r.get('stderr', '')[:200]}")
    except Exception as e:
        results.append({'case': c, 'verdict': 'ERROR', 'artifact': str(e)[:80], 'elapsed': 0, 'trace': []})
        print(f"{c}: ERROR - {e}")
    print(flush=True)

print('=== SUMMARY ===')
pass_count = sum(1 for r in results if r['verdict'] == 'PASS')
print(f'Pass rate: {pass_count}/{len(results)}')
for r in results:
    print(f"  {r['case']}: {r['verdict']} ({r['elapsed']}s)")

with open(os.path.join(OUTPUT_DIR, 'suite-results.json'), 'w') as f:
    json.dump(results, f, indent=2)
print(f'Full results: {OUTPUT_DIR}/suite-results.json')
