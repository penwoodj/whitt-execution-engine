# Validation 05: Benchmark 5 Model

## Purpose
Establish baseline performance with 5 concurrent LLM models in a single workflow.

## Objective

This benchmark establishes the performance baseline for the AgentSDK Execution Engine with a manageable load of 5 concurrent model references. All subsequent benchmarks (20, 50, 120 models) will be compared against this baseline to identify scaling characteristics.

## Prerequisites

**Ensure backend is running and responsive:**

```bash
# Test Ollama (or your chosen backend)
curl -s http://localhost:11434/api/tags | jq '.models[].name'
# Expected: Shows available models

# Ensure at least 5 models are available
curl -s http://localhost:11434/api/tags | jq '.models | length'
# Expected: >= 5
```

**Create benchmark directory:**

```bash
mkdir -p benchmarks
```

## Test Workflow Definition

Create the benchmark workflow file `benchmarks/benchmark-5-model.yaml`:

```yaml
version: "1.0"
name: "Benchmark 5 Model"
description: "Baseline performance benchmark with 5 concurrent model calls"

# Global settings
settings:
  timeout: 300s
  log_level: debug
  parallelism: true

# Steps - 5 concurrent model calls
steps:
  - name: "model_call_1"
    description: "First model call"
    model:
      name: "llama2"  # Or your available model
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 2 + 2? Answer with just the number."
    output:
      variable: "result_1"

  - name: "model_call_2"
    description: "Second model call"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 3 + 3? Answer with just the number."
    output:
      variable: "result_2"

  - name: "model_call_3"
    description: "Third model call"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 4 + 4? Answer with just the number."
    output:
      variable: "result_3"

  - name: "model_call_4"
    description: "Fourth model call"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 5 + 5? Answer with just the number."
    output:
      variable: "result_4"

  - name: "model_call_5"
    description: "Fifth model call"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 6 + 6? Answer with just the number."
    output:
      variable: "result_5"

# Final aggregation step
final_step:
  name: "aggregate_results"
  description: "Aggregate all model results"
  action: "aggregate"
  input:
    variables: ["result_1", "result_2", "result_3", "result_4", "result_5"]
  output:
    format: "json"
    file: "benchmarks/benchmark-5-model-results.json"
```

## Test Procedures

### 1. Validate Benchmark Workflow

**Test:** Ensure the benchmark workflow is valid.

**Command:**
```bash
cargo run --bin agentsdk -- validate benchmarks/benchmark-5-model.yaml
```

**Expected Results:**
- Workflow is valid
- No validation errors

**Verification:**
```bash
# Validate workflow
cargo run --bin agentsdk -- validate benchmarks/benchmark-5-model.yaml
# Expected: "Workflow is valid" or "No errors found"
```

### 2. Run Benchmark with Time Measurement

**Test:** Execute the benchmark and measure execution time.

**Command:**
```bash
# Clear previous logs
rm -rf ~/.agentsdk/logs/*

# Run benchmark with time measurement
/usr/bin/time -v cargo run --bin agentsdk -- \
  --log-level info \
  --log-format json \
  run benchmarks/benchmark-5-model.yaml \
  2>&1 | tee benchmarks/benchmark-5-model-execution.log
```

**Expected Results:**
- Workflow completes successfully
- All 5 model calls complete
- Exit code is 0
- Results are saved to `benchmarks/benchmark-5-model-results.json`

**Verification:**
```bash
# Check exit code
cargo run --bin agentsdk -- run benchmarks/benchmark-5-model.yaml
echo "Exit code: $?"
# Expected: 0

# Verify results file exists
test -f benchmarks/benchmark-5-model-results.json
# Expected: Exit code 0

# Check results content
cat benchmarks/benchmark-5-model-results.json | jq '.'
# Expected: Shows all 5 results
```

### 3. Measure Execution Time

**Test:** Extract execution time metrics.

**Command:**
```bash
# Extract execution time from time output
grep "Elapsed (wall clock) time" benchmarks/benchmark-5-model-execution.log
```

**Expected Results:**
- Execution time is recorded
- Time is reasonable (< 60 seconds for simple queries)

**Verification:**
```bash
# Get execution time
ELAPSED_TIME=$(grep "Elapsed (wall clock) time" benchmarks/benchmark-5-model-execution.log | awk '{print $5}')
echo "Execution time: $ELAPSED_TIME"
# Expected: < 60 seconds (e.g., 0:12.34)

# Also measure with builtin timing
time cargo run --bin agentsdk -- run benchmarks/benchmark-5-model.yaml > /dev/null
# Expected: real < 60s
```

### 4. Measure Memory Usage

**Test:** Measure peak memory usage during execution.

**Command:**
```bash
# Extract memory usage from time output
grep "Maximum resident set size" benchmarks/benchmark-5-model-execution.log
```

**Expected Results:**
- Memory usage is recorded
- Peak memory < 500MB

**Verification:**
```bash
# Get memory usage
MEMORY_KB=$(grep "Maximum resident set size" benchmarks/benchmark-5-model-execution.log | awk '{print $6}')
MEMORY_MB=$((MEMORY_KB / 1024))
echo "Peak memory: ${MEMORY_MB}MB"
# Expected: < 500MB
```

### 5. Measure Token Throughput

**Test:** Calculate tokens per second.

**Command:**
```bash
# Count total tokens in results
TOTAL_TOKENS=$(cat benchmarks/benchmark-5-model-results.json | jq '[.results[].output | length | tonumber] | add')
echo "Total tokens: $TOTAL_TOKENS"

# Get execution time in seconds
ELAPSED_SECONDS=$(grep "Elapsed (wall clock) time" benchmarks/benchmark-5-model-execution.log | awk '{print $5}' | awk -F: '{print $1*60 + $2}')
echo "Elapsed seconds: $ELAPSED_SECONDS"

# Calculate throughput
THROUGHPUT=$(echo "scale=2; $TOTAL_TOKENS / $ELAPSED_SECONDS" | bc)
echo "Token throughput: ${THROUGHPUT} tokens/second"
```

**Expected Results:**
- Token throughput > 100 tokens/second

**Verification:**
```bash
# Verify throughput meets target
if [ $(echo "$THROUGHPUT > 100" | bc) -eq 1 ]; then
  echo "✓ Token throughput meets target"
else
  echo "✗ Token throughput below target"
fi
# Expected: ✓
```

### 6. Measure CPU Utilization

**Test:** Measure CPU usage during execution.

**Command:**
```bash
# Extract CPU usage from time output
grep "User time" benchmarks/benchmark-5-model-execution.log
grep "System time" benchmarks/benchmark-5-model-execution.log
```

**Expected Results:**
- CPU utilization is reasonable (< 80%)

**Verification:**
```bash
# Get user and system time
USER_TIME=$(grep "User time" benchmarks/benchmark-5-model-execution.log | awk '{print $6}')
SYSTEM_TIME=$(grep "System time" benchmarks/benchmark-5-model-execution.log | awk '{print $6}')
TOTAL_CPU_TIME=$(echo "$USER_TIME + $SYSTEM_TIME" | bc)

# Get elapsed time
ELAPSED_TIME=$(grep "Elapsed (wall clock) time" benchmarks/benchmark-5-model-execution.log | awk '{print $5}' | awk -F: '{print $1*60 + $2}')

# Calculate CPU utilization
CPU_UTIL=$(echo "scale=2; $TOTAL_CPU_TIME / $ELAPSED_TIME * 100" | bc)
echo "CPU utilization: ${CPU_UTIL}%"
# Expected: < 80%
```

### 7. Record Metrics to JSON

**Test:** Save all metrics to a JSON file for comparison.

**Command:**
```bash
# Create metrics JSON
cat > benchmarks/baseline-5-model.json <<EOF
{
  "benchmark_name": "5-model baseline",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "num_models": 5,
  "execution_time_seconds": $(echo "$ELAPSED_SECONDS" | bc),
  "peak_memory_kb": $MEMORY_KB,
  "peak_memory_mb": $MEMORY_MB,
  "total_tokens": $TOTAL_TOKENS,
  "token_throughput": $THROUGHPUT,
  "user_time_seconds": $(echo "$USER_TIME" | bc),
  "system_time_seconds": $(echo "$SYSTEM_TIME" | bc),
  "cpu_utilization_percent": $CPU_UTIL,
  "exit_code": 0,
  "status": "completed"
}
EOF

# Verify JSON
cat benchmarks/baseline-5-model.json | jq '.'
```

**Expected Results:**
- Metrics JSON is created
- All values are populated
- JSON is valid

**Verification:**
```bash
# Verify JSON is valid
cat benchmarks/baseline-5-model.json | jq '.' > /dev/null
# Expected: Exit code 0

# Verify metrics
cat benchmarks/baseline-5-model.json | jq '.num_models'
# Expected: 5
```

### 8. Log Verification

**Test:** Verify logs contain expected entries.

**Command:**
```bash
# Check for workflow logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="workflow") | .message' | head -10

# Check for model logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model") | .message' | head -10

# Check for backend logs
cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="backend") | .message' | head -10
```

**Expected Results:**
- Workflow logs show initialization and completion
- Model logs show 5 model calls
- Backend logs show 5 backend requests

**Verification:**
```bash
# Count model calls
MODEL_CALLS=$(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model" and (.message | contains("call" or "request"))) | .message' | wc -l)
echo "Model calls: $MODEL_CALLS"
# Expected: 5

# Count backend requests
BACKEND_REQUESTS=$(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="backend" and (.message | contains("request" or "send"))) | .message' | wc -l)
echo "Backend requests: $BACKEND_REQUESTS"
# Expected: 5
```

### 9. Repeat Benchmark for Consistency

**Test:** Run the benchmark 3 times to verify consistency.

**Command:**
```bash
for run in {1..3}; do
  echo "Run $run:"
  /usr/bin/time -v cargo run --bin agentsdk -- \
    run benchmarks/benchmark-5-model.yaml \
    > /dev/null 2>&1
done
```

**Expected Results:**
- All 3 runs complete successfully
- Execution times are consistent (within 20% of each other)

**Verification:**
```bash
# Run benchmark 3 times and collect times
for run in {1..3}; do
  /usr/bin/time -v cargo run --bin agentsdk -- \
    run benchmarks/benchmark-5-model.yaml \
    > /dev/null 2>&1 | grep "Elapsed (wall clock) time"
done
# Expected: Times are consistent (e.g., 0:12.34, 0:12.56, 0:12.18)
```

## Performance Targets

| Metric | Target | Baseline |
|--------|--------|----------|
| Execution time | < 60 seconds | ~15-30 seconds |
| Peak memory | < 500MB | ~200-400MB |
| Token throughput | > 100 tokens/second | ~200+ tokens/second |
| CPU utilization | < 80% | ~40-60% |
| Exit code | 0 | 0 |

## Success Criteria

- ✅ Workflow completes successfully (exit code 0)
- ✅ All 5 model calls complete
- ✅ Execution time < 60 seconds
- ✅ Peak memory < 500MB
- ✅ Token throughput > 100 tokens/second
- ✅ CPU utilization < 80%
- ✅ Metrics are saved to JSON
- ✅ Logs are complete and valid
- ✅ Results are repeatable (within 20%)

## Failure Procedures

**If workflow fails:**
1. Review error output in execution log
2. Check backend is running and accessible
3. Verify all required models are available
4. Check for network issues
5. Review logs for additional context
6. Fix the issue and re-run benchmark

**If execution time exceeds target:**
1. Check backend response time
2. Verify network latency
3. Check if backend is overloaded
4. Review if workflow is too complex
5. Consider optimizing backend or workflow

**If memory usage exceeds target:**
1. Check for memory leaks in application
2. Review memory-intensive operations
3. Verify data structures are efficient
4. Consider optimizing memory usage

**If throughput is below target:**
1. Check backend performance
2. Verify model response times
3. Check for bottlenecks in processing
4. Review if models are optimized

## Comparison with Later Benchmarks

This baseline will be used to compare against:
- **20-model benchmark**: Should be ~4x execution time (linear scaling)
- **50-model benchmark**: Should be ~10x execution time (linear scaling)
- **120-model benchmark**: Should identify scaling limits

If scaling is worse than linear, investigate:
- Connection pool exhaustion
- Thread pool saturation
- Memory pressure
- Backend limits
- Resource contention

## Sign-Off Checklist

- [ ] Benchmark workflow validated
- [ ] Benchmark executed successfully
- [ ] Execution time < 60 seconds
- [ ] Peak memory < 500MB
- [ ] Token throughput > 100 tokens/second
- [ ] CPU utilization < 80%
- [ ] Metrics saved to JSON
- [ ] Logs verified
- [ ] Benchmark repeatable (3 runs)
- [ ] All verification commands pass
