# Phase 01: MVP Queue - Validation Criteria

**Phase Focus:** Chat session lifecycle, queue state machine, scheduler execution
**Entry Criteria:** Phase 00 complete
**Estimated Duration:** 3-4 weeks
**Blocking for:** Phases 02, 03, 04, 05, 06, 07, 08

---

## Phase Overview

Phase 01 implements the core execution engine: chat session lifecycle, queue state machine, and scheduler. This phase enables workflows to be parsed, queued, and executed. The scheduler handles serial, parallel, and hybrid execution modes, with support for loops, branches, retry logic, and human gating.

**Critical Success Factors:**
1. Correct state machine transitions (no invalid states)
2. All 6 loop types terminate correctly (no infinite loops)
3. Retry logic applies exponential backoff
4. Human gating blocks dangerous operations
5. All 9 logging scopes emit structured logs
6. Metrics collection for all execution components

---

## Phase Entry Criteria

**Status:** Must be verified before starting Phase 01

**Verification Commands:**
```bash
# Verify Phase 00 exit
cargo test --lib phase_00_tests -- --test-threads=1

# Verify Phase 00 schema coverage
cargo run --bin schema_audit -- --phase 0 --output phase_00_coverage.md
# Expected: 100% coverage

# Verify Phase 00 ADR compliance
cargo run --bin adr_compliance -- --phase 0
# Expected: All constraints satisfied
```

**Prerequisites:**
- [ ] Phase 00 exit criteria verified (all 7 layers)
- [ ] Phase 00 schema coverage 100%
- [ ] Phase 00 ADR compliance verified
- [ ] Phase 00 cross-phase regression clean
- [ ] Phase 00 anti-goal-drift checklist complete
- [ ] Test infrastructure ready for execution testing
- [ ] Mock backends prepared for integration testing

**Blocking Violations:**
- Unresolved Phase 00 failures
- Schema coverage < 100%
- ADR compliance violations

---

## ChatSession Lifecycle

**Requirement:** ChatSession lifecycle works correctly

### Lifecycle States

1. **Created**: Chat session initialized, not started
2. **Queued**: Workflow enqueued for execution
3. **Running**: Workflow actively executing
4. **Paused**: Workflow paused by user or system
5. **Completed**: Workflow finished successfully
6. **Failed**: Workflow failed with error
7. **Cancelled**: Workflow cancelled by user
8. **TimedOut**: Workflow exceeded time limit

### Lifecycle Transitions

```
Created → Queued (enqueue_workflow)
Queued → Running (scheduler picks up workflow)
Running → Paused (pause command)
Paused → Running (resume command)
Running → Completed (all steps succeeded)
Running → Failed (step failed, no retry)
Running → Cancelled (cancel command)
Running → TimedOut (time limit exceeded)
Failed → Queued (retry workflow)
```

### Verification Commands

```bash
# Test lifecycle transitions
cargo test --lib lifecycle::tests::creation
cargo test --lib lifecycle::tests::enqueue
cargo test --lib lifecycle::tests::running
cargo test --lib lifecycle::tests::pause_resume
cargo test --lib lifecycle::tests::completion
cargo test --lib lifecycle::tests::failure
cargo test --lib lifecycle::tests::cancellation
cargo test --lib lifecycle::tests::timeout

# Test invalid transitions (should fail)
cargo test --lib lifecycle::tests::invalid_transitions
```

### Test Cases

**Valid Lifecycle:**
1. Create workflow
2. Enqueue workflow
3. Start workflow (Running state)
4. Complete all steps (Completed state)
5. Retrieve results

**Invalid Transition:**
- Direct transition from Created to Running (should fail)
- Transition from Completed to Running (should fail)

### Pass Criteria

- [ ] All lifecycle state transitions work correctly
- [ ] Invalid transitions rejected with clear error messages
- [ ] State persisted correctly across transitions
- [ ] Lifecycle events logged with correct scope

### Evidence Required

- Lifecycle transition test results
- Invalid transition error messages
- Lifecycle event logs (system scope)

---

## Queue State Machine

**Requirement:** Queue state machine covers all states

### Queue States

1. **Idle**: Queue empty, no workflows
2. **Filling**: Workflows being enqueued
3. **Full**: Queue at capacity
4. **Dispatching**: Scheduler dispatching workflows
5. **Error**: Queue error state

### State Machine Diagram

```
      ┌─────────┐
      │  Idle   │
      └────┬────┘
           │ enqueue
           ▼
      ┌─────────┐
      │ Filling │◄─────────┐
      └────┬────┘          │
           │ full          │ complete
           ▼               │
      ┌─────────┐          │
      │   Full  │          │
      └────┬────┘          │
           │ dispatch      │
           ▼               │
      ┌─────────┐          │
      │Dispatching│────────┘
      └────┬────┘
           │ error
           ▼
      ┌─────────┐
      │  Error  │─────┐
      └────┬────┘     │
           │ recover  │
           ▼          │
      ┌─────────┐     │
      │  Idle   │─────┘
      └─────────┘
```

### Verification Commands

```bash
# Test queue state machine
cargo test --lib queue::tests::idle_state
cargo test --lib queue::tests::filling_state
cargo test --lib queue::tests::full_state
cargo test --lib queue::tests::dispatching_state
cargo test --lib queue::tests::error_state

# Test state transitions
cargo test --lib queue::tests::state_transitions

# Property-based testing for state machine invariants
PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib queue::tests::invariants
```

### Invariants to Test

1. **Single State Invariant:** Queue always in exactly one state
2. **Transition Validity Invariant:** Only valid transitions allowed
3. **Capacity Invariant:** Queue never exceeds capacity
4. **Error Recovery Invariant:** Queue can recover from error state

### Pass Criteria

- [ ] All queue states reachable and verifiable
- [ ] State transitions follow state machine diagram
- [ ] Property tests hold for 1000 iterations
- [ ] Capacity limits enforced correctly

### Evidence Required

- Queue state machine test results
- State transition logs
- Property test results
- Capacity limit enforcement logs

---

## Scheduler Execution Modes

**Requirement:** Scheduler executes serial, parallel, and hybrid modes

### Serial Execution

**Definition:** Steps execute one after another

**Configuration:**
```yaml
pipeline:
  execution_mode: serial
  steps:
    - name: step_1
    - name: step_2
    - name: step_3
```

**Execution Order:** step_1 → step_2 → step_3

### Parallel Execution

**Definition:** Steps execute simultaneously (limited by concurrency)

**Configuration:**
```yaml
pipeline:
  execution_mode: parallel
  max_concurrency: 4
  steps:
    - name: step_1
    - name: step_2
    - name: step_3
    - name: step_4
```

**Execution Order:** All steps start simultaneously (up to max_concurrency)

### Hybrid Execution

**Definition:** Serial and parallel execution mixed

**Configuration:**
```yaml
pipeline:
  execution_mode: hybrid
  steps:
    - name: setup_step  # Serial
    - group: parallel_group
      steps:
        - name: parallel_step_1  # Parallel
        - name: parallel_step_2  # Parallel
    - name: cleanup_step  # Serial
```

**Execution Order:**
1. setup_step (serial)
2. parallel_step_1, parallel_step_2 (parallel)
3. cleanup_step (serial)

### Verification Commands

```bash
# Test serial execution
cargo test --lib scheduler::tests::serial_execution

# Test parallel execution
cargo test --lib scheduler::tests::parallel_execution

# Test hybrid execution
cargo test --lib scheduler::tests::hybrid_execution

# Test concurrency limits
cargo test --lib scheduler::tests::concurrency_limits
```

### Pass Criteria

- [ ] Serial execution respects step order
- [ ] Parallel execution respects concurrency limits
- [ ] Hybrid execution mixes serial/parallel correctly
- [ ] No race conditions in parallel execution
- [ ] Resource limits enforced (CPU, memory)

### Evidence Required

- Serial execution test logs (verify order)
- Parallel execution test logs (verify concurrency)
- Hybrid execution test logs (verify mixed mode)
- Concurrency limit enforcement logs

---

## Loop Termination

**Requirement:** All 6 loop types terminate correctly

### Loop Types

#### 1. Fixed Count Loop

**Definition:** Execute N times

**Configuration:**
```yaml
loop:
  type: fixed_count
  iterations: 5
  body:
    - name: iteration_step
```

**Expected Behavior:** Execute iteration_step exactly 5 times

#### 2. Conditional Loop

**Definition:** Execute while condition true

**Configuration:**
```yaml
loop:
  type: conditional
  condition: ${result.score} < 0.9
  body:
    - name: improve_step
```

**Expected Behavior:** Execute improve_step until condition false

#### 3. Iterator Loop

**Definition:** Iterate over collection

**Configuration:**
```yaml
loop:
  type: iterator
  variable: item
  collection: ${items}
  body:
    - name: process_item_step
```

**Expected Behavior:** Execute process_item_step for each item in collection

#### 4. Retry Loop

**Definition:** Retry failed operation

**Configuration:**
```yaml
loop:
  type: retry
  max_attempts: 3
  backoff_ms: 1000
  body:
    - name: retry_step
```

**Expected Behavior:** Execute retry_step up to 3 times with backoff

#### 5. Convergence Loop

**Definition:** Execute until convergence criteria met

**Configuration:**
```yaml
loop:
  type: convergence
  convergence_criteria:
    - metric: score
      threshold: 0.95
    - metric: stability
      threshold: 0.98
  body:
    - name: iterate_step
```

**Expected Behavior:** Execute iterate_step until both metrics meet thresholds

#### 6. Timeout Loop

**Definition:** Execute until time limit

**Configuration:**
```yaml
loop:
  type: timeout
  timeout_ms: 30000
  body:
    - name: long_running_step
```

**Expected Behavior:** Execute long_running_step until 30 seconds elapsed

### Termination Verification

**For each loop type:**

```bash
# Test termination
cargo test --lib loops::tests::fixed_count_termination
cargo test --lib loops::tests::conditional_termination
cargo test --lib loops::tests::iterator_termination
cargo test --lib loops::tests::retry_termination
cargo test --lib loops::tests::convergence_termination
cargo test --lib loops::tests::timeout_termination

# Test infinite loop prevention
cargo test --lib loops::tests::infinite_loop_prevention
```

### Pass Criteria

- [ ] All 6 loop types terminate correctly
- [ ] No infinite loops (all loops respect limits)
- [ ] Loop variables accessible in loop body
- [ ] Loop results aggregated correctly

### Evidence Required

- Loop termination test results for all 6 types
- Infinite loop prevention test results
- Loop variable access test results
- Loop aggregation test results

---

## Branch Evaluation

**Requirement:** Branches evaluate conditions correctly

### Branch Types

#### 1. If-Else Branch

**Configuration:**
```yaml
branch:
  condition: ${result.score} >= 0.8
  true_branch:
    - name: high_score_step
  false_branch:
    - name: low_score_step
```

**Expected Behavior:**
- If condition true → execute high_score_step
- If condition false → execute low_score_step

#### 2. Switch Branch

**Configuration:**
```yaml
branch:
  type: switch
  expression: ${result.category}
  cases:
    - value: "A"
      steps:
        - name: category_a_step
    - value: "B"
      steps:
        - name: category_b_step
  default:
    - name: default_step
```

**Expected Behavior:**
- If expression == "A" → execute category_a_step
- If expression == "B" → execute category_b_step
- Otherwise → execute default_step

#### 3. Multi-Condition Branch

**Configuration:**
```yaml
branch:
  conditions:
    - condition: ${result.score} >= 0.9
      steps:
        - name: excellent_step
    - condition: ${result.score} >= 0.7
      steps:
        - name: good_step
    - condition: ${result.score} >= 0.5
      steps:
        - name: average_step
  default:
    - name: poor_step
```

**Expected Behavior:** Execute first matching condition (top-down)

### Verification Commands

```bash
# Test branch evaluation
cargo test --lib branches::tests::if_else_branch
cargo test --lib branches::tests::switch_branch
cargo test --lib branches::tests::multi_condition_branch

# Test branch conditions
cargo test --lib branches::tests::condition_evaluation
```

### Pass Criteria

- [ ] If-else branch evaluates conditions correctly
- [ ] Switch branch matches values correctly
- [ ] Multi-condition branch uses first match
- [ ] Branch conditions access variables correctly
- [ ] Default branch executed when no match

### Evidence Required

- Branch evaluation test results
- Condition evaluation test results
- Variable access test results
- Branch execution logs

---

## Retry Backoff

**Requirement:** Retry applies exponential backoff correctly

### Retry Configuration

```yaml
retry:
  max_attempts: 5
  backoff_strategy: exponential
  initial_backoff_ms: 1000
  max_backoff_ms: 30000
  backoff_multiplier: 2
```

### Backoff Schedule

| Attempt | Backoff (ms) | Calculation |
|---------|--------------|-------------|
| 1 (initial) | 0 | No backoff on first attempt |
| 2 | 1000 | initial_backoff_ms |
| 3 | 2000 | 1000 * 2 |
| 4 | 4000 | 2000 * 2 |
| 5 | 8000 | 4000 * 2 |

### Verification Commands

```bash
# Test retry backoff
cargo test --lib retry::tests::exponential_backoff
cargo test --lib retry::tests::linear_backoff
cargo test --lib retry::tests::max_backoff_limit
cargo test --lib retry::tests::retry_on_specific_errors
```

### Pass Criteria

- [ ] Exponential backoff applied correctly
- [ ] Backoff never exceeds max_backoff_ms
- [ ] Retry only on specified errors
- [ ] Max attempts respected (no infinite retry)

### Evidence Required

- Retry backoff test results
- Backoff timing measurements
- Error-specific retry test results

---

## Human Gating

**Requirement:** Human gating blocks dangerous operations

### Dangerous Operations

1. File system operations (delete, overwrite)
2. Network operations (external API calls)
3. System operations (process management)
4. Resource-intensive operations (large computations)

### Gating Behavior

**Without Approval:**
```yaml
step:
  name: delete_files
  type: dangerous
  requires_approval: true
  operation: delete
  target: /important/data
```

**Expected Behavior:** Block operation, request human approval

**With Approval:**
```yaml
step:
  name: delete_files
  type: dangerous
  requires_approval: true
  operation: delete
  target: /important/data
  approved_by: user@example.com
  approval_timestamp: "2026-04-06T10:00:00Z"
```

**Expected Behavior:** Execute operation

### Verification Commands

```bash
# Test human gating
cargo test --lib human_gating::tests::block_dangerous_ops
cargo test --lib human_gating::tests::allow_approved_ops
cargo test --lib human_gating::tests::approval_workflow
cargo test --lib human_gating::tests::approval_timeout
```

### Pass Criteria

- [ ] Dangerous operations blocked without approval
- [ ] Approved operations execute successfully
- [ ] Approval workflow tracks approver and timestamp
- [ ] Approval requests expire after timeout

### Evidence Required

- Human gating test results
- Approval workflow logs
- Timeout test results

---

## Logging Coverage

**Requirement:** Logging outputs to all 9 scopes

### Logging Scopes

1. **pipeline**: Pipeline-level events (start, complete, error)
2. **step**: Step-level events (start, complete, error)
3. **model**: Model interactions (requests, responses, tokens)
4. **tool**: Tool invocations (start, complete, output)
5. **backend**: Backend connections (connect, disconnect, error)
6. **ui**: UI events (user actions, state changes)
7. **system**: System events (startup, shutdown, errors)
8. **automation**: Automation triggers and executions
9. **autonomy**: Autonomous decisions and overrides

### Log Structure

```json
{
  "timestamp": "2026-04-06T10:00:00Z",
  "scope": "pipeline",
  "level": "info",
  "workflow_id": "uuid",
  "pipeline_id": "pipeline_1",
  "message": "Pipeline started",
  "metadata": {
    "step_count": 5,
    "execution_mode": "serial"
  }
}
```

### Verification Commands

```bash
# Verify all 9 scopes emit logs
for scope in pipeline step model tool backend ui system automation autonomy; do
  echo "Testing $scope scope..."
  cargo run --bin agentsdk -- run examples/workflows/test.yaml 2>&1 | \
    jq -e "select(.scope == \"$scope\")" || exit 1
done
```

### Pass Criteria

- [ ] All 9 scopes emit structured logs
- [ ] Logs include timestamp, scope, workflow_id, message
- [ ] Log levels appropriate (debug, info, warn, error)
- [ ] No required fields missing

### Evidence Required

- Log samples from all 9 scopes
- Log structure validation results
- Required field checklist

---

## Metrics Collection

**Requirement:** Metrics collect pipeline, step, model, and tool data

### Metrics Categories

#### Pipeline Metrics

- `pipeline_duration_seconds`: Pipeline execution time
- `pipeline_step_count`: Number of steps executed
- `pipeline_success`: Pipeline success (1) or failure (0)

#### Step Metrics

- `step_duration_seconds`: Step execution time
- `step_retry_count`: Number of retries
- `step_success`: Step success (1) or failure (0)

#### Model Metrics

- `model_request_count`: Number of model requests
- `model_response_tokens`: Total response tokens
- `model_request_tokens`: Total request tokens
- `model_latency_seconds`: Model request latency

#### Tool Metrics

- `tool_invocation_count`: Number of tool invocations
- `tool_success_count`: Number of successful tool calls
- `tool_failure_count`: Number of failed tool calls

### Metrics Export

**Prometheus Format:**
```
agentsdk_pipeline_duration_seconds{workflow_id="uuid",pipeline_id="pipeline_1"} 1.234
agentsdk_step_duration_seconds{workflow_id="uuid",step_id="step_1"} 0.567
agentsdk_model_response_tokens{workflow_id="uuid",model_id="gpt-4"} 1500
agentsdk_tool_invocation_count{workflow_id="uuid",tool_id="search"} 3
```

### Verification Commands

```bash
# Verify metrics export
curl -s http://localhost:9090/metrics | grep agentsdk

# Test specific metrics
curl -s http://localhost:9090/metrics | grep agentsdk_pipeline_duration_seconds
curl -s http://localhost:9090/metrics | grep agentsdk_step_duration_seconds
curl -s http://localhost:9090/metrics | grep agentsdk_model_response_tokens
curl -s http://localhost:9090/metrics | grep agentsdk_tool_invocation_count
```

### Pass Criteria

- [ ] All pipeline metrics collected
- [ ] All step metrics collected
- [ ] All model metrics collected
- [ ] All tool metrics collected
- [ ] Metrics exported in Prometheus format

### Evidence Required

- Metrics export samples
- Metric completeness checklist
- Prometheus format validation results

---

## Phase Exit Criteria

### Layer 1: Unit Tests

**Commands:**
```bash
cargo test --lib -- --test-threads=1
```

**Evidence:**
- All Phase 01 unit tests pass
- Test coverage >= 90%
- No clippy warnings

### Layer 2: Integration Tests

**Commands:**
```bash
cargo test --test '*' -- --test-threads=1
```

**Evidence:**
- All Phase 01 integration tests pass
- Scheduler correctly manages workflows
- No deadlocks or race conditions

### Layer 3: Property Tests

**Commands:**
```bash
PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based
```

**Evidence:**
- State machine invariants hold for 1000 iterations
- Loop termination invariants hold
- No shrinking required

### Layer 4: E2E Tests

**Commands:**
```bash
# Execute workflows with scheduler
cargo run --bin agentsdk -- run examples/workflows/serial_workflow.yaml
cargo run --bin agentsdk -- run examples/workflows/parallel_workflow.yaml
cargo run --bin agentsdk -- run examples/workflows/hybrid_workflow.yaml
cargo run --bin agentsdk -- run examples/workflows/loop_workflow.yaml
cargo run --bin agentsdk -- run examples/workflows/branch_workflow.yaml
```

**Evidence:**
- All example workflows execute successfully
- Scheduler respects execution mode
- Loops terminate correctly
- Branches evaluate correctly

### Layer 5: System Log Validation

**Commands:**
```bash
for scope in pipeline step model tool backend ui system automation autonomy; do
  echo "Testing $scope scope..."
  cargo run --bin agentsdk -- run examples/workflows/test.yaml 2>&1 | \
    jq -e "select(.scope == \"$scope\")" || exit 1
done
```

**Evidence:**
- All 9 scopes emit logs
- Logs include required fields
- Log levels appropriate

### Layer 6: Live CLI Verification

**Commands:**
```bash
# Test queue commands
cargo run --bin agentsdk -- queue list
cargo run --bin agentsdk -- queue enqueue examples/workflows/test.yaml
cargo run --bin agentsdk -- queue cancel <workflow_id>

# Test scheduler commands
cargo run --bin agentsdk -- scheduler start
cargo run --bin agentsdk -- scheduler stop
cargo run --bin agentsdk -- scheduler status
```

**Evidence:**
- All queue commands work
- Scheduler commands work
- Error handling works for invalid commands

### Layer 7: Benchmark Performance

**Commands:**
```bash
cargo bench --bench phase_01_benchmarks
```

**Evidence:**
- Scheduler performance within baseline
- Queue operations performant
- No performance regression > 10%

---

## Cross-Phase Regression Tests

**Commands:**
```bash
# Run Phase 00 tests
cargo test --test phase_00_integration -- --test-threads=1

# Run Phase 00 property tests
PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib phase_00_property_tests

# Verify Phase 00 schema coverage
cargo run --bin schema_audit -- --phase 0 --output phase_00_regression.md
```

**Pass Criteria:**
- [ ] All Phase 00 unit tests still pass
- [ ] All Phase 00 integration tests still pass
- [ ] All Phase 00 property tests still hold
- [ ] Phase 00 schema coverage still 100%

---

## Schema Coverage Audit

**Requirement:** 100% of Phase 01-owned fields implemented

### Phase 01-Owned Fields

**RetrySchema:**
- max_attempts
- backoff_strategy
- initial_backoff_ms
- max_backoff_ms
- backoff_multiplier

**LoopSchema:**
- type
- iterations
- condition
- convergence_criteria
- timeout_ms

**BranchSchema:**
- condition
- true_branch
- false_branch
- cases
- default

**HumanGatingSchema:**
- requires_approval
- approved_by
- approval_timestamp

### Verification Commands

```bash
cargo run --bin schema_audit -- --phase 1 --output coverage_report.md
```

**Expected Output:**
- 100% coverage for Phase 01-owned fields
- No unimplemented fields

---

## ADR Compliance

**Relevant ADRs:**
- ADR-002: Queue Scheduler Architecture
- ADR-004: State Machine Safety

### Verification Commands

```bash
cargo run --bin adr_compliance -- --phase 1
```

**Expected Output:**
- All Phase 01-relevant ADR constraints satisfied
- No outstanding ADR TODOs

---

## Anti-Goal-Drift Checklist

**Run after phase completion:**

1. **Requirements Drift:**
   - [ ] Implementation matches Phase 01 requirements
   - [ ] No features from later phases added

2. **Architecture Drift:**
   - [ ] State machine matches ADR-004
   - [ ] Scheduler matches ADR-002

3. **Scope Creep:**
   - [ ] Only queue/scheduler features implemented
   - [ ] No backend or UI features added

4. **Testing Drift:**
   - [ ] Test coverage maintained >= 90%
   - [ ] All 6 loop types tested

---

## Evidence Storage

**Location:** `results/phase_01/`

**Contents:**
- `unit_test_results.json`
- `integration_test_output.log`
- `property_test_results.json`
- `e2e_execution_logs/` (serial, parallel, hybrid, loop, branch workflows)
- `system_log_samples.json` (all 9 scopes)
- `cli_verification/` (queue and scheduler commands)
- `benchmarks/` (scheduler performance)
- `schema_coverage_report.md`
- `adr_compliance_report.md`
- `anti_goal_drift_checklist.md`
- `phase_00_regression/` (Phase 00 regression test results)

---

## Blocking Issues

**Cannot exit Phase 01 if:**
- Any verification layer fails
- Any loop type doesn't terminate
- State machine transitions invalid
- Human gating doesn't block dangerous ops
- Logging scope missing
- Metrics category missing
- Phase 00 regression detected

---

**End of Phase 01 Criteria**
