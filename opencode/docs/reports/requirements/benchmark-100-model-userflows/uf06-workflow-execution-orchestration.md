# UF06: Workflow Execution Orchestration

## Overview
Manages the serial execution of the 8-prompt benchmark workflow against each model: load model, execute each of the 8 steps sequentially, capture outputs, then unload model. Handles the full load-execute-unload cycle per model.

## User Story
As a benchmark operator, I want the system to automatically load each model, run the full 8-prompt benchmark workflow, capture all outputs, and unload the model so that I can benchmark 100+ models sequentially without manual intervention.

## Pre-conditions
- 8-prompt workflow defined (UF05)
- Models installed and registered (UF01, UF02)
- Model loading infrastructure ready (UF04)
- Output directories created

## Trigger
Benchmark session started for a specific model, or next model in the benchmark queue.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Load Model   │────>│ Execute      │────>│ Capture     │
│ (UF04)       │     │ Step 1      │     │ Step 1      │
└──────────────┘     │ Code Gen    │     │ Output      │
                     └──────┬───────┘     └──────┬───────┘
                            │                     │
                            ▼                     │
                     ┌──────────────┐              │
                     │ Execute      │              │
                     │ Step 2      │              │
                     │ Summary     │              │
                     └──────┬───────┘              │
                            │                     │
                            ▼                     │
                     ┌──────────────┐              │
                     │ Execute      │              │
                     │ Steps 3-7    │              │
                     │ (Reasoning → │              │
                     │  Multi-Turn) │              │
                     └──────┬───────┘              │
                            │                     │
                            ▼                     │
                     ┌──────────────┐              │
                     │ Execute      │              │
                     │ Step 8      │              │
                     │ Tool Plan   │              │
                     └──────┬───────┘              │
                            │                     │
                            ▼                     ▼
                     ┌──────────────┐     ┌──────────────┐
                     │ Write       │────>│ Unload      │
                     │ detail.md   │     │ Model       │
                     └──────────────┘     └──────┬───────┘
                                                  │
                                                  ▼
                                           ┌──────────────┐
                                           │ Next Model   │
                                           │ in Queue     │
                                           └──────────────┘
```

## Step-by-Step

### Step 1: Load Model
- **Action**: Load the next model in the benchmark queue into GPU memory (delegates to UF04)
- **Schema Properties Used**: `models.<id>.host.type`, `models.<id>.model_memory.kv_cache_quantization`
- **Input**: Model ID from benchmark queue
- **Output**: Loaded model ready for inference, or load failure
- **Error Handling**: On load failure, trigger UF03 (quantization fallback). If still fails after fallback, skip model and log.
- **New Schema Requirements**: None (delegated to UF03/UF04)

### Step 2: Initialize Execution Context
- **Action**: Create a fresh execution context for this model's benchmark run, including variable space for step outputs
- **Schema Properties Used**: `workflow_execution_strategy.load_unload` (one_at_a_time)
- **Input**: Workflow YAML (UF05), model ID
- **Output**: Execution context with empty variable bindings
- **Error Handling**: If context creation fails, abort this model's benchmark
- **New Schema Requirements**: None

### Step 3: Execute Step 1 — Code Generation
- **Action**: Send the code generation prompt to the model and capture the response
- **Schema Properties Used**: `agentic_workflow.steps.step_1`, `inputs`, `model_overrides.timeout.time_to_first_response`
- **Input**: Prompt template, model reference
- **Output**: Step 1 output saved to execution context variable
- **Error Handling**: On timeout: retry once, then record timeout and continue to next step. On model error: log and continue.
- **New Schema Requirements**: None

### Step 4: Execute Steps 2-7 Sequentially
- **Action**: Execute steps 2 through 7 (Summary, Reasoning, Creative, Factual QA, Instruction Following, Multi-Turn) in order, passing prior outputs as context
- **Schema Properties Used**: `agentic_workflow.steps.step_2` through `step_7`, `inputs`, `"${step.step_N.output}"`
- **Input**: Each step's prompt + outputs from prior steps
- **Output**: Each step's output saved to context variable
- **Error Handling**: Per-step: retry once on timeout. On persistent failure, record error and continue to next step (don't abort entire benchmark for one step).
- **New Schema Requirements**: `retry.level: step_continue` — continue to next step instead of aborting workflow

### Step 5: Execute Step 8 — Tool Use Planning
- **Action**: Execute final step (tool use planning), which may reference all prior step outputs
- **Schema Properties Used**: `agentic_workflow.steps.step_8`, `inputs`
- **Input**: Prompt + accumulated context from steps 1-7
- **Output**: Step 8 output saved to context variable
- **Error Handling**: Same as Step 4
- **New Schema Requirements**: None

### Step 6: Capture All Step Outputs
- **Action**: Collect all 8 step outputs from execution context into a structured result object
- **Schema Properties Used**: `output.save_to`, `output.format`
- **Input**: Execution context variables
- **Output**: Structured result with all 8 step outputs + timing metadata
- **Error Handling**: If any step output is missing, record as "skipped" rather than failing
- **New Schema Requirements**: None

### Step 7: Write detail.md
- **Action**: Generate the exhaustive detail.md file for this model (delegates to UF09)
- **Schema Properties Used**: `output.file_output`
- **Input**: All step outputs, timing data, model metadata, quantization info
- **Output**: `detail.md` file in model's output directory
- **Error Handling**: On write failure, retry with backup path
- **New Schema Requirements**: None (delegated to UF09)

### Step 8: Unload Model
- **Action**: Unload model from GPU memory to free resources for next model
- **Schema Properties Used**: `workflow_execution_strategy.load_unload` (one_at_a_time), `workflow_execution_strategy.memory.pressure_handling.on_oom`
- **Input**: Loaded model reference
- **Output**: GPU memory freed, model state set to "unloaded"
- **Error Handling**: On unload failure, force kill provider connection and retry. Log the failure.
- **New Schema Requirements**: None

### Step 9: Record Timing and Resource Metrics
- **Action**: Log total benchmark time, per-step times, VRAM peak usage, token counts
- **Schema Properties Used**: `metrics.collect`, `logging.scopes`
- **Input**: Timing data from steps, resource monitors
- **Output**: Metrics entry for this model's benchmark run
- **Error Handling**: N/A
- **New Schema Requirements**: `benchmark_execution.models[].metrics` — per-model benchmark metrics

## Post-conditions
- Model has been fully benchmarked (all 8 steps executed or skipped with reason)
- detail.md generated with all step results
- Model unloaded from GPU memory
- Metrics recorded for this model
- Benchmark queue advanced to next model

## Edge Cases
- Model hangs mid-step (timeout kills the step, continues to next)
- Model produces empty responses (record as "empty_output" and continue)
- Context window exceeded during multi-turn step (truncate context, note in detail.md)
- Model crashes during execution (unload, skip, log, continue to next model)
- Power or system interruption during benchmark (handled by UF13 checkpointing)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Serial step execution | Yes (`agentic_workflow.steps`) | None |
| Variable passing between steps | Yes (`inputs`, `"${step.step_N.output}"`) | None |
| Load/unload cycle | Yes (`workflow_execution_strategy.load_unload`) | None |
| Per-step timeout | Yes (`model_overrides.timeout`) | None |
| Error continuation to next step | Partially | `retry.level: step_continue` |
| Per-model metrics | No | `benchmark_execution.models[].metrics` |
| Execution context management | Yes (implicit in workflow engine) | Explicit context schema |

## Dependencies
- UF01 (Discovery) — model list
- UF02 (Installation) — model installed
- UF03 (Quantization) — quantization fallback on OOM
- UF04 (Loading) — actual model load/unload
- UF05 (Workflow Definition) — 8-prompt workflow structure
- UF09 (detail.md Generation) — output file creation
- UF13 (Checkpointing) — resume capability on interruption
