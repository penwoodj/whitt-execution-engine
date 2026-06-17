# 04 — Meta-Workflow Specification

| Attribute | Value |
|-----------|-------|
| Workflow ID | `meta_workflow_generator_v6` |
| YAML file | `docs/benchmarks/workflows/meta-workflow-v6.yml` |
| Output | `docs/benchmarks/outputs/meta-workflow/<RUN_ID>/sw5/workflow.yml` (via SW5) |
| Reads from | User prompt (substituted via `__TASK_PLACEHOLDER__`) |
| Model | `qwen35` only |
| Total steps | 8 (bootstrap + 5 SW invocations + 2 verify/finalize) |

---

## 1. Purpose

The META orchestrator does NOT do generative work itself. It exists to:
1. Set up the run directory and seed the input prompt
2. Sequentially invoke the 5 sub-workflows via shell calls to `whitt benchmark`
3. Verify each SW's output exists before advancing to the next
4. Bookmark the final output for retrieval

All heavy lifting is delegated to SW1–SW5. This file is intentionally thin.

## 2. Run ID Convention

Format: `meta-v6-YYYYMMDD-HHMMSS` (UTC), e.g., `meta-v6-20260613-213000`.

Set by `scripts/generate-workflow.sh` and substituted via `__RUN_ID__` placeholder. The
META workflow uses `__RUN_ID__` literally; substitution happens before invocation.

## 3. Task Placeholder Substitution

Multi-line user prompts cannot be safely `sed`-substituted into YAML strings without
escaping. The proven pattern from v5 is:

1. Write user prompt to `<RUN_ID>/input/prompt.txt` (performed by bootstrap step's shell hook).
2. Sub-workflows `cat` the file via shell hook to inject into LLM context.

Therefore, the META workflow itself only needs `__TASK_PLACEHOLDER__` substitution
for the prompt-echo step. For everything else, file-system passing wins.

## 4. Pipeline

| Step | Purpose |
|------|---------|
| `step_00_bootstrap` | mkdir run-dirs, write prompt file, log start |
| `step_01_invoke_sw1` | shell → `whitt benchmark --workflow sw1-task-deconstruction.yml` |
| `step_02_invoke_sw2` | shell → `whitt benchmark --workflow sw2-desired-output-state.yml` |
| `step_03_invoke_sw3` | shell → `whitt benchmark --workflow sw3-agentic-categorization.yml` |
| `step_04_invoke_sw4` | shell → `whitt benchmark --workflow sw4-yaml-substructure-translation.yml` |
| `step_05_invoke_sw5` | shell → `whitt benchmark --workflow sw5-final-workflow-assembly.yml` |
| `step_06_verify_final` | shell → check sw5/workflow.yml exists + validate-yaml.py |
| `step_99_finalize` | bookmark final workflow path + log completion |

## 5. Detailed YAML Specification

```yaml
workflow_id: meta_workflow_generator_v6
name: "Meta-Workflow Generator v6"
description: "Orchestrates 5 sub-workflows to generate executable agentic YAML from a prompt."
version: "1.0.0"
author: "Whitt Execution Engine"
tags: [meta-workflow, orchestrator, shell-based]
schema_version: "2.0.0"
min_schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

models:
  "qwen35":
    name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
    host:
      type: llama_cpp_with_vulkan

workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: true

agentic_workflow:
  when:
    after_workflow:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info

  steps:

    step_00_bootstrap:
      generative_entity: "${models.qwen35}"
      prompt: |
        Bootstrapping meta-workflow v6 run __RUN_ID__.
        Output "BOOTSTRAPPED".
      model_overrides:
        temperature: 0.0
        max_tokens: 10
      when:
        before_step_starts:
          - shell:
              command: "mkdir -p ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/{sw1,sw2,sw3,sw4,sw5,logs,input} && printf '%s' '__TASK_PLACEHOLDER__' > ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/prompt.txt && wc -c ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/prompt.txt"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, error_message]
              level: error

    step_01_invoke_sw1:
      generative_entity: "${models.qwen35}"
      depends_on: [step_00_bootstrap]
      prompt: |
        Invoking SW1 (task-deconstruction). Output "SW1_INVOKED".
      model_overrides: { temperature: 0.0, max_tokens: 10 }
      when:
        before_step_starts:
          - shell:
              command: "whitt benchmark --workflow ./docs/benchmarks/workflows/sw1-task-deconstruction.yml --output-dir ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1 --run-id __RUN_ID__ 2>&1 | tee ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw1-stdout.log; test -s ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/tasks.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - shell:
              command: "wc -l ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/tasks.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: false
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms]
              level: info

    step_02_invoke_sw2:
      generative_entity: "${models.qwen35}"
      depends_on: [step_01_invoke_sw1]
      prompt: |
        Invoking SW2 (desired-output-state). Output "SW2_INVOKED".
      model_overrides: { temperature: 0.0, max_tokens: 10 }
      when:
        before_step_starts:
          - shell:
              command: "whitt benchmark --workflow ./docs/benchmarks/workflows/sw2-desired-output-state.yml --output-dir ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2 --run-id __RUN_ID__ 2>&1 | tee ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2-stdout.log; test -s ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/outputs.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms]
              level: info

    step_03_invoke_sw3:
      generative_entity: "${models.qwen35}"
      depends_on: [step_02_invoke_sw2]
      prompt: |
        Invoking SW3 (agentic-categorization). Output "SW3_INVOKED".
      model_overrides: { temperature: 0.0, max_tokens: 10 }
      when:
        before_step_starts:
          - shell:
              command: "whitt benchmark --workflow ./docs/benchmarks/workflows/sw3-agentic-categorization.yml --output-dir ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3 --run-id __RUN_ID__ 2>&1 | tee ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw3-stdout.log; test -s ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3/categories.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true

    step_04_invoke_sw4:
      generative_entity: "${models.qwen35}"
      depends_on: [step_03_invoke_sw3]
      prompt: |
        Invoking SW4 (yaml-substructure-translation). Output "SW4_INVOKED".
      model_overrides: { temperature: 0.0, max_tokens: 10 }
      when:
        before_step_starts:
          - shell:
              command: "whitt benchmark --workflow ./docs/benchmarks/workflows/sw4-yaml-substructure-translation.yml --output-dir ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw4 --run-id __RUN_ID__ 2>&1 | tee ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw4-stdout.log; test -s ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw4/structs.md"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true

    step_05_invoke_sw5:
      generative_entity: "${models.qwen35}"
      depends_on: [step_04_invoke_sw4]
      prompt: |
        Invoking SW5 (final-workflow-assembly). Output "SW5_INVOKED".
      model_overrides: { temperature: 0.0, max_tokens: 10 }
      when:
        before_step_starts:
          - shell:
              command: "whitt benchmark --workflow ./docs/benchmarks/workflows/sw5-final-workflow-assembly.yml --output-dir ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw5 --run-id __RUN_ID__ 2>&1 | tee ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw5-stdout.log; test -s ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw5/workflow.yml"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true

    step_06_verify_final:
      generative_entity: "${models.qwen35}"
      depends_on: [step_05_invoke_sw5]
      prompt: |
        Verifying final workflow file. Output result.
      model_overrides: { temperature: 0.0, max_tokens: 100 }
      when:
        before_step_starts:
          - shell:
              command: "python3 scripts/validate-yaml.py ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw5/workflow.yml 2>&1 | tee ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/final-validation.log"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: false
        after_step_succeeds:
          - save_to:
              - meta_final_validation
              - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/final-validation-result.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms]
              level: info

    step_99_finalize:
      generative_entity: "${models.qwen35}"
      depends_on: [step_06_verify_final]
      prompt: |
        Confirming meta-workflow v6 completion. Output "META_DONE".
      model_overrides: { temperature: 0.0, max_tokens: 10 }
      when:
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms, total_workflow_duration_ms]
              level: info
          - bookmark:
              detailed:
                path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw5/workflow.yml"
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, error_message]
              level: error
```

## 6. Failure Handling

If any SW invocation fails (`fail_on_error: true` on shell hook), the META workflow
aborts. The operator inspects `<RUN_ID>/logs/sw<N>-stdout.log` to diagnose.

For development/iteration, set `fail_on_error: false` to allow META to continue even
if an SW fails, producing partial output for diagnosis.

## 7. Invocation Pattern

```bash
# 1. Set up env
export RUN_ID="meta-v6-$(date -u +%Y%m%d-%H%M%S)"
export PROMPT_TEXT="..."

# 2. Substitute and run
./scripts/generate-workflow.sh \
    --template docs/benchmarks/workflows/meta-workflow-v6.yml.template \
    --run-id "$RUN_ID" \
    --prompt "$PROMPT_TEXT" \
    --output docs/benchmarks/workflows/meta-workflow-v6.run.yml

# 3. Execute
whitt benchmark --workflow docs/benchmarks/workflows/meta-workflow-v6.run.yml \
    --output-dir "./docs/benchmarks/outputs/meta-workflow/$RUN_ID"
```

## 8. Time Budget (Qwen3.5-9B CPU-only)

| Step | Estimated Time |
|------|----------------|
| Bootstrap | < 1 min |
| SW1 (task-deconstruction, 4-12 tasks) | 15-30 min |
| SW2 (desired-output-state, 4-12 tasks) | 20-40 min |
| SW3 (agentic-categorization, 4-12 tasks) | 20-35 min |
| SW4 (yaml-substructure-translation, 4 chunks) | 25-45 min |
| SW5 (final-workflow-assembly + per-task verify) | 30-60 min |
| Verify + finalize | < 5 min |
| **Total** | **~2-4 hours per prompt** |

Plan accordingly. Long prompts (high task count) trend toward upper bounds.
