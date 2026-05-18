# Phase 03: Cross-References

## Plan Files
- **Main Plan**: `docs/plans/03-quality-loops/plan.md` (326 lines)
- **Task 0**: `docs/plans/03-quality-loops/tasks/00-verifier-interface.md`
- **Task 1**: `docs/plans/03-quality-loops/tasks/01-generate-verify-repair-runtime.md`
- **Task 2**: `docs/plans/03-quality-loops/tasks/02-benchmark-harness.md`
- **Task 3**: `docs/plans/03-quality-loops/tasks/03-filetype-capability-matrix.md`
- **Task 4**: `docs/plans/03-quality-loops/tasks/04-artifact-workflow-library.md`
- **Task 5**: `docs/plans/03-quality-loops/tasks/05-report-generation.md`
- **Task 6**: `docs/plans/03-quality-loops/tasks/06-quality-dashboard-integration.md`

## Schema Sections
- **Section 5**: Advanced Execution Strategy (Lines 503-536)
  - Quality loop semantics: repair strategies, convergence detection
  - Performance optimization: benchmark definitions, metrics, storage

- **Section 8**: Performance Optimization (Lines 688-703)
  - Benchmark definitions and execution
  - Metrics collection: pipeline, step, model, tool, custom
  - Statistical analysis and trending

## Related QA
- **Phase 00**: Foundation (Schema and domain model complete)
  - Schema types and validation
  - YAML parsing
  - IR compilation
  - Reference: `docs/qa/phase-00/QA-CRITERIA.md`

- **Phase 01**: Core Execution Engine (LLM backends and tool framework complete)
  - Queue and scheduler
  - Step executor
  - Control flow
  - Reference: `docs/qa/phase-01/QA-CRITERIA.md`

- **Phase 02**: CLI & LLM Backends (Step executor and workflow engine complete)
  - CLI interface
  - Backend implementations
  - Tool execution
  - Reference: `docs/qa/phase-02/QA-CRITERIA.md`

- **Phase 04**: Memory & Search (runs in parallel)
  - RAG implementation
  - Local memory management
  - Reference: `docs/qa/phase-04/QA-CRITERIA.md` (when created)

## Validation Criteria
- **Framework**: `docs/plans/validation-criteria/framework.md` (571 lines)
  - 7 Verification Layers: Unit, Integration, Property, E2E, System Log, CLI, Benchmark
  - Evidence collection requirements
  - Phase entry/exit criteria
  - Checkpoint gate criteria
  - Task-level acceptance criteria
  - Cross-phase regression tests
  - Schema coverage audit
  - ADR compliance verification
  - Anti-goal-drift detection

- **ADR-0004 Compliance**: Generate-Verify-Repair Semantics
  - RepairLoop config is part of ExecutionEngine configuration
  - Convergence detection is MANDATORY
  - Verifier failures trigger repair step AUTOMATICALLY
  - Quality thresholds are ENFORCED at runtime
  - NOT "try three times and give up" - keep repairing until verifier passes or convergence criteria met

## External References
- **Verifier Interface**: Pluggable trait for different artifact types
- **Quality Loops**: Runtime semantics, not optional prompt engineering
- **Benchmarking**: Full provenance tracking (workflow version, policy snapshot, backend, file type)
- **Report Generation**: Multi-format (HTML/PDF/JSON/Markdown) with actionable insights
- **Dashboard Integration**: Quality trends and benchmark visualization

---

## Related Documentation
- **Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
- **Extended POC QA**: `docs/qa/extended-poc/QA-AREAS-EXTENDED-POC.md` (335 lines)
- **Extended POC Test Procedures**: `docs/qa/extended-poc/QA-TEST-PROCEDURES-EXTENDED-POC.md` (1405 lines)
