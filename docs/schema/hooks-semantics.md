# Hooks Semantics

## Overview

Hooks provide reactive, event-driven behavior across all schema substructure types. They replace:

- `skip_on_load_failure` → handled by `on_load_failure` hook with `skip` action
- Ad-hoc error handling → unified hook system
- Scattered lifecycle management → consistent hook interface
- Manual logging/notification → declarative hook actions

Hooks enable:
- Observability: log every event
- Resilience: retry, skip, fail workflows on conditions
- Orchestration: route, checkpoint, notify on events
- Control: abort, skip_remaining, conditional branching

---

## Hook Anatomy

### Structure

```yaml
when:                                   # alias: lifecycle_hooks
  <event_name>:                         # lifecycle event timing
    - <action_type>:                    # log, notify, append_to, etc.
        <action_parameters>
    - <another_action>:                # multiple actions per event
        <parameters>
```

### Event Timing Phases

- **before**: Pre-event (no results available)
- **during**: Mid-event (partial results streaming)
- **after**: Post-event (complete results available)
- **on**: Reactive to specific conditions (failure, timeout, retry)

---

## Hook Action Types

### `log`

Write structured logs to file.

```yaml
after_step_fails:
  log:
    to_file_path: "./workspace/logs/errors.log"
    event_fields: [step_name, error_type, error_message, timestamp]
    level: error                          # debug | info | warning | error | critical
    format: json                          # json | yaml | text (default: json)
```

**Fields**: step_name, model_id, provider_id, timestamp, duration_ms, exit_code, retry_count, loop_iteration, workflow_id, run_number

### `notify`

Send notification (MVP: log to notification channel).

```yaml
after_workflow_completes:
  notify:
    message: "Workflow {{workflow_id}} completed in {{duration_ms}}ms"
    severity: info                        # debug | info | warning | error | critical
    channels: [console, slack]            # future: slack, email, webhook
```

### `append_to`

Append structured output to file or variable.

```yaml
after_step_succeeds:
  - append_to: "./workspace/output/results.yaml"
  - append_to: analysis_results           # workflow variable
  - append_to:
      - "./workspace/output/all_runs.yaml"
      - run_history
```

### `run_script`

Execute shell command synchronously.

```yaml
on_model_load:
  run_script:
    command: "./scripts/post_load.sh {{model_id}}"
    timeout_seconds: 30
    environment:
      MODEL_PATH: "{{model_path}}"
      LOAD_TIMESTAMP: "{{now}}"
```

### `set_variable`

Set or update workflow variable.

```yaml
after_step_succeeds:
  set_variable:
    name: quality_score
    value: "{{output.quality_score}}"
    scope: workflow                       # workflow | global | step (default: workflow)
```

### `skip`

Skip current operation with optional reason.

```yaml
on_load_failure:
  skip:
    reason: "Model {{model_id}} unavailable, using fallback"
    log_level: warning
```

**Replaces**: `skip_on_load_failure: true`

### `fail`

Fail current operation with custom message.

```yaml
after_all_retries_exhausted:
  fail:
    message: "Step {{step_name}} failed after {{total_attempts}} attempts"
    error_code: RETRY_EXHAUSTED
    propagate: true                       # true = fail workflow, false = fail step only
```

### `retry`

Retry current operation with custom config.

```yaml
on_retry:
  retry:
    max_attempts: 5                       # override default
    backoff: exponential
    initial_delay: "2s"                   # override default
    max_delay: "60s"
    jitter: true
```

### `checkpoint`

Create workflow checkpoint.

```yaml
after_step_succeeds:
  checkpoint:
    level: full                           # full | minimal | metadata-only
    include_outputs: true
    include_state: true
    note: "Completed {{step_name}}"
```

### `webhook`

Send HTTP request (future).

```yaml
after_workflow_completes:
  webhook:
    url: "https://api.example.com/webhooks/workflow"
    method: POST
    headers:
      Authorization: "Bearer {{api_token}}"
      Content-Type: "application/json"
    body:
      workflow_id: "{{workflow_id}}"
      status: completed
      duration_ms: "{{duration_ms}}"
    timeout_seconds: 10
```

### `emit_event`

Emit custom event for other hooks to react to.

```yaml
after_step_fails:
  emit_event:
    name: step_critical_failure
    payload:
      step_name: "{{step_name}}"
      error_type: "{{error.type}}"
      timestamp: "{{now}}"
```

**Then react**:
```yaml
on_event:
  gwt:
    - given: "event.name == 'step_critical_failure' && event.payload.error_type == 'OOM'"
      then: { route_to: handle_oom }
```

### `gwt` (Given-When-Then)

Deterministic conditional routing in hooks.

```yaml
after_step_fails:
  gwt:
    - given: "error.is_retryable == true"
      when: "error is transient or recoverable"
      then: { route_to: retry_same_step }
    - given: "error.is_retryable == false"
      then: { route_to: fail_workflow }
    - given: "error.code == 'RATE_LIMIT'"
      then: { route_to: switch_provider }
```

**Then clauses**:
- `route_to: step_name` or `[step_a, step_b]` or `sub_workflow_name`
- `save_to: variable_name` or `"./path/file.yaml"` or `[var, "./path"]`
- `format: yaml` or `string`
- `bookmark: true` (alias: `checkpoint`, `save_state`)
- `notify: { message: "Done" }`
- `fail: "reason message"`
- `skip_step: true`
- `skip_loop: true`
- `skip_sub_workflow: true`
- `skip_remaining: true`

---

## Per-Substructure Hooks

### Models (`models:` section)

Model lifecycle hooks. Attach to model definitions.

```yaml
models:
  "primary-analyzer":
    name: "Primary Code Analyzer"
    host:
      type: lmstudio
    when:                                   # model-level hooks
      on_load:
        log:
          to_file_path: "./workspace/logs/models.log"
          event_fields: [model_id, load_timestamp, memory_allocated_mb]
      on_unload:
        log:
          to_file_path: "./workspace/logs/models.log"
          event_fields: [model_id, unload_timestamp, uptime_duration_ms]
      on_load_failure:
        - skip:
            reason: "Model unavailable, using fallback"
            log_level: warning
        - notify:
            message: "Model {{model_id}} load failed: {{error.message}}"
      on_timeout:
        - retry:
            max_attempts: 2
            backoff: linear
        - log:
            to_file_path: "./workspace/logs/timeouts.log"
            event_fields: [model_id, operation, timeout_seconds]
      on_context_overflow:
        - fail:
            message: "Context window exceeded for {{model_id}}"
          emit_event:
            name: model_context_overflow
            payload: { model_id: "{{model_id}}" }
      on_rate_limit:
        - gwt:
            - given: "rate_limit.is_provider_level"
              then: { route_to: switch_provider }
            - given: "rate_limit.is_model_level"
              then: { route_to: throttle_requests }
```

**Model hook events**:
- `on_load`: Model loaded into memory
- `on_unload`: Model evicted from memory
- `on_load_failure`: Model failed to load (replaces `skip_on_load_failure`)
- `on_timeout`: Model inference timeout
- `on_context_overflow`: Context window exceeded
- `on_rate_limit`: Provider rate limit hit (model-scoped)

---

### Providers (`providers:` section)

Provider connection and reliability hooks.

```yaml
providers:
  lmstudio:
    config:
      host: localhost
      port: 1234
    when:                                   # provider-level hooks
      on_connect:
        log:
          to_file_path: "./workspace/logs/providers.log"
          event_fields: [provider_id, connection_timestamp, latency_ms]
        notify:
          message: "Connected to {{provider_id}}"
      on_disconnect:
        - log:
            to_file_path: "./workspace/logs/providers.log"
            event_fields: [provider_id, disconnect_reason, uptime_duration_ms]
        - emit_event:
            name: provider_disconnect
            payload: { provider_id: "{{provider_id}}" }
      on_health_check_fail:
        - log:
            to_file_path: "./workspace/logs/health.log"
            event_fields: [provider_id, check_type, failure_reason]
            level: warning
        - gwt:
            - given: "health_check.consecutive_failures >= 3"
              then: { route_to: failover_provider }
      on_rate_limit:
        - notify:
            message: "Rate limit on {{provider_id}}: {{rate_limit.remaining_requests}} remaining"
        - run_script:
            command: "./scripts/backoff_strategy.sh {{provider_id}}"
      on_retry_exhausted:
        - fail:
            message: "All retries exhausted for {{provider_id}}"
            error_code: PROVIDER_RETRY_EXHAUSTED
        - emit_event:
            name: provider_unavailable
            payload: { provider_id: "{{provider_id}}" }
      on_provider_failover:
        - log:
            to_file_path: "./workspace/logs/failover.log"
            event_fields: [from_provider, to_provider, failover_reason]
        - notify:
            message: "Failing over from {{from_provider}} to {{to_provider}}"
```

**Provider hook events**:
- `on_connect`: Provider connection established
- `on_disconnect`: Provider connection lost
- `on_health_check_fail`: Provider health check failed
- `on_rate_limit`: Provider rate limit hit
- `on_retry_exhausted`: All provider retries exhausted
- `on_provider_failover`: Switching to fallback provider

---

### Agent Workflows (`agentic_workflow:` top-level)

Workflow lifecycle hooks. Defined at L2 workflow level.

```yaml
agentic_workflow:
  when:                                   # workflow-level hooks (apply to all steps)
    before_workflow_starts:
      - log:
          to_file_path: "./workspace/logs/workflow.log"
          event_fields: [workflow_id, start_timestamp, run_number]
      - checkpoint:
          level: minimal
          note: "Workflow start"
      - set_variable:
          name: workflow_start_time
          value: "{{now}}"

    after_workflow_completes:
      - log:
          to_file_path: "./workspace/logs/workflow.log"
          event_fields: [workflow_id, end_timestamp, duration_ms, status]
      - append_to: "./workspace/output/workflow_summary.yaml"
      - notify:
          message: "Workflow {{workflow_id}} completed successfully in {{duration_ms}}ms"

    after_workflow_fails:
      - log:
          to_file_path: "./workspace/logs/workflow.log"
          event_fields: [workflow_id, failure_reason, failed_step, error_type]
          level: critical
      - checkpoint:
          level: full
          include_state: true
          note: "Workflow failed at {{failed_step}}"
      - emit_event:
          name: workflow_failure
          payload:
            workflow_id: "{{workflow_id}}"
            failed_step: "{{failed_step}}"
            error_type: "{{error.type}}"

    after_workflow_aborts:
      - log:
          to_file_path: "./workspace/logs/workflow.log"
          event_fields: [workflow_id, abort_reason, abort_step]
          level: warning
      - skip_remaining: true

    on_checkpoint:
      log:
        to_file_path: "./workspace/logs/checkpoints.log"
        event_fields: [checkpoint_id, checkpoint_level, checkpoint_timestamp]

    on_checkpoint_restore:
      - log:
          to_file_path: "./workspace/logs/checkpoints.log"
          event_fields: [checkpoint_id, restore_timestamp, steps_restored]
      - notify:
          message: "Restored from checkpoint {{checkpoint_id}}"

    on_step_complete:
      log:
        to_file_path: "./workspace/logs/progress.log"
        event_fields: [step_name, status, duration_ms, progress_percent]

    on_step_failure:
      - log:
          to_file_path: "./workspace/logs/failures.log"
          event_fields: [step_name, error_type, failure_timestamp]
          level: error
      - emit_event:
          name: step_failed
          payload: { step_name: "{{step_name}}", error_type: "{{error.type}}" }
```

**Workflow hook events**:
- `before_workflow_starts`: Pre-workflow initialization
- `after_workflow_completes`: Post-workflow cleanup/reporting
- `after_workflow_fails`: Workflow-level failure handling
- `after_workflow_aborts`: Workflow abort handling
- `on_checkpoint`: Checkpoint created
- `on_checkpoint_restore`: Checkpoint restored
- `on_step_complete`: Any step completes (progress tracking)
- `on_step_failure`: Any step fails

---

### Steps (`pipeline:` section)

Step-level hooks. All existing `when:` hooks preserved + new additions.

#### Base Step Hooks

```yaml
steps:
  analyze_code:
    generative_entity: "${models.primary-analyzer}"
    prompt: "Analyze code"
    when:
      before_step_starts:
        - log:
            to_file_path: "./workspace/logs/steps.log"
            event_fields: [step_name, timestamp, model_id]
        - checkpoint:
            level: minimal
            note: "Before {{step_name}}"

      during_step_streaming:
        log:
          to_file_path: "./workspace/logs/streaming.log"
          event_fields: [step_name, chunk_text, tokens_so_far, timestamp]

      after_step_succeeds:
        - append_to: "./workspace/output/analysis.yaml"
        - log:
            to_file_path: "./workspace/logs/completed.log"
            event_fields: [step_name, duration_ms, output_size]
        - checkpoint:
            level: full
            include_outputs: true

      after_step_fails:
        - log:
            to_file_path: "./workspace/logs/errors.log"
            event_fields: [step_name, error_type, error_message, timestamp]
            level: error
        - gwt:
            - given: "error.is_retryable == true"
              when: "error is transient or recoverable"
              then: { route_to: analyze_code }
            - given: "error.is_retryable == false"
              then: { route_to: abort_workflow }

      after_step_aborts:
        - log:
            to_file_path: "./workspace/logs/aborts.log"
            event_fields: [step_name, abort_reason, timestamp]
            level: warning
        - skip_remaining: true

      on_retry:                                 # NEW
        - log:
            to_file_path: "./workspace/logs/retries.log"
            event_fields: [step_name, attempt_number, retry_reason]
        - notify:
            message: "Retrying {{step_name}} (attempt {{attempt_number}})"

      on_timeout:                               # NEW
        - log:
            to_file_path: "./workspace/logs/timeouts.log"
            event_fields: [step_name, timeout_duration, operation]
            level: warning
        - retry:
            max_attempts: 2
            backoff: exponential

      on_checkpoint:                            # NEW
        log:
          to_file_path: "./workspace/logs/checkpoints.log"
          event_fields: [step_name, checkpoint_id, checkpoint_level]

      on_validation_fail:                        # NEW
        - log:
            to_file_path: "./workspace/logs/validation.log"
            event_fields: [step_name, validation_criteria, actual_value, required_value]
            level: error
        - gwt:
            - given: "validation.is_recoverable == true"
              then: { route_to: fix_validation }
            - given: "validation.is_recoverable == false"
              then: { fail: "Validation failed: {{validation.message}}" }

      after_all_retries_exhausted:
        - log:
            to_file_path: "./workspace/logs/fatal.log"
            event_fields: [step_name, total_attempts, last_error]
            level: critical
        - fail:
            message: "Step {{step_name}} failed after {{total_attempts}} attempts"
            propagate: true
```

**Base step hook events**:
- `before_step_starts`: Pre-step setup
- `during_step_streaming`: Mid-step streaming
- `after_step_succeeds`: Step success
- `after_step_fails`: Step failure
- `after_step_aborts`: Step abort
- `on_retry`: Step is being retried (NEW)
- `on_timeout`: Step timed out (NEW)
- `on_checkpoint`: Step checkpoint saved (NEW)
- `on_validation_fail`: Step output failed validation (NEW)
- `after_all_retries_exhausted`: All retries exhausted

---

#### Step-Type-Specific Hooks

Different step types support additional hooks.

##### Generative Steps (LLM calls)

```yaml
generate_summary:
  generative_entity: "${models.primary-analyzer}"
  prompt: "Generate summary"
  when:
    on_token_stream:                           # Generative-only
      log:
        to_file_path: "./workspace/logs/tokens.log"
        event_fields: [step_name, token_text, token_position, timestamp]

    on_tool_call:                             # Generative-only
      log:
        to_file_path: "./workspace/logs/tools.log"
        event_fields: [step_name, tool_name, tool_arguments, call_timestamp]

    on_tool_result:                           # Generative-only
      log:
        to_file_path: "./workspace/logs/tools.log"
        event_fields: [step_name, tool_name, result_size, execution_duration_ms, success]

    on_max_turns_reached:                     # Generative-only
      - log:
          to_file_path: "./workspace/logs/turns.log"
          event_fields: [step_name, max_turns, actual_turns]
          level: warning
      - gwt:
          - given: "turns_result.is_acceptable == true"
            then: { route_to: next_step }
          - given: "turns_result.is_acceptable == false"
            then: { fail: "Max turns reached without acceptable result" }
```

**Generative step hooks**:
- `on_token_stream`: Each token generated
- `on_tool_call`: Tool invocation started
- `on_tool_result`: Tool invocation completed
- `on_max_turns_reached`: Conversation turn limit reached

---

##### Script Steps (shell execution)

```yaml
run_tests:
  tool: shell_exec
  input:
    command: "cargo test --workspace"
    timeout_seconds: 120
  when:
    on_stdout:                                # Script-only
      log:
        to_file_path: "./workspace/logs/stdout.log"
        event_fields: [step_name, line_text, line_number]

    on_stderr:                                # Script-only
      log:
        to_file_path: "./workspace/logs/stderr.log"
        event_fields: [step_name, line_text, line_number, line_level]
        level: error

    on_exit_code:                             # Script-only
      gwt:
        - given: "exit_code == 0"
          then: { route_to: post_tests }
        - given: "exit_code > 0 && exit_code < 128"
          then: { route_to: handle_test_failure }
        - given: "exit_code >= 128"
          then:
            - log:
                to_file_path: "./workspace/logs/signals.log"
                event_fields: [step_name, signal_number, signal_name]
                level: critical
            - fail: "Process terminated by signal {{signal_name}}"
```

**Script step hooks**:
- `on_stdout`: Line written to stdout
- `on_stderr`: Line written to stderr
- `on_exit_code`: Process exited with code

---

##### Sub-Workflow Steps

```yaml
run_validation:
  sub_workflow: validate_workflow
  input:
    target_code: "{{step.analyze_code.output}}"
  when:
    on_sub_workflow_start:                    # Sub-workflow-only
      log:
        to_file_path: "./workspace/logs/sub-workflows.log"
        event_fields: [step_name, sub_workflow_id, start_timestamp]

    on_sub_workflow_complete:                 # Sub-workflow-only
      - log:
          to_file_path: "./workspace/logs/sub-workflows.log"
          event_fields: [step_name, sub_workflow_id, duration_ms, status]
      - append_to: "./workspace/output/validation.yaml"
      - emit_event:
          name: sub_workflow_completed
          payload: { sub_workflow_id: "{{sub_workflow_id}}" }

    on_sub_workflow_fail:                    # Sub-workflow-only
      - log:
          to_file_path: "./workspace/logs/sub-workflows.log"
          event_fields: [step_name, sub_workflow_id, failure_reason]
          level: error
      - gwt:
          - given: "sub_workflow.is_recoverable == true"
            then: { route_to: retry_sub_workflow }
          - given: "sub_workflow.is_recoverable == false"
            then: { fail: "Sub-workflow failed: {{failure_reason}}" }
```

**Sub-workflow step hooks**:
- `on_sub_workflow_start`: Sub-workflow execution started
- `on_sub_workflow_complete`: Sub-workflow execution completed
- `on_sub_workflow_fail`: Sub-workflow execution failed

---

##### File Operation Steps

```yaml
read_config:
  tool: file_read
  input:
    file_path: "./workspace/config.yml"
  when:
    on_file_read:                             # File operation-only
      log:
        to_file_path: "./workspace/logs/files.log"
        event_fields: [step_name, file_path, read_size_bytes, read_duration_ms]

write_output:
  tool: file_write
  input:
    file_path: "./workspace/output/results.yaml"
    content: "{{step.analyze_code.output}}"
  when:
    on_file_write:                            # File operation-only
      log:
        to_file_path: "./workspace/logs/files.log"
        event_fields: [step_name, file_path, write_size_bytes, write_duration_ms, backup_created]

    on_file_error:                            # File operation-only
      - log:
          to_file_path: "./workspace/logs/files.log"
          event_fields: [step_name, operation, file_path, error_type, error_message]
          level: error
      - gwt:
          - given: "error.is_permission_denied"
            then: { fail: "Permission denied: {{file_path}}" }
          - given: "error.is_disk_full"
            then: { fail: "Disk full, cannot write to {{file_path}}" }
          - given: "error.is_recoverable == true"
            then: { retry: { max_attempts: 3, backoff: linear } }
```

**File operation step hooks**:
- `on_file_read`: File read completed
- `on_file_write`: File write completed
- `on_file_error`: File operation error (read/write/delete)

---

## Hook Precedence and Merging

### Scope Levels

1. **L1 (workflow top-level)**: `agentic_workflow.when:` (default hooks)
2. **L3 (step-level)**: `steps.<step>.when:` (step overrides)

### Merging Rules

- **Hooks merge additively**: L2 defaults + L3 step hooks = combined hooks
- **Step hooks override workflow defaults for same event**: L3 takes precedence
- **Action arrays concatenate**: Multiple actions for same event all execute

### Example

```yaml
agentic_workflow:
  when:                                   # L2 defaults
    after_step_fails:
      log:
        to_file_path: "./workspace/logs/global-errors.log"
        event_fields: [step_name, error_type]

  steps:
    critical_step:
      generative_entity: "${models.primary}"
      prompt: "Critical operation"
      when:                               # L3 overrides + merges
        after_step_fails:                  # OVERRIDE: replaces L2 default for this step
          - log:
              to_file_path: "./workspace/logs/critical-errors.log"
              event_fields: [step_name, error_type, timestamp, critical_details]
              level: critical
          - notify:
              message: "Critical step failed: {{step_name}}"
          - fail:
              propagate: true

        before_step_starts:                # ADDITIVE: not in L2, added for this step
          log:
            to_file_path: "./workspace/logs/critical-start.log"
            event_fields: [step_name, timestamp]

    normal_step:
      generative_entity: "${models.primary}"
      prompt: "Normal operation"
      # NO when: key → inherits L2 defaults fully
```

**Result**:
- `critical_step`: Uses L3 `after_step_fails` (override), `before_step_starts` (additive)
- `normal_step`: Inherits L2 `after_step_fails` default

### Hook Execution Order

1. **L2 workflow defaults execute first** (if not overridden)
2. **L3 step hooks execute second** (including overrides)
3. **Within an event, actions execute in order** (array order)

---

## Hook Error Handling

### Hook Failure Policies

When a hook action fails:

1. **Non-critical actions (`log`, `notify`, `append_to`, `set_variable`)**:
   - Log hook failure internally
   - Continue executing remaining hooks
   - Continue workflow execution

2. **Critical actions (`fail`, `skip`, `run_script`, `webhook`)**:
   - Hook failure propagates to workflow
   - Execution stops at failing hook
   - Log hook failure with stack trace

3. **Conditional actions (`gwt`)**:
   - If `given` clause evaluates to error → treat as `false`
   - Log evaluation error
   - Continue to next `given-when-then` clause

### Example

```yaml
after_step_fails:
  - log:                                  # Non-critical, continues if fails
      to_file_path: "./workspace/logs/errors.log"
      event_fields: [step_name, error]
  - fail:                                  # Critical, workflow stops if this fails
      message: "Step failed"
      propagate: true
```

**If `log` fails**:
- Internal log: "Hook action 'log' failed for step X"
- Continue to `fail` action
- If `fail` succeeds → workflow fails

**If `fail` fails**:
- Internal log: "Hook action 'fail' failed for step X"
- Workflow stops
- Return hook error to user

---

## Template Variables

### Available Variables by Scope

#### Workflow Scope
- `{{workflow_id}}`: Workflow identifier string
- `{{workflow_name}}`: Workflow human-readable name
- `{{run.number}}`: Sequential execution run number
- `{{now}}`: Current ISO 8601 timestamp

#### Model Scope
- `{{model_id}}`: Model identifier
- `{{model_name}}`: Model human-readable name
- `{{model_path}}`: Path to model file
- `{{provider_id}}`: Provider identifier (lmstudio, ollama, etc.)

#### Provider Scope
- `{{provider_id}}`: Provider identifier
- `{{provider_host}}`: Provider host address
- `{{provider_port}}`: Provider port number
- `{{connection_latency_ms}}`: Connection latency

#### Step Scope
- `{{step_name}}`: Step identifier
- `{{step_type}}`: Step type (generative, tool, control_flow, sub_workflow, loop)
- `{{step_status}}`: Step status (started, running, succeeded, failed, aborted, retrying)
- `{{step.duration_ms}}`: Step execution duration
- `{{step.output}}`: Step output (.raw_text, .response, .metadata)
- `{{step.inputs.*}}`: Step input values

#### Error Scope
- `{{error.type}}`: Error type/code
- `{{error.message}}`: Error message
- `{{error.is_retryable}}`: Boolean retryable flag
- `{{error.timestamp}}`: Error timestamp
- `{{error.stack_trace}}`: Stack trace (if available)

#### Retry Scope
- `{{attempt_number}}`: Current retry attempt (1-indexed)
- `{{total_attempts}}`: Total attempts including retries
- `{{retry_delay_ms}}`: Current retry delay
- `{{retry_reason}}`: Reason for retry

#### Loop Scope
- `{{loop.iteration}}`: Current loop iteration (0-indexed)
- `{{loop.max_iterations}}`: Maximum loop iterations
- `{{loop.iteration_variable}}`: Current iteration variable value

#### Checkpoint Scope
- `{{checkpoint_id}}`: Checkpoint identifier
- `{{checkpoint_level}}`: Checkpoint level (full, minimal, metadata-only)
- `{{checkpoint_timestamp}}`: Checkpoint creation timestamp
- `{{steps_restored}}`: Number of steps restored from checkpoint

#### Sub-Workflow Scope
- `{{sub_workflow_id}}`: Sub-workflow identifier
- `{{sub_workflow_name}}`: Sub-workflow name
- `{{sub_workflow_output}}`: Sub-workflow output

#### Tool Scope
- `{{tool_name}}`: Tool name
- `{{tool_arguments}}`: Tool arguments
- `{{tool_result}}`: Tool result
- `{{tool_execution_duration_ms}}`: Tool execution duration

#### File Operation Scope
- `{{file_path}}`: File path
- `{{operation}}`: Operation type (read, write, delete)
- `{{file_size_bytes}}`: File size in bytes
- `{{backup_path}}`: Backup file path (if created)

#### Generative Scope
- `{{token_text}}`: Current token text
- `{{token_position}}`: Token position (0-indexed)
- `{{tokens_so_far}}`: Tokens generated so far
- `{{max_turns}}`: Maximum conversation turns
- `{{actual_turns}}`: Actual conversation turns

### Variable Interpolation Syntax

```yaml
# Structural references (resolved at parse time)
${models.primary-analyzer}        # Model definition reference
${workspace.output}                # Workspace path reference

# Dynamic template values (resolved at runtime)
{{step.analyze_code.output}}       # Step output reference
{{inputs.quality_threshold}}        # Workflow input reference
{{loop.current_file}}              # Loop variable reference
{{now}}                           # Current timestamp
```

### Nested Variable Access

```yaml
after_step_succeeds:
  log:
    to_file_path: "./workspace/logs/{{step_name}}.log"
    event_fields: [
      step_name,
      step.output.quality_score,    # Nested access
      step.output.issues.count,      # Deep nested access
      timestamp
    ]
```

---

## Migration Notes

### Replaced Configurations

#### `skip_on_load_failure`

**Before**:
```yaml
models:
  primary:
    name: "Primary"
    skip_on_load_failure: true
```

**After**:
```yaml
models:
  primary:
    name: "Primary"
    when:
      on_load_failure:
        skip:
          reason: "Model unavailable"
          log_level: warning
```

**Benefits**:
- Granular control (log level, reason, side effects)
- Consistent with other hook events
- Can add additional actions (notify, emit_event)

---

#### Ad-Hoc Error Handling

**Before** (scattered, inconsistent):
```yaml
# No unified error handling
# Each step defines its own error logic
# No workflow-level defaults
# No retry coordination
```

**After** (unified, declarative):
```yaml
agentic_workflow:
  when:
    after_step_fails:              # Workflow-level defaults
      log:
        to_file_path: "./workspace/logs/errors.log"
        event_fields: [step_name, error_type, error_message]

  steps:
    critical_step:
      when:
        after_step_fails:          # Step override
          - log:
              to_file_path: "./workspace/logs/critical-errors.log"
              event_fields: [step_name, error_type, critical_details]
              level: critical
          - notify:
              message: "Critical step failed"
          - fail:
              propagate: true
```

**Benefits**:
- Consistent error handling across workflow
- Workflow defaults + step overrides
- Multiple actions per event
- Conditional routing via gwt

---

#### Manual Logging/Notification

**Before** (imperative):
```yaml
# No declarative logging
# Manual logging in step prompts
# Inconsistent log formats
# No centralized notification
```

**After** (declarative):
```yaml
steps:
  analyze_code:
    when:
      before_step_starts:
        log:
          to_file_path: "./workspace/logs/steps.log"
          event_fields: [step_name, timestamp]
      after_step_succeeds:
        - log:
            to_file_path: "./workspace/logs/completed.log"
            event_fields: [step_name, duration_ms, output_size]
        - notify:
            message: "Analysis completed in {{duration_ms}}ms"
```

**Benefits**:
- Declarative logging configuration
- Consistent log formats
- Centralized notification handling
- No imperative code in prompts

---

#### Lifecycle Management

**Before** (manual):
```yaml
# No explicit lifecycle hooks
# Manual checkpoint calls in steps
# No pre/post step hooks
# No workflow lifecycle events
```

**After** (declarative):
```yaml
agentic_workflow:
  when:
    before_workflow_starts:
      - log:
          to_file_path: "./workspace/logs/workflow.log"
          event_fields: [workflow_id, start_timestamp, run_number]
      - checkpoint:
          level: minimal
          note: "Workflow start"
    after_workflow_completes:
      - log:
          to_file_path: "./workspace/logs/workflow.log"
          event_fields: [workflow_id, end_timestamp, duration_ms, status]
      - append_to: "./workspace/output/workflow_summary.yaml"
```

**Benefits**:
- Explicit lifecycle management
- Declarative checkpoint triggers
- Workflow-level hooks apply to all steps
- Consistent start/end behavior

---

### Migration Checklist

- [ ] Replace `skip_on_load_failure: true` with `on_load_failure: { skip: { reason: "..." } }`
- [ ] Add `when:` blocks to model definitions for lifecycle hooks
- [ ] Add `when:` blocks to provider definitions for connection hooks
- [ ] Add `when:` blocks to `agentic_workflow` for workflow-level hooks
- [ ] Review existing step `when:` hooks, add new events (`on_retry`, `on_timeout`, `on_validation_fail`)
- [ ] Add step-type-specific hooks (`on_token_stream`, `on_stdout`, `on_sub_workflow_start`, etc.)
- [ ] Consolidate ad-hoc error handling into unified hook system
- [ ] Move manual logging/notification to declarative hook actions
- [ ] Use workflow-level hooks for default behavior
- [ ] Use step-level hooks for overrides
- [ ] Leverage `gwt` for conditional routing instead of manual branching
- [ ] Test hook precedence (L2 defaults + L3 overrides)

---

## Examples

### Complete Workflow with All Hook Types

```yaml
workflow_id: hooks-demo
name: "Hooks Semantics Demo"

models:
  "primary":
    name: "Primary Model"
    host:
      type: lmstudio
    when:
      on_load:
        log:
          to_file_path: "./workspace/logs/models.log"
          event_fields: [model_id, load_timestamp]
      on_load_failure:
        skip:
          reason: "Using fallback model"

providers:
  lmstudio:
    config:
      host: localhost
      port: 1234
    when:
      on_connect:
        notify:
          message: "Connected to LM Studio"
      on_health_check_fail:
        log:
          to_file_path: "./workspace/logs/health.log"
          event_fields: [provider_id, failure_reason]

agentic_workflow:
  when:
    before_workflow_starts:
      - log:
          to_file_path: "./workspace/logs/workflow.log"
          event_fields: [workflow_id, start_timestamp]
      - checkpoint:
          level: minimal

    after_workflow_completes:
      - log:
          to_file_path: "./workspace/logs/workflow.log"
          event_fields: [workflow_id, end_timestamp, duration_ms]
      - notify:
          message: "Workflow completed"

  steps:
    analyze:
      generative_entity: "${models.primary}"
      prompt: "Analyze code"
      when:
        before_step_starts:
          log:
            to_file_path: "./workspace/logs/steps.log"
            event_fields: [step_name, timestamp]

        after_step_succeeds:
          - append_to: "./workspace/output/analysis.yaml"
          - log:
              to_file_path: "./workspace/logs/completed.log"
              event_fields: [step_name, duration_ms]

        after_step_fails:
          - log:
              to_file_path: "./workspace/logs/errors.log"
              event_fields: [step_name, error_type]
              level: error
          - gwt:
              - given: "error.is_retryable == true"
                then: { route_to: analyze }
              - given: "error.is_retryable == false"
                then: { fail: "Non-retryable error" }

        on_retry:
          notify:
            message: "Retrying {{step_name}} (attempt {{attempt_number}})"

        on_token_stream:
          log:
            to_file_path: "./workspace/logs/tokens.log"
            event_fields: [step_name, token_text, token_position]

    test:
      tool: shell_exec
      input:
        command: "cargo test"
        timeout_seconds: 120
      when:
        on_exit_code:
          gwt:
            - given: "exit_code == 0"
              then: { route_to: report }
            - given: "exit_code > 0"
              then: { fail: "Tests failed" }

    report:
      generative_entity: "${models.primary}"
      prompt: "Generate report"
      when:
        after_step_succeeds:
          - append_to: "./workspace/output/report.md"
          - notify:
              message: "Report generated"
```

---

## Appendix: Hook Event Matrix

| Substructure | Event | Timing | Available Actions |
|--------------|-------|--------|-------------------|
| **Models** | `on_load` | After model loaded | log, notify, set_variable, emit_event |
| | `on_unload` | After model unloaded | log, notify, emit_event |
| | `on_load_failure` | On load error | skip, fail, log, notify, retry |
| | `on_timeout` | On inference timeout | retry, fail, log, notify |
| | `on_context_overflow` | On context window exceeded | fail, log, emit_event |
| | `on_rate_limit` | On rate limit hit | gwt, log, notify, throttle |
| **Providers** | `on_connect` | On connection established | log, notify, set_variable |
| | `on_disconnect` | On connection lost | log, notify, emit_event |
| | `on_health_check_fail` | On health check failure | log, gwt, notify |
| | `on_rate_limit` | On rate limit hit | log, notify, run_script, gwt |
| | `on_retry_exhausted` | On all retries exhausted | fail, log, emit_event |
| | `on_provider_failover` | On provider switch | log, notify |
| **Workflows** | `before_workflow_starts` | Pre-workflow | log, checkpoint, set_variable, notify |
| | `after_workflow_completes` | Post-workflow | log, notify, append_to |
| | `after_workflow_fails` | On workflow failure | log, checkpoint, emit_event, fail |
| | `after_workflow_aborts` | On workflow abort | log, skip_remaining |
| | `on_checkpoint` | On checkpoint created | log |
| | `on_checkpoint_restore` | On checkpoint restored | log, notify |
| | `on_step_complete` | On any step complete | log, emit_event |
| | `on_step_failure` | On any step failure | log, emit_event |
| **Steps (base)** | `before_step_starts` | Pre-step | log, checkpoint, set_variable |
| | `during_step_streaming` | Mid-step (generative) | log, emit_event |
| | `after_step_succeeds` | Post-step success | append_to, log, checkpoint, notify |
| | `after_step_fails` | On step failure | log, gwt, fail, retry, skip |
| | `after_step_aborts` | On step abort | log, skip_remaining |
| | `on_retry` | On retry attempt | log, notify, retry |
| | `on_timeout` | On step timeout | retry, fail, log, notify |
| | `on_checkpoint` | On checkpoint saved | log |
| | `on_validation_fail` | On validation failure | log, gwt, fail, retry |
| | `after_all_retries_exhausted` | On retry limit | log, fail, emit_event |
| **Generative steps** | `on_token_stream` | Per token | log, emit_event |
| | `on_tool_call` | On tool invocation | log, emit_event |
| | `on_tool_result` | On tool result | log, emit_event |
| | `on_max_turns_reached` | On turn limit | log, gwt, fail |
| **Script steps** | `on_stdout` | Per stdout line | log |
| | `on_stderr` | Per stderr line | log |
| | `on_exit_code` | On process exit | gwt, fail, log |
| **Sub-workflow steps** | `on_sub_workflow_start` | On sub-workflow start | log, set_variable |
| | `on_sub_workflow_complete` | On sub-workflow complete | append_to, log, emit_event |
| | `on_sub_workflow_fail` | On sub-workflow failure | log, gwt, fail |
| **File operation steps** | `on_file_read` | On file read | log |
| | `on_file_write` | On file write | log |
| | `on_file_error` | On file operation error | log, gwt, fail, retry |

---

## See Also
- [Unified Workflow Schema](../schema/unified-workflow-schema.yml) - Single source of truth
- [Schema Integration Plan](./hooks-integration-plan.md) - Integration plan for hooks
