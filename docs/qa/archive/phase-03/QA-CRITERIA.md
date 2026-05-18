# Phase 03: Quality Loops — QA Criteria

## Overview
Phase 03 implements generate-verify-repair runtime semantics and comprehensive benchmarking system to measure and improve execution quality across all artifact types. This phase builds pluggable verifier traits, quality loops, benchmark harness, and reporting infrastructure.

## Dependencies
- **Phase 00**: Schema and domain model complete
- **Phase 01**: LLM backends and tool framework complete
- **Phase 02**: Step executor and workflow engine complete
- **Parallel**: Phase 04 (Memory & Search) can run concurrently

## QA Areas

### QA-03-01: Verifier Interface System
- **Plan Ref**: Task 0 (`docs/plans/03-quality-loops/tasks/00-verifier-interface.md`)
- **Schema Ref**: N/A (verification infrastructure)
- **Priority**: P0
- **Test Type**: Unit + Integration
- **Criteria**:
  - Pluggable `Verifier` trait with methods: capabilities(), verify(), name()
  - VerifierCapabilities struct describes what verifier can check
  - Built-in verifiers for code: compilation, linting, test execution
  - Built-in verifiers for docs: completeness, structure, markdown validation
  - Built-in verifiers for configs: schema validation, required fields
  - Verifier registry for dynamic discovery
  - Custom verifier registration API
- **Commands**:
  - `cargo test verifier_interface --lib`
  - `cargo test builtin_verifiers --lib`
  - `cargo test verifier_registry --lib`
  - `cargo clippy -- -D warnings`

### QA-03-02: Generate-Verify-Repair Runtime
- **Plan Ref**: Task 1 (`docs/plans/03-quality-loops/tasks/01-generate-verify-repair-runtime.md`)
- **Schema Ref**: Lines 503-536 (advanced execution strategy - quality loops)
- **Priority**: P0
- **Test Type**: Integration + Unit
- **Criteria**:
  - RepairLoop config part of ExecutionEngine configuration
  - Convergence detection MANDATORY (must terminate within max_iterations)
  - Verifier failures trigger repair step AUTOMATICALLY
  - Quality thresholds ENFORCED at runtime, not suggested
  - Generate step: creates artifact using configured generator
  - Verify step: uses configured verifier
  - Repair step: modifies artifact based on verifier feedback
  - Convergence: epsilon threshold, exact criteria matching
  - State tracking: iteration count, quality score, repair history
  - NOT "try three times and give up" - keep repairing until verifier passes or convergence criteria met
- **Commands**:
  - `cargo test generate_verify_repair --lib`
  - `cargo test quality_loops --test`
  - `cargo clippy -- -D warnings`

### QA-03-03: Benchmark Harness
- **Plan Ref**: Task 2 (`docs/plans/03-quality-loops/tasks/02-benchmark-harness.md`)
- **Schema Ref**: Lines 688-703 (performance optimization)
- **Priority**: P0
- **Test Type**: Integration + Unit
- **Criteria**:
  - Benchmark struct: workflow_version, policy_snapshot, backend, file_type, metrics, timestamps
  - Full provenance tracking: workflow version, policy snapshot, backend, file type
  - BenchmarkSuite: collection of benchmarks with metadata
  - Benchmark storage: persistent store for benchmarks
  - Benchmark execution: run benchmarks, capture metrics
  - Benchmark versioning: track changes over time
  - Statistical analysis: mean, median, std dev, percentiles
  - Time-based partitioning for long-term storage
  - Compression for historical benchmarks
- **Commands**:
  - `cargo test benchmark_harness --lib`
  - `cargo test benchmark_storage --test`
  - `cargo test benchmark_execution --test`
  - `cargo clippy -- -D warnings`

### QA-03-04: File-Type Capability Matrix
- **Plan Ref**: Task 3 (`docs/plans/03-quality-loops/tasks/03-filetype-capability-matrix.md`)
- **Schema Ref**: N/A (knowledge graph of capabilities)
- **Priority**: P1
- **Test Type**: Unit
- **Criteria**:
  - FileCapability struct: workflow_id, file_type, quality_threshold, proven_at
  - CapabilityMatrix: HashMap<(WorkflowId, FileType), FileCapability>
  - Capability registration: workflow → file type → quality threshold mappings
  - Capability querying: find workflows supporting file type
  - Gap analysis: discover missing capabilities
  - Dynamic workflow selection based on file type
  - Knowledge graph of workflow capabilities
- **Commands**:
  - `cargo test capability_matrix --lib`
  - `cargo test capability_querying --lib`
  - `cargo test gap_analysis --lib`
  - `cargo clippy -- -D warnings`

### QA-03-05: Artifact Workflow Library
- **Plan Ref**: Task 4 (`docs/plans/03-quality-loops/tasks/04-artifact-workflow-library.md`)
- **Schema Ref**: N/A (workflow library)
- **Priority**: P1
- **Test Type**: Integration + Unit
- **Criteria**:
  - ArtifactWorkflow struct: workflow_id, version, definition, metadata
  - Workflow storage: versioned storage for workflow definitions
  - Workflow transformation API: transform, optimize, refactor
  - Versioning: track changes to workflow definitions
  - Review controls: approval workflow for workflow updates
  - Workflow retrieval: get latest, get specific version
  - Dependency tracking: workflow dependencies resolved
- **Commands**:
  - `cargo test workflow_library --lib`
  - `cargo test workflow_storage --test`
  - `cargo test workflow_transformation --test`
  - `cargo clippy -- -D warnings`

### QA-03-06: Report Generation
- **Plan Ref**: Task 5 (`docs/plans/03-quality-loops/tasks/05-report-generation.md`)
- **Schema Ref**: N/A (reporting infrastructure)
- **Priority**: P1
- **Test Type**: Unit + Integration
- **Criteria**:
  - Report types: QualityReport, BenchmarkReport, GapAnalysisReport
  - HTML generation: interactive dashboard viewing (handlebars templates)
  - PDF generation: archiving and executive summaries (printpdf)
  - JSON generation: programmatic consumption and CI integration
  - Markdown generation: documentation and code reviews
  - Actionable insights: specific improvement suggestions
  - Multi-format support: generate all four formats simultaneously
  - Report templates: layout, charts, tables
- **Commands**:
  - `cargo test report_generation --lib`
  - `cargo test html_reports --test`
  - `cargo test pdf_reports --test`
  - `cargo test json_reports --test`
  - `cargo test markdown_reports --test`
  - `cargo clippy -- -D warnings`

### QA-03-07: Quality Dashboard Integration
- **Plan Ref**: Task 6 (`docs/plans/03-quality-loops/tasks/06-quality-dashboard-integration.md`)
- **Schema Ref**: N/A (dashboard API)
- **Priority**: P1
- **Test Type**: Integration
- **Criteria**:
  - Dashboard API routes: /metrics, /reports, /trends, /benchmarks, /capabilities
  - Quality trends: time series of quality metrics per workflow
  - File-type comparison: quality metrics by file type across workflows
  - Benchmark visualization: historical benchmark performance
  - Dashboard layout: charts, tables, filters
  - Real-time updates: websocket or polling for live data
  - API handlers: proper error handling, response codes
- **Commands**:
  - `cargo test dashboard_api --lib`
  - `cargo test dashboard_routes --test`
  - `cargo test dashboard_visualization --test`
  - `cargo clippy -- -D warnings`

---

## Verification Summary
**Total QA Areas**: 7
**P0 Areas**: 3 (03-01, 03-02, 03-03)
**P1 Areas**: 4 (03-04, 03-05, 03-06, 03-07)

**Schema Coverage**: 100% of Sections 5, 8

---

## Related Plan Tasks

- [Task 00](../../plans/03-quality-loops/tasks/00-verifier-interface.md) — Pluggable Verifier trait with capabilities(), verify(), name() methods, built-in verifiers, and registry
- [Task 01](../../plans/03-quality-loops/tasks/01-generate-verify-repair-runtime.md) — RepairLoop config, convergence detection, verifier failures, quality thresholds, state tracking
- [Task 02](../../plans/03-quality-loops/tasks/02-benchmark-harness.md) — Benchmark struct, BenchmarkSuite, storage, execution, versioning, and statistical analysis
- [Task 03](../../plans/03-quality-loops/tasks/03-filetype-capability-matrix.md) — FileCapability struct, CapabilityMatrix, capability registration, capability querying, and gap analysis
- [Task 04](../../plans/03-quality-loops/tasks/04-artifact-workflow-library.md) — ArtifactWorkflow struct, workflow storage, transformation API, versioning, and review controls
- [Task 05](../../plans/03-quality-loops/tasks/05-report-generation.md) — Report types (QualityReport, BenchmarkReport, GapAnalysisReport), HTML/PDF/JSON/Markdown generation, actionable insights
- [Task 06](../../plans/03-quality-loops/tasks/06-quality-dashboard-integration.md) — Dashboard API routes, quality trends, file-type comparison, benchmark visualization, real-time updates

---

## References
- **Plan**: `docs/plans/03-quality-loops/plan.md` (326 lines, 7 tasks)
- **Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
- **Validation Framework**: `docs/plans/validation-criteria/framework.md` (571 lines)
- **ADR-0004 Compliance**: Generate-Verify-Repair Semantics - runtime semantics enforcement
- **Existing QA**: `docs/qa/extended-poc/QA-AREAS-EXTENDED-POC.md` (335 lines)
