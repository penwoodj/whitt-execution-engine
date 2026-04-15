# Phase 4: Quality Loops & Benchmarks Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the generate-verify-repair runtime semantics and comprehensive benchmarking system to measure and improve execution quality across all artifact types.

**Architecture:** Implement pluggable verifier traits for different artifact types (code/docs/configs), runtime quality loop semantics with convergence detection, comprehensive benchmark harness that captures workflow version + policy snapshot + backend + file type, and a multi-format reporting system with actionable insights.

**Tech Stack:** Rust, tokio for async, serde for serialization, clap for CLI, plotters for visualization, handlebars for templating, mock LLM backends for testing.

---

## Executive Summary

Phase 4 implements the quality assurance infrastructure that makes AgentSDK production-ready. This phase builds:

1. **Verifier Interface System** — Pluggable traits for validating artifacts (code compilation, linting, test execution, documentation completeness, config validation)
2. **Generate-Verify-Repair Runtime** — Core quality loop semantics that are PART OF RUNTIME BEHAVIOR, not optional prompt engineering
3. **Benchmark Harness** — Storage and execution of benchmark suites with full provenance tracking
4. **File-Type Capability Matrix** — Knowledge graph of what workflows support which file types with what quality thresholds
5. **Artifact Workflow Library** — Versioned storage and transformation API for workflow definitions
6. **Report Generation** — Multi-format reports (HTML/PDF/JSON/Markdown) with actionable insights
7. **Quality Dashboard Integration** — Real-time quality trends and visualization

### ADR-0005 Compliance (CRITICAL)

**Per ADR-0005:** Generate-verify-repair loops are RUNTIME SEMANTICS, not optional prompt engineering patterns.

- The RepairLoop config is part of ExecutionEngine configuration
- Convergence detection is MANDATORY (must terminate within max_iterations)
- Verifier failures trigger repair step AUTOMATICALLY
- Quality thresholds are ENFORCED at runtime, not suggested
- This is NOT "try three times and give up" — it's "keep repairing until verifier passes or convergence criteria met"

### Dependencies

**Hard Dependencies (Must Complete First):**
- Phase 0: Schema and domain model complete
- Phase 1: LLM backends and tool framework complete
- Phase 2: Step executor and workflow engine complete

**Parallel Dependency (Can Run Concurrently):**
- Phase 3: Advanced execution strategies

**Estimated Timeline:** 10-12 weeks

### Schema Domains

This phase implements schema from:
- **Section 5: Advanced Execution Strategy** — Quality loop semantics, repair strategies
- **Section 8: Performance Optimization** — Benchmark definitions, metrics, storage

---

## Task Overview

### Task 0: Verifier Interface System
**Files:** `crates/quality/verifier/src/lib.rs`, `crates/quality/verifier/src/builtin/`, `crates/quality/verifier/src/registry.rs`
**Duration:** 1.5 weeks
Build the pluggable verifier trait with built-in verifiers for code, docs, and configs.

### Task 1: Generate-Verify-Repair Runtime
**Files:** `crates/quality/loops/src/lib.rs`, `crates/quality/loops/src/repair.rs`, `crates/quality/loops/src/state.rs`
**Duration:** 2 weeks
Implement the runtime quality loop semantics with convergence detection and state tracking.

### Task 2: Benchmark Harness
**Files:** `crates/quality/benchmark/src/lib.rs`, `crates/quality/benchmark/src/storage.rs`, `crates/quality/benchmark/src/execution.rs`
**Duration:** 2 weeks
Build the benchmark storage and execution system with full provenance tracking.

### Task 3: File-Type Capability Matrix
**Files:** `crates/quality/capability/src/lib.rs`, `crates/quality/capability/src/matrix.rs`, `crates/quality/capability/src/registry.rs`
**Duration:** 1.5 weeks
Create the knowledge graph of workflow capabilities per file type with quality thresholds.

### Task 4: Artifact Workflow Library
**Files:** `crates/quality/workflow/src/lib.rs`, `crates/quality/workflow/src/storage.rs`, `crates/quality/workflow/src/transform.rs`
**Duration:** 1.5 weeks
Build versioned workflow storage with transformation API and review controls.

### Task 5: Report Generation
**Files:** `crates/quality/reports/src/lib.rs`, `crates/quality/reports/src/generators/`, `crates/quality/reports/src/insights.rs`
**Duration:** 1.5 weeks
Implement multi-format report generation with actionable insights.

### Task 6: Quality Dashboard Integration
**Files:** `crates/quality/dashboard/src/lib.rs`, `crates/quality/dashboard/api/routes.rs`, `crates/quality/dashboard/api/handlers.rs`
**Duration:** 1 week
Build the dashboard API with quality trends and benchmark visualization.

---

## Detailed Task Breakdown

See individual task files in `tasks/` directory:

- [`tasks/00-verifier-interface.md`](./tasks/00-verifier-interface.md) — Verifier trait, VerificationResult, VerifierCapabilities, built-in verifiers, registry
- [`tasks/01-generate-verify-repair-runtime.md`](./tasks/01-generate-verify-repair-runtime.md) — RepairLoop config, generate/verify/repair steps, convergence criteria, state tracking
- [`tasks/02-benchmark-harness.md`](./tasks/02-benchmark-harness.md) — Benchmark struct, BenchmarkSuite, storage, execution, versioning, statistical analysis
- [`tasks/03-filetype-capability-matrix.md`](./tasks/03-filetype-capability-matrix.md) — FileCapability struct, CapabilityMatrix, registration, querying, gap analysis
- [`tasks/04-artifact-workflow-library.md`](./tasks/04-artifact-workflow-library.md) — ArtifactWorkflow struct, storage, versioning, transformation API, review controls
- [`tasks/05-report-generation.md`](./tasks/05-report-generation.md) — Report types, HTML/PDF/JSON/Markdown generation, actionable insights
- [`tasks/06-quality-dashboard-integration.md`](./tasks/06-quality-dashboard-integration.md) — Dashboard layout, quality trends, file-type comparison, benchmark visualization

---

## Validation Criteria

See [`validation/`](./validation/) directory for comprehensive validation criteria:

- [`validation/verifier-interface.md`](./validation/verifier-interface.md) — Verifier trait contract, builtin verifier correctness
- [`validation/quality-loops.md`](./validation/quality-loops.md) — Convergence detection, state consistency, ADR-0005 compliance
- [`validation/benchmark-system.md`](./validation/benchmark-system.md) — Provenance tracking, statistical correctness
- [`validation/capability-matrix.md`](./validation/capability-matrix.md) — Matrix consistency, query accuracy
- [`validation/reporting.md`](./validation/reporting.md) — Report accuracy, insights quality

---

## Test Specifications

See [`tests/`](./tests/) directory for test specifications:

- [`tests/verifier-mocks.md`](./tests/verifier-mocks.md) — Mock verifier implementations for testing
- [`tests/quality-loop-mocks.md`](./tests/quality-loop-mocks.md) — Mock LLM backends for repair step testing
- [`tests/benchmark-mocks.md`](./tests/benchmark-mocks.md) — Mock benchmark data generators
- [`tests/integration-tests.md`](./tests/integration-tests.md) — End-to-end quality loop and benchmark integration tests

---

## Key Design Decisions

### 1. Quality Loops are Runtime Semantics (ADR-0005)

**Decision:** Quality loops are part of ExecutionEngine configuration and enforced at runtime, not optional prompt patterns.

**Rationale:**
- Ensures quality gates are ALWAYS active
- Prevents configuration drift where quality gets "forgotten"
- Enables predictable convergence guarantees
- Makes quality metrics observable and auditable

**Implementation:**
- `RepairLoopConfig` is a required field in `ExecutionEngineConfig`
- Convergence criteria are enforced at loop level
- Verifier failures trigger repair step automatically
- State is tracked and persisted for debugging

### 2. Pluggable Verifier Trait

**Decision:** Implement `Verifier` trait with `VerifierCapabilities` to describe what a verifier can check.

**Rationale:**
- Different artifact types need different verification (code vs docs vs configs)
- Allows community contribution of custom verifiers
- Enables progressive capability discovery
- Supports dynamic verifier selection based on artifact type

**Implementation:**
```rust
pub trait Verifier: Send + Sync {
    fn capabilities(&self) -> VerifierCapabilities;
    async fn verify(&self, artifact: &Artifact) -> VerificationResult;
    fn name(&self) -> &str;
}
```

### 3. Full Provenance Tracking for Benchmarks

**Decision:** Benchmarks MUST store workflow version, policy snapshot, backend, and file type to enable accurate trend analysis.

**Rationale:**
- Quality improvements need context to be meaningful
- Enables attribution of quality changes to specific components
- Supports A/B testing of different configurations
- Provides audit trail for compliance

**Implementation:**
```rust
pub struct Benchmark {
    pub workflow_version: String,
    pub policy_snapshot: serde_json::Value,
    pub backend: LLMBackend,
    pub file_type: String,
    // ... metrics, timestamps, etc.
}
```

### 4. Multi-Format Report Generation

**Decision:** Generate reports in HTML, PDF, JSON, and Markdown formats with actionable insights.

**Rationale:**
- HTML for interactive dashboard viewing
- PDF for archiving and executive summaries
- JSON for programmatic consumption and CI integration
- Markdown for documentation and code reviews

**Implementation:**
- Use handlebars for HTML templating
- Use printpdf for PDF generation
- Use serde for JSON serialization
- Use custom Markdown generator with table/chart support

### 5. Capability Matrix as Knowledge Graph

**Decision:** Store capability matrix as queryable knowledge graph of workflow → file type → quality threshold mappings.

**Rationale:**
- Enables gap analysis to discover missing capabilities
- Supports dynamic workflow selection based on file type
- Provides transparency about what workflows can do
- Facilitates roadmap planning for new capabilities

**Implementation:**
```rust
pub struct CapabilityMatrix {
    capabilities: HashMap<(WorkflowId, FileType), FileCapability>,
}
```

---

## Risk Mitigation

### Risk 1: Infinite Repair Loops

**Mitigation:**
- Enforce `max_iterations` constraint (default 10)
- Implement convergence detection with epsilon threshold
- Add exponential backoff for repair attempts
- Track loop history for debugging

### Risk 2: Verifier Bottlenecks

**Mitigation:**
- Make verifiers async and parallelizable
- Implement verifier caching for repeated artifacts
- Use verifier capability hints to skip unnecessary checks
- Provide progress reporting for long-running verifiers

### Risk 3: Benchmark Storage Growth

**Mitigation:**
- Implement time-based partitioning
- Add compression for historical benchmarks
- Provide pruning strategies based on retention policies
- Support incremental aggregation for long-term trend analysis

### Risk 4: Mock LLM Quality for Testing

**Mitigation:**
- Provide deterministic mock responses
- Support both success and failure scenarios
- Implement realistic delay and error patterns
- Allow configuration of mock behavior per test

---

## Success Criteria

Phase 4 is complete when:

1. ✅ All verifier interfaces are implemented and tested
2. ✅ Generate-verify-repair loops execute with convergence detection
3. ✅ Benchmark harness stores and retrieves benchmarks with full provenance
4. ✅ Capability matrix supports registration and querying
5. ✅ Reports generate in all four formats with accurate insights
6. ✅ Dashboard API provides quality trends and visualization
7. ✅ All validation criteria pass
8. ✅ All integration tests pass
9. ✅ Documentation is complete
10. ✅ Code review and security audit pass

---

## Handoff Criteria

Before handing off to execution:

1. ✅ Plan is reviewed and approved
2. ✅ All task files are complete and detailed
3. ✅ Validation criteria are comprehensive
4. ✅ Test specifications cover all edge cases
5. ✅ Mock strategies are well-defined
6. ✅ ADR-0005 compliance is explicit and enforceable
7. ✅ Timeline is realistic and achievable

---

## Appendix: File Structure

```
opencode/docs/plans/04-quality-loops/
├── plan.md                              # This file
├── tasks/
│   ├── 00-verifier-interface.md         # Verifier trait implementation
│   ├── 01-generate-verify-repair-runtime.md  # Quality loop semantics
│   ├── 02-benchmark-harness.md          # Benchmark storage and execution
│   ├── 03-filetype-capability-matrix.md # Capability matrix
│   ├── 04-artifact-workflow-library.md  # Workflow library
│   ├── 05-report-generation.md          # Report generation
│   └── 06-quality-dashboard-integration.md  # Dashboard API
├── validation/
│   ├── verifier-interface.md            # Verifier validation criteria
│   ├── quality-loops.md                  # Quality loop validation
│   ├── benchmark-system.md              # Benchmark validation
│   ├── capability-matrix.md             # Capability matrix validation
│   └── reporting.md                     # Report validation
└── tests/
    ├── verifier-mocks.md                 # Mock verifiers
    ├── quality-loop-mocks.md            # Mock LLM for repair
    ├── benchmark-mocks.md                # Mock benchmark data
    └── integration-tests.md              # Integration test specs
```

---

## Related Documents

- [ADR-0005: Generate-Verify-Repair Semantics](../../adr/0005-generate-verify-repair.md)
- [Phase 0: Schema and Domain Model](../00-schema/plan.md)
- [Phase 1: LLM Backends and Tool Framework](../01-llm-framework/plan.md)
- [Phase 2: Step Executor and Workflow Engine](../02-execution-engine/plan.md)
- [Phase 3: Advanced Execution Strategies](../03-advanced-execution/plan.md)
- [Schema Reference](../../schema/schema.md)
