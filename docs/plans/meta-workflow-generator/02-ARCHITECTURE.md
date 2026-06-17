# 02 — Architecture

This document fixes the architectural decisions for the meta-workflow generator v6.
Every YAML file we produce must conform to this architecture.

---

## 1. Top-Level Pattern: Shell-Based Orchestration

### 1.1 Why Not `sub_workflow:` Step Key?

The schema (`unified-workflow-schema.yml:395-411`) defines a `sub_workflow:` step key:

```yaml
run_validation:
  sub_workflow: validate_workflow
  input:
    target_code: "{{step.analyze_code.output}}"
```

**Engine reality**: this is a STUB. Background-agent verification on 2026-06-13 confirmed:

- `SubWorkflowRef` type exists in `src/workflow/step.rs:26-33` and is parsed correctly.
- `sub_workflows:` top-level key is in `src/workflow/schema.rs:24` allowed keys.
- `SubWorkflowResolution` enum exists at `src/workflow/schema.rs:698`.
- **BUT**: `src/benchmark/runner.rs` has NO execution path that handles a step with
  `sub_workflow:` set. The step is silently ignored.

Calling `sub_workflow:` from inside a workflow therefore produces nothing. We cannot
use it.

### 1.2 The Shell-Orchestration Pattern

Each "sub-workflow" is a standalone YAML file runnable as
`whitt benchmark --workflow <file>`. The META orchestrator invokes them via **shell
hooks**:

```yaml
meta_step_invoke_sw1:
  generative_entity: "${models.qwen35}"
  prompt: |
    Confirming SW1 invocation.
  model_overrides:
    temperature: 0.0
    max_tokens: 10
  when:
    before_step_starts:
      - shell:
          command: "whitt benchmark --workflow docs/benchmarks/workflows/sw1-task-deconstruction.yml --output-dir docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1 --prompt-file docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input-prompt.txt"
          args: []
          working_dir: "/home/jon/code/whitt-execution-engine"
          fail_on_error: true
      - bookmark:
          detailed:
            path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/invocation.txt"
    after_step_succeeds:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
          event_fields: [step_name, duration_ms]
```

### 1.3 State Passing Between Sub-Workflows

Since each `whitt benchmark` invocation is a separate process, in-memory state does
NOT carry over. **All state passes through the filesystem.**

Convention:
- Each sub-workflow N writes its single output to:
  `docs/benchmarks/outputs/meta-workflow/<RUN_ID>/sw<N>/<output>.md`
- Sub-workflow N+1 reads predecessor output via a `shell: cat <path>` hook in
  `before_step_starts`, storing it as `{{bookmarks.prior_context.stdout}}`.
- The META orchestrator coordinates the run-id directory creation.

### 1.4 Run ID & Prompt Substitution

Following the proven v5 pattern (`scripts/generate-workflow.sh`):

1. **Placeholders** in YAML: `__RUN_ID__` and `__TASK_PLACEHOLDER__`.
2. **Substitution**: `scripts/generate-workflow.sh` does `sed -i` substitution before
   `whitt benchmark` runs.
3. **RUN_ID format**: `meta-v6-YYYYMMDD-HHMMSS` (timestamp-based for natural sorting).
4. **TASK_PLACEHOLDER**: Multi-line prompt string, escaped for sed.

The META workflow itself uses these placeholders; the orchestrator script handles
substitution before invoking META. Sub-workflows receive their inputs via
`before_step_starts` shell hooks that `cat` the prior output, so they do NOT need
TASK_PLACEHOLDER substitution themselves (except SW1 which reads the raw prompt).

---

## 2. Sub-Workflow Internal Pattern

Every sub-workflow follows the same internal shape:

```
┌─────────────────────────────────────────────────────────────────┐
│  SUB-WORKFLOW-N.YML                                             │
│                                                                 │
│  ┌──────────────────┐                                           │
│  │ step_00_bootstrap│  shell: mkdir run-dirs, cat prior output  │
│  └────────┬─────────┘                                           │
│           │                                                     │
│  ┌────────▼─────────┐                                           │
│  │ step_01_load_ctx │  LLM: "Read prior context. Confirm."      │
│  └────────┬─────────┘                                           │
│           │                                                     │
│  ┌────────▼─────────────────────────────────┐                  │
│  │ step_02_chunk_or_iterate                 │                  │
│  │   before_step_starts:                    │                  │
│  │     iterate_values:                      │                  │
│  │       chunk_id: [0, 1, 2, ...]           │                  │
│  │   prompt: "Process chunk {{loop.chunk_id}}" │               │
│  └────────┬─────────────────────────────────┘                  │
│           │                                                     │
│  ┌────────▼─────────┐                                           │
│  │ step_03_evaluate │  GWT: quality checks on accumulated       │
│  │                  │  output. route_to fix step if fails.      │
│  └────────┬─────────┘                                           │
│           │                                                     │
│  ┌────────▼─────────┐                                           │
│  │ step_04_fix      │  LLM: address evaluation failures.        │
│  │                  │  route_to step_03_evaluate (re-eval).     │
│  └────────┬─────────┘                                           │
│           │                                                     │
│  ┌────────▼─────────┐                                           │
│  │ step_99_finalize │  save_to: <single-md-file>. Save_to var.  │
│  └──────────────────┘                                           │
└─────────────────────────────────────────────────────────────────┘
```

### 2.1 Common Step Skeleton

```yaml
step_XX_<name>:
  generative_entity: "${models.qwen35}"
  depends_on: [step_YY_prev]
  prompt: |
    <role-and-context>
    <input-data-via-{{bookmarks.X.stdout}}-or-{{step.Y.output}}>

    <task-instruction>

    <output-format-spec>
    Output ONLY the result.
  model_overrides:
    temperature: 0.2
    max_tokens: 1500
  when:
    before_step_starts:
      - shell: { command: "...", args: [], working_dir: "/home/jon/code/whitt-execution-engine", fail_on_error: false }
      - bookmark: { detailed: { path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw<N>/XX-pre.txt" } }
    after_step_succeeds:
      - append_to: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw<N>/output.md"
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw<N>.log"
          event_fields: [step_name, duration_ms, total_tokens, quality_score]
          level: info
    after_step_fails:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw<N>.log"
          event_fields: [step_name, error_message, error.type]
          level: error
```

### 2.2 GWT Quality Gate Pattern

After every chunk-iteration step, a GWT step gates progression:

```yaml
step_03_evaluate:
  generative_entity: "${models.qwen35}"
  depends_on: [step_02_chunk_or_iterate]
  prompt: |
    Read the accumulated output:
    {{bookmarks.sw_N_output.stdout}}

    Evaluate against criteria:
    - <criterion-1>
    - <criterion-2>

    For each criterion, output: PASS or FAIL with one-sentence reason.
    End with VERDICT: PASS or VERDICT: FAIL.
  model_overrides:
    temperature: 0.1
    max_tokens: 800
  when:
    after_step_succeeds:
      - save_to:
          - sw_N_eval
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw<N>/eval-{{loop.iteration}}.txt"
      - gwt:
          - given: "{{step.step_03_evaluate.output}} contains 'VERDICT: PASS'"
            then: { route_to: step_99_finalize }
          - given: "true"
            then: { route_to: step_04_fix }
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw<N>.log"
          event_fields: [step_name, duration_ms]
          level: info
```

### 2.3 Iteration Loop via `iterate_values`

For chunked processing of N items, use `iterate_values` in `before_step_starts`:

```yaml
step_02_process_chunks:
  generative_entity: "${models.qwen35}"
  depends_on: [step_01_load_ctx]
  prompt: |
    Process chunk index {{loop.chunk_id}}.
    Prior accumulated output:
    {{bookmarks.sw_N_partial.stdout}}

    <task-instruction-for-this-chunk>

    Output ONLY the chunk result.
  model_overrides:
    temperature: 0.2
    max_tokens: 1200
  when:
    before_step_starts:
      - iterate_values:
          chunk_id: ["0", "1", "2", "3", "4"]
      - shell:
          command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw<N>/output.md 2>/dev/null || echo '__INITIAL__'"
          args: []
          working_dir: "/home/jon/code/whitt-execution-engine"
          fail_on_error: false
    after_step_succeeds:
      - append_to:
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw<N>/output.md"
          - sw_N_partial
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw<N>.log"
          event_fields: [step_name, loop.iteration, duration_ms, total_tokens]
          level: info
    after_loop_iteration_fails:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw<N>.log"
          event_fields: [iteration, error_message]
          level: warning
```

⚠️ **`iterate_values` quirk**: The `execute_action` handler at `actions.rs:62-65` is a
STUB. However, `runner.rs:800-836` has a separate `extract_iterate_values` function
that parses `iterate_values` directly from the hook config and uses it to drive
step iteration. **The hook MUST live inside `before_step_starts` to be picked up.**

---

## 3. Meta-Workflow Pattern

```yaml
workflow_id: meta_workflow_generator_v6
name: "Meta-Workflow Generator v6"
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
        Confirming run bootstrap. Output "BOOTSTRAPPED".
      model_overrides: { temperature: 0.0, max_tokens: 10 }
      when:
        before_step_starts:
          - shell:
              command: "mkdir -p ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/{sw1,sw2,sw3,sw4,sw5,logs,input} && echo '__TASK_PLACEHOLDER__' > ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/prompt.txt"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true

    step_01_invoke_sw1:
      generative_entity: "${models.qwen35}"
      depends_on: [step_00_bootstrap]
      prompt: |
        Confirming SW1 (task-deconstruction) invocation. Output "SW1_INVOKED".
      model_overrides: { temperature: 0.0, max_tokens: 10 }
      when:
        before_step_starts:
          - shell:
              command: "whitt benchmark --workflow ./docs/benchmarks/workflows/sw1-task-deconstruction.yml --output-dir ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1 --run-id __RUN_ID__ 2>&1 | tee ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw1-stdout.log"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true

    # ... SW2, SW3, SW4, SW5 follow same pattern ...

    step_99_finalize:
      generative_entity: "${models.qwen35}"
      depends_on: [step_05_invoke_sw5]
      prompt: |
        Confirming meta-workflow completion. Output "META_DONE".
      model_overrides: { temperature: 0.0, max_tokens: 10 }
      when:
        after_step_succeeds:
          - log:
              to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/meta.log"
              event_fields: [step_name, duration_ms]
              level: info
          - bookmark: true
```

---

## 4. Data Flow Diagram

```
User prompt
     │
     ▼
┌──────────────────────────────────────────────────────────┐
│  scripts/generate-workflow.sh                            │
│  - sed __TASK_PLACEHOLDER__ → prompt                     │
│  - sed __RUN_ID__ → timestamp                            │
└──────────────────────────────────────────────────────────┘
     │
     ▼
┌──────────────────────────────────────────────────────────┐
│  whitt benchmark --workflow meta-workflow-v6.yml         │
│  ┌────────────────────────────────────────────────────┐  │
│  │  step_00_bootstrap: mkdir run-dirs, write prompt   │  │
│  │  step_01_invoke_sw1: shell → whitt benchmark SW1   │  │
│  │  step_02_invoke_sw2: shell → whitt benchmark SW2   │  │
│  │  step_03_invoke_sw3: shell → whitt benchmark SW3   │  │
│  │  step_04_invoke_sw4: shell → whitt benchmark SW4   │  │
│  │  step_05_invoke_sw5: shell → whitt benchmark SW5   │  │
│  │  step_99_finalize: log + bookmark                  │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
     │
     ├──► SW1: input/prompt.txt  ──► sw1/tasks.md
     │                                  │
     ├──► SW2: sw1/tasks.md      ──► sw2/outputs.md
     │                                  │
     ├──► SW3: sw1+sw2           ──► sw3/categories.md
     │                                  │
     ├──► SW4: sw1+sw2+sw3       ──► sw4/structs.md
     │                                  │
     └──► SW5: sw4/structs.md    ──► sw5/workflow.yml
                                            │
                                            ▼
                                       FINAL OUTPUT
```

---

## 5. Iteration Loop Architecture (Inside Each SW)

```
                  ┌─────────────────────────┐
                  │ step_02_process_chunk_N │  ← LLM call, max_retries=3
                  └───────────┬─────────────┘
                              │
                              ▼
                  ┌─────────────────────────┐
                  │ step_03_evaluate        │  ← LLM evaluates; GWT gates
                  └───────────┬─────────────┘
                              │
                   ┌──────────┴──────────┐
                   │                     │
              VERDICT: PASS         VERDICT: FAIL
                   │                     │
                   ▼                     ▼
            step_99_finalize    step_04_fix_issues
                                       │
                                       │  (LLM addresses failures)
                                       │
                                       └──► back to step_03_evaluate
                                                       (re-eval)
```

**Retry budget**: step-level `retry.max_attempts: 3` for LLM steps. Workflow-level
`retry.max_attempts: 10`. Beyond this, fail and surface to operator.

**Loop safety**: Maximum of 5 fix iterations per SW before forced advance with WARN log.

---

## 6. Failure Modes & Mitigations

| Failure | Mitigation |
|---------|------------|
| Qwen3.5-9B too slow (>10min/step) | Set `timeout:` per step; use smaller `max_tokens` |
| Output truncated (max_tokens hit) | Increase `max_tokens` in `model_overrides`; chunk smaller |
| YAML parse error in SW5 output | `scripts/fix-generated-yaml.py` auto-fixes; re-validate |
| Markdown fences leak into YAML | Shell hook strips via `sed 's/```[a-z]*//g'` |
| Sub-workflow crash kills META | `fail_on_error: false` on the shell hook + GWT gate on output existence |
| Quality gate loops forever | Hard cap: 5 fix iterations, then WARN + advance |
| Run directory pollution | Each run gets unique `__RUN_ID__` directory; cleanup script in `scripts/` |
| Prompt placeholder not substituted | `scripts/generate-workflow.sh` runs as first step; meta bootstrap verifies |

---

## 7. Build Order (Strict Sequence)

1. **Qwen3.5-9B Docker config** — model loads, `/props` responds, `/v1/chat/completions` returns text
2. **Test dataset** — 5+ prompts curated in `06-TEST-DATASET.md`
3. **Baselines** — Sisyphus manually produces `tasks.md`, `outputs.md`, etc. for each prompt
4. **SW1** — task-deconstruction.yml; iterate until beats baseline
5. **SW2** — desired-output-state.yml; iterate until beats baseline
6. **SW3** — agentic-categorization.yml; iterate until beats baseline
7. **SW4** — yaml-substructure-translation.yml; iterate until beats baseline
8. **SW5** — final-workflow-assembly.yml; iterate until beats baseline
9. **META** — meta-workflow-v6.yml orchestrator; end-to-end live test
10. **Logging hardening** — add per-step log fields, log analysis scripts
11. **Unit tests** — codify confirmed behaviors

Each SW must reach `golden/` status before advancing to the next.

---

## 8. File Naming Conventions

- Workflow YAMLs: `docs/benchmarks/workflows/<name>.yml`
- Run outputs: `docs/benchmarks/outputs/meta-workflow/<RUN_ID>/<sw_N>/<file>.md`
- Logs: `docs/benchmarks/outputs/meta-workflow/<RUN_ID>/logs/<sw_N>.log`
- Plan artifacts: `docs/plans/meta-workflow-generator/artifacts/<sw_N>/{baseline-opencode,iterations,golden}/`
- Unit tests: `tests/meta_workflow/<feature>_test.rs`

---

## 9. Why This Architecture Works for an SLM

1. **Scope narrowing**: Each LLM call only does ONE thing (chunk evaluation, single
   categorization, single YAML block). No global reasoning required.
2. **Iterative refinement**: Output is accumulated across chunks and re-evaluated;
   failures are addressed in isolation.
3. **Deterministic safety nets**: Shell hooks run Python scripts that catch syntax
   errors, schema violations, format issues — independent of LLM quality.
4. **Filesystem state**: Survives crashes; supports resumability.
5. **Independent testability**: Each SW can be run in isolation against any prompt,
   enabling fast iteration loops without restarting META.
