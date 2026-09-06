# Parallel Execution Implementation Plan

## Objective
Enable parallel model inference when driven from YAML workflows, with resource guards to prevent system crashes.

## Current State
- Runner is purely sequential: `current_index += 1` in step loop
- `route_to` with multiple targets only uses `targets[0]`
- No concurrency primitives (zero tokio::spawn in .rs files)
- Model loading unloads ALL others before loading new one
- Schema has `max_concurrent_models`, `max_concurrent_requests`, `gpu_allocation` but none enforced

## Architecture Constraint
- Single Docker llama.cpp server on port 8080 (Vulkan backend)
- One model loaded at a time (server limitation)
- Multiple concurrent inference requests to SAME model → supported via llama.cpp slot system
- Steps using DIFFERENT models → must be sequential (load/unload between)

## Implementation Steps

### CHUNK 2: Handle ALL route_to targets (not just first)
**File**: `src/benchmark/runner.rs`
**Location**: Lines 1707-1736, 1777-1786, 1813-1819, 1852-1859

**Current**: 
```rust
if let Some(ref targets) = step_result.route_to {
    if let Some(&target_idx) = step_index.get(&targets[0]) {
        current_index = target_idx;
        continue;
    }
}
```

**New**: 
When route_to has multiple targets:
1. Resolve all target steps
2. Group by model (generative_entity)
3. Same-model targets → execute concurrently
4. Different-model targets → execute sequentially (load/unload between)
5. After all targets complete → continue from last sequential step

Implementation: Replace single `current_index = target_idx` with `execute_route_targets()` method.

### CHUNK 3: Add concurrent step execution
**New method**: `execute_parallel_steps()`

```rust
async fn execute_parallel_steps(
    &mut self,
    steps: Vec<&WorkflowStep>,
    client: &LlamaHttpClient,
    model: &(String, String),
    max_tokens: usize,
    temperature: f64,
    top_p: f64,
    inference_semaphore: Arc<Semaphore>,
) -> Vec<(String, WorkflowStepResult)>
```

- Uses `tokio::JoinSet` for concurrent execution
- Acquires semaphore permit before each inference
- Collects all results into HashMap<String, WorkflowStepResult>
- Returns (step_id, result) pairs

### CHUNK 4: Add inference semaphore
**File**: `src/benchmark/runner.rs`

Add `Arc<Semaphore>` initialized from `max_concurrent_requests` config (default: 2).
Wrap `chat_completion()` calls with semaphore acquire/release.

### CHUNK 5: VRAM-aware model loading
**File**: `src/benchmark/runner.rs`, `run_model_inference()`

Change the unconditional "unload all others" (line 2102-2107) to:
1. Check if target model is already loaded → skip unload
2. If not loaded, check if VRAM allows loading alongside existing
3. If not enough VRAM, unload LRU model
4. Track loaded models and their estimated VRAM

### CHUNK 6: Test YAML workflows
**New files**:
- `docs/benchmarks/workflows/test-parallel-same-model.yml` — 3 steps with same model, route_to triggers parallel
- `docs/benchmarks/workflows/test-parallel-resource-guard.yml` — tests resource limits
- `docs/benchmarks/workflows/test-parallel-negative.yml` — overload protection

### CHUNK 7-8: Live system testing
Run each YAML against live Docker server, verify:
- Parallel steps execute concurrently (wall time < sum of sequential times)
- Resource guards prevent crashes
- Negative tests fail gracefully (not crash)

## Key Design Decisions

1. **Same-model parallel**: Steps sharing the same `generative_entity` execute concurrently
2. **Different-model sequential**: Steps needing different models still load/unload sequentially
3. **Semaphore limit**: Default 2 concurrent inferences (configurable via `max_concurrent_requests`)
4. **Graceful degradation**: If parallel fails, fall back to sequential execution
5. **Output ordering**: Results stored by step_id regardless of completion order

## Dependencies to add
- `tokio::sync::Semaphore` (already in tokio, just need to use it)
- `tokio::task::JoinSet` (already in tokio)
- No new crate dependencies

## Risk Assessment
- **Low risk**: Same-model concurrent inference (llama.cpp handles this natively)
- **Medium risk**: VRAM tracking (estimates may be inaccurate)
- **High risk**: Multi-model concurrent loading (server may not support it)
- **Mitigation**: All parallel execution behind feature flag / config toggle
