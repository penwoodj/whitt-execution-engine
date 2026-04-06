# UF14: Validation and Quality Scoring

## Overview
Evaluates each model's step outputs using automated quality scoring: code correctness (syntax check + test run), summary quality (compression ratio + key points), reasoning accuracy, factual correctness, instruction compliance, and more.

## User Story
As a benchmark operator, I want each model's output to be automatically scored on quality metrics so that I can objectively compare models without manually reading 800 detail.md files (100 models × 8 steps).

## Pre-conditions
- All 8 step outputs captured for a model (UF06)
- detail.md generated (UF09)
- Scoring criteria and validators defined

## Trigger
All 8 steps complete for a model (scoring runs after each model's benchmark).

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Collect Step │────>│ Score Step  │────>│ Score Step  │
│ 1: Code Gen  │     │ 1: Syntax + │     │ 2: Summary  │
│ Output       │     │     Tests   │     │ Compression │
└──────────────┘     └──────┬───────┘     └──────┬───────┘
                            │                     │
                            ▼                     ▼
                     ┌──────────────┐     ┌──────────────┐
                     │ Score Steps  │     │ Score Steps  │
                     │ 3-5          │     │ 6-8          │
                     │ Reasoning,   │     │ Instruct.,   │
                     │ Creative,    │     │ Multi-Turn,  │
                     │ Factual QA   │     │ Tool Plan    │
                     └──────┬───────┘     └──────┬───────┘
                            │                     │
                            └──────────┬──────────┘
                                       │
                                       ▼
                                ┌──────────────┐
                                │ Calculate    │
                                │ Weighted     │
                                │ Total Score  │
                                └──────────────┘
```

## Scoring Criteria

| Step | Validator | Metric | Weight | Scoring Method |
|------|-----------|--------|--------|----------------|
| 1: Code Gen | Syntax checker | Compilation rate | 15% | Parse output, attempt compilation |
| 1: Code Gen | Test runner | Test pass rate | 10% | Run generated tests (if applicable) |
| 2: Summary | Compression | Summary ratio | 5% | len(summary) / len(source) — ideal: 0.2-0.3 |
| 2: Summary | Key points | Coverage of key info | 5% | Keyword matching against source |
| 3: Reasoning | Logic checker | Correct answer | 15% | Known-answer problem set |
| 4: Creative | Human-like | Style score | 5% | Flesch-Kincaid, vocabulary diversity |
| 5: Factual QA | Fact checker | Accuracy | 15% | Ground-truth answer matching |
| 6: Instruction | Format validator | Compliance rate | 10% | Output matches required format |
| 7: Multi-Turn | Context check | Coherence | 10% | References prior turns correctly |
| 8: Tool Plan | Plan validator | Plan completeness | 10% | Required tools mentioned, logical order |

## Step-by-Step

### Step 1: Score Step 1 — Code Generation
- **Action**: Parse code output, check syntax, optionally run tests
- **Schema Properties Used**: `validation_loop` (for code validation)
- **Input**: Step 1 output (code text)
- **Output**: Score 0.0-1.0 (compilation + test metrics)
- **Error Handling**: If no code generated → score 0. If language not supported for testing → score syntax only.
- **New Schema Requirements**: `benchmark_scoring.step_1.validators: [syntax_check, test_run]`

### Step 2: Score Step 2 — Summarization
- **Action**: Calculate compression ratio and key point coverage
- **Schema Properties Used**: N/A
- **Input**: Source text, model summary
- **Output**: Score 0.0-1.0
- **Error Handling**: If source unavailable, score compression only
- **New Schema Requirements**: `benchmark_scoring.step_2.metrics: [compression_ratio, key_point_coverage]`

### Step 3: Score Steps 3-5 — Reasoning, Creative, Factual
- **Action**: Apply domain-specific validators for each step
- **Schema Properties Used**: `validation_loop.exact_criteria`, `validation_loop.tolerance`
- **Input**: Step outputs, ground truth answers (for reasoning and factual)
- **Output**: Scores 0.0-1.0 per step
- **Error Handling**: If no ground truth available, use heuristic scoring only
- **New Schema Requirements**: `benchmark_scoring.ground_truth_path: string`

### Step 4: Score Steps 6-8 — Instruction, Multi-Turn, Tool Plan
- **Action**: Validate format compliance, context coherence, plan completeness
- **Schema Properties Used**: N/A
- **Input**: Step outputs, format specs
- **Output**: Scores 0.0-1.0 per step
- **Error Handling**: If format spec missing, score manually
- **New Schema Requirements**: `benchmark_scoring.step_6.format_spec: string`

### Step 5: Calculate Weighted Total
- **Action**: Combine all step scores into a single weighted total score
- **Schema Properties Used**: N/A
- **Input**: Per-step scores, weights
- **Output**: Total score 0.0-1.0
- **Error Handling**: If any step scored "not_scored", exclude from weighted total and renormalize
- **New Schema Requirements**: `benchmark_scoring.weights` — per-step weight configuration

### Step 6: Record Scores
- **Action**: Write scores to detail.md and aggregated results
- **Schema Properties Used**: `output.save_to`, `output.format`
- **Input**: Per-step scores, total score
- **Output**: Scores appended to detail.md, added to aggregation dataset
- **Error Handling**: N/A
- **New Schema Requirements**: None

## Post-conditions
- Each step has a quality score 0.0-1.0
- Weighted total score calculated
- Scores recorded in detail.md and available for aggregation (UF10)
- Scoring methodology documented in report (UF17)

## Edge Cases
- Model generates output in wrong language (score based on what was produced, not expected language)
- Ground truth answer has multiple valid forms (fuzzy matching with tolerance)
- Code output is partially correct (partial credit based on test pass rate)
- Creative writing quality is subjective (use objective proxies: vocabulary, sentence length, coherence)
- Model refuses to answer (score 0 for that step, note "refused")

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Validation loop | Yes (`validation_loop`) | None |
| Exact/tolerance matching | Yes (`exact_criteria`, `tolerance`) | None |
| Per-step scoring | No | `benchmark_scoring.step_N.*` |
| Ground truth reference | No | `benchmark_scoring.ground_truth_path` |
| Weight configuration | No | `benchmark_scoring.weights` |
| Scoring methodology | No | Scoring schema definition |

## Dependencies
- UF06 (Orchestration) — provides step outputs
- UF08 (History Capture) — provides chat history for multi-turn scoring
- UF09 (detail.md) — stores scores
- UF10 (Aggregation) — consumes scores for comparison
