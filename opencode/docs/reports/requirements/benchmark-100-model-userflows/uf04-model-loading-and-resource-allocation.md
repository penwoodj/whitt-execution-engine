# UF04: Model Loading and Resource Allocation

## Overview
Load models into GPU/RAM respecting resource constraints, manage concurrent loading limits, and handle model lifecycle (load, warm-up, swap, unload).

## User Story
As a benchmark operator, I want the system to load models efficiently with proper resource management so that I can benchmark 100 models without running out of GPU memory or crashing.

## Pre-conditions
- Models installed (UF02 completed)
- System resource monitoring available
- Benchmark manifest exists (UF01)

## Trigger
Benchmark execution begins for a model, or model needed for workflow step.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Check       │────>│ Estimate     │────>│ Acquire     │
│ Resource    │     │ VRAM Needed  │     │ GPU Lock     │
│ Budget      │     │              │     │              │
└──────┬───────┘     └──────┬───────┘     └──────┬───────┘
       │                       │                    │
       ▼                       ▼                    ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Sufficient  │────>│ Load Model   │────>│ Warm-Up     │
│ Resources?  │     │ into Memory  │     │ (Prefill)   │
└──────┬───────┘     └──────────────┘     └──────────────┘
       │ No                  │                    │
       ▼                    ▼                    ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Wait or     │     │ Swap /      │     │ Model Ready  │
│ Defer      │     │ Fail Fast   │     │ for Benchmark│
└──────────────┘     └──────────────┘     └──────────────┘
```

## Step-by-Step

### Step 1: Estimate Resource Requirements
- **Action**: Calculate VRAM needed for model based on params and quantization
- **Schema Properties Used**: `models.<id>.max_allowed.vram`, `models.<id>.max_allowed.attention_tokens`
- **Input**: Model metadata from manifest
- **Output**: Required VRAM estimate (mb)
- **Error Handling**: If estimate fails, use conservative upper bound
- **New Schema Requirements**: None - existing max_allowed covers this

### Step 2: Check Available Resources
- **Action**: Query current GPU VRAM usage and available budget
- **Schema Properties Used**: `workflow_execution_strategy.memory.max_allowed`
- **Input**: Resource estimate from Step 1, current usage
- **Output**: Available VRAM, decision (can_load: true/false)
- **Error Handling**: If resource query fails, assume 50% VRAM available as safe default
- **New Schema Requirements**: System resource introspection mechanism

### Step 3: Acquire GPU Lock
- **Action**: Reserve GPU memory for model loading (prevent concurrent overload)
- **Schema Properties Used**: `workflow_execution_strategy.concurrency_limits.model_loading`
- **Input**: Available VRAM, concurrent loading limit
- **Output**: GPU lock acquired, or wait queued if limit reached
- **Error Handling**: If wait timeout, fail gracefully and skip model
- **New Schema Requirements**: None - concurrency_limits handles this

### Step 4: Load Model
- **Action**: Send load request to provider with model path and configuration
- **Schema Properties Used**: `models.<id>.host.type`, `models.<id>.model_memory.kv_cache_quantization`
- **Input**: Model ID, configuration overrides
- **Output**: Model loaded in memory, ready for inference
- **Error Handling**: On load failure: retry once. On timeout: fail. On corrupt model: skip.
- **New Schema Requirements**: None

### Step 5: Warm-Up (Optional)
- **Action**: Send a short warm-up prompt to prime the model (reduce first-token latency)
- **Schema Properties Used**: `model_overrides.timeout.time_to_first_response`
- **Input**: Loaded model, warm-up prompt
- Output**: Model ready with warmed cache, timing data
- **Error Handling**: Skip warm-up if disabled or if timeout risk
- **New Schema Requirements**: `model_overrides.warmup.enabled: bool`, `model_overrides.warmup.prompt: string`

### Step 6: Track Model State
- **Action**: Update in-memory model registry with load state
- **Schema Properties Used**: `workflow_execution_strategy.memory.model_lifecycle`
- **Input**: Load result, timing data
- Output`: Updated state (loaded, warming, ready, failed)
- **Error Handling**: N/A
- **New Schema Requirements**: `benchmark_execution.models[].state: loading|ready|failed|unloaded`

## Post-conditions
- Model is in GPU memory and ready for inference
- Resource locks prevent concurrent overload
- Load timing data recorded for benchmark report

## Edge Cases
- Model reports wrong VRAM requirement (over/under-estimate)
- GPU driver crashes during load
- Another process holds GPU memory (wait or fail)
- Model loads but produces garbage (flag during first inference, not here)
- Warm-up causes OOM on small models

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| VRAM estimation | Yes (`max_allowed.vram`) | Per-model VRAM calculator |
| GPU lock acquisition | Yes (`concurrency_limits.model_loading`) | None |
| Concurrent limit | Yes (`concurrency_limits.model_loading`) | None |
| Model lifecycle | Yes (`load_unload`, `cache_size`) | None |
| Warm-up | Partially | `model_overrides.warmup.enabled` |
| Model state tracking | No | `benchmark_execution.models[].state` |

## Dependencies
- UF01 (Discovery) - needs manifest
- UF02 (Installation) - model must be installed
- UF03 (Quantization) - may need quantization adjustment before loading
- UF06 (Orchestration) - manages loading within execution flow
