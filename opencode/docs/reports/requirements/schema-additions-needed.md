# Unified Schema Additions Needed

## Overview

Based on restored requirements specifications (specs 12, 14, 15, 20, 21, 22), this document identifies schema additions needed to the unified workflow schema v2.0.

---

## Schema Additions Required

### 1. Fault Tolerance Section

**Required By:** Spec 14 (Constraints and Assumptions)

**Rationale:** Fault tolerance is critical for production-grade workflow execution. The unified schema currently lacks explicit fault tolerance configuration.

**Addition:**

```yaml
fault_tolerance:
  retry_max_attempts: number        # Maximum retry attempts per operation
  retry_backoff_secs: [number,...] # Exponential backoff sequence in seconds
  skip_failed_models: boolean        # Continue workflow if model fails vs. fail entire workflow
  checkpoint_on_error: boolean       # Save checkpoint before any error (not just on failure)
  checkpoint_interval_secs: number    # Save state at regular intervals (not just on error)
```

**Default Values (from spec 14):**
```yaml
fault_tolerance:
  retry_max_attempts: 3
  retry_backoff_secs: [5, 15, 60]  # 5s → 15s → 60s exponential
  skip_failed_models: true
  checkpoint_on_error: true
  checkpoint_interval_secs: 60  # 1 minute
```

**Schema Section Location:** Add as new top-level section in `unified-workflow-schema.yml`, adjacent to existing `concurrency:` section.

**Related Unified Schema Fields:**
- `steps.type.loop.on_consecutive_failures` - Stop after N consecutive failures
- `steps.type.loop.on_empty_iterations` - What to do when condition false/empty
- `steps.type.loop.on_max_iterations` - Action on reaching iteration limit
- `workflow.execution.retry_max_attempts` - Retry logic for operations
- `workflow.execution.retry_backoff_secs` - Backoff sequence
- `workflow.state_management.checkpoint_interval_secs` - How often to save state

---

### 2. Enhanced Concurrency Section

**Required By:** Spec 20 (Complete YAML Benchmark Examples) and Spec 15 (Configuration Defaults)

**Rationale:** The current `concurrency:` section lacks fine-grained control needed for production benchmarking and resource management.

**Addition:**

```yaml
concurrency:
  max_parallel_models: number         # NEW: Max models loaded simultaneously
  max_parallel_requests: number        # NEW: Max concurrent prompts across all models
  memory_limit_gb: number            # NEW: Total system memory budget
```

**Default Values (from spec 20):**
```yaml
concurrency:
  workflow_level: 4            # Existing: Max concurrent workflows
  model_level: 1                # NEW: Max models loaded at once
  step_level: 16               # Existing: Max parallel steps
  max_total_requests: 64         # NEW: Global request cap
  max_requests_per_model: 8        # NEW: Per-model request limit
  max_total_loaded_models: 2       # NEW: Max models in memory at once
  memory_limit_gb: 16            # NEW: 16GB total memory budget
```

**Schema Section Location:** Extend existing `concurrency:` section in `unified-workflow-schema.yml` with new fields.

**Related Unified Schema Fields:**
- `workflow.execution.timeout_secs` - Respect concurrency limits
- `workflow.memory.strategy` - Apply memory-aware model loading
- `models.*.n_batch` - Adjust batch size based on concurrency limits
- `models.*.n_ctx` - Reduce context if multiple models loaded

---

## Implementation Priority

### High Priority: Fault Tolerance

1. **Add fault_tolerance section to unified schema**
2. Update schema parser to recognize fault_tolerance fields
3. Implement retry logic in workflow engine
4. Add checkpoint hooks that fire on errors (not just interval-based)
5. Update validation to enforce fault_tolerance constraints

### High Priority: Enhanced Concurrency

1. Add 3 new fields to concurrency section
2. Implement resource-aware model loading (respect memory_limit_gb)
3. Add request throttling (max_parallel_requests, max_requests_per_model)
4. Add model unloading when approaching memory limits
5. Update documentation and examples to use new concurrency controls

---

## Verification

Once these additions are integrated into unified-schema.yml, verify:

1. **Schema validation**: All new fields validate successfully
2. **Example workflows**: All 52 example workflows validate without errors
3. **Backward compatibility**: Existing workflows without new fields still work (all optional)
4. **Documentation**: All docs updated to reference new schema fields
5. **Testing**: Unit tests for fault tolerance and concurrency logic

---

## Related Documents

- [Unified Workflow Schema](./unifying-schema/unified-workflow-schema.yml) - Target file to update
- [Advanced Agentic Features](./advanced-agentic-features.md) - Spec defining loop termination edge cases
- [Benchmark YAML Examples](./benchmark-yaml-examples.md) - Spec demonstrating concurrency needs
- [Configuration Defaults](./configuration-defaults.md) - Spec defining default values
- [Requirements Index](./index.md) - Central requirements tracking (already updated)
