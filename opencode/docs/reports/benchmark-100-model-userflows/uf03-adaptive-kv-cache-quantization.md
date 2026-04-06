# UF03: Adaptive KV Cache Quantization

## Overview
When a model fails to load due to VRAM constraints, automatically reduces KV cache quantization level and retries, progressively degrading quality to fit available memory.

## User Story
As a benchmark operator, I want the system to automatically adjust KV cache quantization for models that don't fit in memory so that I can benchmark larger models with reduced fidelity rather than skipping them entirely.

## Pre-conditions
- Model is installed (UF02 completed)
- System has accurate VRAM reporting
- Model supports multiple KV cache quantization levels

## Trigger
Model load fails with OOM or "insufficient VRAM" error.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────────┐
│ Attempt      │────>│ OOM /        │────>│ Lower KV Cache  │
│ Default Load  │     │ Load Failed  │     │ Quantization   │
│ (Q8_0)       │     │              │     │ (Q8_0 → Q6_K)  │
└──────┬───────┘     └──────────────┘     └───────┬──────────┘
       │                                        │
       │                                        ▼
       │                              ┌──────────────────┐
       │                              │ Retry Load with  │
       │                              │ Lower Quant     │
       │                              └───────┬──────────┘
       │                                      │
       ▼                                      ▼
┌──────────────┐                    ┌──────────────┐
│ Success?     │──Yes──>              │ Log Result   │
└──────┬───────┘                    └──────────────┘
       │ No
       ▼
┌──────────────┐
│ More Levels  │
│ Available?   │───No──> Mark as "too large
└──────┬──────┘        for current hardware"
       │ Yes
       ▼
┌──────────────┐
│ Continue     │
│ Degradation  │
└──────────────┘
```

## Step-by-Step

### Step 1: Detect Load Failure
- **Action**: Catch OOM/insufficient VRAM error from provider during model load
- **Schema Properties Used**: `workflow_execution_strategy.memory.pressure_handling.on_oom`
- **Input**: Model load attempt, error message
- **Output**: Error classified as "memory_pressure" with current VRAM usage
- **Error Handling**: If error is not memory-related (e.g., corrupt file), don't attempt quantization - fail immediately
- **New Schema Requirements**: None - covered by existing memory pressure handling

### Step 2: Determine Current Quantization Level
- **Action**: Check what KV cache quantization the model was loaded with
- **Schema Properties Used**: `models.<id>.model_memory.kv_cache_quantization`
- **Input**: Model configuration
- **Output**: Current level (e.g., q8_0, q6_k, q4_k_m, q4_0, q3_k_m, q2_k)
- **Error Handling**: If level can't be determined, assume q8_0 (highest)
- **New Schema Requirements**: None

### Step 3: Calculate Next Lower Level
- **Action**: Select next lower quantization level from the fallback chain
- **Schema Properties Used**: N/A (quantization ladder logic)
- **Input**: Current level, available levels
- **Output**: Next quantization level + estimated VRAM savings
- **Error Handling**: If no lower levels available, mark model as "cannot fit"
- **New Schema Requirements**: `models.<id>.model_memory.kv_cache_quantization` needs expanded enum values. Current schema shows: `min | max | medium | medium-min | medium-max | auto` - needs to include specific quant levels: `q8_0 | q6_k | q5_k_m | q5_0 | q4_k_m | q4_0 | q3_k_m | q2_k`

### Step 4: Estimate Memory at New Level
- **Action**: Calculate expected VRAM usage at the new quantization level
- **Schema Properties Used**: `models.<id>.max_allowed.vram`
- **Input**: Current VRAM usage, quantization ratio (e.g., Q8→Q6 = ~25% savings)
- **Output**: Estimated VRAM at new level, comparison to available VRAM
- **Error Handling**: If estimate shows still insufficient, skip to Step 5 and try next level
- **New Schema Requirements**: `models.<id>.model_memory.quantization_vram_table` - lookup table for VRAM per quantization level per model size

### Step 5: Retry with Lowered Quantization
- **Action**: Configure model with new KV cache quantization and attempt load again
- **Schema Properties Used**: `model_overrides` with `kv_cache_quantization` override
- **Input**: Model ID, new quantization level
- **Output**: Load success/failure result
- **Error Handling**: On failure, loop back to Step 3 with next level. Max 3 degradation steps before marking "cannot fit."
- **New Schema Requirements**: `retry.level: step_unload` already covers this - unload model, reconfigure, retry

### Step 6: Log Degradation
- **Action**: Record that model was benchmarked at reduced quality
- **Schema Properties Used**: `logging.scopes`
- **Input**: Original level, final level, success/failure
- Output**: Degradation event in benchmark log
- **Error Handling**: N/A
- **New Schema Requirements**: `benchmark_manifest.models[].actual_quantization` vs `.requested_quantization`

## Post-conditions
- Model loaded at highest feasible quantization level
- Degradation history recorded for benchmark report accuracy
- Models that can't fit in memory are marked with reason

## Edge Cases
- Model only supports one quantization level (no fallback)
- Degraded model produces unusable output (still benchmark but flag quality)
- Multiple degradation steps needed (Q8_0 → Q4_0 → Q2_K)
- KV cache quantization and weight quantization are different things

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Memory pressure detection | Yes | None |
| Model unload/retry | Yes (`retry.level: step_unload`) | None |
| KV cache quantization enum | Partially | Needs explicit quant levels (q4_k_m, q2_k, etc.) |
| Quantization VRAM estimation | No | Per-model quantization size table |
| Degradation logging | Partially | `benchmark_manifest.models[].actual_quantization` |
| Max degradation steps | No | `models.<id>.model_memory.max_quantization_downgrades` |

## Quantization Fallback Chain (estimated VRAM savings):

| From | To | VRAM Savings | Quality Impact |
|------|----|---------------|----------------|
| q8_0 | q6_k | ~25% | Minimal |
| q6_k | q5_k_m | ~15% | Low |
| q5_k_m | q4_k_m | ~15% | Low-Medium |
| q4_k_m | q4_0 | ~10% | Medium |
| q4_0 | q3_k_m | ~20% | High |
| q3_k_m | q2_k | ~30% | Very High |

## Dependencies
- UF01 (Discovery) - needs manifest with model entries
- UF02 (Installation) - model must be installed before quantization
- UF04 (Resource Allocation) - needs accurate VRAM reporting
