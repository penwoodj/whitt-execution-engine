# Phase 01: Core Execution Engine — QA Criteria

## Overview

Phase 1 implements the core execution engine for AgentSDK, including persistent job queue, scheduler, step executors, control flow, human gating, and observability. This phase builds the runtime infrastructure that all subsequent phases depend on.

**Dependencies:** Phase 0 must be complete (WorkflowSpec, WorkflowIR, ./workspace/ storage layer)
**Estimated Time:** 8-10 weeks (12 tasks, ~1 week each)

**ADR-0002 Compliance:**
- ChatSession as work container for all workflow execution
- Human-gated safety: operation classification, confirmation prompts, diff preview
- Staged diffs: preview changes before execution
- CLI-first interface for user interaction

---

## QA Areas

### QA-01-01: ChatSession Lifecycle
- **Plan Ref:** [Task 00](../../plans/01-core-execution-engine/tasks/00-chatsession-containers.md)
- **Schema Ref:** N/A (execution engine component)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. ChatSession containers created with persistent IDs (UUID v4)
  2. Session lifecycle states: created → activated → completed
  3. Session metadata tracking: workflow ID, inputs, outputs
  4. Execution state tracking: queued, running, paused, completed
  5. Human interaction history stored: confirmations, approvals
  6. Checkpoint data saved for resumption after failure
- **Commands:**
  ```bash
  cargo test --lib chat_session
  cargo test --test '*chat_session*'
  ```

### QA-01-02: Queue State Machine
- **Plan Ref:** [Task 01](../../plans/01-core-execution-engine/tasks/01-queue-state-machine.md)
- **Schema Ref:** N/A (execution engine component)
- **Priority:** P0
- **Test Type:** Unit + Property
- **Criteria:**
  1. Valid state transitions only allowed:
     - pending → scheduled → running → completed/failed/cancelled/timeout
     - running → paused → running
     - Any state → cancelled
  2. State machine is acyclic (no invalid loops)
  3. State transitions logged with timestamps
  4. Job metadata preserved across state changes
  5. Invalid transitions rejected with errors
- **Commands:**
  ```bash
  cargo test --lib queue_state
  PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based -- --test-threads=1
  ```

### QA-01-03: Persistent Queue Storage
- **Plan Ref:** [Task 02](../../plans/01-core-execution-engine/tasks/02-persistent-queue-storage.md)
- **Schema Ref:** N/A (execution engine component)
- **Priority:** P0
- **Test Type:** Integration + Unit
- **Criteria:**
  1. sled-based KV store for durable queue state
  2. Job metadata stored in sled with ACID transactions
  3. Execution history persisted across restarts
  4. Checkpoint data saved with versioned references
  5. Recovery mechanisms work after process crashes
  6. Storage operations are atomic (all-or-nothing rollback)
  7. Concurrent access is thread-safe
- **Commands:**
  ```bash
  cargo test --lib storage
  cargo test --test '*storage*'
  ```

### QA-01-04: Scheduler Core
- **Plan Ref:** [Task 03](../../plans/01-core-execution-engine/tasks/03-scheduler-core.md)
- **Schema Ref:** N/A (execution engine component)
- **Priority:** P0
- **Test Type:** Integration + Unit
- **Criteria:**
  1. Worker pool manages concurrent execution (tokio tasks)
  2. Job prioritization based on configured priority levels
  3. Execution loop with cancellation support
  4. Retry with exponential backoff
  5. Job queue scheduling algorithm fair and predictable
  6. Worker cleanup on job completion or cancellation
  7. Job timeout handling
- **Commands:**
  ```bash
  cargo test --lib scheduler
  cargo test --test '*scheduler*'
  ```

### QA-01-05: Step Executor Interface
- **Plan Ref:** [Task 04](../../plans/01-core-execution-engine/tasks/04-step-executor.md)
- **Schema Ref:** Section 3 (Agentic Workflow - step_types)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. Step executor trait defined with execute() method
  2. Agent step executor for LLM inference (uses LlmBackend trait)
  3. Tool step executor for tool invocation
  4. Code step executor for code execution
  5. Sub-workflow step executor for nested workflows
  6. Step result types (success, output, error)
  7. Step timeouts respected
  8. Step retry logic using configured retry policy
- **Commands:**
  ```bash
  cargo test --lib executor
  cargo test --test '*executor*'
  ```

### QA-01-06: Agent Step Executor
- **Plan Ref:** [Task 04](../../plans/01-core-execution-engine/tasks/04-step-executor.md)
- **Schema Ref:** Section 3 (Agentic Workflow - step_types: agent step)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. LLM backend trait usage for inference calls
  2. Streaming responses handled (SSE/NDJSON)
  3. Chat completion requests constructed correctly
  4. Tool calls integrated into agent step
  5. Model loading/unloading respected
  6. Token usage tracked
  7. Backend health checks respected
- **Commands:**
  ```bash
  cargo test --lib agent_executor
  cargo test --test '*agent*'
  ```

### QA-01-07: Tool Step Executor
- **Plan Ref:** [Task 04](../../plans/01-core-execution-engine/tasks/04-step-executor.md)
- **Schema Ref:** Section 3 (Agentic Workflow - step_types: tool step)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. Tool executor trait defined
  2. Built-in tools: file, web, shell operations
  3. Tool registration and discovery
  4. Tool execution with permission checks
  5. Tool output captured and returned
  6. Tool errors handled gracefully
  7. Tool sandboxing enforced (allowed paths, forbidden commands)
- **Commands:**
  ```bash
  cargo test --lib tool_executor
  cargo test --test '*tool*'
  ```

### QA-01-08: Sub-Workflow Step Executor
- **Plan Ref:** [Task 04](../../plans/01-core-execution-engine/tasks/04-step-executor.md)
- **Schema Ref:** Section 3 (Agentic Workflow - step_types: sub-workflow step)
- **Priority:** P1
- **Test Type:** Unit + Integration
- **Criteria:**
  1. Nested workflow execution supported
  2. Sub-workflow isolation and scoping
  3. Sub-workflow result handling
  4. Sub-workflow error propagation
  5. Nested execution depth limits enforced
  6. Sub-workflow input/output passing
- **Commands:**
  ```bash
  cargo test --lib sub_workflow
  cargo test --test '*workflow*'
  ```

### QA-01-09: Code Step Executor
- **Plan Ref:** [Task 04](../../plans/01-core-execution-engine/tasks/04-step-executor.md)
- **Schema Ref:** Section 3 (Agentic Workflow - step_types: code step)
- **Priority:** P2
- **Test Type:** Unit + Integration
- **Criteria:**
  1. Code step executor for inline code execution
  2. Sandbox execution environment
  3. Code timeout handling
  4. Code output capture and return
  5. Security constraints (no network, limited filesystem)
  6. Code execution result types (success, error, output)
- **Commands:**
  ```bash
  cargo test --lib code_executor
  cargo test --test '*code*'
  ```

### QA-01-10: Loop Runner
- **Plan Ref:** [Task 05](../../plans/01-core-execution-engine/tasks/05-loop-runner.md)
- **Schema Ref:** Section 4 (Pipeline Definition - loops)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. All 6 loop types implemented: count, foreach, while, validation, retry, infinite
  2. Loop termination conditions enforced
  3. Loop state tracked (iterations, condition checks)
  4. Loop iteration limits respected
  5. Nested loops supported
  6. Loop convergence for validation/retry loops
  7. Loop break/continue handling
- **Commands:**
  ```bash
  cargo test --lib loop_runner
  cargo test --test '*loop*'
  ```

### QA-01-11: Branch Evaluator
- **Plan Ref:** [Task 06](../../plans/01-core-execution-engine/tasks/06-branch-evaluator.md)
- **Schema Ref:** Section 4 (Pipeline Definition - branches)
- **Priority:** P1
- **Test Type:** Unit + Integration
- **Criteria:**
  1. Conditional branching based on evaluation results
  2. Branch evaluation logic implemented
  3. Branch enable conditions supported
  4. Branch selection and execution
  5. Default branch behavior when no conditions met
  6. Event-driven branching support
  7. Conditional step skipping
- **Commands:**
  ```bash
  cargo test --lib branch_evaluator
  cargo test --test '*branch*'
  ```

### QA-01-12: Execution Modes
- **Plan Ref:** [Task 09](../../plans/01-core-execution-engine/tasks/09-execution-modes.md)
- **Schema Ref:** Section 5 (Workflow Execution Strategy - execution_modes)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. Serial execution mode: steps execute sequentially
  2. Hybrid execution mode: mixed serial/parallel strategy
  3. Parallel execution moved to agent-queue (not in Phase 1)
  4. Execution mode selection based on configuration
  5. Mode switching at runtime supported
  6. Execution mode persistence across restarts
  7. Concurrent execution limits enforced
- **Commands:**
  ```bash
  cargo test --lib execution_modes
  cargo test --test '*execution*'
  ```

### QA-01-13: Human Gating System
- **Plan Ref:** [Task 10](../../plans/01-core-execution-engine/tasks/10-human-gating.md)
- **Schema Ref:** Section 10 (Orchestration Configuration - human_gating)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. Operation classification: safe/risky/dangerous
  2. Confirmation prompts for risky/dangerous operations
  3. Diff preview and staging for changes
  4. Staged execution on confirmation
  5. Rollback on rejection
  6. Configurable gating policy (always-confirm, risky-only, never-confirm)
  7. Guardrails enforcement (path traversal, command injection prevention)
- **Commands:**
  ```bash
  cargo test --lib human_gating
  cargo test --test '*gating*'
  ```

### QA-01-14: Observability - Logging
- **Plan Ref:** [Task 11](../../plans/01-core-execution-engine/tasks/11-logging.md)
- **Schema Ref:** Section 7 (Logging Configuration - 9-level hierarchy)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. 9-level logging hierarchy implemented: Trace, Debug, Info, Warn, Error, Fatal, Panic, Off, Custom
  2. Per-scope configuration: global, workflow, step, agent, tool, backend, ui, system, automation, autonomy
  3. Output destinations: console, file, structured output
  4. Log format: JSON with timestamps, workflow_id, scope
  5. Log level filtering and routing
  6. Performance-sensitive logging enabled/disabled
  7. No log level violations (debug logs in release builds)
- **Commands:**
  ```bash
  cargo test --lib logging
  cargo test --test '*log*'
  ```

### QA-01-15: Observability - Metrics
- **Plan Ref:** [Task 11](../../plans/01-core-execution-engine/tasks/11-logging.md)
- **Schema Ref:** Section 8 (Metrics Configuration - pipeline_metrics, step_metrics, model_metrics, tool_metrics)
- **Priority:** P1
- **Test Type:** Unit + Integration
- **Criteria:**
  1. Pipeline-level metrics collected: duration, steps completed, errors
  2. Step-level metrics collected: execution time, retry count, output size
  3. Model-level metrics: token usage, cache hits/misses, load/unload operations
  4. Tool-level metrics: invocation count, execution time, success rate
  5. Custom metric collection API
  6. Metrics aggregation and statistical summaries
  7. Metrics export formats (JSON, Prometheus)
- **Commands:**
  ```bash
  cargo test --lib metrics
  cargo test --test '*metric*'
  ```

### QA-01-16: Retry & Error Handling
- **Plan Ref:** [Task 12](../../plans/01-core-execution-engine/tasks/12-retry-error-handling.md)
- **Schema Ref:** Section 5 (Workflow Execution Strategy - retry_policy, error_handling)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. Retry strategies: exponential, linear, fixed, none
  2. Backoff calculation with jitter
  3. Retry limit enforcement (max_attempts)
  4. Error escalation: transient → persistent → critical
  5. Graceful recovery mechanisms
  6. Error context preservation for debugging
  7. Exponential backoff: 2^n * base * multiplier + jitter
  **Commands:**
  ```bash
  cargo test --lib retry
  cargo test --test '*retry*'
  ```

---

## Verification Layers

### Layer 1: Unit Tests
- **Scope:** Individual functions, methods, and data structures in isolation
- **Evidence Required:** `cargo test --lib` output showing 100% pass rate
- **Validation Commands:**
  ```bash
  cargo test --lib
  ```
- **Pass Criteria:**
  - All 15 task groups pass
  - No panics in test output
  - Coverage >= 90% for new/modified code

### Layer 2: Integration Tests
- **Scope:** Module interactions, component communication, data flow between units
- **Evidence Required:** `cargo test --test '*'` output showing all integration tests pass
- **Validation Commands:**
  ```bash
  cargo test --test '*' -- --test-threads=1 --nocapture
  ```
- **Pass Criteria:**
  - All integration tests pass
  - No race conditions or deadlocks detected
  - Communication patterns match ADR specifications

### Layer 3: Property Tests
- **Scope:** Invariants that must hold for all possible inputs
- **Evidence Required:** Proptest output showing 1000 successful iterations
- **Validation Commands:**
  ```bash
  PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based
  ```
- **Pass Criteria:**
  - All property tests pass for 1000 iterations
  - No shrinking required (no failures)
  - Invariants cover state machine transitions

### Layer 4: End-to-End Tests
- **Scope:** Complete workflows from ChatSession creation to completion
- **Evidence Required:** Execution logs showing workflow completion
- **Validation Commands:**
  ```bash
  # Execute test workflows
  for workflow in test_workflows/*.yaml; do
    cargo run --bin agentsdk --run "$workflow"
  done
  ```
- **Pass Criteria:**
  - All phase-specific test workflows execute successfully
  - Output files exist and are non-empty
  - No memory leaks in long-running workflows

### Layer 5: Build Verification
- **Scope:** Full project compilation
- **Evidence Required:** Clean build output
- **Validation Commands:**
  ```bash
  cargo build --release
  ```
- **Pass Criteria:**
  - Successful build with exit code 0
  - No compilation errors
  - No warnings (or only justified warnings)

### Layer 6: Clippy / Code Quality
- **Scope:** All Rust source files
- **Evidence Required:** Zero clippy warnings
- **Validation Commands:**
  ```bash
  cargo clippy --all-features -- -W clippy::all
  ```
- **Pass Criteria:**
  - Zero warnings (or only justified warnings)
  - No unsafe code without comments
  - No `as any` type suppressions

---

## Success Criteria

Phase 1 is complete when:

1. **Queue & Scheduler:** All ChatSession, queue, and scheduler components implemented and tested
2. **Execution Engine:** All step executors (agent, tool, code, sub-workflow) working correctly
3. **Control Flow:** Loop runner and branch evaluator support all 6 loop types and conditional branching
4. **Human Gating:** Operation classification, confirmation prompts, and diff preview working
5. **Observability:** 9-level logging and comprehensive metrics collection functional
6. **Test Coverage:** All unit, integration, and property tests pass with >= 90% coverage
7. **Verification:** All 6 verification layers pass with zero errors
8. **ADR-0002 Compliance:** ChatSession work containers, human-gated safety, and CLI-first interface verified

---

## Anti-Goal-Drift Prevention

### Requirements Drift
- Verify implementation matches ADR-0002 exactly
- No features from later phases (3, 4, 5, 6, 7) implemented early

### Architecture Drift
- Queue state machine matches architecture diagrams
- Component boundaries respected: chat_session/, queue/, scheduler/, executor/, control_flow/
- No deviation from async tokio runtime pattern

### Performance Drift
- Queue scheduling latency < 100ms
- Step executor overhead < 50ms per step
- Metrics collection overhead < 5%

---

## Next Steps

Phase 1 complete when:
- ✅ All 15 QA areas validated with evidence
- ✅ All 6 verification layers pass
- ✅ ADR-0002 compliance verified
- ✅ Documentation updated

Proceed to [Phase 2 (CLI & LLM Backends)](../../phase-02/) implementation.

---

**Document Version:** 1.0.0
**Last Updated:** 2026-04-26
**Status:** Ready for QA Execution

---

## Related Plan Tasks

- [Task 00](../../plans/01-core-execution-engine/tasks/00-chatsession-containers.md) — ChatSession containers created with persistent IDs and lifecycle state management
- [Task 01](../../plans/01-core-execution-engine/tasks/01-queue-state-machine.md) — Queue state machine with valid state transitions and state machine is acyclic
- [Task 02](../../plans/01-core-execution-engine/tasks/02-persistent-queue-storage.md) — sled-based KV store for durable queue state with ACID transactions
- [Task 03](../../plans/01-core-execution-engine/tasks/03-scheduler-core.md) — Worker pool manages concurrent execution, job prioritization, cancellation support
- [Task 04](../../plans/01-core-execution-engine/tasks/04-step-executor.md) — Agent, tool, code, and sub-workflow step executors with tool permissions
- [Task 05](../../plans/01-core-execution-engine/tasks/05-loop-runner.md) — All 6 loop types implemented with termination conditions and convergence detection
- [Task 06](../../plans/01-core-execution-engine/tasks/06-branch-evaluator.md) — Conditional branching based on evaluation results with branch enable conditions
- [Task 09](../../plans/01-core-execution-engine/tasks/09-execution-modes.md) — Serial and hybrid execution modes with concurrency limits enforced
- [Task 10](../../plans/01-core-execution-engine/tasks/10-logging-framework.md) — 9-level logging hierarchy with scope configuration
- [Task 11](../../plans/01-core-execution-engine/tasks/11-metrics-collection.md) — Pipeline, step, model, and tool-level metrics collection
- [Task 12](../../plans/01-core-execution-engine/tasks/12-retry-error-handling.md) — Exponential, linear, and fixed retry strategies with jitter
