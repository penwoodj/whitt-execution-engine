# Phase 01: Cross-References

## Plan Files
- **Main Plan**: `docs/plans/01-core-execution-engine/plan.md` (481 lines)
- **Task 00**: `docs/plans/01-core-execution-engine/tasks/00-chatsession-containers.md`
- **Task 01**: `docs/plans/01-core-execution-engine/tasks/01-queue-state-machine.md`
- **Task 02**: `docs/plans/01-core-execution-engine/tasks/02-persistent-queue-storage.md`
- **Task 03**: `docs/plans/01-core-execution-engine/tasks/03-scheduler-core.md`
- **Task 04**: `docs/plans/01-core-execution-engine/tasks/04-step-executor.md`
- **Task 05**: `docs/plans/01-core-execution-engine/tasks/05-loop-runner.md`
- **Task 06**: `docs/plans/01-core-execution-engine/tasks/06-branch-evaluator.md`
- **Task 09**: `docs/plans/01-core-execution-engine/tasks/09-execution-modes.md`
- **Task 10**: `docs/plans/01-core-execution-engine/tasks/10-human-gating.md`
- **Task 11**: `docs/plans/01-core-execution-engine/tasks/11-logging.md`
- **Task 12**: `docs/plans/01-core-execution-engine/tasks/12-retry-error-handling.md`

## Schema Sections
- **Section 3**: Agentic Workflow (Lines 196-497)
  - Steps: workflow steps with step_type, inputs, outputs
  - User inputs: user-defined input specifications
  - Input types: supported input type definitions
  - Validation: input validation rules

- **Section 4**: Pipeline Definition (Lines 293-497)
  - Step types: Agent, Tool, Code, Sub-workflow step types
  - Dependencies: step dependency specifications
  - Branches: conditional branching definitions
  - Loops: all 6 loop types (count, foreach, while, validation, retry, infinite)

- **Section 5**: Workflow Execution Strategy (Lines 503-598)
  - Execution modes: Serial, Hybrid execution strategies
  - Retry policy: retry configuration with backoff strategies
  - Error handling: error escalation and recovery mechanisms
  - Checkpointing: execution checkpoint configuration
  - Synchronization: step coordination mechanisms
  - Dependency resolution: wait_for_all, continue_with_available, skip_failed, propagate_failure

- **Section 7**: Logging Configuration (Lines 732-741)
  - Log levels: 9-level hierarchy (Trace, Debug, Info, Warn, Error, Fatal, Panic, Off, Custom)
  - Scope config: per-scope logging configuration
  - Output destinations: Console, file, structured output

- **Section 8**: Metrics Configuration (Lines 743-752)
  - Pipeline metrics: pipeline-level metrics collection
  - Step metrics: step-level execution metrics
  - Model metrics: model usage metrics (placeholders for Phase 2)
  - Tool metrics: tool execution metrics
  - Custom metrics: user-defined metric collection

- **Section 9**: Validation Configuration (Lines 753-762)
  - Schema validation: schema validation rules
  - Step output validation: step output validation
  - Business rules: custom business rule validation

- **Section 10**: Orchestration Configuration (Lines 753-787)
  - Step coordination: step ordering and dependency management
  - Sub-agents: sub-agent coordination (trait-based, LLM backend in Phase 2)

## Related QA
- **Phase 00**: Foundation (WorkflowSpec, WorkflowIR, ./workspace/ storage layer)
  - Schema parsing and validation
  - IR compilation
  - Persistence layer
  - Reference: `docs/qa/phase-00/QA-CRITERIA.md`

- **Phase 02**: CLI & LLM Backends (depends on queue, scheduler, step executor)
  - CLI interface
  - Backend abstraction layer
  - Tool permissions
  - Code generation
  - Reference: `docs/qa/phase-02/QA-CRITERIA.md`

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

- **ADR-0002 Compliance**: ChatSession Work Containers, Human-Gated Safety
  - ChatSession as work container for all workflow execution
  - Human-gated safety: operation classification, confirmation prompts, diff preview
  - Staged diffs: preview changes before execution
  - CLI-first interface for user interaction

## External References
- **Architecture Diagram**: Async Rust using tokio for runtime coordination, sled for durable storage
- **Tech Stack**: Rust 1.75+, tokio, sled, tracing, serde, thiserror, uuid, chrono, futures, async-trait
- **File Structure**: chat_session/, queue/, scheduler/, executor/, control_flow/, execution_mode/, safety/, observability/
- **Verification Layers**: State machine validation, storage consistency, execution correctness, concurrency safety, resource management, error recovery, human gating

---

## Related Documentation
- **Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
- **Extended POC QA**: `docs/qa/extended-poc/QA-AREAS-EXTENDED-POC.md` (335 lines)
- **Extended POC Test Procedures**: `docs/qa/extended-poc/QA-TEST-PROCEDURES-EXTENDED-POC.md` (1405 lines)
