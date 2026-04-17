# Unified Schema Additions Needed

## Overview

Based on restored requirements specifications (specs 12, 14, 15, 20, 21, 22), this document identifies schema additions needed for the unified workflow schema v2.0.

---

## 1. Fault Tolerance: NO NEW SECTION NEEDED

**Conclusion:** The unified schema already handles fault tolerance through existing fields. A separate `fault_tolerance:` section would duplicate and conflict with these.

### Existing Schema Coverage

| Fault Tolerance Need | Unified Schema Field | Location |
|---------------------|---------------------|----------|
| **Retry attempts** | `retry.max_attempts: 10` | `agentic_workflow.retry` (workflow-level) |
| **Retry attempts (per-step)** | `retry.step.max_attempts: 3` | `agentic_workflow.retry.step` (step defaults) |
| **Backoff strategy** | `retry.backoff: exponential` | `agentic_workflow.retry` |
| **Backoff timing** | `retry.delay_ms: 5000`, `retry.step.base_ms: 1000`, `retry.step.max_ms: 30000` | `agentic_workflow.retry` |
| **Jitter** | `retry.step.jitter: 0.2` | `agentic_workflow.retry.step` |
| **Retry escalation level** | `retry.level: workflow_restart` | `agentic_workflow.retry` (6 levels) |
| **Checkpoint after retry** | `retry.step.checkpoint_after_retry: true` | `agentic_workflow.retry.step` |
| **Provider-level retry** | `providers.<name>.requests.retry.max_retries: 3` | `providers` section |
| **Provider-level backoff** | `providers.<name>.requests.retry.backoff: exponential` | `providers` section |
| **Timeout per operation** | `workflow_execution_strategy.timeout.per_operation.*` | `workflow_execution_strategy` |
| **Loop failure handling** | `loop.validation.tolerance`, `loop.count.max_iterations` | Step-level `loop` key |
| **Hook-based error routing** | `when.after_step_fails.gwt` (given/when/then) | Step-level `when` hooks |
| **Skip remaining on fatal** | `when.after_step_starts: [fail, skip_remaining]` | Step-level `when` hooks |
| **Model execution timeout** | `models.<name>.execution.timeout.*` | `models` section |

### One Genuinely New Field

**`skip_failed_models`** — the ability to continue a benchmark workflow when a specific model fails to load, while still recording the failure.

This doesn't fit in `retry` (it's not about retrying) or `when` hooks (it's a model-level policy). Proposed addition:

```yaml
# In providers section, under hosting:
providers:
  lmstudio:
    hosting:
      skip_on_load_failure: true         # NEW: Continue workflow if this provider's model fails to load
```

This is a single field, not a whole section.

### Why a Separate Section Would Be Harmful

1. **Ambiguity**: Two places to configure retry (`fault_tolerance.retry_max_attempts` vs `retry.max_attempts`) — which takes priority?
2. **Conflict**: Different defaults in different sections would cause undefined behavior
3. **Violates schema principle**: "One way to do each thing" (see unified-workflow-schema.yml line 10)
4. **Maintenance burden**: Every change to retry logic would need updating in two places

---

## 2. Enhanced Concurrency: MOVED TO AGENT-QUEUE

**Status:** Concurrency features moved to agent-queue project. No changes needed in unified schema.

---

## Implementation Priority

### High Priority: Fix Benchmark Examples

Update `benchmark-yaml-examples.md` to use existing schema fields instead of proposed (now rejected) `fault_tolerance:` section. Replace:
- `fault_tolerance.retry_max_attempts` → `agentic_workflow.retry.max_attempts`
- `fault_tolerance.retry_backoff_secs` → `agentic_workflow.retry.backoff: exponential` + `delay_ms`
- `fault_tolerance.skip_failed_models` → `providers.*.hosting.skip_on_load_failure`
- `fault_tolerance.checkpoint_on_error` → `retry.step.checkpoint_after_retry`
- `fault_tolerance.checkpoint_interval_secs` → Use `when` hooks for periodic checkpointing

### Medium Priority: Concurrency Decision

Await user decision on whether to extend concurrency fields or keep current schema structure.

---

## Verification

After fixing benchmark examples, verify:

1. All benchmark YAML uses only existing unified schema fields
2. No references to proposed `fault_tolerance:` section remain
3. All 52 example workflows still validate
4. Documentation accurately reflects schema capabilities

---

## Related Documents

- [Unified Workflow Schema](../schema/unified-workflow-schema.yml) — Source of truth
- [Advanced Agentic Features](./advanced-agentic-features.md) — Loop termination edge cases
- [Benchmark YAML Examples](./benchmark-yaml-examples.md) — Needs fix to use existing fields
- [Configuration Defaults](./configuration-defaults.md) — Default values
- [Requirements Index](./index.md) — Central tracking
