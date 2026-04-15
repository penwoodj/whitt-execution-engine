# Integration Tests Specification

This document defines integration test requirements for Phase 1 MVP Queue & Scheduler.

---

## Test Philosophy

- **End-to-end workflows**: Test complete user journeys
- **Real components**: Use actual storage, scheduler, executor (not mocks)
- **Realistic scenarios**: Mirror production use cases
- **Fast feedback**: Tests complete in <10 seconds each

---

## Integration Test Scenarios

### 1. Queue → Scheduler → Executor Pipeline

**Purpose:** Verify complete job execution flow

**Setup:**
- Create QueueStorage (in-memory)
- Create Scheduler with 2 workers
- Register StepExecutor with mock LLM

**Test Steps:**
1. Submit workflow with 3 steps (echo, compute, echo)
2. Wait for job to complete
3. Verify job state is Completed
4. Verify all 3 steps executed
5. Verify outputs are correct

**Expected Outcome:**
- Job transitions: Pending → Scheduled → Running → Completed
- All steps execute successfully
- Outputs contain expected results

**File:** `tests/integration/queue_test.rs`

---

### 2. Human Gating Integration

**Purpose:** Verify human gating prevents dangerous operations

**Setup:**
- Create QueueStorage
- Create Scheduler
- Register StepExecutor
- Mock user input to approve operation

**Test Steps:**
1. Submit workflow with write_file operation
2. Verify operation is classified as risky
3. Verify confirmation prompt is displayed
4. User confirms operation
5. Verify operation executes
6. Verify file was written

**Expected Outcome:**
- Write operation classified as risky
- User prompted for confirmation
- Operation executes after confirmation
- File written successfully

**File:** `tests/integration/human_gating_test.rs`

---

### 3. Loop Integration

**Purpose:** Verify loops work in real execution context

**Setup:**
- Create QueueStorage
- Create Scheduler
- Register StepExecutor

**Test Steps (Count Loop):**
1. Submit workflow with count loop (5 iterations)
2. Wait for job to complete
3. Verify loop ran 5 times
4. Verify loop_index variable set correctly

**Test Steps (Foreach Loop):**
1. Submit workflow with foreach loop over [1, 2, 3]
2. Wait for job to complete
3. Verify loop ran 3 times
4. Verify loop_item values are correct

**Test Steps (Retry Loop):**
1. Submit workflow with retry loop (max 3)
2. Configure step to fail first 2 times
3. Wait for job to complete
4. Verify retry_count is 2
5. Verify job completes successfully

**Expected Outcome:**
- All loop types execute correctly
- Loop variables are set
- Retry logic works

**File:** `tests/integration/loop_test.rs`

---

### 4. Branch Integration

**Purpose:** Verify conditional branching in workflows

**Setup:**
- Create QueueStorage
- Create Scheduler
- Register StepExecutor

**Test Steps:**
1. Submit workflow with branch based on variable
2. Set variable to value that matches condition
3. Wait for job to complete
4. Verify correct branch executed
5. Verify other branches were not executed

**Expected Outcome:**
- Condition evaluated correctly
- Only matching branch executed
- Outputs reflect branch result

**File:** `tests/integration/branch_test.rs`

---

### 5. Parallel Execution Integration

**Purpose:** Verify parallel groups execute concurrently

**Setup:**
- Create QueueStorage
- Create Scheduler with 4 workers
- Register StepExecutor

**Test Steps:**
1. Submit workflow with parallel group (3 steps, concurrency 2)
2. Wait for job to complete
3. Verify all 3 steps executed
4. Verify concurrency limit respected (max 2 at a time)

**Expected Outcome:**
- All steps execute
- Concurrency limit enforced
- Job completes successfully

**File:** `tests/integration/parallel_test.rs`

---

### 6. Recovery After Crash

**Purpose:** Verify jobs recover from process crash

**Setup:**
- Create QueueStorage with file persistence (not in-memory)
- Create Scheduler

**Test Steps:**
1. Submit workflow with 10 steps
2. After 3 steps complete, simulate process crash (kill process)
3. Restart process with same storage
4. Verify scheduler recovers running job
5. Verify job resumes from checkpoint
6. Verify remaining steps execute
7. Verify job completes successfully

**Expected Outcome:**
- Running job recovered on restart
- Job state restored
- Execution resumes from checkpoint
- Remaining steps execute
- Job completes successfully

**File:** `tests/integration/recovery_test.rs`

---

### 7. Cancellation During Execution

**Purpose:** Verify jobs can be cancelled mid-execution

**Setup:**
- Create QueueStorage
- Create Scheduler
- Register StepExecutor

**Test Steps:**
1. Submit workflow with long-running step
2. After step starts, send cancel command
3. Verify job state transitions to Cancelled
4. Verify step stops executing
5. Verify no further steps execute

**Expected Outcome:**
- Job cancelled successfully
- Step stops immediately
- Job state is Cancelled
- No further steps execute

**File:** `tests/integration/cancellation_test.rs`

---

### 8. Metrics Collection Integration

**Purpose:** Verify metrics collected during execution

**Setup:**
- Create QueueStorage
- Create Scheduler
- Register StepExecutor with metrics collector

**Test Steps:**
1. Submit workflow with 3 steps
2. Wait for job to complete
3. Verify pipeline metrics collected (duration, step counts)
4. Verify step metrics collected (duration, success)
5. Verify tool metrics collected (tool name, duration)
6. Export metrics to JSON
7. Verify JSON structure is correct

**Expected Outcome:**
- Pipeline metrics accurate
- Step metrics accurate
- Tool metrics accurate
- JSON export works

**File:** `tests/integration/metrics_test.rs`

---

### 9. Logging Integration

**Purpose:** Verify logging works across all components

**Setup:**
- Create QueueStorage
- Create Scheduler with custom logging config
- Register StepExecutor

**Test Steps:**
1. Configure logging with TRACE level
2. Submit workflow with all step types
3. Wait for job to complete
4. Verify logs emitted for all components
5. Verify log levels are correct
6. Verify scope information is present

**Expected Outcome:**
- Logs from all components
- Correct log levels
- Scope information present
- Structured log format

**File:** `tests/integration/logging_test.rs`

---

### 10. Error Handling Integration

**Purpose:** Verify errors are handled gracefully

**Setup:**
- Create QueueStorage
- Create Scheduler
- Register StepExecutor

**Test Steps (Transient Error):**
1. Submit workflow with step that fails with timeout
2. Wait for job to complete
3. Verify retry logic activated
4. Verify job completes after retry

**Test Steps (Permanent Error):**
1. Submit workflow with step that fails permanently
2. Wait for job to complete
3. Verify job state is Failed
4. Verify error message is present

**Expected Outcome:**
- Transient errors retried
- Permanent errors fail immediately
- Job states correct
- Error messages present

**File:** `tests/integration/error_handling_test.rs`

---

## Example Workflow Tests

### 11. Simple Echo Workflow

**Purpose:** Verify basic workflow execution

**Setup:** Use `01-basics/echo.yaml` example

**Test Steps:**
1. Load workflow from YAML
2. Submit to scheduler
3. Wait for completion
4. Verify output matches expected

**Expected Outcome:** Workflow executes successfully with correct output

---

### 12. Loop Workflow

**Purpose:** Verify loop with examples

**Setup:** Use `05-loops-convergence/*` examples

**Test Steps:**
1. Load workflow with count/foreach/while loop
2. Submit to scheduler
3. Wait for completion
4. Verify loop executed correctly

**Expected Outcome:** Loop works as specified

---

### 13. Branch Workflow

**Purpose:** Verify branching with examples

**Setup:** Use `03-branches/*` examples

**Test Steps:**
1. Load workflow with branches
2. Submit to scheduler
3. Wait for completion
4. Verify correct branch taken

**Expected Outcome:** Branching works correctly

---

### 14. Parallel Workflow

**Purpose:** Verify parallel execution with examples

**Setup:** Use `04-parallel/*` examples

**Test Steps:**
1. Load workflow with parallel group
2. Submit to scheduler
3. Wait for completion
4. Verify parallel execution worked

**Expected Outcome:** Parallel execution works

---

## Running Integration Tests

```bash
# Run all integration tests
cargo test --test integration

# Run specific integration test
cargo test --test integration queue_test

# Run with output
cargo test --test integration -- --nocapture

# Run in watch mode
cargo watch -x test --test integration
```

---

## Integration Test Organization

```
tests/
  integration/
    queue_test.rs
    human_gating_test.rs
    loop_test.rs
    branch_test.rs
    parallel_test.rs
    recovery_test.rs
    cancellation_test.rs
    metrics_test.rs
    logging_test.rs
    error_handling_test.rs
    example_workflows_test.rs
```

---

## Success Criteria

- All 14 integration test scenarios pass
- At least 10 example workflows execute successfully
- All integration tests complete in <2 minutes total
- No flaky tests (consistent results)
