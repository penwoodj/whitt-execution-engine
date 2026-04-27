# Phase 01: Test Cases

### P01-001: ChatSession Lifecycle States
- **Area**: QA-01-01
- **Type**: Unit
- **Command**: `cargo test chat_session_lifecycle --lib`
- **Setup**: ChatSession creation, activation, execution, completion
- **Expected**: Session states transition correctly: created → activated → completed
- **Pass Criteria**: ✅ All state transitions valid, no invalid states

### P01-002: ChatSession Metadata Tracking
- **Area**: QA-01-01
- **Type**: Unit
- **Command**: `cargo test chat_session_metadata --lib`
- **Setup**: ChatSession with workflow ID, inputs, outputs
- **Expected**: Session metadata tracked correctly, retrieved on demand
- **Pass Criteria**: ✅ Metadata stored and retrieved accurately

### P01-003: Queue State Transitions
- **Area**: QA-01-02
- **Type**: Unit + Property
- **Command**: `cargo test queue_state_transitions --lib`
- **Setup**: Queue state machine with various transition triggers
- **Expected**: Only valid transitions allowed: pending→scheduled→running, running→paused, any→cancelled
- **Pass Criteria**: ✅ Invalid transitions rejected, valid transitions succeed

### P01-004: Queue State Cycle Detection
- **Area**: QA-01-02
- **Type**: Property
- **Command**: `PROPTEST_NUMBER_OF_TESTS=1000 cargo test queue_state_cycles --lib`
- **Setup**: Random state sequences
- **Expected**: No cycles detected in valid state transitions
- **Pass Criteria**: ✅ All 1000 iterations pass without cycles

### P01-005: Persistent Queue Storage Operations
- **Area**: QA-01-03
- **Type**: Integration
- **Command**: `cargo test queue_storage --test`
- **Setup**: Sled database, job metadata, execution history
- **Expected**: All operations atomic, job metadata persisted correctly
- **Pass Criteria**: ✅ Storage operations succeed, data persists across restarts

### P01-006: Queue Crash Recovery
- **Area**: QA-01-03
- **Type**: Integration
- **Command**: `cargo test queue_crash_recovery --test`
- **Setup**: Queue with active jobs, process crash simulation
- **Expected**: Queue recovers state after crash, jobs resume correctly
- **Pass Criteria**: ✅ Recovery succeeds, no data loss

### P01-007: Scheduler Job Prioritization
- **Area**: QA-01-04
- **Type**: Unit
- **Command**: `cargo test scheduler_prioritization --lib`
- **Setup**: Jobs with different priorities
- **Expected**: High-priority jobs executed before low-priority jobs
- **Pass Criteria**: ✅ Prioritization correct, fair scheduling maintained

### P01-008: Scheduler Worker Pool
- **Area**: QA-01-04
- **Type**: Integration
- **Command**: `cargo test scheduler_worker_pool --test`
- **Setup**: Worker pool with multiple workers, job queue
- **Expected**: Workers pull jobs correctly, no worker starvation
- **Pass Criteria**: ✅ All workers active, jobs distributed fairly

### P01-009: Scheduler Cancellation Support
- **Area**: QA-01-04
- **Type**: Integration
- **Command**: `cargo test scheduler_cancellation --test`
- **Setup**: Running job, cancellation request
- **Expected**: Job cancelled gracefully, worker cleanup executed
- **Pass Criteria**: ✅ Cancellation succeeds, resources released

### P01-010: Step Executor Interface
- **Area**: QA-01-05
- **Type**: Unit
- **Command**: `cargo test step_executor_interface --lib`
- **Setup**: Step executor trait with execute() method
- **Expected**: Interface defined correctly, all step types supported
- **Pass Criteria**: ✅ Trait methods accessible, step types distinguished

### P01-011: Agent Step LLM Inference
- **Area**: QA-01-06
- **Type**: Integration
- **Command**: `cargo test agent_step_inference --test`
- **Setup**: Agent step executor, mock LLM backend
- **Expected**: LLM inference call sent correctly, response handled
- **Pass Criteria**: ✅ Inference succeeds, response processed

### P01-012: Tool Step Execution
- **Area**: QA-01-07
- **Type**: Integration
- **Command**: `cargo test tool_step_execution --test`
- **Setup**: Tool step executor, built-in tool registry
- **Expected**: Tool invoked correctly, output captured
- **Pass Criteria**: ✅ Tool executes, output returned

### P01-013: Code Step Sandbox Execution
- **Area**: QA-01-09
- **Type**: Integration
- **Command**: `cargo test code_step_sandbox --test`
- **Setup**: Code step executor, sandbox enabled
- **Expected**: Code executed in sandbox, output captured
- **Pass Criteria**: ✅ Sandbox constraints enforced, code executes

### P01-014: Sub-Workflow Isolation
- **Area**: QA-01-08
- **Type**: Integration
- **Command**: `cargo test sub_workflow_isolation --test`
- **Setup**: Parent workflow calling sub-workflow
- **Expected**: Sub-workflow executes in isolated scope
- **Pass Criteria**: ✅ Isolation enforced, no variable leakage

### P01-015: Loop Runner - Count Loop
- **Area**: QA-01-10
- **Type**: Unit
- **Command**: `cargo test loop_count --lib`
- **Setup**: Count loop with max_iterations=5
- **Expected**: Loop iterates 5 times, iteration_variable updated
- **Pass Criteria**: ✅ Correct number of iterations, variable tracking

### P01-016: Loop Runner - Foreach Loop
- **Area**: QA-01-10
- **Type**: Unit
- **Command**: `cargo test loop_foreach --lib`
- **Setup**: Foreach loop with list of items
- **Expected**: Loop iterates over each item
- **Pass Criteria**: ✅ All items processed, loop terminates

### P01-017: Loop Runner - While Loop
- **Area**: QA-01-10
- **Type**: Unit
- **Command**: `cargo test loop_while --lib`
- **Setup**: While loop with condition
- **Expected**: Loop continues while condition true, stops when false
- **Pass Criteria**: ✅ Condition evaluated correctly, loop terminates

### P01-018: Loop Runner - Validation Loop
- **Area**: QA-01-10
- **Type**: Integration
- **Command**: `cargo test loop_validation --test`
- **Setup**: Validation loop with quality threshold
- **Expected**: Loop validates artifact quality, repairs until threshold met
- **Pass Criteria**: ✅ Validation triggers repair, convergence detected

### P01-019: Branch Evaluator Logic
- **Area**: QA-01-11
- **Type**: Unit
- **Command**: `cargo test branch_evaluator_logic --lib`
- **Setup**: Branch conditions, evaluation expressions
- **Expected**: Branch evaluates conditions correctly, routes to correct target
- **Pass Criteria**: ✅ All branches evaluated, routing correct

### P01-020: Serial Execution Mode
- **Area**: QA-01-12
- **Type**: Integration
- **Command**: `cargo test execution_serial --test`
- **Setup**: Workflow with serial mode
- **Expected**: Steps execute sequentially, dependencies respected
- **Pass Criteria**: ✅ Steps execute in order, no parallel execution

### P01-021: Hybrid Execution Mode
- **Area**: QA-01-12
- **Type**: Integration
- **Command**: `cargo test execution_hybrid --test`
- **Setup**: Workflow with hybrid mode (serial + parallel mix)
- **Expected**: Steps execute according to hybrid strategy
- **Pass Criteria**: ✅ Execution follows hybrid rules correctly

### P01-022: Human Gating - Safe Operations
- **Area**: QA-01-13
- **Type**: Unit
- **Command**: `cargo test human_gating_safe --lib`
- **Setup**: Operation classified as safe
- **Expected**: Safe operation executes without confirmation
- **Pass Criteria**: ✅ Safe operation bypasses confirmation prompt

### P01-023: Human Gating - Risky Operations
- **Area**: QA-01-13
- **Type**: Unit
- **Command**: `cargo test human_gating_risky --lib`
- **Setup**: Operation classified as risky
- **Expected**: Risky operation triggers confirmation prompt
- **Pass Criteria**: ✅ Confirmation prompt displayed, execution waits for approval

### P01-024: Human Gating - Dangerous Operations
- **Area**: QA-01-13
- **Type**: Unit
- **Command**: `cargo test human_gating_dangerous --lib`
- **Setup**: Operation classified as dangerous
- **Expected**: Dangerous operation triggers confirmation with diff preview
- **Pass Criteria**: ✅ Diff preview shown, execution requires approval

### P01-025: Logging Scope Hierarchy
- **Area**: QA-01-14
- **Type**: Unit
- **Command**: `cargo test logging_scope_hierarchy --lib`
- **Setup**: All 9 scopes with log levels
- **Expected**: Log scope hierarchy: global → workflow → step → agent → tool → backend → ui → system → automation
- **Pass Criteria**: ✅ All 9 scopes implemented correctly

### P01-026: Logging Output Destinations
- **Area**: QA-01-14
- **Type**: Integration
- **Command**: `cargo test logging_output --test`
- **Setup**: Logs configured for console and file output
- **Expected**: Logs written to console and file with correct format
- **Pass Criteria**: ✅ Logs appear in both destinations with correct structure

### P01-027: Metrics Collection - Pipeline Level
- **Area**: QA-01-15
- **Type**: Integration
- **Command**: `cargo test metrics_pipeline --test`
- **Setup**: Workflow execution
- **Expected**: Pipeline metrics collected: duration, steps completed, errors
- **Pass Criteria**: ✅ Pipeline metrics accurate and complete

### P01-028: Metrics Collection - Step Level
- **Area**: QA-01-15
- **Type**: Integration
- **Command**: `cargo test metrics_step --test`
- **Setup**: Step execution with various outcomes
- **Expected**: Step metrics collected: execution time, retry count, output size
- **Pass Criteria**: ✅ Step metrics tracked correctly for each step

### P01-029: Metrics Collection - Model Level
- **Area**: QA-01-15
- **Type**: Integration
- **Command**: `cargo test metrics_model --test`
- **Setup**: Model inference with token usage
- **Expected**: Model metrics collected: token usage, cache hits/misses, load/unload operations
- **Pass Criteria**: ✅ Model metrics accurate and complete

### P01-030: Retry - Exponential Backoff
- **Area**: QA-01-16
- **Type**: Unit
- **Command**: `cargo test retry_exponential --lib`
- **Setup**: Retry with exponential backoff, max_attempts=3
- **Expected**: Delays: 1s, 2s, 4s (base=1s, multiplier=2.0)
- **Pass Criteria**: ✅ Delays match exponential formula

### P01-031: Retry - Linear Backoff
- **Area**: QA-01-16
- **Type**: Unit
- **Command**: `cargo test retry_linear --lib`
- **Setup**: Retry with linear backoff, max_attempts=3
- **Expected**: Delays: 1s, 2s, 3s
- **Pass Criteria**: ✅ Linear backoff produces correct delays

### P01-032: Retry - Jitter
- **Area**: QA-01-16
- **Type**: Unit
- **Command**: `cargo test retry_jitter --lib`
- **Setup**: Retry with jitter=true, base_delay=1s
- **Expected**: Delay is base + random(0..base/10)
- **Pass Criteria**: ✅ Jitter adds randomness to delay

---

## Test Execution Order
**Phase 1 Group 1: ChatSession & Queue**
P01-001 → P01-002 → P01-003 → P01-004 → P01-005 → P01-006

**Phase 1 Group 2: Scheduler**
P01-007 → P01-008 → P01-009

**Phase 1 Group 3: Step Executor**
P01-010 → P01-011 → P01-012 → P01-013

**Phase 1 Group 4: Sub-Workflow**
P01-014

**Phase 1 Group 5: Loop Runner**
P01-015 → P01-016 → P01-017 → P01-018

**Phase 1 Group 6: Branch Evaluator**
P01-019

**Phase 1 Group 7: Execution Modes**
P01-020 → P01-021

**Phase 1 Group 8: Human Gating**
P01-022 → P01-023 → P01-024

**Phase 1 Group 9: Logging**
P01-025 → P01-026

**Phase 1 Group 10: Metrics**
P01-027 → P01-028 → P01-029

**Phase 1 Group 11: Retry**
P01-030 → P01-031 → P01-032
