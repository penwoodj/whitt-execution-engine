# Iteration 2 Plan — Closing Gaps Identified in Iteration 1

**Date:** 2026-06-14
**Reference:** `docs/plans/meta-v6/quality-reports/comparison-iter1.md`
**Goal:** Update SW1-SW5 YAML prompts to close gaps vs single-shot baseline

## Top Priority Fixes (P0)

### P0.1 — Fix SW5 GWT Routing (CRITICAL)
**Issue:** `given: "true"` always evaluates true → infinite fix loop
**Fix:** Update SW5 step_05_eval prompt:
- Replace literal-string conditions with bookmark reference
- Use `{{bookmarks.shell_output.stdout}}` template, NOT literal text
- Ensure GWT expression fully wrapped in single quotes for YAML safety

### P0.2 — Fix SW5 save_to Syntax (CRITICAL)
**Issue:** `[bookmark, "path"]` not valid YAML list-of-list
**Fix:** Update SW5 step_03_assemble prompt:
- Require save_to format:
  ```yaml
  - save_to:
      - bookmark_name
      - "./outputs/path.md"
  ```
- Explicit instruction: "use indented list under save_to, NOT inline array"

### P0.3 — SW1: Expand Task Coverage
**Issue:** Only 8 tasks vs baseline 12 parent / 27 leaf
**Fix:** Update SW1 step_01 and step_02 prompts:
- step_01: Require ≥10 parent tasks, each ≤5 pts, broken into ≥2 leaves if ≥3 pts
- step_02: Add explicit instruction to create analysis-phase subtasks (read, parse, compare) before action subtasks
- step_03 (evaluate): Add coverage check: "EVERY task ≥3 pts must have ≥2 leaf subtasks"

## Medium Priority Fixes (P1)

### P1.1 — SW3: Fix Category Misassignment
**Issue:** 5/15 tasks incorrectly categorized as PARALLEL_FAN_OUT/ACCUMULATOR/ITERATIVE_REFINER when they're simple DATA_TRANSFORMER
**Fix:** Update SW3 step_03 prompt:
- Add explicit constraint: "Default to DATA_TRANSFORMER. Only use other categories when task REQUIRES iteration, parallelism, or accumulation"
- Add decision criteria: "ITERATIVE_REFINER only when quality gate feedback needed"

### P1.2 — SW5: Add Log Hooks to Every Step
**Issue:** Only 3/36 log hooks
**Fix:** Update SW5 step_03_assemble prompt:
- Require every step to have:
  ```yaml
  after_step_succeeds:
    - log:
        to_file_path: "./logs/<step_name>.log"
        event_fields: [step_name, duration_ms, total_tokens]
        level: info
  ```

### P1.3 — SW5: Add Shell Steps for File Operations
**Issue:** LLM-only, no shell verification
**Fix:** Update SW5 step_03 prompt:
- Require shell steps for: file counting, grep verification, find operations
- LLM-only for content generation/modification

## Low Priority Fixes (P2)

### P2.1 — SW2: Specify Artifact Paths
**Issue:** Vague paths like "benchmark-100-model-userflows/mvp-summary-report.md"
**Fix:** Update SW2 step_03 prompt: "All artifact paths must start with `./outputs/` and use descriptive names"

### P2.2 — SW4: Fix depends_on Correctness
**Issue:** Linear chains prevent parallelism
**Fix:** Update SW4 step_02 prompt: "T5.1 and T5.2 (after quality gate) should both depend on T5_evaluate, not chain"

## Execution Plan

### Step 1: Apply P0 fixes to SW YAMLs (60 min)
- Edit SW1: step_01, step_02, step_03 prompts
- Edit SW5: step_03, step_05 prompts
- Document changes in git diff

### Step 2: Apply P1 fixes to SW YAMLs (30 min)
- Edit SW3: step_03 prompt
- Edit SW5: step_03 prompt (log hooks, shell steps)

### Step 3: Re-run SW1 with iter2 prompts (30 min)
- Input: prompt-01.md
- Output: tasks.md
- Verify: ≥10 parent tasks, ≥20 leaf tasks, every task ≥3 pts has leaves

### Step 4: Re-run SW2-SW5 with iter2 prompts (90 min)
- Chain: tasks.md → outputs.md → categories.md → structs.md → workflow.yml
- Verify: workflow.yml parses as valid YAML, all 27 leaf tasks present

### Step 5: Compare iter2 vs baseline (30 min)
- Re-run comparison report
- Target: SW1 PASS, SW5 PASS (most critical)
- Document in `comparison-iter2.md`

## Success Criteria

- [ ] SW5 workflow.yml parses as valid YAML (python yaml.safe_load)
- [ ] SW1 produces ≥20 leaf tasks (currently 5)
- [ ] SW5 produces ≥25 steps (currently 16)
- [ ] SW5 GWT routing uses bookmark reference, not literal text
- [ ] SW5 save_to uses correct indented list syntax
- [ ] SW5 has log hooks on every step
- [ ] SW3 defaults to DATA_TRANSFORMER, ≥80% of tasks

## Rollback Plan
If iter2 regresses (output quality drops), revert YAML changes via git, analyze why, try different prompt strategy.
