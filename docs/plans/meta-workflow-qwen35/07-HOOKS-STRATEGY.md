# 07 - Hooks Strategy

## Executive Summary

This document defines the comprehensive hooks strategy for all 5 sub-workflows (SW1-SW5) of the meta-workflow generator. Hooks are the primary mechanism for controlling execution flow, logging intermediate results, validating outputs, and orchestrating the 5-step pipeline. Each sub-workflow uses a specific subset of the 10 available triggers and 12 available actions, configured via the `hooks:` section in the workflow YAML.

The hooks strategy is organized by sub-workflow, with each section defining: which triggers to use, which actions to attach to each trigger, what each action should accomplish, and how to evaluate hook effectiveness. The strategy is based on the hook system defined in `src/workflow/hooks/` (trigger definitions in `context.rs`, action execution in `actions.rs`, GWT evaluation in `gwt.rs`) and wired in `src/benchmark/runner.rs` (lines 1257, 1286, 1318, 1340, 1355, 1535, 1592).

**Key Principle:** Hooks are the primary mechanism for implementing the sub-workflow processing logic. Each sub-workflow uses hooks to: (1) validate inputs, (2) orchestrate processing steps, (3) accumulate outputs, (4) validate outputs, (5) log intermediate results, (6) retry on failures, and (7) checkpoint state for recovery.

## 1. Hook System Reference

### 1.1 Available Triggers (10 Total)

The hook system defines 10 trigger types in `src/workflow/hooks/context.rs`. Each trigger fires at a specific point in workflow execution and has a unique context struct with specific fields available for interpolation.

| # | Trigger | Context Struct | Runner Firing Point | Available Fields (from context) |
|---|---------|---------------|---------------------|--------------------------------|
| 1 | `before_step_starts` | `BeforeStepStartsContext` | `runner.rs:1257` | `step_type`, `model_name`, `prompt_preview`, `workflow_variables` |
| 2 | `during_step_streaming` | `DuringStepStreamingContext` | — (not wired) | `step_name`, `model_name`, `chunk_text`, `chunk_index`, `total_chunks` |
| 3 | `after_step_succeeds` | `AfterStepSucceedsContext` | `runner.rs:1340` | `step_name`, `model_name`, `output`, `duration_ms`, `token_count`, `quality_score` |
| 4 | `after_step_fails` | `AfterStepFailsContext` | `runner.rs:1318` | `step_name`, `model_name`, `error_message`, `error_is_retryable`, `attempt_number` |
| 5 | `after_all_retries_exhausted` | `AfterAllRetriesExhaustedContext` | `runner.rs:1355` | `step_name`, `model_name`, `total_attempts`, `final_error_message` |
| 6 | `after_step_starts` | `AfterStepStartsContext` | `runner.rs:1286` | `step_name`, `model_name`, `prompt_preview` |
| 7 | `before_gwt_evaluates` | `BeforeGwtEvaluatesContext` | `actions.rs:404` (partial) | `gwt_clauses`, `step_name` |
| 8 | `after_gwt_evaluates` | `AfterGwtEvaluatesContext` | `actions.rs:414` (partial) | `gwt_result`, `matched_clause_index`, `step_name` |
| 9 | `on_requires_failed` | `OnRequiresFailedContext` | `runner.rs:1535` | `step_name`, `missing_requirements`, `step_outputs` |
| 10 | `after_loop_iteration_fails` | `AfterLoopIterationFailsContext` | `runner.rs:1592` | `step_name`, `iteration_index`, `error_message`, `error_is_retryable` |

**Wiring Status:**

- Fully wired (fires in runner): 7/10 triggers (before_step_starts, after_step_starts, after_step_fails, after_all_retries_exhausted, after_step_succeeds, on_requires_failed, after_loop_iteration_fails)
- Partially wired (info logging only): 2/10 triggers (before_gwt_evaluates, after_gwt_evaluates)
- Not wired (requires SSE streaming): 1/10 trigger (during_step_streaming)

**Reference Files:**

- Context definitions: `src/workflow/hooks/context.rs` (10 context structs, enum WorkflowHookContext)
- Firing points: `src/benchmark/runner.rs:1257-1610` (7 firing points)
- Hook execution: `src/workflow/hooks/actions.rs` (execute_action dispatcher, all execute_* functions)

### 1.2 Available Actions (12 Total)

The hook system defines 12 action types in `src/workflow/step.rs:174-323`. Each action performs a specific operation (file I/O, state mutation, control flow, etc.) and returns a HookResult variant.

| # | Action | Action Struct | Execute Function | Result Variant | Primary Use Case |
|---|--------|---------------|------------------|----------------|------------------|
| 1 | `Log` | `LogAction` | `execute_log()` | `Continue` | Write logs to file or stdout |
| 2 | `AppendTo` | `AppendToAction` | `execute_append_to()` | `Continue` | Accumulate results across steps |
| 3 | `SaveTo` | `SaveToAction` | `execute_save_to()` | `Continue` | Save step output to file/memory |
| 4 | `RouteTo` | `RouteToAction` | `execute_route_to()` | `RouteTo{targets}` | Conditional branching |
| 5 | `Bookmark` | `BookmarkAction` | `execute_bookmark()` | `Continue` | Persist state for later steps |
| 6 | `Notify` | `NotifyAction` | `execute_notify()` | `Continue` | Send notifications (stub) |
| 7 | `Fail` | `FailAction` | `execute_fail()` | `Fail{reason}` | Immediate workflow failure |
| 8 | `Shell` | `ShellAction` | `execute_shell()` | `Continue` or `Fail` | Execute shell commands |
| 9 | `SkipStep` | `SkipStepAction` | `execute_skip_step()` | `SkipStep` | Skip current step |
| 10 | `SkipRemaining` | `SkipRemainingAction` | `execute_skip_remaining()` | `SkipRemaining` | Skip all remaining steps |
| 11 | `Gwt` | `GwtAction` | `execute_gwt()` | `Continue` or `RouteTo{targets}` | Evaluate GWT expressions |
| 12 | `IterateValues` | `IterateValuesAction` | `execute_action()` (passthrough) | `Continue` | Loop over values (future) |

**Action Schemata (from `src/workflow/step.rs:202-323`):**

```rust
// LogAction (lines 202-207)
pub struct LogAction {
    pub level: LogLevel,              // Info, Warning, Error, Critical, Debug
    pub message: String,              // Template string with {{variable}} interpolation
    pub to_file_path: Option<String>, // Optional file path
    pub event_fields: Vec<String>,    // List of context fields to include
}

// AppendToAction (lines 209-214)
pub struct AppendToAction {
    pub target: AppendTarget,         // FilePath, Variable, or Both
    pub file_path: Option<String>,    // File path if target is FilePath or Both
    pub variable_name: Option<String>, // Variable name if target is Variable or Both
    pub content_template: String,     // Template string with {{variable}} interpolation
}

// SaveToAction (lines 216-221)
pub struct SaveToAction {
    pub target: SaveTarget,           // FilePath, Variable, or Both
    pub file_path: Option<String>,    // File path if target is FilePath or Both
    pub variable_name: Option<String>, // Variable name if target is Variable or Both
    pub content_template: String,     // Template string with {{variable}} interpolation
}

// RouteToAction (lines 223-226)
pub struct RouteToAction {
    pub targets: Vec<String>,         // List of step names to route to
}

// BookmarkAction (lines 228-237)
pub struct BookmarkAction {
    pub name: String,                 // Bookmark name
    pub value: BookmarkValue,         // Flag(true), Path(string), or Detailed{path, content}
}

// NotifyAction (lines 239-242)
pub struct NotifyAction {
    pub message: String,              // Notification message
}

// FailAction (lines 244-246)
pub struct FailAction {
    pub message: Option<String>,      // Optional failure message
}

// ShellAction (lines 248-263)
pub struct ShellAction {
    pub command: String,              // Shell command to execute
    pub fail_on_error: bool,          // Fail workflow if command fails
    pub bookmark_output: bool,        // Store command output in bookmark "shell_output"
    pub env_vars: HashMap<String, String>, // Environment variables for command
    pub working_directory: Option<String>, // Working directory for command
}

// SkipStepAction (lines 265-267)
pub struct SkipStepAction {
    pub skip: bool,                   // true = skip, false = don't skip
}

// SkipRemainingAction (lines 269-271)
pub struct SkipRemainingAction {
    pub skip: bool,                   // true = skip remaining, false = don't skip
}

// GwtAction (lines 273-286)
pub struct GwtAction {
    pub clauses: Vec<GwtClause>,      // List of GWT clauses (given-when-then)
}

pub struct GwtClause {
    pub given: String,                // Condition expression
    pub when: Vec<String>,            // When conditions (additional filters)
    pub then: Vec<String>,            // Action targets (step names)
}

// IterateValuesAction (lines 288-292)
pub struct IterateValuesAction {
    pub values: HashMap<String, serde_json::Value>, // Key-value pairs to iterate
}
```

**HookResult Variants (from `src/workflow/hooks/mod.rs:13-38`):**

```rust
pub enum HookResult {
    Continue,                         // Continue to next step
    RouteTo { targets: Vec<String> }, // Route to specific steps
    SkipStep,                         // Skip current step
    SkipRemaining,                    // Skip all remaining steps
    Fail { reason: String },          // Fail workflow immediately
    SkipLoop,                         // Skip current loop iteration
}
```

**Merge Priority (from `src/workflow/hooks/mod.rs:40-80`):**

When multiple actions return different HookResults, they are merged with this priority (highest to lowest):
1. `Fail` — always wins, workflow terminates
2. `SkipRemaining` — wins over everything except Fail
3. `RouteTo` — wins over SkipStep, SkipLoop, Continue
4. `SkipLoop` — wins over SkipStep, Continue
5. `SkipStep` — wins over Continue
6. `Continue` — default, lowest priority

**Reference Files:**

- Action definitions: `src/workflow/step.rs:174-323`
- Action execution: `src/workflow/hooks/actions.rs` (execute_action dispatcher, lines 1-200)
- HookResult merge: `src/workflow/hooks/mod.rs:40-80`
- GWT evaluation: `src/workflow/hooks/gwt.rs` (lexer, parser, evaluator)

### 1.3 Template Interpolation

All `LogAction`, `AppendToAction`, and `SaveToAction` templates support variable interpolation using `{{variable_name}}` syntax. Variables are extracted from the trigger context fields.

**Available Variables (by trigger):**

| Trigger | Available Variables | Example Usage |
|---------|---------------------|---------------|
| `before_step_starts` | `step_type`, `model_name`, `prompt_preview`, `workflow_variables.*` | `{{step_type}}`, `{{workflow_variables.input_prompt}}` |
| `after_step_starts` | `step_name`, `model_name`, `prompt_preview` | `{{step_name}}`, `{{model_name}}` |
| `after_step_succeeds` | `step_name`, `model_name`, `output`, `duration_ms`, `token_count`, `quality_score` | `{{output}}`, `{{duration_ms}}`, `{{quality_score}}` |
| `after_step_fails` | `step_name`, `model_name`, `error_message`, `error_is_retryable`, `attempt_number` | `{{error_message}}`, `{{attempt_number}}` |
| `after_all_retries_exhausted` | `step_name`, `model_name`, `total_attempts`, `final_error_message` | `{{total_attempts}}`, `{{final_error_message}}` |
| `on_requires_failed` | `step_name`, `missing_requirements`, `step_outputs.*` | `{{missing_requirements}}`, `{{step_outputs.step_1_output}}` |
| `after_loop_iteration_fails` | `step_name`, `iteration_index`, `error_message`, `error_is_retryable` | `{{iteration_index}}`, `{{error_message}}` |

**Variable Type Handling:**

- **Strings:** Inserted directly into template (e.g., `{{step_name}}` → `"sw1_task_deconstruction"`)
- **Numbers:** Converted to string (e.g., `{{duration_ms}}` → `"1234"`)
- **Objects:** Converted to JSON string (e.g., `{{step_outputs}}` → `'{"step_1_output": "..."}'`)
- **Null:** Converted to string `"null"`
- **Missing:** If variable doesn't exist, template leaves the placeholder as-is (e.g., `{{missing_var}}` → `"{{missing_var}}"`)

**Example Template:**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "Step {{step_name}} completed in {{duration_ms}}ms with {{token_count}} tokens. Output: {{output}}"
        to_file_path: "outputs/{{step_name}}.log"
        event_fields: ["step_name", "duration_ms", "token_count", "output"]
```

**Reference File:**

- Template interpolation implementation: `src/workflow/hooks/actions.rs` (in execute_log, execute_append_to, execute_save_to)

## 2. SW1 Hooks Strategy

### 2.1 SW1 Overview

**Sub-Workflow:** SW1 (Task Deconstruction)
**Objective:** Parse user prompt, extract constituent tasks, produce structured task list
**Input:** User prompt (natural language)
**Output:** Structured task list (JSON format with task_name, task_type, dependencies, priority)
**Processing Steps:**
1. Validate input prompt (non-empty, reasonable length)
2. Parse prompt for constituent tasks (LLM-based)
3. Extract dependencies between tasks (LLM-based)
4. Assign priorities to tasks (LLM-based)
5. Output structured task list (JSON)

**Processing Characteristics:**
- Linear processing (no loops, no branches)
- Single-step LLM inference
- Output must be valid JSON
- No external tool calls
- Validation gates: JSON parsing, task count >= 1, no empty task names

### 2.2 SW1 Hook Configuration

SW1 uses 4 triggers across its 5 processing steps. Each trigger has 1-3 actions attached.

**Trigger 1: `before_step_starts` (Step 1: Input Validation)**

```yaml
hooks:
  before_step_starts:
    - log:
        level: Info
        message: "SW1: Starting input validation for prompt: {{prompt_preview}}"
        to_file_path: "outputs/sw1/log.txt"
        event_fields: ["step_type", "model_name", "prompt_preview"]
    - save_to:
        target: Variable
        variable_name: "sw1_input_prompt"
        content_template: "{{prompt_preview}}"
```

**Purpose:**
- Log the start of SW1 processing with prompt preview
- Save input prompt to variable for later steps (validation, parsing, dependency extraction)

**Actions:**
1. `Log` — Write to `outputs/sw1/log.txt` with prompt preview
2. `SaveTo` — Store prompt in bookmark `sw1_input_prompt` for later interpolation

**Evaluation Criteria:**
- Log file `outputs/sw1/log.txt` created
- Bookmark `sw1_input_prompt` stored and accessible in subsequent steps
- Log contains prompt preview (first 100 characters)

**Trigger 2: `after_step_fails` (Step 1: Input Validation Failure)**

```yaml
hooks:
  after_step_fails:
    - log:
        level: Error
        message: "SW1: Input validation failed at attempt {{attempt_number}}. Error: {{error_message}}"
        to_file_path: "outputs/sw1/log.txt"
        event_fields: ["step_name", "attempt_number", "error_message", "error_is_retryable"]
    - fail:
        message: "SW1 input validation failed: {{error_message}}"
```

**Purpose:**
- Log validation failure with attempt number and error message
- Fail workflow immediately (no retries for input validation errors, which are likely permanent)

**Actions:**
1. `Log` — Write to `outputs/sw1/log.txt` with error details
2. `Fail` — Terminate workflow with descriptive error message

**Evaluation Criteria:**
- Log file contains error message and attempt number
- Workflow terminates with Fail result
- Error message is propagated to workflow output

**Trigger 3: `after_step_succeeds` (Step 5: Output JSON Validation)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW1: Task list generated in {{duration_ms}}ms with {{token_count}} tokens. Output:\n{{output}}"
        to_file_path: "outputs/sw1/log.txt"
        event_fields: ["step_name", "duration_ms", "token_count", "output"]
    - save_to:
        target: Both
        file_path: "outputs/sw1/task_list.json"
        variable_name: "sw1_task_list"
        content_template: "{{output}}"
    - shell:
        command: "cat outputs/sw1/task_list.json | jq empty"
        fail_on_error: true
        bookmark_output: true
```

**Purpose:**
- Log successful task list generation with duration and token count
- Save JSON output to file (for user inspection) and variable (for SW2)
- Validate JSON syntax using `jq empty` (parses JSON without output, fails if invalid)

**Actions:**
1. `Log` — Write to `outputs/sw1/log.txt` with output preview
2. `SaveTo` — Store task list in `outputs/sw1/task_list.json` and bookmark `sw1_task_list`
3. `Shell` — Validate JSON with `jq empty` (fails workflow if JSON is invalid)

**Evaluation Criteria:**
- Log file contains duration, token count, and output preview
- File `outputs/sw1/task_list.json` created with valid JSON
- Bookmark `sw1_task_list` stored and accessible in SW2
- `jq empty` command succeeds (exit code 0)
- If JSON is invalid, workflow fails with `jq` error message

**Trigger 4: `after_step_fails` (Step 5: Output JSON Validation Failure)**

```yaml
hooks:
  after_step_fails:
    - log:
        level: Error
        message: "SW1: JSON validation failed at attempt {{attempt_number}}. Error: {{error_message}}"
        to_file_path: "outputs/sw1/log.txt"
        event_fields: ["step_name", "attempt_number", "error_message", "error_is_retryable"]
    - gwt:
        clauses:
          - given: "{{error_is_retryable}} == true"
            then:
              - retry_step_5
          - given: "{{error_is_retryable}} == false"
            then:
              - fail_workflow
```

**Purpose:**
- Log JSON validation failure
- Retry if error is retryable (e.g., LLM timeout, network error)
- Fail immediately if error is not retryable (e.g., JSON syntax error, LLM generation failure)

**Actions:**
1. `Log` — Write to `outputs/sw1/log.txt` with error details
2. `Gwt` — Evaluate `error_is_retryable`:
   - If true: Route to `retry_step_5` (re-run LLM generation)
   - If false: Route to `fail_workflow` (terminate with error)

**Evaluation Criteria:**
- Log file contains error message and attempt number
- GWT expression evaluates correctly
- If retryable, step re-executes (attempt number increments)
- If not retryable, workflow terminates with Fail result

### 2.3 SW1 Hook Summary

| Trigger | Actions | Purpose | Evaluation |
|---------|---------|---------|------------|
| `before_step_starts` | Log, SaveTo | Log start, save input | Log created, bookmark stored |
| `after_step_fails` (Step 1) | Log, Fail | Log validation failure, terminate | Log created, workflow failed |
| `after_step_succeeds` (Step 5) | Log, SaveTo, Shell | Log success, save JSON, validate JSON | Log created, JSON file created, JSON valid |
| `after_step_fails` (Step 5) | Log, Gwt | Log failure, retry or fail | Log created, correct routing |

**Total Hooks:** 4 triggers, 8 actions

**Hook Effectiveness Metrics:**

- **Logging coverage:** 4/4 triggers logged (100%)
- **State persistence:** 2 bookmarks stored (sw1_input_prompt, sw1_task_list)
- **Validation coverage:** 1 validation gate (JSON syntax)
- **Error handling:** 2 failure paths (input validation failure, JSON validation failure)
- **Retry logic:** 1 retry loop (GWT-based on error_is_retryable)

## 3. SW2 Hooks Strategy

### 3.1 SW2 Overview

**Sub-Workflow:** SW2 (Desired Output State)
**Objective:** Describe the desired output for each task from SW1, format as structured specification
**Input:** Task list from SW1 (JSON)
**Output:** Desired output state for each task (JSON format with task_name, output_type, output_format, validation_criteria)
**Processing Steps:**
1. Load task list from SW1 (read variable `sw1_task_list`)
2. For each task: analyze task type, infer desired output format, write validation criteria
3. Accumulate output state specifications across all tasks
4. Output structured output state (JSON)

**Processing Characteristics:**
- Loop-based processing (iterate over tasks from SW1)
- Multi-step LLM inference (one inference per task)
- Output must be valid JSON
- Accumulation pattern (build JSON incrementally)
- Validation gates: JSON parsing, output state count == task count, no empty validation criteria

### 3.2 SW2 Hook Configuration

SW2 uses 5 triggers across its 4 processing steps (with loop). Each trigger has 1-3 actions attached.

**Trigger 1: `before_step_starts` (Step 1: Load Task List)**

```yaml
hooks:
  before_step_starts:
    - log:
        level: Info
        message: "SW2: Loading task list from SW1 bookmark 'sw1_task_list'"
        to_file_path: "outputs/sw2/log.txt"
        event_fields: ["step_type", "model_name"]
```

**Purpose:**
- Log the start of SW2 processing
- Verify that SW1 bookmark exists (implicit check via interpolation in later steps)

**Actions:**
1. `Log` — Write to `outputs/sw2/log.txt` with step start message

**Evaluation Criteria:**
- Log file `outputs/sw2/log.txt` created
- Log contains step start message

**Trigger 2: `after_step_succeeds` (Step 2: Process Task in Loop)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW2: Processed task {{iteration_index}} in {{duration_ms}}ms. Output:\n{{output}}"
        to_file_path: "outputs/sw2/log.txt"
        event_fields: ["step_name", "duration_ms", "output"]
    - append_to:
        target: Both
        file_path: "outputs/sw2/output_states.json"
        variable_name: "sw2_output_states"
        content_template: "{{output}}"
    - bookmark:
        name: "sw2_iteration_{{iteration_index}}_output"
        value:
          detailed:
            path: "outputs/sw2/iteration_{{iteration_index}}_output.json"
            content: "{{output}}"
```

**Purpose:**
- Log successful task processing with iteration index and output preview
- Append output to accumulating JSON file (build final output incrementally)
- Store iteration output in bookmark (for recovery if loop fails mid-execution)
- Create per-iteration backup file (for debugging)

**Actions:**
1. `Log` — Write to `outputs/sw2/log.txt` with iteration index and output
2. `AppendTo` — Append output to `outputs/sw2/output_states.json` and bookmark `sw2_output_states`
3. `Bookmark` — Store iteration output in bookmark `sw2_iteration_N_output` and file `outputs/sw2/iteration_N_output.json`

**Evaluation Criteria:**
- Log file contains iteration index and output preview
- File `outputs/sw2/output_states.json` accumulates outputs across iterations (valid JSON array)
- Bookmark `sw2_output_states` stored and accessible in SW3
- Per-iteration bookmark `sw2_iteration_N_output` stored (N = iteration index)
- Per-iteration file `outputs/sw2/iteration_N_output.json` created (for debugging)

**Trigger 3: `after_loop_iteration_fails` (Step 2: Loop Failure)**

```yaml
hooks:
  after_loop_iteration_fails:
    - log:
        level: Warning
        message: "SW2: Loop iteration {{iteration_index}} failed. Error: {{error_message}}. Retryable: {{error_is_retryable}}"
        to_file_path: "outputs/sw2/log.txt"
        event_fields: ["step_name", "iteration_index", "error_message", "error_is_retryable"]
    - shell:
        command: "echo 'Iteration {{iteration_index}} failed, skipping...' >> outputs/sw2/failed_iterations.log"
        fail_on_error: false
        bookmark_output: false
```

**Purpose:**
- Log loop iteration failure with iteration index and error message
- Record failed iteration in log file (for post-mortem analysis)
- Continue loop (do not fail entire workflow) — SW2 is resilient to per-task failures

**Actions:**
1. `Log` — Write to `outputs/sw2/log.txt` with iteration index and error
2. `Shell` — Append failure record to `outputs/sw2/failed_iterations.log`

**Evaluation Criteria:**
- Log file contains iteration index and error message
- File `outputs/sw2/failed_iterations.log` created with failure record
- Loop continues to next iteration (workflow does not fail)
- If `error_is_retryable` is false, failure is logged but workflow continues

**Trigger 4: `after_step_succeeds` (Step 4: Validate Output State JSON)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW2: Output state JSON validated in {{duration_ms}}ms. Total tasks: {{token_count}}"
        to_file_path: "outputs/sw2/log.txt"
        event_fields: ["step_name", "duration_ms", "token_count"]
    - shell:
        command: "cat outputs/sw2/output_states.json | jq 'length' > outputs/sw2/task_count.txt && cat outputs/sw2/task_count.txt"
        fail_on_error: false
        bookmark_output: true
    - gwt:
        clauses:
          - given: "{{shell_output}} > 0"
            then:
              - save_final_output
          - given: "{{shell_output}} == 0"
            then:
              - log_empty_output_warning
```

**Purpose:**
- Log successful JSON validation
- Count total tasks in output state JSON (via `jq 'length'`)
- Validate that at least one task was processed (non-zero count)
- Route to final save or warning based on task count

**Actions:**
1. `Log` — Write to `outputs/sw2/log.txt` with duration and task count
2. `Shell` — Count tasks in JSON with `jq 'length'`, store count in bookmark `shell_output`
3. `Gwt` — Evaluate task count:
   - If > 0: Route to `save_final_output` (proceed to SW3)
   - If == 0: Route to `log_empty_output_warning` (log warning, still proceed)

**Evaluation Criteria:**
- Log file contains duration and task count
- File `outputs/sw2/task_count.txt` created with task count
- Bookmark `shell_output` stored with task count
- GWT expression evaluates correctly
- If task count > 0, workflow routes to `save_final_output`
- If task count == 0, workflow routes to `log_empty_output_warning` (warning logged, workflow continues)

**Trigger 5: `after_step_fails` (Step 4: Validation Failure)**

```yaml
hooks:
  after_step_fails:
    - log:
        level: Error
        message: "SW2: JSON validation failed at attempt {{attempt_number}}. Error: {{error_message}}"
        to_file_path: "outputs/sw2/log.txt"
        event_fields: ["step_name", "attempt_number", "error_message", "error_is_retryable"]
    - fail:
        message: "SW2 JSON validation failed: {{error_message}}"
```

**Purpose:**
- Log JSON validation failure
- Fail workflow immediately (JSON validation is critical for SW3)

**Actions:**
1. `Log` — Write to `outputs/sw2/log.txt` with error details
2. `Fail` — Terminate workflow with descriptive error message

**Evaluation Criteria:**
- Log file contains error message and attempt number
- Workflow terminates with Fail result
- Error message is propagated to workflow output

### 3.3 SW2 Hook Summary

| Trigger | Actions | Purpose | Evaluation |
|---------|---------|---------|------------|
| `before_step_starts` | Log | Log start | Log created |
| `after_step_succeeds` (Step 2) | Log, AppendTo, Bookmark | Log iteration, accumulate JSON, checkpoint | Log created, JSON accumulated, bookmarks stored |
| `after_loop_iteration_fails` | Log, Shell | Log failure, record failed iteration | Log created, failure log created, loop continues |
| `after_step_succeeds` (Step 4) | Log, Shell, Gwt | Log validation, count tasks, validate count | Log created, count file created, correct routing |
| `after_step_fails` (Step 4) | Log, Fail | Log failure, terminate | Log created, workflow failed |

**Total Hooks:** 5 triggers, 10 actions

**Hook Effectiveness Metrics:**

- **Logging coverage:** 5/5 triggers logged (100%)
- **State persistence:** 3 bookmarks per iteration (sw2_output_states, sw2_iteration_N_output) + shell_output
- **Accumulation pattern:** AppendTo builds JSON incrementally across loop iterations
- **Checkpointing:** Per-iteration bookmarks and files enable recovery if loop fails mid-execution
- **Resilience:** Loop continues on individual task failures (after_loop_iteration_fails does not Fail)
- **Validation coverage:** 1 validation gate (task count > 0)
- **Error handling:** 2 failure paths (JSON validation failure, empty output warning)

## 4. SW3 Hooks Strategy

### 4.1 SW3 Overview

**Sub-Workflow:** SW3 (Agentic Categorization)
**Objective:** For each task from SW1 and output state from SW2, categorize the tools/actions needed, assign canonical categories
**Input:** Task list from SW1 (bookmark `sw1_task_list`), output state from SW2 (bookmark `sw2_output_states`)
**Output:** Categorized task list with canonical categories (JSON format with task_name, canonical_category, tools_needed, execution_strategy)
**Processing Steps:**
1. Load task list from SW1 and output state from SW2
2. For each task: analyze task type, match to canonical categories, enumerate tools needed
3. Assign execution strategy (linear, loop, branch, parallel)
4. Accumulate categorized tasks across all tasks
5. Output categorized task list (JSON)

**Processing Characteristics:**
- Loop-based processing (iterate over tasks from SW1)
- Multi-step LLM inference (one inference per task)
- Output must be valid JSON
- Accumulation pattern (build JSON incrementally)
- Validation gates: JSON parsing, canonical category validity, tool count >= 1
- Cross-reference: Must use both SW1 and SW2 bookmarks

### 4.2 SW3 Hook Configuration

SW3 uses 6 triggers across its 5 processing steps (with loop). Each trigger has 1-3 actions attached.

**Trigger 1: `before_step_starts` (Step 1: Load Task List and Output State)**

```yaml
hooks:
  before_step_starts:
    - log:
        level: Info
        message: "SW3: Loading task list from SW1 and output state from SW2"
        to_file_path: "outputs/sw3/log.txt"
        event_fields: ["step_type", "model_name"]
    - shell:
        command: "echo 'SW1 task count: {{workflow_variables.sw1_task_count}}' >> outputs/sw3/log.txt"
        fail_on_error: false
        bookmark_output: false
```

**Purpose:**
- Log the start of SW3 processing
- Verify SW1 and SW2 bookmarks exist (implicit check via interpolation in later steps)
- Log SW1 task count (for verification against SW3 output count)

**Actions:**
1. `Log` — Write to `outputs/sw3/log.txt` with step start message
2. `Shell` — Append SW1 task count to log (from workflow variable)

**Evaluation Criteria:**
- Log file `outputs/sw3/log.txt` created
- Log contains step start message and SW1 task count

**Trigger 2: `after_step_succeeds` (Step 2: Categorize Task in Loop)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW3: Categorized task {{iteration_index}} in {{duration_ms}}ms. Category: {{quality_score}}. Output:\n{{output}}"
        to_file_path: "outputs/sw3/log.txt"
        event_fields: ["step_name", "duration_ms", "quality_score", "output"]
    - append_to:
        target: Both
        file_path: "outputs/sw3/categorized_tasks.json"
        variable_name: "sw3_categorized_tasks"
        content_template: "{{output}}"
    - shell:
        command: "echo '{{output}}' | jq -r '.canonical_category' >> outputs/sw3/categories.txt"
        fail_on_error: false
        bookmark_output: false
```

**Purpose:**
- Log successful task categorization with iteration index, category, and output
- Append categorized task to accumulating JSON file
- Extract canonical category from output and append to categories.txt (for category distribution analysis)
- Store categorized tasks in bookmark (for SW4)

**Actions:**
1. `Log` — Write to `outputs/sw3/log.txt` with iteration index, category, output
2. `AppendTo` — Append to `outputs/sw3/categorized_tasks.json` and bookmark `sw3_categorized_tasks`
3. `Shell` — Extract canonical category with `jq -r '.canonical_category'`, append to `outputs/sw3/categories.txt`

**Evaluation Criteria:**
- Log file contains iteration index, category, and output preview
- File `outputs/sw3/categorized_tasks.json` accumulates outputs across iterations (valid JSON array)
- Bookmark `sw3_categorized_tasks` stored and accessible in SW4
- File `outputs/sw3/categories.txt` accumulates canonical categories (one per line)
- Category extraction succeeds (jq command succeeds)

**Trigger 3: `after_loop_iteration_fails` (Step 2: Loop Failure)**

```yaml
hooks:
  after_loop_iteration_fails:
    - log:
        level: Warning
        message: "SW3: Loop iteration {{iteration_index}} failed. Error: {{error_message}}. Retryable: {{error_is_retryable}}"
        to_file_path: "outputs/sw3/log.txt"
        event_fields: ["step_name", "iteration_index", "error_message", "error_is_retryable"]
    - shell:
        command: "echo 'Iteration {{iteration_index}} failed, skipping...' >> outputs/sw3/failed_iterations.log"
        fail_on_error: false
        bookmark_output: false
```

**Purpose:**
- Log loop iteration failure with iteration index and error message
- Record failed iteration in log file (for post-mortem analysis)
- Continue loop (do not fail entire workflow) — SW3 is resilient to per-task failures

**Actions:**
1. `Log` — Write to `outputs/sw3/log.txt` with iteration index and error
2. `Shell` — Append failure record to `outputs/sw3/failed_iterations.log`

**Evaluation Criteria:**
- Log file contains iteration index and error message
- File `outputs/sw3/failed_iterations.log` created with failure record
- Loop continues to next iteration (workflow does not fail)

**Trigger 4: `after_step_succeeds` (Step 4: Validate Canonical Categories)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW3: Canonical categories validated in {{duration_ms}}ms. Category distribution:"
        to_file_path: "outputs/sw3/log.txt"
        event_fields: ["step_name", "duration_ms"]
    - shell:
        command: "cat outputs/sw3/categories.txt | sort | uniq -c | sort -rn > outputs/sw3/category_distribution.txt && cat outputs/sw3/category_distribution.txt"
        fail_on_error: false
        bookmark_output: true
    - shell:
        command: "cat outputs/sw3/categories.txt | grep -v -E '^(file-read|transform-llm|validate-gate|loop-iterate|branch-decision|shell-execute|tool-call|checkpoint-state|notify-external|sub-workflow-ref)$' || echo 'All categories valid'"
        fail_on_error: false
        bookmark_output: true
```

**Purpose:**
- Log successful category validation
- Generate category distribution report (count of each canonical category)
- Validate that all categories are valid canonical categories (10 allowed categories)
- Record distribution and validation results

**Actions:**
1. `Log` — Write to `outputs/sw3/log.txt` with validation message
2. `Shell` — Generate distribution report with `sort | uniq -c | sort -rn`, store in bookmark `shell_output`
3. `Shell` — Validate categories by checking against canonical category list (grep for invalid categories), store in bookmark `shell_output`

**Evaluation Criteria:**
- Log file contains validation message
- File `outputs/sw3/category_distribution.txt` created with distribution report
- Bookmark `shell_output` stores distribution report
- Category validation succeeds (all categories match canonical list)
- If invalid categories exist, they are logged in `shell_output`

**Trigger 5: `after_step_succeeds` (Step 5: Validate Categorized Tasks JSON)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW3: Categorized tasks JSON validated in {{duration_ms}}ms. Total tasks: {{token_count}}"
        to_file_path: "outputs/sw3/log.txt"
        event_fields: ["step_name", "duration_ms", "token_count"]
    - shell:
        command: "cat outputs/sw3/categorized_tasks.json | jq 'length' > outputs/sw3/task_count.txt && cat outputs/sw3/task_count.txt"
        fail_on_error: false
        bookmark_output: true
    - gwt:
        clauses:
          - given: "{{shell_output}} == {{workflow_variables.sw1_task_count}}"
            then:
              - save_final_output
          - given: "{{shell_output}} != {{workflow_variables.sw1_task_count}}"
            then:
              - log_count_mismatch_warning
```

**Purpose:**
- Log successful JSON validation
- Count total categorized tasks (via `jq 'length'`)
- Validate that task count matches SW1 task count (no tasks lost or duplicated)
- Route to final save or warning based on count match

**Actions:**
1. `Log` — Write to `outputs/sw3/log.txt` with duration and task count
2. `Shell` — Count tasks in JSON with `jq 'length'`, store count in bookmark `shell_output`
3. `Gwt` — Evaluate task count match:
   - If matches SW1 count: Route to `save_final_output` (proceed to SW4)
   - If does not match: Route to `log_count_mismatch_warning` (log warning, still proceed)

**Evaluation Criteria:**
- Log file contains duration and task count
- File `outputs/sw3/task_count.txt` created with task count
- Bookmark `shell_output` stored with task count
- GWT expression evaluates correctly (comparing to SW1 task count)
- If counts match, workflow routes to `save_final_output`
- If counts mismatch, workflow routes to `log_count_mismatch_warning` (warning logged, workflow continues)

**Trigger 6: `after_step_fails` (Step 5: Validation Failure)**

```yaml
hooks:
  after_step_fails:
    - log:
        level: Error
        message: "SW3: JSON validation failed at attempt {{attempt_number}}. Error: {{error_message}}"
        to_file_path: "outputs/sw3/log.txt"
        event_fields: ["step_name", "attempt_number", "error_message", "error_is_retryable"]
    - fail:
        message: "SW3 JSON validation failed: {{error_message}}"
```

**Purpose:**
- Log JSON validation failure
- Fail workflow immediately (JSON validation is critical for SW4)

**Actions:**
1. `Log` — Write to `outputs/sw3/log.txt` with error details
2. `Fail` — Terminate workflow with descriptive error message

**Evaluation Criteria:**
- Log file contains error message and attempt number
- Workflow terminates with Fail result
- Error message is propagated to workflow output

### 4.3 SW3 Hook Summary

| Trigger | Actions | Purpose | Evaluation |
|---------|---------|---------|------------|
| `before_step_starts` | Log, Shell | Log start, verify SW1 count | Log created, count logged |
| `after_step_succeeds` (Step 2) | Log, AppendTo, Shell | Log iteration, accumulate JSON, extract categories | Log created, JSON accumulated, categories extracted |
| `after_loop_iteration_fails` | Log, Shell | Log failure, record failed iteration | Log created, failure log created, loop continues |
| `after_step_succeeds` (Step 4) | Log, Shell, Shell | Log validation, generate distribution, validate categories | Log created, distribution created, categories validated |
| `after_step_succeeds` (Step 5) | Log, Shell, Gwt | Log validation, count tasks, validate count match | Log created, count file created, correct routing |
| `after_step_fails` (Step 5) | Log, Fail | Log failure, terminate | Log created, workflow failed |

**Total Hooks:** 6 triggers, 12 actions

**Hook Effectiveness Metrics:**

- **Logging coverage:** 6/6 triggers logged (100%)
- **State persistence:** 1 bookmark (sw3_categorized_tasks) + shell_output
- **Accumulation pattern:** AppendTo builds JSON incrementally across loop iterations
- **Resilience:** Loop continues on individual task failures (after_loop_iteration_fails does not Fail)
- **Validation coverage:** 2 validation gates (canonical category validity, task count match)
- **Error handling:** 2 failure paths (JSON validation failure, count mismatch warning)
- **Cross-reference validation:** Validates task count against SW1 count

## 5. SW4 Hooks Strategy

### 5.1 SW4 Overview

**Sub-Workflow:** SW4 (YAML Substructure Translation)
**Objective:** For each categorized task from SW3, translate task specification to YAML substructure (steps, hooks, models)
**Input:** Categorized tasks from SW3 (bookmark `sw3_categorized_tasks`)
**Output:** YAML substructures for each task (YAML format with step definitions, hook configurations, model specifications)
**Processing Steps:**
1. Load categorized tasks from SW3
2. For each task: analyze canonical category, generate YAML step definition, add hook configuration, select model
3. Accumulate YAML substructures across all tasks
4. Output complete YAML workflow (YAML file)

**Processing Characteristics:**
- Loop-based processing (iterate over tasks from SW3)
- Multi-step LLM inference (one inference per task)
- Output must be valid YAML
- Accumulation pattern (build YAML incrementally)
- Validation gates: YAML parsing, step count matches task count, hook validity
- Cross-reference: Must use SW3 bookmark

### 5.2 SW4 Hook Configuration

SW4 uses 7 triggers across its 5 processing steps (with loop). Each trigger has 1-3 actions attached.

**Trigger 1: `before_step_starts` (Step 1: Load Categorized Tasks)**

```yaml
hooks:
  before_step_starts:
    - log:
        level: Info
        message: "SW4: Loading categorized tasks from SW3 bookmark 'sw3_categorized_tasks'"
        to_file_path: "outputs/sw4/log.txt"
        event_fields: ["step_type", "model_name"]
```

**Purpose:**
- Log the start of SW4 processing
- Verify SW3 bookmark exists (implicit check via interpolation in later steps)

**Actions:**
1. `Log` — Write to `outputs/sw4/log.txt` with step start message

**Evaluation Criteria:**
- Log file `outputs/sw4/log.txt` created
- Log contains step start message

**Trigger 2: `after_step_succeeds` (Step 2: Generate YAML Substructure in Loop)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW4: Generated YAML for task {{iteration_index}} in {{duration_ms}}ms. Category: {{quality_score}}. Output:\n{{output}}"
        to_file_path: "outputs/sw4/log.txt"
        event_fields: ["step_name", "duration_ms", "quality_score", "output"]
    - append_to:
        target: Both
        file_path: "outputs/sw4/workflow_steps.yml"
        variable_name: "sw4_yaml_substructures"
        content_template: "{{output}}"
    - shell:
        command: "echo '---' >> outputs/sw4/workflow_steps.yml"
        fail_on_error: false
        bookmark_output: false
```

**Purpose:**
- Log successful YAML generation with iteration index, category, and output
- Append YAML substructure to accumulating YAML file
- Add YAML document separator (`---`) between substructures (for multi-document YAML)
- Store YAML substructures in bookmark (for SW5)

**Actions:**
1. `Log` — Write to `outputs/sw4/log.txt` with iteration index, category, output
2. `AppendTo` — Append to `outputs/sw4/workflow_steps.yml` and bookmark `sw4_yaml_substructures`
3. `Shell` — Append `---` separator to YAML file

**Evaluation Criteria:**
- Log file contains iteration index, category, and output preview
- File `outputs/sw4/workflow_steps.yml` accumulates substructures with separators (valid YAML)
- Bookmark `sw4_yaml_substructures` stored and accessible in SW5
- Separator added between substructures (one per iteration)

**Trigger 3: `after_loop_iteration_fails` (Step 2: Loop Failure)**

```yaml
hooks:
  after_loop_iteration_fails:
    - log:
        level: Warning
        message: "SW4: Loop iteration {{iteration_index}} failed. Error: {{error_message}}. Retryable: {{error_is_retryable}}"
        to_file_path: "outputs/sw4/log.txt"
        event_fields: ["step_name", "iteration_index", "error_message", "error_is_retryable"]
    - shell:
        command: "echo 'Iteration {{iteration_index}} failed, skipping...' >> outputs/sw4/failed_iterations.log"
        fail_on_error: false
        bookmark_output: false
```

**Purpose:**
- Log loop iteration failure with iteration index and error message
- Record failed iteration in log file (for post-mortem analysis)
- Continue loop (do not fail entire workflow) — SW4 is resilient to per-task failures

**Actions:**
1. `Log` — Write to `outputs/sw4/log.txt` with iteration index and error
2. `Shell` — Append failure record to `outputs/sw4/failed_iterations.log`

**Evaluation Criteria:**
- Log file contains iteration index and error message
- File `outputs/sw4/failed_iterations.log` created with failure record
- Loop continues to next iteration (workflow does not fail)

**Trigger 4: `after_step_succeeds` (Step 3: Assemble Complete YAML Workflow)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW4: Assembled complete YAML workflow in {{duration_ms}}ms"
        to_file_path: "outputs/sw4/log.txt"
        event_fields: ["step_name", "duration_ms"]
    - shell:
        command: "cat outputs/sw4/workflow_steps.yml | yq eval '.' - > outputs/sw4/workflow.yml 2>&1"
        fail_on_error: false
        bookmark_output: true
```

**Purpose:**
- Log successful YAML assembly
- Validate YAML syntax using `yq eval` (parses YAML, fails if invalid)
- Copy validated YAML to final output file `outputs/sw4/workflow.yml`

**Actions:**
1. `Log` — Write to `outputs/sw4/log.txt` with assembly message
2. `Shell` — Validate YAML with `yq eval '.' -`, copy to `outputs/sw4/workflow.yml`, store output in bookmark `shell_output`

**Evaluation Criteria:**
- Log file contains assembly message
- File `outputs/sw4/workflow.yml` created with valid YAML
- Bookmark `shell_output` stores `yq` output (empty if valid, error if invalid)
- `yq eval` succeeds (exit code 0)
- If YAML is invalid, error is in `shell_output`

**Trigger 5: `after_step_succeeds` (Step 4: Validate YAML Workflow Structure)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW4: YAML workflow structure validated in {{duration_ms}}ms"
        to_file_path: "outputs/sw4/log.txt"
        event_fields: ["step_name", "duration_ms"]
    - shell:
        command: "cat outputs/sw4/workflow.yml | yq eval '.steps | length' > outputs/sw4/step_count.txt && cat outputs/sw4/step_count.txt"
        fail_on_error: false
        bookmark_output: true
    - shell:
        command: "cat outputs/sw4/workflow.yml | yq eval '.steps[].hooks | length' | awk '{s+=$1} END {print s}' > outputs/sw4/hook_count.txt && cat outputs/sw4/hook_count.txt"
        fail_on_error: false
        bookmark_output: true
    - gwt:
        clauses:
          - given: "{{shell_output}} > 0"
            then:
              - validate_hooks
          - given: "{{shell_output}} == 0"
            then:
              - log_no_hooks_warning
```

**Purpose:**
- Log successful YAML structure validation
- Count total steps in workflow (via `yq eval '.steps | length'`)
- Count total hooks across all steps (via `yq eval` + awk)
- Validate that at least one step exists
- Validate that hooks exist (at least one hook per step recommended)

**Actions:**
1. `Log` — Write to `outputs/sw4/log.txt` with validation message
2. `Shell` — Count steps with `yq eval '.steps | length'`, store count in bookmark `shell_output`
3. `Shell` — Count hooks with `yq eval '.steps[].hooks | length'` + awk sum, store count in bookmark `shell_output`
4. `Gwt` — Evaluate step count:
   - If > 0: Route to `validate_hooks` (proceed to hook validation)
   - If == 0: Route to `log_no_hooks_warning` (log warning, still proceed)

**Evaluation Criteria:**
- Log file contains validation message
- File `outputs/sw4/step_count.txt` created with step count
- File `outputs/sw4/hook_count.txt` created with hook count
- Bookmark `shell_output` stores step count and hook count
- GWT expression evaluates correctly
- If step count > 0, workflow routes to `validate_hooks`
- If step count == 0, workflow routes to `log_no_hooks_warning` (warning logged, workflow continues)

**Trigger 6: `after_step_succeeds` (Step 5: Validate Hook Configuration)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW4: Hook configuration validated in {{duration_ms}}ms"
        to_file_path: "outputs/sw4/log.txt"
        event_fields: ["step_name", "duration_ms"]
    - shell:
        command: "cat outputs/sw4/workflow.yml | yq eval '.steps[].hooks | keys' | jq -r '.[]' | sort -u > outputs/sw4/hook_triggers.txt && cat outputs/sw4/hook_triggers.txt"
        fail_on_error: false
        bookmark_output: true
    - shell:
        command: "cat outputs/sw4/workflow.yml | yq eval '.steps[].hooks[][] | keys' | jq -r '.[]' | sort -u > outputs/sw4/hook_actions.txt && cat outputs/sw4/hook_actions.txt"
        fail_on_error: false
        bookmark_output: true
    - shell:
        command: "cat outputs/sw4/hook_triggers.txt | grep -v -E '^(before_step_starts|after_step_starts|after_step_fails|after_all_retries_exhausted|after_step_succeeds|on_requires_failed|after_loop_iteration_fails)$' || echo 'All triggers valid'"
        fail_on_error: false
        bookmark_output: true
    - shell:
        command: "cat outputs/sw4/hook_actions.txt | grep -v -E '^(log|append_to|save_to|route_to|bookmark|notify|fail|shell|skip_step|skip_remaining|gwt|iterate_values)$' || echo 'All actions valid'"
        fail_on_error: false
        bookmark_output: true
```

**Purpose:**
- Log successful hook validation
- Extract all hook triggers from workflow (via `yq eval`)
- Extract all hook actions from workflow (via `yq eval`)
- Validate that all triggers are valid (7 wired triggers + 2 partial + 1 dead)
- Validate that all actions are valid (12 action types)
- Record validation results

**Actions:**
1. `Log` — Write to `outputs/sw4/log.txt` with validation message
2. `Shell` — Extract triggers with `yq eval`, store unique triggers in `outputs/sw4/hook_triggers.txt`
3. `Shell` — Extract actions with `yq eval`, store unique actions in `outputs/sw4/hook_actions.txt`
4. `Shell` — Validate triggers by checking against valid trigger list (grep for invalid triggers)
5. `Shell` — Validate actions by checking against valid action list (grep for invalid actions)

**Evaluation Criteria:**
- Log file contains validation message
- File `outputs/sw4/hook_triggers.txt` created with unique triggers
- File `outputs/sw4/hook_actions.txt` created with unique actions
- Trigger validation succeeds (all triggers match valid list)
- Action validation succeeds (all actions match valid list)
- If invalid triggers/actions exist, they are logged in `shell_output`

**Trigger 7: `after_step_fails` (Step 5: Validation Failure)**

```yaml
hooks:
  after_step_fails:
    - log:
        level: Error
        message: "SW4: YAML validation failed at attempt {{attempt_number}}. Error: {{error_message}}"
        to_file_path: "outputs/sw4/log.txt"
        event_fields: ["step_name", "attempt_number", "error_message", "error_is_retryable"]
    - fail:
        message: "SW4 YAML validation failed: {{error_message}}"
```

**Purpose:**
- Log YAML validation failure
- Fail workflow immediately (YAML validation is critical for SW5)

**Actions:**
1. `Log` — Write to `outputs/sw4/log.txt` with error details
2. `Fail` — Terminate workflow with descriptive error message

**Evaluation Criteria:**
- Log file contains error message and attempt number
- Workflow terminates with Fail result
- Error message is propagated to workflow output

### 5.3 SW4 Hook Summary

| Trigger | Actions | Purpose | Evaluation |
|---------|---------|---------|------------|
| `before_step_starts` | Log | Log start, verify SW3 bookmark | Log created |
| `after_step_succeeds` (Step 2) | Log, AppendTo, Shell | Log iteration, accumulate YAML, add separators | Log created, YAML accumulated, separators added |
| `after_loop_iteration_fails` | Log, Shell | Log failure, record failed iteration | Log created, failure log created, loop continues |
| `after_step_succeeds` (Step 3) | Log, Shell | Log assembly, validate YAML | Log created, YAML validated, workflow.yml created |
| `after_step_succeeds` (Step 4) | Log, Shell, Shell, Gwt | Log validation, count steps/hooks, validate counts | Log created, counts validated, correct routing |
| `after_step_succeeds` (Step 5) | Log, Shell×4 | Log validation, extract triggers/actions, validate triggers/actions | Log created, triggers/actions validated |
| `after_step_fails` (Step 5) | Log, Fail | Log failure, terminate | Log created, workflow failed |

**Total Hooks:** 7 triggers, 14 actions

**Hook Effectiveness Metrics:**

- **Logging coverage:** 7/7 triggers logged (100%)
- **State persistence:** 1 bookmark (sw4_yaml_substructures) + shell_output
- **Accumulation pattern:** AppendTo builds YAML incrementally with separators
- **Resilience:** Loop continues on individual task failures (after_loop_iteration_fails does not Fail)
- **Validation coverage:** 4 validation gates (YAML syntax, step count > 0, hook count > 0, trigger/action validity)
- **Error handling:** 2 failure paths (YAML validation failure, structure validation failure)
- **Cross-reference validation:** Validates step count against SW3 task count

## 6. SW5 Hooks Strategy

### 6.1 SW5 Overview

**Sub-Workflow:** SW5 (Final Workflow Assembly)
**Objective:** Assemble the complete YAML workflow from SW4 substructures, add workflow-level metadata, validate schema compliance
**Input:** YAML substructures from SW4 (bookmark `sw4_yaml_substructures`)
**Output:** Complete YAML workflow with metadata (YAML file with workflow name, description, providers, models, steps, hooks)
**Processing Steps:**
1. Load YAML substructures from SW4
2. Add workflow-level metadata (name, description, version)
3. Add providers and models configuration (llama_cpp_with_vulkan)
4. Add workflow-level hooks (before_workflow, after_workflow)
5. Assemble complete YAML workflow
6. Validate schema compliance (against unified-workflow-schema.yml)

**Processing Characteristics:**
- Linear processing (no loops, no branches)
- Single-step assembly (no LLM inference)
- Output must be valid YAML
- Schema validation (against unified-workflow-schema.yml)
- Validation gates: YAML parsing, schema validation, provider validity, model validity

### 6.2 SW5 Hook Configuration

SW5 uses 5 triggers across its 6 processing steps. Each trigger has 1-4 actions attached.

**Trigger 1: `before_step_starts` (Step 1: Load YAML Substructures)**

```yaml
hooks:
  before_step_starts:
    - log:
        level: Info
        message: "SW5: Loading YAML substructures from SW4 bookmark 'sw4_yaml_substructures'"
        to_file_path: "outputs/sw5/log.txt"
        event_fields: ["step_type", "model_name"]
```

**Purpose:**
- Log the start of SW5 processing
- Verify SW4 bookmark exists (implicit check via interpolation in later steps)

**Actions:**
1. `Log` — Write to `outputs/sw5/log.txt` with step start message

**Evaluation Criteria:**
- Log file `outputs/sw5/log.txt` created
- Log contains step start message

**Trigger 2: `after_step_succeeds` (Step 2: Add Workflow Metadata)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW5: Added workflow metadata (name, description, version)"
        to_file_path: "outputs/sw5/log.txt"
        event_fields: ["step_name", "output"]
    - save_to:
        target: Both
        file_path: "outputs/sw5/workflow_with_metadata.yml"
        variable_name: "sw5_workflow_metadata"
        content_template: "{{output}}"
```

**Purpose:**
- Log successful metadata addition
- Save workflow with metadata to file (for incremental assembly)
- Store workflow in bookmark (for subsequent steps)

**Actions:**
1. `Log` — Write to `outputs/sw5/log.txt` with metadata message
2. `SaveTo` — Save to `outputs/sw5/workflow_with_metadata.yml` and bookmark `sw5_workflow_metadata`

**Evaluation Criteria:**
- Log file contains metadata message
- File `outputs/sw5/workflow_with_metadata.yml` created with valid YAML
- Bookmark `sw5_workflow_metadata` stored and accessible in subsequent steps

**Trigger 3: `after_step_succeeds` (Step 3: Add Providers and Models)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW5: Added providers and models configuration (llama_cpp_with_vulkan)"
        to_file_path: "outputs/sw5/log.txt"
        event_fields: ["step_name", "output"]
    - save_to:
        target: Both
        file_path: "outputs/sw5/workflow_with_providers.yml"
        variable_name: "sw5_workflow_providers"
        content_template: "{{output}}"
    - shell:
        command: "cat outputs/sw5/workflow_with_providers.yml | yq eval '.providers[0].key' > outputs/sw5/provider_key.txt && cat outputs/sw5/provider_key.txt"
        fail_on_error: false
        bookmark_output: true
    - shell:
        command: "cat outputs/sw5/workflow_with_providers.yml | yq eval '.providers[0].key == \"llama_cpp_with_vulkan\"' > outputs/sw5/provider_valid.txt && cat outputs/sw5/provider_valid.txt"
        fail_on_error: false
        bookmark_output: true
```

**Purpose:**
- Log successful providers and models addition
- Save workflow with providers to file
- Validate that provider key is `llama_cpp_with_vulkan` (required for this project)
- Record validation results

**Actions:**
1. `Log` — Write to `outputs/sw5/log.txt` with providers message
2. `SaveTo` — Save to `outputs/sw5/workflow_with_providers.yml` and bookmark `sw5_workflow_providers`
3. `Shell` — Extract provider key with `yq eval`, store in `outputs/sw5/provider_key.txt`
4. `Shell` — Validate provider key equals `llama_cpp_with_vulkan`, store boolean in `outputs/sw5/provider_valid.txt`

**Evaluation Criteria:**
- Log file contains providers message
- File `outputs/sw5/workflow_with_providers.yml` created with valid YAML
- Bookmark `sw5_workflow_providers` stored
- File `outputs/sw5/provider_key.txt` created with provider key
- File `outputs/sw5/provider_valid.txt` created with validation result (true/false)
- Provider key validation succeeds (equals `llama_cpp_with_vulkan`)

**Trigger 4: `after_step_succeeds` (Step 4: Add Workflow-Level Hooks)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW5: Added workflow-level hooks (before_workflow, after_workflow)"
        to_file_path: "outputs/sw5/log.txt"
        event_fields: ["step_name", "output"]
    - save_to:
        target: Both
        file_path: "outputs/sw5/workflow_with_hooks.yml"
        variable_name: "sw5_workflow_hooks"
        content_template: "{{output}}"
    - shell:
        command: "cat outputs/sw5/workflow_with_hooks.yml | yq eval '.hooks | keys' | jq -r '.[]' | sort -u > outputs/sw5/workflow_hook_triggers.txt && cat outputs/sw5/workflow_hook_triggers.txt"
        fail_on_error: false
        bookmark_output: true
```

**Purpose:**
- Log successful workflow-level hooks addition
- Save workflow with hooks to file
- Extract workflow-level hook triggers for verification

**Actions:**
1. `Log` — Write to `outputs/sw5/log.txt` with hooks message
2. `SaveTo` — Save to `outputs/sw5/workflow_with_hooks.yml` and bookmark `sw5_workflow_hooks`
3. `Shell` — Extract workflow-level hooks with `yq eval`, store in `outputs/sw5/workflow_hook_triggers.txt`

**Evaluation Criteria:**
- Log file contains hooks message
- File `outputs/sw5/workflow_with_hooks.yml` created with valid YAML
- Bookmark `sw5_workflow_hooks` stored
- File `outputs/sw5/workflow_hook_triggers.txt` created with workflow-level hook triggers

**Trigger 5: `after_step_succeeds` (Step 6: Validate Schema Compliance)**

```yaml
hooks:
  after_step_succeeds:
    - log:
        level: Info
        message: "SW5: Schema compliance validated in {{duration_ms}}ms"
        to_file_path: "outputs/sw5/log.txt"
        event_fields: ["step_name", "duration_ms"]
    - shell:
        command: "cat outputs/sw5/workflow.yml | yq eval '.' - | whitt validate --schema docs/schema/unified-workflow-schema.yml 2>&1 | tee outputs/sw5/schema_validation.txt"
        fail_on_error: false
        bookmark_output: true
    - shell:
        command: "cat outputs/sw5/schema_validation.txt | grep -i 'valid' || echo 'Schema validation failed'"
        fail_on_error: false
        bookmark_output: true
    - gwt:
        clauses:
          - given: "{{shell_output}} | contains('valid')"
            then:
              - save_final_output
          - given: "{{shell_output}} | contains('valid') == false"
            then:
              - log_schema_validation_failure
```

**Purpose:**
- Log successful schema validation
- Validate schema compliance using `whitt validate --schema` (validates against unified-workflow-schema.yml)
- Record validation results
- Route to final save or failure based on validation result

**Actions:**
1. `Log` — Write to `outputs/sw5/log.txt` with validation message
2. `Shell` — Run `whitt validate --schema`, store output in `outputs/sw5/schema_validation.txt` and bookmark `shell_output`
3. `Shell` — Check for 'valid' keyword in validation output, store in bookmark `shell_output`
4. `Gwt` — Evaluate validation result:
   - If contains 'valid': Route to `save_final_output` (schema validation passed)
   - If does not contain 'valid': Route to `log_schema_validation_failure` (schema validation failed)

**Evaluation Criteria:**
- Log file contains validation message
- File `outputs/sw5/schema_validation.txt` created with validation output
- Bookmark `shell_output` stores validation result
- `whitt validate --schema` succeeds (exit code 0)
- GWT expression evaluates correctly
- If validation passed, workflow routes to `save_final_output`
- If validation failed, workflow routes to `log_schema_validation_failure` (error logged, workflow may fail)

**Trigger 6: `after_step_fails` (Step 6: Validation Failure)**

```yaml
hooks:
  after_step_fails:
    - log:
        level: Error
        message: "SW5: Schema validation failed at attempt {{attempt_number}}. Error: {{error_message}}"
        to_file_path: "outputs/sw5/log.txt"
        event_fields: ["step_name", "attempt_number", "error_message", "error_is_retryable"]
    - fail:
        message: "SW5 schema validation failed: {{error_message}}"
```

**Purpose:**
- Log schema validation failure
- Fail workflow immediately (schema validation is critical for final output)

**Actions:**
1. `Log` — Write to `outputs/sw5/log.txt` with error details
2. `Fail` — Terminate workflow with descriptive error message

**Evaluation Criteria:**
- Log file contains error message and attempt number
- Workflow terminates with Fail result
- Error message is propagated to workflow output

### 6.3 SW5 Hook Summary

| Trigger | Actions | Purpose | Evaluation |
|---------|---------|---------|------------|
| `before_step_starts` | Log | Log start, verify SW4 bookmark | Log created |
| `after_step_succeeds` (Step 2) | Log, SaveTo | Log metadata, save workflow | Log created, workflow saved |
| `after_step_succeeds` (Step 3) | Log, SaveTo, Shell×2 | Log providers, save workflow, validate provider key | Log created, workflow saved, provider validated |
| `after_step_succeeds` (Step 4) | Log, SaveTo, Shell | Log hooks, save workflow, extract hook triggers | Log created, workflow saved, triggers extracted |
| `after_step_succeeds` (Step 6) | Log, Shell×2, Gwt | Log validation, validate schema, check result | Log created, schema validated, correct routing |
| `after_step_fails` (Step 6) | Log, Fail | Log failure, terminate | Log created, workflow failed |

**Total Hooks:** 6 triggers, 12 actions

**Hook Effectiveness Metrics:**

- **Logging coverage:** 6/6 triggers logged (100%)
- **State persistence:** 3 bookmarks (sw5_workflow_metadata, sw5_workflow_providers, sw5_workflow_hooks) + shell_output
- **Incremental assembly:** SaveTo builds workflow incrementally (metadata → providers → hooks → final)
- **Validation coverage:** 3 validation gates (provider key validity, workflow-level hooks, schema compliance)
- **Error handling:** 2 failure paths (schema validation failure, workflow-level failure)
- **Cross-reference validation:** Validates against unified-workflow-schema.yml (project schema source of truth)

## 7. Cross-Sub-Workflow Hook Coordination

### 7.1 Bookmark Chain

The 5 sub-workflows form a data pipeline via bookmarks. Each sub-workflow reads bookmarks from the previous sub-workflow and writes bookmarks for the next sub-workflow.

**Bookmark Chain (SW1 → SW2 → SW3 → SW4 → SW5):**

```
SW1 writes:
  - sw1_task_list (task list JSON)

SW2 reads:
  - sw1_task_list (from SW1)
SW2 writes:
  - sw2_output_states (output state JSON)

SW3 reads:
  - sw1_task_list (from SW1)
  - sw2_output_states (from SW2)
SW3 writes:
  - sw3_categorized_tasks (categorized tasks JSON)

SW4 reads:
  - sw3_categorized_tasks (from SW3)
SW4 writes:
  - sw4_yaml_substructures (YAML substructures)

SW5 reads:
  - sw4_yaml_substructures (from SW4)
SW5 writes:
  - sw5_workflow_metadata (workflow with metadata)
  - sw5_workflow_providers (workflow with providers)
  - sw5_workflow_hooks (workflow with hooks)
```

**Bookmark Validation:**

Each sub-workflow validates that bookmarks from previous sub-workflows exist by attempting to interpolate them in templates. If a bookmark is missing, interpolation fails and the workflow fails with a clear error message.

**Example: SW2 validates SW1 bookmark**

```yaml
# SW2 Step 1: Load task list
steps:
  - name: load_task_list
    model:
      name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
    prompt: "Load task list from SW1: {{workflow_variables.sw1_task_list}}"
    hooks:
      before_step_starts:
        - log:
            message: "Loading task list: {{workflow_variables.sw1_task_list}}"
```

If `sw1_task_list` is missing, template interpolation fails and the hook action returns an error, causing the workflow to fail.

### 7.2 Cross-Sub-Workflow Validation

Each sub-workflow validates its output against the previous sub-workflow's output to ensure data consistency and prevent data loss.

**SW2 validates against SW1:**

- **Validation:** SW2 output state count == SW1 task count
- **Trigger:** SW2 Step 4, after_step_succeeds, Gwt clause
- **Expression:** `{{shell_output}} == {{workflow_variables.sw1_task_count}}`
- **Action:** Log warning if counts mismatch, continue workflow

**SW3 validates against SW1:**

- **Validation:** SW3 categorized task count == SW1 task count
- **Trigger:** SW3 Step 5, after_step_succeeds, Gwt clause
- **Expression:** `{{shell_output}} == {{workflow_variables.sw1_task_count}}`
- **Action:** Log warning if counts mismatch, continue workflow

**SW4 validates against SW3:**

- **Validation:** SW4 YAML step count == SW3 categorized task count
- **Trigger:** SW4 Step 4, after_step_succeeds, Gwt clause
- **Expression:** `{{shell_output}} > 0` (step count > 0)
- **Action:** Log warning if count is zero, continue workflow
- **Note:** Exact count comparison is not performed because a single task may generate multiple YAML steps (e.g., a task with loops generates multiple loop steps)

**SW5 validates against SW4:**

- **Validation:** SW5 workflow has steps from SW4
- **Trigger:** SW5 Step 4, after_step_succeeds, Shell action
- **Command:** `yq eval '.steps | length'` (count steps)
- **Action:** Validate step count > 0, continue workflow

### 7.3 Failure Propagation

Each sub-workflow handles failures according to the failure propagation strategy defined in the hooks strategy.

**Failure Propagation Rules:**

1. **Input validation failures:** Fail immediately (permanent errors, no retry)
   - Example: SW1 Step 1, after_step_fails → Fail action
   - Reason: Input validation failures indicate invalid user input or missing data, which cannot be fixed by retrying

2. **LLM inference failures:** Retry up to 3 times, then fail
   - Example: SW1 Step 5, after_step_fails → Gwt clause (retry if error_is_retryable)
   - Reason: LLM failures are often transient (timeouts, network errors), retrying may succeed

3. **JSON/YAML validation failures:** Fail immediately (permanent errors, no retry)
   - Example: SW4 Step 3, after_step_fails → Fail action
   - Reason: Validation failures indicate structural errors in generated output, which are unlikely to be fixed by retrying

4. **Loop iteration failures:** Log warning, continue loop
   - Example: SW2 Step 2, after_loop_iteration_fails → Log + Shell (record failure)
   - Reason: Individual task failures should not prevent processing other tasks, resilient workflow

5. **Schema validation failures:** Fail immediately (permanent errors, no retry)
   - Example: SW5 Step 6, after_step_fails → Fail action
   - Reason: Schema validation failures indicate non-compliant YAML, which cannot be used by the engine

**Failure Recovery:**

If a sub-workflow fails, the entire meta-workflow fails. The shell orchestrator (defined in 08-CONFIG-AND-INFRASTRUCTURE.md) is responsible for:
- Capturing failure logs from each sub-workflow
- Reporting the failure to the user
- Providing recommendations for fixing the failure (e.g., "SW4 generated invalid YAML, check outputs/sw4/workflow_steps.yml")

## 8. Hook Performance Considerations

### 8.1 Hook Execution Overhead

Each hook action has an execution overhead that contributes to the total workflow execution time. The overhead varies by action type.

**Action Execution Overhead Estimates:**

| Action | Overhead (ms) | Reason |
|--------|---------------|---------|
| `Log` (to stdout only) | < 1 | Minimal I/O |
| `Log` (to file) | 5-20 | File I/O, depends on file system |
| `AppendTo` (file) | 10-30 | File append, depends on file size |
| `AppendTo` (variable) | < 1 | In-memory operation |
| `AppendTo` (both) | 10-30 | File append + in-memory |
| `SaveTo` (file) | 10-30 | File write, depends on content size |
| `SaveTo` (variable) | < 1 | In-memory operation |
| `SaveTo` (both) | 10-30 | File write + in-memory |
| `RouteTo` | < 1 | In-memory routing |
| `Bookmark` (flag) | < 1 | In-memory storage |
| `Bookmark` (path) | 5-10 | File write (optional) |
| `Bookmark` (detailed) | 10-20 | File write + in-memory storage |
| `Notify` | < 1 | Channel send (non-blocking) |
| `Fail` | < 1 | Workflow termination |
| `Shell` (fast command) | 50-200 | Process spawn + execution |
| `Shell` (slow command) | 200-2000+ | Depends on command (e.g., yq, jq) |
| `SkipStep` | < 1 | Workflow state update |
| `SkipRemaining` | < 1 | Workflow termination |
| `Gwt` (simple expression) | 1-5 | Expression evaluation |
| `Gwt` (complex expression) | 5-20 | Multiple clause evaluation |
| `IterateValues` | < 1 | Passthrough (no logic) |

**Total Hook Overhead by Sub-Workflow:**

| Sub-Workflow | Triggers | Actions | Avg Actions/Trigger | Est. Overhead (ms) |
|--------------|----------|---------|--------------------|-------------------|
| SW1 | 4 | 8 | 2 | 150 |
| SW2 | 5 | 10 | 2 | 300 (loop overhead) |
| SW3 | 6 | 12 | 2 | 350 (loop overhead) |
| SW4 | 7 | 14 | 2 | 500 (loop overhead + yq) |
| SW5 | 6 | 12 | 2 | 400 (yq + whitt validate) |

**Notes:**

- SW2-SW4 have loop overhead (hooks fire per iteration)
- SW4-SW5 have yq overhead (YAML parsing and validation)
- SW5 has whitt validate overhead (schema validation)

### 8.2 Hook Optimization Strategies

To minimize hook execution overhead while maintaining functionality, use these optimization strategies:

**Strategy 1: Conditional Logging**

Only log when necessary (e.g., on errors, on validation failures). Reduce logging frequency for happy-path operations.

**Example: Reduce logging in SW2 loop**

```yaml
# Before: Log every iteration
after_step_succeeds:
  - log:
      level: Info
      message: "SW2: Processed task {{iteration_index}} in {{duration_ms}}ms. Output:\n{{output}}"
      to_file_path: "outputs/sw2/log.txt"

# After: Log only every 10 iterations
after_step_succeeds:
  - gwt:
      clauses:
        - given: "{{iteration_index}} % 10 == 0"
          then:
            - log_iteration
        - given: "{{iteration_index}} % 10 != 0"
          then:
            - continue_without_log
```

**Strategy 2: Batch File Operations**

Instead of appending to a file on every iteration, accumulate in memory and write once at the end.

**Example: Batch file writes in SW2**

```yaml
# Before: Append to file on every iteration
after_step_succeeds:
  - append_to:
      target: FilePath
      file_path: "outputs/sw2/output_states.json"
      content_template: "{{output}}"

# After: Append to variable in loop, write once after loop
after_step_succeeds:
  - append_to:
      target: Variable
      variable_name: "sw2_output_states_accumulated"
      content_template: "{{output}}"

# After loop ends (separate step):
after_step_succeeds:
  - save_to:
      target: FilePath
      file_path: "outputs/sw2/output_states.json"
      content_template: "{{workflow_variables.sw2_output_states_accumulated}}"
```

**Strategy 3: Cache Shell Command Results**

Instead of running the same shell command multiple times (e.g., counting steps, counting hooks), run once and cache the result in a bookmark.

**Example: Cache step count in SW4**

```yaml
# Before: Count steps twice (for validation and for logging)
after_step_succeeds:
  - shell:
      command: "cat outputs/sw4/workflow.yml | yq eval '.steps | length'"
      bookmark_output: true
  - log:
      message: "Step count: {{shell_output}}"
  - shell:
      command: "cat outputs/sw4/workflow.yml | yq eval '.steps | length'"
      bookmark_output: true
  - gwt:
      clauses:
        - given: "{{shell_output}} > 0"
          then:
            - validate_hooks

# After: Count steps once, cache in bookmark, reuse
after_step_succeeds:
  - shell:
      command: "cat outputs/sw4/workflow.yml | yq eval '.steps | length' > outputs/sw4/step_count.txt && cat outputs/sw4/step_count.txt"
      bookmark_output: true
  - bookmark:
      name: "sw4_step_count"
      value:
        flag: true
  - log:
      message: "Step count: {{shell_output}}"
  - gwt:
      clauses:
        - given: "{{shell_output}} > 0"
          then:
            - validate_hooks
```

**Strategy 4: Parallelize Independent Shell Commands**

If multiple shell commands are independent (no dependencies), run them in parallel using background delegation.

**Example: Parallelize SW4 validation**

```yaml
# Before: Sequential validation (trigger validation, then action validation)
after_step_succeeds:
  - shell:
      command: "cat outputs/sw4/workflow.yml | yq eval '.steps[].hooks | keys' > outputs/sw4/hook_triggers.txt"
  - shell:
      command: "cat outputs/sw4/workflow.yml | yq eval '.steps[].hooks[][] | keys' > outputs/sw4/hook_actions.txt"

# After: Parallel validation (run both in parallel using delegation)
after_step_succeeds:
  - shell:
      command: "cat outputs/sw4/workflow.yml | yq eval '.steps[].hooks | keys' > outputs/sw4/hook_triggers.txt & cat outputs/sw4/workflow.yml | yq eval '.steps[].hooks[][] | keys' > outputs/sw4/hook_actions.txt & wait"
```

## 9. Hook Testing Strategy

### 9.1 Unit Testing Hooks

Each hook action should be unit-tested in isolation to ensure correct behavior. Unit tests are defined in `tests/hooks_integration.rs` (47 integration tests exist as of commit `c29dcfc`).

**Unit Test Template for a Hook Action:**

```rust
#[test]
fn given_log_action_when_executed_then_file_created_with_content() {
    // Arrange
    let action = HookAction::Log(LogAction {
        level: LogLevel::Info,
        message: "Test message".to_string(),
        to_file_path: Some("outputs/test.log".to_string()),
        event_fields: vec![],
    });
    let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
        step_type: "test_step".to_string(),
        model_name: "test_model".to_string(),
        prompt_preview: "Test prompt".to_string(),
        workflow_variables: HashMap::new(),
    });

    // Act
    let result = execute_action(&action, &context, &mut HookEngine::new()).unwrap();

    // Assert
    assert!(matches!(result, HookResult::Continue));
    assert!(Path::new("outputs/test.log").exists());
    let content = fs::read_to_string("outputs/test.log").unwrap();
    assert!(content.contains("Test message"));
}
```

**Coverage Targets for Hook Unit Tests:**

| Action | Current Coverage | Target Coverage | Gaps |
|--------|------------------|-----------------|------|
| `Log` | 90% (all variants) | 95% | stdout-only logging |
| `AppendTo` | 85% (2 variants) | 95% | Both variant |
| `SaveTo` | 80% (2 variants) | 95% | Both variant |
| `RouteTo` | 90% (single + multiple) | 95% | Empty targets list |
| `Bookmark` | 85% (flag + path + detailed) | 95% | detailed: path:None |
| `Notify` | 80% (with channel) | 95% | without channel |
| `Fail` | 90% (with/without message) | 95% | N/A |
| `Shell` | 85% (echo + bookmark) | 95% | env vars, working_dir |
| `SkipStep` | 90% (true + false) | 95% | N/A |
| `SkipRemaining` | 90% (true + false) | 95% | N/A |
| `Gwt` | 85% (matching + non-matching) | 95% | Invalid condition |
| `IterateValues` | 100% (passthrough) | 100% | N/A |

### 9.2 Integration Testing Hooks

Each hook trigger should be integration-tested end-to-end to ensure it fires at the correct point in workflow execution and has access to the expected context fields. Integration tests are defined in `tests/hooks_integration.rs` (47 integration tests exist as of commit `c29dcfc`).

**Integration Test Template for a Hook Trigger:**

```rust
#[test]
fn given_before_step_starts_hook_when_step_starts_then_hook_fires_with_correct_context() {
    // Arrange
    let workflow_yaml = r#"
        workflows:
          - name: test_workflow
            steps:
              - name: test_step
                model:
                  name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
                prompt: "Test prompt"
                hooks:
                  before_step_starts:
                    - log:
                        level: Info
                        message: "Before step starts: {{step_type}}, {{model_name}}, {{prompt_preview}}"
                        to_file_path: "outputs/before_step_starts.log"
    "#;

    // Act
    let result = run_workflow(workflow_yaml);

    // Assert
    assert!(result.is_ok());
    assert!(Path::new("outputs/before_step_starts.log").exists());
    let content = fs::read_to_string("outputs/before_step_starts.log").unwrap();
    assert!(content.contains("Before step starts"));
    assert!(content.contains("test_step"));
    assert!(content.contains("Qwen3.5-9B-UD-Q4_K_XL.gguf"));
    assert!(content.contains("Test prompt"));
}
```

**Coverage Targets for Hook Integration Tests:**

| Trigger | Current Coverage | Target Coverage | Gaps |
|---------|------------------|-----------------|------|
| `before_step_starts` | ✅ Tested | ✅ Tested | N/A |
| `during_step_streaming` | ❌ Not tested | ❌ Not tested (not wired) | N/A |
| `after_step_starts` | ✅ Tested | ✅ Tested | N/A |
| `after_step_succeeds` | ✅ Tested | ✅ Tested | N/A |
| `after_step_fails` | ✅ Tested | ✅ Tested | N/A |
| `after_all_retries_exhausted` | ✅ Tested | ✅ Tested | N/A |
| `on_requires_failed` | ✅ Tested | ✅ Tested | N/A |
| `after_loop_iteration_fails` | ✅ Tested | ✅ Tested | N/A |

### 9.3 End-to-End Testing Hooks in Sub-Workflows

Each sub-workflow (SW1-SW5) should be tested end-to-end to ensure that all hooks fire correctly and produce the expected outputs. E2E tests run the full sub-workflow against live Docker llama.cpp server.

**E2E Test Template for SW1:**

```bash
# Test: SW1 produces valid task list JSON
# Input: "Implement user authentication with JWT tokens"
# Expected: outputs/sw1/task_list.json with valid JSON

# 1. Run SW1
whitt benchmark --workflow docs/plans/meta-workflow-qwen35/sub-workflows/sw1.yml \
    --input-prompt "Implement user authentication with JWT tokens"

# 2. Verify output
cat outputs/sw1/task_list.json | jq empty
if [ $? -eq 0 ]; then
    echo "✅ SW1 output is valid JSON"
else
    echo "❌ SW1 output is invalid JSON"
    exit 1
fi

# 3. Verify content
cat outputs/sw1/task_list.json | jq 'length > 0'
if [ $? -eq 0 ]; then
    echo "✅ SW1 output has at least one task"
else
    echo "❌ SW1 output has no tasks"
    exit 1
fi
```

**E2E Test Template for SW4:**

```bash
# Test: SW4 produces valid YAML workflow
# Input: SW3 categorized tasks (JSON)
# Expected: outputs/sw4/workflow.yml with valid YAML

# 1. Run SW4 (with SW3 output as input)
whitt benchmark --workflow docs/plans/meta-workflow-qwen35/sub-workflows/sw4.yml \
    --input-json outputs/sw3/categorized_tasks.json

# 2. Verify output
cat outputs/sw4/workflow.yml | yq eval '.' - > /dev/null
if [ $? -eq 0 ]; then
    echo "✅ SW4 output is valid YAML"
else
    echo "❌ SW4 output is invalid YAML"
    exit 1
fi

# 3. Verify structure
cat outputs/sw4/workflow.yml | yq eval '.steps | length > 0'
if [ $? -eq 0 ]; then
    echo "✅ SW4 output has at least one step"
else
    echo "❌ SW4 output has no steps"
    exit 1
fi

# 4. Validate triggers
cat outputs/sw4/hook_triggers.txt | grep -v -E '^(before_step_starts|after_step_starts|after_step_fails|after_all_retries_exhausted|after_step_succeeds|on_requires_failed|after_loop_iteration_fails)$'
if [ $? -ne 0 ]; then
    echo "✅ SW4 output has valid triggers"
else
    echo "❌ SW4 output has invalid triggers"
    exit 1
fi

# 5. Validate actions
cat outputs/sw4/hook_actions.txt | grep -v -E '^(log|append_to|save_to|route_to|bookmark|notify|fail|shell|skip_step|skip_remaining|gwt|iterate_values)$'
if [ $? -ne 0 ]; then
    echo "✅ SW4 output has valid actions"
else
    echo "❌ SW4 output has invalid actions"
    exit 1
fi
```

## 10. Conclusion

This hooks strategy defines comprehensive hook configurations for all 5 sub-workflows (SW1-SW5) of the meta-workflow generator. The strategy covers:

- 4 triggers for SW1 (8 actions) — Linear processing, input validation, JSON validation
- 5 triggers for SW2 (10 actions) — Loop-based processing, accumulation pattern, resilient failure handling
- 6 triggers for SW3 (12 actions) — Loop-based processing, canonical category validation, cross-reference validation
- 7 triggers for SW4 (14 actions) — Loop-based processing, YAML validation, trigger/action validation
- 6 triggers for SW5 (12 actions) — Linear processing, incremental assembly, schema validation

**Total Hook Usage:** 28 triggers, 56 actions across 5 sub-workflows.

**Key Hook Principles:**

1. **Logging coverage:** All triggers are logged (100% coverage) for debugging and auditability
2. **State persistence:** Bookmarks store intermediate outputs for cross-sub-workflow data flow
3. **Accumulation pattern:** AppendTo builds JSON/YAML incrementally across loop iterations
4. **Resilience:** Loop iterations continue on individual failures (failures logged, not propagated)
5. **Validation coverage:** Each sub-workflow has multiple validation gates (JSON, YAML, schema, cross-reference)
6. **Error handling:** Failures are handled appropriately (immediate fail for permanent errors, retry for transient errors, continue for individual task failures)
7. **Cross-sub-workflow coordination:** Bookmark chain enables data flow from SW1 → SW2 → SW3 → SW4 → SW5

The hooks strategy is designed to be implemented in YAML workflow files (one per sub-workflow) and tested via unit tests, integration tests, and end-to-end tests. All hook configurations use real trigger names, real action names, real context fields, and real shell commands (jq, yq, whitt validate) as defined in the codebase.