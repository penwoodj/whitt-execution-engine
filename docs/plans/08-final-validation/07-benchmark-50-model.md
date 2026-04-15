# Validation 07: Benchmark 50 Model

## Purpose
Stress test with 50 concurrent models to verify parallel execution handles high load, measure memory pressure, and verify graceful degradation.

## Objective

This stress test validates the AgentSDK Execution Engine's ability to handle 50 concurrent model references (10x increase from baseline). The goal is to verify that parallel execution works correctly, memory pressure is manageable, and graceful degradation occurs if resources are constrained.

## Prerequisites

**Ensure 5-model baseline and 20-model results exist:**

```bash
test -f benchmarks/baseline-5-model.json
test -f benchmarks/scaling-20-model.json
```

**Read previous metrics:**

```bash
BASELINE_TIME=$(cat benchmarks/baseline-5-model.json | jq '.execution_time_seconds')
BASELINE_MEMORY=$(cat benchmarks/baseline-5-model.json | jq '.peak_memory_mb')
TIME_20_MODEL=$(cat benchmarks/scaling-20-model.json | jq '.execution_time_seconds')
MEM_20_MODEL=$(cat benchmarks/scaling-20-model.json | jq '.peak_memory_mb')
```

**Create benchmark workflow generator script:**

```bash
cat > /tmp/generate-50-model-workflow.sh <<'EOF'
#!/bin/bash
cat > benchmarks/benchmark-50-model.yaml <<'YAML'
version: "1.0"
name: "Benchmark 50 Model"
description: "Stress test with 50 concurrent model calls"

settings:
  timeout: 900s
  log_level: info
  parallelism: true

steps:
YAML

# Generate 50 model calls
for i in {1..50}; do
  num=$((i + 1))
  cat >> benchmarks/benchmark-50-model.yaml <<YAML
  - name: "model_call_${i}"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is ${num} + ${num}? Answer with just the number."
    output:
      variable: "result_${i}"
YAML
done

# Add aggregation step
cat >> benchmarks/benchmark-50-model.yaml <<YAML

final_step:
  name: "aggregate_results"
  description: "Aggregate all model results"
  action: "aggregate"
  input:
    variables: [
EOF

# Add variables list
for i in {1..50}; do
  if [ $i -lt 50 ]; then
    echo -n "\"result_${i}\", " >> benchmarks/benchmark-50-model.yaml
  else
    echo "\"result_${i}\"" >> benchmarks/benchmark-50-model.yaml
  fi
done

cat >> benchmarks/benchmark-50-model.yaml <<YAML
    ]
  output:
    format: "json"
    file: "benchmarks/benchmark-50-model-results.json"
YAML
EOF

chmod +x /tmp/generate-50-model-workflow.sh
/tmp/generate-50-model-workflow.sh
```

## Test Procedures

### 1. Validate Benchmark Workflow

```bash
cargo run --bin agentsdk -- validate benchmarks/benchmark-50-model.yaml
# Expected: "Workflow is valid"
```

### 2. Run Benchmark with Resource Monitoring

```bash
# Start resource monitoring in background
/usr/bin/time -v cargo run --bin agentsdk -- \
  --log-level info \
  run benchmarks/benchmark-50-model.yaml \
  2>&1 | tee benchmarks/benchmark-50-model-execution.log

# Record metrics
ELAPSED_TIME=$(grep "Elapsed (wall clock) time" benchmarks/benchmark-50-model-execution.log | awk '{print $5}' | awk -F: '{print $1*60 + $2}')
MEMORY_KB=$(grep "Maximum resident set size" benchmarks/benchmark-50-model-execution.log | awk '{print $6}')
MEMORY_MB=$((MEMORY_KB / 1024))
```

### 3. Verify Parallel Execution

```bash
# Count concurrent executions
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="execution") | .message' | grep "concurrent\|parallel" | wc -l
# Expected: > 0 (shows parallel execution)

# Verify no deadlocks
grep -i "deadlock" ~/.agentsdk/logs/agentsdk.log | wc -l
# Expected: 0
```

### 4. Measure Memory Pressure

```bash
echo "Memory usage: ${MEMORY_MB}MB"
echo "Target: < 5GB"

if [ $MEMORY_MB -lt 5120 ]; then
  echo "✓ Memory within target"
else
  echo "✗ Memory exceeds target - investigate memory leaks or optimize"
fi
```

### 5. Verify Graceful Degradation

```bash
# Check for graceful degradation messages
grep -i "graceful\|degrade\|throttle" ~/.agentsdk/logs/agentsdk.log | head -5

# Check if any requests were throttled
grep -i "throttle\|rate limit" ~/.agentsdk/logs/agentsdk.log | wc -l
# Expected: May be > 0 if system self-throttles (this is acceptable)
```

### 6. Verify No Dropped Requests

```bash
# Count model calls
MODEL_CALLS=$(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model") | .message' | wc -l)
echo "Model calls: $MODEL_CALLS"
# Expected: >= 50

# Check for dropped/failed requests
grep -i "dropped\|lost\|failed.*request" ~/.agentsdk/logs/agentsdk.log | wc -l
# Expected: 0
```

### 7. Verify System Responsiveness

```bash
# Run a quick command during execution (in another terminal)
# System should remain responsive
timeout 5 cargo run --bin agentsdk -- --help > /dev/null
echo "CLI responsiveness test: $?"
# Expected: 0 (CLI responds quickly)
```

### 8. Record Metrics

```bash
TOTAL_TOKENS=$(cat benchmarks/benchmark-50-model-results.json | jq '[.results[].output | length | tonumber] | add')
THROUGHPUT=$(echo "scale=2; $TOTAL_TOKENS / $ELAPSED_TIME" | bc)

cat > benchmarks/stress-50-model.json <<EOF
{
  "benchmark_name": "50-model stress test",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "num_models": 50,
  "execution_time_seconds": $ELAPSED_TIME,
  "peak_memory_kb": $MEMORY_KB,
  "peak_memory_mb": $MEMORY_MB,
  "total_tokens": $TOTAL_TOKENS,
  "token_throughput": $THROUGHPUT,
  "baseline_time_seconds": $BASELINE_TIME,
  "baseline_memory_mb": $BASELINE_MEMORY,
  "20_model_time_seconds": $TIME_20_MODEL,
  "20_model_memory_mb": $MEM_20_MODEL,
  "exit_code": 0,
  "status": "completed"
}
EOF
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Execution time | < 10 minutes |
| Peak memory | < 5GB |
| Parallel execution | Verified |
| Dropped requests | 0 |
| CLI responsiveness | < 1 second |

## Success Criteria

- ✅ Workflow completes successfully
- ✅ All 50 model calls complete
- ✅ No deadlocks or race conditions
- ✅ Peak memory < 5GB
- ✅ No dropped or lost requests
- ✅ CLI remains responsive
- ✅ Graceful degradation occurs if needed

## Failure Procedures

**If deadlock detected:**
1. Review synchronization code
2. Check for lock ordering issues
3. Verify no circular dependencies
4. Fix deadlock and re-run

**If memory > 5GB:**
1. Check for memory leaks
2. Review data structures
3. Optimize memory usage
4. Consider batching or streaming

**If requests dropped:**
1. Review queue implementation
2. Check buffer sizes
3. Verify error handling
4. Fix request dropping

## Sign-Off Checklist

- [ ] Workflow validates
- [ ] All 50 model calls complete
- [ ] No deadlocks
- [ ] Memory < 5GB
- [ ] No dropped requests
- [ ] CLI responsive
- [ ] Metrics saved
