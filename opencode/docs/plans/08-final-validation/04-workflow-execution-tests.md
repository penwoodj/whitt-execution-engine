# Validation 04: Workflow Execution Tests

## Purpose
Execute ALL 53 example workflows and verify correct output for each of 19 categories.

## Prerequisites

**Ensure at least one backend is running:**

```bash
# Test Ollama
curl -s http://localhost:11434/api/tags > /dev/null || echo "Ollama not running, please start it"

# Test LM Studio
curl -s http://localhost:1234/v1/models > /dev/null || echo "LM Studio not running, please start it"

# Test llama.cpp
curl -s http://localhost:8080/health > /dev/null || echo "llama.cpp not running, please start it"
```

**Configure default backend:**

```bash
cargo run --bin agentsdk -- config set backend ollama
cargo run --bin agentsdk -- config set backend-url http://localhost:11434
```

**List all example workflows:**

```bash
find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*.yaml" | sort
```

## Test Procedures

### 1. Category: Simple Execution

**Workflows to test:**
- Simple one-step workflows

**Test:**
```bash
# Find simple execution workflows
SIMPLE_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*simple*" -o -name "*basic*")

for workflow in $SIMPLE_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- All simple workflows execute successfully
- Output contains expected results
- Exit code is 0

**Verification:**
```bash
# Verify all simple workflows passed
grep -c "✓" /tmp/test-results.log
# Expected: Count > 0

# Verify no failures
grep -c "✗" /tmp/test-results.log
# Expected: 0

# Check results are not empty
ls -lh /tmp/result-*.json
# Expected: All files have content
```

### 2. Category: Parallel Models

**Workflows to test:**
- Workflows that execute multiple models in parallel

**Test:**
```bash
# Find parallel model workflows
PARALLEL_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*parallel*" -o -name "*concurrent*")

for workflow in $PARALLEL_WORKFLOWS; do
  echo "Testing: $workflow"
  /usr/bin/time -v cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Parallel workflows execute correctly
- All parallel steps complete
- Results from all parallel branches are returned
- Execution time shows parallelism (faster than sequential)

**Verification:**
```bash
# Verify parallel workflows passed
grep -c "✓" /tmp/test-results.log
# Expected: Count > 0

# Check execution time is reasonable (< 5 minutes for most)
grep "User time" /tmp/result-*.json
# Expected: Reasonable time (not sequential sum)

# Verify all results are present
cat /tmp/result-*.json | jq '.results | length'
# Expected: Matches number of parallel branches
```

### 3. Category: Sequential Steps

**Workflows to test:**
- Workflows with sequential step execution

**Test:**
```bash
# Find sequential workflows
SEQUENTIAL_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*sequential*" -o -name "*step*")

for workflow in $SEQUENTIAL_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Steps execute in correct order
- Output from each step is passed to the next
- Final result reflects all steps
- No step is skipped

**Verification:**
```bash
# Verify sequential workflows passed
grep -c "✓" /tmp/test-results.log
# Expected: Count > 0

# Verify step order in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="execution") | .message' | grep "step"
# Expected: Shows step 1, step 2, step 3, etc. in order
```

### 4. Category: Conditional Logic

**Workflows to test:**
- Workflows with if/else logic

**Test:**
```bash
# Find conditional workflows
CONDITIONAL_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*conditional*" -o -name "*if*")

for workflow in $CONDITIONAL_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Conditional branches are evaluated correctly
- Correct branch is executed based on conditions
- Results match expected branch output
- Both branches can be executed when conditions change

**Verification:**
```bash
# Verify conditional workflows passed
grep -c "✓" /tmp/test-results.log
# Expected: Count > 0

# Verify conditional logic in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.message | contains("condition" or "if" or "else")) | .message'
# Expected: Shows conditional evaluation
```

### 5. Category: Error Handling

**Workflows to test:**
- Workflows with error handling logic

**Test:**
```bash
# Find error handling workflows
ERROR_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*error*" -o -name "*retry*")

for workflow in $ERROR_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  # For error handling workflows, exit code may be non-zero but error should be handled gracefully
  echo "Test completed: $workflow"
done
```

**Expected Results:**
- Errors are caught and handled
- Retry logic works (if applicable)
- Workflow continues or exits gracefully
- Error messages are informative

**Verification:**
```bash
# Verify error handling in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.level=="error") | .message' | head -10
# Expected: Shows errors were handled

# Check for retry attempts
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.message | contains("retry")) | .message'
# Expected: Shows retry attempts if configured
```

### 6. Category: Retry Logic

**Workflows to test:**
- Workflows that retry on failure

**Test:**
```bash
# Find retry workflows
RETRY_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*retry*")

for workflow in $RETRY_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  echo "Test completed: $workflow"
done
```

**Expected Results:**
- Failed operations are retried
- Retry count is respected
- Exponential backoff works (if configured)
- Workflow eventually succeeds or fails gracefully

**Verification:**
```bash
# Verify retry attempts in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.message | contains("retry" or "attempt")) | .message'
# Expected: Shows retry attempts

# Verify backoff
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.message | contains("backoff" or "wait")) | .message'
# Expected: Shows backoff if configured
```

### 7. Category: Timeout Handling

**Workflows to test:**
- Workflows with timeout configuration

**Test:**
```bash
# Find timeout workflows
TIMEOUT_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*timeout*")

for workflow in $TIMEOUT_WORKFLOWS; do
  echo "Testing: $workflow"
  timeout 120 cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  echo "Test completed: $workflow"
done
```

**Expected Results:**
- Timeouts are enforced
- Timeout errors are caught
- Workflow handles timeout gracefully
- No hanging processes

**Verification:**
```bash
# Verify timeout handling in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.message | contains("timeout")) | .message'
# Expected: Shows timeout events

# Verify no hanging processes
ps aux | grep agentsdk
# Expected: No orphaned agent SDK processes
```

### 8. Category: Memory Operations

**Workflows to test:**
- Workflows that read/write memory

**Test:**
```bash
# Find memory workflows
MEMORY_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*memory*")

for workflow in $MEMORY_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Memory values are written correctly
- Memory values are read correctly
- Memory persists between steps
- Memory is cleaned up (if configured)

**Verification:**
```bash
# Verify memory operations in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="execution" and (.message | contains("memory" or "store" or "read"))) | .message'
# Expected: Shows memory operations

# Verify memory content
cargo run --bin agentsdk -- memory list
# Expected: Shows stored values
```

### 9. Category: State Persistence

**Workflows to test:**
- Workflows that save/restore state

**Test:**
```bash
# Find state persistence workflows
STATE_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*state*" -o -name "*persist*")

for workflow in $STATE_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- State is saved correctly
- State is restored correctly
- Resumed workflows continue from saved state
- State file format is valid

**Verification:**
```bash
# Verify state files exist
find ~/.agentsdk/state/ -name "*.json"
# Expected: Shows state files

# Verify state can be restored
cat ~/.agentsdk/state/*.json | jq '.'
# Expected: Valid JSON with state information
```

### 10. Category: Model Selection

**Workflows to test:**
- Workflows that select different models

**Test:**
```bash
# Find model selection workflows
MODEL_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*model*")

for workflow in $MODEL_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Correct model is selected
- Model parameters are applied
- Model-specific features work
- Multiple models can be used in one workflow

**Verification:**
```bash
# Verify model selection in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model") | .message' | head -10
# Expected: Shows model selection and parameters

# Verify different models were used
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.message | contains("model")) | .metadata.model_name' | sort -u
# Expected: Shows multiple model names
```

### 11. Category: Parameter Passing

**Workflows to test:**
- Workflows with complex parameter passing

**Test:**
```bash
# Find parameter workflows
PARAM_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*param*")

for workflow in $PARAM_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" --param test="value" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Parameters are passed correctly
- Parameters are used in workflow
- Complex parameter types work (objects, arrays)
- Default parameters work

**Verification:**
```bash
# Verify parameter usage in results
cat /tmp/result-*.json | jq '.parameters'
# Expected: Shows passed parameters

# Verify parameter types
cat /tmp/result-*.json | jq '.parameters | type'
# Expected: Correct types (object, array, string, etc.)
```

### 12. Category: Output Formatting

**Workflows to test:**
- Workflows with output formatting

**Test:**
```bash
# Find output formatting workflows
OUTPUT_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*output*" -o -name "*format*")

for workflow in $OUTPUT_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" --output-format json > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Output format is correct (JSON, YAML, text)
- Output structure matches schema
- Output is complete
- Output is valid

**Verification:**
```bash
# Verify output format
cat /tmp/result-*.json | jq '.'
# Expected: Valid JSON

# Verify output structure
cat /tmp/result-*.json | jq 'keys'
# Expected: Shows expected keys
```

### 13. Category: Logging Configuration

**Workflows to test:**
- Workflows with custom logging

**Test:**
```bash
# Find logging workflows
LOGGING_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*log*")

for workflow in $LOGGING_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" --log-level debug > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Logging level is respected
- Logs are written to correct location
- Log format is correct
- Logs contain expected information

**Verification:**
```bash
# Verify logs were written
ls -lh ~/.agentsdk/logs/agentsdk.log
# Expected: Log file exists and has content

# Verify log level
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.level=="debug") | .message' | head -5
# Expected: Shows debug messages
```

### 14. Category: Queue Operations

**Workflows to test:**
- Workflows that use queue

**Test:**
```bash
# Find queue workflows
QUEUE_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*queue*")

for workflow in $QUEUE_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run --background "$workflow"
  sleep 5
  cargo run --bin agentsdk -- queue list > /tmp/queue-status.txt
  if grep -q "running\|queued\|completed" /tmp/queue-status.txt; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Workflows are queued correctly
- Queue status is accurate
- Workflows execute from queue
- Queue is cleaned up (if configured)

**Verification:**
```bash
# Verify queue status
cat /tmp/queue-status.txt
# Expected: Shows workflow status

# Verify queue operations in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="queue") | .message'
# Expected: Shows queue operations
```

### 15. Category: Scheduler Integration

**Workflows to test:**
- Workflows that use scheduler

**Test:**
```bash
# Find scheduler workflows
SCHEDULER_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*schedule*")

for workflow in $SCHEDULER_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow"
  sleep 2
  cargo run --bin agentsdk -- schedule list > /tmp/schedule-status.txt
  if grep -q "scheduled\|executing" /tmp/schedule-status.txt; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Workflows are scheduled correctly
- Scheduled time is accurate
- Workflows execute at scheduled time
- Schedule can be modified

**Verification:**
```bash
# Verify schedule status
cat /tmp/schedule-status.txt
# Expected: Shows scheduled workflows

# Verify scheduler operations in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="scheduler") | .message'
# Expected: Shows scheduler operations
```

### 16. Category: Backend Switching

**Workflows to test:**
- Workflows that use multiple backends

**Test:**
```bash
# Find backend switching workflows
BACKEND_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*backend*" -o -name "*ollama*" -o -name "*llama*")

for workflow in $BACKEND_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Correct backend is selected
- Multiple backends can be used
- Backend-specific parameters work
- Backend switching is seamless

**Verification:**
```bash
# Verify backend usage in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="backend") | .message' | head -10
# Expected: Shows backend operations

# Verify multiple backends were used
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.message | contains("backend")) | .metadata.backend_name' | sort -u
# Expected: Shows multiple backend names
```

### 17. Category: Custom Prompts

**Workflows to test:**
- Workflows with custom prompts

**Test:**
```bash
# Find custom prompt workflows
PROMPT_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*prompt*")

for workflow in $PROMPT_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Custom prompts are sent correctly
- Prompt variables are substituted
- System prompts work
- User prompts work

**Verification:**
```bash
# Verify prompt usage in results
cat /tmp/result-*.json | jq '.prompt'
# Expected: Shows prompt content

# Verify prompt substitution
cat /tmp/result-*.json | jq '.prompt | contains("{{")'  # Should be false after substitution
# Expected: false (variables substituted)
```

### 18. Category: Temperature Control

**Workflows to test:**
- Workflows with temperature configuration

**Test:**
```bash
# Find temperature workflows
TEMP_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*temp*")

for workflow in $TEMP_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Temperature parameter is passed to backend
- Different temperatures produce different results
- Temperature range is respected (0.0-2.0)

**Verification:**
```bash
# Verify temperature in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.message | contains("temperature")) | .metadata.temperature'
# Expected: Shows temperature values

# Compare results from different temperatures
# (Manual verification: higher temperature should produce more creative/different outputs)
```

### 19. Category: Max Tokens

**Workflows to test:**
- Workflows with max tokens configuration

**Test:**
```bash
# Find max tokens workflows
TOKENS_WORKFLOWS=$(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*token*")

for workflow in $TOKENS_WORKFLOWS; do
  echo "Testing: $workflow"
  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$(basename $workflow).json" 2>&1
  if [ $? -eq 0 ]; then
    echo "✓ $workflow passed"
  else
    echo "✗ $workflow failed"
  fi
done
```

**Expected Results:**
- Max tokens parameter is passed to backend
- Output is limited to max tokens
- Token count is accurate

**Verification:**
```bash
# Verify max tokens in logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.message | contains("max_tokens")) | .metadata.max_tokens'
# Expected: Shows max_tokens values

# Verify output length
cat /tmp/result-*.json | jq -r '.output' | wc -c
# Expected: Length is reasonable (not excessively long)
```

## Bulk Test All 53 Workflows

**Test:**
```bash
# Run all 53 workflows
TOTAL=0
PASSED=0
FAILED=0

for workflow in $(find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*.yaml" | sort); do
  TOTAL=$((TOTAL + 1))
  WORKFLOW_NAME=$(basename "$workflow")
  echo "[$TOTAL/53] Testing: $WORKFLOW_NAME"

  cargo run --bin agentsdk -- run "$workflow" > "/tmp/result-$WORKFLOW_NAME.log" 2>&1
  EXIT_CODE=$?

  if [ $EXIT_CODE -eq 0 ]; then
    echo "✓ $WORKFLOW_NAME passed"
    PASSED=$((PASSED + 1))
  else
    echo "✗ $WORKFLOW_NAME failed (exit code: $EXIT_CODE)"
    FAILED=$((FAILED + 1))
    echo "Error output:"
    cat "/tmp/result-$WORKFLOW_NAME.log" | tail -20
  fi
done

echo ""
echo "Test Summary:"
echo "Total: $TOTAL"
echo "Passed: $PASSED"
echo "Failed: $FAILED"

if [ $FAILED -eq 0 ]; then
  echo "All tests passed!"
  exit 0
else
  echo "Some tests failed!"
  exit 1
fi
```

**Expected Results:**
- All 53 workflows execute
- All workflows pass (0 failures)
- Execution completes in reasonable time

**Verification:**
```bash
# Verify all workflows executed
find /tmp/result-*.log | wc -l
# Expected: 53

# Verify pass count
grep -c "✓" /tmp/test-summary.log
# Expected: 53

# Verify no failures
grep -c "✗" /tmp/test-summary.log
# Expected: 0
```

## Success Criteria

- ✅ All 53 workflows execute without crashes
- ✅ All workflows produce valid output
- ✅ All 19 categories work correctly
- ✅ No workflows timeout unexpectedly
- ✅ No workflows hang
- ✅ Error handling works across all workflows
- ✅ Logs are complete and informative

## Performance Targets

- Average workflow execution time: < 2 minutes
- Total time for all 53 workflows: < 2 hours
- No individual workflow takes > 10 minutes
- Workflow startup time: < 5 seconds

## Failure Procedures

**If a workflow fails:**
1. Review error output in `/tmp/result-<workflow-name>.log`
2. Check logs in `~/.agentsdk/logs/agentsdk.log`
3. Verify backend is running and accessible
4. Validate workflow YAML: `cargo run --bin agentsdk -- validate <workflow>`
5. Check for missing dependencies (models, files)
6. Fix the issue (workflow bug, backend issue, dependency issue)
7. Re-run the workflow
8. Only proceed when all workflows pass

**If multiple workflows fail:**
1. Identify common patterns
2. Check if backend is the issue
3. Check if configuration is correct
4. Review recent code changes
5. Fix root cause
6. Re-run all failed workflows

**If workflows timeout:**
1. Check backend response time
2. Increase timeout if necessary
3. Optimize workflow if too complex
4. Reduce model count if too many parallel models
5. Re-run with adjusted configuration

## Sign-Off Checklist

- [ ] All 53 workflows executed
- [ ] All 19 categories verified
- [ ] All workflows pass (0 failures)
- [ ] Execution time meets targets
- [ ] No workflows hang or crash
- [ ] Error handling verified
- [ ] Logs complete and informative
- [ ] Results validated for each category
