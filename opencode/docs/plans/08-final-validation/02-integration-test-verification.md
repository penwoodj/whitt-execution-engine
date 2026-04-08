# Validation 02: Integration Test Verification

## Purpose
Run all integration tests, verify cross-module data flows, verify API contracts, and verify state machine transitions.

## Test Procedures

### 1. Run All Integration Tests

**Test:** Execute all integration tests in the workspace.

**Command:**
```bash
# Run all integration tests
cargo test --test '*' --all-features

# Run integration tests with output
cargo test --test '*' --all-features -- --nocapture
```

**Expected Results:**
- All integration tests pass
- Test output shows `test result: ok. XX passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- No tests are marked as `ignored`

**Verification:**
```bash
# Run integration tests
cargo test --test '*' --all-features

# Verify exit code is 0
cargo test --test '*' --all-features
echo "Exit code: $?"
# Expected: Exit code: 0

# Verify all tests passed
cargo test --test '*' --all-features 2>&1 | grep "test result: ok"
# Expected: test result: ok

# Verify no tests failed
cargo test --test '*' --all-features 2>&1 | grep "test result: FAILED"
# Expected: No output (empty)
```

### 2. Verify Cross-Module Data Flows

**Test:** Verify data flows correctly between modules (workflow → execution → model → backend).

**Test Case 1: Workflow to Execution Flow**
```bash
# Run workflow execution integration test
cargo test --test workflow_integration --all-features -- --nocapture

# Verify workflow is parsed and passed to execution
# Verify execution context is created correctly
# Verify execution results are returned
```

**Test Case 2: Execution to Model Flow**
```bash
# Run execution-to-model integration test
cargo test --test execution_model_integration --all-features -- --nocapture

# Verify model requests are created correctly
# Verify model parameters are passed correctly
# Verify model responses are parsed correctly
```

**Test Case 3: Model to Backend Flow**
```bash
# Run model-to-backend integration test
cargo test --test model_backend_integration --all-features -- --nocapture

# Verify backend is selected correctly
# Verify requests are sent to the correct backend
# Verify backend responses are received correctly
```

**Test Case 4: Full End-to-End Flow**
```bash
# Run full workflow execution
cargo run --bin agentsdk -- \
  --log-level debug \
  run opencode/examples/simple-workflow.yaml

# Verify logs show correct flow:
# workflow: parsed workflow
# execution: started execution
# model: prepared model request
# backend: sent request to backend
# backend: received response
# model: parsed response
# execution: completed step
# workflow: completed workflow
```

**Expected Results:**
- Data flows correctly between all modules
- No data loss or corruption
- All transformations are correct

**Verification:**
```bash
# Verify workflow logs show correct flow
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="workflow") | .message' | head -10
# Expected: Workflow parsing, validation, execution start messages

# Verify execution logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="execution") | .message' | head -10
# Expected: Step execution, context management messages

# Verify model logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model") | .message' | head -10
# Expected: Model request preparation, response parsing messages

# Verify backend logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="backend") | .message' | head -10
# Expected: Backend selection, request sending, response receiving messages
```

### 3. Verify API Contracts

**Test:** Verify that API contracts match the schema.

**Test Case 1: Schema Validation**
```bash
# Validate workflow schema
cargo run --bin agentsdk -- validate opencode/examples/simple-workflow.yaml

# Expected: Workflow is valid, no errors
```

**Test Case 2: Request/Response Formats**
```bash
# Run integration test that verifies request formats
cargo test --test api_contract_test --all-features -- --nocapture

# Verify requests match schema
# Verify responses match schema
# Verify error responses match schema
```

**Test Case 3: Backend API Contracts**
```bash
# Test each backend's API contract
cargo test --test backend_ollama_integration --all-features -- --nocapture
cargo test --test backend_lm_studio_integration --all-features -- --nocapture
cargo test --test backend_llama_cpp_integration --all-features -- --nocapture

# Verify each backend's requests/responses match their respective APIs
```

**Expected Results:**
- All workflows validate against the schema
- All requests/responses match their contracts
- No contract violations

**Verification:**
```bash
# Validate all example workflows
find opencode/examples -name "*.yaml" | while read workflow; do
  echo "Validating: $workflow"
  cargo run --bin agentsdk -- validate "$workflow" || echo "FAILED: $workflow"
done

# Expected: All workflows validate successfully

# Verify backend API contracts
cargo test --test backend_integration --all-features 2>&1 | grep "test result: ok"
# Expected: test result: ok
```

### 4. Verify State Machine Transitions

**Test:** Verify state machine transitions work correctly.

**Test Case 1: Normal Execution Flow**
```bash
# Run a simple workflow and verify state transitions
cargo run --bin agentsdk -- \
  --log-level debug \
  run opencode/examples/simple-workflow.yaml

# Verify state transitions in logs:
# 1. workflow: initialized
# 2. execution: running
# 3. execution: paused (if applicable)
# 4. execution: resumed (if applicable)
# 5. execution: completed
```

**Test Case 2: Pause/Resume Transitions**
```bash
# Run a long workflow in background
cargo run --bin agentsdk -- \
  run opencode/examples/long-workflow.yaml &
WORKFLOW_PID=$!

# Wait a bit then pause
sleep 5
cargo run --bin agentsdk -- queue pause $WORKFLOW_PID

# Verify state is paused
cargo run --bin agentsdk -- queue status $WORKFLOW_PID
# Expected: state: paused

# Resume
cargo run --bin agentsdk -- queue resume $WORKFLOW_PID

# Verify state is running
cargo run --bin agentsdk -- queue status $WORKFLOW_PID
# Expected: state: running
```

**Test Case 3: Error State Transitions**
```bash
# Run a workflow that will error
cargo run --bin agentsdk -- \
  run opencode/examples/error-workflow.yaml

# Verify state transitions:
# 1. workflow: initialized
# 2. execution: running
# 3. execution: error
# 4. workflow: failed
```

**Test Case 4: Cancel Transitions**
```bash
# Run a long workflow in background
cargo run --bin agentsdk -- \
  run opencode/examples/long-workflow.yaml &
WORKFLOW_PID=$!

# Cancel it
sleep 2
cargo run --bin agentsdk -- queue cancel $WORKFLOW_PID

# Verify state is cancelled
cargo run --bin agentsdk -- queue status $WORKFLOW_PID
# Expected: state: cancelled
```

**Expected Results:**
- All state transitions are correct
- State machine handles all edge cases
- No invalid state transitions occur

**Verification:**
```bash
# Verify state transitions in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="execution" and (.message | contains("state"))) | .message'
# Expected: Shows valid state transitions (initialized → running → paused/resumed → completed/error/cancelled)

# Verify state machine is robust
cargo test --test state_machine_integration --all-features 2>&1 | grep "test result: ok"
# Expected: test result: ok
```

### 5. Verify Error Handling Across Modules

**Test:** Verify errors are handled correctly across module boundaries.

**Test Case 1: Backend Error Propagation**
```bash
# Test with an unreachable backend
cargo run --bin agentsdk -- \
  --backend-url http://localhost:9999 \
  run opencode/examples/simple-workflow.yaml

# Verify error propagates correctly:
# backend: connection failed
# model: backend error
# execution: model error
# workflow: failed
```

**Test Case 2: Validation Error Propagation**
```bash
# Test with an invalid workflow
cargo run --bin agentsdk -- \
  run opencode/examples/invalid-workflow.yaml

# Verify error is caught at validation:
# workflow: validation error
# execution: not started
```

**Test Case 3: Timeout Error Handling**
```bash
# Test with a timeout that will be exceeded
cargo run --bin agentsdk -- \
  --timeout 1s \
  run opencode/examples/slow-workflow.yaml

# Verify timeout is handled:
# execution: timeout exceeded
# workflow: failed with timeout error
```

**Expected Results:**
- Errors are caught and handled at appropriate levels
- Error messages are informative
- No crashes or panics on errors

**Verification:**
```bash
# Verify error handling works
cargo test --test error_handling_integration --all-features 2>&1 | grep "test result: ok"
# Expected: test result: ok

# Verify error logs are correct
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.level=="error") | .message' | head -10
# Expected: Informative error messages with proper metadata
```

### 6. Verify Concurrency and Parallelism

**Test:** Verify concurrent execution works correctly.

**Test Case 1: Parallel Model Execution**
```bash
# Run a workflow with parallel model calls
cargo run --bin agentsdk -- \
  --log-level debug \
  run opencode/examples/parallel-models.yaml

# Verify parallel execution:
# Verify multiple models are called concurrently
# Verify results are collected correctly
```

**Test Case 2: Concurrent Workflow Execution**
```bash
# Run multiple workflows concurrently
cargo run --bin agentsdk -- run workflow1.yaml &
PID1=$!
cargo run --bin agentsdk -- run workflow2.yaml &
PID2=$!
cargo run --bin agentsdk -- run workflow3.yaml &
PID3=$!

# Wait for all to complete
wait $PID1 $PID2 $PID3

# Verify all workflows completed successfully
```

**Test Case 3: Thread Safety**
```bash
# Run thread safety integration tests
cargo test --test thread_safety_integration --all-features -- --nocapture

# Verify no race conditions
# Verify no deadlocks
# Verify no data races
```

**Expected Results:**
- Parallel execution works correctly
- No race conditions or deadlocks
- Results are consistent

**Verification:**
```bash
# Verify parallel execution works
cargo test --test parallel_integration --all-features 2>&1 | grep "test result: ok"
# Expected: test result: ok

# Verify no concurrency issues in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.level=="error" and (.message | contains("race" or "deadlock" or "concurrency"))) | .message' | wc -l
# Expected: 0
```

### 7. Verify Resource Management

**Test:** Verify resources are managed correctly (connections, memory, file handles).

**Test Case 1: Connection Pooling**
```bash
# Run many workflow executions
for i in {1..20}; do
  cargo run --bin agentsdk -- run opencode/examples/simple-workflow.yaml
done

# Verify connection pooling works:
# Verify connections are reused
# Verify no connection leaks
```

**Test Case 2: Memory Management**
```bash
# Monitor memory usage during workflow execution
/usr/bin/time -v cargo run --bin agentsdk -- \
  run opencode/examples/large-workflow.yaml

# Verify memory is managed correctly:
# Verify no memory leaks
# Verify memory is freed after completion
```

**Test Case 3: File Handle Management**
```bash
# Run many workflows that use files
for i in {1..20}; do
  cargo run --bin agentsdk -- run opencode/examples/file-workflow.yaml
done

# Verify file handles are managed:
# Verify no file descriptor leaks
```

**Expected Results:**
- No resource leaks
- Resources are cleaned up properly
- No resource exhaustion

**Verification:**
```bash
# Verify resource management
cargo test --test resource_management_integration --all-features 2>&1 | grep "test result: ok"
# Expected: test result: ok

# Check for resource leaks
lsof -p $(pgrep -f agentsdk) | wc -l
# Expected: Reasonable number (not growing unbounded)
```

### 8. Verify Backend Integration

**Test:** Verify each backend integrates correctly.

**Test Case 1: Ollama Integration**
```bash
# Ensure Ollama is running
curl -s http://localhost:11434/api/tags > /dev/null

# Run workflow with Ollama
cargo run --bin agentsdk -- \
  --backend ollama \
  --backend-url http://localhost:11434 \
  run opencode/examples/simple-workflow.yaml

# Verify execution completes
```

**Test Case 2: LM Studio Integration**
```bash
# Ensure LM Studio is running
curl -s http://localhost:1234/v1/models > /dev/null

# Run workflow with LM Studio
cargo run --bin agentsdk -- \
  --backend lm-studio \
  --backend-url http://localhost:1234/v1 \
  run opencode/examples/simple-workflow.yaml

# Verify execution completes
```

**Test Case 3: llama.cpp Integration**
```bash
# Ensure llama.cpp server is running
curl -s http://localhost:8080/health > /dev/null

# Run workflow with llama.cpp
cargo run --bin agentsdk -- \
  --backend llama-cpp \
  --backend-url http://localhost:8080 \
  run opencode/examples/simple-workflow.yaml

# Verify execution completes
```

**Expected Results:**
- All backends work correctly
- Backend selection works
- Backend-specific features work

**Verification:**
```bash
# Test each backend
cargo test --test backend_ollama_integration --all-features 2>&1 | grep "test result: ok"
cargo test --test backend_lm_studio_integration --all-features 2>&1 | grep "test result: ok"
cargo test --test backend_llama_cpp_integration --all-features 2>&1 | grep "test result: ok"
# Expected: All show test result: ok
```

## Success Criteria

- ✅ All integration tests pass
- ✅ Cross-module data flows work correctly
- ✅ API contracts are verified
- ✅ State machine transitions work correctly
- ✅ Error handling works across modules
- ✅ Concurrency and parallelism work correctly
- ✅ Resources are managed correctly
- ✅ All backends integrate correctly

## Performance Targets

- Integration test execution: < 10 minutes
- Workflow execution (simple): < 30 seconds
- Parallel workflow execution: Linear or sub-linear scaling
- Resource usage: No leaks, memory freed after completion

## Failure Procedures

**If integration tests fail:**
1. Identify which test(s) failed
2. Run the failing test with verbose output: `cargo test <test_name> -- --nocapture`
3. Review test output for error messages
4. Check logs for additional context
5. Verify external dependencies (backends, databases) are running
6. Fix the issue and re-run the test
7. Only proceed when all tests pass

**If data flow issues:**
1. Review logs to see where data is lost or corrupted
2. Add debug logging to track data flow
3. Verify data transformations are correct
4. Check for serialization/deserialization issues
5. Fix the issue and re-run tests

**If state machine issues:**
1. Review state machine implementation
2. Verify all valid transitions are implemented
3. Verify invalid transitions are prevented
4. Add state transition logging
5. Fix the issue and re-run tests

**If backend integration issues:**
1. Verify the backend is running and accessible
2. Check backend API compatibility
3. Review backend-specific code for bugs
4. Test with a simple request to isolate the issue
5. Fix the issue and re-run tests

## Sign-Off Checklist

- [ ] All integration tests pass
- [ ] Cross-module data flows verified
- [ ] API contracts verified
- [ ] State machine transitions verified
- [ ] Error handling verified
- [ ] Concurrency verified
- [ ] Resource management verified
- [ ] All backends integrated successfully
- [ ] All verification commands pass
- [ ] Performance targets met
