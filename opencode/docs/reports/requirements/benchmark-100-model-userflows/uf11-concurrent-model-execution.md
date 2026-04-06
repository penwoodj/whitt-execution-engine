# UF11: Concurrent Model Execution

## Overview
Manages parallel execution of benchmark workflows across multiple GPU-equipped machines or multiple models on a single machine, with resource-aware scheduling, queue management, and distributed coordination.

## User Story
As a benchmark operator, I want to run benchmarks on multiple models simultaneously across my available hardware so that I can complete a 100-model benchmark in hours instead of days.

## Pre-conditions
- Multiple models installed (UF02)
- Hardware inventory known (GPU count, VRAM per GPU, RAM)
- Concurrency limits configured
- Network connectivity between nodes (if distributed)

## Trigger
Benchmark session started with parallel execution strategy, or workload distribution to multiple workers.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Hardware     │────>│ Calculate    │────>│ Create      │
│ Inventory    │     │ Max         │     │ Execution   │
│              │     │ Concurrency │     │ Queue       │
└──────────────┘     └──────────────┘     └──────┬───────┘
                                                  │
                                                  ▼
                                           ┌──────────────┐
                                           │ Schedule     │
                                           │ Models to    │
                                           │ Workers      │
                                           └──────┬───────┘
                                                  │
                     ┌────────────┬───────────────┤
                     │           │               │
                     ▼           ▼               ▼
              ┌──────────┐ ┌──────────┐   ┌──────────┐
              │ Worker 1 │ │ Worker 2 │   │ Worker N │
              │ GPU 0   │ │ GPU 1   │   │ GPU M   │
              │ Model A │ │ Model B │   │ Model C │
              └────┬─────┘ └────┬─────┘   └────┬─────┘
                   │            │              │
                   ▼            ▼              ▼
              ┌────────────┐ ┌────────────┐ ┌────────────┐
              │ Complete   │ │ Complete   │ │ Complete   │
              │ → Next     │ │ → Next     │ │ → Next     │
              │ Model D    │ │ Model E    │ │ Model F    │
              └────────────┘ └────────────┘ └────────────┘
```

## Step-by-Step

### Step 1: Hardware Inventory
- **Action**: Detect available GPUs, VRAM per GPU, CPU cores, and RAM
- **Schema Properties Used**: `models.host.type` (multi-host support)
- **Input**: System resource detection
- **Output**: Hardware inventory (gpu_count, vram_per_gpu, total_ram)
- **Error Handling**: If hardware detection fails, default to 1 GPU with conservative VRAM
- **New Schema Requirements**: `benchmark_execution.hardware_inventory` — detected hardware profile

### Step 2: Calculate Max Concurrency
- **Action**: Determine how many models can run simultaneously based on GPU count and VRAM
- **Schema Properties Used**: `workflow_execution_strategy.parallel.max_models`, `workflow_execution_strategy.concurrency_limits.model_loading`
- **Input**: Hardware inventory, average model size
- **Output**: max_concurrent_models (e.g., 2 GPUs × 1 model = 2 concurrent)
- **Error Handling**: If calculation impossible, default to 1 (serial execution)
- **New Schema Requirements**: None — existing parallel config covers this

### Step 3: Create Execution Queue
- **Action**: Build a priority queue of models to benchmark, ordered by size (smallest first for throughput) or priority
- **Schema Properties Used**: `benchmark_manifest.models[]`
- **Input**: Model list from manifest
- **Output**: Ordered execution queue
- **Error Handling**: N/A
- **New Schema Requirements**: `benchmark_execution.queue_order: smallest_first | largest_first | manifest_order | priority`

### Step 4: Distribute Work to Workers
- **Action**: Assign models from the queue to available workers/GPUs
- **Schema Properties Used**: `workflow_execution_strategy.parallel`
- **Input**: Queue, available workers
- **Output**: Worker assignments
- **Error Handling**: If a worker fails to accept assignment, reassign to another worker
- **New Schema Requirements**: None

### Step 5: Execute in Parallel
- **Action**: Each worker loads its assigned model, runs the 8-step workflow (UF06), writes detail.md (UF09)
- **Schema Properties Used**: All workflow execution properties per model
- **Input**: Worker assignment
- **Output**: Model benchmark result per worker
- **Error Handling**: Worker failure: reassign model to next available worker (UF07 fault tolerance)
- **New Schema Requirements**: None

### Step 6: Coordinate Completion
- **Action**: Track worker completion, assign next model from queue when worker finishes
- **Schema Properties Used**: `workflow_execution_strategy.parallel`
- **Input**: Worker completion signals
- **Output**: Updated queue, new assignments
- **Error Handling**: Dead worker detection: timeout and reassign remaining models
- **New Schema Requirements**: `benchmark_execution.worker_timeout_seconds: int`

### Step 7: Merge Results
- **Action**: Collect results from all workers into a unified output directory
- **Schema Properties Used**: `workspace.output_path`
- **Input**: Per-worker result directories
- **Output**: Unified results directory with all detail.md files
- **Error Handling**: Merge conflicts (duplicate model IDs) resolved by keeping latest run
- **New Schema Requirements**: None

## Post-conditions
- All models benchmarked across available hardware
- Results merged into single output directory
- Worker utilization tracked for report
- No resource conflicts or deadlocks

## Edge Cases
- Single GPU machine (serial execution with `max_models: 1`)
- Multi-node setup with different GPUs (different VRAM per node — different model assignments)
- Worker crash mid-benchmark (checkpoint recovery via UF13, reassign remaining)
- Network partition between nodes (isolated nodes continue independently, merge on reconnection)
- Mixed model sizes (some 3B, some 70B — schedule carefully to avoid starving large models)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Max concurrent models | Yes (`parallel.max_models`) | None |
| Concurrency limits | Yes (`concurrency_limits.model_loading`) | None |
| Load/unload strategy | Yes (`workflow_execution_strategy.load_unload`) | `adaptive` strategy for multi-GPU |
| Worker coordination | No | Distributed execution protocol |
| Queue ordering | No | `benchmark_execution.queue_order` |
| Hardware inventory | No | `benchmark_execution.hardware_inventory` |
| Worker timeout | No | `benchmark_execution.worker_timeout_seconds` |

## Dependencies
- UF01 (Discovery) — model list for queue
- UF02 (Installation) — models installed
- UF04 (Loading) — per-worker model loading
- UF06 (Orchestration) — per-worker workflow execution
- UF07 (Fault Tolerance) — worker failure recovery
- UF09 (detail.md) — per-worker output generation
- UF13 (Checkpointing) — crash recovery per worker
