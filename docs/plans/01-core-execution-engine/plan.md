# Phase 1: MVP Queue & Scheduler Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a persistent, fault-tolerant job queue and scheduler for executing agentic workflows with human-gated safety controls.

**Architecture:** Async Rust using tokio for runtime coordination, sled for durable storage, and tracing for observability. The scheduler manages ChatSession work containers through a state machine-based queue with staged execution and human confirmation gates.

**Tech Stack:** Rust 1.75+, tokio (async runtime, channels), sled (embedded KV store), tracing (structured logging), serde (serialization), thiserror (error handling), uuid (identifiers)

---

## Phase Overview

This phase implements the core execution engine for the AgentSDK: a persistent job queue and scheduler that manages ChatSession work containers with human-gated safety controls. The scheduler orchestrates workflow execution through step executors, loop runners, and branch evaluators (parallel execution moved to agent-queue), with comprehensive logging, metrics, and error handling.

**Estimated Time:** 8-10 weeks

**Dependencies:** Phase 0 must be complete (WorkflowSpec, WorkflowIR, ./workspace/ storage layer)

**ADR-0002 Compliance:**
- ChatSession as work container for all workflow execution
- Human-gated safety: operation classification, confirmation prompts, diff preview
- Staged diffs: preview changes before execution
- CLI-first interface for user interaction

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        CLI Interface                             │
│  (User submits workflows, confirms gates, monitors progress)     │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                      ChatSession Manager                           │
│  - Session lifecycle management (create, activate, close)        │
│  - ID generation and storage                                   │
│  - Session metadata tracking                                    │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Queue State Machine                           │
│  ┌────────┐     ┌──────────┐     ┌─────────┐                     │
│  │Pending │────▶│Scheduled │────▶│Running  │                     │
│  └────────┘     └──────────┘     └────┬────┘                     │
│     ▲                                  │                         │
│     │                        ┌─────────┴─────────┐               │
│     │                        │                   │               │
│  ┌──┴─────┐             ┌────▼────┐        ┌─────▼────┐          │
│  │Paused  │             │Completed│        │  Failed  │          │
│  └────────┘             └─────────┘        └──────────┘          │
│                                                   │                │
│                                             ┌─────▼────┐         │
│                                             │Cancelled │         │
│                                             └──────────┘         │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Persistent Queue Storage                     │
│  - sled-based KV store for durable queue state                   │
│  - Job metadata, execution history, checkpoints                 │
│  - Recovery and migration support                               │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Scheduler Core                            │
│  - Worker pool management (tokio tasks)                         │
│  - Job prioritization and scheduling                             │
│  - Execution loop with cancellation support                     │
│  - Retry with exponential backoff                               │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Execution Modes Engine                         │
│  ┌──────────┐                    ┌──────────────┐                 │
│  │  Serial  │  Parallel moved to │   Hybrid     │                 │
│  └──────────┘  └──────────┘  └──────────────┘                 │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Step Executors                                │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌─────────────┐ │
│  │Agent Step  │ │Tool Step   │ │Code Step   │ │Sub-Workflow │ │
│  │(LLM calls) │ │(Tool exec) │ │(Code exec) │ │Step         │ │
│  └────────────┘ └────────────┘ └────────────┘ └─────────────┘ │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Control Flow                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐           │
│  │ Loop Runner  │  │Branch Eval   │  │Parallel moved│           │
│  │ 6 loop types │  │Conditional   │  │to agent-queue│           │
│  │              │  │Event routing │  │             │           │
│  └──────────────┘  └──────────────┘  └──────────────┘           │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Human Gating System                            │
│  - Operation classification (safe/risky/dangerous)              │
│  - Confirmation prompts                                         │
│  - Diff preview and staged execution                             │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Observability Stack                              │
│  ┌────────────────┐  ┌──────────────────┐                       │
│  │Logging Framework│  │Metrics Collection │                       │
│  │9-level hierarchy│  │Pipeline/Step/    │                       │
│  │per-scope config │  │Model/Tool levels │                       │
│  └────────────────┘  └──────────────────┘                       │
│  ┌───────────────────────────────────────────┐                  │
│  │Retry & Error Handling                     │                  │
│  │Exponential/Linear/Fixed strategies         │                  │
│  │Error escalation, graceful recovery        │                  │
│  └───────────────────────────────────────────┘                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## File Structure

**New Files to Create:**

```
src/
  chat_session/
    mod.rs              # ChatSession module
    manager.rs          # Session lifecycle management
    types.rs            # Session structs and enums
  queue/
    mod.rs              # Queue module
    state.rs            # Queue state machine
    storage.rs          # sled-based persistent storage
    job.rs              # Job metadata and execution state
  scheduler/
    mod.rs              # Scheduler module
    core.rs             # Worker pool, job prioritization, execution loop
    worker.rs           # Individual worker task
    cancel.rs           # Cancellation handling
  executor/
    mod.rs              # Executor module
    step.rs             # Step execution interface
    agent.rs            # Agent step executor (LLM calls - trait based)
    tool.rs             # Tool step executor
    code.rs             # Code step executor
    workflow.rs         # Sub-workflow step executor
  control_flow/
    mod.rs              # Control flow module
    loop.rs             # Loop runner (6 loop types)
    branch.rs           # Branch evaluator
    # parallel.rs         # Parallel executor (moved to agent-queue)
  execution_mode/
    mod.rs              # Execution mode module
    serial.rs           # Serial execution
    # parallel.rs         # Parallel execution (moved to agent-queue)
    hybrid.rs           # Hybrid execution strategy
  safety/
    mod.rs              # Safety/gating module
    classify.rs         # Operation classification
    prompt.rs           # Confirmation prompts
    diff.rs             # Diff preview and staging
  observability/
    mod.rs              # Observability module
    logging.rs          # 9-level logging framework
    metrics.rs          # Metrics collection
    retry.rs            # Retry strategies and error handling

tests/
  integration/
    queue_test.rs      # Queue state machine tests
    scheduler_test.rs   # Scheduler integration tests
    executor_test.rs    # Executor pipeline tests
    human_gating_test.rs # Human gating tests
  unit/
    chat_session_test.rs
    queue_storage_test.rs
    loop_runner_test.rs
    branch_evaluator_test.rs
    # parallel_executor_test.rs (moved to agent-queue)
    retry_test.rs
  property/
    state_machine_invariants_test.rs
    retry_convergence_test.rs
```

---

## Schema Domain Ownership

This phase implements the following sections of the schema:

### Section 3: Agentic Workflow
- **steps**: Workflow steps with step_type, inputs, outputs
- **user_inputs**: User-defined input specifications
- **input_types**: Supported input type definitions
- **validation**: Input validation rules

### Section 4: Pipeline Definition
- **step_types**: Agent, Tool, Code, Sub-workflow step types
- **dependencies**: Step dependency specifications
- **branches**: Conditional branching definitions
- **loops**: All 6 loop types (count, foreach, while, validation, retry, infinite)
- # **parallel_groups**: Parallel execution group definitions (moved to agent-queue)

### Section 5: Workflow Execution Strategy (partial)
- **execution_modes**: Serial, hybrid execution strategies (parallel moved to agent-queue)
- **retry_policy**: Retry configuration with backoff strategies
- **error_handling**: Error escalation and recovery mechanisms
- **checkpointing**: Execution checkpoint configuration
- # **concurrency_limits**: Worker pool and parallel group concurrency (moved to agent-queue)
- **synchronization**: Step coordination mechanisms

### Section 7: Logging Configuration
- **log_levels**: 9-level hierarchy (Trace, Debug, Info, Warn, Error, Fatal, Panic, Off, Custom)
- **scope_config**: Per-scope logging configuration
- **output_destinations**: Console, file, structured output

### Section 8: Metrics Configuration
- **pipeline_metrics**: Pipeline-level metrics collection
- **step_metrics**: Step-level execution metrics
- **model_metrics**: Model usage metrics (placeholders for Phase 2)
- **tool_metrics**: Tool execution metrics
- **custom_metrics**: User-defined metric collection

### Section 9: Validation Configuration
- **schema_validation**: Schema validation rules
- **step_output_validation**: Step output validation
- **business_rules**: Custom business rule validation

### Section 10: Orchestration Configuration (partial)
- **step_coordination**: Step ordering and dependency management
- **sub_agents**: Sub-agent coordination (trait-based, LLM backend in Phase 2)

---

## Key Concepts

### ChatSession

A ChatSession is the work container for all workflow execution. It provides:
- Persistent ID for tracking execution across runs
- Storage for workflow metadata, inputs, and outputs
- Execution state tracking (queued, running, paused, completed)
- Human interaction history (confirmations, approvals)
- Checkpoint data for resumption after failure

**Lifecycle:**
1. Created when user submits workflow to queue
2. Activated when scheduler starts execution
3. Updated continuously as steps execute
4. Completed when workflow finishes (success or failure)
5. Closed after user confirms results

### Queue State Machine

Jobs transition through these states:

```
pending → scheduled → running → completed/failed/cancelled/timeout
    ↑                                ↑
    └──────────────paused────────────┘
```

**State Descriptions:**
- `pending`: Job created, waiting for scheduler
- `scheduled`: Job assigned to worker, waiting for execution slot
- `running`: Job actively executing steps
- `paused`: Job paused by user or system (can resume)
- `completed`: Job finished successfully
- `failed`: Job failed after retry attempts exhausted
- `cancelled`: Job cancelled by user
- `timeout`: Job exceeded maximum execution time

**Transition Rules:**
- pending → scheduled: When scheduler assigns worker
- scheduled → running: When worker starts execution
- running → paused: When user pauses or system pauses for confirmation
- paused → running: When user resumes execution
- running → completed: When all steps finish successfully
- running → failed: When error cannot be recovered after retries
- running → cancelled: When user cancels or human gate rejected
- running → timeout: When execution time exceeds configured limit
- Any state → cancelled: When user requests cancellation

### Human Gating

Operations are classified by risk level:
- **safe**: Operations with no external impact (e.g., reading files, processing data)
- **risky**: Operations with external side effects (e.g., writing files, API calls)
- **dangerous**: Operations with potentially destructive impact (e.g., rm, system changes)

**Gating Process:**
1. Classifier analyzes operation type and inputs
2. If risky or dangerous, present diff preview to user
3. User confirms or rejects operation
4. If confirmed, execute staged changes
5. If rejected, skip operation or fail workflow (configurable)

### Staged Diffs

Before executing operations with external impact:
1. Generate diff of planned changes
2. Present to user for review
3. Stage changes in temporary location
4. On confirmation, atomically apply staged changes
5. Rollback on failure or rejection

---

## Verification Layers

  1. **State Machine Validation**: Ensure all state transitions are valid and complete
  2. **Storage Consistency**: Verify sled storage operations are atomic and recoverable
  3. **Execution Correctness**: Test that steps execute in correct order with proper inputs
  4. **Concurrency Safety**: Verify worker pool is race-free (parallel execution moved to agent-queue)
  5. **Resource Management**: Ensure channels, tasks, and storage are properly cleaned up
  6. **Error Recovery**: Test retry logic, error handling, and graceful degradation
  7. **Human Gating**: Verify classification accuracy and gate enforcement

---

## Mock Strategies

### Mock Sled Storage
- Use in-memory sled database for unit tests
- Mock sled operations to simulate failure scenarios
- Test recovery and migration paths

### Mock Tokio Channels
- Use bounded channels for queue events
- Mock channel operations to test backpressure and cancellation
- Test with channel closure and sender drop scenarios

### Mock LLM Backend
- Define trait for LLM operations (used in Phase 2)
- Create mock implementations returning canned responses
- Test step executor logic without actual LLM calls

---

## Task Sequence

**Core Foundation (Week 1-2)**
- Task 00: ChatSession Containers
- Task 01: Queue State Machine
- Task 02: Persistent Queue Storage

**Scheduler (Week 3-4)**
- Task 03: Scheduler Core

**Execution Engine (Week 5-6)**
- Task 04: Step Executor
- Task 05: Loop Runner
- Task 06: Branch Evaluator
# - Task 07: Parallel Executor (moved to agent-queue)

**Safety & Controls (Week 7)**
- Task 08: Human Gating
- Task 09: Execution Modes

**Observability (Week 8-9)**
- Task 10: Logging Framework
- Task 11: Metrics Collection
- Task 12: Retry & Error Handling

**Integration & Testing (Week 10)**
- Integration tests
- End-to-end validation

---

## Execution Approach

This plan should be executed using `superpowers:executing-plans` with batch execution and checkpoints after each task group:

**Checkpoint 1:** After Task 02 (Storage layer complete)
**Checkpoint 2:** After Task 03 (Scheduler core complete)
**Checkpoint 3:** After Task 07 (Execution engine complete)
**Checkpoint 4:** After Task 09 (Safety & controls complete)
**Checkpoint 5:** After Task 12 (Full Phase 1 complete)

At each checkpoint:
1. Run all unit tests in completed task groups
2. Run integration tests for completed components
3. Verify acceptance criteria for checkpoint
4. Create git tag for checkpoint milestone

---

## Testing Strategy

### Unit Tests
- Test each component in isolation with mocked dependencies
- Use property-based testing for state machine invariants
- Cover all state transitions and edge cases

### Integration Tests
- Test queue → scheduler → executor pipeline
- Test human gating with mock prompts
- Test recovery from failure scenarios

### Property Tests
- Verify state machine invariants (no invalid states or transitions)
- Verify retry convergence (eventually succeeds or exhausts retries)
- Verify loop termination conditions

---

## Dependencies

**Phase 0 Deliverables (must be complete):**
- `src/spec.rs`: WorkflowSpec struct with YAML schema definitions
- `src/ir.rs`: WorkflowIR struct with validated, typed representation
- `src/parser.rs`: YAML parser from WorkflowSpec to WorkflowIR
- `src/storage.rs`: ./workspace/ persistence layer for workflow files
- `src/interpolation.rs`: Variable interpolation for workflow inputs

**External Dependencies:**
- tokio 1.35+: Async runtime, tasks, channels, time
- sled 0.34+: Embedded KV store for persistent storage
- tracing 0.1.40+: Structured logging with subscriber ecosystem
- tracing-subscriber 0.3.18+: Logging subscriber implementation
- serde 1.0+: Serialization/deserialization
- serde_json 1.0+: JSON support for metrics and logs
- thiserror 1.0.56+: Error handling
- uuid 1.7+: Unique identifier generation
- chrono 0.4+: Time handling for timestamps and timeouts
- futures 0.3+: Future utilities
- async-trait 0.1+: Async trait support

---

## Success Criteria

Phase 1 is complete when:
1. All 12 tasks are implemented and tested
2. All unit tests pass (target: 90%+ coverage)
3. All integration tests pass
4. All property tests pass
5. Checkpoint criteria validated at each checkpoint
6. Acceptance criteria validated at Phase 1 completion
7. At least 10 of the 53 example workflows can execute end-to-end
8. Human gating system correctly classifies and gates operations
9. Persistent storage survives process crashes and recovers state
10. Graceful shutdown with proper cleanup of all resources

---

## Reference Documentation

- ADR-0002: Human-Gated Safety & CLI-First Design
- Workflow Schema: sections 3, 4, 5(partial), 7, 8, 9, 10(partial)
- Phase 0 deliverables: WorkflowSpec, WorkflowIR, storage layer
- Example workflows: requirements-oriented-auto/ directory (53 workflows)

---

## Next Steps

Proceed to task files in `tasks/` directory for detailed implementation instructions. Each task file contains:
- File paths to create/modify
- Complete Rust struct/enum definitions
- Trait signatures for integration with Phase 0
- Step-by-step implementation with checkboxes
- Test specifications
- Mock strategies

Start with **Task 00: ChatSession Containers** to establish the work container foundation.
