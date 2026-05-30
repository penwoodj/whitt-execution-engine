# Hook & Action Full Implementation Plan

## Objective
Implement ALL lifecycle hooks and actions from `docs/schema/unified-workflow-schema.yml` with unit + integration tests (NO live system tests). Create test fixture YAML files that prove the YAML drives engine behavior.

## Scope Decisions (from user Q&A)
- **GWT**: Full expression language with AND/OR/NOT, arithmetic, comparisons, dot-path access
- **Hook integration**: Each hook type provides its own context variables
- **Notify**: Sub-workflow completion notification only (no external services)
- **Bookmark**: File checkpoint that later steps can reference
- **Skip**: Return HookResult enum (Continue/SkipStep/SkipLoop/SkipRemaining/Fail)
- **Test YAML**: Separate files in `tests/fixtures/hooks/`

---

## Architecture

### New Files
```
src/workflow/
  hooks/
    mod.rs              — HookExecutor, HookResult, HookEngine
    context.rs          — Per-trigger HookContext structs
    gwt.rs              — GWT expression lexer, parser, evaluator
    actions.rs          — Action executor (all 10 action types)
    bookmark.rs         — Bookmark file checkpoint system
    notify.rs           — Sub-workflow notification system
tests/fixtures/hooks/
  test-all-triggers.yml — Tests all 13 trigger types
  test-all-actions.yml  — Tests all 10 action types
  test-gwt-routing.yml  — Tests GWT conditional routing
  test-skip-actions.yml — Tests skip/fail control flow
  test-bookmark.yml     — Tests bookmark save/restore
  test-notify.yml       — Tests sub-workflow notify
  test-negative.yml     — Tests error/edge cases
  test-integration.yml  — Full workflow with all features
```

### Modified Files
```
src/workflow/mod.rs     — Add hooks module
src/workflow/step.rs    — Add SkipLoop to HookAction enum
src/benchmark/runner.rs — Use HookEngine instead of inline execute_hook()
src/agent/loop_hooks.rs — Delegate to HookEngine
```

---

## PHASE 1: GWT Expression Language

### File: `src/workflow/hooks/gwt.rs`

**Lexer tokens**:
- Number (integer, float)
- String (double-quoted)
- Boolean (true/false)
- Identifier (field paths with dot notation)
- Operators: ==, !=, >=, <=, >, <, &&, ||, !, +, -, *, /
- Parens: (, )
- Null

**Parser** → AST:
```
Expr := OrExpr
OrExpr := AndExpr (|| AndExpr)*
AndExpr := NotExpr (&& NotExpr)*
NotExpr := !NotExpr | CompExpr
CompExpr := AddExpr (CompOp AddExpr)?
CompOp := == | != | >= | <= | > | <
AddExpr := MulExpr ((+|-) MulExpr)*
MulExpr := Unary ((*|/) Unary)*
Unary := -Unary | Primary
Primary := Number | String | Bool | Null | IdentPath | (Expr)
IdentPath := identifier (. identifier)*
```

**Evaluator**: Takes AST + `serde_json::Value` context → `bool`

**Unit tests** (~40 tests, all GWT-described):
- Given expression "quality_score >= 0.9" and context {"quality_score": 0.95}, When evaluated, Then returns true
- Given expression "error.is_retryable == true && error.count < 3", ...
- Given expression "!false || (x > 0 && y <= 10)", ...
- Negative: type mismatch, missing field, division by zero, syntax error

---

## PHASE 2: Per-Trigger HookContext

### File: `src/workflow/hooks/context.rs`

Each trigger type gets a context struct with the fields it provides:

| Trigger | Context Fields Available |
|---------|------------------------|
| `before_step_starts` | step_name, step_type, model_name, prompt_preview, workflow_variables |
| `during_step_streaming` | step_name, chunk_text, tokens_so_far, elapsed_ms |
| `after_step_succeeds` | step_name, output, duration_ms, quality_score, token_count |
| `after_step_fails` | step_name, error_type, error_message, error.is_retryable, attempt_number |
| `after_all_retries_exhausted` | step_name, total_attempts, last_error |
| `after_step_starts` | step_name, step_type |
| `before_gwt_evaluates` | step_name, input_quality_score |
| `after_gwt_evaluates` | step_name, decision, quality_score, route_target |
| `on_requires_failed` | failed_step, reason, dependency_chain |
| `after_loop_iteration_fails` | step_name, iteration, error_message, loop_type |

All contexts implement `to_json_value()` → `serde_json::Value` for GWT evaluation.

**Unit tests** (~30 tests):
- Given before_step_starts context with step_name="analyze", When get_field("step_name"), Then returns "analyze"
- Given after_step_fails context with retryable error, When get_field("error.is_retryable"), Then returns "true"
- Negative: missing field returns None, wrong type fields handled

---

## PHASE 3: HookResult Enum

### File: `src/workflow/hooks/mod.rs`

```rust
pub enum HookResult {
    Continue,
    SkipStep,
    SkipLoop,
    SkipRemaining,
    Fail { reason: String },
    RouteTo { targets: Vec<String> },
}
```

Actions return HookResult. Multiple actions in a trigger: last non-Continue wins.

**Unit tests** (~10 tests):
- Given empty actions, When execute, Then returns Continue
- Given [Log, SkipRemaining], When execute, Then returns SkipRemaining
- Given [SkipStep, Fail], When execute, Then returns Fail (last wins)

---

## PHASE 4: Action Implementations

### File: `src/workflow/hooks/actions.rs`

All 10 actions + SkipLoop:

| Action | Implementation |
|--------|---------------|
| `log` | Write formatted line to file with template interpolation |
| `save_to` | Save output to file/variable (overwrite) |
| `append_to` | Append output to file/variable |
| `fail` | Return HookResult::Fail with reason |
| `skip_step` | Return HookResult::SkipStep |
| `skip_loop` | Return HookResult::SkipLoop |
| `skip_remaining` | Return HookResult::SkipRemaining |
| `bookmark` | Write checkpoint file + store in engine state |
| `notify` | Signal sub-workflow completion via channel |
| `gwt` | Evaluate clauses, return HookResult::RouteTo for first match |
| `route_to` | Return HookResult::RouteTo with target step(s) |

**Unit tests** (~60 tests, positive + negative for each):
- Given log action with to_file_path, When execute, Then file contains formatted entry
- Given save_to with path, When execute, Then file has output content
- Given fail action, When execute, Then returns Fail result
- Given gwt with matching clause, When evaluate, Then returns RouteTo
- Given gwt with no matching clause, When evaluate, Then returns Continue
- Negative: log to invalid path, save_to permission denied, gwt syntax error

---

## PHASE 5: HookEngine — Wire Triggers to Actions

### File: `src/workflow/hooks/mod.rs`

```rust
pub struct HookEngine {
    bookmarks: HashMap<String, serde_json::Value>,
    notify_tx: Option<tokio::sync::mpsc::Sender<NotifyMessage>>,
}
```

Methods:
- `execute_trigger(trigger_name, context, actions) -> HookResult`
- `fire_before_step_starts(...)`
- `fire_during_step_streaming(chunk_handler)`
- `fire_after_step_succeeds(...)`
- `fire_after_step_fails(...)`
- `fire_after_all_retries_exhausted(...)`
- `fire_after_step_starts(...)`
- `fire_before_gwt_evaluates(...)`
- `fire_after_gwt_evaluates(...)`
- `fire_on_requires_failed(...)`
- `fire_after_loop_iteration_fails(...)`

Each method:
1. Builds the per-trigger HookContext
2. Calls `execute_trigger()` which iterates actions
3. Returns HookResult for caller to act on

**Unit tests** (~30 tests):
- Given step with before_step_starts log hook, When fire, Then log file written
- Given step with after_step_succeeds save_to, When fire, Then file saved
- Given step with gwt routing, When fire_after_step_fails, Then returns RouteTo

---

## PHASE 6: Test Fixture YAML Files

### Directory: `tests/fixtures/hooks/`

8 YAML files, all schema-compliant per unified-workflow-schema.yml v2.0:

1. **test-all-triggers.yml** — One step per trigger type, each with a log action
2. **test-all-actions.yml** — Steps using all 10 action types
3. **test-gwt-routing.yml** — Steps with GWT clauses routing to different steps
4. **test-skip-actions.yml** — Steps that trigger skip_step, skip_remaining, fail
5. **test-bookmark.yml** — Step that bookmarks, later step restores
6. **test-notify.yml** — Sub-workflow step with notify on completion
7. **test-negative.yml** — Invalid scenarios for error handling
8. **test-integration.yml** — Full workflow combining all features

---

## PHASE 7-10: Tests (GWT-described)

### Unit Tests (in src/workflow/hooks/ modules)
- ~40 GWT expression evaluator tests
- ~30 HookContext field access tests  
- ~10 HookResult precedence tests
- ~60 Action execution tests (positive + negative)
- ~30 HookEngine trigger tests

### Integration Tests (new test files)
- `tests/hook_integration_test.rs` — Load fixture YAMLs, parse hooks, execute via mock engine
- `tests/hook_gwt_test.rs` — GWT expression integration with workflow context
- `tests/hook_fixture_test.rs` — Each fixture YAML loaded and validated

All tests use Given/When/Then in descriptions:
```rust
#[test]
fn gwt_given_quality_score_above_threshold_when_evaluated_then_routes_to_success() { ... }
```

---

## Execution Order

Phases 1-3 can run in parallel (GWT engine, contexts, result enum).
Phase 4 depends on 1-3.
Phase 5 depends on 4.
Phase 6 can start in parallel with 4-5.
Phases 7-10 depend on 5+6.

---

## Estimated Size
- GWT engine: ~800 lines
- Contexts: ~300 lines
- Actions: ~400 lines
- HookEngine: ~300 lines
- Bookmark: ~150 lines
- Notify: ~100 lines
- Tests: ~1500 lines
- Fixtures: ~500 lines YAML
- **Total: ~4050 lines**

---

## Checkpoint Questions

After Phase 3, verify:
- [ ] GWT evaluator handles all expression types
- [ ] Context structs provide correct fields per trigger
- [ ] HookResult enum covers all control flow needs

After Phase 5, verify:
- [ ] All 13 triggers fire correctly
- [ ] All 10 actions execute correctly
- [ ] HookResult propagated to caller

After Phase 10, verify:
- [ ] cargo test passes all new + existing tests
- [ ] cargo clippy clean
- [ ] Fixture YAMLs parse correctly
- [ ] Integration tests prove YAML drives behavior
