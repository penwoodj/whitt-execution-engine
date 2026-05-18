# UF03: Adaptive KV Cache Quantization

## Overview
When a model fails to load due to insufficient GPU VRAM, automatically attempt progressively lower KV cache quantization levels (Q8_0 → Q6_K → Q5_K_M → Q4_K_M → Q4_0 → Q3_K_M → Q2_K) until the model fits, or mark it as unloadable.

## User Story
As a benchmark operator, I want the system to automatically downgrade KV cache quantization when a model doesn't fit in VRAM so that I can benchmark the widest possible set of models without manual intervention per-GPU.

## Pre-conditions
- Model installed (UF02 completed)
- Initial quantization level known from model metadata
- GPU VRAM information available
- Provider supports KV cache quantization overrides

## Trigger
Model load fails with OOM error, or VRAM pre-check estimates insufficient memory at current quantization.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Attempt      │────>│ Load        │────>│ OOM or      │
│ Load at      │     │ Attempt     │     │ Success?    │
│ Current      │     │             │     │             │
│ Quant        │     └──────────────┘     └──────┬───────┘
└──────────────┘                                │
                                      Yes ──────┤──── No
                                      │          │
                                      ▼          ▼
                               ┌──────────┐  ┌──────────────┐
                               │ Model    │  │ Downgrade    │
                               │ Ready    │  │ Quant Level  │
                               └──────────┘  └──────┬───────┘
                                                    │
                                                    ▼
                                             ┌──────────────┐
                                             │ More Levels  │──── No ──> ┌──────────┐
                                             │ Available?   │          │ Mark     │
                                             └──────┬───────┘          │ Unload-  │
                                                    │ Yes              │ able     │
                                                    ▼                  └──────────┘
                                             ┌──────────────┐
                                             │ Retry Load   │
                                             │ at Next      │
                                             │ Quant Level  │
                                             └──────────────┘
```

## Quantization Fallback Chain

| Priority | Quant Level | VRAM Savings | Quality Impact | When to Use |
|----------|-------------|-------------|----------------|-------------|
| 0 (initial) | Q8_0 | 0% (baseline) | None | Default for all models |
| 1 | Q6_K | ~15-20% | Minimal | First fallback |
| 2 | Q5_K_M | ~25-30% | Slight | Second fallback |
| 3 | Q4_K_M | ~35-40% | Moderate | Third fallback |
| 4 | Q4_0 | ~40-45% | Moderate-High | Fourth fallback |
| 5 | Q3_K_M | ~50-55% | High | Fifth fallback |
| 6 | Q2_K | ~60-65% | Very High | Last resort |

## VRAM Estimation Table (per-billion params)

| Model Size | Q8_0 | Q6_K | Q5_K_M | Q4_K_M | Q4_0 | Q3_K_M | Q2_K |
|-----------|------|------|--------|--------|------|--------|------|
| 1B | ~120 MB | ~105 MB | ~95 MB | ~85 MB | ~80 MB | ~70 MB | ~55 MB |
| 3B | ~360 MB | ~310 MB | ~280 MB | ~250 MB | ~235 MB | ~205 MB | ~160 MB |
| 7B | ~840 MB | ~720 MB | ~650 MB | ~580 MB | ~550 MB | ~480 MB | ~375 MB |
| 13B | ~1560 MB | ~1340 MB | ~1205 MB | ~1075 MB | ~1020 MB | ~890 MB | ~695 MB |
| 33B | ~3960 MB | ~3400 MB | ~3060 MB | ~2730 MB | ~2585 MB | ~2260 MB | ~1765 MB |
| 70B | ~8400 MB | ~7210 MB | ~6490 MB | ~5790 MB | ~5480 MB | ~4790 MB | ~3740 MB |

## Step-by-Step

### Step 1: Determine Initial Quantization Level
- **Action**: Read the model's current KV cache quantization from metadata or use Q8_0 as default
- **Schema Properties Used**: `models.<id>.model_memory.kv_cache_quantization`
- **Input**: Model ID, model metadata
- **Output**: Starting quantization level (e.g., q8_0)
- **Error Handling**: If no quant metadata available, default to Q8_0
- **New Schema Requirements**: `models.<id>.model_memory.kv_cache_quantization` must support explicit level values (q8_0, q6_k, q5_k_m, q4_k_m, q4_0, q3_k_m, q2_k), not just min/max/medium

### Step 2: Estimate VRAM at Current Quantization
- **Action**: Calculate estimated VRAM usage based on model params + context length + quantization level
- **Schema Properties Used**: `models.<id>.model_memory.cache_size`, `models.<id>.max_allowed.attention_tokens`
- **Input**: Model params, quantization level, context length
- **Output**: Estimated VRAM in MB
- **Error Handling**: If estimation formula unavailable, use lookup table with conservative bounds
- **New Schema Requirements**: `models.<id>.model_memory.quantization_vram_table` — per-model VRAM estimates at each quant level

### Step 3: Compare Against Available VRAM
- **Action**: Check if estimated VRAM fits within available GPU memory budget
- **Schema Properties Used**: `models.<id>.max_allowed.vram`, `workflow_execution_strategy.memory.max_allowed`
- **Input**: VRAM estimate, available VRAM
- **Output**: fits: true/false, deficit_mb if false
- **Error Handling**: If available VRAM query fails, assume worst case (use conservative 50% of total VRAM)
- **New Schema Requirements**: System resource introspection — current GPU VRAM usage query

### Step 4: Attempt Model Load
- **Action**: Send load request to provider with specified KV cache quantization
- **Schema Properties Used**: `models.<id>.model_memory.kv_cache_quantization`
- **Input**: Model ID, quantization override
- **Output**: Load success or OOM error
- **Error Handling**: On OOM, proceed to Step 5. On other errors, do NOT retry with lower quant (model may be corrupt).
- **New Schema Requirements**: None

### Step 5: Select Next Lower Quantization
- **Action**: Move to next quantization level in the fallback chain
- **Schema Properties Used**: `models.<id>.model_memory.max_quantization_downgrades` (cap on how far to downgrade)
- **Input**: Current quantization level, downgrade history
- **Output**: Next quantization level, or "exhausted" if no more levels
- **Error Handling**: If downgrade limit reached, mark model as unloadable
- **New Schema Requirements**: `models.<id>.model_memory.max_quantization_downgrades: int` (default: 3)

### Step 6: Retry Load at Lower Quantization
- **Action**: Go back to Step 4 with new quantization level
- **Schema Properties Used**: Same as Step 4
- **Input**: New quantization level
- **Output**: Load result
- **Error Handling**: Same as Step 4, plus track total downgrade count
- **New Schema Requirements**: None

### Step 7: Record Quantization Decision
- **Action**: Once model loads (or all levels exhausted), record the final quantization used
- **Schema Properties Used**: `benchmark_manifest.models[].final_quantization`, `benchmark_manifest.models[].quant_downgrades`
- **Input**: Final quantization level, number of downgrades attempted
- **Output**: Updated manifest entry
- **Error Handling**: N/A
- **New Schema Requirements**: `benchmark_manifest.models[].final_quantization`, `benchmark_manifest.models[].quant_downgrades`

## Post-conditions
- Model either loaded at best-available quantization or marked unloadable
- Quantization history recorded in manifest for benchmark report
- VRAM usage known and within budget
- No manual intervention needed for VRAM fitting

## Edge Cases
- Model is already at Q2_K and still doesn't fit (mark unloadable)
- Provider doesn't support KV cache quantization overrides (fail with provider-specific error)
- VRAM estimation is inaccurate (actual load may still OOM even after pre-check passes)
- Quantization downgrade affects benchmark fairness (record which quant level was used for each model)
- Model works at lower quant but produces garbage output (flag in benchmark results, not handled here)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| KV cache quantization setting | Partially (`kv_cache_quantization` exists but no explicit level enum) | Explicit quant level values (q8_0, q6_k, etc.) |
| Fallback chain | No | Fallback order configuration |
| VRAM estimation per quant level | No | `quantization_vram_table` per model |
| Downgrade limit | No | `max_quantization_downgrades` |
| Quantization history tracking | No | `benchmark_manifest.models[].final_quantization` |
| VRAM pre-check | Yes (`max_allowed.vram`) | System introspection API |

## Dependencies
- UF01 (Discovery) — needs manifest with model metadata
- UF02 (Installation) — model must be installed first
- UF04 (Model Loading) — quantization is applied during the load attempt
- UF06 (Orchestration) — orchestration triggers quantization fallback on OOM
