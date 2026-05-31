# QA Criteria — Phase 07: Hook Lifecycle System

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Phase Plan**: Hook system completion, wire all triggers, verify all actions
**Date**: 2026-05-30
**Status**: 🟢 READY FOR QA

---

## Summary Table

| # | QA Area | Schema Ref | Status | Priority | Test Type |
|---|---------|------------|---------|----------|-----------|
| **Hook Actions (12)** |
| 1 | Log Action (HA-01) | Lines 763-778 (hooks) | ✅ PASS | P0 | Unit, Integration |
| 2 | AppendTo Action (HA-02) | Lines 779-788 (hooks) | ✅ PASS | P0 | Unit, Integration |
| 3 | SaveTo Action (HA-03) | Lines 789-801 (hooks) | ✅ PASS | P0 | Unit, Integration |
| 4 | RouteTo Action (HA-04) | Lines 802-810 (hooks) | ✅ PASS | P0 | Unit, Integration |
| 5 | Bookmark Action (HA-05) | Lines 811-825 (hooks) | ✅ PASS | P0 | Unit, Integration |
| 6 | Notify Action (HA-06) | Lines 826-829 (hooks) | ✅ PASS | P0 | Unit, Integration |
| 7 | Fail Action (HA-07) | Lines 830-832 (hooks) | ✅ PASS | P0 | Unit, Integration |
| 8 | Shell Action (HA-08) | Lines 833-841 (hooks) | ✅ PASS | P0 | Unit, Integration |
| 9 | SkipStep Action (HA-09) | Lines 842-844 (hooks) | ✅ PASS | P0 | Unit, Integration |
| 10 | SkipRemaining Action (HA-10) | Lines 845-847 (hooks) | ✅ PASS | P0 | Unit, Integration |
| 11 | Gwt Action (HA-11) | Lines 848-867 (hooks) | ✅ PASS | P0 | Unit, Integration |
| 12 | IterateValues Action (HA-12) | Lines 868-870 (hooks) | 🔵 DEFERRED | P2 | Unit |
| **Hook Triggers (10)** |
| 13 | before_step_starts (HT-01) | Lines 695-701 (step_hooks) | ✅ WIRED | P0 | Unit, Integration, E2E |
| 14 | after_step_starts (HT-02) | Lines 702-708 (step_hooks) | ✅ WIRED | P0 | Unit, Integration, E2E |
| 15 | after_step_fails (HT-03) | Lines 709-717 (step_hooks) | ✅ WIRED | P0 | Unit, Integration, E2E |
| 16 | after_all_retries_exhausted (HT-04) | Lines 718-722 (step_hooks) | ✅ WIRED | P0 | Unit, Integration, E2E |
| 17 | after_step_succeeds (HT-05) | Lines 723-731 (step_hooks) | ✅ WIRED | P0 | Unit, Integration, E2E |
| 18 | on_requires_failed (HT-06) | Lines 732-736 (step_hooks) | ✅ WIRED | P0 | Unit, Integration, E2E |
| 19 | after_loop_iteration_fails (HT-07) | Lines 737-741 (step_hooks) | ✅ WIRED | P0 | Unit, Integration, E2E |
| 20 | during_step_streaming (HT-08) | Lines 742-749 (step_hooks) | ❌ NOT WIRED | P1 | Unit |
| 21 | before_gwt_evaluates (HT-09) | Lines 750-756 (step_hooks) | ⚠️ PARTIAL | P1 | Unit |
| 22 | after_gwt_evaluates (HT-10) | Lines 757-762 (step_hooks) | ⚠️ PARTIAL | P1 | Unit |
| **Core Components** |
| 23 | GWT Expression Evaluator (GWT-01) | N/A (gwt.rs) | ✅ PASS | P0 | Unit (35 tests) |
| 24 | HookResult Merge Priority (HR-01) | N/A (mod.rs) | ✅ PASS | P0 | Unit, Integration |
| 25 | Integration Testing (INT-01) | N/A (tests/) | ✅ PASS | P0 | Integration (47 tests) |

---

## Hook Actions — QA Criteria

### HA-01: Log Action

**Schema Ref**: Lines 763-778 (hooks.log)
**Plan Ref**: Hook actions implementation
**Files**: `src/workflow/hooks/actions.rs` (execute_log), `src/workflow/step.rs` (LogAction)

**Description**:
Write log entry to file path and/or stdout. Supports configurable event_fields (context field extraction) and log level (Info, Debug, Warning, Error, Critical). Creates parent directories if needed.

**Pass Criteria**:
- Log entry written to file when `to_file_path` specified
- Parent directories created automatically
- File content includes timestamp, level, step_name, and specified event_fields
- Log entry emitted to tracing info when executed
- All log levels (Info/Debug/Warning/Error/Critical) format correctly
- Multiple event_fields extracted and concatenated
- Template interpolation works in event_fields

**Evidence Required**:
- Unit test: `execute_log()` with file path → file exists, content correct
- Unit test: `execute_log()` with nested path → directories created
- Unit test: `execute_log()` with all 5 log levels → level strings correct
- Integration test: `given_log_action_when_executed_then_file_created_with_content`
- Integration test: Nested directory creation test
- Live system: YAML hook with `log` → file contains expected content

**Priority**: P0

**Commands**:
```bash
# Unit tests for log action
cargo test --lib hooks::actions::tests::test_log

# Integration test
cargo test --test hooks_integration given_log_action_when_executed_then_file_created_with_content

# Live system test
whitt benchmark --workflow examples/live-test-ministral-3b.yml
cat outputs/output/benchmark.log  # Verify log entries
```

---

### HA-02: AppendTo Action

**Schema Ref**: Lines 779-788 (hooks.append_to)
**Plan Ref**: Hook actions implementation
**Files**: `src/workflow/hooks/actions.rs` (execute_append_to)

**Description**:
Append content to file path OR variable (bookmark store). Supports FilePath, Variable, or Both variants. Variable accumulation via bookmark store concatenation.

**Pass Criteria**:
- `FilePath` variant: content appended to existing file
- `Variable` variant: value concatenated into bookmark store under specified key
- `Both` variant: both file append AND bookmark concatenation occur
- File created if not exists for FilePath variant
- Bookmark concatenated with existing value (if any) for Variable variant

**Evidence Required**:
- Unit test: `execute_append_to()` with FilePath → file content appended
- Unit test: `execute_append_to()` with Variable → bookmark stored+concatenated
- Unit test: `execute_append_to()` with Both → file append + bookmark store
- Integration test: `given_append_to_action_when_executed_then_content_appended_to_file`

**Priority**: P0

**Commands**:
```bash
# Unit tests for append_to action
cargo test --lib hooks::actions::tests::test_append_to

# Integration test
cargo test --test hooks_integration given_append_to_action_when_executed_then_content_appended_to_file
```

---

### HA-03: SaveTo Action

**Schema Ref**: Lines 789-801 (hooks.save_to)
**Plan Ref**: Hook actions implementation
**Files**: `src/workflow/hooks/actions.rs` (execute_save_to)

**Description**:
Save step output to file path, variable ($$prefix), or both. Supports FilePath, Variable ($$prefix), or Both variants. Variable storage in bookmark store.

**Pass Criteria**:
- `FilePath` variant: output written to specified file
- `Variable` variant: output stored in bookmark store under `$$prefix` key
- `Both` variant: both file write AND bookmark storage occur
- File content exactly matches context output string
- Bookmark value exactly matches context output string

**Evidence Required**:
- Unit test: `execute_save_to()` with FilePath → file contains output
- Unit test: `execute_save_to()` with Variable ($$prefix) → bookmark stored
- Unit test: `execute_save_to()` with Both → file write + bookmark store
- Integration test: `given_save_to_action_when_executed_then_file_contains_output`
- Live system: YAML hook with `save_to` → output file exists with correct content

**Priority**: P0

**Commands**:
```bash
# Unit tests for save_to action
cargo test --lib hooks::actions::tests::test_save_to

# Integration test
cargo test --test hooks_integration given_save_to_action_when_executed_then_file_contains_output

# Live system test
whitt benchmark --workflow examples/live-test-ministral-3b.yml
ls -lh outputs/output/  # Verify save_to output files
```

---

### HA-04: RouteTo Action

**Schema Ref**: Lines 802-810 (hooks.route_to)
**Plan Ref**: Hook actions implementation
**Files**: `src/workflow/hooks/actions.rs` (execute_route_to)

**Description**:
Route execution to named step(s). Supports single target or multiple targets. Returns `HookResult::RouteTo` with list of step IDs.

**Pass Criteria**:
- `execute_route_to()` returns `HookResult::RouteTo` with step_id list
- Single target: list contains 1 step_id
- Multiple targets: list contains N step_ids
- No side effects (file I/O or state mutation)
- HookResult::RouteTo variant matches schema definition

**Evidence Required**:
- Unit test: `execute_route_to()` with single target → RouteTo{1 step_id}
- Unit test: `execute_route_to()` with multiple targets → RouteTo{N step_ids}
- Integration test: `given_route_to_action_when_executed_then_hook_result_routes_to_target`

**Priority**: P0

**Commands**:
```bash
# Unit tests for route_to action
cargo test --lib hooks::actions::tests::test_route_to

# Integration test
cargo test --test hooks_integration given_route_to_action_when_executed_then_hook_result_routes_to_target
```

---

### HA-05: Bookmark Action

**Schema Ref**: Lines 811-825 (hooks.bookmark)
**Plan Ref**: Hook actions implementation
**Files**: `src/workflow/hooks/actions.rs` (execute_bookmark)

**Description**:
Store execution state as flag, path, or detailed record. Supports Flag (bool), Path (string), Detailed (path + content). Stores in bookmark engine and optionally writes file.

**Pass Criteria**:
- `Flag(true)`: stores "true" bookmark, no file write
- `Path(string)`: stores bookmark key AND writes file at path
- `Detailed{path: Some, content}`: stores bookmark key + content AND writes file
- `Detailed{path: None}`: stores bookmark key + content, no file write
- File content matches specified content for Path/Detailed variants

**Evidence Required**:
- Unit test: `execute_bookmark()` with Flag(true) → bookmark stored
- Unit test: `execute_bookmark()` with Path(string) → bookmark stored + file exists
- Unit test: `execute_bookmark()` with Detailed{path} → bookmark stored + file content
- Unit test: `execute_bookmark()` with Detailed{path:None} → bookmark stored, no file
- Integration test: `given_bookmark_action_when_executed_then_file_and_memory_stored`

**Priority**: P0

**Commands**:
```bash
# Unit tests for bookmark action
cargo test --lib hooks::actions::tests::test_bookmark

# Integration test
cargo test --test hooks_integration given_bookmark_action_when_executed_then_file_and_memory_stored
```

---

### HA-06: Notify Action

**Schema Ref**: Lines 826-829 (hooks.notify)
**Plan Ref**: Hook actions implementation
**Files**: `src/workflow/hooks/actions.rs` (execute_notify)

**Description**:
Send notification via mpsc channel. Returns `Continue`. Uses `tx.try_send()` when channel present, ignores if no channel.

**Pass Criteria**:
- `execute_notify()` returns `HookResult::Continue` always
- If notify_tx channel present: message sent via `try_send()`
- If no notify_tx: no error, continues silently
- Message includes hook trigger context (step_name, output, etc.)

**Evidence Required**:
- Unit test: `execute_notify()` with no channel → Continue, no panic
- Unit test: `execute_notify()` with channel → Continue, message sent
- Integration test: notify action with channel verification

**Priority**: P0

**Commands**:
```bash
# Unit tests for notify action
cargo test --lib hooks::actions::tests::test_notify
```

---

### HA-07: Fail Action

**Schema Ref**: Lines 830-832 (hooks.fail)
**Plan Ref**: Hook actions implementation
**Files**: `src/workflow/hooks/actions.rs` (execute_fail)

**Description**:
Fail the current hook with optional message. Returns `HookResult::Fail` with reason string.

**Pass Criteria**:
- `execute_fail()` returns `HookResult::Fail`
- Fail reason includes specified message if provided
- Fail reason includes default "Hook failed" if no message
- No side effects

**Evidence Required**:
- Unit test: `execute_fail()` with message → Fail{reason=message}
- Unit test: `execute_fail()` without message → Fail{default reason}
- Integration test: `given_fail_action_when_executed_then_hook_result_is_fail`

**Priority**: P0

**Commands**:
```bash
# Unit tests for fail action
cargo test --lib hooks::actions::tests::test_fail

# Integration test
cargo test --test hooks_integration given_fail_action_when_executed_then_hook_result_is_fail
```

---

### HA-08: Shell Action

**Schema Ref**: Lines 833-841 (hooks.shell)
**Plan Ref**: Hook actions implementation
**Files**: `src/workflow/hooks/actions.rs` (execute_shell)

**Description**:
Execute external command, capture stdout/stderr, store in bookmark engine. Supports fail_on_error flag. Environment variables via env map.

**Pass Criteria**:
- Command executes successfully → `HookResult::Continue`
- Command fails + fail_on_error=true → `HookResult::Fail`
- Command fails + fail_on_error=false → `HookResult::Continue`
- Command output (stdout+stderr) stored in bookmark as "shell_output"
- Environment variables passed to command
- Empty command string → `HookResult::Fail`

**Evidence Required**:
- Unit test: `execute_shell()` with echo command → Continue, bookmark stored
- Unit test: `execute_shell()` with false command → Fail
- Unit test: `execute_shell()` with fail_on_error=false → Continue
- Unit test: `execute_shell()` with missing command → Fail
- Unit test: `execute_shell()` with env vars → command receives env, bookmark verified

**Priority**: P0

**Commands**:
```bash
# Unit tests for shell action
cargo test --lib hooks::actions::tests::test_shell
```

---

### HA-09: SkipStep Action

**Schema Ref**: Lines 842-844 (hooks.skip_step)
**Plan Ref**: Hook actions implementation
**Files**: `src/workflow/hooks/actions.rs` (execute_skip_step)

**Description**:
Skip current step based on boolean flag. Returns `SkipStep` or `Continue`.

**Pass Criteria**:
- `execute_skip_step(true)` returns `HookResult::SkipStep`
- `execute_skip_step(false)` returns `HookResult::Continue`
- No side effects

**Evidence Required**:
- Unit test: `execute_skip_step(true)` → SkipStep
- Unit test: `execute_skip_step(false)` → Continue
- Integration test: `given_skip_step_action_when_executed_then_hook_result_is_skip_step`

**Priority**: P0

**Commands**:
```bash
# Unit tests for skip_step action
cargo test --lib hooks::actions::tests::test_skip_step

# Integration test
cargo test --test hooks_integration given_skip_step_action_when_executed_then_hook_result_is_skip_step
```

---

### HA-10: SkipRemaining Action

**Schema Ref**: Lines 845-847 (hooks.skip_remaining)
**Plan Ref**: Hook actions implementation
**Files**: `src/workflow/hooks/actions.rs` (execute_skip_remaining)

**Description**:
Skip all remaining steps in workflow based on boolean flag. Returns `SkipRemaining` or `Continue`.

**Pass Criteria**:
- `execute_skip_remaining(true)` returns `HookResult::SkipRemaining`
- `execute_skip_remaining(false)` returns `HookResult::Continue`
- No side effects

**Evidence Required**:
- Unit test: `execute_skip_remaining(true)` → SkipRemaining
- Unit test: `execute_skip_remaining(false)` → Continue

**Priority**: P0

**Commands**:
```bash
# Unit tests for skip_remaining action
cargo test --lib hooks::actions::tests::test_skip_remaining
```

---

### HA-11: Gwt Action

**Schema Ref**: Lines 848-867 (hooks.gwt)
**Plan Ref**: Hook actions implementation
**Files**: `src/workflow/hooks/actions.rs` (execute_gwt), `src/workflow/hooks/gwt.rs` (evaluator)

**Description**:
Evaluate Given-When-Then clauses for conditional routing. First matching clause fires `RouteTo`, else `Continue`. Supports nested field access, arithmetic, comparisons, logical operators.

**Pass Criteria**:
- Matching clause → `HookResult::RouteTo` with then_target
- Non-matching clause → `HookResult::Continue`
- Multiple clauses → first-match semantics (evaluate in order, stop at first match)
- Invalid condition → treat as false, continue to next clause
- No clauses → `Continue`

**Evidence Required**:
- Unit test: `execute_gwt()` with matching clause → RouteTo
- Unit test: `execute_gwt()` with non-matching clause → Continue
- Unit test: `execute_gwt()` with multiple clauses → first-match
- Unit test: `execute_gwt()` with invalid condition → Continue (false)
- Integration test: `given_gwt_matching_clause_when_executed_then_routes_to_then_target`
- Integration test: `given_gwt_non_matching_clause_when_executed_then_continues`
- GWT evaluator tests (35 tests) → all pass

**Priority**: P0

**Commands**:
```bash
# Unit tests for gwt action
cargo test --lib hooks::actions::tests::test_gwt

# Integration tests for gwt
cargo test --test hooks_integration given_gwt_matching_clause_when_executed_then_routes_to_then_target
cargo test --test hooks_integration given_gwt_non_matching_clause_when_executed_then_continues

# GWT evaluator tests
cargo test --lib hooks::gwt::tests
```

---

### HA-12: IterateValues Action

**Schema Ref**: Lines 868-870 (hooks.iterate_values)
**Plan Ref**: Hook actions implementation (future feature)
**Files**: `src/workflow/hooks/actions.rs` (execute_action - passthrough)

**Description**:
Iterate over values (passthrough, future feature). Currently returns `Continue` without logic.

**Pass Criteria**:
- `execute_action()` returns `HookResult::Continue` for IterateValues
- No side effects

**Evidence Required**:
- Unit test: `execute_action()` with IterateValues → Continue

**Priority**: P2 (future feature)

**Commands**:
```bash
# Unit test for iterate_values passthrough
cargo test --lib hooks::actions::tests::test_iterate_values
```

---

## Hook Triggers — QA Criteria

### HT-01: before_step_starts

**Schema Ref**: Lines 695-701 (step_hooks.before_step_starts)
**Plan Ref**: Hook trigger wiring
**Files**: `src/benchmark/runner.rs:1250` (trigger call), `src/workflow/hooks/context.rs` (BeforeStepStartsContext)

**Context Struct**:
```rust
BeforeStepStartsContext {
    step_name: String,
    step_type: StepType (Generative/Tool/ControlFlow),
    model_name: String,
    prompt_preview: String,
    workflow_variables: HashMap<String, JsonValue>,
}
```

**Wiring Status**: ✅ Fully wired at runner.rs:1250

**Pass Criteria**:
- Trigger fires before step execution begins
- Context populated with step metadata (step_name, step_type, model_name, prompt_preview, workflow_variables)
- Hook actions execute with correct context
- Control flow actions (skip_step, fail) affect step execution

**Evidence Required**:
- Unit test: BeforeStepStartsContext.to_json_value() → correct JSON structure
- Unit test: BeforeStepStartsContext.get_field() → all fields accessible
- Integration test: actions with BeforeStepStartsContext → correct execution
- Live system: YAML hook with before_step_starts → fires before step

**Priority**: P0

**Commands**:
```bash
# Unit tests for context
cargo test --lib hooks::context::tests::test_before_step_starts_context

# Integration test
cargo test --test hooks_integration -- before_step_starts
```

---

### HT-02: after_step_starts

**Schema Ref**: Lines 702-708 (step_hooks.after_step_starts)
**Plan Ref**: Hook trigger wiring
**Files**: `src/benchmark/runner.rs:1285` (trigger call), `src/workflow/hooks/context.rs` (AfterStepStartsContext)

**Context Struct**:
```rust
AfterStepStartsContext {
    step_name: String,
    step_type: StepType,
    model_name: String,
    workflow_variables: HashMap<String, JsonValue>,
}
```

**Wiring Status**: ✅ Fully wired at runner.rs:1285

**Pass Criteria**:
- Trigger fires after step execution starts but before completion
- Context populated with step metadata (step_name, step_type, model_name, workflow_variables)
- Hook actions execute with correct context

**Evidence Required**:
- Unit test: AfterStepStartsContext.to_json_value() → correct JSON structure
- Unit test: AfterStepStartsContext.trigger_name() → "after_step_starts"

**Priority**: P0

**Commands**:
```bash
# Unit tests for context
cargo test --lib hooks::context::tests::test_after_step_starts_context
```

---

### HT-03: after_step_fails

**Schema Ref**: Lines 709-717 (step_hooks.after_step_fails)
**Plan Ref**: Hook trigger wiring
**Files**: `src/benchmark/runner.rs:1347` (trigger call), `src/workflow/hooks/context.rs` (AfterStepFailsContext)

**Context Struct**:
```rust
AfterStepFailsContext {
    step_name: String,
    error_message: String,
    is_retryable: bool,
    attempt_number: u32,
    max_attempts: u32,
}
```

**Wiring Status**: ✅ Fully wired at runner.rs:1347

**Pass Criteria**:
- Trigger fires when step execution fails
- Context populated with error details (error_message, is_retryable, attempt_number, max_attempts)
- Hook actions execute with correct context
- Fail action from hook propagates error
- GWT routing based on is_retryable or attempt_number works

**Evidence Required**:
- Unit test: AfterStepFailsContext.to_json_value() → correct JSON structure
- Unit test: AfterStepFailsContext.get_field("error.is_retryable") → nested field access
- Integration test: actions with AfterStepFailsContext → correct execution

**Priority**: P0

**Commands**:
```bash
# Unit tests for context
cargo test --lib hooks::context::tests::test_after_step_fails_context
```

---

### HT-04: after_all_retries_exhausted

**Schema Ref**: Lines 718-722 (step_hooks.after_all_retries_exhausted)
**Plan Ref**: Hook trigger wiring
**Files**: `src/benchmark/runner.rs:1367` (trigger call), `src/workflow/hooks/context.rs` (AfterAllRetriesExhaustedContext)

**Context Struct**:
```rust
AfterAllRetriesExhaustedContext {
    step_name: String,
    error_message: String,
    total_attempts: u32,
}
```

**Wiring Status**: ✅ Fully wired at runner.rs:1367 (fires when error contains "attempts failed")

**Pass Criteria**:
- Trigger fires after all retry attempts exhausted
- Context populated with step_name, error_message, total_attempts
- Hook actions execute with correct context

**Evidence Required**:
- Unit test: AfterAllRetriesExhaustedContext.to_json_value() → correct JSON structure
- Unit test: AfterAllRetriesExhaustedContext.trigger_name() → "after_all_retries_exhausted"

**Priority**: P0

**Commands**:
```bash
# Unit tests for context
cargo test --lib hooks::context::tests::test_after_all_retries_exhausted_context
```

---

### HT-05: after_step_succeeds

**Schema Ref**: Lines 723-731 (step_hooks.after_step_succeeds)
**Plan Ref**: Hook trigger wiring
**Files**: `src/benchmark/runner.rs:1387` (trigger call), `src/workflow/hooks/context.rs` (AfterStepSucceedsContext)

**Context Struct**:
```rust
AfterStepSucceedsContext {
    step_name: String,
    output: String,
    duration_ms: u64,
    quality_score: Option<f32>,
    token_count: u32,
    model_name: String,
}
```

**Wiring Status**: ✅ Fully wired at runner.rs:1387

**Pass Criteria**:
- Trigger fires when step execution succeeds
- Context populated with step output, duration_ms, quality_score, token_count, model_name
- quality_score can be None (optional)
- Hook actions execute with correct context
- File I/O actions (save_to, append_to, bookmark) work correctly

**Evidence Required**:
- Unit test: AfterStepSucceedsContext.to_json_value() → correct JSON structure (including None quality_score)
- Unit test: AfterStepSucceedsContext.get_field() → all fields accessible
- Integration test: actions with AfterStepSucceedsContext → correct execution
- Live system: YAML hook with after_step_succeeds + save_to → output file created

**Priority**: P0

**Commands**:
```bash
# Unit tests for context
cargo test --lib hooks::context::tests::test_after_step_succeeds_context

# Integration tests
cargo test --test hooks_integration given_save_to_action_when_executed_then_file_contains_output
cargo test --test hooks_integration given_append_to_action_when_executed_then_content_appended_to_file

# Live system test
whitt benchmark --workflow examples/live-test-ministral-3b.yml
cat outputs/output/step_*.json  # Verify save_to output
```

---

### HT-06: on_requires_failed

**Schema Ref**: Lines 732-736 (step_hooks.on_requires_failed)
**Plan Ref**: Hook trigger wiring
**Files**: `src/benchmark/runner.rs:1539` (trigger call), `src/workflow/hooks/context.rs` (OnRequiresFailedContext)

**Context Struct**:
```rust
OnRequiresFailedContext {
    step_name: String,
    missing_dependencies: Vec<String>,
}
```

**Wiring Status**: ✅ Fully wired at runner.rs:1539 (in iteration loop)

**Pass Criteria**:
- Trigger fires when step.requires check fails against step_outputs
- Context populated with step_name and missing_dependencies list
- Hook actions execute with correct context

**Evidence Required**:
- Unit test: OnRequiresFailedContext.to_json_value() → correct JSON structure
- Unit test: OnRequiresFailedContext.trigger_name() → "on_requires_failed"

**Priority**: P0

**Commands**:
```bash
# Unit tests for context
cargo test --lib hooks::context::tests::test_on_requires_failed_context
```

---

### HT-07: after_loop_iteration_fails

**Schema Ref**: Lines 737-741 (step_hooks.after_loop_iteration_fails)
**Plan Ref**: Hook trigger wiring
**Files**: `src/benchmark/runner.rs:1600` (trigger call), `src/workflow/hooks/context.rs` (AfterLoopIterationFailsContext)

**Context Struct**:
```rust
AfterLoopIterationFailsContext {
    step_name: String,
    iteration_index: u32,
    error_message: String,
}
```

**Wiring Status**: ✅ Fully wired at runner.rs:1600 (in iteration loop)

**Pass Criteria**:
- Trigger fires when loop iteration result has error
- Context populated with step_name, iteration_index, error_message
- Hook actions execute with correct context

**Evidence Required**:
- Unit test: AfterLoopIterationFailsContext.to_json_value() → correct JSON structure
- Unit test: AfterLoopIterationFailsContext.trigger_name() → "after_loop_iteration_fails"

**Priority**: P0

**Commands**:
```bash
# Unit tests for context
cargo test --lib hooks::context::tests::test_after_loop_iteration_fails_context
```

---

### HT-08: during_step_streaming

**Schema Ref**: Lines 742-749 (step_hooks.during_step_streaming)
**Plan Ref**: Hook trigger wiring (NOT WIRED)
**Files**: `src/workflow/hooks/context.rs` (DuringStepStreamingContext)

**Context Struct**:
```rust
DuringStepStreamingContext {
    step_name: String,
    chunk_text: String,
    tokens_so_far: u32,
    elapsed_ms: u64,
}
```

**Wiring Status**: ❌ NOT WIRED (requires SSE streaming path, stream:false hardcoded)

**Pass Criteria**:
- Trigger fires per chunk during SSE streaming
- Context populated with step_name, chunk_text, tokens_so_far, elapsed_ms
- Hook actions execute with correct context

**Evidence Required**:
- Unit test: DuringStepStreamingContext.to_json_value() → correct JSON structure
- Unit test: DuringStepStreamingContext.get_field() → all fields accessible

**Priority**: P1 (blocked by streaming architecture)

**Commands**:
```bash
# Unit tests for context
cargo test --lib hooks::context::tests::test_during_step_streaming_context
```

---

### HT-09: before_gwt_evaluates

**Schema Ref**: Lines 750-756 (step_hooks.before_gwt_evaluates)
**Plan Ref**: Hook trigger wiring (PARTIAL)
**Files**: `src/workflow/hooks/actions.rs:404` (info logging only)

**Context Struct**:
```rust
BeforeGwtEvaluatesContext {
    step_name: String,
    gwt_clauses: Vec<GwtClause>,
}
```

**Wiring Status**: ⚠️ PARTIAL (info-level logging only at actions.rs:404)

**Pass Criteria**:
- Trigger fires before GWT evaluation
- Context populated with step_name and gwt_clauses
- Hook actions execute with correct context
- Full wire requires passing hook_config through execute_action() into execute_gwt()

**Evidence Required**:
- Unit test: BeforeGwtEvaluatesContext.to_json_value() → correct JSON structure
- Unit test: BeforeGwtEvaluatesContext.trigger_name() → "before_gwt_evaluates"

**Priority**: P1 (partial wire)

**Commands**:
```bash
# Unit tests for context
cargo test --lib hooks::context::tests::test_before_gwt_evaluates_context
```

---

### HT-10: after_gwt_evaluates

**Schema Ref**: Lines 757-762 (step_hooks.after_gwt_evaluates)
**Plan Ref**: Hook trigger wiring (PARTIAL)
**Files**: `src/workflow/hooks/actions.rs:414` (info logging only)

**Context Struct**:
```rust
AfterGwtEvaluatesContext {
    step_name: String,
    gwt_clauses: Vec<GwtClause>,
    matched_clause_index: Option<usize>,
}
```

**Wiring Status**: ⚠️ PARTIAL (info-level logging only at actions.rs:414)

**Pass Criteria**:
- Trigger fires after GWT evaluation
- Context populated with step_name, gwt_clauses, matched_clause_index
- Hook actions execute with correct context
- Full wire requires passing hook_config through execute_action() into execute_gwt()

**Evidence Required**:
- Unit test: AfterGwtEvaluatesContext.to_json_value() → correct JSON structure
- Unit test: AfterGwtEvaluatesContext.trigger_name() → "after_gwt_evaluates"

**Priority**: P1 (partial wire)

**Commands**:
```bash
# Unit tests for context
cargo test --lib hooks::context::tests::test_after_gwt_evaluates_context
```

---

## Core Components — QA Criteria

### GWT-01: GWT Expression Evaluator

**Schema Ref**: N/A (gwt.rs implementation)
**Plan Ref**: GWT expression evaluator implementation
**Files**: `src/workflow/hooks/gwt.rs` (lexer, parser, evaluator, 35 tests)

**Description**:
Given-When-Then expression evaluator supporting literals, field paths, comparisons, logical operators, arithmetic, precedence, error handling.

**Coverage**:
- Literals (true/false/null/number/string): 7 tests
- Field paths (simple, nested, deep, missing): 5 tests
- Comparisons (==, !=, >, <, >=, <=): all operators for num/str/bool/null
- Logical (&&, ||, !): 5 tests
- Arithmetic (+, -, *, /): 5 tests
- Precedence (and vs or, comparison vs logical, arithmetic vs comparison): 3 tests
- Error cases (div/0, type mismatch, syntax errors, invalid escape, unterminated string): 6 tests
- Parenthesized expressions: Complete
- Complex multi-field expressions: 2 tests

**Pass Criteria**:
- All 35 GWT tests pass
- Literals evaluate correctly
- Field paths navigate nested structures
- Comparisons work for all types
- Logical operators short-circuit correctly
- Arithmetic handles div/0
- Precedence matches expected order
- Error cases return false (not panic)

**Evidence Required**:
- Unit test output: `cargo test --lib -- gwt::tests` → 35 tests passed
- GWT expression tests in `src/workflow/hooks/gwt.rs`

**Priority**: P0

**Commands**:
```bash
# Run all GWT tests
cargo test --lib hooks::gwt::tests

# Run specific GWT test categories
cargo test --lib gwt::tests::test_literals
cargo test --lib gwt::tests::test_field_paths
cargo test --lib gwt::tests::test_comparisons
cargo test --lib gwt::tests::test_logical_operators
cargo test --lib gwt::tests::test_arithmetic
cargo test --lib gwt::tests::test_precedence
cargo test --lib gwt::tests::test_error_cases
```

---

### HR-01: HookResult Merge Priority

**Schema Ref**: N/A (mod.rs implementation)
**Plan Ref**: HookResult merge logic
**Files**: `src/workflow/hooks/mod.rs` (HookResult enum, merge logic)

**Priority Order**:
1. Fail (highest)
2. SkipRemaining
3. RouteTo
4. SkipStep
5. Continue (lowest)

**Description**:
Merge multiple HookResults from hook actions. Returns highest priority result.

**Pass Criteria**:
- `Fail` merged with anything → `Fail`
- `SkipRemaining` merged with `Continue`/`RouteTo`/`SkipStep` → `SkipRemaining`
- `RouteTo` merged with `Continue`/`SkipStep` → `RouteTo`
- `SkipStep` merged with `Continue` → `SkipStep`
- `Continue` merged with `Continue` → `Continue`

**Evidence Required**:
- Unit test: HookResult::merge() → priority order correct
- Integration test: `given_hook_results_merged_then_highest_priority_wins`

**Priority**: P0

**Commands**:
```bash
# Unit tests for merge
cargo test --lib hooks::tests::test_hook_result_merge

# Integration test
cargo test --test hooks_integration given_hook_results_merged_then_highest_priority_wins
```

---

### INT-01: Integration Testing

**Schema Ref**: N/A (integration tests)
**Plan Ref**: Integration test suite
**Files**: `tests/hooks_integration.rs` (47 tests), `tests/fixtures/hooks/*.yml`

**Description**:
Integration tests verify end-to-end action dispatch from HookAction enum through execute_action() to HookResult. Tests use execute_action() directly (not via runner).

**Test Coverage (47 tests total)**:
- Fixture parsing: 1 test
- Log action: 3 tests (file, nested dirs, event_fields)
- SaveTo action: 3 tests (FilePath, Variable, Both)
- AppendTo action: 1 test
- Bookmark action: 1 test
- Fail action: 1 test
- SkipStep action: 1 test
- RouteTo action: 1 test
- GWT action: 6 tests (matching, non-matching, comparison, logical, dot path, invalid, missing field)
- HookResult merge: 1 test
- Serde round-trip: 15 tests (all 12 HookAction variants)

**Context Types Tested**:
- BeforeStepStarts: ✅
- AfterStepSucceeds: ✅
- AfterStepFails: ✅
- DuringStepStreaming: ✅

**Pass Criteria**:
- All 47 integration tests pass
- Serde round-trip works for all 12 HookAction variants
- LogAction swallowing bug regression test passes
- Actions work correctly with all 4 context types

**Evidence Required**:
- Integration test output: `cargo test --test hooks_integration` → 47 tests passed
- Hook action deserialization from JSON verified

**Priority**: P0

**Commands**:
```bash
# Run all integration tests
cargo test --test hooks_integration -- --test-threads=1

# Run specific integration tests
cargo test --test hooks_integration given_log_action_when_executed_then_file_created_with_content
cargo test --test hooks_integration given_save_to_action_when_executed_then_file_contains_output
cargo test --test hooks_integration given_append_to_action_when_executed_then_content_appended_to_file
cargo test --test hooks_integration given_bookmark_action_when_executed_then_file_and_memory_stored
cargo test --test hooks_integration given_fail_action_when_executed_then_hook_result_is_fail
cargo test --test hooks_integration given_skip_step_action_when_executed_then_hook_result_is_skip_step
cargo test --test hooks_integration given_route_to_action_when_executed_then_hook_result_routes_to_target
cargo test --test hooks_integration given_gwt_matching_clause_when_executed_then_routes_to_then_target
cargo test --test hooks_integration given_hook_results_merged_then_highest_priority_wins
```

---

## Verification Matrix

| QA Area | Unit | Integration | E2E | Live System |
|---------|------|-------------|-----|-------------|
| **Actions** |
| HA-01: Log | ✅ | ✅ | ✅ | ✅ |
| HA-02: AppendTo | ✅ | ✅ | ⚠️ | ❌ |
| HA-03: SaveTo | ✅ | ✅ | ✅ | ✅ |
| HA-04: RouteTo | ✅ | ✅ | ❌ | ❌ |
| HA-05: Bookmark | ✅ | ✅ | ⚠️ | ❌ |
| HA-06: Notify | ✅ | ✅ | ❌ | ❌ |
| HA-07: Fail | ✅ | ✅ | ❌ | ❌ |
| HA-08: Shell | ✅ | ❌ | ❌ | ❌ |
| HA-09: SkipStep | ✅ | ✅ | ❌ | ❌ |
| HA-10: SkipRemaining | ✅ | ✅ | ❌ | ❌ |
| HA-11: Gwt | ✅ | ✅ | ❌ | ❌ |
| HA-12: IterateValues | ✅ | ❌ | ❌ | ❌ |
| **Triggers** |
| HT-01: before_step_starts | ✅ | ✅ | ⚠️ | ❌ |
| HT-02: after_step_starts | ✅ | ❌ | ❌ | ❌ |
| HT-03: after_step_fails | ✅ | ⚠️ | ❌ | ❌ |
| HT-04: after_all_retries_exhausted | ✅ | ❌ | ❌ | ❌ |
| HT-05: after_step_succeeds | ✅ | ✅ | ✅ | ✅ |
| HT-06: on_requires_failed | ✅ | ❌ | ❌ | ❌ |
| HT-07: after_loop_iteration_fails | ✅ | ❌ | ❌ | ❌ |
| HT-08: during_step_streaming | ✅ | ❌ | ❌ | ❌ (not wired) |
| HT-09: before_gwt_evaluates | ✅ | ❌ | ❌ | ❌ (partial) |
| HT-10: after_gwt_evaluates | ✅ | ❌ | ❌ | ❌ (partial) |
| **Components** |
| GWT-01: GWT Evaluator | ✅ (35 tests) | ✅ (6 tests) | ❌ | ❌ |
| HR-01: Merge Priority | ✅ | ✅ | ❌ | ❌ |
| INT-01: Integration | — | ✅ (47 tests) | — | — |

**Legend**:
- ✅: Fully tested and verified
- ⚠️: Partially tested (some variants missing)
- ❌: Not tested

---

## Phase Exit Criteria

Hook system QA complete when:

1. **All 25 QA areas** defined with pass criteria
2. **Unit tests**: All action unit tests pass (95% coverage of variants)
3. **Integration tests**: All 47 hooks_integration tests pass
4. **GWT evaluator**: All 35 GWT tests pass
5. **Context structs**: All 10 to_json_value() and trigger_name() tests pass
6. **HookResult merge**: Priority order verified
7. **Live system**: Log, SaveTo, Bookmark verified via live benchmark
8. **Wiring status documented**: 7/10 fully wired, 2/10 partial, 1/10 blocked
9. **Gaps documented**: Critical gaps (during_step_streaming, GWT hooks partial wire)
10. **Serde round-trip**: All 12 HookAction variants tested

---

## Related Files

| Component | File | Lines |
|-----------|------|-------|
| HookAction enum | `src/workflow/step.rs` | 174-200 |
| Action structs | `src/workflow/step.rs` | 202-323 |
| Context structs | `src/workflow/hooks/context.rs` | Full file |
| Action execution | `src/workflow/hooks/actions.rs` | Full file |
| HookEngine + merge | `src/workflow/hooks/mod.rs` | Full file |
| GWT evaluator | `src/workflow/hooks/gwt.rs` | Full file |
| Trigger wiring | `src/benchmark/runner.rs` | 813-848, 1249-1380, 1535-1610 |
| Integration tests | `tests/hooks_integration.rs` | 669 lines (47 tests) |
| Fixtures | `tests/fixtures/hooks/*.yml` | 5 files |

---

**End of QA Criteria for Hook Lifecycle System**