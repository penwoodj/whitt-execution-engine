# META-v6 Iteration Protocol

> **Purpose:** Define how to iterate each sub-workflow until quality exceeds baseline

## Iteration Loop

**Goal:** Improve generated YAML quality until score > baseline

**Loop Steps:**
1. Run SW on baseline prompt dataset
2. Collect outputs in timestamped directory
3. Analyze outputs vs baseline
4. Compute quality score (see `05-QUALITY-BENCHMARK.md`)
5. Compare score vs baseline
6. Identify quality gaps
7. Improve YAML workflow
8. Rerun SW
9. Compare new score vs previous
10. Continue until quality > baseline or max 5 iterations

## Baseline Setup

### Collect Baseline Prompts

**Source:** Opencode session DB
**Count:** 50-100 real user prompts
**Storage:** `docs/plans/meta-v6/baseline/prompts/`
**Format:** One `.md` file per prompt

**Example:**
```markdown
# Prompt 001
Build a REST API with authentication using JWT tokens. Implement rate limiting and logging middleware.
```

**Naming:** `prompt-001.md`, `prompt-002.md`, ..., `prompt-100.md`

### Generate Single-Shot Baselines

**Purpose:** Provide quality baseline for comparison
**Method:** Run all 5 SWs sequentially on each prompt without iteration
**Storage:** `docs/plans/meta-v6/baseline/single-shot/`
**Format:** `baseline-<prompt_id>-workflow.yml`

**Process:**
```bash
for prompt in docs/plans/meta-v6/baseline/prompts/*.md; do
  # Run SW1-SW5 on prompt
  ./scripts/meta-v6/run-sw1.sh --input $prompt
  ./scripts/meta-v6/run-sw2.sh
  ./scripts/meta-v6/run-sw3.sh
  ./scripts/meta-v6/run-sw4.sh
  ./scripts/meta-v6/run-sw5.sh

  # Copy final workflow to baseline dir
  cp docs/plans/meta-v6/outputs/final-workflow.yml \
     docs/plans/meta-v6/baseline/single-shot/baseline-$(basename $prompt .md)-workflow.yml
done
```

### Baseline Quality Score

**Method:** Manual assessment using rubric (see `05-QUALITY-BENCHMARK.md`)
**Storage:** `docs/plans/meta-v6/baseline/baseline-quality-report.md`
**Metrics:**
- Schema validity (pass/fail)
- Input coverage (percentage)
- Task granularity (avg story points)
- Hook presence (triggers per step)
- GWT validity (percentage)
- Quality score (0-100)

## Iteration Execution

### Step 1: Run SW on Dataset

**Command:** `./scripts/meta-v6/run-sw<N>.sh --input <prompt_file>`
**Output Directory:** `docs/benchmarks/outputs/meta-workflow/meta-<run_id>-sw<N>-<timestamp>/`
**Contents:**
- `run.log` - Full execution log
- `output.md` - Generated markdown (SW1-SW4)
- `workflow.yml` - Final YAML (SW5 only)
- `benchmark.log` - Benchmark-specific log

**Tracking:** Update `.current-meta-run` with run ID

**Example:**
```bash
# Run SW1 on all prompts
for prompt in docs/plans/meta-v6/baseline/prompts/*.md; do
  ./scripts/meta-v6/run-sw1.sh --input $prompt
done
```

### Step 2: Collect Outputs

**Gather:** All output directories for current iteration
**Storage:** `docs/plans/meta-v6/iterations/iter-<iter_num>/`
**Structure:**
```
docs/plans/meta-v6/iterations/iter-01/
├── prompts/
│   ├── prompt-001-output/
│   ├── prompt-002-output/
│   └── ...
├── aggregated-metrics.md
└── iteration-report.md
```

### Step 3: Analyze vs Baseline

**Compare:** Current iteration output vs single-shot baseline
**Metrics:**
- Schema validity (same)
- Input coverage (current - baseline)
- Task granularity (current - baseline)
- Hook presence (current - baseline)
- GWT validity (current - baseline)
- Quality score (current - baseline)

**Automated Checks:**
```bash
# Schema validity
grep -c "schema_valid=true" docs/benchmarks/outputs/meta-workflow/*/run.log

# Input coverage
diff <(grep "^## Task:" tasks.md | wc -l) \
     <(grep "^  - name:" workflow.yml | wc -l)

# Hook presence
grep -c "hooks:" workflow.yml
```

### Step 4: Compute Quality Score

**Rubric:** See `05-QUALITY-BENCHMARK.md`
**Dimensions:**
1. Schema validity (20 points)
2. Input coverage (20 points)
3. Task granularity (15 points)
4. Hook presence (15 points)
5. GWT validity (10 points)
6. Action variety (10 points)
7. Template interpolation (10 points)

**Calculation:** Sum of dimension scores
**Storage:** `docs/plans/meta-v6/iterations/iter-<iter_num>/quality-scores.md`

### Step 5: Identify Quality Gaps

**Gap Analysis:** For each dimension where current < baseline:
1. Identify specific failure
2. Determine root cause
3. Plan improvement

**Common Gaps:**

| Gap | Symptom | Root Cause | Improvement |
|-----|---------|------------|-------------|
| Schema invalid | `schema_valid=false` | Wrong provider key, non-schema key | Fix YAML structure, reference schema |
| <100% coverage | Missing tasks | Task decomposition incomplete | Improve prompt in SW1 |
| >5 points/leaf | Large steps | Insufficient decomposition | Split tasks in SW1 |
| <5 triggers/step | Missing hooks | Hook insertion incomplete | Add hooks in SW4 |
| GWT parse error | Invalid expression | GWT syntax error | Simplify expression, fix syntax |
| Low action variety | Only Log actions | Action variety incomplete | Add SaveTo, Bookmark, etc. |

### Step 6: Improve YAML Workflow

**Target:** The YAML workflow file for the SW being iterated
**Method:** Edit YAML prompt template, hook config, or structure

**Improvement Types:**

1. **Prompt Improvement**
   - Add more specific instructions
   - Add examples of desired output
   - Add constraints (e.g., "max 5 story points per leaf")

2. **Hook Improvement**
   - Add missing triggers
   - Add more actions per trigger
   - Improve GWT expressions

3. **Template Improvement**
   - Add more interpolation variables
   - Add better structure
   - Add validation checks

4. **Structure Improvement**
   - Reorder steps for better flow
   - Add intermediate validation steps
   - Add error handling hooks

**Example Improvement:**
```yaml
# Before
hooks:
  after_step_succeeds:
    - log:
        message: "Step completed"

# After
hooks:
  after_step_succeeds:
    - log:
        message: "Step {{step_name}} completed"
        to_file_path: "logs/{{step_name}}.log"
    - save_to:
        file_path: "outputs/{{step_name}}.md"
        content: "{{step_output}}"
    - bookmark:
        key: "step_{{step_name}}_result"
        value: "{{step_output}}"
    - gwt:
        - given: "{{output.file_size}}"
          when: "> 0"
          then:
            - log:
                message: "✅ Output file written"
          else:
            - fail:
                message: "❌ Output file empty"
```

### Step 7: Rerun SW

**Command:** Same as Step 1
**Output:** New timestamped directory
**Tracking:** Update `.current-meta-run` with new run ID

### Step 8: Compare New Score vs Previous

**Metric:** Quality score difference
**Target:** +5 points per iteration (minimum)
**Storage:** Update `docs/plans/meta-v6/iterations/iter-<iter_num>/quality-scores.md`

**Example:**
```
Iteration 1: 65/100
Iteration 2: 72/100 (+7)
Iteration 3: 79/100 (+7)
Iteration 4: 84/100 (+5) → > baseline (80/100) ✅
```

## Iteration Gates

### Gate 1: Schema Validation

**Check:** `schema_valid=true` in logs
**Pass:** Continue to quality assessment
**Fail:** Fix YAML structure, rerun

**Command:**
```bash
grep "schema_valid=true" docs/benchmarks/outputs/meta-workflow/*/run.log
```

### Gate 2: Input Coverage

**Check:** 100% of tasks covered
**Pass:** Continue to quality assessment
**Fail:** Improve task decomposition in SW1, rerun

**Command:**
```bash
task_count=$(grep "^## Task:" tasks.md | wc -l)
step_count=$(grep "^  - name:" workflow.yml | wc -l)
[ $task_count -eq $step_count ]
```

### Gate 3: Task Granularity

**Check:** ≤5 story points per leaf task
**Pass:** Continue to quality assessment
**Fail:** Improve task splitting in SW1, rerun

**Command:**
```bash
# Manual check in structs.md
# Look for story points > 5
```

### Gate 4: Hook Coverage

**Check:** ≥5 triggers per step
**Pass:** Continue to quality assessment
**Fail:** Add hooks in SW4, rerun

**Command:**
```bash
grep -A 10 "after_step_succeeds:" workflow.yml | grep -c "  -"
```

### Gate 5: Quality Score

**Check:** Quality score > baseline
**Pass:** Stop iteration
**Fail:** Continue to next iteration

**Condition:**
```
if current_score > baseline_score:
    stop
elif iteration_count < 5:
    continue
else:
    stop (max iterations reached)
```

## Iteration Tracking

### Iteration Report Template

**File:** `docs/plans/meta-v6/iterations/iter-<iter_num>/iteration-report.md`

**Format:**
```markdown
# Iteration <iter_num> Report

## Summary
- Run ID: <run_id>
- Timestamp: <YYYYMMDD-HHMMSS>
- SW Iterated: <SW1|SW2|SW3|SW4|SW5>
- Quality Score: <score>/100
- Baseline Score: <score>/100
- Improvement: <+X or -X points>

## Metrics
| Dimension | Current | Baseline | Delta |
|-----------|---------|----------|-------|
| Schema Validity | <Y/N> | <Y/N> | - |
| Input Coverage | <%> | <%> | <+X%> |
| Task Granularity | <avg points> | <avg points> | <+X> |
| Hook Presence | <avg triggers> | <avg triggers> | <+X> |
| GWT Validity | <%> | <%> | <+X%> |

## Quality Gaps Found
1. <gap description>
   - Root cause: <cause>
   - Improvement: <what was changed>

## Improvements Made
1. <improvement description>
   - File changed: <workflow file>
   - Lines modified: <line numbers>

## Next Steps
- [ ] Rerun SW with improvements
- [ ] Reassess quality
- [ ] Compare vs previous iteration
```

### Quality Scores Summary

**File:** `docs/plans/meta-v6/iterations/quality-scores-summary.md`

**Format:**
```markdown
# Quality Scores Summary

| Iteration | Score | Baseline | Delta | Status |
|-----------|-------|----------|-------|--------|
| 0 (baseline) | 65 | 65 | 0 | - |
| 1 | 72 | 65 | +7 | ✅ Improved |
| 2 | 79 | 65 | +14 | ✅ Improved |
| 3 | 84 | 65 | +19 | ✅ Complete |
```

## Stop Conditions

### Success Condition

**When:** Quality score > baseline
**Action:** Stop iteration, document final result
**Storage:** `docs/plans/meta-v6/iterations/FINAL-REPORT.md`

### Failure Condition

**When:** Max 5 iterations reached without improvement
**Action:** Stop iteration, document findings, escalate

**Escalation:**
1. Document gaps in `docs/plans/meta-v6/iterations/FAILED-ITERATION.md`
2. Review with human
3. Decide: adjust baseline, improve prompts, or accept current state

### Technical Failure Condition

**When:** Server OOM, panic, or unrecoverable error
**Action:** Stop iteration, document error, fix infrastructure

## Iteration Strategy per SW

### SW1 Iteration Strategy

**Focus:** Task decomposition quality
**Metrics:** Input coverage, task granularity
**Common Issues:**
- Missing tasks
- Tasks too large (>5 points)
- Poor task naming

**Improvements:**
- Add explicit instruction: "Break down tasks until each leaf has ≤5 story points"
- Add examples of good vs bad decomposition
- Add validation step to check coverage

### SW2 Iteration Strategy

**Focus:** Category assignment quality
**Metrics:** Output assignment, category non-empty
**Common Issues:**
- Unassigned outputs
- Empty categories
- Poor category naming

**Improvements:**
- Add instruction: "Every output must belong to exactly one category"
- Add validation: "No empty categories allowed"
- Add examples of good category design

### SW3 Iteration Strategy

**Focus:** Dependency mapping quality
**Metrics:** Dependency validity, model consistency
**Common Issues:**
- Cyclic dependencies
- Wrong model name
- Missing interpolation

**Improvements:**
- Add instruction: "Dependencies must form a DAG"
- Add validation: "Check for cycles"
- Add template examples

### SW4 Iteration Strategy

**Focus:** YAML structure quality
**Metrics:** Schema validity, hook coverage
**Common Issues:**
- Wrong provider key
- Missing hooks
- Non-schema keys

**Improvements:**
- Add instruction: "Provider key must be llama_cpp_with_vulkan"
- Add hook template: "Include at least 2 hooks per step"
- Add schema reference comments

### SW5 Iteration Strategy

**Focus:** Validation and final assembly
**Metrics:** Schema validity, error handling
**Common Issues:**
- Schema validation fails
- Missing error handling
- Missing workflow hooks

**Improvements:**
- Add validation step
- Add error handling hooks
- Add workflow-level hooks

## Parallel vs Sequential Iteration

**Strategy:** Sequential (one SW at a time)

**Reason:**
- Each SW depends on previous SW output
- Quality improvements cascade
- Easier to track root causes

**Order:**
1. Iterate SW1 until quality > baseline
2. Iterate SW2 until quality > baseline
3. Iterate SW3 until quality > baseline
4. Iterate SW4 until quality > baseline
5. Iterate SW5 until quality > baseline

## Documentation During Iteration

### Per-Iteration

- Create iteration directory
- Save all outputs
- Generate iteration report
- Update quality scores summary

### Post-Iteration

- Compare vs baseline
- Document gaps
- Plan improvements
- Apply improvements

### Final

- Generate final report
- Archive all iterations
- Document lessons learned

---

**Document Status:** Draft
**Last Updated:** 2026-06-14
**Author:** META-v6 Planning Session
**Review Status:** Pending