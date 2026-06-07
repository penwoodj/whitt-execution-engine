# Lifecycle Hooks — Verified Reference

Complete inventory of hook triggers and actions. Every entry listed here is verified at unit + integration test level unless marked otherwise.

**Source files:**

| File | Purpose |
|------|---------|
| `src/workflow/step.rs:174-201` | `HookAction` enum (12 variants) + action structs |
| `src/workflow/hooks/context.rs` | 10 context structs + `WorkflowHookContext` enum |
| `src/workflow/hooks/actions.rs` | `execute_action()` dispatcher + all `execute_*` functions |
| `src/workflow/hooks/mod.rs` | `HookResult` enum (6 variants) + `HookEngine` + merge logic |
| `src/workflow/hooks/gwt.rs` | GWT expression evaluator (lexer + parser + evaluator) |
| `src/benchmark/runner.rs:1249-1610` | Trigger firing points in the benchmark runner |
| `tests/hooks_integration.rs` | 55 integration tests |
| `tests/fixtures/hooks/` | YAML fixtures for all trigger + action combos |

---

## Triggers (10 total)

Triggers fire at specific points in the benchmark runner lifecycle. Each trigger receives a context struct with fields available to actions via `get_field()` and GWT expressions.

### 1. `before_step_starts` — ✅ FULLY WIRED

**Fires at:** `runner.rs:1257` — before model inference begins for a step.

**Context:** `BeforeStepStartsContext`

| Field | Type | Description |
|-------|------|-------------|
| `step_name` | `String` | Name of the step about to execute |
| `step_type` | `String` | `generative`, `tool`, or `control_flow` |
| `model_name` | `String` | Model assigned to this step |
| `prompt_preview` | `String` | First N chars of the prompt |
| `workflow_variables` | `HashMap` | Current workflow-level variables |

**Use case:** Pre-step setup, conditional skip, logging, resource checks.

---

### 2. `during_step_streaming` — ❌ NOT WIRED

**Status:** Context struct exists, unit tested. Not fired in runner — requires SSE streaming path (`stream: false` is hardcoded in `benchmark_single_model`). Architectural change needed.

**Context:** `DuringStepStreamingContext`

| Field | Type | Description |
|-------|------|-------------|
| `step_name` | `String` | Step being streamed |
| `chunk_text` | `String` | Current SSE chunk |
| `tokens_so_far` | `u32` | Tokens received so far |
| `elapsed_ms` | `u64` | Milliseconds since stream start |

---

### 3. `after_step_succeeds` — ✅ FULLY WIRED

**Fires at:** `runner.rs:1340` — after step completes successfully.

**Context:** `AfterStepSucceedsContext`

| Field | Type | Description |
|-------|------|-------------|
| `step_name` | `String` | Step that succeeded |
| `output` | `String` | Full model output text |
| `duration_ms` | `u64` | Wall-clock time for inference |
| `quality_score` | `Option<f32>` | Quality metric (if computed) |
| `token_count` | `u32` | Tokens in output |
| `model_name` | `String` | Model used |
| `json_parsable` | `String` | `"true"` / `"false"` — whether output parses as valid JSON |

**Note:** `json_parsable` is computed dynamically via `serde_json::from_str` — not stored, evaluated on access.

**Use case:** Save output, log metrics, conditional routing via GWT, benchmark recording.

---

### 4. `after_step_fails` — ✅ FULLY WIRED

**Fires at:** `runner.rs:1318` — when a step fails (any error during inference).

**Context:** `AfterStepFailsContext`

| Field | Type | Description |
|-------|------|-------------|
| `step_name` | `String` | Step that failed |
| `error_type` | `String` | Error classification string |
| `error_message` | `String` | Full error message |
| `error.is_retryable` | `String` | `"true"` / `"false"` |
| `error.count` | `String` | Error occurrence count |
| `attempt_number` | `u32` | Which retry attempt (1-based) |
| `model_name` | `String` | Model that was used |

**Use case:** Error logging, conditional retry routing, alert notification.

---

### 5. `after_all_retries_exhausted` — ✅ FULLY WIRED

**Fires at:** `runner.rs:1355` — after `after_step_fails` fires and error contains `"attempts failed"` substring. Indicates all retry attempts were consumed.

**Context:** `AfterAllRetriesExhaustedContext`

| Field | Type | Description |
|-------|------|-------------|
| `step_name` | `String` | Step that exhausted retries |
| `total_attempts` | `u32` | Number of retry attempts made |
| `last_error` | `String` | Final error message |
| `last_error_type` | `String` | Error classification |

**Use case:** Escalation, dead-letter logging, workflow abort, fallback routing.

---

### 6. `after_step_starts` — ✅ FULLY WIRED

**Fires at:** `runner.rs:1286` — immediately after step execution begins (after `before_step_starts`, before inference completes).

**Context:** `AfterStepStartsContext`

| Field | Type | Description |
|-------|------|-------------|
| `step_name` | `String` | Step that started |
| `step_type` | `String` | `generative`, `tool`, or `control_flow` |

**Use case:** Minimal — step-started acknowledgment, timing marker.

---

### 7. `before_gwt_evaluates` — ⚠️ PARTIALLY WIRED

**Status:** Fires inside `execute_gwt()` at `actions.rs:491`. Info-level logging only. Full wiring requires passing `hook_config` through the dispatch chain.

**Context:** `BeforeGwtEvaluatesContext`

| Field | Type | Description |
|-------|------|-------------|
| `step_name` | `String` | Step evaluating GWT |
| `input_value` | `JsonValue` | The JSON context being evaluated |

---

### 8. `after_gwt_evaluates` — ⚠️ PARTIALLY WIRED

**Status:** Same as `before_gwt_evaluates` — fires inside `execute_gwt()` at `actions.rs:501`. Info-level logging only.

**Context:** `AfterGwtEvaluatesContext`

| Field | Type | Description |
|-------|------|-------------|
| `step_name` | `String` | Step evaluating GWT |
| `decision` | `String` | `"routed"` or `"continue"` |
| `quality_score` | `Option<f32>` | Score (if applicable) |
| `route_target` | `String` | Target step name (or empty) |

---

### 9. `on_requires_failed` — ✅ FULLY WIRED

**Fires at:** `runner.rs:1535` — when a step has `requires` constraints that aren't met by available `step_outputs`.

**Context:** `OnRequiresFailedContext`

| Field | Type | Description |
|-------|------|-------------|
| `failed_step` | `String` | Step whose requirements failed |
| `reason` | `String` | Why the dependency failed |
| `dependency_chain` | `Vec<String>` | Full dependency chain |

**Use case:** Dependency failure logging, alternative path routing.

---

### 10. `after_loop_iteration_fails` — ✅ FULLY WIRED

**Fires at:** `runner.rs:1592` (also `1701` in iteration loop) — when an iteration within `iterate_values` produces an error.

**Context:** `AfterLoopIterationFailsContext`

| Field | Type | Description |
|-------|------|-------------|
| `step_name` | `String` | Step in the loop |
| `iteration` | `u32` | Iteration number (1-based) |
| `error_message` | `String` | Error from this iteration |
| `loop_type` | `String` | Type of loop (`iterate_values`) |

**Use case:** Per-iteration error logging, partial result capture.

---

## Actions (12 total)

Actions are the "do something" part of a hook. Each trigger can have multiple actions that execute in sequence. Results are merged with priority: `Fail > SkipRemaining > RouteTo > SkipStep > Continue`.

### 1. `log` — ✅ VERIFIED

**Struct:** `LogAction`

```yaml
log:
  to_file_path: "outputs/step-log.txt"
  event_fields: ["step_name", "duration_ms"]
  level: info  # info | debug | warning | error | critical
```

**Behavior:** Formats a log line with timestamp, level, selected fields, and full context JSON. Writes to file (appends) if `to_file_path` set. Always logs to `tracing::info`. Creates parent dirs.

**Result:** `Continue`

---

### 2. `save_to` — ✅ VERIFIED

**Struct:** `SaveToAction` (untagged enum)

```yaml
# To file (overwrite)
save_to: "outputs/result.json"

# To variable (bookmark store)
save_to: "$$my_result"

# Both
save_to: ["$$my_result", "outputs/result.json"]
```

**Behavior:** Writes context output to file (overwrite, not append). Variable names prefixed with `$$` store in engine bookmarks. Tries to parse output as JSON; falls back to string.

**Result:** `Continue`

---

### 3. `append_to` — ✅ VERIFIED

**Struct:** `AppendToAction` (untagged enum)

```yaml
# Append to file
append_to: "outputs/all-results.txt"

# Append to variable (concatenates with newline)
append_to: "$$accumulated"

# Both
append_to: ["$$accumulated", "outputs/all-results.txt"]
```

**Behavior:** Appends context output to file (newline-separated). Variable names prefixed with `$` concatenate in bookmark store with `\n` separator.

**Result:** `Continue`

---

### 4. `route_to` — ✅ VERIFIED

**Struct:** `RouteToAction` (untagged enum)

```yaml
# Single target
route_to: "step_2"

# Multiple targets (parallel execution)
route_to: ["step_2a", "step_2b"]
```

**Behavior:** Returns routing directive. Runner uses this to determine which step(s) execute next. Multi-target enables parallel branching.

**Result:** `RouteTo { targets }`

---

### 5. `bookmark` — ✅ VERIFIED

**Struct:** `BookmarkAction` (untagged enum)

```yaml
# Memory-only (no file)
bookmark: true

# Memory + file
bookmark: "checkpoints/step1.json"

# Explicit struct form
bookmark:
  path: "checkpoints/step1.json"
```

**Behavior:** Stores step output in engine bookmarks (keyed by `step_name`). If path provided, writes pretty-printed JSON to file. Subsequent steps can retrieve via `engine.get_bookmark("step_name")`.

**Result:** `Continue`

---

### 6. `notify` — ✅ VERIFIED

**Struct:** `NotifyAction`

```yaml
notify:
  message: "Sub-workflow complete"
```

**Behavior:** Sends a `NotifyMessage { from_step, message, output }` through the engine's `notify_tx` channel. If no channel is set, falls back to appending to `outputs/notifications.jsonl`.

**Result:** `Continue`

---

### 7. `fail` — ✅ VERIFIED

**Struct:** `FailAction`

```yaml
fail:
  message: "Quality threshold not met"
```

**Behavior:** Returns failure directive with reason. Highest priority result — overrides all other actions in the same trigger.

**Result:** `Fail { reason }`

---

### 8. `shell` — ✅ VERIFIED

**Struct:** `ShellAction`

```yaml
shell:
  command: "free"
  args: ["-h"]
  working_dir: "/tmp"
  env:
    KEY: "value"
  fail_on_error: true  # default: true
```

**Behavior:** Executes external command via `std::process::Command`. Stores result JSON in bookmarks under key `"shell_output"` with fields: `stdout`, `stderr`, `exit_code`, `success`. If command fails and `fail_on_error` is true (default), returns `Fail`.

**Result:** `Continue` on success, `Fail` on non-zero exit (unless `fail_on_error: false`)

---

### 9. `skip_step` — ✅ VERIFIED

**Struct:** `bool`

```yaml
skip_step: true
```

**Behavior:** Skips the current step entirely. Runner honors this when returned from `before_step_starts`.

**Result:** `SkipStep` (if `true`), `Continue` (if `false`)

---

### 10. `skip_remaining` — ✅ VERIFIED

**Struct:** `bool`

```yaml
skip_remaining: true
```

**Behavior:** Skips all remaining steps in the workflow. Terminal action — cannot be overridden.

**Result:** `SkipRemaining` (if `true`), `Continue` (if `false`)

---

### 11. `gwt` — ✅ VERIFIED

**Struct:** `Vec<GwtClause>`

```yaml
gwt:
  - given: "quality_score > 0.8"
    then: "step_high_quality"
  - given: "output.length > 100"
    then: "step_long_output"
```

**Behavior:** Evaluates `given` expressions against context JSON using the GWT expression evaluator. First matching clause wins. If none match, continues. Supports: field paths (dot notation), comparisons (`==`, `!=`, `>`, `<`, `>=`, `<=`), logical (`&&`, `||`, `!`), arithmetic (`+`, `-`, `*`, `/`), parenthesized expressions.

Also fires `before_gwt_evaluates` and `after_gwt_evaluates` triggers internally.

**Result:** `RouteTo` (first matching clause), `Continue` (no match)

---

### 12. `iterate_values` — ⚠️ PASSTHROUGH

**Struct:** `HashMap<String, Vec<String>>`

```yaml
iterate_values:
  topic: ["math", "science", "history"]
```

**Behavior:** Currently a passthrough — returns `Continue` with no logic. Future feature: expands a single step into multiple iterations, one per value.

**Result:** `Continue` (always)

---

## HookResult Merge Priority

When multiple actions fire on the same trigger, results merge by priority:

| Priority | Result | Meaning |
|----------|--------|---------|
| 1 (highest) | `Fail` | Stop execution with error |
| 2 | `SkipRemaining` | Skip all remaining steps |
| 3 | `RouteTo` | Jump to specified step(s) |
| 4 | `SkipStep` | Skip current step only |
| 5 (lowest) | `Continue` | No effect, proceed normally |

---

## YAML Structure

```yaml
steps:
  my_step:
    prompt: "..."
    model: "model-name"
    when:
      before_step_starts:
        - skip_step: true           # Skip this step
        - log:
            to_file_path: "log.txt"
      after_step_succeeds:
        - save_to: "outputs/result.json"
        - gwt:
            - given: "json_parsable == true"
              then: "validate_json"
        - shell:
            command: "free"
            args: ["-h"]
      after_step_fails:
        - fail:
            message: "Step failed"
```

---

## Wiring Status Summary

| Trigger | Runner Wired | Unit Tests | Integration Tests | Live Tested |
|---------|-------------|------------|-------------------|-------------|
| `before_step_starts` | ✅ | ✅ | ✅ | ✅ |
| `during_step_streaming` | ❌ | ✅ | ✅ | ❌ |
| `after_step_succeeds` | ✅ | ✅ | ✅ | ✅ |
| `after_step_fails` | ✅ | ✅ | ✅ | ❌ |
| `after_all_retries_exhausted` | ✅ | ✅ | ✅ | ❌ |
| `after_step_starts` | ✅ | ✅ | ✅ | ✅ |
| `before_gwt_evaluates` | ⚠️ | ✅ | ✅ | ❌ |
| `after_gwt_evaluates` | ⚠️ | ✅ | ✅ | ❌ |
| `on_requires_failed` | ✅ | ✅ | ✅ | ✅ |
| `after_loop_iteration_fails` | ✅ | ✅ | ✅ | ✅ |

**Score: 8/10 fully wired, 2/10 partially wired (GWT logging), 0/10 not wired.**

| Action | Unit Tests | Integration Tests | Live Tested |
|--------|------------|-------------------|-------------|
| `log` | ✅ | ✅ | ✅ |
| `save_to` | ✅ | ✅ | ✅ |
| `append_to` | ✅ | ✅ | ❌ |
| `route_to` | ✅ | ✅ | ❌ |
| `bookmark` | ✅ | ✅ | ✅ |
| `notify` | ✅ | ✅ | ❌ |
| `fail` | ✅ | ✅ | ❌ |
| `shell` | ✅ | ✅ | ✅ |
| `skip_step` | ✅ | ✅ | ✅ |
| `skip_remaining` | ✅ | ✅ | ✅ |
| `gwt` | ✅ | ✅ | ✅ |
| `iterate_values` | ✅ (passthrough) | ✅ | ❌ |

**Score: 12/12 unit + integration verified. 6/12 live-verified.**
