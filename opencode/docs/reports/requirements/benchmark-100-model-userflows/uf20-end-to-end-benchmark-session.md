# UF20: End-to-End Benchmark Session

## Overview
The complete end-to-end flow from benchmark initialization through final teardown, orchestrating all 19 prior userflows into a single cohesive benchmark session against 100+ local LLM models.

## User Story
As a benchmark operator, I want to run a single command that discovers models, installs missing ones, benchmarks each against the 8-prompt workflow, scores quality, generates reports, and cleans up — all fault-tolerantly — so that I can get comprehensive model comparison data with minimal manual intervention.

## Pre-conditions
- At least one LLM provider running (LM Studio, Ollama, or llama.cpp)
- Sufficient disk space for models and outputs
- Benchmark configuration file present (UF16)
- System meets minimum hardware requirements (8GB+ VRAM recommended)

## Trigger
Operator runs `benchmark run` command (or equivalent).

## Flow Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        END-TO-END BENCHMARK SESSION                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Phase 1: INITIALIZATION                                                │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │ UF16     │  │ UF15     │  │ UF13     │  │ UF19     │               │
│  │ Config   │─>│ Resource │─>│ Init     │─>│ Load     │               │
│  │ Load     │  │ Monitor  │  │ Progress │  │ Registry │               │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘               │
│                                                                         │
│  Phase 2: MODEL PREPARATION                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │ UF01     │  │ UF02     │  │ UF12     │  │ UF11     │               │
│  │ Discover │─>│ Install  │─>│ Provider │─>│ Schedule │               │
│  │ Models   │  │ Missing  │  │ Setup    │  │ Queue    │               │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘               │
│                                                                         │
│  Phase 3: BENCHMARK EXECUTION (per model, repeated N times)             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │ UF04     │  │ UF03     │  │ UF05+06  │  │ UF08     │               │
│  │ Load     │─>│ Quant    │─>│ Execute  │─>│ Capture  │               │
│  │ Model    │  │ Fallback │  │ 8 Steps  │  │ History  │               │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘               │
│        │                                        │                      │
│        │            ┌──────────┐  ┌──────────┐   │                      │
│        └───────────>│ UF07     │  │ UF14     │<──┘                      │
│                     │ Error    │  │ Score    │                           │
│                     │ Recovery │  │ Quality  │                           │
│                     └──────────┘  └──────────┘                           │
│                                  │                                      │
│                                  ▼                                      │
│                           ┌──────────┐                                  │
│                           │ UF09     │                                  │
│                           │ Write    │                                  │
│                           │ detail.md│                                  │
│                           └──────────┘                                  │
│                                                                         │
│  Phase 4: POST-BENCHMARK                                                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │ UF10     │  │ UF17     │  │ UF19     │  │ UF18     │               │
│  │ Aggregate│─>│ Report   │─>│ Update   │─>│ Cleanup  │               │
│  │ Results  │  │ Generate │  │ Registry │  │ & Exit   │               │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘               │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## Step-by-Step

### Step 1: Load Configuration (UF16)
- **Action**: Load and validate benchmark configuration, apply defaults and overrides
- **Userflows**: UF16
- **Output**: Frozen configuration object
- **Failure**: Abort with config error details

### Step 2: Initialize Monitoring (UF15)
- **Action**: Start resource monitoring probes
- **Userflows**: UF15 (Steps 1-2)
- **Output**: Active monitors
- **Failure**: Continue without monitoring (log warning)

### Step 3: Initialize Progress Tracking (UF13)
- **Action**: Create progress state, check for resumable checkpoint
- **Userflows**: UF13 (Steps 1, 5)
- **Output**: Progress tracker, possibly restored from checkpoint
- **Failure**: Abort (can't track progress = unsafe)

### Step 4: Load Model Registry (UF19)
- **Action**: Load existing model registry for historical comparison
- **Userflows**: UF19 (Step 1)
- **Output**: Registry data
- **Failure**: Create new empty registry

### Step 5: Discover Models (UF01)
- **Action**: Scan providers for available models, build manifest
- **Userflows**: UF01 (Steps 1-5)
- **Output**: Benchmark manifest
- **Failure**: Abort if no models found

### Step 6: Install Missing Models (UF02)
- **Action**: Download and install models not yet present
- **Userflows**: UF02 (Steps 1-6)
- **Output**: All models installed or flagged unavailable
- **Failure**: Skip unavailable models, continue with available

### Step 7: Setup Provider Adapters (UF12)
- **Action**: Initialize provider abstraction layer
- **Userflows**: UF12 (Steps 1-2)
- **Output**: Provider adapter pool
- **Failure**: Abort if no providers available

### Step 8: Schedule Execution Queue (UF11)
- **Action**: Create execution queue, assign workers
- **Userflows**: UF11 (Steps 1-4)
- **Output**: Execution queue, worker assignments
- **Failure**: Fall back to serial execution

### Step 9: For Each Model in Queue (UF04 → UF03 → UF06 → UF08 → UF14 → UF09 → UF07)
- **Action**: Execute the per-model benchmark loop:
  1. Load model (UF04) — if OOM, trigger quantization fallback (UF03)
  2. Execute 8-step workflow (UF05+UF06)
  3. Capture chat/tool history (UF08)
  4. Score quality (UF14)
  5. Write detail.md (UF09)
  6. Handle errors (UF07)
  7. Unload model (UF04)
  8. Update progress + checkpoint (UF13)
  9. Adapt to resource changes (UF15)
- **Output**: detail.md per model, scores, progress updated
- **Failure**: Skip model, continue to next (UF07)

### Step 10: Aggregate Results (UF10)
- **Action**: Collect all detail.md files into unified dataset
- **Userflows**: UF10 (Steps 1-6)
- **Output**: Aggregated results
- **Failure**: Generate partial report with available data

### Step 11: Generate Report (UF17)
- **Action**: Create comprehensive benchmark report
- **Userflows**: UF17 (Steps 1-8)
- **Output**: benchmark-report.md
- **Failure**: Log error, attempt simplified report

### Step 12: Update Registry (UF19)
- **Action**: Update model registry with latest results
- **Userflows**: UF19 (Steps 2-6)
- **Output**: Updated registry
- **Failure**: Log warning, registry not critical

### Step 13: Cleanup and Teardown (UF18)
- **Action**: Final checkpoint, unload models, stop monitors, close connections
- **Userflows**: UF18 (Steps 1-7)
- **Output**: Clean system state
- **Failure**: Force cleanup

## Session Statistics Collected

| Metric | Source | Reported In |
|--------|--------|-------------|
| Total models attempted | UF01/UF11 | Report |
| Models succeeded | UF13 | Report |
| Models failed/skipped | UF07/UF13 | Report |
| Total wall time | UF13 | Report |
| Average time per model | UF13 | Report |
| Quantization downgrades | UF03 | Report |
| Peak VRAM usage | UF15 | Report |
| Total tokens consumed | UF06 | Report |
| Average quality score | UF14 | Report |
| Provider crashes/restarts | UF12 | Report |

## Post-conditions
- Benchmark complete (or checkpoint saved if interrupted)
- detail.md exists for every successfully benchmarked model
- Comprehensive report generated
- Model registry updated
- System cleaned up, resources freed
- Checkpoint saved for potential future comparison runs

## Edge Cases
- Zero models discovered (abort with guidance)
- All models fail at load (report shows 0% success, analyze failure patterns)
- Power loss mid-benchmark (resume from last checkpoint on restart)
- Configuration changed mid-benchmark (frozen config prevents this)
- New model added to provider during benchmark (not discovered — next run only)
- Provider upgraded mid-benchmark (adapter may break — health check detects)
- Benchmark operator cancels via Ctrl+C (graceful shutdown via UF18)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Full workflow execution | Yes (`agentic_workflow.steps`) | None |
| Model management | Yes (`models.*`) | Benchmark-specific extensions |
| Resource management | Yes (`workflow_execution_strategy.*`) | Monitoring/adaptation |
| Error handling | Yes (`retry.*`) | Circuit breaker, error taxonomy |
| State management | Yes (`checkpointing.*`, `state_management.*`) | Resume detection |
| Output management | Yes (`output.*`) | Report template, registry |
| Configuration | Yes (all root properties) | Benchmark config overlay |

## Dependencies
- All UF01 through UF19 — this is the orchestration of everything
- Unified workflow schema — provides the structural foundation
