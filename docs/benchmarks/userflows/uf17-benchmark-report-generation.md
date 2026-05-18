# UF17: Benchmark Report Generation

## Overview
Generates the final benchmark report: executive summary, ranking tables, per-capability analysis, performance charts data, fault analysis, and recommendations. Consumes aggregated results from UF10.

## User Story
As a benchmark operator, I want a comprehensive report with rankings, tables, and analysis so that I can understand which models excel at what, identify trends, and make informed decisions about model selection.

## Pre-conditions
- All models benchmarked (or benchmark completed with some failures)
- Aggregated results available (UF10)
- All detail.md files generated (UF09)

## Trigger
Benchmark session completes (all models processed or benchmark stopped).

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Load        │────>│ Generate    │────>│ Generate    │
│ Aggregated  │     │ Executive   │     │ Ranking     │
│ Results     │     │ Summary     │     │ Tables      │
└──────────────┘     └──────┬───────┘     └──────┬───────┘
                            │                     │
                            ▼                     ▼
                     ┌──────────────┐     ┌──────────────┐
                     │ Per-Step    │     │ Performance │
                     │ Analysis    │     │ Statistics  │
                     └──────┬───────┘     └──────┬───────┘
                            │                     │
                            ▼                     ▼
                     ┌──────────────┐     ┌──────────────┐
                     │ Fault       │     │ Resource    │
                     │ Analysis    │     │ Utilization │
                     └──────┬───────┘     └──────┬───────┘
                            │                     │
                            └──────────┬──────────┘
                                       │
                                       ▼
                                ┌──────────────┐
                                │ Write       │
                                │ Final       │
                                │ Report.md   │
                                └──────────────┘
```

## Report Structure

```markdown
# Benchmark Report: {session_id}
## Executive Summary
- Total models: N (X succeeded, Y partial, Z failed)
- Date range: {start} — {end}
- Hardware: {GPU specs}
- Key finding: {one-liner}

## Overall Rankings (Top 20)
| Rank | Model | Params | Quant | Score | Time |
|------|-------|--------|-------|-------|------|

## Per-Capability Rankings
### Code Generation
### Summarization
### Reasoning
### ... (8 capability tables)

## Performance Statistics
- Mean total time: {X}s
- Mean time per step: table
- Token efficiency: table

## Fault Analysis
- Models that failed: list with reasons
- Most common error types: table
- Quantization downgrades: table

## Resource Utilization
- Peak VRAM usage
- Average GPU utilization
- Thermal events

## Recommendations
- Best models per capability
- Best value (quality/speed ratio)
- Models to avoid

## Appendix
- Full results table
- Methodology notes
- Scoring criteria details
```

## Step-by-Step

### Step 1: Load Aggregated Results
- **Action**: Load the aggregated dataset from UF10
- **Schema Properties Used**: `benchmark_results.aggregated`
- **Input**: Aggregated results file/dataset
- **Output**: In-memory results data
- **Error Handling**: If results missing, generate report with available data and note gaps
- **New Schema Requirements**: None

### Step 2: Generate Executive Summary
- **Action**: Calculate top-level stats (total models, success rate, time range, key finding)
- **Schema Properties Used**: N/A
- **Input**: Aggregated results
- **Output**: Executive summary text
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 3: Generate Ranking Tables
- **Action**: Sort models by overall score and per-capability scores, generate markdown tables
- **Schema Properties Used**: `benchmark_results.rankings`
- **Input**: Aggregated results with scores
- **Output**: Ranking tables (overall + 8 capability-specific)
- **Error Handling**: If insufficient data for ranking, note small sample
- **New Schema Requirements**: None

### Step 4: Generate Performance Statistics
- **Action**: Calculate timing stats (mean, median, P95 per step), token efficiency
- **Schema Properties Used**: `metrics.collect`
- **Input**: Per-model timing data
- **Output**: Performance statistics tables
- **Error Handling**: Missing timing data → "N/A"
- **New Schema Requirements**: None

### Step 5: Generate Fault Analysis
- **Action**: Summarize failures, error types, quantization downgrades
- **Schema Properties Used**: `benchmark_manifest.models[].status`
- **Input**: Failure records from UF07
- **Output**: Fault analysis section
- **Error Handling**: No faults → "All models completed successfully"
- **New Schema Requirements**: None

### Step 6: Generate Resource Utilization Summary
- **Action**: Summarize resource usage patterns from monitoring data (UF15)
- **Schema Properties Used**: `metrics.collect`
- **Input**: Resource monitoring logs
- **Output**: Resource utilization section
- **Error Handling**: Missing monitoring data → note "resource monitoring was not enabled"
- **New Schema Requirements**: None

### Step 7: Generate Recommendations
- **Action**: Derive recommendations from results (best per capability, best value, models to avoid)
- **Schema Properties Used**: N/A
- **Input**: Ranking tables, performance stats
- **Output**: Recommendations section
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 8: Write Final Report
- **Action**: Assemble all sections into a single markdown report file
- **Schema Properties Used**: `output.file_output`
- **Input**: All report sections
- **Output**: `benchmark-report-{session_id}.md`
- **Error Handling**: On write failure, retry. On truncation, split into multiple files.
- **New Schema Requirements**: None

## Post-conditions
- Comprehensive benchmark report exists with rankings, stats, analysis
- Report is human-readable and self-contained
- Report references individual detail.md files for deep dives

## Edge Cases
- All models failed (report shows 0% success rate, analyzes failure patterns)
- Only 1 model completed (limited analysis, note small sample)
- Models vary wildly in size (segment by size tiers in rankings)
- Scoring methodology changed mid-benchmark (note inconsistency in report)
- Report exceeds reasonable length (generate summary + detailed appendix)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Aggregated results | Partially | `benchmark_results.aggregated` |
| Output file | Yes (`output.file_output`) | None |
| Metrics data | Yes (`metrics.collect`) | None |
| Report template | No | Report generation template |
| Ranking data | Partially | `benchmark_results.rankings` |

## Dependencies
- UF09 (detail.md) — individual model details
- UF10 (Aggregation) — cross-model comparison data
- UF14 (Scoring) — quality scores
- UF15 (Resource Monitoring) — utilization data
- UF07 (Fault Tolerance) — failure analysis data
