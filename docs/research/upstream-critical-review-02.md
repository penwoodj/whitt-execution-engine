# Upstream Factor Critical Review #2

**Date:** 2026-04-27
**Reviewer:** Sisyphus (Ralph Loop)
**Scope:** Full plan file suite + QA documentation + implementation alignment
**Schema Source of Truth:** [docs/schema/unified-workflow-schema.yml](../schema/unified-workflow-schema.yml) (805 lines)
**Previous Review:** [Critical Review #1](./upstream-critical-review-01.md)

---

## Executive Summary

Second critical review evaluating 8 NEW upstream factors (distinct from Review #1). Focuses on implementation feasibility, data flow integrity, error propagation, and operational readiness.

**Overall Status:** ⚠️ 2 ALIGNED, 4 MISALIGNED, 2 BLOCKING

---

## Factor 1: Data Flow Integrity Across Phases

- **Status:** ⚠️ MISALIGNED
- **Finding:** No formal data flow contract between phases
- **Plan Ref:** All phase plans ([00](../plans/00-foundation/plan.md) through [07](../plans/07-final-validation/plan.md))
- **QA Ref:** All [QA-CRITERIA.md](../qa/phase-00/QA-CRITERIA.md) files
- **Action Required:** Define interface contracts between phase boundaries

### Detailed Findings

Phase boundaries have implicit data dependencies but no explicit contracts:

| From Phase | To Phase | Data Contract | Documented? |
|-----------|----------|---------------|-------------|
| 00 (Foundation) | 01 (Core) | UnifiedConfig, ModelSpec | ⚠️ Implicit |
| 01 (Core) | 02 (CLI) | ReactAgent, ToolRegistry | ⚠️ Implicit |
| 02 (CLI) | 03 (Quality) | WorkflowState, Checkpoint | ❌ Missing |
| 03 (Quality) | 04 (Memory) | SearchResult, MemoryEntry | ❌ Missing |
| 04 (Memory) | 05 (Automation) | ScheduleEntry, CronExpr | ❌ Missing |
| 05 (Automation) | 06 (Metrics) | MetricEvent, Dashboard | ❌ Missing |
| 06 (Metrics) | 07 (Validation) | BenchmarkResult, Report | ❌ Missing |

### Key Issue
Plan files describe each phase independently but don't define the data types that cross phase boundaries. This risks incompatible implementations between phases.

---

## Factor 2: Error Propagation Strategy

- **Status:** ⚠️ MISALIGNED
- **Finding:** Error handling is phase-local; no cross-phase error propagation strategy
- **Plan Ref:** [Phase 00 Task 01](../plans/00-foundation/tasks/01-error-module.md), [Phase 01 Task 12](../plans/01-core-execution-engine/tasks/12-retry-error-handling.md)
- **QA Ref:** [QA-00-02](../qa/phase-00/QA-CRITERIA.md)
- **Action Required:** Define error hierarchy that spans all phases

### Detailed Findings

| Error Domain | Current | Needed For |
|-------------|---------|------------|
| ParseError | ✅ src/error.rs | YAML parsing |
| BackendError | ⚠️ Implicit | LLM communication |
| AgentError | ⚠️ Partial | Step execution |
| MemoryError | ❌ Missing | Phase 04 search/storage |
| ScheduleError | ❌ Missing | Phase 05 cron/automation |
| MetricError | ❌ Missing | Phase 06 collection |
| ValidationError | ⚠️ Partial | Phase 03 quality loops |

### Key Issue
`src/error.rs` defines `WhittError` enum with ~10 variants. Phases 04-06 need new error variants (MemoryError, ScheduleError, MetricError) but no plan task covers extending the error hierarchy.

---

## Factor 3: Async Runtime Resource Management

- **Status:** ✅ ALIGNED
- **Finding:** tokio runtime properly configured; resource limits in schema
- **Plan Ref:** [Phase 00 Task 00](../plans/00-foundation/tasks/00-cargo-project-setup.md)
- **QA Ref:** [QA-00-01](../qa/phase-00/QA-CRITERIA.md)
- **Action Required:** None

### Verification

- tokio `{ version = "1.40", features = ["full", "fs"] }` — correct
- Schema defines `execution.memory.max_allocated_memory_mb` and `model_memory_mb`
- Resource limits implemented in `src/model/resource.rs`
- Agent executor respects max_iterations

---

## Factor 4: Configuration Hot-Reload and Merge Strategy

- **Status:** ⚠️ MISALIGNED
- **Finding:** Config merge logic exists but no hot-reload capability
- **Plan Ref:** [Phase 00 Task 06](../plans/00-foundation/tasks/06-config-merge-override.md)
- **QA Ref:** [QA-00-06](../qa/phase-00/QA-CRITERIA.md)
- **Action Required:** Document config reload strategy for running workflows

### Detailed Findings

| Capability | Status | Impact |
|-----------|--------|--------|
| Config file loading | ✅ `UnifiedConfig::from_file()` | One-time load |
| CLI override merge | ✅ `merge_configs()` in config/mod.rs | Startup only |
| Environment variable override | ⚠️ Partial | Not documented |
| Hot-reload (running workflow) | ❌ Missing | Phase 05 needs this for schedule changes |
| Per-step config override | ❌ Missing | Schema allows `${...}` interpolation |

### Key Issue
Phase 05 (Automation) requires dynamic schedule updates without restarting the engine. No task in any phase covers config hot-reload.

---

## Factor 5: Observability and Telemetry Pipeline

- **Status:** ⚠️ MISALIGNED
- **Finding:** Logging exists but no structured telemetry pipeline
- **Plan Ref:** [Phase 01 Task 10](../plans/01-core-execution-engine/tasks/10-logging-framework.md), [Phase 06](../plans/06-autonomy-metrics/plan.md)
- **QA Ref:** [QA-01-10](../qa/phase-01/QA-CRITERIA.md)
- **Action Required:** Define telemetry schema before Phase 06 implementation

### Detailed Findings

| Observability Layer | Status | Consumer |
|-------------------|--------|----------|
| `tracing` crate | ✅ Configured | Human-readable logs |
| `tracing-subscriber` | ✅ With env-filter | RUST_LOG control |
| Structured JSON logs | ❌ Missing | Machine parsing |
| OpenTelemetry export | ❌ Missing | External monitoring |
| Custom metrics emission | ❌ Missing | Phase 06 dashboards |
| Log correlation IDs | ❌ Missing | Cross-step tracing |

### Key Issue
Phase 06 (Autonomy Metrics) needs structured telemetry to power dashboards. Current logging is text-based. Need to define a telemetry schema (what events, what fields, what format) before Phase 06 tasks can be implemented.

---

## Factor 6: State Serialization Format Stability

- **Status:** ❌ BLOCKING
- **Finding:** Persistence uses serde_json but format is not versioned or backward-compatible
- **Plan Ref:** [Phase 01 Task 02](../plans/01-core-execution-engine/tasks/02-persistent-queue-storage.md)
- **QA Ref:** [QA-01-02](../qa/phase-01/QA-CRITERIA.md), [QA-14](../qa/extended-poc/QA-AREAS-EXTENDED-POC.md)
- **Action Required:** Define state schema versioning strategy

### Detailed Findings

| State Type | Format | Versioned? | Backward Compat? |
|-----------|--------|-----------|-----------------|
| WorkflowState | JSON | ❌ No | ❌ No |
| Checkpoint | JSON | ❌ No | ❌ No |
| Schedule (Phase 05) | TBD | ❌ TBD | ❌ TBD |
| Memory entries (Phase 04) | TBD | ❌ TBD | ❌ TBD |

### Key Issue
`src/agent/persistence.rs` serializes `WorkflowState` with `serde_json`. If the struct changes between versions, saved checkpoints become unreadable. This will break Phase 05 (scheduling) and Phase 04 (memory) which depend on long-lived state. Need a state migration strategy.

---

## Factor 7: Docker and Deployment Readiness

- **Status:** ✅ ALIGNED
- **Finding:** Docker setup functional with documented Vulkan constraints
- **Plan Ref:** [Phase 02](../plans/02-cli-and-llm-backend-integration/plan.md)
- **QA Ref:** [QA-02-03](../qa/phase-02/QA-CRITERIA.md), [QA Docker](../qa/poc-local-llm-docker/QA-AREAS.md)
- **Action Required:** None for POC; full deployment guide for production

### Verification

- `docker-compose.yml` with base/AMD/NVIDIA variants ✅
- `docker/entrypoint.sh` with Vulkan-safe flags ✅
- `--no-cache-prompt` enforced ✅
- `--cont-batching` disabled ✅
- f16 KV cache enforced ✅
- Entrypoint mounted at `/entrypoint.sh:ro` ✅

---

## Factor 8: Testing Infrastructure Scalability

- **Status:** ❌ BLOCKING
- **Finding:** Test infrastructure doesn't support Phase 07's 120-model benchmark requirement
- **Plan Ref:** [Phase 07 Task 08](../plans/07-final-validation/plan.md)
- **QA Ref:** [QA-07-09](../qa/phase-07/QA-CRITERIA.md)
- **Action Required:** Design test infrastructure for concurrent model scaling tests

### Detailed Findings

| Test Scale | Current Support | Phase 07 Requirement |
|-----------|----------------|---------------------|
| 1 model | ✅ Working | Baseline |
| 5 models | ⚠️ Theoretical | Benchmark Task 05 |
| 20 models | ❌ No infrastructure | Benchmark Task 06 |
| 50 models | ❌ No infrastructure | Benchmark Task 07 |
| 120 models | ❌ No infrastructure | Benchmark Task 08 (CRITICAL) |

### Infrastructure Gaps

1. **Model mocking**: `MockLlmBackend` exists but can't simulate 120 concurrent models
2. **Resource simulation**: No GPU/RAM simulation for resource limit testing
3. **Parallel test runner**: No infrastructure for concurrent model lifecycle tests
4. **Benchmark harness**: criterion exists but no model-scaling benchmarks defined

### Key Issue
Phase 07 requires proving the engine handles 120 concurrent models with memory management. This needs a scalable mock infrastructure that doesn't exist yet. The plan files don't include a task to build this mock infrastructure — they assume it exists.

---

## Summary of Findings

| Factor | Status | Severity | Action |
|--------|--------|----------|--------|
| 1. Data Flow Integrity | ⚠️ MISALIGNED | HIGH | Define interface contracts at phase boundaries |
| 2. Error Propagation | ⚠️ MISALIGNED | MEDIUM | Extend error hierarchy for phases 04-06 |
| 3. Async Runtime | ✅ ALIGNED | LOW | None |
| 4. Config Hot-Reload | ⚠️ MISALIGNED | MEDIUM | Add hot-reload task to Phase 05 |
| 5. Observability Pipeline | ⚠️ MISALIGNED | HIGH | Define telemetry schema before Phase 06 |
| 6. State Serialization | ❌ BLOCKING | HIGH | Add state versioning strategy to Phase 01 |
| 7. Docker Readiness | ✅ ALIGNED | LOW | None for POC |
| 8. Test Infrastructure | ❌ BLOCKING | HIGH | Design scalable mock infrastructure for Phase 07 |

## Action Items (Priority Order)

1. **P0:** Add state schema versioning task to Phase 01 plan (backward-compatible serialization)
2. **P0:** Design scalable mock infrastructure task for Phase 07 (120-model benchmark)
3. **P1:** Define data flow contracts at each phase boundary (interface types document)
4. **P1:** Extend error hierarchy — add MemoryError, ScheduleError, MetricError to error.rs planning
5. **P1:** Define telemetry schema (structured event format for Phase 06 dashboards)
6. **P2:** Add config hot-reload capability task to Phase 05
7. **P2:** Document environment variable override patterns
8. **P2:** Add log correlation IDs for cross-step tracing
