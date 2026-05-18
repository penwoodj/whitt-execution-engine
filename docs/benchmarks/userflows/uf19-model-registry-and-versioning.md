# UF19: Model Registry and Versioning

## Overview
Maintains a persistent registry of all models ever benchmarked, tracking model versions, benchmark history, provenance metadata, and enabling historical comparison across benchmark runs.

## User Story
As a benchmark operator, I want a persistent model registry so that I can track which models I've benchmarked, compare results across different benchmark runs, and see how model performance changes over time.

## Pre-conditions
- Model registry storage exists (local file or database)
- Benchmark session configured to update registry

## Trigger
Model benchmark completes (add entry to registry), or registry queried for historical comparison.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Model       │────>│ Check       │────>│ Update      │
│ Benchmarked │     │ Registry    │     │ Existing    │
│             │     │ for Entry   │     │ Entry       │
└──────────────┘     └──────┬───────┘     └──────┬───────┘
                            │                     │
                     Exists │               New   │
                            ▼                     ▼
                     ┌──────────────┐     ┌──────────────┐
                     │ Append New  │     │ Create New  │
                     │ Run Result  │     │ Registry    │
                     │ to History  │     │ Entry       │
                     └──────┬───────┘     └──────┬───────┘
                            │                     │
                            └──────────┬──────────┘
                                       │
                                       ▼
                                ┌──────────────┐
                                │ Write       │
                                │ Updated     │
                                │ Registry    │
                                └──────────────┘
```

## Registry Entry Schema

```yaml
models:
  llama-3.2-3b-instruct-q4_k_m:
    first_seen: "2026-04-01"
    last_benchmarked: "2026-04-06"
    provider: ollama
    parameters: 3.2B
    quantization: q4_k_m
    source:
      registry: ollama
      url: "https://ollama.com/library/llama3.2"
    benchmark_history:
      - session_id: bench_2026_04_06_001
        date: "2026-04-06"
        score: 0.78
        status: success
        quant_downgrades: 0
        total_time_seconds: 342
      - session_id: bench_2026_03_28_002
        date: "2026-03-28"
        score: 0.72
        status: partial
        quant_downgrades: 1
        total_time_seconds: 456
    tags: [small, fast, instruction-tuned]
    notes: "Good for code gen, weak on factual QA"
```

## Step-by-Step

### Step 1: Load Existing Registry
- **Action**: Load the model registry from persistent storage
- **Schema Properties Used**: N/A (registry is external to workflow schema)
- **Input**: Registry file path
- **Output**: Current registry data
- **Error Handling**: If registry missing, create new empty registry
- **New Schema Requirements**: `benchmark_registry.path: string` — registry file location

### Step 2: Check for Existing Model Entry
- **Action**: Look up the benchmarked model in the registry by name+quantization+provider
- **Schema Properties Used**: N/A
- **Input**: Model metadata from manifest
- **Output**: Existing entry or null (new model)
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 3: Create or Update Entry
- **Action**: If model exists, append new benchmark run to history. If new, create full entry.
- **Schema Properties Used**: `benchmark_manifest.models[]`
- **Input**: Model metadata, benchmark result
- **Output**: Updated registry entry
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 4: Store Registry
- **Action**: Write updated registry back to persistent storage
- **Schema Properties Used**: N/A
- **Input**: Updated registry data
- **Output**: Registry file on disk
- **Error Handling**: Atomic write (write to temp, rename). If fails, don't lose existing registry.
- **New Schema Requirements**: None

### Step 5: Enable Historical Comparison
- **Action**: When generating reports, cross-reference registry to show trend data
- **Schema Properties Used**: N/A
- **Input**: Current results, historical registry data
- **Output**: Trend data in report (e.g., "Score improved 8% since last run")
- **Error Handling**: If no history, skip trend analysis
- **New Schema Requirements**: None

### Step 6: Tag and Annotate Models
- **Action**: Allow operator to add tags and notes to model entries for future reference
- **Schema Properties Used**: N/A
- **Input**: Model ID, tags, notes
- **Output**: Updated registry entry
- **Error Handling**: N/A
- **New Schema Requirements**: `benchmark_registry.models.<id>.tags: string[]`, `benchmark_registry.models.<id>.notes: string`

## Post-conditions
- Model registry updated with latest benchmark results
- Historical comparison data available for reports
- New models registered with full metadata
- Registry persisted to disk

## Edge Cases
- Same model name with different quantization (treated as separate entries)
- Same model on different providers (separate entries with provider tag)
- Registry file corrupted (backup last known good version)
- Registry grows very large (1000+ models × 50+ runs = pruning needed)
- Benchmark run with same session ID as existing (detect and prevent duplicate)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Model metadata | Yes (`models.<id>.*`) | None |
| Benchmark history | No | `benchmark_registry` root section |
| Tags and annotations | No | `models.<id>.tags`, `models.<id>.notes` |
| Registry persistence | No | `benchmark_registry.path` |
| Trend analysis | No | Historical comparison logic |

## Dependencies
- UF01 (Discovery) — model metadata
- UF06 (Orchestration) — benchmark results
- UF14 (Scoring) — quality scores for registry
- UF17 (Report) — historical comparison in reports
