# META-v6 Quality Benchmark

> **Purpose:** Define baseline dataset strategy, scoring rubric, and pass criteria

## Baseline Dataset Strategy

### Data Source

**Source:** Opencode session database
**Export Method:** SQL query on session messages
**Count:** 50-100 prompts
**Diversity:** Multiple domains (web dev, CLI tools, API servers, etc.)

**Export Query:**
```sql
SELECT
    s.id as session_id,
    m.content as user_prompt
FROM messages m
JOIN sessions s ON m.session_id = s.id
WHERE m.role = 'user'
  AND LENGTH(m.content) > 50
  AND LENGTH(m.content) < 500
ORDER BY RANDOM()
LIMIT 100;
```

### Data Collection

**Storage:** `docs/plans/meta-v6/baseline/prompts/`
**Format:** One `.md` file per prompt

**Naming Convention:**
- `prompt-001.md` through `prompt-100.md`
- Sequential numbering for tracking

**File Format:**
```markdown
# Prompt <ID>

<user_prompt_text>

---
**Source:** Session <session_id>
**Length:** <char_count> characters
**Date:** <YYYY-MM-DD>
```

**Example:**
```markdown
# Prompt 001

Build a REST API with authentication using JWT tokens. Implement rate limiting and logging middleware.

---
**Source:** Session 12345
**Length:** 127 characters
**Date:** 2026-06-14
```

### Single-Shot Baseline Generation

**Purpose:** Generate quality baseline for comparison
**Method:** Run all 5 SWs sequentially without iteration
**Storage:** `docs/plans/meta-v6/baseline/single-shot/`

**Generation Script:**
```bash
#!/bin/bash
mkdir -p docs/plans/meta-v6/baseline/single-shot

for prompt_file in docs/plans/meta-v6/baseline/prompts/*.md; do
  prompt_id=$(basename "$prompt_file" .md | sed 's/prompt-//')

  echo "Processing prompt-$prompt_id..."

  # Run all 5 SWs
  ./scripts/meta-v6/run-sw1.sh --input "$prompt_file"
  ./scripts/meta-v6/run-sw2.sh
  ./scripts/meta-v6/run-sw3.sh
  ./scripts/meta-v6/run-sw4.sh
  ./scripts/meta-v6/run-sw5.sh

  # Copy final workflow to baseline dir
  cp docs/plans/meta-v6/outputs/final-workflow.yml \
     "docs/plans/meta-v6/baseline/single-shot/baseline-prompt-${prompt_id}-workflow.yml"

  echo "Saved baseline for prompt-$prompt_id"
done
```

### Baseline Quality Assessment

**Method:** Manual assessment using rubric (below)
**Storage:** `docs/plans/meta-v6/baseline/baseline-quality-report.md`

**Assessment Process:**
1. Load each baseline YAML
2. Run through rubric (7 dimensions)
3. Compute score (0-100)
4. Average across all baselines
5. Record in report

**Baseline Report Format:**
```markdown
# Baseline Quality Report

## Summary
- Total Prompts: 100
- Total Baselines: 100
- Average Score: 65/100
- Min Score: 50/100
- Max Score: 78/100

## Per-Prompt Scores
| Prompt ID | Score | Schema | Coverage | Granularity | Hooks | GWT | Variety | Template |
|-----------|-------|--------|----------|-------------|-------|-----|---------|----------|
| 001 | 65 | ✅ | 100% | 6.2 | 3 | 90% | 2 | ✅ |
| 002 | 70 | ✅ | 100% | 5.1 | 4 | 95% | 3 | ✅ |
| ... | ... | ... | ... | ... | ... | ... | ... | ... |

## Quality Distribution
- 0-49: 0 prompts
- 50-59: 15 prompts
- 60-69: 50 prompts
- 70-79: 30 prompts
- 80-89: 5 prompts
- 90-100: 0 prompts

## Common Issues
1. Task granularity > 5 points (30% of baselines)
2. Hook coverage < 5 triggers/step (40% of baselines)
3. GWT expressions too simple (50% of baselines)
```

## Scoring Rubric

### Dimension 1: Schema Validity (20 points)

**Check:** YAML passes schema validation
**Evidence:** `schema_valid=true` in logs
**Scoring:**
- 20 points: `schema_valid=true`
- 0 points: `schema_valid=false` or validation error

**Validation Command:**
```bash
grep "schema_valid=true" docs/benchmarks/outputs/meta-workflow/*/run.log | wc -l
```

### Dimension 2: Input Coverage (20 points)

**Check:** 100% of tasks from input covered in output
**Evidence:** Count tasks in tasks.md vs steps in workflow.yml
**Scoring:**
- 20 points: 100% coverage
- 15 points: 90-99% coverage
- 10 points: 80-89% coverage
- 5 points: 70-79% coverage
- 0 points: <70% coverage

**Validation Command:**
```bash
task_count=$(grep "^## Task:" tasks.md | wc -l)
step_count=$(grep "^  - name:" workflow.yml | wc -l)
coverage=$((step_count * 100 / task_count))
echo "Coverage: $coverage%"
```

### Dimension 3: Task Granularity (15 points)

**Check:** Average story points per leaf task ≤5
**Evidence:** Parse structs.md, compute average
**Scoring:**
- 15 points: ≤4 points avg
- 12 points: 4.1-5.0 points avg
- 8 points: 5.1-6.0 points avg
- 4 points: 6.1-7.0 points avg
- 0 points: >7.0 points avg

**Validation Method:**
- Manual inspection of structs.md
- Look for "story_points" field
- Compute average

### Dimension 4: Hook Presence (15 points)

**Check:** Average triggers per step ≥5
**Evidence:** Count hooks in workflow.yml
**Scoring:**
- 15 points: ≥7 triggers/step avg
- 12 points: 5-6 triggers/step avg
- 8 points: 3-4 triggers/step avg
- 4 points: 1-2 triggers/step avg
- 0 points: 0 triggers/step

**Validation Command:**
```bash
avg_hooks=$(grep -c "  - " workflow.yml)
step_count=$(grep "^  - name:" workflow.yml | wc -l)
triggers_per_step=$((avg_hooks / step_count))
echo "Triggers per step: $triggers_per_step"
```

### Dimension 5: GWT Validity (10 points)

**Check:** 100% of GWT expressions parse without error
**Evidence:** GWT evaluator parsing test
**Scoring:**
- 10 points: 100% valid
- 7 points: 90-99% valid
- 5 points: 80-89% valid
- 3 points: 70-79% valid
- 0 points: <70% valid

**Validation Method:**
- Extract all GWT expressions from YAML
- Run through GWT evaluator in `src/workflow/hooks/gwt.rs`
- Count parse errors

**Known Bug:** String == comparison may be broken
- Mitigation: Use numeric/boolean comparisons only
- If string == used, mark as "potentially broken" but still count as parseable

### Dimension 6: Action Variety (10 points)

**Check:** Multiple action types used (not just Log)
**Evidence:** Count unique action types in workflow.yml
**Scoring:**
- 10 points: ≥5 unique action types
- 7 points: 4 unique action types
- 5 points: 3 unique action types
- 3 points: 2 unique action types
- 0 points: 1 action type (only Log)

**Action Types:**
- Log, AppendTo, SaveTo, RouteTo, Bookmark, Notify, Fail, Shell, SkipStep, SkipRemaining, Gwt, IterateValues

**Validation Command:**
```bash
grep -E "  (log|append_to|save_to|route_to|bookmark|notify|fail|shell|skip_step|skip_remaining|gwt|iterate_values):" workflow.yml | sed 's/.*  //' | sed 's/:.*//' | sort -u | wc -l
```

### Dimension 7: Template Interpolation (10 points)

**Check:** Templates use interpolation variables
**Evidence:** Count {{variable}} usage
**Scoring:**
- 10 points: ≥3 interpolation variables per template
- 7 points: 2 interpolation variables per template
- 5 points: 1 interpolation variable per template
- 0 points: 0 interpolation variables

**Template Variables:**
- `{{step_name}}`, `{{model_name}}`, `{{category}}`, `{{output.file_path}}`, etc.

**Validation Command:**
```bash
grep -o '{{[^}]*}}' workflow.yml | sort -u | wc -l
```

## Score Calculation

### Total Score

**Formula:** Sum of all 7 dimension scores
**Range:** 0-100 points

**Example:**
```
Schema Validity: 20/20
Input Coverage: 20/20
Task Granularity: 12/15
Hook Presence: 12/15
GWT Validity: 7/10
Action Variety: 7/10
Template Interpolation: 7/10
---
Total: 85/100
```

### Pass Criteria

**Overall Pass:** Total score ≥80/100

**Dimension Passes (must all pass):**
- Schema Validity: 20/20 (mandatory)
- Input Coverage: ≥15/20 (≥75%)
- Task Granularity: ≥8/15 (≤6.0 avg)
- Hook Presence: ≥8/15 (≥3 triggers/step)
- GWT Validity: ≥7/10 (≥90%)
- Action Variety: ≥5/10 (≥3 action types)
- Template Interpolation: ≥5/10 (≥1 variable)

**Failure Modes:**
- Schema Validity < 20: Critical failure, must fix
- Input Coverage < 15: Quality failure, iterate
- Task Granularity < 8: Quality failure, iterate
- Hook Presence < 8: Quality failure, iterate
- GWT Validity < 7: Quality failure, iterate
- Action Variety < 5: Quality failure, iterate
- Template Interpolation < 5: Quality failure, iterate

## Quality Assessment Process

### Automated Checks

Run for every generated workflow:

```bash
#!/bin/bash
# assess-quality.sh <workflow.yml>

WORKFLOW=$1

# Check 1: Schema validity
SCHEMA_VALID=$(grep -c "schema_valid=true" docs/benchmarks/outputs/meta-workflow/*/run.log)
echo "Schema Validity: $SCHEMA_VALID"

# Check 2: Input coverage (assuming tasks.md exists)
TASK_COUNT=$(grep "^## Task:" tasks.md | wc -l)
STEP_COUNT=$(grep "^  - name:" "$WORKFLOW" | wc -l)
COVERAGE=$((STEP_COUNT * 100 / TASK_COUNT))
echo "Input Coverage: $COVERAGE%"

# Check 3: Hook presence
HOOK_COUNT=$(grep -c "  - " "$WORKFLOW")
TRIGGERS_PER_STEP=$((HOOK_COUNT / STEP_COUNT))
echo "Triggers per Step: $TRIGGERS_PER_STEP"

# Check 4: Action variety
ACTION_TYPES=$(grep -E "  (log|append_to|save_to|route_to|bookmark|notify|fail|shell|skip_step|skip_remaining|gwt|iterate_values):" "$WORKFLOW" | sed 's/.*  //' | sed 's/:.*//' | sort -u | wc -l)
echo "Unique Action Types: $ACTION_TYPES"

# Check 5: Template interpolation
INTERPOLATION_COUNT=$(grep -o '{{[^}]*}}' "$WORKFLOW" | sort -u | wc -l)
echo "Unique Interpolation Variables: $INTERPOLATION_COUNT"
```

### Manual Checks

**Task Granularity:**
1. Open structs.md
2. Look for story_points field
3. Compute average
4. Compare to rubric

**GWT Validity:**
1. Extract all GWT expressions
2. Run through evaluator (in code, not CLI)
3. Count parse errors
4. Compare to rubric

### Scoring Spreadsheet

**Template:** `docs/plans/meta-v6/quality-scoring-template.csv`

**Columns:**
- Prompt ID
- Schema Validity (0-20)
- Input Coverage (0-20)
- Task Granularity (0-15)
- Hook Presence (0-15)
- GWT Validity (0-10)
- Action Variety (0-10)
- Template Interpolation (0-10)
- Total Score (0-100)
- Pass/Fail
- Notes

## Baseline Comparison

### Comparison Method

**For each iteration:**
1. Score iteration outputs using rubric
2. Compare to baseline score
3. Compute delta (iteration - baseline)
4. Determine improvement or regression

**Target:** +5 points per iteration minimum

### Comparison Report

**File:** `docs/plans/meta-v6/iterations/comparison-report.md`

**Format:**
```markdown
# Quality Comparison Report

## Baseline
- Average Score: 65/100
- Min Score: 50/100
- Max Score: 78/100

## Iteration 1
- Average Score: 72/100
- Min Score: 58/100
- Max Score: 82/100
- Improvement: +7 points ✅

## Dimension Comparison
| Dimension | Baseline | Iter 1 | Delta |
|-----------|----------|--------|-------|
| Schema Validity | 18/20 | 20/20 | +2 |
| Input Coverage | 18/20 | 20/20 | +2 |
| Task Granularity | 10/15 | 12/15 | +2 |
| Hook Presence | 9/15 | 12/15 | +3 |
| GWT Validity | 8/10 | 8/10 | 0 |
| Action Variety | 6/10 | 7/10 | +1 |
| Template Interpolation | 6/10 | 7/10 | +1 |

## Significant Improvements
1. Schema validity improved from 90% to 100%
2. Hook coverage increased from 3 to 4 triggers/step avg
3. Action variety added Bookmark action

## Remaining Gaps
1. Task granularity still >5 points avg
2. GWT expressions still simple
3. Need more complex hook chains
```

## Quality Examples

### Excellent Example (85+ points)

**Characteristics:**
- Schema valid
- 100% coverage
- ≤4 points avg
- ≥6 triggers/step
- Complex GWT with multiple clauses
- 5+ action types
- 3+ interpolation variables

**YAML Snippet:**
```yaml
hooks:
  before_step_starts:
    - log:
        message: "Starting {{step_name}} for {{category}}"
        to_file_path: "logs/{{step_name}}.log"
    - bookmark:
        key: "start_time_{{step_name}}"
        value: "{{timestamp}}"

  after_step_succeeds:
    - save_to:
        file_path: "{{output.file_path}}"
        content: "{{step_output}}"
    - log:
        message: "Completed {{step_name}} in {{duration_ms}}ms"
    - gwt:
        - given: "{{output.file_size}}"
          when: "> 0"
          then:
            - log:
                message: "✅ Output file written ({{output.file_size}} bytes)"
          else:
            - fail:
                message: "❌ Output file empty"
    - bookmark:
        key: "output_{{step_name}}"
        value:
          file_path: "{{output.file_path}}"
          size: "{{output.file_size}}"
```

### Good Example (70-84 points)

**Characteristics:**
- Schema valid
- 100% coverage
- 4-5 points avg
- 4-5 triggers/step
- Simple GWT
- 3-4 action types
- 2 interpolation variables

**YAML Snippet:**
```yaml
hooks:
  after_step_succeeds:
    - save_to:
        file_path: "{{output.file_path}}"
        content: "{{step_output}}"
    - log:
        message: "Completed {{step_name}}"
    - bookmark:
        key: "step_{{step_name}}_result"
        value: "{{step_output}}"
```

### Poor Example (<70 points)

**Characteristics:**
- Schema valid or invalid
- <100% coverage
- >6 points avg
- <3 triggers/step
- No GWT
- 1-2 action types
- 0-1 interpolation variables

**YAML Snippet:**
```yaml
hooks:
  after_step_succeeds:
    - log:
        message: "Done"
```

## Quality Benchmark Execution

### Phase 1: Baseline Assessment

1. Collect 50-100 prompts from opencode DB
2. Generate single-shot baselines
3. Assess quality using rubric
4. Compute baseline average score
5. Document in `baseline-quality-report.md`

### Phase 2: Iteration Assessment

For each iteration:
1. Run SW on prompt dataset
2. Collect outputs
3. Assess quality using rubric
4. Compute iteration average score
5. Compare to baseline
6. Document in iteration report

### Phase 3: Final Assessment

After all iterations complete:
1. Assess final outputs
2. Compare to baseline
3. Determine if quality > baseline
4. Document final report

---

**Document Status:** Draft
**Last Updated:** 2026-06-14
**Author:** META-v6 Planning Session
**Review Status:** Pending