# UF10: Benchmark Results Aggregation

## Overview
Collects individual model results (detail.md, metrics, scores) and aggregates them into a unified dataset for cross-model comparison, ranking, and analysis.

## User Story
As a benchmark operator, I want to aggregate results from all 100 models into a single structured dataset so that I can compare models, rank them by capability, and identify performance patterns.

## Pre-conditions
- At least one model's benchmark completed (detail.md exists)
- Output directory contains model result files
- Scoring completed for completed models

## Trigger
A model's benchmark completes and its detail.md is written, or explicitly requested after full benchmark run.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Scan Output  │────>│ Parse Each  │────>│ Extract     │
│ Directory    │     │ detail.md   │     │ Key Metrics │
└──────────────┘     └──────────────┘     └──────┬───────┘
                                                  │
                                                  ▼
                                           ┌──────────────┐
                                           │ Normalize   │
                                           │ Metrics     │
                                           └──────┬───────┘
                                                  │
                                                  ▼
                                           ┌──────────────┐
                                           │ Build       │
                                           │ Comparison  │
                                           │ Table       │
                                           └──────┬───────┘
                                                  │
                                                  ▼
                                           ┌──────────────┐
                                           │ Rank Models │
                                           │ Per Category│
                                           └──────────────┘
```

## Step-by-Step

### Step 1: Scan Output Directory
- **Action**: Enumerate all detail.md files in the benchmark output directory
- **Schema Properties Used**: `workspace.output_path`
- **Input**: Output directory path
- **Output**: List of model result files with timestamps
- **Error Handling**: If directory doesn't exist, error and suggest running benchmark first
- **New Schema Requirements**: `workspace.output_path` — benchmark results root directory

### Step 2: Parse Each detail.md
- **Action**: Extract structured data from each model's detail.md (model info, step scores, timing, token counts)
- **Schema Properties Used**: N/A (parsing logic)
- **Input**: detail.md file path
- **Output**: Structured result record per model
- **Error Handling**: If parsing fails, skip file and log warning. If partial parse, record what was extracted.
- **New Schema Requirements**: None

### Step 3: Extract Key Metrics
- **Action**: Normalize metrics across models into comparable format (adjusting for quantization differences, context length, etc.)
- **Schema Properties Used**: `metrics.collect`
- **Input**: Parsed result records
- **Output**: Normalized metric set per model per step
- **Error Handling**: Missing metrics filled with "N/A" (don't interpolate)
- **New Schema Requirements**: None

### Step 4: Build Comparison Table
- **Action**: Create a cross-model comparison table with all key metrics side by side
- **Schema Properties Used**: N/A (output formatting)
- **Input**: Normalized metrics from all models
- **Output**: Comparison data structure (JSON/CSV-ready)
- **Error Handling**: N/A
- **New Schema Requirements**: `benchmark_results.aggregated` — aggregated result schema

### Step 5: Rank Models Per Category
- **Action**: Rank models by overall score and per-step capability scores
- **Schema Properties Used**: N/A (ranking logic)
- **Input**: Aggregated metrics
- **Output**: Rankings table (overall, per-step, per-capability)
- **Error Handling**: Models with incomplete benchmarks ranked separately (with note)
- **New Schema Requirements**: `benchmark_results.rankings` — ranking result schema

### Step 6: Calculate Aggregate Statistics
- **Action**: Compute summary statistics across all models (mean, median, std dev, min, max per metric)
- **Schema Properties Used**: N/A
- **Input**: Aggregated metrics
- **Output**: Statistical summary (per-step and overall)
- **Error Handling**: If too few models for meaningful stats, note small sample size
- **New Schema Requirements**: None

## Post-conditions
- Aggregated results dataset exists with all completed models
- Models ranked by overall score and per-capability
- Comparison table ready for report generation (UF17)
- Aggregate statistics computed

## Edge Cases
- Only 1 model completed (limited comparison, note in report)
- Models with different quantization levels (flag quantization differences in comparison)
- Models with different parameter sizes (separate into size tiers for fair comparison)
- Some steps skipped for some models (exclude from per-step ranking, include in overall with penalty)
- 500+ models (pagination or summary aggregation needed)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Output file location | Partially | `workspace.output_path` |
| Metrics collection | Yes (`metrics.collect`) | None |
| Aggregated results | No | `benchmark_results.aggregated` |
| Rankings | No | `benchmark_results.rankings` |
| Per-step comparison | No | Comparison schema |

## Dependencies
- UF09 (detail.md Generation) — source data for aggregation
- UF14 (Quality Scoring) — scores to compare
- UF17 (Report Generation) — consumes aggregated results
