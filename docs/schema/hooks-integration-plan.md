# 🔗 Hooks Integration Plan

**Created**: 2026-04-15
**Status**: PLANNING
**Companion**: `hooks-semantics.md`

---

## 📋 Purpose

Plan for integrating the hooks system defined in `hooks-semantics.md` into the unified schema. Phased approach — hooks added incrementally with each implementation phase.

---

## 📊 Current State

- Steps have `when:` hooks (6 lifecycle events: before_step_starts, during_step_streaming, after_step_succeeds, after_step_fails, after_step_aborts, after_step_completes)
- No hooks on models, providers, or workflows
- `skip_on_load_failure` removed — needs hook replacement

---

## 📝 Phase 1: Schema Additions (Documentation)

### Step 1.1: Add model-level hooks to schema

```yaml
models:
  hooks:
    on_load:
      - log: { to_file_path: "./logs/model-load.log" }
    on_load_failure:
      - skip: {}  # replaces skip_on_load_failure
    on_unload:
      - log: { to_file_path: "./logs/model-load.log" }
    on_timeout:
      - notify: { message: "Model {{model_id}} timed out" }
```

### Step 1.2: Add provider-level hooks to schema

```yaml
providers:
  hooks:
    on_connect:
      - log: { event_fields: [provider_id, timestamp] }
    on_disconnect:
      - log: { event_fields: [provider_id, reason] }
    on_health_check_fail:
      - notify: { message: "Provider {{provider_id}} health check failed" }
    on_rate_limit:
      - log: { event_fields: [provider_id, retry_after] }
```

### Step 1.3: Add workflow-level hooks to schema

```yaml
agent_workflows:
  hooks:
    before_workflow_starts:
      - log: { to_file_path: "./logs/workflow.log" }
    after_workflow_completes:
      - append_to: "./workspace/output/results.yaml"
    after_workflow_fails:
      - notify: { message: "Workflow {{workflow_name}} failed" }
    on_checkpoint:
      - checkpoint: { level: compressed_summary }
```

### Step 1.4: Expand step hooks

Add to existing `when:` structure:
- `on_retry` — step being retried
- `on_timeout` — step timed out
- `on_validation_fail` — output failed validation
- `on_tool_call` — tool invoked (generative steps)
- `on_tool_result` — tool returned (generative steps)

---

## 🦀 Phase 2: Rust Implementation

### Step 2.1: Define HookAction enum

```rust
enum HookAction {
    Log { to_file_path: String, event_fields: Vec<String> },
    Notify { message: String },
    AppendTo { path: String },
    RunScript { command: String },
    SetVariable { key: String, value: String },
    Skip,
    Fail { message: String },
    Retry { max_attempts: u32 },
    Checkpoint { level: String },
    Webhook { url: String, method: String },
    EmitEvent { event_type: String },
}
```

### Step 2.2: Define HookTrigger enum

```rust
enum HookTrigger {
    // Model
    OnLoad, OnUnload, OnLoadFailure, OnTimeout, OnContextOverflow, OnRateLimit,
    // Provider
    OnConnect, OnDisconnect, OnHealthCheckFail, OnProviderFailover,
    // Workflow
    BeforeWorkflowStarts, AfterWorkflowCompletes, AfterWorkflowFails, OnCheckpoint,
    // Step
    BeforeStepStarts, DuringStepStreaming, AfterStepSucceeds, AfterStepFails,
    AfterStepAborts, OnRetry, OnTimeout, OnValidationFail,
    // Generative step
    OnToolCall, OnToolResult, OnMaxTurnsReached,
}
```

### Step 2.3: Implement HookExecutor

```rust
struct HookExecutor {
    hooks: HashMap<HookTrigger, Vec<HookAction>>,
    template_vars: HashMap<String, String>,
}
```

### Step 2.4: Hook merging logic

Workflow defaults merged with step overrides. Additive for same trigger — both run.

---

## 🧪 Phase 3: Integration Tests

- Test each hook trigger fires at correct lifecycle point
- Test hook action execution (log file created, variable set, etc.)
- Test hook merging (workflow + step hooks both fire)
- Test hook error handling (hook failure doesn't crash workflow)
- Test template variable substitution

---

## 📝 Phase 4: Downstream Doc Updates

After schema changes:
- [ ] Update `docs/requirements/configuration-defaults.md` with hook defaults
- [ ] Update `docs/requirements/constraints-and-assumptions.md` with hook constraints
- [ ] Update `docs/plans/traceability/schema-to-phase-matrix.md` with hook fields
- [ ] Update `docs/roadmap/` with hook integration milestone
- [ ] Add hook examples to `docs/workflows/examples/` in `17-hooks-lifecycle/`

---

## Files Changed

| File | Change |
|------|--------|
| `docs/schema/unified-workflow-schema.yml` | Add hooks to models, providers, workflows; expand step hooks |
| `docs/schema/hooks-semantics.md` | Reference document (already created) |
| `docs/schema/hooks-integration-plan.md` | This file |
| `src/schema/types.rs` | HookAction, HookTrigger enums |
| `src/hooks/executor.rs` | HookExecutor implementation |
| `src/hooks/mod.rs` | Module definition |
| `docs/requirements/configuration-defaults.md` | Hook defaults |
| `docs/plans/traceability/schema-to-phase-matrix.md` | Hook field mapping |

---

## 🔗 Related Documentation

| Document | Description |
|----------|-------------|
| [README.md](./README.md) | Schema documentation |
| [hooks-semantics.md](./hooks-semantics.md) - Hook semantics and execution order |
| [unified-workflow-schema.yml](./unified-workflow-schema.yml) - Complete schema definition |
| [../guides/](../guides/) | Development guides |
| [../workflows/examples/](../workflows/examples/) - Workflow examples |
