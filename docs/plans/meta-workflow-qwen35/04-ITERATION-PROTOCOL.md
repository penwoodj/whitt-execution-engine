# 04 — Iteration Protocol

> **Status:** Planning Phase — Documentation Only  
> **Phase:** Meta-Workflow Generator Development (Qwen 3.5-9B)  
> **Focus:** How to iterate each SW until quality surpasses opencode baseline

## Baseline Creation Process

The iteration protocol requires high-quality baselines for comparison. Baselines are created using OpenCode's Sisyphus agent with full capabilities (read, write, shell, browser, etc.) to generate complete workflow artifacts in a single shot without the five-stage pipeline.

### Baseline Generation Protocol

**Step 1: Load Prompt into OpenCode Session**

1. Use `session_read()` to load the prompt session:
   ```
   session_read(session_id="<session_id>", include_transcript=true)
   ```

2. Extract the user's prompt text from the message history

3. Store prompt in temporary file: `docs/plans/meta-workflow-qwen35/dataset/prompts/temp-prompt.md`

**Step 2: Invoke Sisyphus Agent**

1. Create Sisyphus delegation with full capabilities:
   ```
   delegate(agent="Sisyphus", prompt="Generate a complete workflow YAML for the following prompt: <prompt_text>. Produce: 1) Task breakdown, 2) Desired states, 3) Categories, 4) YAML skeletons, 5) Final workflow. Save each stage to separate markdown files.")
   ```

2. Wait for Sisyphus to complete (may take 5-10 minutes for complex prompts)

**Step 3: Extract Baseline Artifacts**

1. Read Sisyphus's output files (usually in workspace)

2. Extract each stage's output:
   - Task breakdown → `baseline-sw1-tasks.md`
   - Desired states → `baseline-sw2-outputs.md`
   - Categories → `baseline-sw3-categories.md`
   - YAML skeletons → `baseline-sw4-structs.md`
   - Final workflow → `baseline-sw5-workflow.yml`

3. Store baselines in `docs/plans/meta-workflow-qwen35/dataset/baseline-opencode/<prompt_id>/`

**Step 4: Validate Baseline Completeness**

1. Check that all 5 baseline files exist
2. Validate that baseline-sw5-workflow.yml is parseable YAML
3. Validate that baseline-sw5-workflow.yml passes schema validation (schema_valid=true in logs)
4. If baseline is incomplete or invalid, regenerate (repeat Steps 2-3)

**Baseline Storage Structure:**

```
docs/plans/meta-workflow-qwen35/dataset/baseline-opencode/
├── prompt-001/
│   ├── baseline-sw1-tasks.md
│   ├── baseline-sw2-outputs.md
│   ├── baseline-sw3-categories.md
│   ├── baseline-sw4-structs.md
│   └── baseline-sw5-workflow.yml
├── prompt-002/
│   ├── ...
└── prompt-003/
    └── ...
```

### Baseline Quality Assessment

Before using baselines for comparison, assess their quality:

**Assessment Dimensions:**

1. **Schema Validity** — Does baseline-sw5-workflow.yml pass schema validation?
2. **Input Coverage** — Does baseline cover all tasks from original prompt?
3. **Task Granularity** — Are atomic tasks ≤ 5 story points?
4. **Hook Presence** — Does baseline have adequate hook coverage (≥ 3 triggers per step)?
5. **GWT Validity** — Do GWT expressions parse without errors?
6. **Action Variety** — Does baseline use multiple action types (≥ 3 types)?
7. **Template Interpolation** — Does baseline use {{...}} references correctly?

**Scoring Baselines:**

For each dimension, score baseline from 0-10:
- 10: Excellent (exceeds expectation)
- 8-9: Good (meets expectation with minor issues)
- 6-7: Acceptable (meets expectation with notable issues)
- 4-5: Marginal (barely acceptable, significant issues)
- 0-3: Poor (unacceptable)

**Overall Baseline Score:**

Calculate weighted average across dimensions:
- Schema validity: 20% weight
- Input coverage: 20% weight
- Task granularity: 15% weight
- Hook presence: 15% weight
- GWT validity: 15% weight
- Action variety: 7.5% weight
- Template interpolation: 7.5% weight

**Baseline Minimum Threshold:**

Only use baselines with overall score ≥ 60/100 (acceptable quality). If baseline score < 60, regenerate with more explicit prompt to Sisyphus.

## Comparison Metrics

Comparison metrics quantify how well generated outputs surpass baselines. Each sub-workflow has specific metrics aligned with its stage objectives.

### SW1 Comparison Metrics

**Metric 1: Task Coverage Completeness**

- **Definition:** Percentage of tasks from original prompt that appear in generated tasks.md
- **Baseline Typical:** 80-90% (some tasks dropped or merged)
- **Surpass Threshold:** 100% (all tasks present)
- **Measurement:** Manual comparison of original prompt vs generated tasks.md

**Metric 2: Task Granularity Score**

- **Definition:** Average story points per atomic task (lower is better)
- **Baseline Typical:** 6-8 points (overly coarse tasks)
- **Surpass Threshold:** ≤ 5 points (all tasks atomic or decomposed)
- **Measurement:** Parse tasks.md, calculate average story points for atomic tasks

**Metric 3: Naming Quality Score**

- **Definition:** Percentage of tasks following verb-noun pattern
- **Baseline Typical:** 70-80% (some tasks poorly named)
- **Surpass Threshold:** ≥ 95% (almost all tasks well-named)
- **Measurement:** Regex analysis of task names for verb-noun pattern

**Metric 4: Hierarchy Depth Score**

- **Definition:** Maximum nesting depth (lower is better for maintainability)
- **Baseline Typical:** 2-3 levels (reasonable but could be flatter)
- **Surpass Threshold:** ≤ 2 levels (flat hierarchy preferred)
- **Measurement:** Parse tasks.md, calculate maximum nesting depth

**Metric 5: Task Description Quality Score**

- **Definition:** Percentage of tasks with clear, specific descriptions
- **Baseline Typical:** 75-85% (some tasks vague or generic)
- **Surpass Threshold:** ≥ 90% (almost all tasks well-described)
- **Measurement:** Manual review of task descriptions

**SW1 Overall Score Calculation:**

- Task coverage completeness: 25%
- Task granularity score: 25%
- Naming quality score: 20%
- Hierarchy depth score: 15%
- Task description quality score: 15%

**Surpass Condition:** Overall score ≥ 80/100 AND all individual thresholds met.

### SW2 Comparison Metrics

**Metric 1: Desired State Coverage**

- **Definition:** Percentage of tasks with corresponding desired state criteria
- **Baseline Typical:** 85-95% (some tasks missing desired states)
- **Surpass Threshold:** 100% (all tasks have desired states)
- **Measurement:** Count tasks in tasks.md vs criteria sections in outputs.md

**Metric 2: Criteria Testability Score**

- **Definition:** Percentage of criteria that are testable (observable, measurable)
- **Baseline Typical:** 70-80% (some criteria subjective or unobservable)
- **Surpass Threshold:** ≥ 90% (almost all criteria testable)
- **Measurement:** Manual review of GWT criteria for testability

**Metric 3: Criteria Unambiguity Score**

- **Definition:** Percentage of criteria with clear pass/fail conditions
- **Baseline Typical:** 65-75% (some criteria ambiguous)
- **Surpass Threshold:** ≥ 85% (almost all criteria unambiguous)
- **Measurement:** Manual review of GWT criteria for ambiguity

**Metric 4: Criteria Count Balance**

- **Definition:** Average criteria per task (not too many, not too few)
- **Baseline Typical:** 2.5-3.5 criteria per task (reasonable)
- **Surpass Threshold:** 1.5-3.0 criteria per task (balanced)
- **Measurement:** Count criteria, divide by task count

**Metric 5: GWT Syntax Validity**

- **Definition:** Percentage of criteria that parse in GWT evaluator without errors
- **Baseline Typical:** 60-70% (some criteria have syntax errors)
- **Surpass Threshold:** 100% (all criteria parse correctly)
- **Measurement:** Parse all criteria in GWT evaluator at `src/workflow/hooks/gwt.rs`

**SW2 Overall Score Calculation:**

- Desired state coverage: 25%
- Criteria testability score: 25%
- Criteria ambiguity score: 20%
- Criteria count balance: 15%
- GWT syntax validity: 15%

**Surpass Condition:** Overall score ≥ 80/100 AND all individual thresholds met.

### SW3 Comparison Metrics

**Metric 1: Category Assignment Coverage**

- **Definition:** Percentage of tasks with category assignments
- **Baseline Typical:** 90-95% (some tasks unassigned)
- **Surpass Threshold:** 100% (all tasks categorized)
- **Measurement:** Count tasks in tasks.md vs category assignments in categories.md

**Metric 2: Canonical Set Adherence**

- **Definition:** Percentage of categories from canonical set (no invented categories)
- **Baseline Typical:** 85-90% (some invented categories)
- **Surpass Threshold:** 100% (all categories from canonical set)
- **Measurement:** Compare categories against canonical set of 10

**Metric 3: Mutual Exclusivity**

- **Definition:** Percentage of tasks with exactly one category
- **Baseline Typical:** 80-90% (some tasks have multiple categories)
- **Surpass Threshold:** 100% (all tasks have one category)
- **Measurement:** Count tasks, verify each has exactly one category

**Metric 4: Category Reasoning Presence**

- **Definition:** Percentage of category assignments with reasoning explanations
- **Baseline Typical:** 70-80% (some assignments lack reasoning)
- **Surpass Threshold:** 100% (all assignments have reasoning)
- **Measurement:** Check each assignment has reasoning section

**Metric 5: Category Consistency Score**

- **Definition:** Percentage of similar tasks with same category assignment
- **Baseline Typical:** 75-85% (some inconsistency)
- **Surpass Threshold:** ≥ 90% (high consistency)
- **Measurement:** Manually identify similar tasks, compare categories

**Metric 6: Category Distribution Balance**

- **Definition:** Category diversity (categories used / total tasks)
- **Baseline Typical:** 0.2-0.3 (low diversity, all tasks same category)
- **Surpass Threshold:** ≥ 0.3 (high diversity, mix of categories)
- **Measurement:** Count unique categories, divide by total tasks

**SW3 Overall Score Calculation:**

- Category assignment coverage: 20%
- Canonical set adherence: 20%
- Mutual exclusivity: 20%
- Category reasoning presence: 15%
- Category consistency score: 15%
- Category distribution balance: 10%

**Surpass Condition:** Overall score ≥ 80/100 AND all individual thresholds met.

### SW4 Comparison Metrics

**Metric 1: Skeleton Coverage**

- **Definition:** Percentage of tasks with corresponding YAML skeleton structures
- **Baseline Typical:** 85-90% (some tasks missing skeletons)
- **Surpass Threshold:** 100% (all tasks have skeletons)
- **Measurement:** Count tasks in tasks.md vs skeletons in structs.md

**Metric 2: YAML Syntax Validity**

- **Definition:** Percentage of skeletons that parse without serde_yaml errors
- **Baseline Typical:** 75-85% (some skeletons have syntax errors)
- **Surpass Threshold:** 100% (all skeletons parse correctly)
- **Measurement:** Parse each YAML block with serde_yaml

**Metric 3: Schema Compliance**

- **Definition:** Percentage of skeletons following unified schema structure
- **Baseline Typical:** 70-80% (some skeletons don't follow schema)
- **Surpass Threshold:** 100% (all skeletons follow schema)
- **Measurement:** Compare skeleton structure against unified schema line 318-522

**Metric 4: Model Reference Correctness**

- **Definition:** Percentage of skeletons using `${models.primary-analyzer}`
- **Baseline Typical:** 80-90% (some skeletons use wrong model reference)
- **Surpass Threshold:** 100% (all skeletons use correct reference)
- **Measurement:** Search skeletons for `${models.primary-analyzer}` pattern

**Metric 5: Hook Presence**

- **Definition:** Average number of triggers per skeleton
- **Baseline Typical:** 2-3 triggers (minimal hook coverage)
- **Surpass Threshold:** ≥ 5 triggers (comprehensive hook coverage)
- **Measurement:** Count triggers in each skeleton, calculate average

**Metric 6: Non-YAML Block Absence**

- **Definition:** Percentage of skeletons with only YAML code blocks
- **Baseline Typical:** 90-95% (some non-YAML blocks present)
- **Surpass Threshold:** 100% (only YAML blocks, no rust/python/bash)
- **Measurement:** Search for code blocks with languages other than yaml

**SW4 Overall Score Calculation:**

- Skeleton coverage: 20%
- YAML syntax validity: 20%
- Schema compliance: 20%
- Model reference correctness: 15%
- Hook presence: 15%
- Non-YAML block absence: 10%

**Surpass Condition:** Overall score ≥ 80/100 AND all individual thresholds met.

### SW5 Comparison Metrics

**Metric 1: Schema Validity**

- **Definition:** schema_valid=true in logs
- **Baseline Typical:** 70-80% (some schema violations)
- **Surpass Threshold:** 100% (no schema violations)
- **Measurement:** Check logs for schema_valid flag

**Metric 2: Input Coverage**

- **Definition:** Percentage of tasks from tasks.md appearing as steps in workflow.yml
- **Baseline Typical:** 80-90% (some tasks missing)
- **Surpass Threshold:** 100% (all tasks present)
- **Measurement:** Extract step names from workflow.yml, compare to tasks.md

**Metric 3: YAML Parseability**

- **Definition:** Workflow parses without serde_yaml errors
- **Baseline Typical:** 75-85% (some parse errors)
- **Surpass Threshold:** 100% (no parse errors)
- **Measurement:** Parse workflow.yml with serde_yaml

**Metric 4: Dependency Correctness**

- **Definition:** Percentage of requires: references pointing to existing steps
- **Baseline Typical:** 85-90% (some broken dependencies)
- **Surpass Threshold:** 100% (all dependencies correct)
- **Measurement:** Extract requires: references, verify step existence

**Metric 5: Hook Completeness**

- **Definition:** Average number of triggers per step
- **Baseline Typical:** 2-3 triggers (minimal hook coverage)
- **Surpass Threshold:** ≥ 5 triggers (comprehensive hook coverage)
- **Measurement:** Count triggers in each step, calculate average

**Metric 6: Template Interpolation Correctness**

- **Definition:** Percentage of {{...}} references that resolve correctly
- **Baseline Typical:** 80-90% (some broken references)
- **Surpass Threshold:** 100% (all references resolve)
- **Measurement:** Check template references against defined variables

**Metric 7: Executability**

- **Definition:** Workflow runs on live Docker system without errors
- **Baseline Typical:** 70-80% (some execution failures)
- **Surpass Threshold:** 100% (workflow executes successfully)
- **Measurement:** Run workflow on live system, check for [workflow:end] event

**SW5 Overall Score Calculation:**

- Schema validity: 20%
- Input coverage: 15%
- YAML parseability: 15%
- Dependency correctness: 15%
- Hook completeness: 15%
- Template interpolation correctness: 10%
- Executability: 10%

**Surpass Condition:** Overall score ≥ 80/100 AND all individual thresholds met.

## Quality Gates

Quality gates are specific thresholds that must be met before proceeding to the next stage of iteration or before declaring success. Each sub-workflow has stage-specific quality gates that trigger fix loops.

### SW1 Quality Gates

**Gate 1: Task Coverage Completeness**

**Trigger Point:** After Step 5 (Final Assembly)

**Pass Condition:** 100% of tasks from original prompt appear in tasks.md

**Fail Condition:** Some tasks missing from tasks.md

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "tasks_in_md == tasks_in_prompt"
          when: "All tasks from prompt present in output"
          then: { route_to: accept_sw1 }
        - given: "tasks_in_md < tasks_in_prompt"
          when: "Some tasks missing from output"
          then: { route_to: fix_task_coverage }
```

**Fix Strategy:** Add missing tasks (refer to Fix Loop 1 in SW3 specs)

**Gate 2: Task Granularity**

**Trigger Point:** After Step 5 (Final Assembly)

**Pass Condition:** All atomic tasks ≤ 5 story points

**Fail Condition:** Some atomic tasks > 5 story points

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "max_story_points <= 5"
          when: "All atomic tasks are ≤ 5 story points"
          then: { route_to: validate_naming }
        - given: "max_story_points > 5"
          when: "Some atomic tasks > 5 story points (overly coarse)"
          then: { route_to: fix_decomposition }
```

**Fix Strategy:** Decompose over-coarse tasks (refer to Fix Loop 2 in SW3 specs)

**Gate 3: Naming Quality**

**Trigger Point:** After Step 4 (GWT Quality Gate)

**Pass Condition:** ≥ 95% of tasks follow verb-noun pattern

**Fail Condition:** < 95% of tasks follow verb-noun pattern

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "naming_quality_score >= 0.95"
          when: "≥ 95% of tasks follow verb-noun pattern"
          then: { route_to: validate_verbosity }
        - given: "naming_quality_score < 0.95"
          when: "< 95% of tasks follow verb-noun pattern"
          then: { route_to: fix_naming }
```

**Fix Strategy:** Rename poorly named tasks (refer to Fix Loop 3 in SW3 specs)

**Gate 4: Hierarchy Depth**

**Trigger Point:** After Step 5 (Final Assembly)

**Pass Condition:** Maximum nesting depth ≤ 2 levels

**Fail Condition:** Maximum nesting depth > 2 levels

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "max_nesting_depth <= 2"
          when: "Hierarchy is flat (≤ 2 levels)"
          then: { route_to: accept_sw1 }
        - given: "max_nesting_depth > 2"
          when: "Hierarchy is deep (> 2 levels)"
          then: { route_to: flatten_hierarchy }
```

**Fix Strategy:** Flatten hierarchy by decomposing deep tasks (refer to Fix Loop 2 in SW3 specs)

**Gate 5: Task Description Quality**

**Trigger Point:** After Step 4 (GWT Quality Gate)

**Pass Condition:** ≥ 90% of tasks have clear, specific descriptions

**Fail Condition:** < 90% of tasks have clear, specific descriptions

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "description_quality_score >= 0.90"
          when: "≥ 90% of tasks have clear, specific descriptions"
          then: { route_to: accept_sw1 }
        - given: "description_quality_score < 0.90"
          when: "< 90% of tasks have clear, specific descriptions"
          then: { route_to: fix_descriptions }
```

**Fix Strategy:** Clarify vague task descriptions (refer to Fix Loop 4 in SW3 specs)

### SW2 Quality Gates

**Gate 1: Desired State Coverage**

**Trigger Point:** After Step 6 (Merge and Finalize)

**Pass Condition:** 100% of tasks have corresponding desired state criteria

**Fail Condition:** Some tasks lack desired state criteria

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "criteria_count == task_count"
          when: "All tasks have desired state criteria"
          then: { route_to: validate_testability }
        - given: "criteria_count < task_count"
          when: "Some tasks lack desired state criteria"
          then: { route_to: add_missing_states }
```

**Fix Strategy:** Add missing desired states (refer to Fix Loop 1 in SW2 specs)

**Gate 2: Criteria Testability**

**Trigger Point:** After Step 4 (GWT Quality Gate per Chunk)

**Pass Condition:** ≥ 90% of criteria are testable

**Fail Condition:** < 90% of criteria are testable

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "testability_score >= 0.90"
          when: "≥ 90% of criteria are testable"
          then: { route_to: validate_unambiguous }
        - given: "testability_score < 0.90"
          when: "< 90% of criteria are testable"
          then: { route_to: fix_testability }
```

**Fix Strategy:** Fix untestable criteria (refer to Fix Loop 2 in SW2 specs)

**Gate 3: Criteria Unambiguity**

**Trigger Point:** After Step 4 (GWT Quality Gate per Chunk)

**Pass Condition:** ≥ 85% of criteria are unambiguous

**Fail Condition:** < 85% of criteria are unambiguous

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "unambiguity_score >= 0.85"
          when: "≥ 85% of criteria are unambiguous"
          then: { route_to: validate_scope }
        - given: "unambiguity_score < 0.85"
          when: "< 85% of criteria are unambiguous"
          then: { route_to: fix_ambiguity }
```

**Fix Strategy:** Fix ambiguous criteria (refer to Fix Loop 3 in SW2 specs)

**Gate 4: Criteria Count Balance**

**Trigger Point:** After Step 5 (Category Distribution Validation)

**Pass Condition:** 1.5-3.0 criteria per task (balanced)

**Fail Condition:** < 1.5 or > 3.0 criteria per task (unbalanced)

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "avg_criteria_per_task >= 1.5 && avg_criteria_per_task <= 3.0"
          when: "Criteria count is balanced (1.5-3.0 per task)"
          then: { route_to: validate_gwt_syntax }
        - given: "avg_criteria_per_task < 1.5"
          when: "Criteria count too low (under-constrained)"
          then: { route_to: add_more_criteria }
        - given: "avg_criteria_per_task > 3.0"
          when: "Criteria count too high (over-constrained)"
          then: { route_to: remove_criteria }
```

**Fix Strategy:** Add or remove criteria to balance (refer to Fix Loop 4 in SW2 specs)

**Gate 5: GWT Syntax Validity**

**Trigger Point:** After Step 4 (GWT Quality Gate per Chunk)

**Pass Condition:** 100% of criteria parse in GWT evaluator

**Fail Condition:** Some criteria have GWT syntax errors

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "gwt_parse_success_rate == 1.0"
          when: "All criteria parse correctly in GWT evaluator"
          then: { route_to: accept_sw2 }
        - given: "gwt_parse_success_rate < 1.0"
          when: "Some criteria have GWT syntax errors"
          then: { route_to: fix_gwt_syntax }
```

**Fix Strategy:** Fix GWT syntax errors (refer to Fix Loop 5 in SW2 specs)

### SW3 Quality Gates

**Gate 1: Category Assignment Coverage**

**Trigger Point:** After Step 5 (Final Assembly)

**Pass Condition:** 100% of tasks have category assignments

**Fail Condition:** Some tasks lack category assignments

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "category_count == task_count"
          when: "All tasks have category assignments"
          then: { route_to: validate_canonical_set }
        - given: "category_count < task_count"
          when: "Some tasks lack category assignments"
          then: { route_to: add_categories }
```

**Fix Strategy:** Add missing category assignments (refer to Fix Loop 1 in SW3 specs)

**Gate 2: Canonical Set Adherence**

**Trigger Point:** After Step 4 (GWT Quality Gate per Task)

**Pass Condition:** 100% of categories from canonical set

**Fail Condition:** Some categories not from canonical set

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "canonical_set_adherence == true"
          when: "All categories from canonical set"
          then: { route_to: validate_mutually_exclusive }
        - given: "canonical_set_adherence == false"
          when: "Some categories not from canonical set (invented)"
          then: { route_to: fix_categories }
```

**Fix Strategy:** Fix invalid categories (refer to Fix Loop 2 in SW3 specs)

**Gate 3: Mutual Exclusivity**

**Trigger Point:** After Step 4 (GWT Quality Gate per Task)

**Pass Condition:** 100% of tasks have exactly one category

**Fail Condition:** Some tasks have multiple categories

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "mutual_exclusivity == true"
          when: "One category per task (mutually exclusive)"
          then: { route_to: validate_reasoning }
        - given: "mutual_exclusivity == false"
          when: "Some tasks have multiple categories"
          then: { route_to: select_single_category }
```

**Fix Strategy:** Select single category for multi-category tasks (refer to Fix Loop 3 in SW3 specs)

**Gate 4: Category Reasoning Presence**

**Trigger Point:** After Step 4 (GWT Quality Gate per Task)

**Pass Condition:** 100% of assignments have reasoning explanations

**Fail Condition:** Some assignments lack reasoning explanations

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "reasoning_presence == true"
          when: "All assignments have reasoning explanations"
          then: { route_to: validate_consistency }
        - given: "reasoning_presence == false"
          when: "Some assignments lack reasoning explanations"
          then: { route_to: add_reasoning }
```

**Fix Strategy:** Add missing reasoning (refer to Fix Loop 4 in SW3 specs)

**Gate 5: Category Consistency**

**Trigger Point:** After Step 4 (GWT Quality Gate per Task)

**Pass Condition:** ≥ 90% of similar tasks have same category assignment

**Fail Condition:** < 90% of similar tasks have same category assignment

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "category_consistency_score >= 0.90"
          when: "Similar tasks have same category assignment (high consistency)"
          then: { route_to: validate_distribution }
        - given: "category_consistency_score < 0.90"
          when: "Similar tasks have different category assignments (inconsistent)"
          then: { route_to: harmonize_categories }
```

**Fix Strategy:** Harmonize inconsistent categories (refer to Fix Loop 4 in SW3 specs)

**Gate 6: Category Distribution Balance**

**Trigger Point:** After Step 5 (Category Distribution Validation)

**Pass Condition:** Category diversity ≥ 0.3 (at least 30% of tasks have different categories)

**Fail Condition:** Category diversity < 0.3 (all tasks same category)

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "category_diversity >= 0.3"
          when: "High category diversity (good mix of categories)"
          then: { route_to: accept_sw3 }
        - given: "category_diversity < 0.3"
          when: "Low category diversity (all tasks same category)"
          then: { route_to: improve_diversity }
```

**Fix Strategy:** Improve category diversity (refer to Fix Loop 4 in SW3 specs)

### SW4 Quality Gates

**Gate 1: Skeleton Coverage**

**Trigger Point:** After Step 7 (Final Assembly)

**Pass Condition:** 100% of tasks have YAML skeleton structures

**Fail Condition:** Some tasks lack YAML skeleton structures

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "skeleton_count == task_count"
          when: "All tasks have YAML skeleton structures"
          then: { route_to: validate_yaml_syntax }
        - given: "skeleton_count < task_count"
          when: "Some tasks lack YAML skeleton structures"
          then: { route_to: add_missing_skeletons }
```

**Fix Strategy:** Add missing skeletons (refer to Fix Loop 1 in SW4 specs)

**Gate 2: YAML Syntax Validity**

**Trigger Point:** After Step 5 (YAML Syntax Validation)

**Pass Condition:** 100% of skeletons parse without serde_yaml errors

**Fail Condition:** Some skeletons have YAML syntax errors

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "yaml_parse_success_rate == 1.0"
          when: "All skeletons parse correctly"
          then: { route_to: validate_schema_compliance }
        - given: "yaml_parse_success_rate < 1.0"
          when: "Some skeletons have YAML syntax errors"
          then: { route_to: fix_yaml_syntax }
```

**Fix Strategy:** Fix YAML syntax errors (refer to Fix Loop 2 in SW4 specs)

**Gate 3: Schema Compliance**

**Trigger Point:** After Step 6 (Schema Validation)

**Pass Condition:** 100% of skeletons follow unified schema structure

**Fail Condition:** Some skeletons don't follow unified schema structure

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "schema_compliance == true"
          when: "All skeletons follow unified schema structure"
          then: { route_to: validate_model_reference }
        - given: "schema_compliance == false"
          when: "Some skeletons don't follow unified schema structure"
          then: { route_to: fix_schema_mismatches }
```

**Fix Strategy:** Fix schema mismatches (refer to Fix Loop 2 in SW4 specs)

**Gate 4: Model Reference Correctness**

**Trigger Point:** After Step 7 (Final Assembly)

**Pass Condition:** 100% of skeletons use `${models.primary-analyzer}`

**Fail Condition:** Some skeletons don't use `${models.primary-analyzer}`

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "model_reference_correctness == true"
          when: "All skeletons use ${models.primary-analyzer}"
          then: { route_to: validate_hook_wiring }
        - given: "model_reference_correctness == false"
          when: "Some skeletons don't use ${models.primary-analyzer}"
          then: { route_to: fix_model_references }
```

**Fix Strategy:** Fix model references (refer to Fix Loop 3 in SW4 specs)

**Gate 5: Hook Presence**

**Trigger Point:** After Step 7 (Final Assembly)

**Pass Condition:** Average ≥ 5 triggers per skeleton

**Fail Condition:** Average < 5 triggers per skeleton

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "avg_triggers_per_skeleton >= 5"
          when: "Adequate hook coverage (≥ 5 triggers per skeleton)"
          then: { route_to: validate_yaml_blocks_only }
        - given: "avg_triggers_per_skeleton < 5"
          when: "Insufficient hook coverage (< 5 triggers per skeleton)"
          then: { route_to: add_hooks }
```

**Fix Strategy:** Add hooks (refer to Fix Loop 3 in SW4 specs)

**Gate 6: Non-YAML Block Absence**

**Trigger Point:** After Step 7 (Final Assembly)

**Pass Condition:** 100% of code blocks are YAML only

**Fail Condition:** Non-YAML code blocks present (rust, python, bash)

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "yaml_blocks_only == true"
          when: "Only YAML code blocks present (no rust/python/bash)"
          then: { route_to: accept_sw4 }
        - given: "yaml_blocks_only == false"
          when: "Non-YAML code blocks present"
          then: { route_to: remove_non_yaml_blocks }
```

**Fix Strategy:** Remove non-YAML blocks (refer to Fix Loop 3 in SW4 specs)

### SW5 Quality Gates

**Gate 1: Schema Validity**

**Trigger Point:** After Step 6 (Deterministic Post-Processing)

**Pass Condition:** schema_valid=true in logs

**Fail Condition:** schema_valid=false or unknown keys present

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "schema_valid == true"
          when: "Schema validation passed (schema_valid=true in logs)"
          then: { route_to: validate_input_coverage }
        - given: "schema_valid == false"
          when: "Schema validation failed"
          then: { route_to: fix_schema_validation }
```

**Fix Strategy:** Fix schema violations (refer to Fix Loop 1 in SW5 specs)

**Gate 2: Input Coverage**

**Trigger Point:** After Step 7 (Exhaustive Review Loop)

**Pass Condition:** 100% of tasks from tasks.md appear as steps in workflow.yml

**Fail Condition:** Some tasks missing from workflow.yml

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "tasks_in_workflow == tasks_in_md"
          when: "All tasks from tasks.md present in workflow"
          then: { route_to: validate_yaml_parseability }
        - given: "tasks_in_workflow < tasks_in_md"
          when: "Some tasks missing from workflow"
          then: { route_to: add_missing_tasks }
```

**Fix Strategy:** Add missing tasks (refer to Fix Loop 1 in SW5 specs)

**Gate 3: YAML Parseability**

**Trigger Point:** After Step 6 (Deterministic Post-Processing)

**Pass Condition:** Workflow parses without serde_yaml errors

**Fail Condition:** Workflow doesn't parse (serde_yaml error)

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "yaml_parse_success == true"
          when: "Workflow parses without errors"
          then: { route_to: validate_dependencies }
        - given: "yaml_parse_success == false"
          when: "Workflow has parse errors"
          then: { route_to: fix_yaml_parse_errors }
```

**Fix Strategy:** Fix parse errors (refer to Fix Loop 2 in SW5 specs)

**Gate 4: Dependency Correctness**

**Trigger Point:** After Step 7 (Exhaustive Review Loop)

**Pass Condition:** 100% of requires: references point to existing steps

**Fail Condition:** Some requires: references don't exist

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "all_dependencies_valid == true"
          when: "All dependencies reference existing steps"
          then: { route_to: validate_hook_completeness }
        - given: "all_dependencies_valid == false"
          when: "Some dependencies reference non-existent steps"
          then: { route_to: fix_broken_dependencies }
```

**Fix Strategy:** Fix broken dependencies (refer to Fix Loop 2 in SW5 specs)

**Gate 5: Hook Completeness**

**Trigger Point:** After Step 7 (Exhaustive Review Loop)

**Pass Condition:** Average ≥ 5 triggers per step

**Fail Condition:** Average < 5 triggers per step

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "avg_triggers_per_step >= 5"
          when: "Adequate hook coverage (≥ 5 triggers per step)"
          then: { route_to: validate_template_interpolation }
        - given: "avg_triggers_per_step < 5"
          when: "Insufficient hook coverage (< 5 triggers per step)"
          then: { route_to: add_hooks }
```

**Fix Strategy:** Add hooks (refer to Fix Loop 3 in SW5 specs)

**Gate 6: Template Interpolation Correctness**

**Trigger Point:** After Step 8 (Final Pass)

**Pass Condition:** 100% of {{...}} references resolve correctly

**Fail Condition:** Some {{...}} references fail to resolve

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "all_references_resolve == true"
          when: "All template references resolve correctly"
          then: { route_to: validate_executability }
        - given: "all_references_resolve == false"
          when: "Some template references fail to resolve"
          then: { route_to: fix_broken_references }
```

**Fix Strategy:** Fix broken references (refer to Fix Loop 2 in SW5 specs)

**Gate 7: Executability**

**Trigger Point:** After Step 8 (Final Pass)

**Pass Condition:** Workflow runs on live Docker system without errors

**Fail Condition:** Workflow panics or crashes on live system

**GWT Evaluation:**
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "execution_success == true"
          when: "Workflow executes successfully on live system"
          then: { route_to: accept_sw5 }
        - given: "execution_success == false"
          when: "Workflow fails on live system"
          then: { route_to: fix_execution_issues }
```

**Fix Strategy:** Fix execution issues (refer to Fix Loop 1 in SW5 specs)

## When to Move to Next SW

The iteration protocol defines strict criteria for advancing from one sub-workflow to the next. These gates ensure quality before proceeding.

### Advancement Criteria

**From SW1 to SW2:**

1. **SW1 quality gates all pass** (Task coverage, granularity, naming, hierarchy, description quality)
2. **SW1 overall score ≥ 80/100**
3. **SW1 surpasses baseline on at least 3 metrics** (e.g., better task coverage, finer granularity, better naming)
4. **SW1 iteration count ≤ 5** (prevent infinite loops)

**Verification:** Check SW1 iteration report for evidence of meeting criteria.

**From SW2 to SW3:**

1. **SW2 quality gates all pass** (Coverage, testability, unambiguity, count balance, GWT syntax)
2. **SW2 overall score ≥ 80/100**
3. **SW2 surpasses baseline on at least 3 metrics** (e.g., better coverage, more testable criteria, fewer syntax errors)
4. **SW2 iteration count ≤ 5**

**Verification:** Check SW2 iteration report for evidence of meeting criteria.

**From SW3 to SW4:**

1. **SW3 quality gates all pass** (Coverage, canonical set, mutual exclusivity, reasoning, consistency, distribution)
2. **SW3 overall score ≥ 80/100**
3. **SW3 surpasses baseline on at least 3 metrics** (e.g., better coverage, higher consistency, better diversity)
4. **SW3 iteration count ≤ 5**

**Verification:** Check SW3 iteration report for evidence of meeting criteria.

**From SW4 to SW5:**

1. **SW4 quality gates all pass** (Coverage, syntax validity, schema compliance, model reference, hook presence, non-YAML block absence)
2. **SW4 overall score ≥ 80/100**
3. **SW4 surpasses baseline on at least 3 metrics** (e.g., better coverage, fewer syntax errors, better schema compliance)
4. **SW4 iteration count ≤ 5**

**Verification:** Check SW4 iteration report for evidence of meeting criteria.

**From SW5 to Completion:**

1. **SW5 quality gates all pass** (Schema validity, input coverage, parseability, dependencies, hook completeness, template interpolation, executability)
2. **SW5 overall score ≥ 80/100**
3. **SW5 surpasses baseline on at least 3 metrics** (e.g., better schema validity, fewer missing tasks, better executability)
4. **SW5 iteration count ≤ 5**

**Verification:** Check SW5 iteration report for evidence of meeting criteria.

### Block and Escalation Rules

**Block: SW1 Cannot Advance**

**Trigger Conditions:**
- SW1 iteration count = 5 without meeting quality gates
- SW1 overall score < 80/100 across all iterations
- SW1 fails to surpass baseline on all metrics

**Escalation Options:**

1. **Re-examine Prompt** — Prompt may be too complex or ambiguous for current model
   - Simplify prompt by removing sub-objectives
   - Break prompt into multiple independent prompts
   - Move complex prompt to deferred dataset

2. **Model Limitation** — Qwen 3.5-9B may be insufficient for prompt complexity
   - Document model limitation in iteration report
   - Move prompt to deferred dataset for future model improvements
   - Focus on simpler prompts that model can handle

3. **Architectural Issue** — SW1 design may have fundamental flaw
   - Review task decomposition logic in SW1 prompt templates
   - Add additional decomposition stages (e.g., pre-decomposition)
   - Create issue for architectural reconsideration

**Block: SW2 Cannot Advance**

**Trigger Conditions:**
- SW2 iteration count = 5 without meeting quality gates
- SW2 overall score < 80/100 across all iterations
- SW2 fails to surpass baseline on all metrics

**Escalation Options:**

1. **SW1 Output Issue** — SW2 depends on SW1 output, SW1 may have quality issues
   - Re-run SW1 with stricter quality gates
   - Improve SW1 task decomposition before running SW2
   - Verify SW1 output quality before advancing

2. **Criteria Generation Issue** — Model may struggle with GWT format
   - Simplify GWT criteria requirements (reduce complexity)
   - Provide more examples in SW2 prompt templates
   - Switch to simpler testable criteria format (not GWT)

3. **Architectural Issue** — SW2 design may have fundamental flaw
   - Review desired state generation logic in SW2 prompt templates
   - Add more explicit quality gate definitions
   - Create issue for architectural reconsideration

**Block: SW3 Cannot Advance**

**Trigger Conditions:**
- SW3 iteration count = 5 without meeting quality gates
- SW3 overall score < 80/100 across all iterations
- SW3 fails to surpass baseline on all metrics

**Escalation Options:**

1. **SW2 Output Issue** — SW3 depends on SW2 output, SW2 may have quality issues
   - Re-run SW2 with stricter quality gates
   - Improve SW2 criteria quality before running SW3
   - Verify SW2 output quality before advancing

2. **Categorization Logic Issue** — Model may struggle with category assignment
   - Simplify canonical set (reduce from 10 to 7 categories)
   - Provide more category examples in SW3 prompt templates
   - Use heuristic categorization instead of LLM-based categorization

3. **Architectural Issue** — SW3 design may have fundamental flaw
   - Review categorization logic in SW3 prompt templates
   - Add more explicit canonical set definitions
   - Create issue for architectural reconsideration

**Block: SW4 Cannot Advance**

**Trigger Conditions:**
- SW4 iteration count = 5 without meeting quality gates
- SW4 overall score < 80/100 across all iterations
- SW4 fails to surpass baseline on all metrics

**Escalation Options:**

1. **SW3 Output Issue** — SW4 depends on SW3 output, SW3 may have quality issues
   - Re-run SW3 with stricter quality gates
   - Improve SW3 categorization before running SW4
   - Verify SW3 output quality before advancing

2. **YAML Generation Issue** — Model may struggle with YAML syntax
   - Add more YAML examples in SW4 prompt templates
   - Use post-processing script to fix common YAML issues
   - Implement stricter validation after YAML generation

3. **Architectural Issue** — SW4 design may have fundamental flaw
   - Review YAML generation logic in SW4 prompt templates
   - Add more schema line number references
   - Create issue for architectural reconsideration

**Block: SW5 Cannot Advance**

**Trigger Conditions:**
- SW5 iteration count = 5 without meeting quality gates
- SW5 overall score < 80/100 across all iterations
- SW5 fails to surpass baseline on all metrics

**Escalation Options:**

1. **SW4 Output Issue** — SW5 depends on SW4 output, SW4 may have quality issues
   - Re-run SW4 with stricter quality gates
   - Improve SW4 skeleton quality before running SW5
   - Verify SW4 output quality before advancing

2. **Assembly Issue** — Model may struggle with complex workflow assembly
   - Simplify assembly by processing steps in smaller groups
   - Use more explicit assembly instructions in SW5 prompt templates
   - Implement more granular validation after each assembly stage

3. **Architectural Issue** — SW5 design may have fundamental flaw
   - Review assembly logic in SW5 prompt templates
   - Add more extensive validation scripts
   - Create issue for architectural reconsideration

### Final Block: Entire Pipeline Cannot Complete

**Trigger Conditions:**
- All sub-workflows hit block conditions
- No prompts in dataset successfully generate executable workflows
- Overall pipeline success rate < 20% (4/5 or more prompts fail)

**Escalation Options:**

1. **Model Capability Issue** — Qwen 3.5-9B may be insufficient for this use case
   - Document model limitation comprehensively
   - Consider alternative models (larger, different architecture)
   - Create issue for model reevaluation

2. **Architectural Reconsideration** — 5-stage pipeline may not be right approach
   - Simplify pipeline (fewer stages, different decomposition)
   - Consider alternative architectures (single-shot with better prompts)
   - Create issue for architectural reconsideration

3. **Scope Reduction** — Project may be too ambitious for current state
   - Reduce prompt dataset complexity (focus on simpler prompts)
   - Reduce quality thresholds (accept lower quality)
   - Document realistic expectations in project scope

## Maximum Iterations

Each sub-workflow has a maximum of 5 iterations per prompt. This limit prevents infinite loops and forces escalation when quality cannot be achieved.

### Iteration Counter Implementation

**Tracking Iteration Count:**

Each sub-workflow tracks iteration count via:

1. **Bookmark State** — Store iteration count in bookmark store
2. **File-Based Counter** — Write iteration number to file (for restart capability)
3. **Log Metadata** — Log iteration count in each run log

**Implementation:**

```yaml
when:
  before_step_starts:
    - shell:
        command: "if [ -f {{iteration_counter_path}} ]; then cat {{iteration_counter_path}}; else echo '0'; fi"
        bookmark_as: "iteration_count"
        fail_on_error: false
    - shell:
        command: "echo $(( {{bookmarks.iteration_count}} + 1 )) > {{iteration_counter_path}}"
        fail_on_error: false
```

**Iteration Count Check:**

```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "{{bookmarks.iteration_count}} < 5"
          when: "Iteration count below maximum ({{bookmarks.iteration_count}} < 5), continue iteration"
          then: { route_to: continue_iteration }
        - given: "{{bookmarks.iteration_count}} >= 5"
          when: "Maximum iterations reached ({{bookmarks.iteration_count}} >= 5), stop iteration"
          then: { route_to: max_iterations_reached }
```

### Max Iterations Reached Behavior

**When Max Iterations Reached:**

1. **Log Catastrophic Failure** — Document all quality gate failures
2. **Escalation** — Apply block and escalation rules
3. **Documentation** — Create comprehensive failure report
4. **Decision** — Determine whether to defer prompt or escalate architectural issue

**Failure Report Contents:**

```
# Failure Report: <prompt_id> - <sub-workflow>

## Max Iterations Reached

**Iteration Count:** 5
**Overall Score:** <score>/100
**Quality Gates Failed:**
- Gate 1: <gate_name> - <failure_description>
- Gate 2: <gate_name> - <failure_description>
- ...

## Metrics vs Baseline

| Metric | Baseline | Generated | Surpasses |
|--------|----------|-----------|-----------|
| <metric_1> | <baseline_score> | <generated_score> | <yes/no> |
| <metric_2> | <baseline_score> | <generated_score> | <yes/no> |
| ...

## Escalation Decision

<escalation_decision>:
- Option chosen: <option>
- Rationale: <rationale>
- Next steps: <next_steps>
```

## Failure Recovery Strategies

When quality gates fail, specific recovery strategies guide iteration back to success.

### SW1 Failure Recovery

**Failure Mode 1: Missing Tasks**

**Diagnosis:** Some tasks from prompt not represented in tasks.md

**Recovery:**

1. Identify missing tasks by comparing prompt to tasks.md
2. LLM call to generate missing task names and descriptions
3. Add missing tasks to tasks.md
4. Re-run complexity scoring and decomposition for new tasks
5. Re-validate all quality gates

**Failure Mode 2: Overly Coarse Tasks**

**Diagnosis:** Some atomic tasks have story points > 5

**Recovery:**

1. Identify over-coarse tasks by scanning story point scores
2. LLM call to decompose over-coarse tasks into subtasks
3. Replace over-coarse tasks with decomposed subtasks
4. Re-score new subtasks to ensure ≤ 5 points
5. Re-validate all quality gates

**Failure Mode 3: Poor Task Naming**

**Diagnosis:** < 95% of tasks don't follow verb-noun pattern

**Recovery:**

1. Identify poorly named tasks via regex analysis
2. LLM call to rename tasks following verb-noun pattern
3. Replace poorly named tasks with improved names
4. Verify new names follow pattern
5. Re-validate all quality gates

**Failure Mode 4: Deep Hierarchy**

**Diagnosis:** Maximum nesting depth > 2 levels

**Recovery:**

1. Identify deepest tasks (max depth > 2)
2. LLM call to flatten hierarchy by decomposing deep tasks
3. Replace deep tasks with flattened structure
4. Verify new depth ≤ 2 levels
5. Re-validate all quality gates

**Failure Mode 5: Vague Task Descriptions**

**Diagnosis:** < 90% of tasks have vague descriptions

**Recovery:**

1. Identify vague tasks (subjective, generic, unclear)
2. LLM call to clarify descriptions with specific details
3. Replace vague descriptions with improved descriptions
4. Verify descriptions are clear and specific
5. Re-validate all quality gates

### SW2 Failure Recovery

**Failure Mode 1: Missing Desired States**

**Diagnosis:** Some tasks lack desired state criteria

**Recovery:**

1. Identify tasks without desired states
2. LLM call to generate desired states for missing tasks
3. Add missing desired states to outputs.md
4. Re-validate all quality gates

**Failure Mode 2: Untestable Criteria**

**Diagnosis:** < 90% of criteria not testable

**Recovery:**

1. Identify untestable criteria (subjective, unobservable)
2. LLM call to rewrite criteria to be testable
3. Replace untestable criteria with improved versions
4. Verify criteria are testable
5. Re-validate all quality gates

**Failure Mode 3: Ambiguous Criteria**

**Diagnosis:** < 85% of criteria ambiguous

**Recovery:**

1. Identify ambiguous criteria (unclear conditions)
2. LLM call to rewrite criteria to be unambiguous
3. Replace ambiguous criteria with improved versions
4. Verify criteria are unambiguous
5. Re-validate all quality gates

**Failure Mode 4: Criteria Count Imbalance**

**Diagnosis:** < 1.5 or > 3.0 criteria per task

**Recovery:**

1. If under-constrained (< 1.5 criteria per task):
   - Identify tasks with too few criteria
   - LLM call to add criteria to under-constrained tasks
   - Add criteria to meet 1.5 minimum
2. If over-constrained (> 3.0 criteria per task):
   - Identify tasks with too many criteria
   - LLM call to remove redundant criteria from over-constrained tasks
   - Remove criteria to meet 3.0 maximum
3. Re-validate all quality gates

**Failure Mode 5: GWT Syntax Errors**

**Diagnosis:** Some criteria have GWT syntax errors

**Recovery:**

1. Identify criteria with syntax errors (parse in GWT evaluator)
2. LLM call to fix syntax errors in failing criteria
3. Replace broken criteria with fixed versions
4. Verify all criteria parse correctly
5. Re-validate all quality gates

### SW3 Failure Recovery

**Failure Mode 1: Missing Categories**

**Diagnosis:** Some tasks lack category assignments

**Recovery:**

1. Identify tasks without category assignments
2. LLM call to assign categories to missing tasks
3. Add missing category assignments to categories.md
4. Re-validate all quality gates

**Failure Mode 2: Invalid Categories**

**Diagnosis:** Some categories not from canonical set

**Recovery:**

1. Identify tasks with invalid categories (not from canonical set)
2. LLM call to reassign categories using canonical set
3. Replace invalid categories with canonical categories
4. Verify all categories from canonical set
5. Re-validate all quality gates

**Failure Mode 3: Multiple Categories Per Task**

**Diagnosis:** Some tasks have multiple categories

**Recovery:**

1. Identify tasks with multiple categories
2. LLM call to select single best category for each task
3. Replace multiple-category assignments with single-category assignments
4. Verify one category per task
5. Re-validate all quality gates

**Failure Mode 4: Missing Reasoning**

**Diagnosis:** Some assignments lack reasoning explanations

**Recovery:**

1. Identify assignments without reasoning
2. LLM call to add reasoning explanations
3. Add reasoning to assignments lacking explanations
4. Verify all assignments have reasoning
5. Re-validate all quality gates

**Failure Mode 5: Inconsistent Categorization**

**Diagnosis:** < 90% of similar tasks have different categories

**Recovery:**

1. Identify groups of similar tasks with different categories
2. LLM call to harmonize categories within each group
3. Replace inconsistent categories with harmonized versions
4. Verify similar tasks have same categories
5. Re-validate all quality gates

**Failure Mode 6: Poor Category Diversity**

**Diagnosis:** Category diversity < 0.3 (all tasks same category)

**Recovery:**

1. Identify dominant category (> 80% of tasks)
2. LLM call to rebalance categories (diversify assignments)
3. Reassign some tasks from dominant category to other categories
4. Verify category diversity ≥ 0.3
5. Re-validate all quality gates

### SW4 Failure Recovery

**Failure Mode 1: Missing Skeletons**

**Diagnosis:** Some tasks lack YAML skeleton structures

**Recovery:**

1. Identify tasks without skeletons
2. LLM call to generate skeletons for missing tasks
3. Add missing skeletons to structs.md
4. Re-validate all quality gates

**Failure Mode 2: YAML Syntax Errors**

**Diagnosis:** Some skeletons have YAML syntax errors

**Recovery:**

1. Identify skeletons with syntax errors (parse with serde_yaml)
2. LLM call to fix syntax errors in failing skeletons
3. Replace broken skeletons with fixed versions
4. Verify all skeletons parse correctly
5. Re-validate all quality gates

**Failure Mode 3: Schema Mismatches**

**Diagnosis:** Some skeletons don't follow unified schema structure

**Recovery:**

1. Identify skeletons with schema mismatches
2. LLM call to fix schema mismatches (reference schema line numbers)
3. Replace mismatched skeletons with corrected versions
4. Verify all skeletons follow schema
5. Re-validate all quality gates

**Failure Mode 4: Wrong Model References**

**Diagnosis:** Some skeletons don't use `${models.primary-analyzer}`

**Recovery:**

1. Identify skeletons with wrong model references
2. LLM call to fix model references (use `${models.primary-analyzer}`)
3. Replace wrong references with correct references
4. Verify all skeletons use correct model reference
5. Re-validate all quality gates

**Failure Mode 5: Missing Hooks**

**Diagnosis:** Average < 5 triggers per skeleton

**Recovery:**

1. Identify skeletons with insufficient hooks (< 5 triggers)
2. LLM call to add hooks based on category
3. Add appropriate hooks to under-hooked skeletons
4. Verify average ≥ 5 triggers per skeleton
5. Re-validate all quality gates

**Failure Mode 6: Non-YAML Blocks**

**Diagnosis:** Non-YAML code blocks present (rust, python, bash)

**Recovery:**

1. Identify non-YAML code blocks
2. LLM call to remove non-YAML blocks
3. Remove offending code blocks from structs.md
4. Verify only YAML blocks present
5. Re-validate all quality gates

### SW5 Failure Recovery

**Failure Mode 1: Schema Validation Fails**

**Diagnosis:** schema_valid=false or unknown keys present

**Recovery:**

1. Identify schema violations (unknown keys, invalid structures)
2. LLM call to fix schema violations (reference schema line numbers)
3. Replace violating sections with corrected versions
4. Re-run schema validation
5. Re-validate all quality gates

**Failure Mode 2: Missing Tasks in Workflow**

**Diagnosis:** Some tasks from tasks.md missing from workflow.yml

**Recovery:**

1. Identify missing tasks by comparing tasks.md to workflow.yml
2. LLM call to generate steps for missing tasks
3. Add missing steps to workflow.yml
4. Re-run exhaustive review loop
5. Re-validate all quality gates

**Failure Mode 3: YAML Parse Errors**

**Diagnosis:** Workflow doesn't parse (serde_yaml error)

**Recovery:**

1. Identify parse error locations (line numbers, error messages)
2. LLM call to fix parse errors (fix indentation, broken syntax)
3. Replace broken YAML with fixed version
4. Re-run YAML parsing validation
5. Re-validate all quality gates

**Failure Mode 4: Broken Dependencies**

**Diagnosis:** Some requires: references don't exist

**Recovery:**

1. Identify broken dependencies (requires: points to non-existent steps)
2. LLM call to fix broken dependencies
3. Fix or remove invalid requires: references
4. Re-run exhaustive review loop
5. Re-validate all quality gates

**Failure Mode 5: Missing Hooks**

**Diagnosis:** Average < 5 triggers per step

**Recovery:**

1. Identify steps with insufficient hooks (< 5 triggers)
2. LLM call to add hooks based on step category
3. Add appropriate hooks to under-hooked steps
4. Verify average ≥ 5 triggers per step
5. Re-validate all quality gates

**Failure Mode 6: Broken Template References**

**Diagnosis:** Some {{...}} references fail to resolve

**Recovery:**

1. Identify broken template references
2. LLM call to fix broken references (use correct variable names)
3. Fix or remove invalid references
4. Re-run template interpolation validation
5. Re-validate all quality gates

**Failure Mode 7: Execution Fails**

**Diagnosis:** Workflow panics or crashes on live system

**Recovery:**

1. Analyze execution logs for specific failure point
2. LLM call to fix execution issues (fix problematic steps)
3. Replace broken sections with corrected versions
4. Re-run workflow on live system
5. Re-validate all quality gates

---

**Document Status:** Draft  
**Last Updated:** 2026-06-17  
**Author:** Sisyphus-Junior (Whitt Execution Engine Planning)  
**Review Status:** Ready for Execution