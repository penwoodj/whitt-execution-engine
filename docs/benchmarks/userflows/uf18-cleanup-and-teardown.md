# UF18: Cleanup and Teardown

## Overview
Gracefully shuts down the benchmark system after completion: unloads all models, stops resource monitors, finalizes checkpoints, and optionally archives or prunes output files.

## User Story
As a benchmark operator, I want the system to clean up after itself so that my GPU memory is freed, providers are in a clean state, and I don't have leftover processes consuming resources after the benchmark finishes.

## Pre-conditions
- Benchmark session completed (all models processed or stopped)
- Final checkpoint saved (UF13)
- Final report generated (UF17)

## Trigger
Benchmark session completes, operator sends stop signal, or fatal error requires shutdown.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Benchmark   │────>│ Save Final  │────>│ Unload All  │
│ Completes   │     │ Checkpoint  │     │ Loaded      │
└──────────────┘     └──────────────┘     │ Models      │
                                           └──────┬───────┘
                                                  │
                                                  ▼
                                           ┌──────────────┐
                                           │ Stop        │
                                           │ Resource    │
                                           │ Monitors    │
                                           └──────┬───────┘
                                                  │
                                                  ▼
                                           ┌──────────────┐
                                           │ Close       │
                                           │ Provider    │
                                           │ Connections │
                                           └──────┬───────┘
                                                  │
                                                  ▼
                                           ┌──────────────┐
                                           │ Archive /   │──── Yes ──> ┌──────────┐
                                           │ Prune       │            │ Compress │
                                           │ Old Files?  │            │ Outputs  │
                                           └──────────────┘            └──────────┘
```

## Step-by-Step

### Step 1: Save Final Checkpoint
- **Action**: Write a final checkpoint marking the benchmark as complete
- **Schema Properties Used**: `state_management.checkpoint_directory`, `checkpointing.enabled`
- **Input**: Final progress state (all models processed)
- **Output**: Final checkpoint file
- **Error Handling**: If checkpoint write fails, log error but don't prevent teardown
- **New Schema Requirements**: None

### Step 2: Unload All Loaded Models
- **Action**: Unload any models still in GPU memory
- **Schema Properties Used**: `workflow_execution_strategy.load_unload`
- **Input**: List of currently loaded models
- **Output**: All models unloaded, GPU memory freed
- **Error Handling**: If unload fails for a model, force disconnect from provider
- **New Schema Requirements**: None

### Step 3: Stop Resource Monitors
- **Action**: Stop all resource monitoring probes
- **Schema Properties Used**: `benchmark_execution.monitoring.enabled`
- **Input**: Active monitor handles
- **Output**: Monitors stopped
- **Error Handling**: Force kill monitor processes if graceful stop fails
- **New Schema Requirements**: None

### Step 4: Close Provider Connections
- **Action**: Close all provider adapter connections gracefully
- **Schema Properties Used**: `models.host.endpoint`
- **Input**: Active provider adapter instances
- **Output**: All connections closed
- **Error Handling**: Force close if graceful close times out
- **New Schema Requirements**: None

### Step 5: Finalize Log Files
- **Action**: Flush and close all log files
- **Schema Properties Used**: `logging.global.output_type`, `logging.global.format`
- **Input**: Open log file handles
- **Output**: All logs flushed to disk
- **Error Handling**: Force close if flush fails
- **New Schema Requirements**: None

### Step 6: Archive or Prune Outputs (Optional)
- **Action**: Optionally compress detail.md files, prune temp files, archive old benchmark results
- **Schema Properties Used**: `workspace.output_path`
- **Input**: Output directory, pruning policy
- **Output**: Clean output directory
- **Error Handling**: If archive fails, leave files as-is (data loss risk)
- **New Schema Requirements**: `benchmark_execution.cleanup.archive_old_results: bool`, `benchmark_execution.cleanup.prune_temp_files: bool`

### Step 7: Report Telemetry
- **Action**: Log final benchmark statistics (total time, models completed, errors encountered)
- **Schema Properties Used**: `logging.scopes.performance_metrics`
- **Input**: Final statistics
- **Output**: Telemetry log entry
- **Error Handling**: N/A
- **New Schema Requirements**: None

## Post-conditions
- All models unloaded from GPU memory
- All provider connections closed
- All resource monitors stopped
- All log files flushed and closed
- Final checkpoint saved
- System resources freed (GPU at idle, no orphan processes)

## Edge Cases
- Benchmark killed with SIGKILL (no graceful teardown — next run detects incomplete checkpoint)
- Provider crashes during teardown (force close, log error)
- Disk full during final checkpoint (log error, proceed with teardown)
- Operator stops benchmark mid-run (teardown from partial state, save checkpoint first)
- Multiple benchmark instances running (only tear down own resources)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Model unload | Yes (`load_unload`) | None |
| Checkpoint finalization | Yes (`checkpointing`) | None |
| Log finalization | Yes (`logging`) | None |
| Cleanup policies | No | `benchmark_execution.cleanup.*` |
| Telemetry reporting | Partially | `metrics.collect` completion event |

## Dependencies
- UF04 (Loading) — needs to know what's loaded to unload
- UF06 (Orchestration) — final state to teardown
- UF11 (Concurrent) — per-worker teardown
- UF13 (Checkpointing) — final checkpoint before teardown
- UF15 (Monitoring) — monitors to stop
