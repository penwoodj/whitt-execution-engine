# 100-Model Benchmark System: MVP Summary Report

## Executive Summary

This report defines the MVP requirements for a fault-tolerant benchmark system that runs an 8-prompt chain workflow across 100+ local LLM models. The system must handle model discovery, adaptive KV cache quantization, parallel execution, quality scoring, exhaustive per-model output, and comprehensive reporting — all without human intervention across what may be a 48-hour benchmark run.

**20 userflows** were analyzed across 4 phases. This report categorizes them by MVP priority, identifies schema gaps, and quantifies what's in scope vs. deferred.

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    BENCHMARK SESSION (UF20)                          │
│                                                                     │
│  ┌───────────┐  ┌────────────┐  ┌────────────┐  ┌─────────────┐  │
│  │ Config    │  │ Model      │  │ Execution  │  │ Reporting   │  │
│  │ (UF16)    │  │ Management │  │ Engine     │  │ & Analysis  │  │
│  │           │  │            │  │            │  │             │  │
│  │ - YAML    │  │ Discover   │  │ Load → Run │  │ Aggregate   │  │
│  │ - Vars    │  │ Install    │  │ → Score    │  │ Rank        │  │
│  │ - Schema  │  │ Quantize   │  │ → Capture  │  │ Report      │  │
│  └───────────┘  └────────────┘  └────────────┘  └─────────────┘  │
│                                                                     │
│  Cross-cutting: Fault Tolerance (UF07) │ Resource Monitor (UF15)   │
│                 Progress/Checkpoint (UF13) │ Provider Abstract (UF12)│
└─────────────────────────────────────────────────────────────────────┘
```

---

## MVP Phases

### Phase 1: Core Execution (MUST HAVE — ~60% of effort)

The bare minimum to run a benchmark against N models sequentially.

| # | Userflow | Purpose | Effort | Schema Coverage |
|---|----------|---------|--------|----------------|
| UF16 | Configuration Management | Load/validate/layer config | M | 85% covered |
| UF12 | Model Provider Abstraction | Unified API for Ollama/LM Studio/llama.cpp | L | 60% covered |
| UF01 | Model Discovery & Registration | Scan providers, build manifest | S | 40% covered |
| UF05 | 8-Prompt Workflow Definition | Define benchmark steps | S | 95% covered |
| UF06 | Workflow Execution Orchestration | Load→execute 8 steps→unload per model | L | 75% covered |
| UF07 | Fault Tolerance & Error Recovery | Retry, classify errors, skip failures | L | 50% covered |
| UF09 | detail.md Generation | Exhaustive per-model output file | M | 70% covered |
| UF18 | Cleanup & Teardown | Unload models, free resources | S | 90% covered |

**Phase 1 delivers**: A working benchmark that can discover models, run 8 prompts against each, capture outputs, and produce detail.md files — with basic error recovery.

### Phase 2: Quality & Scale (SHOULD HAVE — ~25% of effort)

Adds scoring, parallelism, and resource awareness needed for 100-model runs.

| # | Userflow | Purpose | Effort | Schema Coverage |
|---|----------|---------|--------|----------------|
| UF03 | Adaptive KV Cache Quantization | Q8→Q2 fallback chain to fit VRAM | M | 30% covered |
| UF04 | Model Loading & Resource Allocation | GPU locks, concurrent limits, warm-up | M | 60% covered |
| UF08 | Chat & Tool History Capture | Full conversation logging per step | S | 75% covered |
| UF14 | Validation & Quality Scoring | Automated per-step scoring (0.0-1.0) | L | 25% covered |
| UF11 | Concurrent Model Execution | Multi-GPU scheduling, worker pool | L | 35% covered |
| UF15 | Resource Monitoring & Adaptation | VRAM/RAM/CPU/temp tracking + adaptation | M | 40% covered |

**Phase 2 delivers**: Fair quality scores, adaptive memory management, and multi-GPU parallelism — making 100-model runs practical.

### Phase 3: Ops & Intelligence (NICE TO HAVE — ~15% of effort)

Polish features for production-grade benchmarking.

| # | Userflow | Purpose | Effort | Schema Coverage |
|---|----------|---------|--------|----------------|
| UF02 | Model Download & Installation | Auto-install from HuggingFace/Ollama registry | M | 10% covered |
| UF10 | Benchmark Results Aggregation | Cross-model comparison dataset | M | 15% covered |
| UF13 | Progress Tracking & Checkpointing | Resume after crash, progress display | M | 80% covered |
| UF17 | Benchmark Report Generation | Rankings, tables, analysis report | M | 30% covered |
| UF19 | Model Registry & Versioning | Historical tracking across runs | S | 5% covered |

**Phase 3 delivers**: Auto-installation, crash recovery, comprehensive reports, and historical tracking.

---

## Schema Gap Analysis

### What the Unified Workflow Schema Already Covers

| Area | Properties | Status |
|------|-----------|--------|
| Workflow steps | `agentic_workflow.steps`, `input_variables` | ✅ Complete |
| Model configuration | `models.<id>.*`, `models.host.type` | ✅ Complete |
| Retry logic | `retry.max_attempts`, `retry.backoff`, `retry.level` | ✅ Complete |
| Timeout control | `model_overrides.timeout.*` | ✅ Complete |
| Output capture | `output.save_to`, `output.format`, `output.file_output` | ✅ Complete |
| Logging hierarchy | `logging.scopes.*` (9 levels) | ✅ Complete |
| State management | `checkpointing.*`, `state_management.*` | ✅ Complete |
| Memory strategy | `workflow_execution_strategy.load_unload`, `memory.*` | ✅ Complete |
| Concurrency | `parallel.max_models`, `concurrency_limits.*` | ✅ Complete |
| Validation loops | `validation_loop.*` | ✅ Complete |

### What Needs to Be Added to the Schema (MVP Gaps)

| New Property | Location | Priority | Userflows Needing It |
|-------------|----------|----------|---------------------|
| `benchmark_manifest` root section | Root | P1 | UF01, UF02, UF06 |
| `benchmark_manifest.models[].status` | Under manifest | P1 | UF06, UF07 |
| `benchmark_manifest.models[].installed` | Under manifest | P1 | UF01, UF02 |
| `benchmark_manifest.models[].final_quantization` | Under manifest | P2 | UF03 |
| `models.<id>.source.url` | Under models | P3 | UF02 |
| `models.<id>.source.registry` | Under models | P3 | UF02 |
| `models.<id>.model_memory.kv_cache_quantization` (explicit levels) | Under models | P2 | UF03, UF04 |
| `models.<id>.model_memory.quantization_vram_table` | Under models | P2 | UF03 |
| `models.<id>.model_memory.max_quantization_downgrades` | Under models | P2 | UF03 |
| `model_overrides.warmup.enabled` | Under model_overrides | P2 | UF04 |
| `model_overrides.warmup.prompt` | Under model_overrides | P2 | UF04 |
| `retry.level: model_skip` | Under retry | P1 | UF07 |
| `benchmark_execution.circuit_breaker.*` | New root section | P2 | UF07 |
| `benchmark_execution.models[].metrics` | New root section | P1 | UF06 |
| `benchmark_execution.models[].state` | New root section | P1 | UF04, UF06 |
| `workspace.output_path` | New root section | P1 | UF09, UF10 |
| `workspace.model_storage_path` | New root section | P3 | UF02 |
| `benchmark_scoring.*` (weights, ground truth, validators) | New root section | P2 | UF14 |
| `benchmark_execution.monitoring.*` | New root section | P2 | UF15 |
| `benchmark_execution.thresholds.*` | New root section | P2 | UF15 |
| `benchmark_execution.adaptation.*` | New root section | P2 | UF15 |
| `benchmark_results.aggregated` | New root section | P3 | UF10 |
| `benchmark_results.rankings` | New root section | P3 | UF10 |
| `benchmark_execution.queue_order` | New root section | P3 | UF11 |
| `benchmark_execution.hardware_inventory` | New root section | P3 | UF11 |
| `benchmark_registry.*` | New root section | P3 | UF19 |
| `benchmark_execution.cleanup.*` | New root section | P3 | UF18 |

**Total new schema properties**: ~28 additions across 5 new root sections

---

## Implementation Complexity Breakdown

### By Effort (estimated person-days for a solo Rust developer)

| Effort | Userflows | Total Days |
|--------|-----------|------------|
| **Small (1-2 days)** | UF01, UF05, UF08, UF18, UF19 | 7 days |
| **Medium (3-5 days)** | UF02, UF03, UF04, UF09, UF10, UF13, UF15, UF16, UF17 | 33 days |
| **Large (5-10 days)** | UF06, UF07, UF11, UF12, UF14, UF20 | 40 days |
| **Total** | 20 userflows | **~80 person-days** |

### By Dependency Depth

```
Depth 0 (no deps):  UF16 (Config)
Depth 1:             UF01 (Discovery), UF12 (Provider), UF15 (Monitor)
Depth 2:             UF02 (Install), UF04 (Load), UF05 (Workflow), UF13 (Checkpoint), UF19 (Registry)
Depth 3:             UF03 (Quant), UF06 (Orchestrate), UF08 (History), UF11 (Concurrent), UF14 (Score)
Depth 4:             UF07 (Fault Tolerance), UF09 (detail.md), UF10 (Aggregate)
Depth 5:             UF17 (Report), UF18 (Cleanup)
Depth 6:             UF20 (End-to-End)
```

---

## What's In MVP vs. What's Deferred

### MVP (Phase 1 + Phase 2) = 14 userflows

| In MVP | What It Gives You |
|--------|------------------|
| UF16 Config | Single YAML config with defaults and env overrides |
| UF12 Provider | Works with Ollama, LM Studio, llama.cpp interchangeably |
| UF01 Discovery | Auto-finds installed models across providers |
| UF05 Workflow | 8 standardized prompt types for fair comparison |
| UF06 Orchestration | Sequential load→execute→unload cycle per model |
| UF07 Fault Tolerance | Never crashes on single model failure; retries with backoff |
| UF09 detail.md | Exhaustive per-model output with full history |
| UF18 Cleanup | Clean teardown, no orphan GPU processes |
| UF03 Quantization | Models that don't fit get lower KV cache quant automatically |
| UF04 Loading | GPU locks prevent concurrent overload |
| UF08 History | Full chat + tool call log per step |
| UF14 Scoring | Automated quality scores (0.0-1.0) per step |
| UF11 Concurrent | Multi-GPU parallel execution |
| UF15 Resource Monitor | Auto-throttle on VRAM/thermal pressure |

### Deferred (Phase 3) = 6 userflows

| Deferred | What You'll Miss | When to Add |
|----------|-----------------|-------------|
| UF02 Download/Install | Must manually install models before benchmarking | After MVP stable |
| UF10 Aggregation | No cross-model comparison table in data form | v2 |
| UF13 Checkpointing | Crash = restart from scratch | After first long run |
| UF17 Report | detail.md only, no summary report with rankings | v2 |
| UF19 Registry | No historical tracking across runs | v3 |
| UF20 E2E | Manual orchestration of the phases | Ties everything together in v2 |

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| Provider API changes break adapters | Medium | High | Abstract adapters, version detection |
| Model OOM causes cascade failures | Medium | High | Circuit breaker, serial fallback |
| 48-hour run crashes without checkpoint | High (Phase 1-2) | Critical | Promote UF13 to Phase 2 |
| Scoring quality disputes | Medium | Low | Document methodology, allow weight config |
| Schema additions break existing workflows | Low | Medium | New root sections, no modifications to existing |
| VRAM estimation inaccuracy | Medium | Medium | Conservative estimates, actual measurement |
| Disk space exhaustion (100 detail.md files) | Medium | Medium | Disk check pre-flight, compress old results |

---

## Recommended MVP Scope Adjustment

**Strongly consider promoting UF13 (Checkpointing) from Phase 3 to Phase 2.** A 48-hour benchmark run without checkpoint recovery is a single point of failure that could waste hours of compute. The schema already covers 80% of checkpointing needs (`checkpointing.*`, `state_management.*`), so the implementation effort is modest (~3-5 days).

---

## File Index

| File | Lines | Purpose |
|------|-------|---------|
| uf01-model-discovery-and-registration.md | 112 | Provider scan, manifest generation |
| uf02-model-download-and-installation.md | 107 | Auto-install from registries |
| uf03-adaptive-kv-cache-quantization.md | 169 | Q8→Q2 VRAM fallback chain |
| uf04-model-loading-and-resource-allocation.md | 116 | GPU locks, warm-up, lifecycle |
| uf05-eight-prompt-chain-workflow-definition.md | 144 | 8 capability test prompts |
| uf06-workflow-execution-orchestration.md | 140 | Load→execute→unload cycle |
| uf07-fault-tolerance-and-error-recovery.md | 143 | Error classification, retry, circuit breaker |
| uf08-chat-and-tool-history-capture.md | 137 | Full conversation logging |
| uf09-detail-md-generation.md | 149 | Exhaustive per-model output |
| uf10-benchmark-results-aggregation.md | 121 | Cross-model comparison dataset |
| uf11-concurrent-model-execution.md | 155 | Multi-GPU parallel scheduling |
| uf12-model-provider-abstraction.md | 147 | Unified provider API |
| uf13-progress-tracking-and-checkpointing.md | 128 | Crash recovery, progress display |
| uf14-validation-and-quality-scoring.md | 151 | Automated quality scoring |
| uf15-resource-monitoring-and-adaptation.md | 152 | VRAM/RAM/temp monitoring + adaptation |
| uf16-configuration-management.md | 148 | Config layering, validation, defaults |
| uf17-benchmark-report-generation.md | 153 | Rankings, stats, recommendations |
| uf18-cleanup-and-teardown.md | 118 | Graceful shutdown, resource free |
| uf19-model-registry-and-versioning.md | 132 | Historical tracking across runs |
| uf20-end-to-end-benchmark-session.md | 191 | Full session orchestration |
| **mvp-summary-report.md** (this file) | ~280 | Executive summary and scope analysis |
| **Total** | **~2,754** | |

---

## Quick Reference: 8-Prompt Chain

| Step | Capability | Validator | Weight |
|------|-----------|-----------|--------|
| 1 | Code Generation | Syntax + test run | 25% |
| 2 | Summarization | Compression ratio + key points | 10% |
| 3 | Reasoning | Known-answer logic/math | 15% |
| 4 | Creative Writing | Style + vocabulary diversity | 5% |
| 5 | Factual QA | Ground-truth accuracy | 15% |
| 6 | Instruction Following | Format compliance | 10% |
| 7 | Multi-Turn Conversation | Context coherence | 10% |
| 8 | Tool Use Planning | Plan completeness | 10% |

---

## Next Steps

1. **Schema extension**: Add the 28 identified properties as a `benchmark_execution` root section overlay
2. **Phase 1 implementation**: Config → Provider → Discovery → Workflow → Orchestration → Error Recovery → detail.md → Cleanup
3. **Phase 2 implementation**: Quantization → Loading → History → Scoring → Concurrency → Resource Monitor
4. **Phase 3 implementation**: Download → Aggregation → Checkpointing → Report → Registry → E2E
5. **Integration testing**: Run against 5 models locally, then scale to 20, then 100+
