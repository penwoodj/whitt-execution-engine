# META-v6 Hooks Strategy

> **Purpose:** Define hook coverage plan, triggers to wire, and action variants to use

## Hook System Overview

**Implementation:** `src/workflow/hooks/mod.rs`
**Contexts:** `src/workflow/hooks/context.rs` (10 variants)
**Actions:** `src/workflow/hooks/actions.rs` (12 variants)
**Runner:** `src/benchmark/runner.rs` (firing points at lines 813-848)

## Triggers Inventory

### Fully Wired Triggers (7/10)

These triggers fire in the runner and dispatch hook actions:

| # | Trigger | Context Struct | Runner Line | Status |
|---|---------|----------------|-------------|--------|
| 1 | `before_step_starts` | `BeforeStepStartsContext` | 1257 | ✅ WIRED |
| 2 | `after_step_starts` | `AfterStepStartsContext` | 1286 | ✅ WIRED |
| 3 | `after_step_fails` | `AfterStepFailsContext` | 1318 | ✅ WIRED |
| 4 | `after_step_succeeds` | `AfterStepSucceedsContext` | 1340 | ✅ WIRED |
| 5 | `after_all_retries_exhausted` | `AfterAllRetriesExhaustedContext` | 1355 | ✅ WIRED |
| 6 | `on_requires_failed` | `OnRequiresFailedContext` | 1535 | ✅ WIRED |
| 7 | `after_loop_iteration_fails` | `AfterLoopIterationFailsContext` | 1592 | ✅ WIRED |

### Partially Wired Triggers (2/10)

These triggers fire but only perform logging (full wire requires future work):

| # | Trigger | Context Struct | Actions Line | Status |
|---|---------|----------------|-------------|--------|
| 8 | `before_gwt_evaluates` | `BeforeGwtEvaluatesContext` | 404 | ⚠️ PARTIAL (logging only) |
| 9 | `after_gwt_evaluates` | `AfterGwtEvaluatesContext` | 414 | ⚠️ PARTIAL (logging only) |

### Dead Trigger (1/10)

This trigger does not fire in the runner:

| # | Trigger | Context Struct | Status |
|---|---------|----------------|--------|
| 10 | `during_step_streaming` | `DuringStepStreamingContext` | ❌ DEAD (requires SSE streaming) |

**Reason:** Runner uses `stream:false` in benchmark mode. Full wire requires architectural change to access chunk-level hooks from LlamaHttpClient::chat_completion.

## Action Variants Inventory

| # | Action | Struct Location | Side Effects | Use Case |
|---|--------|-----------------|--------------|----------|
| 1 | `Log(LogAction)` | `actions.rs:202-215` | File write + stdout | Logging progress, errors |
| 2 | `AppendTo(AppendToAction)` | `actions.rs:217-227` | File append + bookmark | Accumulating results |
| 3 | `SaveTo(SaveToAction)` | `actions.rs:229-244` | File write + bookmark | Saving step outputs |
| 4 | `RouteTo(RouteToAction)` | `actions.rs:246-257` | Control flow (jump) | Conditional routing |
| 5 | `Bookmark(BookmarkAction)` | `actions.rs:259-279` | Memory + file | State management |
| 6 | `Notify(NotifyAction)` | `actions.rs:281-288` | Channel send | Coordination (stub) |
| 7 | `Fail(FailAction)` | `actions.rs:290-297` | Workflow fail | Error handling |
| 8 | `Shell(ShellAction)` | `actions.rs:299-315` | Subprocess + bookmark | System commands |
| 9 | `SkipStep(bool)` | `actions.rs:317-320` | Control flow (skip) | Conditional execution |
| 10 | `SkipRemaining(bool)` | `actions.rs:322-325` | Control flow (skip) | Early termination |
| 11 | `Gwt(Vec<GwtClause>)` | `actions.rs:327-381` | Control flow (eval) | Conditional logic |
| 12 | `IterateValues(HashMap)` | `actions.rs:383-393` | PASSTHROUGH | Future feature |

## META-v6 Hook Coverage Strategy

### Target Coverage

**Goal:** Use all 7 wired triggers with appropriate actions

**Minimum per Step:**
- `before_step_starts` (log start)
- `after_step_succeeds` (save output, bookmark)
- `after_step_fails` (log error, retry or fail)

**Optional per Step:**
- `after_step_starts` (log progress)
- `after_all_retries_exhausted` (log failure, skip or fail)
- `on_requires_failed` (log dependency failure)
- `after_loop_iteration_fails` (log iteration failure)

### Hook Priority

**HookResult Priority:** Fail > SkipRemaining > RouteTo > SkipLoop > SkipStep > Continue

**Implication:**
- If multiple hooks fire, highest priority wins
- Fail action always terminates workflow
- Use RouteTo for conditional branching
- Use SkipStep for conditional execution

### Template Interpolation in Hooks

**Available Variables:**
- `{{step_name}}` - Current step name
- `{{model_name}}` - Model name
- `{{category}}` - Category (from structs.md)
- `{{output.file_path}}` - Output file path
- `{{step_output}}` - Step output text
- `{{duration_ms}}` - Step duration (after_step_succeeds)
- `{{timestamp}}` - Current timestamp
- `{{error_message}}` - Error message (after_step_fails)

**Usage:**
```yaml
hooks:
  after_step_succeeds:
    - log:
        message: "Completed {{step_name}} in {{duration_ms}}ms"
        to_file_path: "logs/{{step_name}}.log"
```

## Hook Patterns for META-v6

### Pattern 1: Logging Pattern

**Purpose:** Track execution progress
**Triggers:** All triggers
**Actions:** Log

**Example:**
```yaml
hooks:
  before_step_starts:
    - log:
        message: "Starting {{step_name}}"
        to_file_path: "logs/workflow.log"
        level: info

  after_step_succeeds:
    - log:
        message: "Completed {{step_name}} successfully"
        to_file_path: "logs/workflow.log"
        level: info

  after_step_fails:
    - log:
        message: "Failed {{step_name}}: {{error_message}}"
        to_file_path: "logs/workflow.log"
        level: error
```

### Pattern 2: Save and Bookmark Pattern

**Purpose:** Persist step outputs and state
**Triggers:** after_step_succeeds
**Actions:** SaveTo, Bookmark

**Example:**
```yaml
hooks:
  after_step_succeeds:
    - save_to:
        file_path: "{{output.file_path}}"
        content: "{{step_output}}"
    - bookmark:
        key: "step_{{step_name}}_result"
        value:
          file_path: "{{output.file_path}}"
          timestamp: "{{timestamp}}"
          size: "{{output.file_size}}"
```

### Pattern 3: Error Handling Pattern

**Purpose:** Handle failures gracefully
**Triggers:** after_step_fails, after_all_retries_exhausted
**Actions:** Log, Fail, Gwt

**Example:**
```yaml
hooks:
  after_step_fails:
    - log:
        message: "Step {{step_name}} failed: {{error_message}}"
        to_file_path: "logs/errors.log"
        level: error
    - gwt:
        - given: "{{error.is_retryable}}"
          when: "== true"
          then:
            - log:
                message: "Will retry {{step_name}}"
          else:
            - fail:
                message: "Non-retryable error in {{step_name}}"

  after_all_retries_exhausted:
    - log:
        message: "Max retries exhausted for {{step_name}}"
        level: critical
    - fail:
        message: "Failed {{step_name}} after 3 attempts"
```

### Pattern 4: Conditional Routing Pattern

**Purpose:** Route to different steps based on conditions
**Triggers:** after_step_succeeds
**Actions:** Gwt, RouteTo

**Example:**
```yaml
hooks:
  after_step_succeeds:
    - gwt:
        - given: "{{output.file_size}}"
          when: "> 0"
          then:
            - route_to: "validate_output"
          else:
            - route_to: "handle_empty_output"
    - log:
        message: "Routed based on output size"
```

### Pattern 5: Shell Command Pattern

**Purpose:** Execute system commands
**Triggers:** after_step_succeeds, after_workflow
**Actions:** Shell

**Example:**
```yaml
hooks:
  after_step_succeeds:
    - shell:
        command: "wc -c {{output.file_path}}"
        bookmark_as: "file_size"
    - gwt:
        - given: "{{file_size}}"
          when: "> 0"
          then:
            - log:
                message: "✅ Output file non-empty ({{file_size}} bytes)"
          else:
            - fail:
                message: "❌ Output file empty"

  after_workflow:
    - shell:
        command: "find docs/benchmarks/outputs -name '*.yml' -exec chmod 644 {} \\;"
        fail_on_error: false
```

### Pattern 6: Accumulation Pattern

**Purpose:** Accumulate results across multiple steps
**Triggers:** after_step_succeeds
**Actions:** AppendTo

**Example:**
```yaml
hooks:
  after_step_succeeds:
    - append_to:
        file_path: "docs/plans/meta-v6/outputs/all-outputs.md"
        content: |
          ## {{step_name}}
          File: {{output.file_path}}
          Size: {{output.file_size}} bytes

          Output:
          {{step_output}}
    - log:
        message: "Appended {{step_name}} output to all-outputs.md"
```

### Pattern 7: Dependency Failure Pattern

**Purpose:** Handle failed dependencies
**Triggers:** on_requires_failed
**Actions:** Log, SkipStep

**Example:**
```yaml
hooks:
  on_requires_failed:
    - log:
        message: "Dependency {{failed_dependency}} failed for {{step_name}}"
        level: warning
    - gwt:
        - given: "{{failed_dependency}}"
          when: "== 'critical_step'"
          then:
            - fail:
                message: "Critical dependency failed, aborting workflow"
          else:
            - skip_step: true
    - log:
        message: "Skipping {{step_name}} due to failed dependency"
```

## GWT Expression Strategy

### Supported GWT Syntax

**Literals:** `true`, `false`, `null`, numbers, strings
**Field Paths:** `{{field}}`, `{{nested.field}}`, `{{array[0].field}}`
**Comparisons:** `==`, `!=`, `>`, `<`, `>=`, `<=`
**Logical:** `&&`, `||`, `!`
**Arithmetic:** `+`, `-`, `*`, `/`
**Parentheses:** `(expression)`

### GWT Clauses Structure

```yaml
gwt:
  - given: "{{field}}"
    when: "> 0"
    then:
      - log:
          message: "Field is positive"
    else:
      - fail:
          message: "Field is non-positive"
```

**Multi-Clause (first-match semantics):**
```yaml
gwt:
  - given: "{{output.type}}"
    when: "== 'json'"
    then:
      - log:
          message: "JSON output"
  - given: "{{output.type}}"
    when: "== 'yaml'"
    then:
      - log:
          message: "YAML output"
  - given: "true"
    when: "true"  # Default case
    then:
      - log:
          message: "Unknown output type"
```

### GWT Best Practices

1. **Avoid String == (Known Bug)**
   - Bug: String equality comparison may be broken
   - Workaround: Use numeric or boolean comparisons
   - Example: Use `when: "!= ''"` instead of `when: "== 'non-empty'"`

2. **Use Numeric Comparisons**
   - Example: `when: "> 0"` (check if non-zero)
   - Example: `when: ">= 5"` (check if ≥5)

3. **Use Boolean Checks**
   - Example: `when: "== true"` (check if true)
   - Example: `when: "!= true"` (check if false)

4. **Combine with Logical Operators**
   - Example: `when: "> 0 && < 100"` (range check)
   - Example: `when: "== true || != ''"` (either true or non-empty)

5. **Use Parentheses for Clarity**
   - Example: `when: "({{a}} > 0) && ({{b}} < 10)"`
   - Example: `when: "({{x}} + {{y}}) > {{z}}"`

## Hook Configuration per SW

### SW1 Hook Config

**Triggers:** before_workflow, before_step_starts, after_step_succeeds, after_workflow

**Strategy:**
- Log workflow start
- Log each step processing
- Save each step output to markdown
- Log workflow completion

**Example:**
```yaml
workflow_hooks:
  before_workflow:
    - log:
        message: "SW1: Task Analysis started"
        to_file_path: "logs/sw1-start.log"

steps:
  - name: analyze_tasks
    hooks:
      before_step_starts:
        - log:
            message: "Processing tasks.md"
      after_step_succeeds:
        - save_to:
            file_path: "docs/plans/meta-v6/outputs/outputs.md"
            content: "{{step_output}}"
        - log:
            message: "Saved outputs.md"

workflow_hooks:
  after_workflow:
    - log:
        message: "SW1: Task Analysis completed"
```

### SW2 Hook Config

**Triggers:** before_workflow, after_step_succeeds, after_workflow

**Strategy:**
- Log workflow start
- Append each category to categories.md
- Log workflow completion

**Example:**
```yaml
workflow_hooks:
  before_workflow:
    - log:
        message: "SW2: Output Structure started"

steps:
  - name: define_categories
    hooks:
      after_step_succeeds:
        - append_to:
            file_path: "docs/plans/meta-v6/outputs/categories.md"
            content: "{{step_output}}"
        - log:
            message: "Appended category to categories.md"

workflow_hooks:
  after_workflow:
    - log:
        message: "SW2: Output Structure completed"
```

### SW3 Hook Config

**Triggers:** before_workflow, after_step_succeeds, after_workflow

**Strategy:**
- Log workflow start
- Save each step definition to structs.md
- Validate DAG (no cycles)
- Log workflow completion

**Example:**
```yaml
workflow_hooks:
  before_workflow:
    - log:
        message: "SW3: Category Mapping started"

steps:
  - name: map_dependencies
    hooks:
      after_step_succeeds:
        - save_to:
            file_path: "docs/plans/meta-v6/outputs/structs.md"
            content: "{{step_output}}"
        - bookmark:
            key: "step_count"
            value: "{{workflow_variables.step_count}}"

workflow_hooks:
  after_workflow:
    - gwt:
        - given: "{{workflow_variables.step_count}}"
          when: "> 0"
          then:
            - log:
                message: "✅ {{workflow_variables.step_count}} steps defined"
          else:
            - fail:
                message: "❌ No steps defined"
```

### SW4 Hook Config

**Triggers:** before_workflow, after_step_succeeds, after_workflow

**Strategy:**
- Log workflow start
- Append each step to YAML skeleton
- Validate provider key
- Validate host type
- Log workflow completion

**Example:**
```yaml
workflow_hooks:
  before_workflow:
    - log:
        message: "SW4: Struct Generation started"

steps:
  - name: generate_yaml
    hooks:
      after_step_succeeds:
        - append_to:
            file_path: "docs/plans/meta-v6/outputs/workflow.yml"
            content: "{{step_output}}"

workflow_hooks:
  after_workflow:
    - shell:
        command: "grep 'providers:' docs/plans/meta-v6/outputs/workflow.yml"
        bookmark_as: "provider_count"
    - gwt:
        - given: "{{provider_count}}"
          when: "== 1"
          then:
            - log:
                message: "✅ Provider section present"
          else:
            - fail:
                message: "❌ Provider section missing or duplicate"
```

### SW5 Hook Config

**Triggers:** before_workflow, after_workflow

**Strategy:**
- Log workflow start
- Validate schema
- Save final validated YAML
- Log workflow completion

**Example:**
```yaml
workflow_hooks:
  before_workflow:
    - log:
        message: "SW5: Workflow Assembly started"

steps:
  - name: validate_and_finalize
    hooks:
      after_step_succeeds:
        - log:
            message: "Workflow validation passed"

workflow_hooks:
  after_workflow:
    - shell:
        command: "grep 'schema_valid=true' docs/benchmarks/outputs/meta-workflow/*/run.log"
        bookmark_as: "schema_valid"
    - gwt:
        - given: "{{schema_valid}}"
          when: "!= ''"
          then:
            - log:
                message: "✅ Schema validation passed"
            - save_to:
                file_path: "docs/plans/meta-v6/outputs/final-workflow.yml"
                source_file_path: "docs/plans/meta-v6/outputs/workflow.yml"
          else:
            - fail:
                message: "❌ Schema validation failed"
```

## Hook Testing Strategy

### Manual Testing

**Method:** Inspect logs for hook execution

**Checklist:**
- [ ] Hook triggers fire (check logs for trigger name)
- [ ] Actions execute (check for expected output)
- [ ] Template interpolation works (check for resolved variables)
- [ ] GWT expressions evaluate (check for correct routing)
- [ ] File I/O works (check for files created/appended)
- [ ] Bookmark storage works (check for bookmark values)
- [ ] Error handling works (check for error logs)

**Example Test:**
```bash
# Check if before_step_starts fired
grep "before_step_starts" docs/benchmarks/outputs/meta-workflow/*/run.log

# Check if Log action wrote file
ls -la logs/sw1-start.log

# Check if SaveTo action created file
ls -la docs/plans/meta-v6/outputs/outputs.md

# Check if GWT expression routed correctly
grep "Routed based on output size" docs/benchmarks/outputs/meta-workflow/*/run.log
```

### Unit Testing (Future)

**After Live Validation Passes:**
- Test hook execution paths
- Test GWT evaluation
- Test template interpolation
- Test file I/O
- Test bookmark storage

**Reference:** `08-TESTING-STRATEGY.md`

## Known Hook Limitations

### L1: during_step_streaming Dead

**Impact:** Cannot test chunk-level hooks
**Workaround:** Use buffered output (stream:false)
**Future Fix:** Architectural change required

### L2: GWT String Comparison Broken

**Impact:** String == may not work correctly
**Workaround:** Use numeric/boolean comparisons
**Future Fix:** Fix GWT evaluator in `src/workflow/hooks/gwt.rs`

### L3: Partial GWT Trigger Wiring

**Impact:** before/after_gwt_evaluates only log
**Workaround:** Accept logging-only for now
**Future Fix:** Pass hook_config through execute_action chain

### L4: Notify Action Stub

**Impact:** No actual notification sent
**Workaround:** Use Log action for notifications
**Future Fix:** Implement actual notification channel

---

**Document Status:** Draft
**Last Updated:** 2026-06-14
**Author:** META-v6 Planning Session
**Review Status:** Pending