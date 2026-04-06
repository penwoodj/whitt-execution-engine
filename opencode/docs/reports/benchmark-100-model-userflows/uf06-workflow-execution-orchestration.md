# UF06: Workflow Execution Orchestration

## Overview
Execute the 8-prompt benchmark workflow across all 100 registered models with fault tolerance, managing model lifecycle, progress tracking, and execution ordering.

## User Story
As a benchmark operator, I want to run the benchmark workflow across all 100 models automatically with progress tracking and fault tolerance so that I can go to sleep and wake up to results.

## Pre-conditions
- Benchmark manifest complete (UF01)
- All models installed (UF02) or quantized (UF03)
- 8-prompt workflow defined (UF05)
- Resource allocation configured (UF04)

## Trigger
User initiates benchmark session.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Load        │────>│ For Each     │────>│ Execute     │
│ Manifest    │     │ Model in     │     │ Workflow    │
│             │     │ Manifest    │     │ (8 steps)  │
└──────┬─────┘     └──────┬─────┘     └──────┬─────┘
       │                   │                   │
       ▼                   ▼                   ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Load Model  │     │ Run Steps    │     │ Save        │
│ (UF04)      │     │ Sequentially  │     │ Results     │
└──────┬─────┘     └──────┬──────┘     └──────┬─────┘
       │                   │                   │
       ▼                   ▼                   ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Unload      │     │ Error?       │     │ Next Model  │
│ Model       │     │ Skip/Fail    │     │             │
└──────────────┘     └──────────────┘     └──────────────┘
```

## Step-by-Step

### Step 1: Initialize Execution Context
- **Action**: Create benchmark session, load manifest, prepare output directories
- **Schema Properties Used**: `workflow_id`, `workspace.output_path`
- **Input**: Benchmark manifest ID
- **Output**: Execution context (session ID, output dir, model queue)
- **Error Handling**: If manifest load fails, abort with clear error
- **New Schema Requirements**: None

### Step 2: Create Model Execution Queue
- **Action**: Build ordered list of models from manifest, applying any filters
- **Schema Properties Used**: `benchmark_manifest.models[]`
- **Input**: Manifest, optional filters (skip models, priority order)
- **Output**: Ordered model queue with estimated total time
- **Error Handling**: Empty queue = abort with summary
- **New Schema Requirements**: None

### Step 3: Process Next Model
- **Action**: Pop model from queue, load model (UF04), execute workflow
- **Schema Properties Used**: `workflow_execution_strategy.load_unload`, `retry.level: step_unload`
- **Input**: Model config from manifest
- **Output**: 8 step results + timing data + error log
- **Error Handling**: On model load failure: try quantization (UF03), then skip. On workflow error: save partial results, continue.
- **New Schema Requirements**: None

### Step 4: Execute 8-Prompt Workflow
- **Action**: Run steps sequentially, passing outputs between steps via variables
- **Schema Properties Used**: `agentic_workflow.steps`, `input_variables`, `output`
- **Input**: Loaded model, workflow YAML, model_overrides
- **Output**: Per-step results (response text, timing, tokens)
- **Error Handling**: On step failure: save what we have, continue to next step with null input. On model timeout: skip remaining steps.
- **New Schema Requirements**: None

### Step 5: Save Model Results
- **Action**: Persist all step results to model's detail.md file
- **Schema Properties Used**: `output.file_output.path`
- **Input**: Step results, model metadata
- **Output**: `detail.md` file with full benchmark data
- **Error Handling**: On save failure: retry once, then keep in memory
- **New Schema Requirements**: `output.file_output.path` with model ID interpolation

### Step 6: Unload Model
- **Action**: Free GPU memory, prepare for next model
- **Schema Properties Used**: `workflow_execution_strategy.memory.model_lifecycle.unload_unused`
- **Input**: Loaded model ID
- Output: VRAM freed, model state = unloaded
- **Error Handling**: If unload fails (stuck model), attempt force unload after timeout
- **New Schema Requirements**: None

### Step 7: Update Progress
- **Action**: Update progress manifest (X/100 complete, ETA, success/fail counts)
- **Schema Properties Used**: `checkpointing.triggers.interval.steps`
- **Input**: Current model result
- **Output**: Updated progress manifest
- **Error Handling**: Write progress on every model completion (don't batch)
- **New Schema Requirements**: None

### Step 8: Check Completion
- **Action**: If all models processed, generate summary. If interrupted, save checkpoint for resume.
- **Schema Properties Used**: `checkpointing.enabled`, `checkpointing.triggers.on_timeout`
- **Input**: Model queue status
- Output: Completion status or checkpoint save
- **Error Handling**: N/A
- **New Schema Requirements**: None

## Post-conditions
- All 100 models attempted (or skipped with reason)
- Per-model results saved as detail.md
- Progress trackable and resumable
- GPU memory freed after completion

## Edge Cases
- User interrupts mid-benchmark (checkpoint + resume)
- Provider crashes during execution (skip model, continue)
- System runs out of disk space (checkpoint + warn + continue)
- Model takes extremely long (timeout per step, move on)
- 100 models × 8 steps = 800 inference calls (token budget considerations)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Serial model processing | Yes (`load_unload: one_at_a_time`) | None |
| Model lifecycle | Yes (`model_lifecycle`) | None |
| Variable passing between steps | Yes (`input_variables`) | None |
| Per-model output files | Yes (`output.file_output.path`) | None |
| Progress tracking | Partially | `checkpointing.triggers` |
| Checkpoint/resume | Yes (`checkpointing.time_travel`) | None |
| Execution order | Partially | `advanced_scheduling.algorithm` |

## Estimated Resource Usage

| Models | Steps/Model | Inferences | Est. Time (at 10s/inf) |
|--------|-------------|-----------|---------------------|
| 100 | 8 | 800 | ~2.2 hours |
| 100 | 8 | 800 | ~2.2 hours |

## Dependencies
- UF01, UF02, UF03, UF04 (prerequisites)
- UF07 (Fault Tolerance) - handles errors during execution
- UF08 (History Capture) - captures step results
- UF09 (Detail.md Generation) - formats output
- UF13 (Progress Tracking) - tracks progress
