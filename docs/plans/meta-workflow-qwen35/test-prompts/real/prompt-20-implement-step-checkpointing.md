# Task: Implement Step Checkpointing

## Objective
If a 24-step workflow crashes at step 20, the entire run must restart from step 1. This wastes hours of compute.

## Requirements
1. Read `src/agent/persistence.rs` to understand existing checkpoint logic
2. Read `src/benchmark/runner.rs` to understand step execution flow
3. Implement a `CheckpointManager` that:
   - After each successful step, saves state to `./checkpoints/<workflow_id>/step_<N>.json`
   - State includes: step ID, output, bookmarks, timestamp
   - On workflow start: checks for existing checkpoints
   - If checkpoint exists: skips steps that already completed, resumes from last incomplete
4. Add CLI flag `--resume <checkpoint-dir>` to resume from specific checkpoint
5. Write the complete implementation

## Output
Complete Rust implementation including:
- `CheckpointManager` struct
- Save/load checkpoint functions
- Step skip logic for resumed workflows
- CLI integration
