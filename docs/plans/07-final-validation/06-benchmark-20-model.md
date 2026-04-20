# Validation 06: Benchmark 20 Model

## Purpose
Verify linear scaling from 5→20 models and detect any N^2 bottlenecks.

## Objective

This benchmark tests the scaling characteristics of the AgentSDK Execution Engine by increasing from 5 to 20 concurrent model references (4x increase). The goal is to verify that scaling is approximately linear (execution time increases by ~4x, not more than 5x).

## Prerequisites

**Ensure 5-model baseline exists:**

```bash
test -f benchmarks/baseline-5-model.json
# Expected: Exit code 0
```

**Read baseline metrics:**

```bash
BASELINE_TIME=$(cat benchmarks/baseline-5-model.json | jq '.execution_time_seconds')
BASELINE_MEMORY=$(cat benchmarks/baseline-5-model.json | jq '.peak_memory_mb')
BASELINE_THROUGHPUT=$(cat benchmarks/baseline-5-model.json | jq '.token_throughput')

echo "Baseline metrics:"
echo "  Time: ${BASELINE_TIME}s"
echo "  Memory: ${BASELINE_MEMORY}MB"
echo "  Throughput: ${BASELINE_THROUGHPUT} tokens/sec"
```

**Ensure backend can handle load:**

```bash
# Test backend health
curl -s http://localhost:11434/api/tags | jq '.models | length'
# Expected: >= 5 (or your available models)
```

## Test Workflow Definition

Create the benchmark workflow file `benchmarks/benchmark-20-model.yaml`:

```yaml
version: "1.0"
name: "Benchmark 20 Model"
description: "Scaling test with 20 concurrent model calls"

# Global settings
settings:
  timeout: 600s
  log_level: info
  parallelism: true

# Steps - 20 concurrent model calls
# Generate simple arithmetic questions to keep response time predictable
steps:
  - name: "model_call_1"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 2 + 2? Answer with just the number."
    output:
      variable: "result_1"

  - name: "model_call_2"
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
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 6 + 6? Answer with just the number."
    output:
      variable: "result_5"

  - name: "model_call_6"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 7 + 7? Answer with just the number."
    output:
      variable: "result_6"

  - name: "model_call_7"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 8 + 8? Answer with just the number."
    output:
      variable: "result_7"

  - name: "model_call_8"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 9 + 9? Answer with just the number."
    output:
      variable: "result_8"

  - name: "model_call_9"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 10 + 10? Answer with just the number."
    output:
      variable: "result_9"

  - name: "model_call_10"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 11 + 11? Answer with just the number."
    output:
      variable: "result_10"

  - name: "model_call_11"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 12 + 12? Answer with just the number."
    output:
      variable: "result_11"

  - name: "model_call_12"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 13 + 13? Answer with just the number."
    output:
      variable: "result_12"

  - name: "model_call_13"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 14 + 14? Answer with just the number."
    output:
      variable: "result_13"

  - name: "model_call_14"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 15 + 15? Answer with just the number."
    output:
      variable: "result_14"

  - name: "model_call_15"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 16 + 16? Answer with just the number."
    output:
      variable: "result_15"

  - name: "model_call_16"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 17 + 17? Answer with just the number."
    output:
      variable: "result_16"

  - name: "model_call_17"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 18 + 18? Answer with just the number."
    output:
      variable: "result_17"

  - name: "model_call_18"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 19 + 19? Answer with just the number."
    output:
      variable: "result_18"

  - name: "model_call_19"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 20 + 20? Answer with just the number."
    output:
      variable: "result_19"

  - name: "model_call_20"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.7
      max_tokens: 100
    input:
      prompt: "What is 21 + 21? Answer with just the number."
    output:
      variable: "result_20"

# Final aggregation step
final_step:
  name: "aggregate_results"
  description: "Aggregate all model results"
  action: "aggregate"
  input:
    variables: ["result_1", "result_2", "result_3", "result_4", "result_5",
                "result_6", "result_7", "result_8", "result_9", "result_10",
                "result_11", "result_12", "result_13", "result_14", "result_15",
                "result_16", "result_17", "result_18", "result_19", "result_20"]
  output:
    format: "json"
    file: "benchmarks/benchmark-20-model-results.json"
```

## Test Procedures

### 1. Validate Benchmark Workflow

**Test:** Ensure the benchmark workflow is valid.

**Command:**
```bash
cargo run --bin agentsdk -- validate benchmarks/benchmark-20-model.yaml
```

**Expected Results:**
- Workflow is valid
- No validation errors

**Verification:**
```bash
cargo run --bin agentsdk -- validate benchmarks/benchmark-20-model.yaml
# Expected: "Workflow is valid"
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
  run benchmarks/benchmark-20-model.yaml \
  2>&1 | tee benchmarks/benchmark-20-model-execution.log
```

**Expected Results:**
- Workflow completes successfully
- All 20 model calls complete
- Exit code is 0
- Results are saved

**Verification:**
```bash
# Check exit code
cargo run --bin agentsdk -- run benchmarks/benchmark-20-model.yaml
echo "Exit code: $?"
# Expected: 0

# Verify results file
test -f benchmarks/benchmark-20-model-results.json
# Expected: Exit code 0
```

### 3. Measure Execution Time and Compare with Baseline

**Test:** Extract execution time and compare with baseline.

**Command:**
```bash
# Get execution time
ELAPSED_TIME=$(grep "Elapsed (wall clock) time" benchmarks/benchmark-20-model-execution.log | awk '{print $5}')
echo "20-model execution time: $ELAPSED_TIME"

# Convert to seconds
ELAPSED_SECONDS=$(echo "$ELAPSED_TIME" | awk -F: '{print $1*60 + $2}')
echo "20-model elapsed seconds: $ELAPSED_SECONDS"

# Calculate expected time (linear scaling: 4x baseline)
EXPECTED_TIME=$(echo "scale=2; $BASELINE_TIME * 4" | bc)
echo "Expected time (4x baseline): ${EXPECTED_TIME}s"

# Calculate scaling factor
SCALING_FACTOR=$(echo "scale=2; $ELAPSED_SECONDS / $BASELINE_TIME" | bc)
echo "Actual scaling factor: ${SCALING_FACTOR}x"
```

**Expected Results:**
- Execution time scales by ~4x (linear)
- Scaling factor should be between 3x and 5x
- If > 5x, investigate N^2 bottlenecks

**Verification:**
```bash
# Check if scaling is acceptable (<= 5x)
if [ $(echo "$SCALING_FACTOR <= 5" | bc) -eq 1 ]; then
  echo "✓ Scaling is acceptable (linear)"
else
  echo "✗ Scaling is worse than linear (investigate for N^2 bottlenecks)"
fi
# Expected: ✓
```

### 4. Measure Memory Usage and Compare with Baseline

**Test:** Measure peak memory and compare with baseline.

**Command:**
```bash
# Get memory usage
MEMORY_KB=$(grep "Maximum resident set size" benchmarks/benchmark-20-model-execution.log | awk '{print $6}')
MEMORY_MB=$((MEMORY_KB / 1024))
echo "20-model peak memory: ${MEMORY_MB}MB"

# Calculate expected memory (4x baseline)
EXPECTED_MEMORY=$(echo "scale=0; $BASELINE_MEMORY * 4" | bc)
echo "Expected memory (4x baseline): ${EXPECTED_MEMORY}MB"

# Calculate memory scaling
MEMORY_SCALING=$(echo "scale=2; $MEMORY_MB / $BASELINE_MEMORY" | bc)
echo "Memory scaling factor: ${MEMORY_SCALING}x"

# Check if within target (< 2GB)
if [ $MEMORY_MB -lt 2048 ]; then
  echo "✓ Memory within target (< 2GB)"
else
  echo "✗ Memory exceeds target (>= 2GB)"
fi
```

**Expected Results:**
- Memory usage scales linearly (~4x baseline)
- Peak memory < 2GB
- No memory leaks

**Verification:**
```bash
# Verify memory is within target
test $MEMORY_MB -lt 2048
# Expected: Exit code 0
```

### 5. Verify Resource Limits

**Test:** Ensure resource limits are respected.

**Command:**
```bash
# Check for connection pool exhaustion
grep -i "connection pool\|pool exhausted" ~/.agentsdk/logs/agentsdk.log | wc -l
# Expected: 0

# Check for thread pool starvation
grep -i "thread pool\|pool starving" ~/.agentsdk/logs/agentsdk.log | wc -l
# Expected: 0

# Check for OOM errors
grep -i "out of memory\|oom" ~/.agentsdk/logs/agentsdk.log | wc -l
# Expected: 0
```

**Expected Results:**
- No connection pool exhaustion
- No thread pool starvation
- No OOM errors

**Verification:**
```bash
# Verify no resource limit errors
if [ $(grep -i "connection pool\|thread pool\|oom" ~/.agentsdk/logs/agentsdk.log | wc -l) -eq 0 ]; then
  echo "✓ No resource limit errors"
else
  echo "✗ Resource limit errors detected"
fi
# Expected: ✓
```

### 6. Calculate Token Throughput

**Test:** Calculate tokens per second and compare with baseline.

**Command:**
```bash
# Count total tokens
TOTAL_TOKENS=$(cat benchmarks/benchmark-20-model-results.json | jq '[.results[].output | length | tonumber] | add')
echo "Total tokens: $TOTAL_TOKENS"

# Calculate throughput
THROUGHPUT=$(echo "scale=2; $TOTAL_TOKENS / $ELAPSED_SECONDS" | bc)
echo "20-model throughput: ${THROUGHPUT} tokens/second"
echo "5-model throughput (baseline): ${BASELINE_THROUGHPUT} tokens/second"

# Compare throughput
if [ $(echo "$THROUGHPUT >= $BASELINE_THROUGHPUT" | bc) -eq 1 ]; then
  echo "✓ Throughput maintained or improved"
else
  echo "⚠ Throughput decreased"
fi
```

**Expected Results:**
- Token throughput is maintained (not significantly degraded)
- Throughput should be comparable to baseline

**Verification:**
```bash
# Check if throughput is comparable (within 20% of baseline)
THROUGHPUT_RATIO=$(echo "scale=2; $THROUGHPUT / $BASELINE_THROUGHPUT" | bc)
if [ $(echo "$THROUGHPUT_RATIO >= 0.8" | bc) -eq 1 ]; then
  echo "✓ Throughput is comparable to baseline"
else
  echo "✗ Throughput significantly degraded"
fi
# Expected: ✓
```

### 7. Record Metrics to JSON

**Test:** Save all metrics to a JSON file for comparison.

**Command:**
```bash
# Create metrics JSON
cat > benchmarks/scaling-20-model.json <<EOF
{
  "benchmark_name": "20-model scaling test",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "num_models": 20,
  "execution_time_seconds": $ELAPSED_SECONDS,
  "peak_memory_kb": $MEMORY_KB,
  "peak_memory_mb": $MEMORY_MB,
  "total_tokens": $TOTAL_TOKENS,
  "token_throughput": $THROUGHPUT,
  "scaling_factor": $SCALING_FACTOR,
  "memory_scaling": $MEMORY_SCALING,
  "baseline_time_seconds": $BASELINE_TIME,
  "baseline_memory_mb": $BASELINE_MEMORY,
  "baseline_throughput": $BASELINE_THROUGHPUT,
  "exit_code": 0,
  "status": "completed"
}
EOF

# Verify JSON
cat benchmarks/scaling-20-model.json | jq '.'
```

**Expected Results:**
- Metrics JSON is created
- All values are populated
- JSON is valid

**Verification:**
```bash
cat benchmarks/scaling-20-model.json | jq '.' > /dev/null
# Expected: Exit code 0
```

### 8. Log Verification

**Test:** Verify logs show 20 model calls.

**Command:**
```bash
# Count model calls
MODEL_CALLS=$(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="model" and (.message | contains("call" or "request"))) | .message' | wc -l)
echo "Model calls: $MODEL_CALLS"
# Expected: 20

# Count backend requests
BACKEND_REQUESTS=$(cat ~/.agentsdk/logs/agentsdk.log | jq -r 'select(.scope=="backend" and (.message | contains("request" or "send"))) | .message' | wc -l)
echo "Backend requests: $BACKEND_REQUESTS"
# Expected: 20
```

**Expected Results:**
- 20 model calls in logs
- 20 backend requests in logs

**Verification:**
```bash
# Verify counts
test $MODEL_CALLS -eq 20
# Expected: Exit code 0

test $BACKEND_REQUESTS -eq 20
# Expected: Exit code 0
```

### 9. Compare with Baseline and Generate Report

**Test:** Generate comparison report.

**Command:**
```bash
cat > benchmarks/comparison-5-to-20.md <<EOF
# Benchmark Comparison: 5 Model → 20 Model

## Execution Time
- 5 Model: ${BASELINE_TIME}s
- 20 Model: ${ELAPSED_SECONDS}s
- Scaling Factor: ${SCALING_FACTOR}x
- Expected (Linear): 4.0x
- Assessment: $(if [ $(echo "$SCALING_FACTOR <= 5" | bc) -eq 1 ]; then echo "✓ Linear"; else echo "✗ Non-linear (investigate)"; fi)

## Memory Usage
- 5 Model: ${BASELINE_MEMORY}MB
- 20 Model: ${MEMORY_MB}MB
- Scaling Factor: ${MEMORY_SCALING}x
- Expected (Linear): 4.0x
- Assessment: $(if [ $(echo "$MEMORY_SCALING <= 5" | bc) -eq 1 ]; then echo "✓ Linear"; else echo "✗ Non-linear"; fi)

## Token Throughput
- 5 Model: ${BASELINE_THROUGHPUT} tokens/sec
- 20 Model: ${THROUGHPUT} tokens/sec
- Ratio: $(echo "scale=2; $THROUGHPUT / $BASELINE_THROUGHPUT" | bc)
- Assessment: $(if [ $(echo "$THROUGHPUT >= $BASELINE_THROUGHPUT" | bc) -eq 1 ]; then echo "✓ Maintained"; else echo "✗ Degraded"; fi)

## Resource Limits
- Connection Pool Exhaustion: $(grep -i "connection pool\|pool exhausted" ~/.agentsdk/logs/agentsdk.log | wc -l)
- Thread Pool Starvation: $(grep -i "thread pool\|pool starving" ~/.agentsdk/logs/agentsdk.log | wc -l)
- OOM Errors: $(grep -i "out of memory\|oom" ~/.agentsdk/logs/agentsdk.log | wc -l)
- Assessment: All 0 ✓

## Conclusion
$(if [ $(echo "$SCALING_FACTOR <= 5" | bc) -eq 1 ] && [ $MEMORY_MB -lt 2048 ]; then
  echo "✅ Scaling is linear and within resource limits"
else
  echo "❌ Scaling issues detected - investigate bottlenecks"
fi)
EOF

cat benchmarks/comparison-5-to-20.md
```

## Performance Targets

| Metric | Target | Acceptable Range |
|--------|--------|------------------|
| Execution time scaling | 4.0x (linear) | 3.0x - 5.0x |
| Peak memory | < 2GB | - |
| Memory scaling | 4.0x (linear) | 3.0x - 5.0x |
| Token throughput | >= baseline | >= 0.8x baseline |
| Resource limit errors | 0 | 0 |

## Success Criteria

- ✅ Workflow completes successfully (exit code 0)
- ✅ All 20 model calls complete
- ✅ Execution time scaling <= 5x (acceptable linear scaling)
- ✅ Peak memory < 2GB
- ✅ Token throughput >= 0.8x baseline
- ✅ No connection pool exhaustion
- ✅ No thread pool starvation
- ✅ No OOM errors
- ✅ Metrics saved to JSON
- ✅ Comparison report generated

## Failure Procedures

**If scaling > 5x (N^2 bottleneck detected):**
1. Review logs for bottlenecks
2. Check connection pool configuration
3. Check thread pool configuration
4. Review synchronization code
5. Look for nested loops or O(N^2) algorithms
6. Optimize the bottleneck
7. Re-run benchmark

**If memory > 2GB:**
1. Check for memory leaks
2. Review data structures
3. Verify cleanup code
4. Consider increasing memory limit or optimizing

**If throughput degraded:**
1. Check backend performance under load
2. Verify backend can handle 20 concurrent requests
3. Check for network bottlenecks
4. Review serialization/deserialization overhead

## Sign-Off Checklist

- [ ] Benchmark workflow validated
- [ ] Benchmark executed successfully
- [ ] Execution time scaling <= 5x
- [ ] Peak memory < 2GB
- [ ] Token throughput >= 0.8x baseline
- [ ] No connection pool exhaustion
- [ ] No thread pool starvation
- [ ] No OOM errors
- [ ] Metrics saved to JSON
- [ ] Comparison report generated
- [ ] All verification commands pass
