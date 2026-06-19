# SW3 Specification — Agentic Categorization

| Attribute | Value |
|-----------|-------|
| Workflow ID | `sw3_agentic_categorization` |
| YAML file | `docs/benchmarks/workflows/sw3-agentic-categorization.yml` |
| Output | `docs/benchmarks/outputs/meta-workflow/<RUN_ID>/sw3/categories.md` (single MD) |
| Reads from | `sw1/tasks.md` + `sw2/outputs.md` |
| Consumed by | SW4 (reads `categories.md`) |
| Model | `qwen35` only |
| Iteration target | Per-task categorization; chunk size 3 scaling to 5 |

---

## 1. Purpose

Bucket every task into a **correlative agentic behavior category**. We are NOT yet
translating to the whitt framework — only assigning semantic categories that
describe *what kind of agentic behavior* the task requires. SW4 then maps these
categories to YAML substructures.

## 2. Canonical Category Set (CLOSED)

| Category ID | Name | Description | Example Tasks |
|-------------|------|-------------|---------------|
| C1 | `file_read` | Read a file from disk, return contents | Read CSV, parse config, load schema |
| C2 | `file_write` | Write content to a file on disk | Save report, persist output |
| C3 | `transform_llm` | LLM transforms input → output (single-shot) | Summarize, classify, extract entities |
| C4 | `validate_gate` | Check if input meets criteria; PASS/FAIL decision | Quality check, schema validation |
| C5 | `loop_iterate` | Iterate over a list, processing each item | For each file in dir, for each row in CSV |
| C6 | `branch_decision` | Conditional routing based on prior output | If quality > 0.9 → success, else → fix |
| C7 | `shell_execute` | Execute a shell command | Run script, invoke tool, system call |
| C8 | `aggregate_reduce` | Combine multiple inputs into single output | Summarize across files, merge results |
| C9 | `external_call` | Call an external API/service | HTTP request, DB query |
| C10 | `prompt_user` | Ask user for input | Confirmation, parameter prompt |

**Rules**:
- Every task MUST be assigned exactly ONE primary category.
- A task may have a SECONDARY category if its behavior genuinely spans two (e.g.,
  `transform_llm` + `validate_gate` when LLM also judges PASS/FAIL).
- The category set is closed. If a task doesn't fit, choose the closest and explain.

## 3. Output Schema (`categories.md`)

```markdown
# Agentic Categorization

**Source:** sw1/tasks.md + sw2/outputs.md
**Total tasks:** N
**Category distribution:** C1=n1, C2=n2, ...

---

## T1 — <task-name>

**Task action:** <verbatim>
**Story points:** X
**Primary category:** C3 — transform_llm
**Secondary category:** C4 — validate_gate (if applicable)
**Why primary:** <1-2 sentences explaining why this category fits>
**Why secondary:** <if applicable, why this also fits>
**Agentic pattern:** <one-line description of expected execution pattern>
  e.g. "LLM ingests prior step output, transforms per prompt, emits result; GWT gate
  on output completeness."
**Sub-workflow affinity:** <which whitt pattern this maps to>
  e.g. "single LLM step with after_step_succeeds save_to"
**Iteration need:** none | chunk-of-N | fix-loop

---

## T2 — <task-name>
...
```

## 4. Pipeline Stages

| Step | Purpose |
|------|---------|
| `step_00_bootstrap` | mkdir sw3/, cat tasks.md + outputs.md |
| `step_01_load_context` | LLM: confirm understanding of tasks + states |
| `step_02_categorize_chunk` | LLM (iterate_values over chunks of 3): assign categories |
| `step_03_evaluate` | GWT gate on category quality |
| `step_04_fix` | LLM: address failures |
| `step_05_merge_chunks` | Assemble single categories.md |
| `step_99_finalize` | Save + bookmark |

## 5. Detailed Step Specifications

### step_02_categorize_chunk

```yaml
step_02_categorize_chunk:
  generative_entity: "${models.qwen35}"
  depends_on: [step_01_load_context]
  prompt: |
    You are an agentic workflow categorizer. Assign each task a category.

    TASKS (chunk {{loop.chunk_id}}):
    {{loop.current_chunk_tasks}}

    TASKS REFERENCE (full list):
    {{bookmarks.sw1_tasks.stdout}}

    DESIRED STATES REFERENCE:
    {{bookmarks.sw2_outputs.stdout}}

    CANONICAL CATEGORIES (closed set):
    C1 file_read       — read file, return contents
    C2 file_write      — write content to file
    C3 transform_llm   — LLM single-shot transform
    C4 validate_gate   — PASS/FAIL decision
    C5 loop_iterate    — iterate over list
    C6 branch_decision — conditional routing
    C7 shell_execute   — shell command
    C8 aggregate_reduce — combine inputs
    C9 external_call   — external API/service
    C10 prompt_user    — ask user

    For EACH task in this chunk output:

    ## T<id> — <task-name>
    **Task action:** <verbatim>
    **Story points:** <X>
    **Primary category:** C<n> — <name>
    **Secondary category:** C<n> — <name> (or "none")
    **Why primary:** <reason>
    **Why secondary:** <reason or "n/a">
    **Agentic pattern:** <one-line execution description>
    **Sub-workflow affinity:** <whitt pattern hint>
    **Iteration need:** none | chunk-of-N | fix-loop

    Rules:
    - Primary MUST be from C1-C10
    - Secondary optional, must also be from C1-C10
    - "Why" must reference task specifics, not generic

    Output ONLY the formatted task category blocks.
  model_overrides: { temperature: 0.2, max_tokens: 1800 }
  when:
    before_step_starts:
      - iterate_values:
          chunk_id: ["chunk_0", "chunk_1", "chunk_2", "chunk_3"]
          current_chunk_tasks: [
            "T1, T2, T3",
            "T4, T5, T6",
            "T7, T8, T9",
            "T10, T11, T12"
          ]
      - shell:
          command: "echo '=== TASKS ==='; cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/tasks.md; echo '=== OUTPUTS ==='; cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/outputs.md"
          args: []
          working_dir: "/home/jon/code/whitt-execution-engine"
          fail_on_error: false
    after_step_succeeds:
      - append_to:
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3/02-chunks.md"
          - sw3_chunks_accumulator
```

### step_03_evaluate

```yaml
step_03_evaluate:
  generative_entity: "${models.qwen35}"
  depends_on: [step_02_categorize_chunk]
  prompt: |
    You are a quality reviewer for task categorization.

    CATEGORIZED TASKS:
    {{bookmarks.sw3_chunks_accumulator.stdout}}

    Evaluate:
    1. CLOSED_SET: All categories from C1-C10 (no invented categories)
    2. EXCLUSIVE: Each task has exactly one PRIMARY category
    3. JUSTIFIED: "Why" references task specifics (not generic boilerplate)
    4. COMPLETE: Every task in tasks.md has a category block here
    5. PATTERN_SENSIBLE: Agentic pattern aligns with category (e.g., C5 → iteration)
    6. AFFINITY_PRESENT: Sub-workflow affinity field is non-empty

    End with: VERDICT: PASS or VERDICT: FAIL
  model_overrides: { temperature: 0.1, max_tokens: 1200 }
  when:
    after_step_succeeds:
      - save_to:
          - sw3_eval
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3/03-eval.txt"
      - gwt:
          - given: "{{step.step_03_evaluate.output}} contains 'VERDICT: PASS'"
            then: { route_to: step_05_merge_chunks }
          - given: "true"
            then: { route_to: step_04_fix }
```

## 6. Quality Gates

| Gate | Criterion |
|------|-----------|
| G1 | CLOSED_SET: categories from C1-C10 only |
| G2 | EXCLUSIVE: exactly one primary per task |
| G3 | JUSTIFIED: "why" references task specifics |
| G4 | COMPLETE: every task categorized |
| G5 | PATTERN_SENSIBLE: pattern aligns with category |
| G6 | AFFINITY_PRESENT: sub-workflow affinity non-empty |

## 7. Acceptance Criteria

- [ ] Every task in `tasks.md` appears in `categories.md`
- [ ] All categories from closed set C1-C10
- [ ] All 6 gates pass
- [ ] `categories.md` is valid markdown
- [ ] Live run completes within 35 minutes
- [ ] **Surpasses** Sisyphus manual baseline on at least 3 of 5 prompts

## 8. Failure Modes

| Failure | Mitigation |
|---------|------------|
| LLM invents categories outside set | Add explicit "CLOSED SET" warning + few-shot |
| Same category assigned to all tasks (lazy) | Add distribution diversity hint in prompt |
| "Why" is generic boilerplate | Few-shot example showing vague vs specific justification |
| Task missing from output | Cross-check via Python script |

## 9. Baseline Reference

`docs/plans/meta-workflow-generator/artifacts/sw3-agentic-categorization/baseline-opencode/`

## 10. Iteration Log

See `../08-ITERATION-LOG.md` section "SW3 Iterations".
