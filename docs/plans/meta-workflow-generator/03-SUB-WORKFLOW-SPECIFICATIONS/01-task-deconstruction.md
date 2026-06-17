# SW1 Specification — Task Deconstruction

| Attribute | Value |
|-----------|-------|
| Workflow ID | `sw1_task_deconstruction` |
| YAML file | `docs/benchmarks/workflows/sw1-task-deconstruction.yml` |
| Output | `docs/benchmarks/outputs/meta-workflow/<RUN_ID>/sw1/tasks.md` (single MD) |
| Reads from | `docs/benchmarks/outputs/meta-workflow/<RUN_ID>/input/prompt.txt` (raw user prompt) |
| Consumed by | SW2 (reads `tasks.md`) |
| Model | `qwen35` only |
| Iteration target | 3–5 chunk passes; up to 5 fix cycles per evaluation gate |

---

## 1. Purpose

Given a high-complexity agentic prompt, decompose it into a tree of atomic tasks such
that no leaf task exceeds complexity story-point threshold 5 (≈ 2–3 days for a mid-level
engineer). Every task must satisfy quality engineering principles and have GWT criteria
fully flushed out.

## 2. Story-Point Complexity Scale

| Points | Description | Examples |
|--------|-------------|----------|
| 1 | Trivial. < 4 hours. Single obvious action. | "Read file X", "echo hello" |
| 2 | Small. 4 hours – 1 day. One well-known transform. | "Parse CSV → JSON", "Call API X" |
| 3 | Medium. 1–2 days. Multi-step but linear. | "Filter list by predicate, sort, group" |
| 5 | Large. 2–3 days. Multi-step with branching. | "Validate input, transform, persist, validate output" |
| 8 | Too large. 3–5 days. Multi-component. | "Build ingestion + processing + reporting pipeline" |
| 13 | Epic. 1+ week. | "Refactor authentication across services" |

**Hard rule**: Any task scoring ≥ 5 MUST be expanded into 2+ subtasks. Loop until all
tasks are < 5 pts or genuinely atomic.

## 3. Output Schema (`tasks.md`)

```markdown
# Task Breakdown

**Source prompt:** (verbatim quote, max 200 chars + ellipsis)

**Total tasks:** N
**Total story points:** M
**Maximum leaf complexity:** X pts

---

## T1 — <descriptive-name> (X pts)

**Story points:** X
**Why:** <one sentence justifying the score>
**Action:** <one sentence describing what to do>
**Depends on:** T0 (none if first)
**GWT criteria:**
  - **Given:** <preconditions>
  - **When:** <action taken>
  - **Then:** <observable outcome>

### T1.1 — <subtask-name> (Y pts)
...

### T1.2 — <subtask-name> (Y pts)
...

---

## T2 — <descriptive-name> (X pts)
...

---

## Complexity Distribution

| Points | Count |
|--------|-------|
| 1 | n1 |
| 2 | n2 |
| 3 | n3 |
| 5 | n5 (must be 0 — all expanded) |
```

## 4. Pipeline Stages

| Step | Purpose | LLM/Shell | Hooks |
|------|---------|-----------|-------|
| `step_00_bootstrap` | mkdir run dirs, read raw prompt | shell | before_step_starts: shell (mkdir + cat prompt) |
| `step_01_initial_breakdown` | LLM: produce top-level task list with story points | LLM | after_step_succeeds: save_to tasks.md (initial) |
| `step_02_complexity_review` | LLM: for each task ≥ 5pts, expand into subtasks | LLM (iterate_values over tasks) | iterate_values: task_id list |
| `step_03_evaluate` | LLM: GWT criteria check; route to fix if any fail | LLM | after_step_succeeds: gwt route_to step_04 or step_99 |
| `step_04_fix` | LLM: address specific failures from step_03 | LLM | after_step_succeeds: route_to step_03 |
| `step_99_finalize` | Save final tasks.md + bookmark for SW2 | LLM (trivial) | after_step_succeeds: save_to var + file, bookmark |

## 5. Detailed Step Specifications

### step_00_bootstrap

```yaml
step_00_bootstrap:
  generative_entity: "${models.qwen35}"
  prompt: |
    Bootstrapping SW1 (task-deconstruction). Output "READY".
  model_overrides: { temperature: 0.0, max_tokens: 10 }
  when:
    before_step_starts:
      - shell:
          command: "mkdir -p ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/{sw1,logs} && cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/input/prompt.txt"
          args: []
          working_dir: "/home/jon/code/whitt-execution-engine"
          fail_on_error: true
      - bookmark:
          detailed:
            path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/00-bootstrap.txt"
    after_step_succeeds:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw1.log"
          event_fields: [step_name, duration_ms]
          level: info
```

### step_01_initial_breakdown

```yaml
step_01_initial_breakdown:
  generative_entity: "${models.qwen35}"
  depends_on: [step_00_bootstrap]
  prompt: |
    You are an engineering architect. Decompose the following prompt into a top-level
    task list. Each task gets a story-point estimate on the scale 1/2/3/5/8/13.

    PROMPT:
    {{bookmarks.shell_output.stdout}}

    Output ONLY this format, nothing else:

    T1 — <name> (<points>pts): <one-line action>
    T2 — <name> (<points>pts): <one-line action>
    ...

    Rules:
    - Tasks must be ordered (T1, T2, ...)
    - Points MUST come from {1, 2, 3, 5, 8, 13}
    - Action is ONE line, < 100 chars
    - Do NOT expand subtasks yet
    - Aim for 4-12 top-level tasks
  model_overrides: { temperature: 0.2, max_tokens: 1500 }
  when:
    after_step_succeeds:
      - save_to:
          - sw1_initial_breakdown
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/01-initial.txt"
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw1.log"
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
    after_step_fails:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw1.log"
          event_fields: [step_name, error_message]
          level: error
```

### step_02_expand_high_complexity

Iterates over each top-level task to expand those scoring ≥ 5.

```yaml
step_02_expand_high_complexity:
  generative_entity: "${models.qwen35}"
  depends_on: [step_01_initial_breakdown]
  prompt: |
    You are decomposing a high-complexity task into subtasks.

    TASK TO DECOMPOSE:
    {{loop.current_task}}

    PRIOR CONTEXT (other tasks for reference):
    {{step.step_01_initial_breakdown.output}}

    Story-point scale: 1/2/3/5/8/13.
    A task scoring >= 5 MUST be split into 2+ smaller subtasks each scoring < 5.
    A task scoring 1-3 needs NO subtask expansion.

    If the task scores 1-3, output ONLY: "NO_EXPANSION_NEEDED"
    Otherwise output:

    T<x>.1 — <subtask-name> (<pts>pts): <action>
    T<x>.2 — <subtask-name> (<pts>pts): <action>
    ...

    Rules:
    - Each subtask action < 100 chars
    - Subtasks must be smaller than parent
    - All subtask points must be < 5
  model_overrides: { temperature: 0.2, max_tokens: 1000 }
  when:
    before_step_starts:
      # Iterate over each T<id> — name (pts): action line from step_01 output.
      # These are populated by scripts/generate-workflow.sh OR a pre-step shell
      # that parses step_01 output and emits a JSON list.
      - shell:
          command: "python3 scripts/parse-task-list.py ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/01-initial.txt --format iterate_values_json"
          args: []
          working_dir: "/home/jon/code/whitt-execution-engine"
          fail_on_error: false
      - iterate_values:
          current_task: []  # populated at runtime by the shell output above
    after_step_succeeds:
      - append_to:
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/02-expanded.md"
          - sw1_expanded_accumulator
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw1.log"
          event_fields: [step_name, loop.iteration, duration_ms, total_tokens]
          level: info
    after_loop_iteration_fails:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw1.log"
          event_fields: [iteration, error_message]
          level: warning
```

⚠️ **NOTE on dynamic iterate_values**: The engine's `extract_iterate_values`
(`runner.rs:800-836`) reads `iterate_values` from the YAML statically — it does NOT
read runtime shell output to populate the list. Therefore, we have two options:

**Option A (RECOMMENDED for v6)**: Hardcode 12 task slots in `iterate_values`:
`task_id: ["T1", "T2", ..., "T12"]`. Step_02 prompt then asks LLM to expand ONLY the
task at `{{loop.task_id}}` by reading the full list from step_01 output. Steps for
non-existent tasks are skipped via GWT (if `step_01.output contains "T12"` etc.).

**Option B**: Pre-process step_01 output via shell + Python into a fixed-length
matrix the YAML can statically reference. More fragile.

v6 uses Option A.

### step_03_evaluate

```yaml
step_03_evaluate:
  generative_entity: "${models.qwen35}"
  depends_on: [step_02_expand_high_complexity]
  prompt: |
    You are a quality reviewer. Evaluate the task breakdown for engineering quality.

    FULL TASK BREAKDOWN:
    Initial tasks:
    {{step.step_01_initial_breakdown.output}}

    Subtask expansions:
    {{bookmarks.sw1_expanded_accumulator.stdout}}

    Evaluate against each criterion. For each, output PASS or FAIL with reason:
    1. ATOMIC: Every leaf task is a single, well-scoped action
    2. COMPLETE: Every part of the original prompt is covered by some task
    3. NON_REDUNDANT: No two tasks duplicate work
    4. GWT_FLUSHED: Every task has observable Given/When/Then
    5. NO_HIGH_COMPLEXITY_LEAVES: No leaf task scores >= 5 pts
    6. NOT_OVERLY_VERBOSE: Each task description < 100 chars

    End with: VERDICT: PASS  or  VERDICT: FAIL
  model_overrides: { temperature: 0.1, max_tokens: 1200 }
  when:
    after_step_succeeds:
      - save_to:
          - sw1_eval
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/03-eval.txt"
      - gwt:
          - given: "{{step.step_03_evaluate.output}} contains 'VERDICT: PASS'"
            then: { route_to: step_99_finalize }
          - given: "true"
            then: { route_to: step_04_fix }
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw1.log"
          event_fields: [step_name, duration_ms]
          level: info
```

### step_04_fix

```yaml
step_04_fix:
  generative_entity: "${models.qwen35}"
  depends_on: [step_03_evaluate]
  prompt: |
    You are fixing issues in a task breakdown.

    CURRENT BREAKDOWN:
    {{step.step_01_initial_breakdown.output}}
    {{bookmarks.sw1_expanded_accumulator.stdout}}

    EVALUATION FAILURES:
    {{step.step_03_evaluate.output}}

    Output a CORRECTED task breakdown in the same format. Address each FAIL.
    If a task needs to be split, split it. If a description is too long, shorten it.
    Output ONLY the corrected breakdown.
  model_overrides: { temperature: 0.2, max_tokens: 2000 }
  when:
    after_step_succeeds:
      - save_to:
          - sw1_initial_breakdown  # OVERWRITE predecessor
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/01-initial.txt"
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw1.log"
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
    # Then loop back to step_03_evaluate via GWT
    # (We rely on step_03 being re-executed via the route_to in step_04's after_step_succeeds gwt)
```

⚠️ Loop safety: Add a counter via bookmark. After 5 fix cycles, force advance.

### step_99_finalize

```yaml
step_99_finalize:
  generative_entity: "${models.qwen35}"
  depends_on: [step_03_evaluate]
  prompt: |
    The task breakdown passed all evaluation gates. Assemble the final markdown
    document in this exact format:

    # Task Breakdown
    **Source prompt:** (first 200 chars of prompt)
    **Total tasks:** N
    **Total story points:** M
    **Maximum leaf complexity:** X pts

    (then each task in T<id> — name (Xpts) format with subtasks indented)

    Output ONLY the final markdown.
  model_overrides: { temperature: 0.1, max_tokens: 2500 }
  when:
    after_step_succeeds:
      - save_to:
          - sw1_final
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/tasks.md"
      - bookmark: true
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw1.log"
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
    after_step_fails:
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw1.log"
          event_fields: [step_name, error_message]
          level: error
```

## 6. Quality Gates (Summary)

| Gate | Criterion | Failure Action |
|------|-----------|----------------|
| G1 | ATOMIC: leaf tasks are single actions | route_to step_04 |
| G2 | COMPLETE: prompt fully covered | route_to step_04 |
| G3 | NON_REDUNDANT: no duplicates | route_to step_04 |
| G4 | GWT_FLUSHED: all tasks have Given/When/Then | route_to step_04 |
| G5 | NO_HIGH_COMPLEXITY_LEAVES: no leaf ≥ 5pts | route_to step_04 |
| G6 | NOT_OVERLY_VERBOSE: descriptions < 100 chars | route_to step_04 |

## 7. Acceptance Criteria

- [ ] All 5+ test-dataset prompts produce a `tasks.md` matching the output schema
- [ ] Every leaf task in every output scores < 5 pts
- [ ] Every task has GWT criteria
- [ ] No task description exceeds 100 chars
- [ ] Total task count between 4 and 30 (sanity bounds)
- [ ] `tasks.md` is valid markdown (passes `python3 -c "import markdown; markdown.markdown(open('tasks.md').read())"`)
- [ ] Live run completes within 30 minutes per prompt on Qwen3.5-9B
- [ ] Output **surpasses** Sisyphus manual single-shot baseline on at least 3 of 5 prompts

## 8. Failure Modes

| Failure | Mitigation |
|---------|------------|
| LLM emits tasks with no story points | Prompt explicit; post-process via Python script |
| LLM marks everything as 1pt (lazy) | Add few-shot example in prompt showing realistic distribution |
| Step_02 iterate_values runs empty slots | GWT gate on each iteration: skip if `step_01.output contains "{{loop.task_id}}" == false` |
| Fix loop never converges | Hard cap 5 iterations; then advance with WARN log |
| tasks.md contains markdown fences | Strip via `sed 's/```[a-z]*//g'` in finalize shell hook |

## 9. Baseline Reference

`docs/plans/meta-workflow-generator/artifacts/sw1-task-deconstruction/baseline-opencode/`
— one baseline per test-dataset prompt, authored manually by Sisyphus using his own
reasoning (not LLM-assisted).

## 10. Iteration Log

See `../08-ITERATION-LOG.md` section "SW1 Iterations". Each iteration entry contains:
- iter-NNN timestamp
- input prompt path
- output `tasks.md` path
- log path
- evaluation result (PASS/FAIL per gate)
- comparison vs baseline
- next action
