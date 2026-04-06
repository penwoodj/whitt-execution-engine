# UF07: Fault Tolerance and Error Recovery

## Overview
Defines the error handling strategy for the benchmark system: retry policies, escalation paths, circuit breakers, and graceful degradation to ensure 100-model benchmarks can survive individual model failures without aborting the entire run.

## User Story
As a benchmark operator, I want the system to automatically recover from model failures, timeouts, and crashes so that a single bad model doesn't kill my 48-hour benchmark run.

## Pre-conditions
- Benchmark session in progress
- Retry and error handling configuration defined
- Logging infrastructure operational

## Trigger
Any error occurs during model loading, step execution, or output capture.

## Flow Diagram

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Error        │────>│ Classify    │────>│ Retry       │
│ Detected     │     │ Error Type  │     │ Policy      │
└──────────────┘     └──────┬───────┘     └──────┬───────┘
                            │                     │
                            ▼                     ▼
                     ┌──────────────┐     ┌──────────────┐
                     │ Transient   │     │ Max Retries │
                     │ or          │     │ Exceeded?   │
                     │ Permanent?  │     │             │
                     └──────┬───────┘     └──────┬───────┘
                            │                     │
                     Yes ───┤               No ───┤──── Yes
                     │      ▼               │     ▼
                     │  ┌──────────┐    ┌──────────┐  ┌──────────┐
                     │  │ Retry    │    │ Retry    │  │ Escalate │
                     │  │ Action   │    │ With     │  │ to Next │
                     │  │          │    │ Backoff  │  │ Recovery │
                     │  └──────────┘    └──────────┘  └──────────┘
                     │                                     │
                     │ No                                  ▼
                     │                              ┌──────────────┐
                     ▼                              │ Skip Model / │
              ┌──────────┐                           │ Skip Step /  │
              │ Record   │                           │ Abort Run   │
              │ Failure  │                           └──────────────┘
              │ Continue │
              └──────────┘
```

## Error Classification

| Error Type | Category | Retry? | Recovery Action |
|-----------|----------|--------|----------------|
| Model OOM on load | Transient | Yes | Quantization downgrade (UF03) |
| Provider timeout | Transient | Yes (3x) | Backoff + retry |
| Model corrupt/garbage | Permanent | No | Skip model, log failure |
| Network error (download) | Transient | Yes (5x) | Exponential backoff |
| Disk full | Permanent (until fixed) | No | Pause benchmark, alert operator |
| Provider crash | Transient | Yes (2x) | Restart provider, retry |
| YAML validation error | Permanent | No | Skip workflow, log error |
| Context window exceeded | Permanent | No | Truncate context, continue step |
| Step output empty | Permanent | No | Record "empty", continue to next step |

## Step-by-Step

### Step 1: Detect Error
- **Action**: Catch error from model loading, step execution, or output writing
- **Schema Properties Used**: `retry.condition` (matches error type)
- **Input**: Error object with type, message, stack trace
- **Output**: Error classified and routed to handler
- **Error Handling**: N/A (this IS error handling)
- **New Schema Requirements**: `retry.condition` should support regex/matching on error types

### Step 2: Classify Error
- **Action**: Determine if error is transient (retryable) or permanent (skip)
- **Schema Properties Used**: N/A (classification logic)
- **Input**: Error object
- **Output**: Error category (transient/permanent), recommended action
- **Error Handling**: If classification fails, treat as transient with single retry
- **New Schema Requirements**: None

### Step 3: Check Retry Budget
- **Action**: Check if retries remain for this error type at this level
- **Schema Properties Used**: `retry.max_attempts`, `retry.level`
- **Input**: Error category, retry history for current operation
- **Output**: can_retry: true/false, remaining_attempts
- **Error Handling**: N/A
- **New Schema Requirements**: None — existing `retry.max_attempts` covers this

### Step 4: Apply Retry with Backoff
- **Action**: Wait for backoff period, then retry the failed operation
- **Schema Properties Used**: `retry.backoff` (exponential/linear/fixed)
- **Input**: Backoff strategy, attempt number
- **Output**: Retry result (success/failure)
- **Error Handling**: If retry also fails, go back to Step 3
- **New Schema Requirements**: None — existing `retry.backoff` covers this

### Step 5: Escalate Recovery (if retries exhausted)
- **Action**: Determine escalation path based on error level and type
- **Schema Properties Used**: `retry.level` (workflow_unload | step_restart | prompt_restart | tools_retry)
- **Input**: Exhausted error, current execution state
- **Output**: Recovery action to execute
- **Error Handling**: N/A
- **New Schema Requirements**: `retry.level: model_skip` — skip to next model entirely

### Step 6: Record Failure and Continue
- **Action**: Log the failure with full context, update model's benchmark status
- **Schema Properties Used**: `logging.scopes`, `benchmark_manifest.models[].status`
- **Input**: Error details, model ID, step ID
- **Output**: Failure record in manifest and logs
- **Error Handling**: If logging fails, write to fallback log file
- **New Schema Requirements**: `benchmark_manifest.models[].status: success|partial|failed|skipped`

### Step 7: Circuit Breaker Check
- **Action**: If too many consecutive models fail, trigger circuit breaker to pause the benchmark
- **Schema Properties Used**: N/A
- **Input**: Recent failure history
- **Output**: Continue or pause benchmark
- **Error Handling**: N/A
- **New Schema Requirements**: `benchmark_execution.circuit_breaker.consecutive_failures_threshold: int` (default: 10)

## Post-conditions
- Error has been handled (retried, skipped, or escalated)
- Failure recorded in manifest and logs
- Benchmark continues to next model/step unless circuit breaker triggered
- No unhandled exceptions crash the benchmark process

## Edge Cases
- All models fail at the same step (systemic issue, not model issue — circuit breaker)
- Provider crashes and doesn't restart (permanent failure, need human intervention)
- Error occurs during error logging (write to fallback stderr)
- Retry causes cascading failures (circuit breaker at retry level)
- Model partially succeeds (some steps done, some failed) — record as "partial"

## Schema Coverage

| Feature | Covered by Schema | Needs Addition |
|---------|-------------------|----------------|
| Retry max attempts | Yes (`retry.max_attempts`) | None |
| Retry backoff | Yes (`retry.backoff`) | None |
| Retry level escalation | Yes (`retry.level`) | `model_skip` level |
| Error classification | No | Error taxonomy matching |
| Circuit breaker | No | `benchmark_execution.circuit_breaker` |
| Per-model failure status | No | `benchmark_manifest.models[].status` |
| Failure history tracking | No | `benchmark_execution.failure_log` |

## Dependencies
- UF06 (Orchestration) — errors originate from workflow execution
- UF03 (Quantization) — OOM recovery path
- UF04 (Loading) — load failure recovery path
- UF13 (Checkpointing) — recovery from total process crash
