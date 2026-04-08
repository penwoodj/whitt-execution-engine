# Validation 03: CLI Live Testing

## Purpose
Test ALL CLI commands against real backends to ensure the user-facing interface works correctly end-to-end.

## Prerequisites

**Ensure at least one backend is running before testing:**

```bash
# Test Ollama
curl -s http://localhost:11434/api/tags > /dev/null && echo "Ollama is running"

# Test LM Studio
curl -s http://localhost:1234/v1/models > /dev/null && echo "LM Studio is running"

# Test llama.cpp
curl -s http://localhost:8080/health > /dev/null && echo "llama.cpp is running"
```

**Configure CLI with backend:**

```bash
cargo run --bin agentsdk -- config set backend ollama
cargo run --bin agentsdk -- config set backend-url http://localhost:11434
```

## Test Procedures

### 1. Test `run` Command

**Test Case 1: Basic Workflow Execution**
```bash
# Run a simple workflow
cargo run --bin agentsdk -- run opencode/examples/simple-workflow.yaml

# Expected: Workflow executes successfully, output is shown
```

**Verification:**
```bash
# Verify exit code is 0
cargo run --bin agentsdk -- run opencode/examples/simple-workflow.yaml
echo "Exit code: $?"
# Expected: Exit code: 0

# Verify output contains expected content
cargo run --bin agentsdk -- run opencode/examples/simple-workflow.yaml | grep -i "result\|output\|completed"
# Expected: Shows completion message or results
```

**Test Case 2: Workflow with Parameters**
```bash
# Run workflow with parameters
cargo run --bin agentsdk -- run \
  opencode/examples/parameterized-workflow.yaml \
  --param message="Hello, World!"

# Expected: Parameters are passed correctly
```

**Verification:**
```bash
# Verify parameters are used
cargo run --bin agentsdk -- run \
  opencode/examples/parameterized-workflow.yaml \
  --param message="Test Message" | grep "Test Message"
# Expected: Output contains "Test Message"
```

**Test Case 3: Background Execution**
```bash
# Run workflow in background
cargo run --bin agentsdk -- run \
  --background \
  opencode/examples/long-workflow.yaml

# Expected: Workflow ID is returned, command exits immediately
```

**Verification:**
```bash
# Run in background
WORKFLOW_ID=$(cargo run --bin agentsdk -- run --background opencode/examples/long-workflow.yaml | grep -oE "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}")

# Verify workflow is running
cargo run --bin agentsdk -- queue status $WORKFLOW_ID
# Expected: Shows workflow is running
```

### 2. Test `generate` Command

**Test Case 1: Generate from Template**
```bash
# Generate a workflow from a template
cargo run --bin agentsdk -- generate \
  --template simple \
  --output generated-workflow.yaml

# Expected: Workflow file is created
```

**Verification:**
```bash
# Verify file exists
test -f generated-workflow.yaml
# Expected: Exit code 0

# Verify generated workflow is valid
cargo run --bin agentsdk -- validate generated-workflow.yaml
# Expected: Workflow is valid
```

**Test Case 2: Generate with Custom Parameters**
```bash
# Generate with custom parameters
cargo run --bin agentsdk -- generate \
  --template parallel \
  --param num-models=3 \
  --output parallel-workflow.yaml

# Expected: Workflow with 3 models is generated
```

**Verification:**
```bash
# Verify generated workflow has 3 models
cat parallel-workflow.yaml | grep -c "model:"
# Expected: 3
```

### 3. Test `queue` Commands

**Test Case 1: `queue list`**
```bash
# Start a background workflow
cargo run --bin agentsdk -- run --background opencode/examples/long-workflow.yaml &

# Wait a moment
sleep 2

# List queued workflows
cargo run --bin agentsdk -- queue list

# Expected: Shows running workflows
```

**Verification:**
```bash
# Verify list shows workflows
cargo run --bin agentsdk -- queue list | grep -i "workflow\|running\|queued"
# Expected: Shows at least one workflow
```

**Test Case 2: `queue status`**
```bash
# Start a background workflow
WORKFLOW_ID=$(cargo run --bin agentsdk -- run --background opencode/examples/long-workflow.yaml | grep -oE "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}")

# Get status
cargo run --bin agentsdk -- queue status $WORKFLOW_ID

# Expected: Shows workflow status (running, paused, completed, etc.)
```

**Verification:**
```bash
# Verify status information
cargo run --bin agentsdk -- queue status $WORKFLOW_ID | grep -i "id\|status\|progress"
# Expected: Shows ID, status, and progress
```

**Test Case 3: `queue pause`**
```bash
# Start a background workflow
WORKFLOW_ID=$(cargo run --bin agentsdk -- run --background opencode/examples/long-workflow.yaml | grep -oE "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}")

# Wait for it to start
sleep 3

# Pause the workflow
cargo run --bin agentsdk -- queue pause $WORKFLOW_ID

# Expected: Workflow is paused
```

**Verification:**
```bash
# Verify workflow is paused
cargo run --bin agentsdk -- queue status $WORKFLOW_ID | grep -i "paused"
# Expected: Shows status as paused
```

**Test Case 4: `queue resume`**
```bash
# Resume the paused workflow
cargo run --bin agentsdk -- queue resume $WORKFLOW_ID

# Expected: Workflow resumes execution
```

**Verification:**
```bash
# Verify workflow is running again
cargo run --bin agentsdk -- queue status $WORKFLOW_ID | grep -i "running"
# Expected: Shows status as running
```

**Test Case 5: `queue cancel`**
```bash
# Start another background workflow
WORKFLOW_ID_2=$(cargo run --bin agentsdk -- run --background opencode/examples/long-workflow.yaml | grep -oE "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}")

# Wait for it to start
sleep 3

# Cancel the workflow
cargo run --bin agentsdk -- queue cancel $WORKFLOW_ID_2

# Expected: Workflow is cancelled
```

**Verification:**
```bash
# Verify workflow is cancelled
cargo run --bin agentsdk -- queue status $WORKFLOW_ID_2 | grep -i "cancelled"
# Expected: Shows status as cancelled
```

### 4. Test `config` Commands

**Test Case 1: `config get`**
```bash
# Get all configuration
cargo run --bin agentsdk -- config get

# Expected: Shows all configuration values
```

**Verification:**
```bash
# Verify config is shown
cargo run --bin agentsdk -- config get | grep -i "backend\|url\|timeout"
# Expected: Shows configuration values
```

**Test Case 2: `config set`**
```bash
# Set a configuration value
cargo run --bin agentsdk -- config set timeout 60

# Expected: Value is set
```

**Verification:**
```bash
# Verify value was set
cargo run --bin agentsdk -- config get timeout
# Expected: Shows "60"
```

**Test Case 3: `config unset`**
```bash
# Unset a configuration value
cargo run --bin agentsdk -- config unset timeout

# Expected: Value is unset (reverts to default)
```

**Verification:**
```bash
# Verify value was unset
cargo run --bin agentsdk -- config get timeout
# Expected: Shows default value or "not set"
```

**Test Case 4: `config list`**
```bash
# List all available configuration options
cargo run --bin agentsdk -- config list

# Expected: Shows all configuration keys
```

**Verification:**
```bash
# Verify config options are listed
cargo run --bin agentsdk -- config list | grep -i "backend\|url\|timeout\|log"
# Expected: Shows configuration options
```

### 5. Test `schedule` Commands

**Test Case 1: Schedule a Workflow**
```bash
# Schedule a workflow to run in 5 minutes
cargo run --bin agentsdk -- schedule \
  --in 5m \
  opencode/examples/simple-workflow.yaml

# Expected: Workflow is scheduled
```

**Verification:**
```bash
# Verify workflow is scheduled
SCHEDULED_ID=$(cargo run --bin agentsdk -- schedule --in 5m opencode/examples/simple-workflow.yaml | grep -oE "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}")
cargo run --bin agentsdk -- queue status $SCHEDULED_ID | grep -i "scheduled"
# Expected: Shows status as scheduled
```

**Test Case 2: Schedule with Cron**
```bash
# Schedule a workflow to run every hour
cargo run --bin agentsdk -- schedule \
  --cron "0 * * * *" \
  opencode/examples/simple-workflow.yaml

# Expected: Workflow is scheduled with cron
```

**Verification:**
```bash
# Verify cron schedule
cargo run --bin agentsdk -- schedule --cron "0 * * * *" opencode/examples/simple-workflow.yaml | grep -i "cron"
# Expected: Shows cron expression
```

**Test Case 3: List Scheduled Workflows**
```bash
# List all scheduled workflows
cargo run --bin agentsdk -- schedule list

# Expected: Shows scheduled workflows
```

**Verification:**
```bash
# Verify scheduled workflows are listed
cargo run --bin agentsdk -- schedule list | grep -i "workflow\|scheduled\|cron"
# Expected: Shows scheduled workflows
```

### 6. Test `experiment` Commands

**Test Case 1: List Experiments**
```bash
# List available experiments
cargo run --bin agentsdk -- experiment list

# Expected: Shows available experiments
```

**Verification:**
```bash
# Verify experiments are listed
cargo run --bin agentsdk -- experiment list | grep -i "experiment"
# Expected: Shows experiment names
```

**Test Case 2: Enable Experiment**
```bash
# Enable an experimental feature
cargo run --bin agentsdk -- experiment enable parallel-execution

# Expected: Feature is enabled
```

**Verification:**
```bash
# Verify feature is enabled
cargo run --bin agentsdk -- experiment list | grep "parallel-execution" | grep -i "enabled"
# Expected: Shows as enabled
```

**Test Case 3: Disable Experiment**
```bash
# Disable the feature
cargo run --bin agentsdk -- experiment disable parallel-execution

# Expected: Feature is disabled
```

**Verification:**
```bash
# Verify feature is disabled
cargo run --bin agentsdk -- experiment list | grep "parallel-execution" | grep -i "disabled"
# Expected: Shows as disabled
```

### 7. Test `merge` Command

**Test Case 1: Merge Workflow Results**
```bash
# Run two workflows
cargo run --bin agentsdk -- run opencode/examples/workflow-a.yaml --output results-a.json
cargo run --bin agentsdk -- run opencode/examples/workflow-b.yaml --output results-b.json

# Merge results
cargo run --bin agentsdk -- merge \
  results-a.json \
  results-b.json \
  --output merged-results.json

# Expected: Results are merged
```

**Verification:**
```bash
# Verify merged file exists
test -f merged-results.json
# Expected: Exit code 0

# Verify merged content
cat merged-results.json | jq '.'
# Expected: Shows merged results from both workflows
```

### 8. Test `rollback` Command

**Test Case 1: Rollback to Previous Version**
```bash
# Save current state
cargo run --bin agentsdk -- state save --name current-state

# Make some changes
cargo run --bin agentsdk -- config set timeout 120

# Rollback
cargo run --bin agentsdk -- rollback --to current-state

# Expected: State is restored
```

**Verification:**
```bash
# Verify rollback worked
cargo run --bin agentsdk -- config get timeout
# Expected: Shows previous value (not 120)
```

**Test Case 2: List Rollback Points**
```bash
# List available rollback points
cargo run --bin agentsdk -- rollback list

# Expected: Shows saved states
```

**Verification:**
```bash
# Verify states are listed
cargo run --bin agentsdk -- rollback list | grep -i "state\|time\|version"
# Expected: Shows saved states
```

### 9. Test `metrics` Commands

**Test Case 1: Show Current Metrics**
```bash
# Run a workflow to generate metrics
cargo run --bin agentsdk -- run opencode/examples/simple-workflow.yaml

# Show metrics
cargo run --bin agentsdk -- metrics show

# Expected: Shows execution metrics
```

**Verification:**
```bash
# Verify metrics are shown
cargo run --bin agentsdk -- metrics show | grep -i "time\|memory\|tokens\|requests"
# Expected: Shows metric values
```

**Test Case 2: Export Metrics**
```bash
# Export metrics to file
cargo run --bin agentsdk -- metrics export --output metrics.json

# Expected: Metrics file is created
```

**Verification:**
```bash
# Verify file exists and contains metrics
test -f metrics.json
cat metrics.json | jq '.'
# Expected: Shows metrics in JSON format
```

**Test Case 3: Compare Metrics**
```bash
# Run another workflow
cargo run --bin agentsdk -- run opencode/examples/parallel-workflow.yaml

# Compare with previous metrics
cargo run --bin agentsdk -- metrics compare metrics.json

# Expected: Shows comparison
```

**Verification:**
```bash
# Verify comparison works
cargo run --bin agentsdk -- metrics compare metrics.json | grep -i "diff\|change\|vs"
# Expected: Shows metric differences
```

### 10. Test `memory` Commands

**Test Case 1: Store Data in Memory**
```bash
# Store a value
cargo run --bin agentsdk -- memory set my-key "my-value"

# Expected: Value is stored
```

**Verification:**
```bash
# Verify value is stored
cargo run --bin agentsdk -- memory get my-key
# Expected: Shows "my-value"
```

**Test Case 2: Retrieve Data from Memory**
```bash
# Retrieve the value
cargo run --bin agentsdk -- memory get my-key

# Expected: Returns stored value
```

**Verification:**
```bash
# Verify retrieval works
cargo run --bin agentsdk -- memory get my-key | grep "my-value"
# Expected: Shows the value
```

**Test Case 3: Search Memory**
```bash
# Store multiple values
cargo run --bin agentsdk -- memory set key1 "value1"
cargo run --bin agentsdk -- memory set key2 "value2"
cargo run --bin agentsdk -- memory set key3 "value3"

# Search memory
cargo run --bin agentsdk -- memory search "value"

# Expected: Shows matching keys
```

**Verification:**
```bash
# Verify search works
cargo run --bin agentsdk -- memory search "value" | grep -i "key"
# Expected: Shows key1, key2, key3
```

**Test Case 4: List All Memory Keys**
```bash
# List all keys
cargo run --bin agentsdk -- memory list

# Expected: Shows all stored keys
```

**Verification:**
```bash
# Verify list works
cargo run --bin agentsdk -- memory list | grep -i "key"
# Expected: Shows key1, key2, key3, my-key
```

**Test Case 5: Delete from Memory**
```bash
# Delete a key
cargo run --bin agentsdk -- memory delete key1

# Expected: Key is deleted
```

**Verification:**
```bash
# Verify deletion works
cargo run --bin agentsdk -- memory list | grep "key1"
# Expected: No output (key1 is deleted)
```

### 11. Test `validate` Command

**Test Case 1: Validate Valid Workflow**
```bash
# Validate a valid workflow
cargo run --bin agentsdk -- validate opencode/examples/simple-workflow.yaml

# Expected: Workflow is valid
```

**Verification:**
```bash
# Verify validation passes
cargo run --bin agentsdk -- validate opencode/examples/simple-workflow.yaml | grep -i "valid"
# Expected: Shows "valid" or "no errors"
```

**Test Case 2: Validate Invalid Workflow**
```bash
# Validate an invalid workflow
cargo run --bin agentsdk -- validate opencode/examples/invalid-workflow.yaml

# Expected: Shows validation errors
```

**Verification:**
```bash
# Verify errors are shown
cargo run --bin agentsdk -- validate opencode/examples/invalid-workflow.yaml | grep -i "error\|invalid"
# Expected: Shows error messages
```

### 12. Test `help` Command

**Test Case 1: Show General Help**
```bash
# Show help
cargo run --bin agentsdk -- --help

# Expected: Shows general help information
```

**Verification:**
```bash
# Verify help is shown
cargo run --bin agentsdk -- --help | grep -i "usage\|commands\|options"
# Expected: Shows usage information
```

**Test Case 2: Show Command-Specific Help**
```bash
# Show help for a specific command
cargo run --bin agentsdk -- run --help

# Expected: Shows help for the run command
```

**Verification:**
```bash
# Verify command help is shown
cargo run --bin agentsdk -- run --help | grep -i "run\|workflow\|options"
# Expected: Shows run command help
```

## Success Criteria

- ✅ All CLI commands execute without errors
- ✅ All commands produce expected output
- ✅ Error messages are clear and helpful
- ✅ Background execution works correctly
- ✅ Queue management (pause/resume/cancel) works correctly
- ✅ Configuration management works correctly
- ✅ Scheduling works correctly
- ✅ Experiments can be enabled/disabled
- ✅ Results can be merged
- ✅ Rollback works correctly
- ✅ Metrics are captured and displayed
- ✅ Memory operations work correctly
- ✅ Validation works correctly

## Performance Targets

- Command execution time: < 1 second for most commands
- Help command: < 100ms
- Validation: < 500ms
- Workflow execution: Depends on backend, but CLI should be responsive

## Failure Procedures

**If a command fails:**
1. Review error message
2. Check if prerequisites are met (backend running, valid input, etc.)
3. Run with `--verbose` or `--debug` flag for more information
4. Check logs for additional context
5. Verify command syntax
6. Fix the issue and re-test

**If command hangs:**
1. Check if backend is responsive
2. Verify network connectivity
3. Check system resources
4. Try with timeout
5. Kill and retry

**If output is incorrect:**
1. Verify input parameters
2. Check configuration
3. Review expected vs actual output
4. Fix command or implementation
5. Re-test

## Sign-Off Checklist

- [ ] All `run` command tests pass
- [ ] All `generate` command tests pass
- [ ] All `queue` command tests pass
- [ ] All `config` command tests pass
- [ ] All `schedule` command tests pass
- [ ] All `experiment` command tests pass
- [ ] All `merge` command tests pass
- [ ] All `rollback` command tests pass
- [ ] All `metrics` command tests pass
- [ ] All `memory` command tests pass
- [ ] All `validate` command tests pass
- [ ] All `help` command tests pass
- [ ] All verification commands pass
- [ ] Performance targets met
