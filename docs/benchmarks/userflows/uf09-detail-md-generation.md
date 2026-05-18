# UF09: detail.md Generation

## Overview
Generates an exhaustive markdown file per model containing the complete benchmark results: model metadata, configuration, all 8 step outputs, chat/tool history, timing data, scoring, and error logs.

## User Story
As a benchmark operator, I want each model to produce a comprehensive detail.md file so that I can deep-dive into any individual model's performance, review its actual responses, and understand scoring decisions.

## Pre-conditions
- All 8 workflow steps executed for this model (UF06)
- Chat and tool history captured (UF08)
- Scoring completed (UF14)
- Model metadata available from manifest (UF01)

## Trigger
All 8 steps for a model have completed (succeeded or failed).

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Collect All  │────>│ Generate    │────>│ Write       │
│ Step Outputs │     │ Markdown    │     │ detail.md   │
└──────────────┘     └──────┬───────┘     └──────┬───────┘
                            │                     │
                            ▼                     │
                     ┌──────────────┐              │
                     │ Format      │              │
                     │ Chat History│              │
                     │ as Code     │              │
                     │ Blocks      │              │
                     └──────┬───────┘              │
                            │                     │
                            ▼                     │
                     ┌──────────────┐              │
                     │ Append      │              │
                     │ Metrics &   │              │
                     │ Scoring     │              │
                     └──────────────┘              │
                                                  │
                            ┌─────────────────────┘
                            ▼
                     ┌──────────────┐
                     │ Validate    │
                     │ File Written│
                     └──────────────┘
```

## detail.md Template Structure

```markdown
# Benchmark: {model_name}

## Model Information
- Provider: {provider}
- Parameters: {params}
- Quantization: {quant_level}
- KV Cache Quant: {kv_quant_used}
- Context Length: {context_length}
- Benchmark Date: {timestamp}

## Configuration
- Workflow: {workflow_id}
- Retry Policy: {retry_config}
- Timeout: {timeout_config}
- Quantization Downgrades: {count}

## Results Summary
| Step | Status | Score | Time (s) | Tokens |
|------|--------|-------|----------|--------|
| 1. Code Gen    | ... | ... | ... | ... |
| 2. Summary     | ... | ... | ... | ... |
| ...            | ... | ... | ... | ... |
| **Total**      | ... | ... | ... | ... |

## Step Details

### Step 1: Code Generation
**Prompt:**
\`\`\`
{full_prompt}
\`\`\`

**Response:**
\`\`\`
{full_response}
\`\`\`

**Metrics:** tokens_prompt={n}, tokens_completion={n}, ttft={ms}, total_time={ms}

### Step 2: Summary
...

## Chat History
{Full conversation log from UF08}

## Tool Execution Log
{Tool calls and results from UF08}

## Errors and Warnings
{Any errors encountered during benchmark}

## Scoring Breakdown
{Detailed scoring from UF14}
```

## Step-by-Step

### Step 1: Collect Step Outputs
- **Action**: Gather all 8 step outputs from the execution context, including skipped/failed steps
- **Schema Properties Used**: `output.save_to`, `output.format`
- **Input**: Execution context variables for all 8 steps
- **Output**: Structured collection of step results
- **Error Handling**: Missing step outputs recorded as "skipped" with reason
- **New Schema Requirements**: None

### Step 2: Collect Chat History
- **Action**: Retrieve the full chat/tool history from UF08 for this model
- **Schema Properties Used**: `logging.global.output_type: chat`
- **Input**: Chat history object from UF08
- **Output**: Formatted conversation log
- **Error Handling**: If history unavailable, note "history capture failed" in detail.md
- **New Schema Requirements**: None

### Step 3: Collect Metrics
- **Action**: Gather timing, token usage, VRAM, and resource metrics for this model's run
- **Schema Properties Used**: `metrics.collect`, `logging.scopes.performance_metrics`
- **Input**: Metrics data from execution engine
- **Output**: Metrics summary (per-step and total)
- **Error Handling**: Missing metrics recorded as "N/A"
- **New Schema Requirements**: None

### Step 4: Collect Scoring
- **Action**: Retrieve quality scores from UF14 for each step
- **Schema Properties Used**: `validation_loop` results
- **Input**: Scoring results per step
- **Output**: Score per step + total weighted score
- **Error Handling**: Missing scores recorded as "not_scored"
- **New Schema Requirements**: None

### Step 5: Generate Markdown
- **Action**: Assemble all collected data into the detail.md template
- **Schema Properties Used**: `output.file_output`
- **Input**: Model info, step outputs, chat history, metrics, scoring
- **Output**: Complete detail.md content string
- **Error Handling**: If template rendering fails, use simplified fallback template
- **New Schema Requirements**: None

### Step 6: Write File
- **Action**: Write detail.md to the model's output directory
- **Schema Properties Used**: `output.file_output.path`
- **Input**: Markdown content, output path
- **Output**: detail.md file on disk
- **Error Handling**: On write failure, retry with backup path. If still fails, write to temp directory.
- **New Schema Requirements**: `workspace.output_path` — root output directory for benchmark results

### Step 7: Validate Output
- **Action**: Verify detail.md was written and is non-empty
- **Schema Properties Used**: N/A
- **Input**: Written file path
- **Output**: Validation result (valid/invalid)
- **Error Handling**: If validation fails, log error and continue (don't abort benchmark)
- **New Schema Requirements**: None

## Post-conditions
- detail.md exists for this model in the output directory
- File contains complete benchmark data: model info, all step results, chat history, metrics, scoring
- File is human-readable and can be reviewed independently

## Edge Cases
- Model completed 0 of 8 steps (detail.md shows all skipped)
- Step response exceeds 100KB (truncate in detail.md, note truncation)
- Chat history includes binary tool output (summarize instead of embedding)
- Scoring not available yet (UF14 may run asynchronously — write "pending" placeholder)
- Disk full during write (write to fallback location, log warning)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Output file path | Yes (`output.file_output.path`) | `workspace.output_path` |
| Output format | Yes (`output.format`) | `markdown` as format option |
| Metrics collection | Yes (`metrics.collect`) | None |
| Chat history format | Yes (`logging.global.output_type: chat`) | None |
| Template-based output | Partially | Template engine integration |

## Dependencies
- UF01 (Discovery) — model metadata
- UF06 (Orchestration) — step execution results
- UF08 (History Capture) — chat and tool history
- UF14 (Quality Scoring) — per-step scores
- UF03 (Quantization) — quantization metadata to include
