# Plan: Simplify Unified Workflow Schema

## ✅ Status: COMPLETE

All 12 phases executed. Schema v2.0 delivered at `opencode/docs/reports/requirements/unified-workflow-schema.yml` (801 lines, down from 1705).

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Remove Pipeline + Replace Output with When Hooks | ✅ Done |
| 2 | Consolidate Loops + Step Type Inference | ✅ Done |
| 3 | Introduce `when` / `lifecycle_hooks` Schema | ✅ Done |
| 4 | Remove Advanced Scheduling | ✅ Done |
| 5 | Remove Adaptive Behavior | ✅ Done |
| 6 | Tool Permissions + Absorb AI Operations | ✅ Done |
| 7 | Remove .glyphnova References + Simplify Refs | ✅ Done |
| 8 | Flatten Output Fields | ✅ Done |
| 9 | Documentation Cleanup + Thresholds | ✅ Done |
| 10 | Move RAG Under Memory + Remove `enabled` Keys | ✅ Done |
| 11 | Preserved Sections (No Changes) | ✅ Done |
| 12 | Final Assembly and Validation | ✅ Done |

Committed: `ac0a645` — ref(schema): Rebuild unified workflow schema to v2.0

## Scope
This plan ONLY edits the file: `opencode/docs/reports/requirements/unified-workflow-schema.yml`
No other files are modified by this plan. A future plan will handle updating example workflows.

## Objectives
1. Human-readable: natural language key names, skimmable, non-technical person can understand
2. Extensible: easy to add new things later, backwards compatible if major change needed
3. Simple: deduplicated, holdable in your head at once, fits together logically
4. Preserve ALL features from current schema

---

## Phase 1: Remove Pipeline Array Format + Replace Output with When Hooks

### 1A: Remove Pipeline
Remove the `pipeline:` array format (lines 499-599). All workflow definitions use `agentic_workflow:` with named steps only. No features are lost — every pipeline array feature maps to agentic_workflow named steps.

### 1B: Replace Output with When Hooks + Default save_to Behavior

Remove the `output:` section on steps entirely. Output is achieved through `when` hook actions. The step defines what it produces (prompt + generative_entity), and hooks define where the result goes.

**Key design: `save_to` defaults and substructure**

When a step completes, its output is automatically saved to `step_name.output` — a variable named `output` scoped to the step name. This variable is either a **string** (raw LLM text) or an **object** (parsed YAML/structured data):

```yaml
# Auto-created after step "analyze_code" completes:
#   {{step.analyze_code.output}} — string or object
#
# If the LLM returns parseable YAML/structured content:
analyze_code:
  output:
    quality_score: 0.92
    issues: ["missing error handling", "unused imports"]
    suggestions: ["Add try/catch blocks", "Remove dead code"]
#
# If the LLM returns plain text:
analyze_code:
  output: "The code analysis reveals several issues..."
#
# Reference: {{step.analyze_code.output}}
# For structured: {{step.analyze_code.output.quality_score}}
```

**`save_to` and `append_to` are the SAME action** — both append. For variables, appending to a new name = starting from empty string. For files, appending = adding to end of file content.

```yaml
# save_to and append_to are IDENTICAL — both append:
save_to: my_var           # appends to my_var (starts as empty string if new)
append_to: my_var         # exact same behavior
save_to: "./output.log"   # appends to file
append_to: "./output.log" # exact same behavior
```

**`then` and top-level work identically** — you can put actions at the hook level directly OR inside a `then` block. They are the same thing:

```yaml
# These are IDENTICAL — use whichever reads better:

# Option A: actions directly on the hook (flat)
when:
  after_step_succeeds:
    save_to: custom_variable_name    # override default step-name variable
    append_to: "./workspace/output/results.json"
    log:
      to_file_path: "./workspace/logs/steps.log"
      event_fields: [step_name, duration_ms]

# Option B: actions inside then (explicit)
when:
  after_step_succeeds:
    then:
      save_to: custom_variable_name
      append_to: "./workspace/output/results.json"
    log:
      to_file_path: "./workspace/logs/steps.log"
      event_fields: [step_name, duration_ms]
```

### BEFORE (Current Schema):
```yaml
steps:
  analyze_code:
    type: agent
    generative_entity: "${models.primary}"
    prompt: "Analyze the code"
    output:
      save_to: analysis_result
      format: json
      fields: [quality_score, issues, suggestions]
      file_output:
        enabled: true
        path: /workspace/output/analysis.json
        format: json
      console:
        enabled: false
        prefix: "[Analyze] "
```

### AFTER (New Schema):
```yaml
steps:
  analyze_code:
    generative_entity: "${models.primary}"    # agent step (has generative_entity + prompt)
    prompt: "Analyze the code"
    # No output section needed — "analyze_code" variable auto-created with substructure:
    #   analyze_code.raw_text, analyze_code.response, analyze_code.quality_score, etc.
    when:
      after_step_succeeds:
        append_to: "./workspace/output/analysis.json"   # write to file
        log:
          to_file_path: "./workspace/logs/steps.log"
          event_fields: [step_name, quality_score, duration_ms]

      after_step_fails:
        log:
          to_file_path: "./workspace/logs/errors.log"
          event_fields: [timestamp, step_name, error.type, error.message]
```

### More Extensive Before/After Examples:

**BEFORE — Complex step with output + file + console:**
```yaml
steps:
  step_4_generate_final_output:
    type: agent
    generative_entity: "${models.primary}"
    prompt: "Generate final report"
    output:
      save_to: final_report
      format: json
      fields:
        - workflow_status
        - step_status
        - execution_outcome
        - file_operations
        - validation_results
        - metrics
        - retry_branching
        - provenance
      file_output:
        enabled: true
        path: /workspace/output/final_report.json
        format: json
        encoding: utf-8
      console:
        enabled: true
        prefix: "[Report] "
        color: blue
```

**AFTER — Same step using when hooks:**
```yaml
steps:
  generate_final_output:
    generative_entity: "${models.primary}"
    prompt: "Generate final report"
    # "generate_final_output" variable auto-created with:
    #   generate_final_output.raw_text
    #   generate_final_output.response (parsed JSON)
    #   generate_final_output.metadata.model, .duration_ms, .tokens_used
    when:
      after_step_succeeds:
        - append_to: "./workspace/output/final_report.json"
          # append_to writes the response to the file path
        - log:
            to_file_path: "./workspace/logs/summary.log"
            event_fields: [workflow_status, validation_results, metrics]
            level: info
```

**BEFORE — Step with save_to only (simple case):**
```yaml
steps:
  analyze:
    type: agent
    generative_entity: "${models.primary}"
    prompt: "Analyze"
    output:
      save_to: analysis_data
      format: json
```

**AFTER — Default save_to (even simpler):**
```yaml
steps:
  analyze:
    generative_entity: "${models.primary}"
    prompt: "Analyze"
    # "analyze" variable auto-created — no save_to needed
    # Reference later as: {{step.analyze.response}} or {{step.analyze.raw_text}}
```

This replaces:
- `output.save_to` → default step-name variable OR `save_to:` in hook for custom name
- `output.format` → handled by JSON substructure parsing automatically
- `output.fields` → `log.event_fields` in hooks
- `output.file_output` → `append_to:` or `save_to:` with a file path in hooks
- `output.console` → `log.to_console: true` (future MVP extension)

### Validation
- [ ] New schema parses as valid YAML
- [ ] Every pipeline feature present in agentic_workflow format
- [ ] Output section fully removed from steps
- [ ] Default step-name variables auto-created with substructure (raw_text, response, metadata)
- [ ] `then` and top-level hook actions work identically
- [ ] `save_to` / `append_to` work as both variable names and file paths

---

## Phase 2: Consolidate Loop Configurations + Step Type Inference (No `type` Property)

### 2A: Loop Consolidation
Flatten `loop.type + X_config` pattern into named subkeys under `loops`. Keep all 5 types: `count`, `time`, `validation`, `retry`, `infinite`.

### Competing Loop Configurations
When multiple loop types are configured simultaneously, the engine applies the **minimum constraint** principle — it tries to satisfy ALL active loop configs, stopping at whichever constraint hits first:

```yaml
loops:
  count:
    max_iterations: 20
  time:
    duration_seconds: 20
```
**Resolution**: Run as many iterations as possible within 20 seconds, up to 20 max. If 20 seconds elapses at iteration 15, stop at 15. The tighter constraint always wins.

General rule: **min(active_constraints)** — the engine enforces all present loop configs and stops at the first constraint that triggers.

### 2B: Remove `type` Property — Step Type Inference

Steps no longer need a `type:` property. The engine infers the step type from what keys are present:

| Step Type | How Inferred | Required Keys |
|-----------|-------------|---------------|
| **Agent/LLM** | `generative_entity` + `prompt` both present | `generative_entity`, `prompt` |
| **Tool** | `tool` key present | `tool` |
| **Control** | Only `when` present (no generative_entity, no prompt, no tool) | `when` |
| **Sub-workflow** | `sub_workflow` key present | `sub_workflow` |
| **Loop** | `loop` key present | `loop` |

```yaml
# Agent step — inferred from generative_entity + prompt
analyze_code:
  generative_entity: "${models.primary}"
  prompt: "Analyze the code"

# Tool step — inferred from tool key
read_config:
  tool: file_read
  input:
    file_path: "./workspace/config.yml"

# Control step — inferred from only having when
check_results:
  when:
    gwt:
      - given: "result.quality_score >= 0.9"
        then: { route_to: step_success }
      - given: "result.quality_score < 0.9"
        then: { route_to: step_retry }

# Sub-workflow step — inferred from sub_workflow key
run_validation:
  sub_workflow: validation_workflow
  input:
    target: "{{step.analyze_code.response}}"

# Loop step — inferred from loop key
retry_until_valid:
  loop:
    count:
      max_iterations: 5
      stop_condition: "result.is_valid == true"
```

### 2C: Sub-Workflow Top-Level Configuration

Sub-workflows need a top-level configuration similar to `models` or `agents`. This lives at the top level of the workflow file:

```yaml
# ── SUB-WORKFLOW DEFINITIONS ─────────────────────
sub_workflows:
  # Option 1: String value = path to external YAML file
  validation_workflow: "./workflows/validation.yml"
  
  # Option 2: Inline object = full workflow schema
  fix_code_workflow:
    steps:
      identify_issues:
        generative_entity: "${models.primary}"
        prompt: "Identify code issues in: {{input.code}}"
      apply_fix:
        generative_entity: "${models.primary}"
        prompt: "Fix these issues: {{step.identify_issues.response}}"
  
  # Option 3: Path with overrides
  code_review_workflow:
    path: "./workflows/code-review.yml"
    default_inputs:
      strict_mode: true
```

Then reference in steps:
```yaml
steps:
  run_validation:
    sub_workflow: validation_workflow         # references sub_workflows key above
    input:
      target: "{{step.analyze_code.response}}"
    when:
      after_sub_workflow_succeeds:
        save_to: validation_results
```

### 2D: Retry vs Loop — Distinction and Examples

**Retry** = re-attempt the SAME step when it fails. Has validation criteria (deterministic or non-deterministic) evaluated to a boolean. Specifications around how many times and how logging functions.

**Loop** = iterate a DIFFERENT action multiple times. Can iterate sub-workflows through a list of inputs, do repeated parallel behavior, validation loop structures.

```yaml
# ── RETRY EXAMPLE ──────────────────────────────────
# Scenario: Step fails, re-run same step with adjustment
steps:
  generate_code:
    generative_entity: "${models.primary}"
    prompt: "Generate Rust code for: {{input.spec}}"
    retry:                              # presence = retry enabled
      max_attempts: 3                   # re-run this step up to 3 times
      backoff: exponential
      delay_ms: 1000
      condition: "error.is_retryable == true"   # only retry if condition met
      adjustment_strategy: loosen_tolerance
      tolerance_adjustment: 0.15
    when:
      after_retry_attempt_fails:          # hook fires on EACH failed retry
        log:
          to_file_path: "./workspace/logs/retries.log"
          event_fields: [attempt_number, error_type, tolerance_current]
      after_all_retries_exhausted:        # hook fires when all retries used up
        log:
          to_file_path: "./workspace/logs/fatal.log"
          level: error

# ── LOOP EXAMPLE 1: Validation Loop ─────────────────
# Scenario: Generate code, validate, fix issues, re-validate until passing
steps:
  validation_loop:
    loop:
      validation:
        tolerance: 0.05
        max_iterations: 5
        exact_criteria:
          - metric: quality_score
            operator: ">="
            target: 0.90
    sub_workflow: build_and_validate_workflow
    # Each loop iteration runs the full build_and_validate_workflow
    # Loop stops when validation criteria met OR max_iterations hit

# ── LOOP EXAMPLE 2: List Iteration ──────────────────
# Scenario: Iterate through a list of files, run same workflow for each
steps:
  process_all_files:
    loop:
      count:
        max_iterations: "{{file_list.length}}"    # iterate over list
        iteration_variable: current_file
    sub_workflow: process_single_file_workflow
    input:
      file_path: "{{loop.current_file}}"

# ── LOOP EXAMPLE 3: Parallel Repeated Behavior ──────
# Scenario: Run multiple analyses in parallel, collect results
steps:
  parallel_analysis:
    loop:
      count:
        max_iterations: 5
    sub_workflow: single_analysis_workflow
    input:
      variation: "{{loop.iteration_index}}"
    when:
      after_step_succeeds:
        append_to: "./workspace/output/all_analyses.json"

# ── RETRY vs LOOP KEY DISTINCTION ───────────────────
# RETRY: Same step, re-attempt on failure
#   - Has backoff, delay, condition
#   - Fires retry-specific hooks (after_retry_attempt_fails, etc.)
#   - Adjustment strategies (loosen_tolerance, etc.)
#   - The step definition doesn't change between attempts
#
# LOOP: Different execution per iteration
#   - Has iteration counting, time limits, validation convergence
#   - Can iterate sub-workflows, tool calls, or agent calls
#   - Each iteration can have different inputs (list iteration)
#   - Stops on convergence criteria, not failure conditions
#   - The iteration variable changes between executions
```

### Validation
- [ ] All 5 loop types still configurable
- [ ] Competing loop configs resolve by min-constraint principle
- [ ] `type` property removed from steps
- [ ] Step types correctly inferred from keys present
- [ ] Sub-workflow top-level configuration works (path, inline, path+overrides)
- [ ] Retry and Loop distinction clear and non-overlapping
- [ ] New schema parses as valid YAML

---

## Phase 3: Introduce `when` / `lifecycle_hooks` Schema (Core Redesign)

### What changes
Replace ALL hook sections with a single unified `when:` schema. `lifecycle_hooks` is a required alias. Logging is ONLY through hooks.

### Flat Hook Notation (Preferred)
Both nested and flat forms are valid. Flat form joins path segments with underscores:

```yaml
# These are IDENTICAL — use whichever reads better:
when:
  before_step_starts: { log: { to_file_path: "./logs/steps.log" } }
  # is the same as:
  before:
    step:
      starts: { log: { to_file_path: "./logs/steps.log" } }
```

### Hook value: single object or array of objects
```yaml
when:
  after_step_succeeds:
    # Single object
    log: { to_file_path: "./logs/steps.log", event_fields: [step_name, duration_ms] }

  after_execution_fails:
    # Array of objects — conditional branching
    - given: "error.is_retryable == true"
      log: { to_file_path: "./logs/retries.log", level: warning }
      then: { route_to: step_retry }
    - given: "error.is_retryable == false"
      log: { to_file_path: "./logs/fatal.log", level: error }
      then: { route_to: step_abort }
```

### GWT (Given/When/Then) Integration

`gwt` (alias: `given_when_then`) lives inside `when:` at the top level for abstract LLM/agent routing, and inside individual hooks for deterministic conditional logic.

```yaml
# GWT at top level of when = abstract, LLM/agent-evaluated routing
steps:
  analyze_results:
    generative_entity: "${models.primary}"
    prompt: "Analyze the codebase"
    when:
      gwt:
        - given: "code quality score is below 0.9"
          when: "validation results show compilation errors"
          then: { route_to: step_fix_code }

        - given: "all validations pass"
          then: { route_to: step_success }

      after_step_succeeds:
        log:
          to_file_path: "./workspace/logs/steps.log"
          event_fields: [step_name, quality_score, duration_ms]
```

**GWT inside a hook = deterministic boolean**:
```yaml
when:
  after_step_succeeds:
    - gwt:
        - given: "result.quality_score >= 0.9"
          then: { route_to: step_next_phase }
        - given: "result.quality_score >= 0.7"
          then: { route_to: step_retry_with_looser_tolerance }
        - given: "true"  # fallback
          then: { route_to: step_abort }
      log:
        to_file_path: "./workspace/logs/routing.log"
        event_fields: [decision, quality_score]
```

**Control flow steps use GWT directly** — control steps have `when.gwt` as their primary mechanism:

```yaml
# Control step — routing logic via gwt (not generic after_step_succeeds)
check_results:
  when:
    gwt:
      - given: "quality_score >= 0.9"
        then: { route_to: step_success }
      - given: "quality_score >= 0.7"
        then: { route_to: step_retry }
      - given: "true"
        then: { route_to: step_abort }
    after_gwt_evaluates:              # hook fires after gwt routing decision
      log:
        to_file_path: "./workspace/logs/routing.log"
        event_fields: [decision, quality_score, route_target]
```

### Dependency Notation and Routing

Steps declare dependencies and routing:

```yaml
steps:
  step_1_initialize:
    generative_entity: "${models.primary}"
    prompt: "Initialize the system"
    when:
      after_step_succeeds:
        then: { route_to: [step_2_analyze, step_3_parallel_check] }

  step_2_analyze:
    depends_on: [step_1_initialize]
    parallel_group: analysis_group
    generative_entity: "${models.primary}"

  step_3_parallel_check:
    depends_on: [step_1_initialize]
    parallel_group: analysis_group

  step_4_finalize:
    depends_on: [step_2_analyze, step_3_parallel_check]
    generative_entity: "${models.primary}"
```

### Dependency Relationship Labels + Control Flow

Standard relationships inspired by Scrum/agile systems and workflow engines:

| Relationship | Key | Meaning |
|-------------|-----|---------|
| **finish_to_start** | `depends_on` (default) | B starts after A finishes |
| **start_to_start** | `starts_with` | B starts when A starts |
| **finish_to_finish** | `finishes_with` | B finishes when A finishes |
| **start_to_finish** | `triggers_close` | B finishing triggers A to start |

### `requires` — AND Logic (ALL Must Succeed)

`requires` enforces that ALL listed steps succeed with their conditions met:

```yaml
steps:
  deploy_code:
    requires:
      - step: validate_code
        condition: "result.all_tests_passed == true"
      - step: code_review
        condition: "result.approved == true"
    when:
      on_requires_failed:                        # fires when ANY require condition fails
        - given: "failed_step == 'validate_code'"
          then: { route_to: fix_tests }
        - given: "failed_step == 'code_review'"
          then: { route_to: request_review }
```

### `on_requires_failed` — Hook for Dependency Failures

`on_requires_failed` is a `when` hook that fires when a `requires` condition is not met. It supports:
- **String form**: simple action `"skip"` or `"fail"`
- **Object form**: full hook object with `if`/`given` and any response actions

```yaml
# Simple string form:
when:
  on_requires_failed: skip          # just skip this step

# Object form with conditions:
when:
  on_requires_failed:
    - given: "failure_count > 3"
      then: { fail: "Too many dependency failures" }
    - given: "failed_step == 'validate_code'"
      then: { route_to: fix_code }
      log:
        to_file_path: "./workspace/logs/dep_failures.log"
        event_fields: [failed_step, condition, failure_count]
```

### Control Flow Defaults — Requires with Simple String References

`requires` can accept a mix of strings and objects. A bare string = "this step must not fail". An object = "this step must meet condition". This keeps parallel-mode configs concise:

```yaml
# Simple string form — step just needs to not fail:
deploy_code:
  requires: [validate_code, code_review]    # both must not fail
  # equivalent to:
  requires:
    - step: validate_code
    - step: code_review

# Mixed form — some with conditions, some without:
deploy_code:
  requires:
    - validate_code                          # just don't fail
    - step: code_review
      condition: "result.approved == true"   # must meet condition

# Default behavior for parallel mode:
# If step is in a parallel_group and references another step via requires/depends_on,
# the default is: "that step completed without failing" — no condition needed.
```

### Skip Actions — Granular Skip Control

`skip` is a general action with specific variants for different scopes:

```yaml
then:
  skip_step: true                    # skip just this step, continue workflow
  skip_loop: true                    # skip current loop iteration (not the whole loop)
  skip_sub_workflow: true            # skip current sub-workflow execution
  skip_remaining: true               # skip all remaining steps in workflow

# skip_loop example — skip current iteration, continue looping:
when:
  after_step_fails:
    - given: "error.type == 'file_not_found'"
      then: { skip_loop: true }      # skip this file, continue with next iteration

# skip_step example — skip this step, let dependents decide:
when:
  on_requires_failed:
    then: { skip_step: true }
```

### `route_to` — Kept as-is

`route_to` stays as the routing key. No rename needed.

### `then` is Optional in Hooks
When a hook only has `log:` or `save_to:`, no `then` wrapper is needed:
```yaml
when:
  after_step_succeeds:
    # No then needed — just direct actions
    save_to: my_variable
    log:
      to_file_path: "./logs/steps.log"
      event_fields: [step_name]
```

### Hook Timing: before / after / during

Hooks have three timing categories based on when they fire relative to an event:

| Timing | Meaning | Results Available | Use Case |
|--------|---------|-------------------|----------|
| **before** | Fires BEFORE the event happens | No results yet | Setup, validation, resource checks |
| **after** | Fires AFTER the event completes | Full results available | Logging, routing, save_to, branching |
| **during** | Fires AS results stream in | Partial/streaming results | Real-time logging, progress updates, streaming to file |

```yaml
when:
  # before = no results, used for setup
  before_step_starts:
    log:
      to_file_path: "./workspace/logs/steps.log"
      event_fields: [step_name, timestamp]
  
  # during = streaming results as they arrive
  during_step_streaming:
    log:
      to_file_path: "./workspace/logs/streaming.log"
      event_fields: [step_name, chunk_text, tokens_so_far]
      # Each chunk of the LLM response triggers this hook
  
  # after = complete results, used for routing/saving
  after_step_succeeds:
    save_to: analysis_output
    log:
      to_file_path: "./workspace/logs/completed.log"
      event_fields: [step_name, duration_ms, quality_score]
```

**Control flow step hooks** use step-specific timing:

```yaml
check_results:
  when:
    before_gwt_evaluates:              # before routing decision
      log: { to_file_path: "./logs/gwt.log", event_fields: [input_data] }
    during_gwt_evaluates:              # streaming LLM evaluation (for agent-evaluated gwt)
      log: { to_file_path: "./logs/gwt_stream.log", event_fields: [reasoning_chunk] }
    after_gwt_evaluates:               # after routing decision made
      log: { to_file_path: "./logs/routing.log", event_fields: [decision, target_step] }
    gwt:
      - given: "quality >= 0.9"
        then: { route_to: step_success }
```

### Remove `input_variables` — Use `inputs` at Workflow Level

Remove `input_variables` from step definitions. Variable binding happens at the top level via the new `inputs:` key on `agentic_workflow`. Steps reference variables via `prompt:` interpolation.

```yaml
agentic_workflow:
  # Workflow-level inputs with defaults
  inputs:
    workspace_path:                          # name = key, type defaults to string
      description: "Root workspace directory"
      default: "/workspace"
    quality_threshold:
      description: "Minimum quality score"
      type: number                           # string (default) | number | boolean | object | list<string> | list<number> | list<object>
      default: 0.90
    strict_mode:
      description: "Enable strict validation"
      type: boolean
      default: false
    target_files:
      description: "Files to process"
      type: list<string>
      default: []
    
    user_inputs: { ... }                     # interactive user inputs (existing)
  
  steps:
    analyze:
      generative_entity: "${models.primary}"
      prompt: |
        Analyze code in {{inputs.workspace_path}}
        with quality threshold {{inputs.quality_threshold}}
        targeting files: {{inputs.target_files}}
```

**Sub-workflow `input` is fine** — sub-workflows take explicit inputs from the calling step:

```yaml
steps:
  run_validation:
    sub_workflow: validation_workflow
    input:                                   # sub-workflow takes inputs
      target: "{{step.analyze_code.output}}"
      strict: "{{inputs.strict_mode}}"
```

### User Inputs at Step Level

User inputs can be specified directly on a step without going through `inputs_by_step`:

```yaml
# Simple step-level user input:
steps:
  confirm_deploy:
    prompt: "Deploy to {{inputs.environment}}?"
    user_input:                              # direct step-level input
      type: confirm
      message: "About to deploy. Continue?"
      default: true
    
  get_target:
    user_input:                              # step can be just a user input
      type: text_line
      label: "Target branch name"
      default: "main"
```

### Default Backoff: Linear for Step Retry

Step retry default `backoff` is `linear` (not `exponential`):

```yaml
agentic_workflow:
  retry:
    step:
      backoff: linear              # DEFAULT for steps
      delay_ms: 1000
    # workflow-level can be different
    backoff: exponential           # DEFAULT for workflow-level retry
```

### Allocation → ram_allocation

Rename `allocation` keys to `ram_allocation` for clarity:

```yaml
# BEFORE:
allocation:
  strategy: adaptive

# AFTER:
ram_allocation:
  strategy: adaptive               # static | dynamic | adaptive
```

### System Operations — Robust Subschema

`system_operations` under `tool_permissions` gets a more robust subschema:

```yaml
tool_permissions:
  system_operations:
    process_management:
      disabled: true                        # off by default
      # If enabled:
      allowed_signals: [SIGTERM, SIGINT]    # which signals can be sent
      max_cpu_percent: 50                   # CPU usage cap
      max_memory_mb: 1024                   # memory cap
      monitored_processes: [node, python]   # which process names to allow
    
    network_operations:
      disabled: true
      # If enabled:
      allowed_ports: [8080, 3000]
      allowed_protocols: [tcp, udp]
      max_bandwidth_mbps: 100
      blocked_hosts: ["*.internal", "169.254.*"]
    
    file_system_mount:
      disabled: true
      # If enabled:
      allowed_mount_points: ["/tmp", "/workspace"]
      mount_options: ["ro", "noexec"]
    
    environment_variables:
      read: { allowed_patterns: ["APP_*", "PATH"] }
      write: { disabled: true }
    
    kernel_module_management:
      disabled: true
    
    service_management:
      disabled: true
      # If enabled:
      allowed_services: [docker, nginx]
      allowed_actions: [start, stop, restart, status]
```

### Semantic Hierarchy: Workflow → Agentic Workflow → Step

Configuration keys follow a three-level semantic hierarchy:

**Level 1: Top-level (workflow-wide)** — Settings that apply to the entire workflow lifecycle, infrastructure, and external integrations.

**Level 2: `agentic_workflow` (per-step defaults)** — Settings that provide defaults for all steps. Individual steps can override.

**Level 3: Step-level** — Settings specific to one step. Overrides Level 2 defaults.

```yaml
# ── SEMANTIC HIERARCHY ──────────────────────────
# Top-level = workflow infrastructure
# agentic_workflow = per-step defaults
# step = step-specific overrides

workspace:                    # Level 1 ONLY — workflow-wide physical environment
  root_path: /workspace
  directories: { ... }

providers:                    # Level 1 ONLY — infrastructure connections
  lmstudio: { ... }

models:                       # Level 1 ONLY — model definitions
  primary: { ... }

sub_workflows:                # Level 1 ONLY — sub-workflow definitions
  validation: { ... }

agentic_workflow:             # Level 2 = per-step defaults live here
  retry:                      # Level 2 — default retry for all steps
    step:
      max_attempts: 3
      backoff: linear         # DEFAULT: linear for steps
  
  when:                       # Level 2 — default hooks for all steps
    after_step_fails:
      log: { to_file_path: "./logs/errors.log" }
  
  steps:
    my_step:                  # Level 3 = step-specific overrides
      retry:                  # overrides Level 2 retry.step
        max_attempts: 5
      when:                   # ADDS to Level 2 hooks (merged, not replaced)
        after_step_succeeds:
          save_to: my_output
```

**Common schemas that appear at multiple levels:**

| Schema | Level 1 (Workflow) | Level 2 (agentic_workflow) | Level 3 (Step) | Notes |
|--------|-------------------|---------------------------|----------------|-------|
| `retry` | ❌ | ✅ `retry.step:` | ✅ `retry:` | Level 2 sets defaults, Level 3 overrides |
| `when` (hooks) | ❌ | ✅ default hooks | ✅ step hooks | Step hooks MERGE with agentic_workflow hooks |
| `timeout` | ✅ workflow-level | ❌ | ✅ per-step timeout | Different scope at each level |
| `workspace` | ✅ physical dirs | ❌ | ❌ | Only at top — steps reference via `${workspace.*}` |
| `tool_permissions` | ✅ global baseline | ❌ | ✅ `tool:` key | Step tools cannot exceed global permissions |
| `memory` (model) | ✅ in `workflow_execution_strategy` | ❌ | ❌ | Engine-level memory management |
| `memory` (RAG) | ✅ `memory.rag` | ❌ | ❌ | Global RAG config |
| `providers` | ✅ | ❌ | ❌ | Infrastructure only |
| `models` | ✅ definitions | ❌ | ✅ `generative_entity` reference | Steps reference, don't define |
| `sub_workflows` | ✅ definitions | ❌ | ✅ `sub_workflow:` reference | Steps reference, don't define |
| `ram_allocation` | ✅ `workflow_execution_strategy` | ❌ | ❌ | Engine-level resource management |
| `parallel_group` | ❌ | ❌ | ✅ | Only on steps |
| `depends_on` | ❌ | ❌ | ✅ | Only on steps |
| `requires` | ❌ | ❌ | ✅ | Only on steps |
| `user_input` | ✅ `agentic_workflow.user_inputs` | ❌ | ✅ direct on step | Both workflow and step level |
| `inputs` | ✅ `agentic_workflow.inputs` | ❌ | ❌ | Workflow-level defaults only |
| `prompt` | ❌ | ❌ | ✅ | Only on steps |
| `format` | ❌ | ❌ | ✅ in `then:` | Optional per-action override |
| `checkpoint` | ✅ `workflow_execution_strategy.checkpointing` | ❌ | ✅ in `then:` | Global policy + per-action trigger |

```yaml
# Default behavior — structured output saved as YAML:
step_name:
  output:
    quality_score: 0.92
    issues: ["missing error handling"]

# Plain text stays as string:
step_name:
  output: "The code analysis reveals several issues..."
```

`format` is optional. When present, it forces explicit conversion:
```yaml
then:
  save_to: report_data
  format: markdown   # force markdown output
```

**Decision: Default format is YAML for structured data, string for unstructured. `format` key is optional for explicit conversion only.**

### `checkpoint` — Explanation and Alias Suggestions

A **checkpoint** in the Agent SDK is a snapshot of the entire workflow execution state at a specific point in time. It captures:
- All step outputs completed so far
- Current step being executed
- Model states (loaded/unloaded)
- Variable values
- Loop iteration counts
- Retry counts and history

This enables **time-travel debugging** — you can restore a workflow to any previous checkpoint and re-execute from that point.

**8 alias suggestions for `checkpoint`:**
1. `snapshot` — `then: { snapshot: true }`
2. **`bookmark`** ← **CHOSEN ALIAS** — `then: { bookmark: true }`
3. `save_state` — `then: { save_state: true }`
4. `preserve` — `then: { preserve: true }`
5. `freeze` — `then: { freeze: true }`
6. `capture` — `then: { capture: true }`
7. `milestone` — `then: { milestone: true }`
8. `pin` — `then: { pin: true }`

**`bookmark` is an alias for `checkpoint`.** `checkpoint` is the canonical name. `bookmark` and `save_state` are aliases that parse to the same internal action.

```yaml
# All three are identical:
then: { checkpoint: true }
then: { bookmark: true }
then: { save_state: true }
```

### `save_to` / `append_to` / `set_variable` — Aliases

These three keys are **aliases** for the same underlying action:

```yaml
# All of these do the same thing — store a value:
then:
  save_to: variable_name        # Save step output to a variable
  # OR
  append_to: variable_name      # Append step output to variable (same as save_to for new vars)
  # OR
  set_variable: { key: value }  # Explicitly set a variable (same mechanism)

# They also accept file paths — if value starts with "/" or "./" it writes to file:
then:
  save_to: "./workspace/output/results.json"     # writes to file
  append_to: "./workspace/output/append.log"     # appends to file
  save_to: my_variable                           # saves to workflow variable

# Can be a single string OR an array:
then:
  save_to:
    - "./workspace/output/results.json"    # write to file
    - analysis_data                        # AND save to variable
```

### `fail` and `skip_remaining` Actions

```yaml
# fail — explicitly fail the step or workflow with a reason
when:
  after_step_fails:
    - given: "error.type == 'out_of_memory'"
      then:
        fail: "Insufficient memory to continue. Allocate more VRAM."

# skip_remaining — skip all remaining steps in the workflow
when:
  after_step_succeeds:
    - given: "result.skip_remaining == true"
      then:
        skip_remaining: true
        log:
          to_file_path: "./workspace/logs/skipped.log"
          event_fields: [skipped_steps, reason]

# Combined example: conditional abort with logging
when:
  after_step_fails:
    - gwt:
        - given: "error.is_fatal == true"
          then:
            fail: "Fatal error: {{error.message}}"
        - given: "error.is_fatal == false"
          then: { route_to: step_retry }
    - then:
        skip_remaining: true
        fail: "Maximum error threshold exceeded"
```

### `notify` — Future Extension Point
Notifications require fleshing out transport, format, and routing. For MVP, this is a stub that logs. Full implementation deferred:

```yaml
then:
  notify:
    message: "Step {{step_name}} completed"
    level: info
    channels: [log]    # MVP: only log channel. Future: email, webhook, etc.
```

### All Possible `then` Clauses (Complete List)
```yaml
then:
  route_to: step_name                    # route to a named step
  route_to: [step_a, step_b]             # route to multiple steps (parallel)
  route_to: sub_workflow_name            # route to a sub-workflow
  save_to: variable_name                 # append output to variable (alias: append_to)
  save_to: "./path/to/file.yml"          # append output to file path
  save_to: [var_name, "./path.yml"]      # save to both variable and file
  format: yaml                           # optional explicit format conversion (default: yaml)
  bookmark: true                         # create a bookmark/checkpoint (alias: checkpoint, save_state)
  notify: { message: "Done" }            # send notification (MVP: log only)
  fail: "reason message"                 # explicitly fail the step/workflow
  skip_step: true                        # skip just this step
  skip_loop: true                        # skip current loop iteration (not the whole loop)
  skip_sub_workflow: true                # skip current sub-workflow execution
  skip_remaining: true                   # skip all remaining steps in workflow
```

### Retry — Flat at Workflow Level with Step Subkey
Workflow retry is flat (workflow-level keys at top). `step:` subkey provides global step retry defaults:

```yaml
agentic_workflow:
  retry:  # presence = retry enabled
    # Flat keys = workflow retry behavior
    max_attempts: 10
    backoff: exponential
    delay_ms: 5000
    level: workflow_restart

    # step subkey = global defaults for ALL steps
    step:
      max_attempts: 3
      backoff: exponential
      delay_ms: 1000
      base_ms: 1000
      max_ms: 30000
      jitter: 0.2
      level: step_restart
      adjustment_strategy: loosen_tolerance
      tolerance_adjustment: 0.15
      checkpoint_after_retry: true
```

Step-level retry overrides the `step:` defaults:
```yaml
steps:
  my_step:
    retry:
      max_attempts: 5
      backoff: linear
      delay_ms: 2000
```

### Validation
- [ ] All existing hook events covered
- [ ] Both flat and nested hook notation parse
- [ ] Logging only through hooks
- [ ] GWT works at top-level (LLM) and inside hooks (deterministic)
- [ ] `when` and `lifecycle_hooks` both work as keys
- [ ] `gwt` and `given_when_then` both work as keys (NOT gtw)
- [ ] Workflow retry is flat with `step:` subkey
- [ ] `then` is optional in hooks
- [ ] `save_to` / `append_to` / `set_variable` are aliases
- [ ] `format` is optional (default: YAML for structured, string for unstructured)
- [ ] `fail` and `skip_remaining` actions work
- [ ] Dependency relationship labels are robust
- [ ] Hook timing: before (no results), after (complete results), during (streaming)
- [ ] Control flow steps use `when.gwt` as primary mechanism
- [ ] `input_variables` removed — use `inputs` at agentic_workflow level
- [ ] User inputs can be at step level directly
- [ ] Step retry default backoff = linear
- [ ] `allocation` renamed to `ram_allocation`
- [ ] `system_operations` has robust subschema
- [ ] Semantic hierarchy documented (workflow → agentic_workflow → step)
- [ ] New schema parses as valid YAML

---

## Phase 4: Remove Advanced Scheduling

Remove `workflow_execution_strategy.advanced_scheduling`. Scheduling expressed through workflow structure (step ordering, parallel_group, depends_on, route_to).

### Validation
- [ ] No features lost
- [ ] New schema parses as valid YAML

---

## Phase 5: Remove Adaptive Behavior (Moved to Model Router)

Remove ALL adaptive behavior features from this schema. **Dynamic scaling and adaptive resource management are handled by the model router project**, not the YAML-to-Rust AgentSDK. The AgentSDK is a straight execution engine — no adaptive features.

The following adaptive features are REMOVED from `unified-workflow-schema.yml` and should be moved to the model router repository documentation:
- Resource monitoring (cpu, memory, gpu, io, network)
- Memory pressure handling (throttle, swap, fail_fast)
- Concurrency adjustment (auto-scale, scale_up_down)
- Performance-based scheduling

**The model router can override sub-workflow and workflow model selection for dynamic scaling.** The model router project handles all scaling concerns and can override the execution hierarchy. This schema only defines static workflow configuration — the model router provides the dynamic layer.

```yaml
# REMOVED — these go to model router:
# adaptive:
#   resource_monitoring: ...
#   memory_pressure_handling: ...
#   concurrency_adjustment: ...
#   performance_based_scheduling: ...
```

### Validation
- [ ] All adaptive features removed from schema
- [ ] Requirements documented for migration to model router repo
- [ ] New schema parses as valid YAML

---

## Phase 6: Tool Permissions + Absorb AI Operations

Remove `enabled:` keys (presence = enabled). Add `disabled:` key. Remove `ai_operations:` — absorbed into `tool_permissions` with rate limits (inline or file-referenced).

```yaml
tool_permissions:
  file_operations:
    read: { require_confirmation: false, allowed_paths: [...], disabled: false }
    write: { require_confirmation: true, allowed_paths: [...] }
  web_operations:
    fetch: { max_concurrent_requests: 5, rate_limits: { max_per_hour: 100 }, rate_limits_file: "./config/web_limits.yml" }
  shell_operations:
    exec: { require_confirmation: true, allowed_commands: [...] }
  content_operations:
    generate: { rate_limits: { max_tokens_per_hour: 50000 } }
  system_operations:
    process_management: { disabled: true }
    network_operations: { disabled: true }
```

### Validation
- [ ] All granularity preserved
- [ ] AI operations absorbed
- [ ] Rate limits referenceable by file
- [ ] Presence = enabled, `disabled` to turn off
- [ ] New schema parses as valid YAML

---

## Phase 7: Remove .glyphnova References + Simplify Reference Resolution

### 7A: Remove ALL .glyphnova References
Remove every reference to `.glyphnova/` from the schema. This is a separate application (moving to the `whitt` repository). The schema should use generic workspace paths instead:

```yaml
# BEFORE:
state_file_path: "${workspace.checkpoint_path}/state.json"

# AFTER (no .glyphnova):
state_file_path: "${workspace.directories.checkpoints}/state.json"
```

### 7B: Reference Resolution — Scope and Purpose

**What reference resolution does:** When a workflow is used as a sub-workflow, or when variables reference external resources (models, configs, other workflows), the engine needs rules for how to find and resolve those references. Reference resolution defines:
- **Where to look** for referenced resources (local file, registry, parent workflow)
- **How to cache** resolved references (TTL, validation)
- **What to do on conflicts** (version mismatches, ambiguous references)

**Why it's needed:** Without it, sub-workflows can't reliably reference parent resources, model configs can't be shared across workflows, and variable interpolation across workflow boundaries is undefined.

**However:** The user believes much of this belongs in the `models:` configuration with keyword-reserved schema structures. If a workflow is used as a sub-flow, the reference resolution for models should be configured inside the models config, not as a separate top-level key.

**Decision: Remove `models.routing:` entirely.** Model routing/reference resolution is handled by the model router project, not the execution engine. The schema only needs `default_router: automatic`. Keep sub-workflow reference resolution as a smaller section under `workflow_execution_strategy.sub_workflow`:**

```yaml
models:
  # ... existing model config ...
  default_router: automatic
  # No routing: section — model router handles all routing logic

workflow_execution_strategy:
  sub_workflow:
    reference_resolution:
      strategy: local_first
      cache_ttl_secs: 3600
      on_version_conflict: error
```

**Also remove `framework:` from model definitions.** The framework (agentsdk) is a backend implementation detail, not a schema-configurable option. It should not be part of the workflow schema.

### Variable Override Precedence

When a sub-workflow runs inside a parent workflow, variables follow this precedence:

1. **Sub-workflow explicit values** (highest priority)
2. **Parent workflow passed values** (via `input:` on the sub-workflow step)
3. **Sub-workflow default values** (from `hardcoded_values:` or `default_inputs:`)
4. **Parent workflow defaults** (lowest priority)

Rules:
- Sub-workflow > parent workflow always
- Parent workflows should NOT be able to modify sub-workflow outputs
- Sub-workflow output is accessible like any step: `{{step.sub_workflow_step_name.output}}`
- Default behavior: lowest-level declaration wins unless explicitly overridden

**`forced_values` are REMOVED for now.** Forcing values down the hierarchy is only possible through the model router, which is more secure. This can be added back later if needed.

**`exposes` is REMOVED.** Sub-workflows behave like any other step — they produce output accessible via `{{step.sub_workflow_step_name.output}}`. No explicit exposure list needed.

### Sub-Workflow Top-Down Referencing

Sub-workflows defined in `sub_workflows:` can reference each other top-down. A sub-workflow defined ABOVE another can be used by the one below it:

```yaml
sub_workflows:
  # Defined first — available to all below
  analyze_code:
    path: "./workflows/analyze.yml"
  
  # Can reference analyze_code (defined above)
  fix_code:
    path: "./workflows/fix.yml"
    input:
      analysis: "{{sub_workflow.analyze_code.output}}"
  
  # Can reference both above
  validate_fix:
    path: "./workflows/validate.yml"
    input:
      analysis: "{{sub_workflow.analyze_code.output}}"
      fix: "{{sub_workflow.fix_code.output}}"
```

Reference resolution follows definition order: only sub-workflows defined ABOVE can be referenced by those below.

### Model Router Override Capability

The model router project handles dynamic scaling and can override model selection at both workflow and sub-workflow levels. This means:

```yaml
models:
  default_router: automatic
  # No routing: section — model router handles all routing
  # Model router can dynamically override which model is used
  # for any step or sub-workflow at runtime for scaling purposes
```

The schema defines the **static** configuration. The model router provides the **dynamic** override layer. The router can:
- Override model selection for any step or sub-workflow
- Redirect execution to different models based on resource availability
- Scale execution up/down by adjusting parallelism dynamically
- Override the execution hierarchy for load balancing

### Validation
- [ ] ALL `.glyphnova` references removed
- [ ] `models.routing:` removed (model router responsibility)
- [ ] `framework:` key removed from model definitions
- [ ] Sub-workflow reference resolution simplified
- [ ] Variable override precedence documented
- [ ] New schema parses as valid YAML

---

## Phase 8: Flatten Output Fields

Output fields are now handled through hooks (Phase 1B). This phase removes the standalone output field list and replaces with structured categories inside hook `log.event_fields`:

```yaml
# Instead of a flat 90+ field list, event_fields uses categories:
when:
  after_step_succeeds:
    log:
      to_file_path: "./workspace/logs/steps.log"
      event_fields:
        - step_name
        - duration_ms
        - result.quality_score
        - result.issues
        - metadata.tokens_used
```

The available field categories (for reference in hook event_fields):
- **workflow_status**: system_initialized, state_loaded, workspace_configured
- **step_status**: step_name, step_id, status, started_at, completed_at
- **execution_outcome**: success, failed, reason, final_outcome
- **file_operations**: code_file_written, documentation_file_written, file_path
- **validation**: code_quality_score, validation_results, all_validation_passed
- **knowledge**: documentation_completeness, knowledge_base_updated
- **metrics**: tokens_used, performance_metrics, turns, time_taken_ms
- **retry_branching**: retry_count, branch_decision, adjustment_strategy
- **provenance**: execution_id, event_log, checkpoint_saved

### Validation
- [ ] All output fields accessible through hook event_fields
- [ ] No standalone output section
- [ ] New schema parses as valid YAML

---

## Phase 9: Documentation Cleanup + Thresholds via hardcoded_values

### 9A: Remove `numeric_thresholds` Externalization
Instead of a separate `numeric_thresholds:` section, thresholds are handled through `hardcoded_values` with a naming convention:

```yaml
agentic_workflow:
  hardcoded_values:
    # Standard variables
    workspace_path: "/workspace"
    
    # Thresholds use a _threshold suffix by convention
    quality_threshold: 0.90
    tolerance_threshold: 0.05
    max_iterations_threshold: 5
    memory_pressure_threshold_percent: 85
    
    # Or group them:
    thresholds:
      quality_min: 0.90
      tolerance: 0.05
      max_iterations: 5
      memory_pressure_percent: 85
```

The engine uses a **heuristic**: any value in `hardcoded_values` ending in `_threshold` or nested under `thresholds:` is treated as a configurable numeric limit. This achieves the same objective as a separate thresholds section without adding schema surface area.

### 9B: Remove `validation:` Top-Level Key — Move to Step-Level

**What `validation:` does currently:** Defines schema validation (required fields, valid URIs), step output validation (format checking), and business rules (quality_score >= 0.95).

**Why it's questionable as top-level:**
- Schema validation is an engine concern, not a workflow config concern
- Step output validation belongs on the step itself (or in retry/loop)
- Business rules are validation criteria that belong in retry conditions or loop convergence criteria

**Decision: Remove top-level `validation:`. Move business rules into step-level retry/loop conditions:**

```yaml
# BEFORE: Top-level validation
validation:
  business_rules:
    - rule: "quality_score >= 0.95"
      severity: error
      description: "Code quality must meet minimum threshold"

# AFTER: In step retry/loop condition
steps:
  validate_code:
    generative_entity: "${models.primary}"
    prompt: "Validate the code quality"
    retry:
      condition: "result.quality_score < 0.95"
      max_attempts: 3
      adjustment_strategy: loosen_tolerance
```

### 9C: Remove `orchestration:` Top-Level Key

**What `orchestration:` does currently:** Defines step coordination, sub-agent orchestration, event handling, validation aggregation, and checkpoint coordination.

**Why it's questionable as top-level:**
- Step coordination is already handled by `depends_on`, `parallel_group`, and `route_to` on steps
- Sub-agent orchestration is just sub-workflow invocation (already in Phase 2C)
- Event handling is covered by `when` hooks
- Validation aggregation is covered by validation loop convergence criteria
- Checkpoint coordination is covered by checkpoint actions in hooks

**Decision: Remove top-level `orchestration:`. Features absorbed into:**
- `depends_on` / `parallel_group` / `route_to` → step coordination
- `sub_workflows:` → sub-agent/sub-workflow definitions
- `when:` hooks → event handling
- Loop validation criteria → validation aggregation
- `checkpoint:` action in hooks → checkpoint coordination

### 9D: Remove `metrics:` Top-Level Key

**What `metrics:` does currently:** Defines collection levels (pipeline, step, model, tool), performance optimization (cache, batching, prefetching, memory), and output config.

**How this differs from hook-based logging:**
- **Logging** = write events to files/console for human consumption
- **Metrics** = structured numerical measurements for performance analysis and optimization

Metrics collection is an **engine concern**, not a workflow config concern. The engine should always collect standard metrics. Custom metrics can be expressed through hook `log.event_fields`:

```yaml
# Custom metrics via hooks
when:
  after_step_succeeds:
    log:
      to_file_path: "./workspace/metrics/step_metrics.json"
      event_fields: [execution_time_secs, memory_peak_mb, cache_hit_rate, tokens_processed]
```

Performance optimization (cache, batching, prefetching) belongs in `workflow_execution_strategy` if kept at all.

**Decision: Remove top-level `metrics:`. Custom metric collection happens through hooks. Performance optimization moves to `workflow_execution_strategy` if needed.**

### 9E: Remove `adaptive:` Entirely — Moved to Model Router

Adaptive behavior is NOT part of the YAML-to-Rust AgentSDK. The AgentSDK is a straight execution engine. ALL adaptive features (resource monitoring, memory pressure handling, concurrency adjustment, performance-based scheduling) are removed and moved to the model router repository documentation. See Phase 5 for details.

### 9F: Remove `features_demonstrated:` Entirely

This was a vestigial key added for human review of example workflows. It serves no runtime purpose and should be removed from the schema entirely.

### 9G: Documentation Cleanup
1. Remove external doc links (keep `schema_version`)
2. Keep default behavior reference and scope/inheritance rules as comments
3. Model lifecycle: comment only, not configurable (MVP)
4. Add "Parallelism and permissions are independent control surfaces" clarification comment

### Validation
- [ ] No standalone `numeric_thresholds` section
- [ ] Thresholds accessible through `hardcoded_values`
- [ ] Top-level `validation:` removed
- [ ] Top-level `orchestration:` removed
- [ ] Top-level `metrics:` removed
- [ ] `adaptive:` removed entirely (moved to model router)
- [ ] `features_demonstrated:` removed entirely
- [ ] External doc links removed
- [ ] New schema parses as valid YAML

---

## Phase 10: Move RAG Under Memory + Remove ALL `enabled` Keys + Workspace

### 10A: Move RAG Under Memory Structure

RAG (Retrieval-Augmented Generation) is a memory capability, not a standalone feature. Move `rag:` into a `memory:` configuration:

```yaml
# BEFORE: Top-level RAG with enabled key
rag:
  enabled: false
  knowledge_base:
    path: /workspace/rag/knowledge_base
  embedding_model:
    model_ref: "${models.embedding_model}"
  retrieval:
    max_results: 10
    similarity_threshold: 0.7

# AFTER: RAG under memory, no enabled key
# Presence of the rag key = RAG is enabled
# Comment out or remove the rag key to disable it
memory:
  rag:
    knowledge_base:
      path: /workspace/rag/knowledge_base
    embedding_model:
      model_ref: "${models.embedding_model}"
    retrieval:
      max_results: 10
      similarity_threshold: 0.7
  
  # Future: other memory types
  # conversation_history: { max_turns: 10 }
  # long_term: { storage_path: "./workspace/memory/long_term.json" }
```

### 10B: Remove ALL `enabled` Keys Schema-Wide

**Design principle: Presence = enabled. Comment out or remove to disable.**

Every `enabled: true` / `enabled: false` key is removed across the entire schema. If a feature section is present, it's enabled. To disable, comment it out in YAML (`#`) or remove it.

```yaml
# BEFORE:
logging:
  enabled: true
  scopes:
    workflow:
      enabled: true

tool_permissions:
  file_operations:
    read:
      enabled: true

rag:
  enabled: false

checkpointing:
  enabled: true

# AFTER — no enabled keys anywhere:
# If you want logging, the logging config exists. If you don't, comment it out.
# If you want RAG, the rag key exists under memory. If you don't, comment it out.
# If you want checkpointing, the checkpointing config exists. If not, remove it.
```

This applies to ALL sections: `logging`, `rag`, `checkpointing`, `tool_permissions`, `parallel`, `synchronization`, `circular_reference_detection`, `isolated_environments`, `sub_workflow`, `resource_allocation`, `step_prioritization`, `performance_optimization`, `guardrails`, and any other section.

To explicitly turn something OFF while keeping the config for reference, use the `disabled` key:

```yaml
tool_permissions:
  system_operations:
    process_management: { disabled: true }    # explicitly disabled, config preserved
```

### 10B: Workspace Discussion

Workspace is a useful concept for defining directories and file scope. It could stay as top-level since it defines the physical environment:

```yaml
workspace:
  root_path: /workspace
  directories:
    output: /workspace/output
    checkpoints: /workspace/checkpoints
    logs: /workspace/logs
    metrics: /workspace/metrics
```

However, the non-deterministic step substructure (scripts, file operations) could be integrated into workspace rather than being separate top-level concerns. **For now, keep `workspace:` as top-level — it defines the physical environment and is referenced by hooks and steps.**

### Validation
- [ ] RAG moved under `memory:` structure
- [ ] Workspace kept as top-level (physical environment definition)
- [ ] All features preserved
- [ ] New schema parses as valid YAML

---

## Phase 11: Preserved Sections (No Changes)

These sections carry over as-is:
- `providers:` — Provider-specific config
- `workspace:` — Directory structure and file permissions
- Variable interpolation syntax comments (`${...}` vs `{{...}}`)
- Default behavior reference comments
- Scope & inheritance rules comments
- Model lifecycle state machine comments (not configurable in MVP)
- Duplicate configuration systems clarification comment

### Sections Removed:
- `pipeline:` — Removed in Phase 1A
- `output:` on steps — Removed in Phase 1B
- `features_demonstrated:` — Removed in Phase 9F
- `validation:` top-level — Removed in Phase 9B
- `orchestration:` top-level — Removed in Phase 9C
- `metrics:` top-level — Removed in Phase 9D
- `adaptive:` — Removed entirely, moved to model router (Phase 5)
- `rag:` top-level — Moved to `memory.rag` in Phase 10A
- `ai_operations:` under tool_permissions — Absorbed in Phase 6
- `advanced_scheduling:` — Removed in Phase 4
- `logging:` top-level — Replaced by hook-based logging in Phase 3
- ALL `enabled:` keys — Removed in Phase 10B (presence = enabled)
- `forced_values` — Removed for now, deferred to model router

---

## Phase 12: Final Assembly and Validation

### Execution order
1-10: Apply phases sequentially
11: Preserve sections (carry over as-is)
12: Final validation

### Top-Level Key Summary (After All Phases)
```yaml
# ── WORKFLOW IDENTIFICATION
workflow_id, name, description, version, author, tags

# ── MODEL CONFIGURATION
models:
  # No routing: section — model router handles routing (not part of execution engine)
  # No framework: key — backend implementation detail, not schema-configurable
  
# ── SUB-WORKFLOW DEFINITIONS
sub_workflows: { ... }

# ── AGENTIC WORKFLOW
agentic_workflow:
  hardcoded_values: { ... }  # Includes thresholds
  user_inputs: { ... }
  steps: { ... }             # No type property, no output section
  retry: { ... }             # Flat workflow retry + step: subkey

# ── WORKFLOW EXECUTION STRATEGY (straight execution, no adaptive)
workflow_execution_strategy:
  load_unload: ...
  processing: ...
  parallel: { ... }
  memory: { ... }
  timeout: { ... }
  error_handling: { ... }
  sub_workflow: { ... }      # Reference resolution moved here
  checkpointing: { ... }

# ── TOOL PERMISSIONS
tool_permissions: { ... }    # Includes rate_limits (absorbed AI operations)

# ── PROVIDERS
providers: { ... }

# ── MEMORY
memory:
  rag: { ... }               # Moved from top-level rag:, no enabled key

# ── WORKSPACE
workspace: { ... }

# ── SCHEMA METADATA
schema_version: ...
```

### Validation criteria
- [ ] Entire file parses as valid YAML
- [ ] Every feature from original schema is present or absorbed
- [ ] No standalone `logging:` section
- [ ] No standalone `validation:` section
- [ ] No standalone `orchestration:` section
- [ ] No standalone `metrics:` section
- [ ] No standalone `rag:` section (under `memory:`)
- [ ] No `features_demonstrated:` section
- [ ] No `pipeline:` section
- [ ] No `advanced_scheduling:` section
- [ ] No `ai_operations:` section
- [ ] No `.glyphnova` references
- [ ] No adaptive features (moved to model router)
- [ ] No `enabled:` keys anywhere (presence = enabled, comment out to disable)
- [ ] `when` and `lifecycle_hooks` both work as keys
- [ ] `gtw` → `gwt` everywhere (Given/When/Then, not GTW)
- [ ] `gwt` and `given_when_then` both work as keys
- [ ] Both flat and nested hook notation work
- [ ] GWT at top-level = LLM/agent routing, in hooks = deterministic
- [ ] Hook timing: before (no results), after (complete), during (streaming)
- [ ] Control flow steps use `when.gwt` directly
- [ ] Workflow retry flat with `step:` subkey
- [ ] Output replaced by hooks + default step-name.output variables
- [ ] No `type:` property on steps (inferred from keys)
- [ ] Tool permissions: presence = enabled, `disabled` to turn off
- [ ] AI operations absorbed into tool_permissions
- [ ] Rate limits referenceable by file
- [ ] Competing loop configs resolve by min-constraint
- [ ] All 5 loop types preserved
- [ ] `save_to` / `append_to` are aliases (both append)
- [ ] `bookmark` = alias for `checkpoint`
- [ ] No `models.routing:` section (model router handles)
- [ ] No `framework:` key on model definitions
- [ ] `bookmark` = alias for `checkpoint` (not replacement)
- [ ] `route_to` kept as-is
- [ ] `then` is optional in hooks
- [ ] Skip actions: `skip_step`, `skip_loop`, `skip_sub_workflow`, `skip_remaining`
- [ ] `on_requires_failed` hook for dependency failures
- [ ] `forced_values` removed (deferred to model router)
- [ ] Model router can override model selection dynamically
- [ ] Default format is YAML for structured data
- [ ] Line count reduced from 1705
- [ ] `exposes` removed (sub-workflows output like any step)
- [ ] Sub-workflows can reference each other top-down
- [ ] `input_variables` removed — `inputs` at agentic_workflow level
- [ ] User inputs available at step level directly
- [ ] Step retry default backoff = linear
- [ ] `allocation` renamed to `ram_allocation`
- [ ] `system_operations` has robust subschema
- [ ] Semantic hierarchy enforced (workflow → agentic_workflow → step)

### Critical review checkpoints
After each phase, review:
1. **Human-readable**: Natural language? Skimmable? Non-technical person?
2. **Extensible**: Add new things without restructuring? Backwards compatible?
3. **Simple**: Deduplicated? Holdable in head? One way to do things?
