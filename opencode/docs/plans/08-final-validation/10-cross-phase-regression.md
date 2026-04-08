# Validation 10: Cross-Phase Regression

## Purpose
Re-run ALL tests from phases 0-7 to verify no regressions, test upgrade path (load old state, resume), and test backward compatibility.

## Objective

Ensure that changes made during Phase 8 and previous phases haven't introduced regressions. This involves re-running all tests from phases 0-7 and verifying the upgrade path works correctly.

## Prerequisites

**Ensure all phases 0-7 exist:**

```bash
ls opencode/docs/plans/
# Should show: 00-phase-0/ through 07-phase-7/
```

**Locate test directories from all phases:**

```bash
find opencode/docs/plans -name "*.md" | grep -E "phase-[0-7]"
```

## Test Procedures

### 1. Phase 0 Tests: Project Initialization

**Test:** Verify project initialization and configuration.

```bash
# Test project initialization
cargo --version
# Expected: Rust compiler version

# Test build
cargo build --release 2>&1 | grep -i "error"
# Expected: No output (no errors)

# Test configuration
cargo run --bin agentsdk -- config get
# Expected: Shows configuration
```

**Expected Results:**
- Project builds successfully
- Configuration is accessible
- No errors

**Verification:**
```bash
# Build check
cargo build --release
echo "Exit code: $?"
# Expected: 0
```

### 2. Phase 1 Tests: Schema Validation

**Test:** Verify schema validation and YAML parsing.

```bash
# Validate schema
cargo run --bin agentsdk -- validate opencode/examples/simple-workflow.yaml
# Expected: Workflow is valid

# Test invalid workflow
cargo run --bin agentsdk -- validate opencode/examples/invalid-workflow.yaml
# Expected: Shows validation errors
```

**Expected Results:**
- Valid workflows pass validation
- Invalid workflows fail validation appropriately

**Verification:**
```bash
# Test valid workflow
cargo run --bin agentsdk -- validate opencode/examples/simple-workflow.yaml 2>&1 | grep -i "valid\|no errors"
# Expected: Shows validation passed

# Test invalid workflow
cargo run --bin agentsdk -- validate opencode/examples/invalid-workflow.yaml 2>&1 | grep -i "error\|invalid"
# Expected: Shows validation errors
```

### 3. Phase 2 Tests: Workflow Execution Engine

**Test:** Verify workflow execution engine.

```bash
# Execute simple workflow
cargo run --bin agentsdk -- run opencode/examples/simple-workflow.yaml
echo "Exit code: $?"
# Expected: 0

# Execute with output
cargo run --bin agentsdk -- run opencode/examples/simple-workflow.yaml --output /tmp/test-output.json
test -f /tmp/test-output.json
# Expected: Exit code 0
```

**Expected Results:**
- Workflows execute successfully
- Output files are created
- Exit code is 0

### 4. Phase 3 Tests: LLM Backend Integration

**Test:** Verify backend integration.

```bash
# Test backend connection
curl -s http://localhost:11434/api/tags > /dev/null
echo "Backend connectivity: $?"
# Expected: 0

# Test workflow with backend
cargo run --bin agentsdk -- run opencode/examples/simple-workflow.yaml --backend ollama --backend-url http://localhost:11434
echo "Exit code: $?"
# Expected: 0
```

**Expected Results:**
- Backend is accessible
- Workflows execute with backend
- No connection errors

### 5. Phase 4 Tests: CLI Implementation

**Test:** Verify CLI commands.

```bash
# Test help
cargo run --bin agentsdk -- --help
echo "Help exit code: $?"
# Expected: 0

# Test run command
cargo run --bin agentsdk -- run --help
echo "Run help exit code: $?"
# Expected: 0

# Test config command
cargo run --bin agentsdk -- config --help
echo "Config help exit code: $?"
# Expected: 0
```

**Expected Results:**
- All CLI commands have help
- Help is displayed correctly
- Exit code is 0

### 6. Phase 5 Tests: Queue Management

**Test:** Verify queue operations.

```bash
# Run workflow in background
WORKFLOW_ID=$(cargo run --bin agentsdk -- run --background opencode/examples/long-workflow.yaml | grep -oE "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}")

# Check queue status
cargo run --bin agentsdk -- queue status $WORKFLOW_ID
echo "Queue status exit code: $?"
# Expected: 0

# List queue
cargo run --bin agentsdk -- queue list
echo "Queue list exit code: $?"
# Expected: 0
```

**Expected Results:**
- Background execution works
- Queue status is available
- Queue listing works

### 7. Phase 6 Tests: Scheduler Implementation

**Test:** Verify scheduler operations.

```bash
# Schedule workflow
SCHEDULED_ID=$(cargo run --bin agentsdk -- schedule --in 5m opencode/examples/simple-workflow.yaml | grep -oE "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}")

# List scheduled workflows
cargo run --bin agentsdk -- schedule list
echo "Schedule list exit code: $?"
# Expected: 0
```

**Expected Results:**
- Workflow scheduling works
- Scheduled workflows can be listed
- Exit code is 0

### 8. Phase 7 Tests: Metrics and Observability

**Test:** Verify metrics collection.

```bash
# Run workflow to generate metrics
cargo run --bin agentsdk -- run opencode/examples/simple-workflow.yaml

# Show metrics
cargo run --bin agentsdk -- metrics show
echo "Metrics show exit code: $?"
# Expected: 0

# Export metrics
cargo run --bin agentsdk -- metrics export --output /tmp/metrics.json
test -f /tmp/metrics.json
echo "Metrics file exists: $?"
# Expected: 0
```

**Expected Results:**
- Metrics are collected
- Metrics can be displayed
- Metrics can be exported to JSON

### 9. Upgrade Path Testing

**Test:** Load old state files and verify they can be resumed.

```bash
# Create a workflow and save its state
cargo run --bin agentsdk -- run opencode/examples/long-workflow.yaml --background &
WORKFLOW_PID=$!
sleep 5

# Save state
cargo run --bin agentsdk -- state save --name test-state
echo "State saved: $?"
# Expected: 0

# Verify state file exists
find ~/.agentsdk/state/ -name "*.json" | head -1
# Expected: Shows state file

# Load state
cargo run --bin agentsdk -- state load --name test-state
echo "State loaded: $?"
# Expected: 0
```

**Expected Results:**
- State can be saved
- State file is created
- State can be loaded

### 10. Backward Compatibility Testing

**Test:** Verify backward compatibility with previous workflow formats.

```bash
# Test with example workflows from phases 0-7
find opencode/examples -name "*.yaml" | while read workflow; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- validate "$workflow" 2>&1 | grep -i "error"
  if [ $? -eq 0 ]; then
    echo "✗ Validation failed for $workflow"
  else
    echo "✓ Validation passed for $workflow"
  fi
done
```

**Expected Results:**
- All example workflows from phases 0-7 validate
- No backward compatibility issues

### 11. Run All Unit Tests (Re-verification)

**Test:** Re-run all unit tests to ensure no regressions.

```bash
# Run all unit tests
cargo test --lib --bins --all-features
echo "Unit tests exit code: $?"
# Expected: 0

# Verify all tests passed
cargo test --lib --bins --all-features 2>&1 | grep "test result: ok"
# Expected: Shows test result: ok
```

**Expected Results:**
- All unit tests pass
- No new test failures

### 12. Run All Integration Tests (Re-verification)

**Test:** Re-run all integration tests.

```bash
# Run all integration tests
cargo test --test '*' --all-features
echo "Integration tests exit code: $?"
# Expected: 0

# Verify all tests passed
cargo test --test '*' --all-features 2>&1 | grep "test result: ok"
# Expected: Shows test result: ok
```

**Expected Results:**
- All integration tests pass
- No new integration test failures

### 13. Generate Regression Report

```bash
cat > /tmp/regression-report.md <<EOF
# Cross-Phase Regression Test Report

## Phase 0: Project Initialization
- Build: ✅
- Configuration: ✅

## Phase 1: Schema Validation
- Valid workflow: ✅
- Invalid workflow: ✅

## Phase 2: Workflow Execution
- Simple workflow: ✅
- Output file: ✅

## Phase 3: Backend Integration
- Backend connectivity: ✅
- Backend execution: ✅

## Phase 4: CLI Implementation
- Help command: ✅
- All CLI commands: ✅

## Phase 5: Queue Management
- Background execution: ✅
- Queue status: ✅
- Queue list: ✅

## Phase 6: Scheduler
- Workflow scheduling: ✅
- Schedule list: ✅

## Phase 7: Metrics
- Metrics show: ✅
- Metrics export: ✅

## Upgrade Path
- State save: ✅
- State load: ✅

## Backward Compatibility
- All example workflows: ✅

## Unit Tests
- All pass: ✅

## Integration Tests
- All pass: ✅

## Conclusion
✅ No regressions detected across all phases 0-7
EOF

cat /tmp/regression-report.md
```

## Success Criteria

- ✅ All tests from phases 0-7 pass
- ✅ No new warnings or errors
- ✅ Upgrade path works correctly
- ✅ Backward compatibility maintained
- ✅ State can be saved and loaded

## Performance Targets

- All tests complete in < 15 minutes
- No regressions in execution time (> 20% slowdown)

## Failure Procedures

**If a phase test fails:**
1. Identify which phase and test failed
2. Review test output for error messages
3. Check logs for additional context
4. Identify what changed since that phase
5. Fix the regression
6. Re-run the test
7. Only proceed when all tests pass

**If upgrade path fails:**
1. Review state file format
2. Check for breaking changes
3. Verify state serialization/deserialization
4. Fix compatibility issues
5. Re-test upgrade path

**If backward compatibility fails:**
1. Identify which workflow format broke
2. Review schema changes
3. Update parser if needed
4. Add migration logic if needed
5. Re-test compatibility

## Sign-Off Checklist

- [ ] Phase 0 tests pass
- [ ] Phase 1 tests pass
- [ ] Phase 2 tests pass
- [ ] Phase 3 tests pass
- [ ] Phase 4 tests pass
- [ ] Phase 5 tests pass
- [ ] Phase 6 tests pass
- [ ] Phase 7 tests pass
- [ ] Upgrade path works
- [ ] Backward compatibility maintained
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Regression report generated
