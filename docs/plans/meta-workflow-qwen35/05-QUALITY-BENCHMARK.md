# 05 — Quality Benchmark

> **Status:** Planning Phase — Documentation Only
> **Phase:** Meta-Workflow Generator Development (Qwen 3.5-9B)
> **Focus:** Baseline dataset strategy, scoring rubric, evaluation criteria checklist

## Baseline Dataset Strategy

The quality benchmarking methodology relies on a carefully curated baseline dataset that represents the output quality achievable by OpenCode's Sisyphus agent using full capabilities (read, write, shell, browser, etc.) in a single-shot generation attempt. This baseline represents the current state-of-the-art in agentic workflow generation and serves as the comparison point for the Qwen 3.5-9B meta-workflow generator.

### Baseline Creation Process Overview

**Step 1: Prompt Selection**

The baseline dataset must include prompts that represent a realistic distribution of complexity encountered in real OpenCode sessions. Prompts are extracted from actual OpenCode sessions using the session tools.

**Session Sources:**

- ses_13d246579ffeoYfYtn38hAEldq — High complexity multi-stage task
- ses_17a245a9cffeWo9uVFAyuF6C9I — Medium complexity data transformation task
- ses_18911fba3ffeCV0aETu2P7em8Q — Low complexity file operations task
- ses_21eda916dffexLBSamby9C941e — Medium complexity debugging workflow
- ses_252ddda20ffeFXdMZYwVt5gbWu — High complexity system configuration task

**Prompt Extraction Protocol:**

1. Use `session_read(session_id="...", include_transcript=true)` to load session message history
2. Identify user messages containing substantive task descriptions (500+ characters, multiple action verbs)
3. Filter for messages that represent complete, standalone objectives (not clarification questions)
4. Extract prompt text while preserving technical specificity
5. Clean only spelling/grammar (NO modification of technical content or meaning)

**Prompt Storage:**

Store extracted prompts as markdown files in `docs/plans/meta-workflow-qwen35/dataset/prompts/`:
```
docs/plans/meta-workflow-qwen35/dataset/prompts/
├── prompt-001-medium-file-operations.md
├── prompt-002-high-data-transformation.md
├── prompt-003-low-simple-validation.md
├── prompt-004-medium-debugging-workflow.md
├── prompt-005-high-system-configuration.md
...
```

Each prompt file contains:
```markdown
# Prompt <number>: <brief description>

**Session Source:** <session_id>
**Extraction Date:** <YYYY-MM-DD>
**Complexity:** low | medium | high

**Prompt Text:**

<prompt_text_here>
```

**Step 2: Baseline Generation via Sisyphus Agent**

Baseline artifacts are created by invoking OpenCode's Sisyphus agent with full capabilities to generate complete workflow artifacts in a single shot, without the five-stage decomposition pipeline.

**Sisyphus Invocation Protocol:**

```python
# Pseudo-code for baseline generation
session = load_session(session_id="<source_session_id>")
prompt = extract_user_prompt(session)

baseline_result = delegate(
    agent="Sisyphus",
    prompt=f"""
    Generate a complete workflow YAML for the following prompt. Produce all stages in one shot:

    {prompt}
    
    Produce the following outputs, each in a separate markdown file:
    1. Task breakdown (hierarchical task tree with complexity scores)
    2. Desired output states (testable end-state criteria per task)
    3. Category assignments (behavior category per task from canonical set)
    4. YAML skeleton structures (step skeletons for each task)
    5. Final workflow YAML (complete executable workflow)
    
    For each stage, provide complete output — no placeholders or TBD sections.
    """
)
```

**Step 3: Baseline Artifact Extraction**

After Sisyphus completes generation, extract baseline artifacts to appropriate locations:

```python
# Pseudo-code for artifact extraction
for stage, artifact in baseline_result.artifacts.items():
    artifact_path = f"docs/plans/meta-workflow-qwen35/dataset/baseline-opencode/{prompt_id}/{stage}.md"
    write_file(artifact_path, artifact.content)
```

**Baseline Storage Structure:**

```
docs/plans/meta-workflow-qwen35/dataset/baseline-opencode/
├── prompt-001-medium-file-operations/
│   ├── baseline-sw1-tasks.md
│   ├── baseline-sw2-outputs.md
│   ├── baseline-sw3-categories.md
│   ├── baseline-sw4-structs.md
│   └── baseline-sw5-workflow.yml
├── prompt-002-high-data-transformation/
│   ├── ...
└── prompt-003-low-simple-validation/
    └── ...
```

**Step 4: Baseline Quality Assessment**

Before using baselines for comparison, assess their quality to ensure they represent reasonable floor expectations:

**Quality Assessment Dimensions:**

1. **Schema Validity** — Does baseline-sw5-workflow.yml pass schema validation?
2. **Input Coverage** — Does baseline cover all tasks from original prompt?
3. **Task Granularity** — Are atomic tasks ≤ 5 story points?
4. **Hook Presence** — Does baseline have adequate hook coverage?
5. **GWT Validity** — Do GWT expressions parse without errors?
6. **Action Variety** — Does baseline use multiple action types?
7. **Template Interpolation** — Does baseline use {{...}} references correctly?

**Scoring Baselines:**

For each dimension, score baseline from 0-10:
- **10**: Excellent (exceeds expectation)
- **8-9**: Good (meets expectation with minor issues)
- **6-7**: Acceptable (meets expectation with notable issues)
- **4-5**: Marginal (barely acceptable, significant issues)
- **0-3**: Poor (unacceptable)

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

Only use baselines with overall score ≥ 60/100 (acceptable quality). If baseline score < 60:
1. Regenerate baseline with more explicit prompt to Sisyphus
2. If 2 regeneration attempts fail, discard prompt from baseline dataset
3. Focus on prompts that Sisyphus can handle reliably

### Baseline Completeness Checklist

Before using a baseline for comparison, verify:

- [ ] All 5 baseline files present for prompt (sw1-sw5)
- [ ] baseline-sw5-workflow.yml is valid YAML (parses without errors)
- [ ] baseline-sw5-workflow.yml passes schema validation (schema_valid=true)
- [ ] baseline-sw1-tasks.md covers all tasks from prompt
- [ ] baseline-sw2-outputs.md has desired state for every task
- [ ] baseline-sw3-categories.md has category for every task
- [ ] baseline-sw4-structs.md has skeleton for every task
- [ ] Baseline overall score ≥ 60/100
- [ ] Baseline metadata file created (see below)

**Baseline Metadata File:**

Create `docs/plans/meta-workflow-qwen35/dataset/baseline-opencode/<prompt_id>/baseline-metadata.yml`:

```yaml
prompt_id: prompt-001
session_source: ses_13d246579ffeoYfYtn38hAEldq
extraction_date: 2026-06-17
complexity: medium

quality_assessment:
  schema_validity: 8
  input_coverage: 9
  task_granularity: 7
  hook_presence: 6
  gwt_validity: 7
  action_variety: 4
  template_interpolation: 5
  overall_score: 69/100

baseline_files:
  sw1: baseline-sw1-tasks.md
  sw2: baseline-sw2-outputs.md
  sw3: baseline-sw3-categories.md
  sw4: baseline-sw4-structs.md
  sw5: baseline-sw5-workflow.yml
```

## Comparison Methodology

The comparison methodology defines how generated outputs are compared against baseline outputs to determine if quality surpasses baseline. This involves both quantitative metrics and qualitative assessment.

### Quantitative Comparison Process

**Step 1: Extract Metrics from Generated Output**

For each sub-workflow iteration output, extract relevant metrics using automated scripts:

```bash
# Extract metrics for SW1
./scripts/extract-sw1-metrics.sh --input ./workspace/sw1-iteration-001/tasks.md --output ./metrics/sw1-iter001.json

# Extract metrics for SW2
./scripts/extract-sw2-metrics.sh --input ./workspace/sw2-iteration-001/outputs.md --output ./metrics/sw2-iter001.json

# Extract metrics for SW3
./scripts/extract-sw3-metrics.sh --input ./workspace/sw3-iteration-001/categories.md --output ./metrics/sw3-iter001.json

# Extract metrics for SW4
./scripts/extract-sw4-metrics.sh --input ./workspace/sw4-iteration-001/structs.md --output ./metrics/sw4-iter001.json

# Extract metrics for SW5
./scripts/extract-sw5-metrics.sh --input ./workspace/sw5-iteration-001/workflow.yml --output ./metrics/sw5-iter001.json
```

**Step 2: Extract Metrics from Baseline Output**

Extract the same metrics from baseline artifacts:

```bash
# Extract metrics for SW1 baseline
./scripts/extract-sw1-metrics.sh --input ./dataset/baseline-opencode/prompt-001/baseline-sw1-tasks.md --output ./metrics/baseline-sw1-prompt001.json

# Extract metrics for SW2 baseline
./scripts/extract-sw2-metrics.sh --input ./dataset/baseline-opencode/prompt-001/baseline-sw2-outputs.md --output ./metrics/baseline-sw2-prompt001.json

# Extract metrics for SW3 baseline
./scripts/extract-sw3-metrics.sh --input ./dataset/baseline-opencode/prompt-001/baseline-sw3-categories.md --output ./metrics/baseline-sw3-prompt001.json

# Extract metrics for SW4 baseline
.//scripts/extract-sw4-metrics.sh --input ./dataset/baseline-opencode/prompt-001/baseline-sw4-structs.md --output ./metrics/baseline-sw4-prompt001.json

# Extract metrics for SW5 baseline
./scripts/extract-sw5-metrics.sh --input ./dataset/baseline-opencode/prompt-001/baseline-sw5-workflow.yml --output ./metrics/baseline-sw5-prompt001.json
```

**Step 3: Calculate Scores**

For each sub-workflow, calculate scores using weighted metric formulas defined in 04-ITERATION-PROTOCOL.md:

```python
# SW1 score calculation
sw1_weighted_score = (
    task_coverage_completeness * 0.25 +
    task_granularity_score * 0.25 +
    naming_quality_score * 0.20 +
    hierarchy_depth_score * 0.15 +
    task_description_quality_score * 0.15
)

# SW2 score calculation
sw2_weighted_score = (
    desired_state_coverage * 0.25 +
    criteria_testability_score * 0.25 +
    criteria_unambiguity_score * 0.20 +
    criteria_count_balance * 0.15 +
    gwt_syntax_validity * 0.15
)

# SW3 score calculation
sw3_weighted_score = (
    category_assignment_coverage * 0.20 +
    canonical_set_adherence * 0.20 +
    mutual_exclusivity * 0.20 +
    category_reasoning_presence * 0.15 +
    category_consistency_score * 0.15 +
    category_distribution_balance * 0.10
)

# SW4 score calculation
sw4_weighted_score = (
    skeleton_coverage * 0.20 +
    yaml_syntax_validity * 0.20 +
    schema_compliance * 0.20 +
    model_reference_correctness * 0.15 +
    hook_presence * 0.15 +
    non_yaml_block_absence * 0.10
)

# SW5 score calculation
sw5_weighted_score = (
    schema_validity * 0.20 +
    input_coverage * 0.15 +
    yaml_parseability * 0.15 +
    dependency_correctness * 0.15 +
    hook_completeness * 0.15 +
    template_interpolation_correctness * 0.10 +
    executability * 0.10
)
```

**Step 4: Compare Scores and Calculate Improvement**

For each sub-workflow, calculate improvement:

```python
# Calculate improvement
sw1_improvement = sw1_weighted_score - baseline_sw1_weighted_score
sw2_improvement = sw2_weighted_score - baseline_sw2_weighted_score
sw3_improvement = sw3_weighted_score - baseline_sw3_weighted_score
sw4_improvement = sw4_weighted_score - baseline_sw4_weighted_score
sw5_improvement = sw5_weighted_score - baseline_sw5_weighted_score

# Calculate overall improvement
overall_improvement = (
    sw1_improvement * 0.20 +
    sw2_improvement * 0.20 +
    sw3_improvement * 0.20 +
    sw4_improvement * 0.20 +
    sw5_improvement * 0.20
)
```

**Step 5: Determine Surpass Status**

A generated output "surpasses" baseline when:

1. **Overall improvement ≥ 10%** — Quantitative threshold (10% better overall)
2. **All individual quality gates pass** — Generated output meets all pass conditions
3. **No regression in any metric** — No metric is worse than baseline by > 5%

**Surpass Decision Matrix:**

| Condition | SW1 | SW2 | SW3 | SW4 | SW5 | Overall |
|----------|-----|-----|-----|-----|-----|--------|
| Overall improvement ≥ 10% | YES | YES | YES | YES | YES | YES |
| All quality gates pass | YES | YES | YES | YES | YES | YES |
| No metric regression > 5% | YES | YES | YES | YES | YES | YES |
| **Surpasses** | **YES** | **YES** | **YES** | **YES** | **YES** |

If any condition is NO, the output does NOT surpass baseline.

### Qualitative Comparison Process

Beyond quantitative metrics, qualitative assessment captures nuanced quality differences that numbers may miss.

**Qualitative Assessment Dimensions:**

1. **Task Decomposition Quality** — Are tasks more atomic and well-scoped than baseline?
2. **Criteria Precision** — Are desired state criteria more specific and actionable than baseline?
3. **Category Appropriateness** — Are categories better aligned with task behavior than baseline?
4. **YAML Maintainability** — Is YAML structure cleaner, more readable than baseline?
5. **Hook Integration** — Are hooks more thoughtful, better integrated than baseline?
6. **Error Handling** — Is error handling more robust, more graceful than baseline?

**Qualitative Assessment Process:**

1. **Side-by-Side Comparison** — Open baseline and generated artifacts side-by-side in editor
2. **Expert Review** — Conduct expert review comparing quality across all dimensions
3. **Subjective Scoring** — Score each dimension 0-10 based on expert judgment
4. **Qualitative Report** — Document findings in iteration report

**Expert Review Guidelines:**

- **Task Decomposition**: Focus on atomicity, scoping, and hierarchy flatness
- **Criteria Precision**: Focus on specificity, testability, and actionability
- **Category Appropriateness**: Focus on alignment with task behavior, reasoning quality
- **YAML Maintainability**: Focus on readability, structure, adherence to schema
- **Hook Integration**: Focus on appropriateness, completeness, integration quality
- **Error Handling**: Focus on robustness, specific error messages, graceful degradation

**Qualitative Score Weighting:**

- Task decomposition quality: 25%
- Criteria precision: 20%
- Category appropriateness: 20%
- YAML maintainability: 15%
- Hook integration: 15%
- Error handling: 20%
- Overall qualitative score: Weighted average of dimensions

**Qualitative Surpass Threshold:**

Generated output "surpasses" baseline when qualitative score ≥ 70/100 AND at least 3 dimensions show ≥ 10% improvement over baseline.

### Combined Surpass Determination

Final surpass determination combines quantitative and qualitative assessments:

**Surpass Decision Matrix:**

| Condition | Quantitative | Qualitative | Final Decision |
|----------|--------------|------------|---------------|
| Overall improvement ≥ 10% | YES | YES | **SURPASSES** |
| Overall improvement ≥ 10% | YES | NO | **DOESN'T SURPASS** |
| Overall improvement ≥ 10% | NO | YES | **DOESN'T SURPASS** |
| Overall improvement ≥ 10% | NO | NO | **DOESN'T SURPASS** |
| Overall improvement < 10% | - | - | **DOESN'T SURPASS** |

The quantitative threshold (≥ 10% improvement) is the primary gate. The qualitative assessment provides additional validation but cannot compensate for insufficient quantitative improvement.

## Scoring Rubric

The scoring rubric defines specific scoring criteria for each dimension, enabling consistent assessment across evaluators.

### SW1 Scoring Rubric

**Dimension 1: Task Coverage Completeness (25% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | All tasks from original prompt present in tasks.md | Manual verification matches 100% |
| 8-9 (80-90%) | ≥ 95% of tasks present, minor omissions only | Manual verification shows 1-2 missing tasks |
| 6-7 (60-70%) | ≥ 80% of tasks present, notable omissions | Manual verification shows 3-5 missing tasks |
| 4-5 (40-50%) | ≥ 60% of tasks present, significant omissions | Manual verification shows 6-10 missing tasks |
| 0-3 (0-30%) | < 60% of tasks present, major omissions | Manual verification shows > 10 missing tasks |

**Dimension 2: Task Granularity (25% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | All atomic tasks ≤ 5 story points, perfect granularity | Parse tasks.md, max story points = 5, all atomic |
| 8-9 (80-90%) | ≥ 95% of atomic tasks ≤ 5 points, minor granularity issues | Parse tasks.md, max story points = 6, 95% atomic |
| 6-7 (60-70%) | ≥ 80% of atomic tasks ≤ 5 points, some granularity issues | Parse tasks.md, max story points = 8, 80% atomic |
| 4-5 (40-50%) | ≥ 60% of atomic tasks ≤ 5 points, coarse granularity | Parse tasks.md, max story points = 10, 60% atomic |
| 0-3 (0-30%) | < 60% of atomic tasks ≤ 5 points, overly coarse | Parse tasks.md, max story points = 13+, < 60% atomic |

**Dimension 3: Naming Quality (20% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | ≥ 95% of tasks follow verb-noun pattern | Regex analysis: 95%+ match pattern |
| 8-9 (80-90%) | ≥ 90% of tasks follow verb-noun pattern, minor naming issues | Regex analysis: 90-94% match pattern |
| 6-7 (60-70%) | ≥ 80% of tasks follow verb-noun pattern, notable naming issues | Regex analysis: 80-84% match pattern |
| 4-5 (40-50%) | ≥ 60% of tasks follow verb-noun pattern, poor naming quality | Regex analysis: 60-64% match pattern |
| 0-3 (0-30%) | < 60% of tasks follow verb-noun pattern, very poor naming | Regex analysis: < 60% match pattern |

**Regex Pattern:** `^[A-Za-z]+ [A-Za-z0-9_\-]*$` (verb-noun pattern with optional modifiers)

**Dimension 4: Hierarchy Depth (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | Maximum nesting depth ≤ 2 levels (flat hierarchy) | Parse tasks.md, max depth = 1 or 2 |
| 8-9 (80-90%) | Maximum nesting depth = 3 levels (slightly deep) | Parse tasks.md, max depth = 3 |
| 6-7 (60-70%) | Maximum nesting depth = 4 levels (moderately deep) | Parse tasks.md, max depth = 4 |
| 4-5 (40-50%) | Maximum nesting depth ≥ 5 levels (very deep) | Parse tasks.md, max depth = 5+ |
| 0-3 (0-30%) | Maximum nesting depth ≥ 6 levels (extremely deep) | Parse tasks.md, max depth = 6+ |

**Dimension 5: Task Description Quality (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | ≥ 90% of tasks have clear, specific descriptions | Manual review: 90%+ descriptions are clear, specific |
| 8-9 (80-90%) | ≥ 85% of tasks have clear, specific descriptions | Manual review: 85-89% descriptions are clear, specific |
| 6-7 (60-70%) | ≥ 80% of tasks have clear, specific descriptions | Manual review: 80-84% descriptions are clear, specific |
| 4-5 (40-50%) | ≥ 70% of tasks have clear, specific descriptions | Manual review: 70-74% descriptions are clear, specific |
| 0-3 (0-30%) | < 70% of tasks have clear, specific descriptions | Manual review: < 70% descriptions are clear, specific |

**Description Quality Definition:**

- **Clear:** Unambiguous, no vague language ("optimize", "improve" without context)
- **Specific:** Contains concrete details (file paths, tool names, action verbs)
- **Actionable:** Describes what to do, not just what is (uses imperative mood)

**SW1 Overall Score Calculation:**

```
SW1 Score = (Task Coverage Completeness * 0.25) +
           (Task Granularity Score * 0.25) +
           (Naming Quality Score * 0.20) +
           (Hierarchy Depth Score * 0.15) +
           (Task Description Quality Score * 0.15)

SW1 Maximum Score: 100.0
SW1 Minimum Passing Score: 80.0
```

### SW2 Scoring Rubric

**Dimension 1: Desired State Coverage (25% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of tasks have corresponding desired state criteria | Count criteria sections vs task count, must match exactly |
| 8-9 (80-90%) | ≥ 95% of tasks have desired state criteria | Count criteria sections vs task count, 1-5% gaps allowed |
| 6-7 (60-70%) | ≥ 80% of tasks have desired state criteria | Count criteria sections vs task count, 5-20% gaps allowed |
| 4-5 (40-50%) | ≥ 60% of tasks have desired state criteria | Count criteria sections vs task count, 20-40% gaps allowed |
| 0-3 (0-30%) | < 60% of tasks have desired state criteria | Count criteria sections vs task count, >40% gaps allowed |

**Dimension 2: Criteria Testability (25% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | ≥ 90% of criteria are testable (observable, measurable) | Manual review: 90%+ criteria testable |
| 8-9 (80-90%) | ≥ 80% of criteria are testable | Manual review: 80-89% criteria testable |
| 6-7 (60-70%) | ≥ 70% of criteria are testable | Manual review: 70-79% criteria testable |
| 4-5 (40-50%) | ≥ 60% of criteria are testable | Manual review: 60-69% criteria testable |
| 0-3 (0-30%) | < 60% of criteria are testable | Manual review: < 60% criteria testable |

**Testability Definition:**

- **Observable**: Can be verified via file existence, process state, or output content
- **Measurable**: Has clear pass/fail condition (e.g., "file exists", "process running", "output contains X")
- **Verifiable**: Can be checked via shell command or tool output

**Dimension 3: Criteria Unambiguity (20% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | ≥ 85% of criteria are unambiguous (clear pass/fail) | Manual review: 85%+ criteria unambiguous |
| 8-9 (80-90%) | ≥ 80% of criteria are unambiguous | Manual review: 80-84% criteria unambiguous |
| 6-7 (60-70%) | ≥ 70% of criteria are unambiguous | Manual review: 70-79% criteria unambiguous |
| 4-5 (40-50%) | ≥ 60% of criteria are unambiguous | Manual review: 60-69% criteria unambiguous |
| 0-3 (0-30%) | < 60% of criteria are unambiguous | Manual review: < 60% criteria unambiguous |

**Unambiguity Definition:**

- **Clear pass/fail**: No subjective judgments, objective criteria
- **No vague language**: "good", "adequate", "reasonable" prohibited
- **Concrete checks**: Specific values (file paths, process names, output content)

**Dimension 4: Criteria Count Balance (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 1.5-3.0 criteria per task (perfect balance) | Calculate avg criteria per task, must be in range |
| 8-9 (80-90%) | 1.2-3.5 criteria per task (slightly imbalanced) | Calculate avg criteria per task, minor deviations allowed |
| 6-7 (60-70%) | 1.0-4.0 criteria per task (somewhat imbalanced) | Calculate avg criteria per task, moderate deviations allowed |
| 4-5 (40-50%) | 0.5-5.0 criteria per task (significantly imbalanced) | Calculate avg criteria per task, major deviations allowed |
| 0-3 (0-30%) | < 0.5 or > 5.0 criteria per task (extremely imbalanced) | Calculate avg criteria per task, extreme deviations |

**Balance Rationale:**

- **1.5-3.0 criteria per task** ensures each task has adequate validation without over-constraint
- **Under-constrained (< 1.5 criteria)**: Risk of incomplete validation, ambiguous completion
- **Over-constrained (> 3.0 criteria)**: Risk of validation paralysis, excessive iteration

**Dimension 5: GWT Syntax Validity (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of criteria parse in GWT evaluator without errors | Parse all criteria with GWT evaluator, 100% succeed |
| 8-9 (80-90%) | ≥ 95% of criteria parse without GWT errors | Parse all criteria with GWT evaluator, 1-5% parse errors allowed |
| 6-7 (60-70%) | ≥ 90% of criteria parse without GWT errors | Parse all criteria with GWT evaluator, 1-10% parse errors allowed |
| 4-5 (40-50%) | ≥ 80% of criteria parse without GWT errors | Parse all criteria with GWT evaluator, 1-20% parse errors allowed |
| 0-3 (0-30%) | < 80% of criteria parse without GWT errors | Parse all criteria with GWT evaluator, >20% parse errors |

**GWT Evaluator Location:**

GWT evaluator implementation at `src/workflow/hooks/gwt.rs` (lines 1-1805). Parse all criteria in evaluator and check for parse errors.

**SW2 Overall Score Calculation:**

```
SW2 Score = (Desired State Coverage * 0.25) +
           (Criteria Testability Score * 0.25) +
           (Criteria Unambiguity Score * 0.20) +
           (Criteria Count Balance Score * 0.15) +
           (GWT Syntax Validity Score * 0.15)

SW2 Maximum Score: 100.0
SW2 Minimum Passing Score: 80.0
```

### SW3 Scoring Rubric

**Dimension 1: Category Assignment Coverage (20% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of tasks have category assignments | Count category assignments vs task count, must match exactly |
| 8-9 (80-90%) | ≥ 95% of tasks have category assignments | Count category assignments vs task count, 1-5% gaps allowed |
| 6-7 (60-70%) | ≥ 80% of tasks have category assignments | Count category assignments vs task count, 5-20% gaps allowed |
| 4-5 (40-50%) | ≥ 60% of tasks have category assignments | Count category assignments vs task count, 20-40% gaps allowed |
| 0-3 (0-30%) | < 60% of tasks have category assignments | Count category assignments vs task count, >40% gaps allowed |

**Dimension 2: Canonical Set Adherence (20% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of categories from canonical set (10 categories: file-read, transform-llm, validate-gate, loop-iterate, branch-decision, shell-execute, tool-call, checkpoint-state, notify-external, sub-workflow-ref) | Compare each category against canonical set, all match |
| 8-9 (80-90%) | ≥ 95% of categories from canonical set | Compare each category against canonical set, 1-2 invented categories allowed |
| 6-7 (60-70%) | ≥ 80% of categories from canonical set | Compare each category against canonical set, 2-4 invented categories allowed |
| 4-5 (40-50%) | ≥ 60% of categories from canonical set | Compare each category against canonical set, 5-10 invented categories allowed |
| 0-3 (0-30%) | < 60% of categories from canonical set | Compare each category against canonical set, >10 invented categories allowed |

**Dimension 3: Mutual Exclusivity (20% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of tasks have exactly one category | Verify each task has exactly one category label |
| 8-9 (80-90%) | ≥ 95% of tasks have exactly one category | Verify each task has exactly one category, minor violations allowed |
| 6-7 (60-70%) | ≥ 80% of tasks have exactly one category | Verify each task has exactly one category, some multi-category tasks allowed |
| 4-5 (40-50%) | ≥ 60% of tasks have exactly one category | Verify each task has exactly one category, many multi-category tasks allowed |
| 0-3 (0-30%) | < 60% of tasks have exactly one category | Verify each task has exactly one category, most tasks multi-category |

**Dimension 4: Category Reasoning Presence (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of category assignments have reasoning explanations | Check each category assignment has reasoning section |
| 8-9 (80-90%) | ≥ 95% of category assignments have reasoning | Check each category assignment, 1-5% missing reasoning |
| 6-7 (60-70%) | ≥ 80% of category assignments have reasoning | Check each category assignment, 5-20% missing reasoning |
| 4-5 (40-50%) | ≥ 60% of category assignments have reasoning | Check each category assignment, 20-40% missing reasoning |
| 0-3 (0-30%) | < 60% of category assignments have reasoning | Check each category assignment, >40% missing reasoning |

**Reasoning Quality Definition:**

- **Present**: Reasoning section exists for category assignment
- **Explains WHY**: Reasoning explains why category fits (references task characteristics)
- **Specific**: References specific task features (keywords, action verbs, output type)

**Dimension 5: Category Consistency (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | ≥ 90% of similar tasks have same category assignment | Identify groups of similar tasks (by description complexity), verify same category across group |
| 8-9 (80-90%) | ≥ 80% of similar tasks have same category assignment | Identify groups of similar tasks, verify same category across most |
| 6-7 (60-70%) | ≥ 70% of similar tasks have same category assignment | Identify groups of similar tasks, verify same category across some |
| 4-5 (40-50%) | ≥ 60% of similar tasks have same category assignment | Identify groups of similar tasks, verify same category across few |
| 0-3 (0-30%) | < 60% of similar tasks have same category assignment | Identify groups of similar tasks, verify same category across few |

**Similar Task Identification:**

Tasks are similar when they:
- Have comparable complexity scores (± 2 story points)
- Use similar action verbs (read, write, transform, validate)
- Have comparable output types (file content, structured data, boolean decision)
- Belong to same complexity category (low, medium, high)

**Dimension 6: Category Distribution Balance (10% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | Category diversity ≥ 0.5 (≥ 50% of tasks have different categories) | Calculate diversity = unique categories / total tasks, must be ≥ 0.5 |
| 8-9 (80-90%) | Category diversity ≥ 0.4 (≥ 40% of tasks have different categories) | Calculate diversity = unique categories / total tasks, must be ≥ 0.4 |
| 6-7 (60-70%) | Category diversity ≥ 0.3 (≥ 30% of tasks have different categories) | Calculate diversity = unique categories / total tasks, must be ≥ 0.3 |
| 4-5 (40-50%) | Category diversity ≥ 0.2 (≥ 20% of tasks have different categories) | Calculate diversity = unique categories / total tasks, must be ≥ 0.2 |
| 0-3 (0-30%) | Category diversity < 0.2 (all tasks same category) | Calculate diversity = unique categories / total tasks, < 0.2 |

**Distribution Rationale:**

- **High diversity (≥ 0.5)**: Workflow uses diverse categories, robust task handling
- **Low diversity (< 0.3)**: All tasks same category, suggests poor categorization
- **Balance**: Not all tasks need unique categories, but diversity should be reasonable

**SW3 Overall Score Calculation:**

```
SW3 Score = (Category Assignment Coverage * 0.20) +
           (Canonical Set Adherence * 0.20) +
           (Mutual Exclusivity * 0.20) +
           (Category Reasoning Presence * 0.15) +
           (Category Consistency Score * 0.15) +
           (Category Distribution Balance * 0.10)

SW3 Maximum Score: 100.0
SW3 Minimum Passing Score: 80.0
```

### SW4 Scoring Rubric

**Dimension 1: Skeleton Coverage (20% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of tasks have corresponding YAML skeleton structures | Count skeletons in structs.md vs tasks in tasks.md, must match exactly |
| 8-9 (80-90%) | ≥ 95% of tasks have YAML skeleton structures | Count skeletons in structs.md vs tasks in tasks.md, 1-5% gaps allowed |
| 6-7 (60-70%) | ≥ 80% of tasks have YAML skeleton structures | Count skeletons in structs.md vs tasks in tasks.md, 5-20% gaps allowed |
| 4-5 (40-50%) | ≥ 60% of tasks have YAML skeleton structures | Count skeletons in structs.md vs tasks in tasks.md, 20-40% gaps allowed |
| 0-3 (0-30%) | < 60% of tasks have YAML skeleton structures | Count skeletons in structs.md vs tasks in tasks.md, >40% gaps allowed |

**Dimension 2: YAML Syntax Validity (20% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of skeletons parse without serde_yaml errors | Parse each YAML block with serde_yaml, 100% succeed |
| 8-9 (80-90%) | ≥ 95% of skeletons parse without serde_yaml errors | Parse each YAML block with serde_yaml, 1-5% parse errors allowed |
| 6-7 (60-70%) | ≥ 80% of skeletons parse without serde_yaml errors | Parse each YAML block with serde_yaml, 1-10% parse errors allowed |
| 4-5 (40-50%) | ≥ 60% of skeletons parse without serde_yaml errors | Parse each YAML block with serde_yaml, 1-20% parse errors allowed |
| 0-3 (0-30%) | < 60% of skeletons parse without serde_yaml errors | Parse each YAML block with serde_yaml, >20% parse errors allowed |

**Parse Error Types to Monitor:**

- Indentation errors (tabs vs spaces)
- Invalid structure (incorrect nesting)
- Missing required keys (generative_entity, prompt)
- Duplicate keys
- Invalid escape sequences
- Trailing commas

**Dimension 3: Schema Compliance (20% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of skeletons follow unified schema structure | Compare each skeleton structure against unified schema lines 318-367, 100% match |
| 8-9 (80-90%) | ≥ 95% of skeletons follow unified schema structure | Compare each skeleton structure against unified schema lines 318-367, 1-5% mismatches allowed |
| 6-7 (60-70%) | ≥ 80% of skeletons follow unified schema structure | Compare each skeleton structure against unified schema lines 318-367, 5-20% mismatches allowed |
| 4-5 (40-50%) | ≥ 60% of skeletons follow unified schema structure | Compare each skeleton structure against unified schema lines 318-367, 20-40% mismatches allowed |
| 0-3 (0-30%) | < 60% of skeletons follow unified schema structure | Compare each skeleton structure against unified schema lines 318-367, >40% mismatches allowed |

**Schema Structure Requirements:**

- Follows `agentic_workflow.steps.<step_name>:` shape
- Required fields present: generative_entity, prompt, when (optional but recommended)
- Hooks configured under when key with proper trigger names
- No unknown top-level keys (only keys from unified schema allowed)

**Dimension 4: Model Reference Correctness (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of skeletons use `${models.primary-analyzer}` | Search skeletons for pattern `${models.primary-analyzer}`, 100% match |
| 8-9 (80-90%) | ≥ 95% of skeletons use `${models.primary-analyzer}` | Search skeletons for pattern `${models.primary-analyzer}`, 1-5% use wrong reference |
| 6-7 (60-70%) | ≥ 80% of skeletons use `${models.primary-analyzer}` | Search skeletons for pattern `${models.primary-analyzer}`, 5-20% use wrong reference |
| 4-5 (40-50%) | ≥ 60% of skeletons use `${models.primary-analyzer}` | Search skeletons for pattern `${models.primary-analyzer}`, 20-40% use wrong reference |
| 0-3 (0-30%) | < 60% of skeletons use `${models.primary-analyzer}` | Search skeletons for pattern `${models.primary-analyzer}`, >40% use wrong reference |

**Wrong Reference Patterns to Avoid:**
- `${models.qwen35}` — Wrong model, should use primary-analyzer
- `${models.qwen-3.5-9b}` — Wrong model name format
- `qwen35` — Missing models prefix
- Direct model name without `${models.}` prefix

**Dimension 5: Hook Presence (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | Average ≥ 5 triggers per skeleton | Count triggers in each skeleton, calculate average, must be ≥ 5 |
| 8-9 (80-90%) | Average ≥ 4 triggers per skeleton | Count triggers in each skeleton, calculate average, must be ≥ 4 |
| 6-7 (60-70%) | Average ≥ 3 triggers per skeleton | Count triggers in each skeleton, calculate average, must be ≥ 3 |
| 4-5 (40-50%) | Average ≥ 2 triggers per skeleton | Count triggers in each skeleton, calculate average, must be ≥ 2 |
| 0-3 (0-30%) | Average < 2 triggers per skeleton | Count triggers in each skeleton, calculate average, < 2 |

**Trigger Types Counted:**

- before_step_starts
- after_step_starts
- after_step_fails
- after_step_succeeds
- after_all_retries_exhausted
- on_requires_failed
- after_loop_iteration_fails

**Dimension 6: Non-YAML Block Absence (10% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of code blocks are YAML only | Check code block markers, only ```yaml markers present |
| 8-9 (80-90%) | ≥ 95% of code blocks are YAML only | Check code block markers, 1-5% non-YAML blocks allowed |
| 6-7 (60-70%) | ≥ 80% of code blocks are YAML only | Check code block markers, 5-20% non-YAML blocks allowed |
| 4-5 (40-50%) | ≥ 60% of code blocks are YAML only | Check code block markers, 20-40% non-YAML blocks allowed |
| 0-3 (0-30%) | < 60% of code blocks are YAML only | Check code block markers, >40% non-YAML blocks allowed |

**Non-YAML Block Types to Avoid:**

- ```rust — Rust code blocks (not allowed)
- ```python — Python code blocks (not allowed)
- ```bash — Bash/shell code blocks (not allowed)
- ```json — JSON data blocks (use save_to action instead)
- ```xml — XML data blocks (use file content instead)

**SW4 Overall Score Calculation:**

```
SW4 Score = (Skeleton Coverage * 0.20) +
           (YAML Syntax Validity Score * 0.20) +
           (Schema Compliance Score * 0.20) +
           (Model Reference Correctness Score * 0.15) +
           (Hook Presence Score * 0.15) +
           (Non-YAML Block Absence Score * 0.10)

SW4 Maximum Score: 100.0
SW4 Minimum Passing Score: 80.0
```

### SW5 Scoring Rubric

**Dimension 1: Schema Validity (20% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | schema_valid=true in logs (no schema violations) | Check logs for `schema_valid=true` flag |
| 8-9 (80-90%) | schema_valid=true in logs, 1-2 warnings (non-critical) | Check logs for `schema_valid=true` flag + warnings |
| 6-7 (60-70%) | schema_valid=true in logs, 3-5 warnings (some issues) | Check logs for `schema_valid=true` flag + warnings |
| 4-5 (40-50%) | schema_valid=false in logs or critical errors | Check logs for `schema_valid=false` flag or errors |
| 0-3 (0-30%) | schema_valid=false in logs or critical errors with panics | Check logs for `schema_valid=false` flag or panics |

**Schema Violations to Check:**

- Unknown top-level keys not in unified schema
- Invalid field types (string where number expected, etc.)
- Improper nesting (workflow nested under wrong section)
- Missing required fields (generative_entity, prompt for steps)
- Invalid hook configurations (unknown trigger, invalid action)

**Dimension 2: Input Coverage (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of tasks from tasks.md appear as steps in workflow.yml | Extract step names from workflow.yml, compare to tasks.md, 100% match |
| 8-9 (80-90%) | ≥ 95% of tasks from tasks.md appear as steps in workflow.yml | Extract step names from workflow.yml, compare to tasks.md, 1-5% gaps allowed |
| 6-7 (60-70%) | ≥ 80% of tasks from tasks.md appear as steps in workflow.yml | Extract step names from workflow.yml, compare to tasks.md, 5-20% gaps allowed |
| 4-5 (40-50%) | ≥ 60% of tasks from tasks.md appear as steps in workflow.yml | Extract step names from workflow.yml, compare to tasks.md, 20-40% gaps allowed |
| 0-3 (0-30%) | < 60% of tasks from tasks.md appear as steps in workflow.yml | Extract step names from workflow.yml, compare to tasks.md, >40% gaps allowed |

**Task Name Matching Rules:**

- Exact match: `task_name` matches `step_name` exactly
- Reasonable variations allowed: Add prefixes/suffixes (e.g., `task_1_generate`, `task_1_generate_v2`)
- Semantically equivalent: `read_config_files` ≈ `read_configuration` (if both refer to same action)

**Dimension 3: YAML Parseability (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | Workflow parses without serde_yaml errors (no syntax errors) | Parse workflow.yml with serde_yaml, succeeds without errors |
| 8-9 (80-90%) | Workflow parses with serde_yaml, 1-2 non-critical errors | Parse workflow.yml with serde_yaml, succeeds with warnings |
| 6-7 (60-70%) | Workflow parses with serde_yaml, 3-5 non-critical errors | Parse workflow.yml with serde_yaml, succeeds with errors |
| 4-5 (40-50%) | Workflow parses with serde_yaml, >5 errors | Parse workflow.yml with serde_yaml, fails or succeeds with many errors |
| 0-3 (0-30%) | Workflow doesn't parse with serde_yaml (critical error) | Parse workflow.yml with serde_yaml, fails with critical error |

**Parse Error Types to Monitor:**

- Indentation errors (tabs vs spaces)
- Invalid structure (incorrect nesting)
- Missing required keys (workflow_id, name, providers, models)
- Duplicate keys (e.g., multiple workflow keys)
- Invalid escape sequences
- Trailing commas
- Unicode issues

**Dimension 4: Dependency Correctness (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of requires: references point to existing steps | Extract all requires: references, verify all exist as steps |
| 8-9 (80-90%) | ≥ 95% of requires: references point to existing steps | Extract all requires: references, verify 1-5% broken references |
| 6-7 (60-70%) | ≥ 80% of requires: references point to existing steps | Extract all requires: references, verify 5-20% broken references |
| 4-5 (40-50%) | ≥ 60% of requires: references point to existing steps | Extract all requires: references, verify 20-40% broken references |
| 0-3 (0-30%) | < 60% of requires: references point to existing steps | Extract all requires: references, verify >40% broken references |

**Dependency Validation Rules:**

- All step names in requires: must exist as steps in agentic_workflow.steps
- Circular dependencies not allowed (A requires B, B requires A)
- Self-references not allowed (step requires itself)
- Forward references only (step can require step with higher execution order)

**Dimension 5: Hook Completeness (15% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | Average ≥ 5 triggers per step | Count triggers in each step, calculate average, must be ≥ 5 |
| 8-9 (80-90%) | Average ≥ 4 triggers per step | Count triggers in each step, calculate average, must be ≥ 4 |
| 6-7 (60-70%) | Average ≥ 3 triggers per step | Count triggers in each step, calculate average, must be ≥ 3 |
| 4-5 (40-50%) | Average ≥ 2 triggers per step | Count triggers in each step, calculate average, must be ≥ 2 |
| 0-3 (0-30%) | Average < 2 triggers per step | Count triggers in each step, calculate average, < 2 |

**Trigger Types Counted:**

Same as SW4 (10 triggers)

**Hook Coverage Validation:**

- Each step must have at least one trigger configured
- Common triggers: after_step_succeeds, after_step_fails
- Category-specific triggers: validate-gate uses gwt, transform-llm uses save_to

**Dimension 6: Template Interpolation Correctness (10% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | 100% of {{...}} references resolve correctly | Check all template references against defined variables, all resolve |
| 8-9 (80-90%) | ≥ 95% of {{...}} references resolve correctly | Check all template references, 1-5% broken references allowed |
| 6-7 (60-70%) | ≥ 80% of {{...}} references resolve correctly | Check all template references, 5-20% broken references allowed |
| 4-5 (40-50%) | ≥ 60% of {{...}} references resolve correctly | Check all template references, 20-40% broken references allowed |
| 0-3 (0-30%) | < 60% of {{...}} references resolve correctly | Check all template references, >40% broken references |

**Template Reference Types:**

- Step outputs: `{{step.step_name.output}}`
- Workflow inputs: `{{inputs.field_name}}`
- Loop variables: `{{loop.iteration_variable}}`
- Timestamp: `{{now}}`
- Workflow ID: `{{workflow_id}}`
- Run number: `{{run.number}}`

**Reference Resolution Rules:**

- All referenced variables must be defined before use
- Template interpolation happens at runtime, not at parse time
- Undefined variables cause runtime errors, not parse errors

**Dimension 7: Executability (10% weight)**

| Score | Criteria | Evidence |
|-------|----------|----------|
| 10 (100%) | Workflow runs on live Docker system without errors | Run workflow, check logs for `[workflow:end]` event, no panics |
| 8-9 (80-90%) | Workflow runs on live Docker system with warnings but completes | Run workflow, check logs for `[workflow:end]` event, warnings in logs |
| 6-7 (60-70%) | Workflow runs on live system with partial execution | Run workflow, check logs for partial completion, warnings in logs |
| 4-5 (40-50%) | Workflow fails on live system (panics or crashes) | Run workflow, check logs for panics or crashes |
| 0-3 (0-30%) | Workflow fails to start on live system (load errors, Docker issues) | Run workflow, check logs for load errors, Docker issues |

**Live System Execution Checks:**

- Model loads within 30-minute timeout
- All steps execute without panics
- Hooks fire at correct trigger points (check log files)
- Output directory created with expected artifacts
- [workflow:end] event appears in logs at completion

**SW5 Overall Score Calculation:**

```
SW5 Score = (Schema Validity Score * 0.20) +
           (Input Coverage Score * 0.15) +
           (YAML Parseability Score * 0.15) +
           (Dependency Correctness Score * 0.15) +
           (Hook Completeness Score * 0.15) +
           (Template Interpolation Correctness Score * 0.10) +
           (Executability Score * 0.10)

SW5 Maximum Score: 100.0
SW5 Minimum Passing Score: 80.0
```

## What "Surpasses" Means Quantitatively

The term "surpasses baseline" has a specific quantitative definition that must be met before a sub-workflow can advance to the next stage.

### Surpass Definition (Quantitative)

**Primary Criterion (Required):**

```
Overall improvement ≥ 10% AND
All quality gates pass AND
No metric regression > 5%
```

**Overall Improvement Calculation:**

```
Overall improvement = (
    (sw1_improvement * 0.20) +
    (sw2_improvement * 0.20) +
    (sw3_improvement * 0.20) +
    (sw4_improvement * 0.20) +
    (sw5_improvement * 0.20)
)
```

Where:
- `swN_improvement = generated_swN_score - baseline_swN_score`
- Each dimension weighted equally (20% each)

**Secondary Criterion (Validation):**

**All quality gates pass** — For the specific sub-workflow, every quality gate defined in the scoring rubric must pass:
- SW1: Task coverage, granularity, naming, hierarchy, description quality
- SW2: Coverage, testability, unambiguity, count balance, GWT syntax
- SW3: Coverage, canonical set, mutual exclusivity, reasoning, consistency, distribution
- SW4: Coverage, syntax validity, schema compliance, model reference, hook presence, non-YAML blocks
- SW5: Schema validity, input coverage, parseability, dependencies, hooks, interpolation, executability

**Tertiary Criterion (Anti-Regression):**

**No metric regression > 5%** — No dimension scores are more than 5 points worse than baseline. This ensures that improvements don't come at the cost of degrading other aspects of quality.

### Surpass Definition (Qualitative)

The qualitative assessment provides additional validation that the generated output represents meaningful improvement over baseline, beyond just meeting numerical thresholds.

**Primary Criterion (Required):**

```
Qualitative score ≥ 70/100 AND
At least 3 dimensions show ≥ 10% improvement over baseline
```

**Qualitative Score Calculation:**

```
Qualitative Score = (
    (Task Decomposition Quality * 0.25) +
    (Criteria Precision * 0.20) +
    (Category Appropriateness * 0.20) +
    (YAML Maintainability * 0.15) +
    (Hook Integration * 0.15) +
    (Error Handling * 0.20)
)
```

**Dimension Improvement Check:**

For each dimension, calculate: `generated_score - baseline_score`. If ≥ 1.0 (10% improvement), mark as improved.

**Overall Surpass Decision:**

Generated output "surpasses" baseline when:
1. Overall improvement ≥ 10% (quantitative threshold)
2. All quality gates pass (quality gates are required for advancing anyway)
3. No metric regression > 5% (no catastrophic failures)
4. Qualitative score ≥ 70/100 AND at least 3 dimensions show ≥ 10% improvement

If conditions 1, 2, 3 are met but 4 is not met, the output "does not surpass" baseline but still advances (quantitative improvement is sufficient, qualitative is supplementary validation).

If conditions 1 is not met, the output "does not surpass" baseline regardless of other conditions (quantitative threshold is mandatory).

### Examples of Surpass vs Not-Surpass

**Example 1: Clear Surpass**

**Baseline:**
- Overall score: 65/100
- Task coverage: 80%
- Task granularity: 6.5 points avg
- Naming quality: 75%
- Hierarchy depth: 3 levels
- Task description quality: 70%

**Generated:**
- Overall score: 82/100 (+17 improvement)
- Task coverage: 100% (+20%)
- Task granularity: 4.2 points avg (-35% improvement)
- Naming quality: 95% (+20% improvement)
- Hierarchy depth: 2 levels (-1 level improvement)
- Task description quality: 88% (+18% improvement)

**Analysis:**
- Overall improvement = 17% ≥ 10% ✓
- All quality gates pass ✓
- No metric regression > 5% (all improvements or minor degradations) ✓
- Qualitative assessment shows 4/6 dimensions show > 10% improvement ✓

**Result:** **SURPASSES**

**Example 2: Not-Surpass (Quantitative Failure)**

**Baseline:**
- Overall score: 65/100
- Task coverage: 80%
- Task granularity: 6.5 points avg
- Naming quality: 75%
- Hierarchy depth: 3 levels
- Task description quality: 70%

**Generated:**
- Overall score: 78/100 (+13 improvement)
- Task coverage: 100% (+20%)
- Task granularity: 7.0 points avg (-7% degradation)
- Naming quality: 95% (+20% improvement)
- Hierarchy depth: 3 levels (same)
- Task description quality: 88% (+18% improvement)

**Analysis:**
- Overall improvement = 13% ≥ 10% ✓
- All quality gates pass ✓
- Metric regression present (task granularity -7%) ✗

**Result:** **DOESN'T SURPASS** (quantitative threshold met, but metric regression present)

**Example 3: Not-Surpass (Qualitative Failure)**

**Baseline:**
- Overall score: 65/100
- Task coverage: 80%
- Task granularity: 6.5 points avg
- Naming quality: 75%
- Hierarchy depth: 3 levels
- Task description quality: 70%

**Generated:**
- Overall score: 82/100 (+17 improvement)
- Task coverage: 100% (+20%)
- Task granularity: 4.2 points avg (-35% improvement)
- Naming quality: 95% (+20% improvement)
- Hierarchy depth: 2 levels (-1 level improvement)
- Task description quality: 88% (+18% improvement)
- Qualitative assessment: 65/100 (only 1/6 dimensions show > 10% improvement)

**Analysis:**
- Overall improvement = 17% ≥ 10% ✓
- All quality gates pass ✓
- No metric regression > 5% ✓
- Qualitative assessment shows only 1/6 dimensions show > 10% improvement ✗

**Result:** **DOESN'T SURPASS** (quantitative threshold met, but qualitative threshold not met)

## Evaluation Criteria Checklist

Comprehensive checklist for verifying that all evaluation criteria have been applied correctly before claiming a sub-workflow output surpasses baseline.

### SW1 Evaluation Checklist

**File Presence:**
- [ ] Output file exists: `workspace/sw1-iteration-<N>/tasks.md`
- [ ] Output file is non-empty (≥ 10 characters)
- [ ] Output file is valid markdown (parseable)

**Task Coverage Verification:**
- [ ] Extracted task names from tasks.md
- [ ] Extracted tasks from original prompt
- [ ] Compared lists side-by-side
- [ ] All tasks from prompt present in tasks.md (100% coverage)
- [ ] No extra tasks in tasks.md (no hallucinated tasks)

**Task Granularity Verification:**
- [ ] Extracted story point scores from tasks.md
- [ Identified atomic tasks (tasks with no subtasks)
- [ ] Verified all atomic tasks ≤ 5 story points
- [ ] No atomic tasks exceed 5 story points

**Naming Quality Verification:**
- [ ] Extracted task names from tasks.md
- [ ] Applied regex pattern: `^[A-Za-z]+ [A-Za-z0-9_\-]*$`
- [ ] Calculated naming quality score (verb-noun pattern adherence)
- [ ] Score ≥ 95% (≥ 95% tasks follow pattern)

**Hierarchy Depth Verification:**
- [ ] Extracted nesting structure from tasks.md
- [ ] Calculated maximum nesting depth
- [ ] Score passes (depth ≤ 2 levels)

**Task Description Quality Verification:**
- [ ] Extracted task descriptions from tasks.md
- [ ] Manual review for clarity, specificity, actionability
- [ ] Score ≥ 90% (≥ 90% descriptions clear/specific)

**Overall SW1 Score Calculation:**
- [ ] Task coverage completeness score calculated
- [ ] Task granularity score calculated
- [ ] Naming quality score calculated
- [ ] Hierarchy depth score calculated
- [ ] Task description quality score calculated
- [ ] Overall weighted score ≥ 80/100
- [ ] Overall improvement ≥ 10% vs baseline

**Quality Gates Verification:**
- [ ] Task coverage completeness gate passed (100% coverage)
- [ ] Task granularity gate passed (all atomic ≤ 5 points)
- [ ] Naming quality gate passed (≥ 95% verb-noun pattern)
- [ ] Hierarchy depth gate passed (≤ 2 levels max)
- [ ] Task description quality gate passed (≥ 90% clear/specific)

### SW2 Evaluation Checklist

**File Presence:**
- [ ] Output file exists: `workspace/sw2-iteration-<N>/outputs.md`
- [ ] Output file is non-empty (≥ 10 characters)
- [ ] Output file is valid markdown (parseable)
- [ ] Output file includes tasks.md content (accumulation pattern)

**Desired State Coverage Verification:**
- [ ] Extracted desired state sections from outputs.md
- [ ] Extracted task names from tasks.md
- [ ] Compared counts: desired state sections vs task count
- [ ] 100% coverage (all tasks have desired states)

**Criteria Testability Verification:**
- [ ] Extracted GWT criteria from outputs.md
- [ ] Manual review for testability (observable, measurable)
- [ ] Score ≥ 90% (≥ 90% criteria testable)

**Criteria Unambiguity Verification:**
- [ ] Extracted GWT criteria from outputs.md
- [ ] Manual review for ambiguity (clear pass/fail conditions)
- [ ] Score ≥ 85% (≥ 85% criteria unambiguous)

**Criteria Count Balance Verification:**
- [ ] Extracted criteria count from outputs.md
- [ ] Extracted task count from tasks.md
- [ ] Calculated avg criteria per task
- [ ] Score passes (1.5-3.0 criteria per task, balanced)

**GWT Syntax Validity Verification:**
- [ ] Extracted GWT criteria from outputs.md
- [ ] Parse all criteria in GWT evaluator
- [ ] Score = 100% (all criteria parse correctly)

**Overall SW2 Score Calculation:**
- [ ] Desired state coverage score calculated
- [ ] Criteria testability score calculated
- [ ] Criteria unambiguity score calculated
- [ ] Criteria count balance score calculated
- [ ] GWT syntax validity score calculated
- [ ] Overall weighted score ≥ 80/100
- [ ] Overall improvement ≥ 10% vs baseline

**Quality Gates Verification:**
- [ ] Desired state coverage gate passed (100% coverage)
- [ ] Criteria testability gate passed (≥ 90% testable)
- [ ] Criteria unambiguity gate passed (≥ 85% unambiguous)
- [ ] Criteria count balance gate passed (1.5-3.0 criteria per task)
- [ ] GWT syntax validity gate passed (100% parseable)

### SW3 Evaluation Checklist

**File Presence:**
- [ ] Output file exists: `workspace/sw3-iteration-<N>/categories.md`
- [ ] Output file is non-empty (≥ 10 characters)
- [ ] Output file is valid markdown (parseable)
- [ ] Output file includes tasks.md + outputs.md content (accumulation pattern)

**Category Assignment Coverage Verification:**
- [ ] Extracted category assignments from categories.md
- [ ] Extracted task names from tasks.md
- [ ] Compared counts: category assignments vs task count
- [ ] 100% coverage (all tasks have categories)

**Canonical Set Adherence Verification:**
- [ ] Extracted category labels from categories.md
- [ ] Compared each category against canonical set (10 categories)
- [ ] Score = 100% (all categories from canonical set)

**Mutual Exclusivity Verification:**
- [ ] Extracted category assignments from categories.md
- [ ] Verified each task has exactly one category
- [ ] Score = 100% (one category per task)

**Category Reasoning Presence Verification:**
- [ ] Extracted category assignments from categories.md
- [ ] Checked each assignment has reasoning section
- [ ] Score = 100% (all assignments have reasoning)

**Category Consistency Verification:**
- [ ] Identified groups of similar tasks (by complexity + keywords)
- [ ] Verified each group has same category assignment
- [ ] Score ≥ 90% (≥ 90% consistency)

**Category Distribution Balance Verification:**
- [ ] Extracted category assignments from categories.md
- [ ] Calculated category diversity (unique categories / total tasks)
- [ ] Score passes (diversity ≥ 0.3)

**Overall SW3 Score Calculation:**
- [ ] Category assignment coverage score calculated
- [ ] Canonical set adherence score calculated
- [ ] Mutual exclusivity score calculated
- [ ] Category reasoning presence score calculated
- [ ] Category consistency score calculated
- [ ] Category distribution balance score calculated
- [ ] Overall weighted score ≥ 80/100
- [ ] Overall improvement ≥ 10% vs baseline

**Quality Gates Verification:**
- [ ] Category assignment coverage gate passed (100% coverage)
- [ ] Canonical set adherence gate passed (100% canonical)
- [ ] Mutual exclusivity gate passed (one category per task)
- [ ] Category reasoning presence gate passed (all have reasoning)
- [ ] Category consistency gate passed (≥ 90% consistency)
- [ ] Category distribution balance gate passed (diversity ≥ 0.3)

### SW4 Evaluation Checklist

**File Presence:**
- [ ] Output file exists: `workspace/sw4-iteration-<N>/structs.md`
- [ ] Output file is non-empty (≥ 10 characters)
- [ ] Output file is valid markdown (parseable)
- [ ] Output file includes tasks.md + outputs.md + categories.md content (accumulation pattern)

**Skeleton Coverage Verification:**
- [ ] Extracted YAML skeletons from structs.md
- [ ] Extracted task names from tasks.md
- [ ] Compared counts: skeletons vs task count
- [ ] 100% coverage (all tasks have skeletons)

**YAML Syntax Validity Verification:**
- [ ] Extracted YAML blocks from structs.md (```yaml markers)
- [ ] Parsed each YAML block with serde_yaml
- [ ] Score = 100% (all skeletons parse without errors)

**Schema Compliance Verification:**
- [ ] Extracted YAML skeletons from structs.md
- [ ] Compared structure against unified schema (lines 318-367)
- [ ] Score = 100% (all skeletons follow schema)

**Model Reference Correctness Verification:**
- [ ] Extracted YAML skeletons from structs.md
- [ ] Searched for model references in skeletons
- [ ] Score = 100% (all use `${models.primary-analyzer}`)

**Hook Presence Verification:**
- [ ] Extracted YAML skeletons from structs.md
- [ ] Counted triggers in each skeleton
- [ ] Calculated average triggers per skeleton
- [ ] Score passes (average ≥ 5 triggers)

**Non-YAML Block Absence Verification:**
- [ ] Extracted code blocks from structs.md
- [ ] Verified only ```yaml blocks present (no rust/python/bash)
- [ ] Score = 100% (only YAML blocks)

**Overall SW4 Score Calculation:**
- [ ] Skeleton coverage score calculated
- [ ] YAML syntax validity score calculated
- [ ] Schema compliance score calculated
- [ ] Model reference correctness score calculated
- [ ] Hook presence score calculated
- [ ] Non-YAML block absence score calculated
- [ ] Overall weighted score ≥ 80/100
- [ ] Overall improvement ≥ 10% vs baseline

**Quality Gates Verification:**
- [ ] Skeleton coverage gate passed (100% coverage)
- [ ] YAML syntax validity gate passed (all parse errors fixed)
- [ ] Schema compliance gate passed (all skeletons follow schema)
- [ ] Model reference correctness gate passed (all use primary-analyzer)
- [ ] Hook presence gate passed (≥ 5 triggers per skeleton)
- [ ] Non-YAML block absence gate passed (only YAML blocks)

### SW5 Evaluation Checklist

**File Presence:**
- [ ] Output file exists: `workspace/sw5-iteration-<N>/workflow.yml`
- [ ] Output file is non-empty (≥ 10 characters)
- [ ] Output file is valid YAML (parseable)

**Schema Validity Verification:**
- [ ] Extracted schema_valid flag from logs
- [ ] Verified schema_valid=true in logs
- [ ] Verified no unknown keys in workflow.yml

**Input Coverage Verification:**
- [ ] Extracted step names from workflow.yml
- [ ] Extracted task names from tasks.md
- [ ] Compared lists: step names vs task names
- [ ] 100% coverage (all tasks present as steps)

**YAML Parseability Verification:**
- [ ] Parsed workflow.yml with serde_yaml
- [ ] Verified parse succeeds without errors

**Dependency Correctness Verification:**
- [ ] Extracted requires: references from workflow.yml
- [ ] Verified all requires: point to existing steps
- [ ] Verified no circular dependencies (A requires B, B requires A)
- [ ] Verified no self-references

**Hook Completeness Verification:**
- [ ] Counted triggers in each step
- [ ] Calculated average triggers per step
- [ ] Score passes (average ≥ 5 triggers)

**Template Interpolation Correctness Verification:**
- [ ] Extracted all {{...}} template references from workflow.yml
- [ ] Validated all references resolve correctly
- [ ] Score = 100% (all references resolve)

**Executability Verification:**
- [ ] Ran workflow.yml on live Docker system
- [ ] Verified logs contain `[workflow:end]` event
- [ ] Verified no panics or crashes
- [ ] Verified output directory contains expected artifacts

**Overall SW5 Score Calculation:**
- [ ] Schema validity score calculated
- [ ] Input coverage score calculated
- [ ] YAML parseability score calculated
- [ ] Dependency correctness score calculated
- [ ] Hook completeness score calculated
- [ ] Template interpolation correctness score calculated
- [ ] Executability score calculated
- [ ] Overall weighted score ≥ 80/100
- [ ] Overall improvement ≥ 10% vs baseline

**Quality Gates Verification:**
- [ ] Schema validity gate passed (schema_valid=true)
- [ ] Input coverage gate passed (100% coverage)
- [ ] YAML parseability gate passed (no parse errors)
- [ ] Dependency correctness gate passed (all dependencies valid)
- [ ] Hook completeness gate passed (≥ 5 triggers per step)
- [ ] Template interpolation correctness gate passed (all references resolve)
- [ ] Executability gate passed (runs successfully on live system)

## Evaluation Criteria Checklist

Comprehensive checklist for validating that evaluation criteria have been applied correctly before claiming a sub-workflow surpasses baseline.

### SW1 Evaluation Criteria

**Quantitative Metrics:**
- [ ] Task coverage completeness score: <value>/100 (≥ 80 required)
- [ ] Task granularity score: <value>/100 (≥ 80 required)
- [ ] Naming quality score: <value>/100 (≥ 95 required)
- [ ] Hierarchy depth score: <value>/100 (≤ 2 required)
- [ ] Task description quality score: <value>/100 (≥ 90 required)
- [ ] Overall weighted score: <value>/100 (≥ 80 required)
- [ ] Overall improvement vs baseline: <value>% (≥ 10% required)

**Qualitative Assessment:**
- [ ] Task decomposition quality: <baseline description> | Better: more atomic, flatter, well-scoped | Same: similar granularity | Worse: coarser, deeper hierarchy
- [ ] Criteria precision: <baseline description> | Better: more specific, more testable | Same: similar precision | Worse: less specific, more vague
- [ ] Category appropriateness: <baseline description> | Better: better alignment, clearer reasoning | Same: similar alignment | Worse: poor alignment, unclear reasoning
- [ ] YAML maintainability: <baseline description> | Better: cleaner structure, better readability | Same: similar structure | Worse: messy structure
- [ ] Hook integration: <baseline description> | Better: more thoughtful, better integrated | Same: similar integration | Worse: poor integration
- [ ] Error handling: <baseline description> | Better: more robust, specific errors | Same: similar handling | Worse: fragile handling

**Quality Gates:**
- [ ] Task coverage completeness gate passed (100% coverage)
- [ ] Task granularity gate passed (all atomic ≤ 5 points)
- [ ] Naming quality gate passed (≥ 95% verb-noun pattern)
- [ ] Hierarchy depth gate passed (≤ 2 levels max)
- [ ] Task description quality gate passed (≥ 90% clear/specific)

**SW1 Surpass Status:**
- [ ] **SURPASSES** — All quantitative and qualitative criteria met
- [ ] **DOESN'T SURPASS** — One or more criteria not met

### SW2 Evaluation Criteria

**Quantitative Metrics:**
- [ ] Desired state coverage score: <value>/100 (≥ 80 required)
- [ ] Criteria testability score: <value>/100 (≥ 90 required)
- [ ] Criteria unambiguity score: <value>/100 (≥ 85 required)
- [ ] Criteria count balance score: <value>/100 (1.5-3.0 criteria per task required)
- [ ] GWT syntax validity score: <value>/100 (100% required)
- [ ] Overall weighted score: <value>/100 (≥ 80 required)
- [ ] Overall improvement vs baseline: <value>% (≥ 10% required)

**Qualitative Assessment:**
- [ ] Criteria precision: <baseline description> | Better: more specific, more testable | Same: similar precision | Worse: less specific, less testable
- [ ] Testability: <baseline description> | Better: more observable, more measurable | Same: similar testability | Worse: less observable, less measurable
- [ ] Scope alignment: <baseline description> | Better: matches task scope exactly | Same: similar scope | Worse: exceeds or under task scope
- [ ] Balance: <baseline description> | Better: 1.5-3.0 criteria per task | Same: similar balance | Worse: over-constrained or under-constrained
- [ ] GWT format: <baseline description> | Better: perfect GWT syntax | Same: similar GWT format | Worse: GWT syntax errors

**Quality Gates:**
- [ ] Desired state coverage gate passed (100% coverage)
- [ ] Criteria testability gate passed (≥ 90% testable)
- [ ] Criteria unambiguity gate passed (≥ 85% unambiguous)
- [ ] Criteria count balance gate passed (1.5-3.0 criteria per task)
- [ ] GWT syntax validity gate passed (100% parseable)

**SW2 Surpass Status:**
- [ ] **SURPASSES** — All quantitative and qualitative criteria met
- [ ] **DOESN'T SURPASS** — One or more criteria not met

### SW3 Evaluation Criteria

**Quantitative Metrics:**
- [ ] Category assignment coverage score: <value>/100 (≥ 80 required)
- [ ] Canonical set adherence score: <value>/100 (100% required)
- [ ] Mutual exclusivity score: <value>/100 (100% required)
- [ ] Category reasoning presence score: <value>/100 (100% required)
- [ ] Category consistency score: <value>/100 (≥ 90% required)
- [ ] Category distribution balance score: <value>/100 (≥ 0.3 diversity required)
- [ ] Overall weighted score: <value>/100 (≥ 80 required)
- [ ] Overall improvement vs baseline: <value>% (≥ 10% required)

**Qualitative Assessment:**
- [ ] Category alignment: <baseline description> | Better: better alignment, clearer reasoning | Same: similar alignment | Worse: poor alignment
- [ ] Consistency: <baseline description> | Better: high consistency across similar tasks | Same: similar consistency | Worse: inconsistent
- [ ] Diversity: <baseline description> | Better: high diversity of categories | Same: similar diversity | Worse: low diversity
- [ ] Reasoning quality: <baseline description> | Better: detailed reasoning provided | Same: similar reasoning quality | Worse: shallow or generic reasoning
- ] Balance: <baseline description> | Better: balanced 1.5-3.0 criteria per task | Same: similar balance | Worse: over-constrained or under-constrained

**Quality Gates:**
- [ ] Category assignment coverage gate passed (100% coverage)
- [ ] Canonical set adherence gate passed (100% canonical)
- [ ] Mutual exclusivity gate passed (one category per task)
- [ ] Category reasoning presence gate passed (all have reasoning)
- [ ] Category consistency gate passed (≥ 90% consistency)
- [ ] Category distribution balance gate passed (diversity ≥ 0.3)

**SW3 Surpass Status:**
- [ ] **SURPASSES** — All quantitative and qualitative criteria met
- [ ] **DOESN'T SURPASS** — One or more criteria not met

### SW4 Evaluation Criteria

**Quantitative Metrics:**
- [ ] Skeleton coverage score: <value>/100 (≥ 80 required)
- [ ] YAML syntax validity score: <value>/100 (≥ 80% required)
- [ ] Schema compliance score: <value>/100 (≥ 80% required)
- [ ] Model reference correctness score: <value>/100 (100% required)
- [ ] Hook presence score: <value>/100 (≥ 5 triggers per skeleton required)
- [ ] Non-YAML block absence score: <value>/100 (100% required)
- [ ] Overall weighted score: <value>/100 (≥ 80 required)
- [ ] Overall improvement vs baseline: <value>% (≥ 10% required)

**Qualitative Assessment:**
- [ ] Schema adherence: <baseline description> | Better: strict schema adherence | Same: similar adherence | Worse: schema violations
- [ ] Model reference: <baseline description> | Better: all use primary-analyzer | Same: some use wrong reference | Worse: many use wrong references
- [ ] Hook wiring: <baseline description> | Better: appropriate hooks for category | Same: similar hook patterns | Worse: poor hook coverage
- [ ] YAML structure: <baseline description> | Better: clean, readable structure | Same: similar structure | Worse: messy structure
- **Error robustness**: <baseline description> | Better: graceful error handling | Same: similar error handling | Worse: fragile error handling

**Quality Gates:**
- [ ] Skeleton coverage gate passed (100% coverage)
- [ ] YAML syntax validity gate passed (all parse errors fixed)
- [ ] Schema compliance gate passed (all skeletons follow schema)
- [ ] Model reference correctness gate passed (all use primary-analyzer)
- [ ] Hook presence gate passed (≥ 5 triggers per skeleton)
- [ ] Non-YAML block absence gate passed (only YAML blocks)

**SW4 Surpass Status:**
- [ ] **SURPASSES** — All quantitative and qualitative criteria met
- [ ] **DOESN'T SURPASS** — One or more criteria not met

### SW5 Evaluation Criteria

**Quantitative Metrics:**
- [ ] Schema validity score: <value>/100 (≥ 80 required)
- [ ] Input coverage score: <value>/100 (≥ 80 required)
- [ ] YAML parseability score: <value>/100 (≥ 80% required)
- [ ] Dependency correctness score: <value>/100 (≥ 80% required)
- [ ] Hook completeness score: <value>/100 (≥ 5 triggers per step required)
- [ ] Template interpolation correctness score: <value>/100 (100% required)
- [ ] Executability score: <value>/100 (≥ 80% required)
- [ ] Overall weighted score: <value>/100 (≥ 80 required)
- [ ] Overall improvement vs baseline: <value>% (≥ 10% required)

**Qualitative Assessment:**
- [ ] Schema adherence: <baseline description> | Better: strict schema adherence | Same: similar adherence | Worse: schema violations
- [ ] Task completeness: <baseline description> | Better: all tasks present and complete | Same: similar task presence | Worse: missing or incomplete tasks
- [ ] Dependency logic: <baseline description> | Better: no circular dependencies, clear flow | Same: similar dependency logic | Worse: broken dependencies
- [ ] Hook integration: <baseline description> | Better: thoughtful hook integration | Same: similar hook patterns | Worse: poor hook integration
- [ ] Error handling: <baseline description> | Better: graceful error handling, specific errors | Same: similar error handling | Worse: poor error handling
- **Execution reliability**: <baseline description> | Better: runs successfully on live system | Same: similar execution reliability | Worse: execution failures

**Quality Gates:**
- [ ] Schema validity gate passed (schema_valid=true)
- [ ] Input coverage gate passed (100% coverage)
- [ ] YAML parseability gate passed (no parse errors)
- [ ] Dependency correctness gate passed (all dependencies valid)
- [ ] Hook completeness gate passed (≥ 5 triggers per step)
- [ ] Template interpolation correctness gate passed (all references resolve)
- [ ] Executability gate passed (runs successfully on live system)

**SW5 Surpass Status:**
- [ ] **SURPASSES** — All quantitative and qualitative criteria met
- [ ] **DOESN'T SURPASS** — One or more criteria not met

---

**Document Status:** Draft  
**Last Updated:** 2026-06-17  
**Author:** Sisyphus-Junior (Whitt Execution Engine Planning)  
**Review Status:** Ready for Execution