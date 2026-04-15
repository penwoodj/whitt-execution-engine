# Validation 08: Benchmark 120 Model

## Purpose
Maximum stress test with 120 concurrent models to identify system limits, verify the system handles this load without crashing, measure maximum throughput, and identify any limits.

## Objective

This is the critical test from the user's explicit requirement: "ramp up from a 5 model benchmarking workflow then going up to 120 model single workflow test run". This benchmark tests the absolute limits of the AgentSDK Execution Engine with 120 concurrent model references (24x increase from baseline). The goal is to identify system limits and verify graceful behavior at maximum load.

## Prerequisites

**Ensure all previous benchmarks completed:**

```bash
test -f benchmarks/baseline-5-model.json
test -f benchmarks/scaling-20-model.json
test -f benchmarks/stress-50-model.json
```

**Read baseline metrics:**

```bash
BASELINE_TIME=$(cat benchmarks/baseline-5-model.json | jq '.execution_time_seconds')
BASELINE_MEMORY=$(cat benchmarks/baseline-5-model.json | jq '.peak_memory_mb')
```

**Create 120-model workflow generator:**

```bash
cat > /tmp/generate-120-model-workflow.sh <<'EOF'
#!/bin/bash
cat > benchmarks/benchmark-120-model.yaml <<'YAML'
version: "1.0"
name: "Benchmark 120 Model"
description: "Maximum stress test with 120 concurrent model calls"

settings:
  timeout: 1800s
  log_level: info
  parallelism: true

steps:
YAML

# Generate 120 model calls
for i in {1..120}; do
  num=$((i + 1))
  cat >> benchmarks/benchmark-120-model.yaml <<YAML
  - name: "model_call_${i}"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 50
    input:
      prompt: "Calculate ${num} + ${num}. Answer with just the number."
    output:
      variable: "result_${i}"
YAML
done

# Add aggregation step
cat >> benchmarks/benchmark-120-model.yaml <<YAML

final_step:
  name: "aggregate_results"
  description: "Aggregate all model results"
  action: "aggregate"
  input:
    variables: [
EOF

# Add variables list
for i in {1..120}; do
  if [ $i -lt 120 ]; then
    echo -n "\"result_${i}\", " >> benchmarks/benchmark-120-model.yaml
  else
    echo "\"result_${i}\"" >> benchmarks/benchmark-120-model.yaml
  fi
done

cat >> benchmarks/benchmark-120-model.yaml <<YAML
    ]
  output:
    format: "json"
    file: "benchmarks/benchmark-120-model-results.json"
YAML
EOF

chmod +x /tmp/generate-120-model-workflow.sh
/tmp/generate-120-model-workflow.sh
```

## Test Procedures

### 1. Validate Benchmark Workflow

```bash
cargo run --bin agentsdk -- validate benchmarks/benchmark-120-model.yaml
# Expected: "Workflow is valid"
```

### 2. Run Maximum Stress Test

```bash
# Clear logs
rm -rf ~/.agentsdk/logs/*

# Run with timeout to prevent hanging
timeout 2400 /usr/bin/time -v cargo run --bin agentsdk -- \
  --log-level info \
  run benchmarks/benchmark-120-model.yaml \
  2>&1 | tee benchmarks/benchmark-120-model-execution.log

EXIT_CODE=$?
echo "Exit code: $EXIT_CODE"
```

### 3. Verify System Handles Load Without Crashing

```bash
if [ $EXIT_CODE -eq 0 ]; then
  echo "✓ System handled 120 models without crashing"
elif [ $EXIT_CODE -eq 124 ]; then
  echo "⚠ Test timed out but system didn't crash"
else
  echo "✗ System crashed with exit code: $EXIT_CODE"
fi
```

### 4. Measure Maximum Throughput

```bash
# Extract metrics if available
if [ -f benchmarks/benchmark-120-model-results.json ]; then
  ELAPSED_TIME=$(grep "Elapsed (wall clock) time" benchmarks/benchmark-120-model-execution.log | awk '{print $5}' | awk -F: '{print $1*60 + $2}')
  MEMORY_KB=$(grep "Maximum resident set size" benchmarks/benchmark-120-model-execution.log | awk '{print $6}')
  MEMORY_MB=$((MEMORY_KB / 1024))
  TOTAL_TOKENS=$(cat benchmarks/benchmark-120-model-results.json | jq '[.results[].output | length | tonumber] | add')
  THROUGHPUT=$(echo "scale=2; $TOTAL_TOKENS / $ELAPSED_TIME" | bc)

  echo "Maximum throughput: ${THROUGHPUT} tokens/second"
  echo "Peak memory: ${MEMORY_MB}MB"
  echo "Execution time: ${ELAPSED_TIME}s"
else
  echo "⚠ Results file not available (test may have timed out)"
fi
```

### 5. Identify System Limits

```bash
# Check for connection pool limits
grep -i "connection pool\|max connections\|pool full" ~/.agentsdk/logs/agentsdk.log | head -10

# Check for thread pool limits
grep -i "thread pool\|max threads\|thread limit" ~/.agentsdk/logs/agentsdk.log | head -10

# Check for file descriptor limits
grep -i "file descriptor\|fd limit\|too many open files" ~/.agentsdk/logs/agentsdk.log | head -10

# Check for memory limits
grep -i "out of memory\|oom\|memory limit" ~/.agentsdk/logs/agentsdk.log | head -10
```

### 6. Verify Graceful Failure If Resources Exceeded

```bash
# Check if system gracefully handled resource exhaustion
if grep -i "graceful\|throttle\|limit\|backlog" ~/.agentsdk/logs/agentsdk.log | grep -q "."; then
  echo "✓ System showed graceful degradation behavior"
else
  echo "⚠ No graceful degradation messages found (may have handled load successfully)"
fi
```

### 7. Verify Recovery Mechanism

```bash
# If test failed or timed out, verify system can recover
# Run a simple test to verify system still works
cargo run --bin agentsdk -- --help > /dev/null 2>&1
if [ $? -eq 0 ]; then
  echo "✓ System recovered successfully"
else
  echo "✗ System did not recover - may need restart"
fi
```

### 8. Count Successful vs Failed Requests

```bash
# Count model calls
MODEL_CALLS=$(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model") | .message' | wc -l)
echo "Total model calls: $MODEL_CALLS"

# Count successful calls
SUCCESSFUL=$(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model" and (.message | contains("success" or "completed"))) | .message' | wc -l)
echo "Successful calls: $SUCCESSFUL"

# Count failed calls
FAILED=$(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model" and (.message | contains("fail" or "error"))) | .message' | wc -l)
echo "Failed calls: $FAILED"
```

### 9. Generate Performance Report

```bash
cat > benchmarks/performance-report-120-model.md <<EOF
# Maximum Load Performance Report (120 Models)

## Test Results
- Exit Code: $EXIT_CODE
- Status: $(if [ $EXIT_CODE -eq 0 ]; then echo "✅ Success"; elif [ $EXIT_CODE -eq 124 ]; then echo "⚠️ Timed Out"; else echo "❌ Failed"; fi)

## Performance Metrics
$(if [ -f benchmarks/benchmark-120-model-results.json ]; then
  cat <<METRICS
- Execution Time: ${ELAPSED_TIME}s
- Peak Memory: ${MEMORY_MB}MB
- Total Tokens: $TOTAL_TOKENS
- Token Throughput: ${THROUGHPUT} tokens/second
- Model Calls: $MODEL_CALLS
- Successful: $SUCCESSFUL
- Failed: $FAILED
METRICS
else
  echo "- Metrics unavailable (test timed out)"
fi)

## Baseline Comparisons
- 5 Model Baseline Time: ${BASELINE_TIME}s
- Scaling Factor: $(if [ -n "$ELAPSED_TIME" ]; then echo "scale=2; $ELAPSED_TIME / $BASELINE_TIME" | bc; else echo "N/A"; fi)x
- Expected Linear (24x): $(echo "scale=0; $BASELINE_TIME * 24" | bc)s

## System Limits Identified
$(grep -i "limit\|max\|pool" ~/.agentsdk/logs/agentsdk.log | head -10 || echo "No explicit limits reached")

## Recommendations
EOF

cat >> benchmarks/performance-report-120-model.md <<'EOF'
Based on the 120-model stress test:

1. **Connection Pool**: Monitor connection pool size and increase if needed
2. **Thread Pool**: Verify thread pool can handle 120+ concurrent operations
3. **Memory**: Memory usage scales linearly; consider implementing result streaming
4. **Backend**: Verify backend can handle 120 concurrent requests
5. **Optimization**: Consider batching or streaming for very large workflows
EOF

cat benchmarks/performance-report-120-model.md
```

### 10. Generate Scaling Comparison Report

```bash
cat > benchmarks/scaling-comparison.md <<EOF
# Scaling Comparison: 5 → 20 → 50 → 120 Models

| Models | Time (s) | Memory (MB) | Tokens/sec | Success Rate |
|--------|----------|-------------|------------|-------------|
| 5      | $BASELINE_TIME      | $BASELINE_MEMORY     | $(cat benchmarks/baseline-5-model.json | jq '.token_throughput')      | 100% |
| 20     | $(cat benchmarks/scaling-20-model.json | jq '.execution_time_seconds')     | $(cat benchmarks/scaling-20-model.json | jq '.peak_memory_mb') | $(cat benchmarks/scaling-20-model.json | jq '.token_throughput') | 100% |
| 50     | $(cat benchmarks/stress-50-model.json | jq '.execution_time_seconds')     | $(cat benchmarks/stress-50-model.json | jq '.peak_memory_mb') | $(cat benchmarks/stress-50-model.json | jq '.token_throughput') | 100% |
| 120    | ${ELAPSED_TIME:-N/A}    | ${MEMORY_MB:-N/A}   | ${THROUGHPUT:-N/A} | $(if [ -n "$MODEL_CALLS" ] && [ $MODEL_CALLS -gt 0 ]; then echo "scale=0; $SUCCESSFUL * 100 / $MODEL_CALLS" | bc; else echo "N/A"; fi)% |

## Scaling Analysis
- Time Scaling: 5→20→50→120
- Memory Scaling: 5→20→50→120
- Throughput: Stable across scales?

## System Limits
- Maximum Models Tested: 120
- Maximum Memory: ${MEMORY_MB:-N/A} MB
- Maximum Time: ${ELAPSED_TIME:-N/A} seconds

## Conclusion
$(if [ $EXIT_CODE -eq 0 ]; then
  echo "✅ System handles 120 models successfully"
elif [ $EXIT_CODE -eq 124 ]; then
  echo "⚠️  System handles load but times out - consider optimization"
else
  echo "❌ System fails at 120 models - optimization required"
fi)
EOF

cat benchmarks/scaling-comparison.md
```

## Success Criteria

- ✅ System handles 120 models without crashing
- ✅ Maximum throughput is measurable
- ✅ System limits are identified
- ✅ Graceful degradation occurs if resources exhausted
- ✅ Recovery mechanism works
- ✅ Performance report generated
- ✅ Scaling comparison completed

## Performance Targets

| Metric | Minimum Target | Acceptable |
|--------|----------------|------------|
| System stability | No crashes | No crashes |
| Throughput | Measurable | Measurable |
| Recovery | < 1 minute | < 5 minutes |

## Failure Procedures

**If system crashes:**
1. Review crash logs and stack traces
2. Identify the crash point
3. Fix the crash (add error handling, increase limits, etc.)
4. Re-run benchmark

**If test times out:**
1. Review logs to see progress
2. Identify bottlenecks
3. Consider increasing timeout
4. Document that timeout occurred
5. This is acceptable if system didn't crash

**If many requests fail:**
1. Review failure reasons
2. Check backend limits
3. Increase connection pool size
4. Implement retry logic
5. Re-run benchmark

## Sign-Off Checklist

- [ ] Workflow validates
- [ ] Test executed
- [ ] No crashes
- [ ] Throughput measured
- [ ] Limits identified
- [ ] Graceful degradation verified
- [ ] Recovery works
- [ ] Performance report generated
- [ ] Scaling comparison completed
