# SW4 Specification — YAML Substructure Translation

| Attribute | Value |
|-----------|-------|
| Workflow ID | `sw4_yaml_substructure_translation` |
| YAML file | `docs/benchmarks/workflows/sw4-yaml-substructure-translation.yml` |
| Output | `docs/benchmarks/outputs/meta-workflow/<RUN_ID>/sw4/structs.md` (single MD; **YAML code blocks ONLY**) |
| Reads from | `sw1/tasks.md` + `sw2/outputs.md` + `sw3/categories.md` |
| Consumed by | SW5 (reads `structs.md`) |
| Model | `qwen35` only |
| Iteration target | Per-category-group; chunk size 2-3 categories |

---

## 1. Purpose

Translate categorized tasks into YAML substructure skeletons matching the whitt
unified schema. Output is a single `structs.md` document containing YAML code blocks
(no other code-block languages) interleaved with markdown prose explaining intent.

This is NOT the final workflow assembly (that's SW5). This produces a **structured
intent document** showing how each task maps to a YAML substructure.

## 2. Output Schema (`structs.md`)

```markdown
# YAML Substructure Translation

**Source:** sw1/tasks.md + sw2/outputs.md + sw3/categories.md
**Total substructures:** N
**YAML blocks:** N

---

## Substructure for T1 — <task-name>

**Task:** <verbatim action>
**Category:** C<n> — <name>
**Intent:** <1-2 sentences describing what this YAML block accomplishes>
**Fits in workflow as:** <position hint — e.g., "first step, no deps">

```yaml
step_01_<name>:
  generative_entity: "${models.qwen35}"
  prompt: |
    <role-and-task>
    Input: {{step.PREVIOUS.output}}
    Output ONLY the result.
  model_overrides:
    temperature: <0.05-0.3>
    max_tokens: <N>
  depends_on: []  # or [step_XX_prev]
  when:
    after_step_succeeds:
      - save_to: "./docs/benchmarks/outputs/output/step_01_<name>-output.txt"
      - log:
          to_file_path: "./docs/benchmarks/outputs/logs/workflow.log"
          event_fields: [step_name, duration_ms, total_tokens]
          level: info
    after_step_fails:
      - log:
          to_file_path: "./docs/benchmarks/outputs/logs/workflow.log"
          event_fields: [step_name, error_message]
          level: error
```

**Hooks explained:**
- `save_to`: persists output for downstream steps
- `log`: traceability for QA
- (no `before_step_starts` shell because this is an LLM step, not file-read)

---

## Substructure for T2 — <task-name>
...
```

## 3. Hard Rule: YAML Code Blocks Only

The output document may contain:
- Markdown prose (anywhere)
- ```` ```yaml ```` code blocks (for YAML substructures)
- Inline code with single backticks for keywords

The output document MUST NOT contain:
- ```` ```rust ````, ```` ```python ````, ```` ```bash ````, ```` ```sh ````, ```` ```json ```` etc.
- Reason: SW5 will extract content between ```` ```yaml ```` fences; any other fenced
  block pollutes the assembly.

The evaluation step (step_03_evaluate) enforces this via a Python check.

## 4. Category → YAML Mapping (Reference)

| Category | YAML substructure shape |
|----------|------------------------|
| C1 file_read | Step with `before_step_starts: shell: cat <file>`; LLM prompt consumes `{{bookmarks.shell_output.stdout}}` |
| C2 file_write | Step with `after_step_succeeds: save_to: <file>` |
| C3 transform_llm | Plain LLM step with `generative_entity`, `prompt`, `model_overrides` |
| C4 validate_gate | LLM step + `after_step_succeeds: gwt: [{given: ..., then: {route_to: ...}}]` |
| C5 loop_iterate | Step with `before_step_starts: iterate_values: {<var>: [...]}` |
| C6 branch_decision | LLM step that emits decision + GWT routing in `after_step_succeeds` |
| C7 shell_execute | Step with `before_step_starts: shell:` OR an LLM step consuming shell output |
| C8 aggregate_reduce | LLM step consuming multiple `{{step.X.output}}` references |
| C9 external_call | Shell step with `curl` or similar; output captured to bookmark |
| C10 prompt_user | LLM step with `user_input: {type: confirm, message: ...}` |

## 5. Pipeline Stages

| Step | Purpose |
|------|---------|
| `step_00_bootstrap` | mkdir sw4/, cat tasks.md + outputs.md + categories.md |
| `step_01_load_context` | LLM: confirm understanding of all 3 inputs |
| `step_02_translate_category_group` | LLM (iterate_values over category groups): emit YAML substructures |
| `step_03_evaluate` | GWT gate on YAML quality + format purity |
| `step_04_fix` | LLM: address failures |
| `step_05_merge` | Assemble structs.md |
| `step_99_finalize` | Save + bookmark |

## 6. Detailed Step Specifications

### step_02_translate_category_group

```yaml
step_02_translate_category_group:
  generative_entity: "${models.qwen35}"
  depends_on: [step_01_load_context]
  prompt: |
    You are a YAML substructure designer for the whitt execution engine schema.

    CATEGORY GROUP ({{loop.chunk_id}}):
    {{loop.current_category_group}}

    TASKS IN THIS CATEGORY (from tasks.md):
    {{bookmarks.sw1_tasks.stdout}}

    DESIRED STATES (from outputs.md):
    {{bookmarks.sw2_outputs.stdout}}

    CATEGORIZATION (from categories.md):
    {{bookmarks.sw3_categories.stdout}}

    SCHEMA EXCERPT (whitt unified-workflow-schema.yml):
    {{bookmarks.schema_excerpt.stdout}}

    For EACH task in this category group, output:

    ## Substructure for T<id> — <task-name>
    **Task:** <verbatim>
    **Category:** C<n> — <name>
    **Intent:** <what this YAML block accomplishes>
    **Fits in workflow as:** <position hint>

    ```yaml
    step_XX_<name>:
      generative_entity: "${models.qwen35}"
      prompt: |
        <role-and-task>
        <input reference>
        <output format spec>
        Output ONLY the result.
      model_overrides:
        temperature: <0.05-0.3>
        max_tokens: <N>
      depends_on: [<prior_step>]
      when:
        after_step_succeeds:
          - save_to: "./docs/benchmarks/outputs/output/step_XX_<name>-output.txt"
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/workflow.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info
        after_step_fails:
          - log:
              to_file_path: "./docs/benchmarks/outputs/logs/workflow.log"
              event_fields: [step_name, error_message]
              level: error
    ```

    **Hooks explained:** <bullet list of why each hook is present>

    HARD RULES:
    - Output ONLY ```yaml code blocks (no ```rust, ```python, ```bash, etc.)
    - Every step uses generative_entity: "${models.qwen35}"
    - Every step has after_step_succeeds AND after_step_fails
    - First step in workflow: depends_on: []
    - File paths use ./docs/benchmarks/outputs/output/ or ./docs/benchmarks/outputs/logs/

    Output ONLY the formatted substructure blocks.
  model_overrides: { temperature: 0.2, max_tokens: 2500 }
  when:
    before_step_starts:
      - iterate_values:
          chunk_id: ["cat_C1_C2", "cat_C3", "cat_C4_C6", "cat_C5_C7_C8_C9_C10"]
          current_category_group: [
            "C1 (file_read), C2 (file_write)",
            "C3 (transform_llm)",
            "C4 (validate_gate), C6 (branch_decision)",
            "C5 (loop_iterate), C7 (shell_execute), C8 (aggregate_reduce), C9 (external_call), C10 (prompt_user)"
          ]
      - shell:
          command: "echo '=== TASKS ==='; cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw1/tasks.md; echo '=== OUTPUTS ==='; cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw2/outputs.md; echo '=== CATEGORIES ==='; cat ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw3/categories.md; echo '=== SCHEMA ==='; head -200 docs/schema/unified-workflow-schema.yml"
          args: []
          working_dir: "/home/jon/code/whitt-execution-engine"
          fail_on_error: false
    after_step_succeeds:
      - append_to:
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw4/02-chunks.md"
          - sw4_chunks_accumulator
```

### step_03_evaluate

```yaml
step_03_evaluate:
  generative_entity: "${models.qwen35}"
  depends_on: [step_02_translate_category_group]
  prompt: |
    You are a YAML quality reviewer.

    SUBSTRUCTURES:
    {{bookmarks.sw4_chunks_accumulator.stdout}}

    Evaluate:
    1. YAML_ONLY_BLOCKS: All code blocks are ```yaml (no other languages)
    2. SCHEMA_VALID_KEYS: Every key in every YAML block exists in unified-workflow-schema.yml
    3. MODEL_REFERENCE: Every step uses ${models.qwen35}
    4. HOOKS_PRESENT: Every step has after_step_succeeds AND after_step_fails
    5. COMPLETE: Every task in tasks.md has a substructure block
    6. DEPENDENCIES_CORRECT: depends_on references valid prior step names
    7. PATHS_VALID: All file paths under ./docs/benchmarks/outputs/

    End with: VERDICT: PASS or VERDICT: FAIL
  model_overrides: { temperature: 0.1, max_tokens: 1500 }
  when:
    after_step_succeeds:
      - save_to:
          - sw4_eval
          - "./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw4/03-eval.txt"
      - gwt:
          - given: "{{step.step_03_evaluate.output}} contains 'VERDICT: PASS'"
            then: { route_to: step_05_merge }
          - given: "true"
            then: { route_to: step_04_fix }
```

**Additionally**: Run a Python check via shell hook in step_03 to verify no non-YAML
code blocks exist:

```yaml
      - shell:
          command: "python3 scripts/check-yaml-only-blocks.py ./docs/benchmarks/outputs/meta-workflow/__RUN_ID__/sw4/02-chunks.md"
          args: []
          working_dir: "/home/jon/code/whitt-execution-engine"
          fail_on_error: false
```

The script `scripts/check-yaml-only-blocks.py` (TO CREATE) parses the markdown file,
extracts all fenced code blocks, and exits non-zero if any non-yaml language is found.

## 7. Quality Gates

| Gate | Criterion |
|------|-----------|
| G1 | YAML_ONLY_BLOCKS: no non-YAML code blocks |
| G2 | SCHEMA_VALID_KEYS: all keys in schema |
| G3 | MODEL_REFERENCE: ${models.qwen35} only |
| G4 | HOOKS_PRESENT: every step has after_step_succeeds + after_step_fails |
| G5 | COMPLETE: every task has a substructure |
| G6 | DEPENDENCIES_CORRECT: depends_on references valid steps |
| G7 | PATHS_VALID: paths under ./docs/benchmarks/outputs/ |

## 8. Acceptance Criteria

- [ ] Every task has a YAML substructure in `structs.md`
- [ ] Zero non-YAML code blocks (verified by `scripts/check-yaml-only-blocks.py`)
- [ ] All 7 gates pass
- [ ] Each YAML substructure is individually valid YAML (parseable in isolation)
- [ ] `structs.md` is valid markdown
- [ ] Live run completes within 45 minutes
- [ ] **Surpasses** Sisyphus manual baseline on at least 3 of 5 prompts

## 9. Failure Modes

| Failure | Mitigation |
|---------|------------|
| LLM emits ```python or ```rust block | Python post-processor strips non-YAML blocks |
| LLM invents schema keys | Cross-check against `scripts/validate-yaml.py` per-substructure |
| Missing dependencies | Static analyzer checks depends_on references |
| Step names not unique | Python script checks for duplicates |
| YAML indentation broken | `scripts/fix-generated-yaml.py` post-processes |

## 10. Baseline Reference

`docs/plans/meta-workflow-generator/artifacts/sw4-yaml-substructure/baseline-opencode/`

## 11. Iteration Log

See `../08-ITERATION-LOG.md` section "SW4 Iterations".
