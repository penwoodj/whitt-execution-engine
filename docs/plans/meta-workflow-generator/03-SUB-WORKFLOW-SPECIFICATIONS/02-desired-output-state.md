# SW2 Specification — Desired Output State

| Attribute | Value |
|-----------|-------|
| Workflow ID | `sw2_desired_output_state` |
| YAML file | `docs/benchmarks/workflows/sw2-desired-output-state.yml` |
| Output | `docs/benchmarks/outputs/meta-workflow/<RUN_ID>/sw2/outputs.md` (single MD) |
| Reads from | `docs/benchmarks/outputs/meta-workflow/<RUN_ID>/sw1/tasks.md` (from SW1) |
| Consumed by | SW3 (reads `outputs.md`) |
| Model | `qwen35` only |
| Iteration target | Chunks of 3 tasks initially, scaling to 5 as SW proves stable |

---

## 1. Purpose

For every task in `tasks.md`, produce a well-defined **desired end-state** that
proves the task is complete. The desired state must be testable, unambiguous, and
tied to a specific task. This becomes the acceptance contract for SW4/SW5 to encode
into the generated YAML workflow.

## 2. Output Schema (`outputs.md`)

```markdown
# Desired Output States

**Source task breakdown:** sw1/tasks.md
**Total tasks covered:** N
**Chunk strategy:** start=3, scaled to=5

---

## T1 — <task-name>

**Task action:** <verbatim from tasks.md>
**Story points:** X

**Desired end-state:**
<1-3 sentences describing observable state when task is complete>

**Testable criteria:**
  - [ ] <criterion-1 — observable, binary PASS/FAIL>
  - [ ] <criterion-2>
  - [ ] <criterion-3>

**Given/When/Then:**
  - **Given:** <preconditions>
  - **When:** <action>
  - **Then:** <observable result>

**Anti-criteria (what NOT to produce):**
  - <common-mistake-1>
  - <common-mistake-2>

---

## T2 — <task-name>
...
```

## 3. Pipeline Stages

| Step | Purpose | LLM/Shell |
|------|---------|-----------|
| `step_00_bootstrap` | mkdir sw2/, cat tasks.md | shell |
| `step_01_load_tasks` | LLM: confirm understanding of task list | LLM |
| `step_02_chunk_describe` | LLM (iterate_values): for each chunk of tasks, produce desired state | LLM |
| `step_03_evaluate` | LLM: GWT check; route_to fix if any criterion fails | LLM |
| `step_04_fix` | LLM: address evaluation failures | LLM |
| `step_05_merge_chunks` | LLM/shell: assemble all chunks into single outputs.md | LLM |
| `step_99_finalize` | Save outputs.md + bookmark for SW3 | LLM (trivial) |

## 4. Chunk Strategy

Initial chunk size: **3 tasks per LLM call**. After 2 successful iterations at this
size, increase to **4**, then **5**. Cap at 5 to stay within Qwen3.5-9B effective
context budget under CPU slowdown.

The chunk size is hardcoded in the YAML's `iterate_values` block. Different YAMLs
may exist for different chunk sizes (`sw2-desired-output-state-c3.yml`,
`-c4.yml`, `-c5.yml`). v6 starts with c3.

## 5. Detailed Step Specifications

### step_00_bootstrap

```yaml
step_00_bootstrap:
  generative_entity: "${models.qwen35}"
  prompt: |
    Bootstrapping SW2 (desired-output-state). Output "READY".
  model_overrides: { temperature: 0.0, max_tokens: 10 }
  when:
    before_step_starts:
      - shell:
          command: "mkdir -p ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2 && cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/tasks.md"
          args: []
          working_dir: "/home/jon/code/whitt-execution-engine"
          fail_on_error: true
      - bookmark:
          detailed:
            path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/00-bootstrap.txt"
```

### step_02_chunk_describe (iterate_values over chunks)

```yaml
step_02_chunk_describe:
  generative_entity: "${models.qwen35}"
  depends_on: [step_01_load_tasks]
  prompt: |
    You are defining desired end-states for engineering tasks.

    TASKS (chunk {{loop.chunk_id}} of total):
    {{loop.current_chunk_tasks}}

    FULL CONTEXT (entire task breakdown for reference):
    {{bookmarks.shell_output.stdout}}

    For EACH task in this chunk, output in EXACTLY this format:

    ## T<id> — <task-name>
    **Task action:** <verbatim>
    **Story points:** <X>
    **Desired end-state:**
    <1-3 sentences>
    **Testable criteria:**
      - [ ] <criterion-1>
      - [ ] <criterion-2>
      - [ ] <criterion-3>
    **Given/When/Then:**
      - **Given:** <preconditions>
      - **When:** <action>
      - **Then:** <observable>
    **Anti-criteria:**
      - <mistake-1>

    Output ONLY the formatted task state blocks. No preamble.
  model_overrides: { temperature: 0.2, max_tokens: 2000 }
  when:
    before_step_starts:
      # Hardcode chunk list based on expected task count range.
      # For 4-12 task prompts, 4 chunks of 3 covers up to 12 tasks.
      - iterate_values:
          chunk_id: ["chunk_0", "chunk_1", "chunk_2", "chunk_3"]
          current_chunk_tasks: [
            "T1, T2, T3",
            "T4, T5, T6",
            "T7, T8, T9",
            "T10, T11, T12"
          ]
      - shell:
          command: "cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/tasks.md"
          args: []
          working_dir: "/home/jon/code/whitt-execution-engine"
          fail_on_error: false
    after_step_succeeds:
      - append_to:
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/02-chunks.md"
          - sw2_chunks_accumulator
      - log:
          to_file_path: "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/logs/sw2.log"
          event_fields: [step_name, loop.iteration, duration_ms]
          level: info
```

### step_03_evaluate

```yaml
step_03_evaluate:
  generative_entity: "${models.qwen35}"
  depends_on: [step_02_chunk_describe]
  prompt: |
    You are a quality reviewer. Evaluate the desired output states.

    TASKS:
    {{bookmarks.shell_output.stdout}}

    DESIRED STATES:
    {{bookmarks.sw2_chunks_accumulator.stdout}}

    For each criterion, output PASS or FAIL with reason:
    1. TESTABLE: Every criterion is binary PASS/FAIL observable
    2. UNAMBIGUOUS: No vague terms ("good", "fast", "complete")
    3. TIED_TO_TASK: Every task has a corresponding state block
    4. NOT_OVER_SCOPED: Desired state covers only its task, not neighbors
    5. GWT_PRESENT: Given/When/Then is filled for every task
    6. ANTI_CRITERIA_PRESENT: Anti-criteria list non-empty

    End with: VERDICT: PASS or VERDICT: FAIL
  model_overrides: { temperature: 0.1, max_tokens: 1500 }
  when:
    after_step_succeeds:
      - save_to:
          - sw2_eval
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/03-eval.txt"
      - gwt:
          - given: "{{step.step_03_evaluate.output}} contains 'VERDICT: PASS'"
            then: { route_to: step_05_merge_chunks }
          - given: "true"
            then: { route_to: step_04_fix }
```

### step_05_merge_chunks

```yaml
step_05_merge_chunks:
  generative_entity: "${models.qwen35}"
  depends_on: [step_03_evaluate]
  prompt: |
    Assemble the final outputs.md from the chunk accumulator.

    CHUNKS:
    {{bookmarks.sw2_chunks_accumulator.stdout}}

    Output a single coherent markdown document in this format:

    # Desired Output States
    **Source task breakdown:** sw1/tasks.md
    **Total tasks covered:** N
    **Chunk strategy:** start=3, scaled to=5

    ---

    (all task state blocks, in T<id> order)

    Output ONLY the final markdown.
  model_overrides: { temperature: 0.1, max_tokens: 2500 }
  when:
    after_step_succeeds:
      - save_to:
          - sw2_final
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/outputs.md"
      - bookmark: true
```

## 6. Quality Gates

| Gate | Criterion |
|------|-----------|
| G1 | TESTABLE: criteria are binary observable |
| G2 | UNAMBIGUOUS: no vague terms |
| G3 | TIED_TO_TASK: every task has a state block |
| G4 | NOT_OVER_SCOPED: covers only own task |
| G5 | GWT_PRESENT: Given/When/Then filled |
| G6 | ANTI_CRITERIA_PRESENT: anti-criteria non-empty |

## 7. Acceptance Criteria

- [ ] Every task in `tasks.md` has a corresponding state block in `outputs.md`
- [ ] All 6 quality gates pass for every chunk
- [ ] `outputs.md` is valid markdown
- [ ] Live run completes within 40 minutes per prompt
- [ ] Output **surpasses** Sisyphus manual baseline on at least 3 of 5 prompts

## 8. Failure Modes

| Failure | Mitigation |
|---------|------------|
| LLM forgets tasks in chunk | Cross-check task count between tasks.md and outputs.md via Python script |
| LLM emits vague criteria | Add few-shot example showing vague vs concrete criteria |
| Chunk boundary splits related tasks | Allow chunk size 4 or 5 to keep related tasks together |
| Fix loop diverges | Hard cap 5 cycles; advance with WARN |

## 9. Baseline Reference

`docs/plans/meta-workflow-generator/artifacts/sw2-desired-output-state/baseline-opencode/`

## 10. Iteration Log

See `../08-ITERATION-LOG.md` section "SW2 Iterations".
