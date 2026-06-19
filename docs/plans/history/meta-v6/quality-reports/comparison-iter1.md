# META-v6 vs Baseline Comparison — Iteration 1

**Generated:** 2026-06-14
**Scope:** Compare META-v6 iterative outputs against single-shot baselines for SW1-SW5
**Goal:** Identify quality gaps and recommend specific YAML improvements

---

## Summary

| SW | Status | Size Ratio | Coverage | Top Gap |
|----|--------|------------|----------|---------|
| SW1 (Tasks) | ❌ NEEDS_ITERATION | 48% (8763/18392) | 67% (8/12) | Missing analysis phase |
| SW2 (Outputs) | ❌ NEEDS_ITERATION | 51% (12635/27402) | 42% (8/19) | Artifact specifications |
| SW3 (Categories) | ❌ NEEDS_ITERATION | 38% (9551/423) | 56% (15/27) | Category misassignment |
| SW4 (Structs) | ❌ NEEDS_ITERATION | 36% (12565/44410) | 56% (15/27) | Incomplete YAML |
| SW5 (Workflow) | ❌ FAIL | 35% (8921/21068) | 44% (16/36) | Invalid GWT routing |

**Overall:** All 5 SWs require iteration. META-v6 produces smaller, less detailed outputs with missing task coverage, incorrect categorization, and structural YAML issues.

---

## SW1 Analysis — Task Breakdown

### Metrics

| Metric | META-v6 | Baseline | Gap |
|--------|---------|----------|-----|
| Total tasks | 8 | 12 | -33% |
| Total leaves | 5 | 27 | -81% |
| Total story points | 27 | 27 | Equal |
| Max leaf complexity | 3 pts | 3 pts | None |
| Avg words per task | ~1095 | ~1532 | -30% |
| GWT completeness | 8/8 (100%) | 27/27 (100%) | Equal |

### Coverage Gaps

**Missing tasks:**
- T1.1-T1.3: Read/parse overlap analysis steps (3 tasks → collapsed into T1)
- T3.1-T3.4: Scattered testing guidance categorization (4 tasks → collapsed into T2)
- T9.1-T9.2: Formatting pattern extraction + file counting (2 tasks → collapsed into T5)
- T10.1-T10.2: Prioritization + phased scheduling (2 tasks → missing)
- T11: Cross-directory overlap matrix (1 task → missing)
- T12: Automation recommendations (3 tasks → missing)

**Impact:** META-v6 skips analysis phases, jumps directly to execution without gap analysis, prioritization, or automation recommendations.

### Detail Depth Gaps

**Example T1 consolidation:**

META-v6:
```markdown
**Action:** Merge `requirements/index.md` into `mvp-summary-report.md` and delete the source file.
```

Baseline:
```markdown
**Action:** Read requirements/index.md and benchmark-100-model-userflows/mvp-summary-report.md, compare content sections, document specific overlap areas (quotes, line ranges)
```

**Gap:** META-v6 doesn't specify overlap detection method, line range extraction, or verification approach.

### GWT Completeness Gaps

**T1 GWT:**
- META-v6: "single source of truth exists, all references updated" (vague)
- Baseline: "Produce overlap analysis showing exact duplicate text ranges and unique content per file" (specific)

**T5.2 GWT:**
- META-v6: "Every markdown includes a consistent TOC section at the top" (how?)
- Baseline: "Parse each file to verify presence of TOC block; assert header count matches TOC entry count" (method)

### Atomicity

✅ **PASS:** All leaf tasks ≤ 3 story points. Both outputs meet requirement.

### Actionability Gaps

**META-v6 vague actions:**
- T1: "Merge... and delete" — no grep command for reference updates
- T2: "Refine... guidance" — no specific content filters
- T5: "Apply consistent emoji headings" — no regex patterns

**Baseline concrete actions:**
- T1.3: "Compare parsed outputs... create overlap analysis showing exact duplicate text ranges"
- T2.2: "Edit to remove workflow patterns and focus strictly on how to write tests"
- T5.1: "Process remaining 260 files adding consistent headers, wiki-links, and cross-directory references"

### Recommendations

1. **SW1 step_01 prompt:** Require T1 include 3 subtasks: (1) parse index.md, (2) parse mvp-summary-report.md, (3) generate overlap-analysis.md with line ranges before consolidation.

2. **SW1 step_03 prompt:** Add requirement for grep command: "include shell step to find all files referencing requirements/index.md and update to new path".

3. **SW1 step_02 prompt:** Specify content filters: "remove sections about automation, architectural context, workflow patterns; keep only syntax and execution instructions".

4. **SW1 step_05 prompt:** Require regex patterns in output: "specify emoji pattern (🔗📋🎯🛠️⚙️🧪), wiki-link regex, TOC anchor format".

5. **SW1 step_10/11/12 prompts:** Add new tasks for prioritization (T10), overlap matrix (T11), and automation recommendations (T12).

---

## SW2 Analysis — Output State Definitions

### Metrics

| Metric | META-v6 | Baseline | Gap |
|--------|---------|----------|-----|
| Tasks covered | 8 | 19 | -58% |
| Leaves covered | 5 | 27 | -81% |
| Acceptance criteria avg | 3 per task | 2 per task | +50% (more verbose, less precise) |
| Artifacts specified | 8/8 (100%) | 27/27 (100%) | Equal |
| Evaluation methods | 8/8 (100%) | 27/27 (100%) | Equal |

### Coverage Gaps

**Missing output states:**
- T1.1-T1.3: Parsed JSON structures and overlap-analysis.md
- T2.2-T2.3: File deletion verification, reference update verification
- T3.1-T3.4: Testing content categorization matrix
- T5.1-T5.2: Schema reference catalog, gap analysis
- T10.1-T10.2: Priority-sorted table, phased roadmap
- T11.1-T11.2: Topic-mapping CSV, overlap matrix
- T12.1-T12.3: Automation recommendation sections

**Impact:** Output states cover consolidation actions but skip all analysis artifacts needed for informed decisions.

### Detail Depth Gaps

**Example T1 acceptance criteria:**

META-v6 (3 criteria, vague):
```markdown
- File `requirements/index.md` no longer exists in the filesystem.
- File `mvp-summary-report.md` contains the merged executive summary section from the original index.
- All markdown files previously linking to `requirements/index.md` now link to `mvp-summary-report.md`.
```

Baseline (3 criteria, specific):
```markdown
- `mvp-summary-report.md` has new "## Executive Summary" section
- `requirements/index.md` either deleted OR contains only directory listing
- `grep` returns 0 results for old reference path
```

**Gap:** META-v6 misses verification method (grep command) and alternative scenarios (repurpose option).

### Evaluation Method Gaps

**META-v6 vague methods:**
- T1: "Run `find . -name 'index.md' -path '*requirements*' | wc -l` (must return 0)" — doesn't verify content quality
- T5: "Script to count files with emoji headings and wiki-links; assert count equals total pending file count (260)" — no script specified

**Baseline specific methods:**
- T1.1: "Unit test validates JSON structure has required fields" — clear verification
- T5.2: "Parse each file to verify presence of TOC block; assert header count matches TOC entry count" — parsing logic specified

### Artifact Specification Gaps

**META-v6 artifacts:**
- "Updated `benchmark-100-model-userflows/mvp-summary-report.md`" — path ambiguous

**Baseline artifacts:**
- "`./outputs/t1_1_parsed.json`" — exact path specified
- "`./outputs/overlap-analysis.md`" — clear artifact name

### Recommendations

1. **SW2 step_01 prompt:** Require artifact paths use `./outputs/` prefix with descriptive filenames like `overlap-analysis.md`.

2. **SW2 step_05 prompt:** Add evaluation method: "Run grep command to find all schema references; count must match grep results (≥10 files)".

3. **SW2 step_03 prompt:** Specify output state as markdown matrix with 3×3 dimensions: "matrix with rows for guides/, workflows/, plans/ and columns for audience, scope, unique_content".

4. **SW2 step_01 eval method:** Add content verification: "diff merged file against original to confirm executive summary added, duplicate sections removed".

5. **SW2 step_10 prompt:** Require priority table with columns: Task ID, Impact, Effort, Priority Score, Priority Category; sorted by score descending.

---

## SW3 Analysis — Agentic Categorization

### Metrics

| Metric | META-v6 | Baseline | Gap |
|--------|---------|----------|-----|
| Tasks categorized | 15 | 27 | -44% |
| Leaves categorized | 5 | 27 | -81% |
| Categories used | 4 (SEQ, PAR, ITER, DATA, ACCUM) | 1 (DATA) | Over-categorization |
| Avg iteration budget | 2.4 | 1.0 | +140% (higher cost) |
| Substructure hints | 15/15 (100%) | 27/27 (100%) | Equal |

### Coverage Gaps

**Missing categories:**
- T1.1-T1.3: 3 tasks not categorized
- T3.1-T3.4: 4 tasks not categorized
- T9.1-T9.2: 2 tasks not categorized
- T10.1-T10.2: 2 tasks not categorized
- T11.1-T11.2: 2 tasks not categorized
- T12.1-T12.3: 3 tasks not categorized

**Impact:** Categorization incomplete; only parent tasks categorized, not leaves.

### Category Misassignment Gaps

**T2 (PARALLEL_FAN_OUT) - INCORRECT:**

META-v6 reasoning: "Separating mixed content into three distinct logical domains... requiring parallel processing"

Baseline: DATA_TRANSFORMER (correct). These are sequential file operations, not parallelizable:
- T2.1: merge strategy into ARCHITECTURE.md
- T2.2: filter testing-guide.md

**T3 (ACCUMULATOR) - INCORRECT:**

META-v6 reasoning: "Aggregating scattered schema-related documentation... into a new central index"

Baseline: DATA_TRANSFORMER (correct). Single file creation from existing schema docs, not accumulation.

**T5 (ITERATIVE_REFINER) - QUESTIONABLE:**

META-v6: ITERATIVE_REFINER with budget=3
Baseline: DATA_TRANSFORMER with budget=1

Real behavior: Batch processing of 260 files can be parallelized, not iterated. ITERATIVE_REFINER reserved for quality gates needing feedback loops.

**T6 (SEQUENTIAL_PROCESSOR) - QUESTIONABLE:**

META-v6: SEQUENTIAL_PROCESSOR with budget=3
Baseline: DATA_TRANSFORMER with budget=1

Real behavior: Add links → verify cycles. Two-step chain doesn't merit SEQUENTIAL_PROCESSOR category.

**Correct categorizations:**
- T1: DATA_TRANSFORMER (both) ✅
- T4: DATA_TRANSFORMER (both) ✅
- T7: DATA_TRANSFORMER (both) ✅

### Iteration Budget Gaps

**META-v6 higher costs:**
- T2: budget=3 (PARALLEL_FAN_OUT) → should be 1 (DATA_TRANSFORMER)
- T5: budget=3 (ITERATIVE_REFINER) → should be 1 (DATA_TRANSFORMER)
- T6: budget=3 (SEQUENTIAL_PROCESSOR) → should be 1 (DATA_TRANSFORMER)
- T7: budget=3 (ITERATIVE_REFINER) → should be 1 (DATA_TRANSFORMER)

**Baseline: All tasks budget=1** (correct for simple transformations)

### Substructure Hint Gaps

**META-v6 accurate hints:**
- T1: "single LLM step with save_to + log hooks" ✅
- T2: "parallel branches with different prompts, merged via append_to" ❌ (not parallel)
- T5: "loop with GWT quality gate, max 3 iterations" ❌ (no feedback loop needed)

**Baseline precise hints:**
- T1.1: "Single LLM step with save_to hook storing JSON in memory variable" ✅
- T2.1: "Sequential chain: step_2_1 → step_2_2 → step_2_3, each with save_to + log hooks" ✅

### Recommendations

1. **SW3 step_02 prompt:** Correct T2 to DATA_TRANSFORMER. Remove parallel branching; specify sequential file operations.

2. **SW3 step_03 prompt:** Correct T3 to DATA_TRANSFORMER. Single file creation (NAVIGATION.md) is transformation, not accumulation.

3. **SW3 step_05 prompt:** Change T5 from ITERATIVE_REFINER to DATA_TRANSFORMER. Batch processing of 260 files is parallelizable, not iterative.

4. **SW3 step_06 prompt:** Change T6 from SEQUENTIAL_PROCESSOR to DATA_TRANSFORMER. Two-step chain doesn't merit special category.

5. **SW3 step_05/06/07 prompts:** Set iteration budgets to 1 (simple transformations), not 2-3.

6. **SW3 step_05 prompt:** Change substructure hint from "loop with GWT quality gate" to "sequential steps: apply emojis → add wiki-links → insert TOC".

---

## SW4 Analysis — YAML Substructures

### Metrics

| Metric | META-v6 | Baseline | Gap |
|--------|---------|----------|-----|
| Total substructures | 15 | 27 | -44% |
| Categories used | 4 | 1 | Over-categorization |
| Hooks specified | 15/15 (100%) | 27/27 (100%) | Equal |
| YAML completeness | 15/15 (100%) | 27/27 (100%) | Equal |
| Depends_on correctness | 8/15 (53%) | 27/27 (100%) | -47% |

### Coverage Gaps

**Missing substructures:**
- T1.1-T1.3: 3 parsing steps
- T3.1-T3.4: 4 categorization steps
- T9.1-T9.2: 2 formatting analysis steps
- T10.1-T10.2: 2 prioritization steps
- T11.1-T11.2: 2 matrix creation steps
- T12.1-T12.3: 3 recommendation steps

**Impact:** Substructures skip all intermediate analysis steps, only show execution.

### YAML Structure Gaps

**T2 PARALLEL_FAN_OUT substructure:**

META-v6 (line 55-76):
```yaml
step_t2_branch_arch:
  prompt: "Extract high-level testing strategy..."
  when:
    after_step_succeeds:
      - save_to: [arch_content, "plans/ARCHITECTURE.md"]

step_t2_branch_dev:
  prompt: "Filter the testing guide..."
  when:
    after_step_succeeds:
      - save_to: [dev_content, "guides/testing-guide.md"]

step_t2_merge:
  depends_on: [step_t2_branch_arch, step_t2_branch_dev]
  prompt: "Merge the refined architecture strategy..."
```

**Problems:**
1. Steps are sequential (arch → dev → merge), not parallel
2. No `depends_on: []` on arch/dev to enable parallel execution
3. Merge step would overwrite files from previous steps
4. No shell steps for file deletion (testing-strategy.md cleanup)

**Baseline correct T4 DATA_TRANSFORMER:**
```yaml
step_t4_1_consolidate:
  depends_on: [step_t3_4_categorize]
  prompt: "Using categorization from {{bookmarks.t3_categorization}}, merge unique strategy content..."
  when:
    after_step_succeeds:
      - save_to: [t4_1_architecture, "./outputs/t4_1_architecture.md"]
      - shell:
          command: "rm plans/testing-strategy.md"
          fail_on_error: true
```

### Depends_on Correctness Gaps

**T5 loop dependency:**

META-v6 (line 167-196):
```yaml
step_t5_generate:
  prompt: "Apply Foam syntax to the current batch..."
  when:
    after_step_succeeds:
      - save_to: [formatted_batch, "/home/jon/code/whitt-execution-engine/docs/batch_output.md"]

step_t5_evaluate:
  depends_on: [step_t5_generate]
  prompt: "Verify that all files in the batch have emoji headings..."

step_t5_generate_fix:
  depends_on: [step_t5_evaluate]
  prompt: "Fix formatting deviations in the pending files..."

step_t5_1_apply_visuals:
  depends_on: [step_t5_generate_fix]  # WRONG: should be parallel to T5.2
```

**Problems:**
1. Linear chain (generate → evaluate → fix → apply_visuals) doesn't enable parallelism
2. T5.1 and T5.2 should both depend on T5_evaluate (parallel after quality gate)
3. No loop mechanism to iterate over files
4. No `route_to` to restart loop on FAIL

**Baseline correct T9:**
```yaml
step_t9_1_document_patterns:
  prompt: "Extract and document 5 formatting patterns..."
  # No deps - can run in parallel with T9.2

step_t9_2_count_unformatted:
  depends_on: [step_t9_1_document_patterns]
  shell:
    command: |
      formatted=$(find docs/ -name '*.md' -exec grep -l '🔗\|📋\|🎯' {} + | wc -l)
      total=$(find docs/ -name '*.md' | wc -l)
      unformatted=$((total - formatted))
      echo "Total: $total, Formatted: $formatted, Unformatted: $unformatted" > "./outputs/unformatted-count.md"
```

### Hook Specification Gaps

**META-v6 save_to syntax:**
```yaml
- save_to: [merged_report, "benchmark-100-model-userflows/mvp-summary-report.md"]
```

**Problem:** Two-element array but first element is bookmark name, second is path. Should be `[bookmark_name, path]` with both as strings.

**Baseline correct:**
```yaml
- save_to:
    - t1_1_parsed  # bookmark name
    - "./outputs/t1_1_parsed.json"  # file path
```

**Missing log hooks:**
- META-v6 has no log hooks on most steps
- Baseline has log hooks on every step with `event_fields: [step_name, duration_ms]`

### Recommendations

1. **SW4 step_02 prompt:** Rewrite T2 substructure as DATA_TRANSFORMER with sequential chain: (1) merge strategy → (2) delete testing-strategy.md → (3) filter testing-guide.md. Remove parallel branching.

2. **SW4 step_05 prompt:** Fix T5 dependency graph. Make T5.1 (apply_visuals) and T5.2 (add_toc) both depend on T5_evaluate, then both feed into T7. Remove broken loop mechanism.

3. **SW4 step_02 prompt:** Add shell hook to delete `plans/testing-strategy.md` after merge, with `fail_on_error: true`.

4. **SW4 all prompts:** Add log hooks to every step:
   ```yaml
   - log:
       to_file_path: "./logs/{step_name}.log"
       event_fields: [step_name, duration_ms]
       level: info
   ```

5. **SW4 step_03 prompt:** Change T3 from ACCUMULATOR to DATA_TRANSFORMER. Remove iterate_values; use single step to create NAVIGATION.md.

6. **SW4 step_05 prompt:** Add shell step for file counting:
   ```yaml
   shell:
     command: "find docs/ -name '*.md' | wc -l"
   ```

---

## SW5 Analysis — Generated Workflow YAML

### Schema Correctness Gaps

**✅ Valid schema fields:**
- `workflow_id`, `name`, `description`, `version` ✅
- `schema_version: "2.0.0"` ✅
- `providers: llama_cpp_with_vulkan` ✅
- `models: qwen35` ✅
- `workflow_execution_strategy.memory.model_lifecycle.unload_unused: false` ✅

**❌ Invalid schema fields:**
- `author: "Whitt Execution Engine"` (baseline only, META-v6 missing)
- `tags: [...]` (baseline only, META-v6 missing)

**Provider configuration:**
```yaml
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
```
✅ Correct per schema line 29-31

### Model Configuration Gaps

**Both outputs use same model:**
```yaml
models:
  "qwen35":
    name: "Qwen3-5-9B-Q4_K_M.gguf"
    host:
      type: llama_cpp_with_vulkan
```
✅ Correct per schema line 71

### Structural Gaps

**Missing steps:**
- T1.1-T1.3: 3 parsing steps (baseline has them, META-v6 missing)
- T3.1-T3.4: 4 categorization steps (baseline has them, META-v6 missing)
- T9.1-T9.2: 2 formatting analysis steps (baseline has them, META-v6 missing)
- T10.1-T10.2: 2 prioritization steps (baseline has them, META-v6 missing)
- T11.1-T11.2: 2 matrix creation steps (baseline has them, META-v6 missing)
- T12.1-T12.3: 3 recommendation steps (baseline has them, META-v6 missing)

**Step count:**
- META-v6: 16 steps
- Baseline: 36 steps
- Gap: -56% coverage

### Hook Specification Gaps

**❌ Invalid GWT routing (line 135-138):**
```yaml
- gwt:
    - given: '"# Agentic Categorization ... Evaluation verdict: PASS"' == "PASS"
      then: step_t5_finalize
    - given: "true"
      then: step_t5_generate_fix
```

**Problems:**
1. `given: "true"` always evaluates to true → always routes to `step_t5_generate_fix`
2. `then: step_t5_finalize` unreachable (dead code)
3. GWT condition compares string literal to "PASS" — won't match actual evaluation output
4. No step named `step_t5_finalize` exists in workflow

**Impact:** Quality gate broken; loop always routes to "fix" step, never completes.

**Correct GWT pattern (baseline):**
```yaml
- gwt:
    - given: "get_field('eval_result', 'verdict') == 'PASS'"
      then: step_t5_finalize
    - given: "true"
      then: step_t5_generate_fix
```

**❌ Invalid save_to hook (line 41, 57):**
```yaml
- save_to: [merged_report, "benchmark-100-model-userflows/mvp-summary-report.md"]
```

**Problems:**
1. Two-element array but schema expects either `[bookmark_name]` or `[[bookmark_name, path]]` (nested)
2. Second element is file path, not bookmark name
3. No bookmark name provided for subsequent steps to reference

**Correct pattern (baseline):**
```yaml
- save_to:
    - t2_1_merged  # bookmark name
    - "./outputs/t2_1_merged.md"  # file path
```

**❌ Missing log hooks:**
- META-v6: Only 3 log hooks (after_workflow, step_t1_cleanup, step_t5_evaluate)
- Baseline: 36 log hooks (every step has one)
- Gap: -92% coverage

### Depends_on Correctness Gaps

**❌ T5 loop dependency chain (line 125-168):**
```yaml
step_t5_generate:
  prompt: "Apply Foam syntax to the current batch..."

step_t5_evaluate:
  depends_on: [step_t5_generate]
  prompt: "Verify that all files in the batch have emoji headings..."

step_t5_generate_fix:
  depends_on: [step_t5_evaluate]
  prompt: "Fix formatting deviations in the pending files..."

step_t5_1_apply_visuals:
  depends_on: [step_t5_generate_fix]  # WRONG: should be parallel to T5.2

step_t5_2_add_toc:
  depends_on: [step_t5_1_apply_visuals]  # WRONG: linear chain prevents parallelism
```

**Problems:**
1. Linear chain doesn't enable parallel processing of 260 files
2. T5.1 and T5.2 should both depend on T5_evaluate (parallel after quality gate)
3. No loop construct to iterate over file batches
4. No shell steps for actual file operations (grep, find, sed)

**Correct pattern (baseline T9):**
```yaml
step_t9_1_document_patterns:
  prompt: "Extract and document 5 formatting patterns..."  # No deps

step_t9_2_count_unformatted:
  depends_on: [step_t9_1_document_patterns]
  shell:
    command: "find docs/ -name '*.md' | wc -l"
```

### Actionability Gaps

**META-v6 vague prompts:**
- T1: "Extract schema coverage data from index.md and inject it into mvp-summary-report.md" — no extraction method
- T2: "Extract high-level testing strategy from scattered sources" — no source file paths
- T5: "Apply Foam syntax to the current batch of pending markdown files" — no file discovery mechanism

**Baseline concrete prompts:**
- T1.1: "Read requirements/index.md and parse its structure. Extract all section headers, summarize content scope in <200 chars..." — file path + output format
- T3.1: "Read guides/testing-guide.md and extract: all section headers, summary (<150 chars), intended audience..." — constraints specified
- T5.1: "Identify pending markdown files missing emoji headings or wiki-style links..." — file discovery logic

### Verification Method Gaps

**META-v5 missing verification:**
- No shell steps to verify file operations
- No grep commands to check reference updates
- No find commands to count formatted files
- No diff commands to verify merge quality

**Baseline verification:**
```yaml
shell:
  command: "grep -r 'requirements/index.md' docs/"  # verify references updated
shell:
  command: "find docs/ -name '*.md' -exec grep -l '🔗\|📋\|🎯' {} + | wc -l"  # count formatted files
```

### Recommendations

1. **SW5 step_02/03/04 prompts:** Add T1.1-T1.3 parsing steps, T3.1-T3.4 categorization steps, T4.1-T4.2 cross-reference steps.

2. **SW5 step_05 prompt:** Fix GWT routing. Change `given: "true"` to `given: "get_field('eval_result', 'verdict') != 'PASS'"` and rename `step_t5_finalize` to `step_t5_next_batch`.

3. **SW5 step_01/02/03 prompts:** Fix save_to hooks. Change from `[bookmark, path]` to nested array `[[bookmark, path]]` or use separate `save_to` (bookmark) and `log` (path) actions.

4. **SW5 all prompts:** Add log hooks to every step with event_fields [step_name, duration_ms, output_tokens].

5. **SW5 step_05 prompt:** Add shell steps for file operations:
   - `find docs/ -name '*.md'` to discover files
   - `grep -E '🔗|📋|🎯'` to check formatting
   - `sed -i 's/old/new/g'` to update references

6. **SW5 step_05 prompt:** Fix T5 dependency graph. Make T5.1 (apply_visuals) and T5.2 (add_toc) both depend on T5_evaluate, enabling parallel execution.

7. **SW5 step_01 prompt:** Specify file paths in prompts: "Read requirements/index.md at /home/jon/code/whitt-execution-engine/docs/requirements/index.md".

8. **SW5 step_05 prompt:** Replace loop mechanism with explicit batch processing: add step to read file list from `find` output, then process in batches of 10 files per step.

---

## Top 3 Highest-Impact Improvements

### 1. Fix GWT Routing Logic (SW5, affects T5 and T7)

**Problem:** GWT hooks use `given: "true"` which always routes to fix step, never completes. Creates infinite loop.

**Impact:** HIGH — Breaks iterative quality gates for formatting (T5) and TOC standardization (T7). Cannot achieve PASS state.

**Recommendation:**
```yaml
# Change from:
- gwt:
    - given: "true"
      then: step_t5_generate_fix

# To:
- gwt:
    - given: "get_field('eval_result', 'verdict') == 'PASS'"
      then: step_t5_finalize
    - given: "get_field('eval_result', 'verdict') != 'PASS'"
      then: step_t5_generate_fix
```

**Affected SWs:** SW5 (workflow), SW4 (structs), SW3 (categories)

---

### 2. Fix save_to Hook Syntax (SW5, affects all steps)

**Problem:** save_to hooks use invalid two-element array `[bookmark, path]`. Schema expects nested array `[[bookmark, path]]` or separate actions.

**Impact:** HIGH — All save operations fail. No bookmarks created for downstream steps to reference.

**Recommendation:**
```yaml
# Change from:
- save_to: [merged_report, "benchmark-100-model-userflows/mvp-summary-report.md"]

# To:
- save_to:
    - merged_report
    - "./outputs/t5_merged.md"
```

**Affected SWs:** SW5 (workflow), SW4 (structs)

---

### 3. Add Missing Analysis Tasks (SW1-SW5, affects 19 tasks)

**Problem:** META-v6 skips analysis phases: overlap detection, categorization, prioritization, gap analysis. Jumps directly to execution.

**Impact:** HIGH — No informed decision-making. Consolidations happen without understanding overlaps, prioritization, or gaps.

**Recommendation:**
- **SW1:** Add T1.1-T1.3 (read/index.md, read/mvp-summary-report.md, generate overlap-analysis.md)
- **SW2:** Add T3.1-T3.4 (read/testing-guide.md, read/workflows-README.md, read/testing-strategy.md, generate categorization matrix)
- **SW5:** Add T9.1-T9.2 (extract formatting patterns, count unformatted files), T10.1-T10.2 (prioritize by impact/effort, create phased roadmap)

**Affected SWs:** SW1 (tasks), SW2 (outputs), SW5 (workflow)

---

## Additional High-Priority Improvements

### 4. Correct Category Misassignments (SW3, affects 5 tasks)

**Problem:** T2 (PARALLEL_FAN_OUT), T3 (ACCUMULATOR), T5 (ITERATIVE_REFINER), T6 (SEQUENTIAL_PROCESSOR) are incorrectly categorized. All should be DATA_TRANSFORMER.

**Impact:** MEDIUM — Incorrect categorization misguides YAML generation, but doesn't break execution.

**Recommendation:** Change SW3 step_02/03/05/06 prompts to use DATA_TRANSFORMER with iteration budget=1.

---

### 5. Add Shell Hooks for File Operations (SW5, affects 12 tasks)

**Problem:** YAML uses generative_entity for all operations. No shell steps for file system operations (grep, find, sed, rm).

**Impact:** MEDIUM — File operations unreliable. Model may hallucinate file paths or fail to execute commands correctly.

**Recommendation:** Replace LLM-based file operations with shell steps:
```yaml
# Change from:
prompt: "Delete requirements/index.md..."

# To:
shell:
  command: "rm requirements/index.md"
  fail_on_error: true
```

**Affected SWs:** SW5 (workflow), SW4 (structs)

---

### 6. Add Log Hooks to All Steps (SW5, affects 33 steps)

**Problem:** Only 3 steps have log hooks. Missing logs make debugging impossible.

**Impact:** MEDIUM — Cannot troubleshoot failures. No audit trail of step execution.

**Recommendation:** Add log hooks to every step:
```yaml
when:
  after_step_succeeds:
    - log:
        to_file_path: "./logs/{step_name}.log"
        event_fields: [step_name, duration_ms, output_tokens]
        level: info
```

**Affected SWs:** SW5 (workflow), SW4 (structs)

---

## Pass/Fail Determination

| SW | Pass/Fail | Reason |
|----|-----------|--------|
| SW1 | ❌ FAIL | Missing 4 tasks (T10-T12), lacks analysis phase, vague GWT criteria |
| SW2 | ❌ FAIL | Missing 11 task outputs, vague evaluation methods, no grep verification |
| SW3 | ❌ FAIL | 5/15 tasks misclassified, incorrect iteration budgets, missing leaf categorizations |
| SW4 | ❌ FAIL | Invalid T2 parallel structure, broken T5 loop, missing log hooks |
| SW5 | ❌ FAIL | Invalid GWT routing (infinite loop), invalid save_to syntax, missing 20 steps |

**Overall Verdict:** ❌ NEEDS_ITERATION — All 5 SWs fail baseline comparison. Critical structural issues (GWT, save_to) prevent execution. Coverage gaps (-44% to -81%) miss analysis phases essential for informed consolidation.

---

## Next Steps

1. **Immediate fixes:** Update SW5 prompts to fix GWT routing and save_to syntax.
2. **Task coverage:** Add missing analysis tasks (T1.1-T1.3, T3.1-T3.4, T10.1-T10.2) to SW1-SW5.
3. **Categorization correction:** Update SW3 prompts to use DATA_TRANSFORMER for 5 misclassified tasks.
4. **Verification methods:** Add shell hooks (grep, find, sed) to SW4-SW5 for concrete verification.
5. **Second iteration:** Re-run META-v6 with updated prompts, re-evaluate against this baseline.

---

**Report generated:** 2026-06-14
**Iteration:** 1
**Baseline reference:** `/docs/plans/meta-v6/baseline/single-shot/`
**META-v6 outputs:** `/docs/benchmarks/outputs/meta-workflow/meta-meta-v6-q8-20260614-194420/`