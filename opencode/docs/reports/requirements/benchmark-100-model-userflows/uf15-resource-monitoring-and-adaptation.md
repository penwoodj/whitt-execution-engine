# UF15: Resource Monitoring and Adaptation

## Overview
Continuously monitors system resources (GPU VRAM, RAM, CPU, disk, temperature) during benchmark execution and adapts behavior dynamically: throttle concurrency, trigger quantization downgrades, pause on thermal throttling.

## User Story
As a benchmark operator, I want the system to monitor hardware resources in real-time and adapt its behavior so that my benchmark doesn't crash my machine from overheating or running out of memory.

## Pre-conditions
- Benchmark session in progress
- System monitoring tools available (nvidia-smi, /proc/meminfo, etc.)
- Resource thresholds configured

## Trigger
- Periodic monitoring interval (e.g., every 10 seconds)
- Resource threshold breach
- Model load/unload events

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Monitor      │────>│ Check        │────>│ All         │
│ Resources    │     │ Thresholds   │     │ Within      │
│ (VRAM/RAM/   │     │              │     │ Limits?     │
│  CPU/Temp)   │     │              │     │             │
└──────────────┘     └──────┬───────┘     └──────┬───────┘
                            │                     │
                     Breach │               OK ───┤
                            │                     │
                            ▼                     ▼
                     ┌──────────────┐     ┌──────────────┐
                     │ Classify    │     │ Continue    │
                     │ Breach      │     │ Execution   │
                     └──────┬───────┘     └──────────────┘
                            │
                     ┌──────┴───────┐
                     │              │
                     ▼              ▼
              ┌──────────┐   ┌──────────┐
              │ Adapt:   │   │ Adapt:   │
              │ Reduce   │   │ Pause &  │
              │ Concurr. │   │ Alert    │
              └──────────┘   └──────────┘
```

## Resource Thresholds

| Resource | Warning | Critical | Action |
|----------|---------|----------|--------|
| GPU VRAM | >85% used | >95% used | Warning → reduce concurrency; Critical → pause loads |
| System RAM | >80% used | >90% used | Warning → reduce concurrency; Critical → pause benchmark |
| GPU Temp | >80°C | >90°C | Warning → throttle; Critical → pause until cool |
| CPU Usage | >90% sustained | >95% sustained | Warning → reduce parallel; Critical → serial only |
| Disk Space | <10GB free | <2GB free | Warning → alert; Critical → pause benchmark |
| GPU Utilization | <10% | <5% | Possible stuck model — investigate |

## Step-by-Step

### Step 1: Initialize Resource Monitors
- **Action**: Set up monitoring probes for GPU (nvidia-smi/vulkaninfo), RAM, CPU, disk, temperature
- **Schema Properties Used**: `workflow_execution_strategy.memory.max_allowed`, `workflow_execution_strategy.memory.pressure_handling`
- **Input**: Hardware inventory from UF11
- **Output**: Active monitoring probes
- **Error Handling**: If a probe fails to initialize, log warning and disable that metric (don't abort)
- **New Schema Requirements**: `benchmark_execution.monitoring.enabled: bool`, `benchmark_execution.monitoring.interval_seconds: int`

### Step 2: Collect Resource Metrics
- **Action**: Periodically poll all monitoring probes and record current values
- **Schema Properties Used**: `metrics.collect`
- **Input**: Active probes
- **Output**: Current resource state (vram_used, ram_used, cpu_pct, gpu_temp_c, disk_free_gb)
- **Error Handling**: If poll fails, use last known value. If 3 consecutive failures, disable probe.
- **New Schema Requirements**: None

### Step 3: Evaluate Thresholds
- **Action**: Compare current resource state against warning/critical thresholds
- **Schema Properties Used**: `models.<id>.max_allowed.vram`, `models.<id>.min_allowed.ram`
- **Input**: Current state, threshold config
- **Output**: Breach list (resource, severity, current_value, threshold)
- **Error Handling**: N/A
- **New Schema Requirements**: `benchmark_execution.thresholds.vram_warning_pct: float`, `benchmark_execution.thresholds.vram_critical_pct: float`

### Step 4: Apply Adaptation Strategy
- **Action**: Based on breach severity, adapt execution strategy
- **Schema Properties Used**: `workflow_execution_strategy.memory.pressure_handling.on_oom`, `workflow_execution_strategy.parallel.max_models`
- **Input**: Breach list, current execution strategy
- **Output**: Adapted strategy (reduced concurrency, paused loads, etc.)
- **Error Handling**: If adaptation fails, fall back to safest option (pause all loading)
- **New Schema Requirements**: `benchmark_execution.adaptation.auto_reduce_concurrency: bool`

### Step 5: Resume After Recovery
- **Action**: When resources return to safe levels, gradually restore execution strategy
- **Schema Properties Used**: Same as Step 4
- **Input**: Current resource state, adapted strategy
- **Output**: Restored (or partially restored) strategy
- **Error Handling**: Don't immediately restore to max — ramp up gradually
- **New Schema Requirements**: `benchmark_execution.adaptation.restore_grace_period_seconds: int` (default: 60)

### Step 6: Log Resource Events
- **Action**: Record resource breaches, adaptations, and recoveries for the benchmark report
- **Schema Properties Used**: `logging.scopes.performance_metrics`
- **Input**: Resource events
- **Output**: Resource event log
- **Error Handling**: N/A
- **New Schema Requirements**: None

## Post-conditions
- System resources monitored throughout benchmark
- Adaptations applied when thresholds breached
- Resource events logged for analysis
- No crashes due to resource exhaustion

## Edge Cases
- Monitoring probe returns negative values (bug in probe — disable and use estimates)
- GPU driver crash (resource monitoring also fails — use last known state)
- Another process suddenly claims GPU memory (detect via monitoring, adapt immediately)
- Thermal throttling causes model to run very slowly (detect via timing anomalies, not just temp)
- Swap usage spiking (RAM pressure — reduce concurrency aggressively)

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Memory limits | Yes (`max_allowed.vram`, `min_allowed.ram`) | None |
| OOM handling | Yes (`pressure_handling.on_oom`) | None |
| Memory pressure config | Yes (`memory.pressure_handling`) | None |
| Resource monitoring | Partially | `monitoring.enabled`, `monitoring.interval_seconds` |
| Threshold configuration | No | `benchmark_execution.thresholds.*` |
| Auto-adaptation | No | `benchmark_execution.adaptation.*` |
| Restore grace period | No | `adaptation.restore_grace_period_seconds` |

## Dependencies
- UF04 (Loading) — affected by resource adaptations
- UF06 (Orchestration) — execution strategy adapts based on resources
- UF07 (Fault Tolerance) — resource exhaustion is a fault type
- UF11 (Concurrent Execution) — concurrency reduced on resource pressure
