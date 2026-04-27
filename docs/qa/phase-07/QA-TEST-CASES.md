# QA Test Cases — Phase 07: Final Validation

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Test ID Prefix**: P07
**Date**: 2026-04-26

---

## Test Case Summary Table

| Test ID | QA Area | Description | Command | Expected Result |
|----------|----------|-------------|----------|----------------|
| P07-001 | System Log Verification | Structured logging validation | cargo run --bin agentsdk --run examples/workflows/test.yaml 2>&1 \| jq -e 'select(.scope)' | All 9 scopes emit logs |
| P07-002 | System Log Verification | Metrics export populated | curl -s http://localhost:9090/metrics | grep -q 'agentsdk_pipeline' | Metrics include required fields |
| P07-003 | Unit Test Verification | All unit tests pass | cargo test --lib -- --test-threads=1 | All unit tests pass |
| P07-004 | Unit Test Verification | Code coverage > 90% | cargo tarpaulin --out Html --workspace | Coverage >= 90% |
| P07-005 | Unit Test Verification | Proptest passes (1000 iterations) | PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based | All property tests pass |
| P07-006 | Unit Test Verification | No compiler warnings | cargo clippy --all-features -- -D warnings | 0 warnings |
| P07-007 | Integration Test Verification | All integration tests pass | cargo test --test '*' -- --test-threads=1 --nocapture | All integration tests pass |
| P07-008 | CLI Live Testing | All subcommands display help | cargo run --bin agentsdk -- --help | Help without crash |
| P07-009 | CLI Live Testing | Error handling | cargo run --bin agentsdk run nonexistent.yaml | Invalid input error |
| P07-010 | Workflow Execution Tests | All 50 workflows execute | cargo run --bin agentsdk --run workflow.yaml | Workflow executes successfully |
| P07-011 | Workflow Execution Tests | 18 categories verified | grep -q "Workflow completed" output/*.log | All categories covered |
| P07-012 | Benchmark 5 Model | Baseline < 60s, < 500MB | cargo test --bench benchmark_5_model | Baseline targets met |
| P07-013 | Benchmark 20 Model | Linear scaling (4x), < 2GB | cargo test --bench benchmark_20_model | Linear scaling verified |
| P07-014 | Benchmark 50 Model | Stress test, < 5GB | cargo test --bench benchmark_50_model | 50 concurrent handled |
| P07-015 | Benchmark 120 Model | Max stress test, no crash | cargo test --bench benchmark_120_model | Handles 120 models |
| P07-016 | Workflow Generation Test | Generated workflows pass schema | cargo run --bin agentsdk validate generated.yaml | Schema validation passes |
| P07-017 | Workflow Generation Test | Generated workflows execute | cargo run --bin agentsdk run generated.yaml | Generated workflows run |
| P07-018 | Workflow Generation Test | Meaningful results produced | grep -q "not empty" output/* | Results are meaningful |
| P07-019 | Workflow Generation Test | Repeatable generation | cargo run --bin agentsdk run generator.yaml 2>&1 \| grep -q "success" | Generation is repeatable |
| P07-020 | Cross-Phase Regression | All prior phase tests pass | cargo test --test phase_0-6_integration | All prior tests pass |
| P07-021 | Cross-Phase Regression | Upgrade path works | cargo run --bin agentsdk resume old-state.json | State resumes correctly |
| P07-022 | Cross-Phase Regression | No new warnings/errors | grep -q "warning\|error" test output | No regressions |
| P07-023 | Final Signoff | All 12 validation files pass | for file in validation/*.md; do bash -c "./$file"; done | All validations pass |
| P07-024 | Final Signoff | All benchmarks meet targets | grep -q "< 60s" benchmark*.json | All targets met |
| P07-025 | Final Signoff | 50 workflows verified | ls workflows/*.yaml \| wc -l | All 50 workflows execute |
| P07-026 | Final Signoff | Performance report generated | ls -lh benchmarks/*.json | Performance report exists |

---

## Detailed Test Cases

### P07-001: System Log Verification - Structured Logging

**Description**: Verify logging infrastructure produces correct structured output
**QA Area**: Area 1 - System Log Verification
**Priority**: P0
**Test Type**: Unit

**Test Command**:
```bash
cargo run --bin agentsdk --run examples/workflows/test.yaml 2>&1 | jq -e 'select(.scope == "pipeline" or .scope == "step" or .scope == "model")'
```

**Expected Result**:
- All 9 scopes emit logs: pipeline, step, model, tool, backend, ui, system, automation, autonomy
- Log structure includes: timestamp, scope, workflow_id, message
- Metrics export shows: pipeline_duration, step_duration, model_tokens, tool_calls
- No missing required fields in structured logs

**Verification**:
```bash
# Check test output
cargo run --bin agentsdk --run examples/workflows/test.yaml 2>&1 | jq -e 'select(.scope == "pipeline" or .scope == "step" or .scope == "model")' | grep -q "pipeline"
```

---

### P07-003: Unit Test Verification - All Unit Tests Pass

**Description**: Verify all unit tests across all modules pass
**QA Area**: Area 2 - Unit Test Verification
**Priority**: P0
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib -- --test-threads=1
```

**Expected Result**:
- All unit tests pass
- Test result: test result: ok
- No test failures or panics
- All phase-specific unit tests covered

**Verification**:
```bash
# Check test output
cargo test --lib -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P07-005: Unit Test Verification - Code Coverage > 90%

**Description**: Verify code coverage is > 90% using tarpaulin
**QA Area**: Area 2 - Unit Test Verification
**Priority**: P0
**Test Type**: Coverage

**Test Command**:
```bash
cargo tarpaulin --out Html --exclude-files '*/tests/*' --workspace
```

**Expected Result**:
- Code coverage >= 90% for new/modified code
- Coverage report generated as HTML
- Coverage report shows: percentage, uncovered lines, files

**Verification**:
```bash
# Check coverage percentage
cargo tarpaulin --out Html --exclude-files '*/tests/*' --workspace 2>&1 | grep -q "^\s*[0-9]\+\.[0-9]\+\s*%\s*$"
```

---

### P07-010: Workflow Execution Tests - All 50 Workflows Execute

**Description**: Verify all 50 example workflows execute successfully
**QA Area**: Area 5 - Workflow Execution Tests
**Priority**: P0
**Test Type**: E2E

**Test Command**:
```bash
# Execute all 50 workflows
for workflow in docs/reports/requirements/example-workflows/requirements-oriented-auto/*.yaml; do
  cargo run --bin agentsdk --run "$workflow" > /tmp/e2e_output.log
  grep -q "Workflow completed successfully" /tmp/e2e_output.log || exit 1
done
```

**Expected Result**:
- All 50 workflows execute successfully
- Output files exist and are non-empty
- No memory leaks in long-running workflows
- Execution time within 2x expected baseline
- Exit code is 0 for all workflows

**Verification**:
```bash
# Check all workflows executed
if grep -q "Workflow completed successfully" /tmp/e2e_output.log; then
  echo "E2E test PASSED"
else
  echo "E2E test FAILED"
  exit 1
fi

# Check output files exist
ls -lh docs/reports/requirements/example-workflows/requirements-oriented-auto/output/
```

---

### P07-015: Benchmark 120 Model - Max Stress Test

**Description**: Verify system handles 120 concurrent models without crashing (CRITICAL user requirement)
**QA Area**: Area 9 - Benchmark 120 Model
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench benchmark_120_model -- --test-threads=1
```

**Expected Result**:
- System handles 120 models without crashing
- Maximum throughput is measurable (tokens/second, requests/second)
- System limits identified (connection pool size, thread pool size, memory)
- Graceful failure if resources exhausted
- Recovery mechanism works (can resume after failure)
- All metrics logged to benchmarks/max-120-model.json
- Performance report generated comparing 5→20→50→120 scaling

**Verification**:
```bash
# Check max stress test results
cat benchmarks/max-120-model.json

# Verify system didn't crash
grep -q "no_crash.*true" benchmarks/max-120-model.json

# Identify system limits
grep -q "connection_pool_size\|thread_pool_size\|memory_limit" benchmarks/max-120-model.json
```

---

### P07-019: Workflow Generation Test - Generated Workflows Pass Schema

**Description**: Verify generated workflows pass schema validation
**QA Area**: Area 10 - Workflow Generation Test
**Priority**: P0
**Test Type**: E2E

**Test Command**:
```bash
# Validate generated workflows
cargo run --bin agentsdk validate generated-workflow-simple.yaml
cargo run --bin agentsdk validate generated-workflow-sequential.yaml
cargo run --bin agentsdk validate generated-workflow-parallel.yaml
cargo run --bin agentsdk validate generated-workflow-conditional.yaml
cargo run --bin agentsdk validate generated-workflow-error-handling.yaml
```

**Expected Result**:
- All generated workflows pass schema validation
- Schema version 2.0.0 validated
- Invalid schemas rejected with clear error
- No syntax errors in generated workflows

**Verification**:
```bash
# Check all validations pass
if cargo run --bin agentsdk validate generated-workflow-simple.yaml; then
  echo "Schema validation PASSED"
else
  echo "Schema validation FAILED"
  exit 1
fi
```

---

### P07-020: Cross-Phase Regression - All Prior Phase Tests Pass

**Description**: Re-run all tests from phases 0-6 to verify no regressions
**QA Area**: Area 11 - Cross-Phase Regression
**Priority**: P0
**Test Type**: Integration, Regression

**Test Command**:
```bash
# Run all prior phase tests
for phase in $(seq 0 6); do
  cargo test --test "phase_${phase}_integration" -- --test-threads=1
done
```

**Expected Result**:
- All tests from phases 0-6 pass
- No new warnings or errors
- Upgrade path works (state can be resumed from Phase 6)
- Backward compatibility maintained (old workflow formats still work)
- No performance regression in existing features

**Verification**:
```bash
# Check all prior phase tests pass
if cargo test --test phase_0-6_integration -- --test-threads=1; then
  echo "Cross-phase regression test PASSED"
else
  echo "Cross-phase regression test FAILED"
  exit 1
fi

# Verify upgrade path
cargo run --bin agentsdk resume --from ./workspace/state/phase-06-state.json
```

---

## Performance Test Cases

### P07-PERF-001: Benchmark 5 Model - Baseline

**Description**: Verify baseline performance with 5 concurrent LLM models
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench benchmark_5_model -- --test-threads=1
```

**Expected Result**:
- Execution time: target < 60 seconds for simple queries
- Memory usage: target < 500MB peak
- Token throughput: target > 100 tokens/second
- CPU utilization: target < 80%
- No significant regression from baseline
- Metrics logged to benchmarks/baseline-5-model.json

**Verification**:
```bash
# Parse benchmark results
cat benchmarks/baseline-5-model.json
```

---

### P07-PERF-003: Benchmark 20 Model - Linear Scaling

**Description**: Verify linear scaling from 5→20 models
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench benchmark_20_model -- --test-threads=1
```

**Expected Result**:
- Execution time scales approximately linearly (4x 5-model time)
- Detect any N^2 bottlenecks (if time > 5x baseline, investigate)
- Memory usage scales linearly (should not exceed 2GB)
- Resource limits are respected (no OOM, no connection pool exhaustion)
- Compare with baseline from 5-model benchmark
- Metrics logged to benchmarks/scaling-20-model.json

**Verification**:
```bash
# Check scaling metrics
cat benchmarks/scaling-20-model.json
```

---

### P07-PERF-005: Benchmark 50 Model - Stress Test

**Description**: Stress test with 50 concurrent models
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench benchmark_50_model -- --test-threads=1
```

**Expected Result**:
- Parallel execution handles 50 concurrent models without deadlocks
- Memory pressure is manageable (target: < 5GB peak)
- Graceful degradation occurs if resources are constrained
- No model requests are dropped or lost
- System remains responsive (CLI still responds)
- Metrics logged to benchmarks/stress-50-model.json
- Monitor for connection pool exhaustion, thread pool starvation, OOM

**Verification**:
```bash
# Check stress test results
cat benchmarks/stress-50-model.json
```

---

### P07-PERF-006: Benchmark 120 Model - Max Stress Test (CRITICAL User Requirement)

**Description**: Maximum stress test with 120 concurrent models
**Priority**: P0
**Test Type**: Benchmark (CRITICAL)

**Test Command**:
```bash
cargo test --bench benchmark_120_model -- --test-threads=1
```

**Expected Result**:
- System handles this load without crashing
- Maximum throughput is measurable (tokens/second, requests/second)
- Identify any system limits (connection pool size, thread pool size, memory)
- Verify graceful failure if resources are exhausted
- Recovery mechanism works (can resume after failure)
- All metrics logged to benchmarks/max-120-model.json
- This is CRITICAL test from user's explicit requirement
- Generate performance report comparing 5→20→50→120 scaling

**Verification**:
```bash
# Check max stress test results
cat benchmarks/max-120-model.json

# Verify system didn't crash
grep -q "no_crash.*true" benchmarks/max-120-model.json
```

---

## Mock Strategy Notes

### System Logs
- Validate log structure across all 9 scopes
- Use jq or similar tool to verify JSON structure
- Mock metrics export endpoint for testing

### Unit Tests
- Use cargo test for running all unit tests
- Use cargo tarpaulin for code coverage
- Use cargo clippy for linting

### Integration Tests
- Use cargo test --test for integration tests
- Enable verbose logging with RUST_LOG=debug for debugging

### CLI Testing
- Test all CLI commands with help flag
- Test error handling with invalid inputs
- Test with real LLM backends (LM Studio, Ollama, llama.cpp)

### Workflow Execution Tests
- Use all 50 example workflows from requirements-oriented-auto
- Test each workflow category (18 categories)
- Verify output files exist and are non-empty
- Monitor for memory leaks in long-running workflows

### Benchmark Tests
- Use Criterion or custom benchmarking harness
- Test with increasing concurrency (5, 20, 50, 120)
- Measure execution time, memory usage, CPU utilization, token throughput
- Identify system limits and bottlenecks

### Workflow Generation Test
- Create workflow generator that generates new workflows
- Use LLM to generate YAML workflow definitions
- Validate generated workflows against schema
- Execute generated workflows and verify they work

---

**End of QA Test Cases for Phase 07**
