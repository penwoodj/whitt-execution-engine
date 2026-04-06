# UF13: Progress Tracking and Checkpointing

## Overview
Tracks benchmark progress in real-time, persists state to checkpoints so that a crashed or interrupted benchmark can resume from the last known good state instead of starting over.

## User Story
As a benchmark operator, I want the benchmark to save progress periodically so that if my machine crashes during a 48-hour 100-model benchmark, I can resume where I left off instead of starting from scratch.

## Pre-conditions
- Benchmark session started
- Checkpoint directory configured
- State persistence mechanism available

## Trigger
- Periodic interval timer (e.g., every model completion)
- On model completion (per-model checkpoint)
- On benchmark shutdown/interruption
- On error (failure checkpoint)

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Model        │────>│ Update      │────>│ Checkpoint  │
│ Completes    │     │ Progress    │     │ Trigger?    │
└──────────────┘     │ Counter     │     └──────┬───────┘
                            │               Yes │   │ No
                            │                   │   └───> Continue
                            ▼                   ▼
                     ┌──────────────┐    ┌──────────────┐
                     │ Save State  │    │ Write       │
                     │ Snapshot    │    │ Checkpoint  │
                     └──────────────┘    │ File        │
                                        └──────┬───────┘
                                               │
                    ┌──────────────────────────┤
                    │                          │
                    ▼                          ▼
             ┌──────────────┐          ┌──────────────┐
             │ On Resume   │          │ Prune Old   │
             │ Load Last   │          │ Checkpoints │
             │ Checkpoint  │          └──────────────┘
             └──────────────┘
```

## Step-by-Step

### Step 1: Initialize Progress State
- **Action**: Create initial progress tracking state (total models, completed, failed, in_progress, start_time)
- **Schema Properties Used**: `checkpointing.enabled`, `checkpointing.triggers`
- **Input**: Benchmark manifest (total model count)
- **Output**: Initial progress state object
- **Error Handling**: If state init fails, abort benchmark (can't track progress = unsafe to run)
- **New Schema Requirements**: None

### Step 2: Update Progress Per Model
- **Action**: When a model completes, update progress state (completed+1, record model result)
- **Schema Properties Used**: `benchmark_manifest.models[].status`
- **Input**: Model completion result
- **Output**: Updated progress state
- **Error Handling**: If state update fails, continue but log warning
- **New Schema Requirements**: None

### Step 3: Check Checkpoint Triggers
- **Action**: Evaluate if a checkpoint should be saved (interval reached, model completed, failure occurred)
- **Schema Properties Used**: `checkpointing.triggers.interval`, `checkpointing.triggers.on_failure`, `checkpointing.triggers.on_timeout`
- **Input**: Current progress, last checkpoint time, trigger config
- **Output**: should_checkpoint: true/false
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 4: Save Checkpoint
- **Action**: Write full benchmark state to a checkpoint file (models completed, current model, queue state, timing)
- **Schema Properties Used**: `state_management.checkpoint_directory`, `state_management.versioning`
- **Input**: Current progress state, execution context
- **Output**: Checkpoint file written to disk
- **Error Handling**: On write failure, retry once. If still fails, log critical warning.
- **New Schema Requirements**: None

### Step 5: Resume from Checkpoint
- **Action**: On benchmark restart, detect existing checkpoint and offer to resume
- **Schema Properties Used**: `state_management.enabled`, `state_management.integrity_validation`
- **Input**: Checkpoint file path
- **Output**: Restored progress state, remaining model queue
- **Error Handling**: If checkpoint is corrupt, start fresh (warn user). If checkpoint is stale (>7 days), warn user.
- **New Schema Requirements**: `benchmark_execution.max_checkpoint_age_days: int` (default: 7)

### Step 6: Progress Display
- **Action**: Display real-time progress to operator (console output: 45/100 models complete, ETA, current model)
- **Schema Properties Used**: N/A (UI concern)
- **Input**: Current progress state
- **Output**: Console/log output with progress info
- **Error Handling**: N/A
- **New Schema Requirements**: None

### Step 7: Prune Old Checkpoints
- **Action**: Remove checkpoint files older than retention period to save disk space
- **Schema Properties Used**: N/A
- **Input**: Checkpoint directory, retention policy
- **Output**: Old checkpoints deleted
- **Error Handling**: If deletion fails, log warning
- **New Schema Requirements**: `benchmark_execution.checkpoint_retention_count: int` (default: 5)

## Post-conditions
- Progress tracked continuously throughout benchmark
- Checkpoints saved at configured triggers
- Benchmark can be resumed from last checkpoint
- Old checkpoints pruned automatically

## Edge Cases
- Checkpoint file written during model load (model left in "loading" state — skip on resume)
- Multiple checkpoint files exist (use most recent)
- Checkpoint from different benchmark session (detect via session ID, don't offer resume)
- Disk full during checkpoint write (log critical, continue benchmark without checkpoint)
- Very fast models (checkpoint overhead may exceed model execution time)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Checkpoint enabled flag | Yes (`checkpointing.enabled`) | None |
| Checkpoint triggers | Yes (`checkpointing.triggers`) | None |
| Checkpoint directory | Yes (`state_management.checkpoint_directory`) | None |
| State versioning | Yes (`state_management.versioning`) | None |
| Integrity validation | Yes (`state_management.integrity_validation`) | None |
| Auto-save interval | Yes (`state_management.auto_save_interval_secs`) | None |
| Resume detection | Partially | Session-based resume validation |
| Checkpoint retention | No | `checkpoint_retention_count` |

## Dependencies
- UF06 (Orchestration) — progress events originate here
- UF07 (Fault Tolerance) — failure triggers checkpoints
- UF11 (Concurrent Execution) — per-worker progress tracking
