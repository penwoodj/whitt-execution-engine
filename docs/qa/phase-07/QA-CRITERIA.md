# QA Criteria — Phase 07: Final Validation

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Phase Plan**: `docs/plans/07-final-validation/plan.md` (662 lines, 12 validation tasks)
**Date**: 2026-04-26
**Status**: 🟡 IN PROGRESS — Phase 3 hooks complete, UF05/16/18 implemented, YAML generator audit done

---

## Summary Table

| # | QA Area | Schema Ref | Status | Priority | Test Type |
|---|---------|------------|------------|
| 1 | System Log Verification (7-layer verification) | Lines 14-805 (full schema) | P0 | Unit, Integration |
| 2 | Unit Test Verification (90%+ coverage, 0 warnings) | N/A (code quality) | P0 | Unit, Coverage |
| 3 | Integration Test Verification (cross-module data flows) | N/A (cross-module) | P0 | Integration |
| 4 | CLI Live Testing (all CLI commands vs real backends) | N/A (CLI integration) | P0 | E2E |
| 5 | Workflow Execution Tests (50 example workflows) | Lines 196-497 (agentic_workflow) | P0 | E2E |
| 6 | Benchmark 5 Model (baseline < 60s, < 500MB) | N/A (performance) | P0 | Benchmark |
| 7 | Benchmark 20 Model (linear scaling, < 2GB) | N/A (performance) | P0 | Benchmark |
| 8 | Benchmark 50 Model (stress test, < 5GB) | N/A (performance) | P0 | Benchmark |
| 9 | Benchmark 120 Model (max stress test, no crash) | N/A (performance) | P0 | Benchmark |
| 10 | Workflow Generation Test (workflow makes workflows) | N/A (meta-validation) | P0 | E2E |
| 11 | Cross-Phase Regression (phases 0-6 regression tests) | N/A (regression) | P0 | Integration |
| 12 | Final Signoff (acceptance criteria) | Lines 14-805 (full schema) | P0 | Signoff |

---

## Area Details

### Area 1: System Log Verification

**Schema Ref**: Lines 14-805 (full unified workflow schema - logging infrastructure)
**Plan Ref**: Task 00 (`docs/plans/07-final-validation/00-system-log-verification.md`)
**Files**: N/A (validation infrastructure, existing logging system)

**Criteria**:
- System logs produce correct structured output across all 9 scopes
- Scopes: pipeline, step, model, tool, backend, ui, system, automation, autonomy
- Log structure includes: timestamp, scope, workflow_id, message
- Metrics export shows all required fields populated
- No log level violations (debug logs in release builds)
- Log samples from all scopes validated
- Telemetry completeness verified

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Validate log structure
cargo run --bin agentsdk --run examples/workflows/test.yaml 2>&1 | \
  jq -e 'select(.scope == "pipeline" or .scope == "step" or .scope == "model")'

# Verify metrics export
curl -s http://localhost:9090/metrics | grep -q 'agentsdk_pipeline_duration_seconds'
```

---

### Area 2: Unit Test Verification

**Schema Ref**: N/A (code quality - not schema-specific)
**Plan Ref**: Task 01 (`docs/plans/07-final-validation/01-unit-test-verification.md`)
**Files**: All phase 00-06 modules with tests

**Criteria**:
- All unit tests across all modules pass (cargo test --lib)
- 90%+ code coverage using `tarpaulin` or `cargo-llvm-cov`
- No panics in unit test output
- Verify no compiler warnings (cargo clippy -- -D warnings)
- Proptest passes with 1000 iterations
- Test coverage of edge cases beyond example inputs
- Coverage >= 90% for new/modified code
- All modules from phases 0-6 have passing unit tests

**Test Type**: Unit, Coverage
**Priority**: P0

**Commands**:
```bash
# Run all unit tests
cargo test --lib -- --test-threads=1

# Code coverage report
cargo tarpaulin --out Html --exclude-files '*/tests/*' --workspace

# Proptest
PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based

# Clippy check
cargo clippy --all-features -- -D warnings
```

---

### Area 3: Integration Test Verification

**Schema Ref**: N/A (cross-module data flows - not schema-specific)
**Plan Ref**: Task 02 (`docs/plans/07-final-validation/02-integration-test-verification.md`)
**Files**: All phase 00-06 integration tests

**Criteria**:
- All integration tests across all phases pass
- Cross-module data flows verified
- API contracts validated (request/response formats match schema)
- State machine transitions verified (initialized → running → paused → completed)
- No race conditions or deadlocks detected
- Communication patterns match ADR specifications
- All phases 0-6 integration tests pass

**Test Type**: Integration
**Priority**: P0

**Commands**:
```bash
# Run all integration tests
cargo test --test '*' -- --test-threads=1 --nocapture

# Verbose logging for debugging
RUST_LOG=debug cargo test --test 'integration_*' -- --test-threads=1 --nocapture
```

---

### Area 4: CLI Live Testing

**Schema Ref**: N/A (CLI integration - existing CLI commands)
**Plan Ref**: Task 03 (`docs/plans/07-final-validation/03-cli-live-testing.md`)
**Files**: CLI implementation across all phases

**Criteria**:
- Test ALL CLI commands against real backends:
  - `run` - Execute workflows
  - `generate` - Generate workflows from templates
  - `queue list/status/pause/resume/cancel` - Queue management
  - `config` - Configuration management
  - `schedule` - Scheduled workflow execution
  - `experiment` - Experimental features
  - `merge` - Merge workflow results
  - `rollback` - Rollback to previous state
  - `metrics` - View system metrics
  - `memory search` - Search memory store
  - `autonomy` - Autonomy control
- Test procedures, expected results, and verification commands
- Error messages displayed for invalid inputs
- Help text is current and accurate
- Progress indicators work for long-running operations

**Test Type**: E2E
**Priority**: P0

**Commands**:
```bash
# Test all subcommands
cargo run --bin agentsdk -- --help
cargo run --bin agentsdk run --help
cargo run --bin agentsdk queue --help
cargo run --bin agentsdk config --help

# Test error handling
cargo run --bin agentsdk run nonexistent.yaml
! cargo run --bin agentsdk run invalid.yaml

# Test all CLI commands with real backend
cargo run --bin agentsdk run examples/workflows/simple-pipeline.yaml
cargo run --bin agentsdk generate examples/templates/simple-template.yml
cargo run --bin agentsdk queue list
cargo run --bin agentsdk config get
cargo run --bin agentsdk schedule list
cargo run --bin agentsdk experiment list
cargo run --bin agentsdk merge list
cargo run --bin agentsdk rollback list
cargo run --bin agentsdk metrics view
cargo run --bin agentsdk memory search "test query"
cargo run --bin agentsdk autonomy start examples/autonomy-workflow.yaml
```

---

### Area 5: Workflow Execution Tests

**Schema Ref**: Lines 196-497 (agentic_workflow) + Lines 14-805 (full schema)
**Plan Ref**: Task 04 (`docs/plans/07-final-validation/04-workflow-execution-tests.md`)
**Files**: 50 example workflows from `docs/reports/requirements/example-workflows/requirements-oriented-auto/`

**Criteria**:
- Execute ALL 50 example workflows
- Verify correct output for each of 18 categories:
  1. Simple execution
  2. Parallel models
  3. Sequential steps
  4. Conditional logic
  5. Error handling
  6. Retry logic
  7. Timeout handling
  8. Memory operations
  9. State persistence
  10. Model selection
  11. Parameter passing
  12. Output formatting
  13. Logging configuration
  14. Queue operations
  15. Scheduler integration
  16. Backend switching
  17. Custom prompts
  18. Temperature control
  19. Max tokens
- Output files exist and are non-empty
- No memory leaks in long-running workflows
- Execution time within 2x expected baseline

**Test Type**: E2E
**Priority**: P0

**Commands**:
```bash
# Execute all 50 example workflows
for workflow in docs/reports/requirements/example-workflows/requirements-oriented-auto/*.yaml; do
  cargo run --bin agentsdk --run "$workflow" > /tmp/e2e_output.log
  grep -q "Workflow completed successfully" /tmp/e2e_output.log || exit 1
done

# Verify output files exist
ls -lh docs/reports/requirements/example-workflows/requirements-oriented-auto/output/
```

---

### Area 6: Benchmark 5 Model

**Schema Ref**: N/A (performance benchmarking - not schema-specific)
**Plan Ref**: Task 05 (`docs/plans/07-final-validation/05-benchmark-5-model.md`)
**Files**: N/A (benchmarking infrastructure)

**Criteria**:
- Baseline performance with 5 concurrent LLM models
- Target execution time < 60 seconds for simple queries
- Target memory usage < 500MB peak
- Target token throughput > 100 tokens/second
- Target CPU utilization < 80%
- Log all metrics to `benchmarks/baseline-5-model.json`
- Test workflow definition with 5 different model references
- Verify scaling from baseline

**Test Type**: Benchmark
**Priority**: P0

**Commands**:
```bash
# Run benchmark with 5 models
cargo test --bench benchmark_5_model -- --test-threads=1

# Check benchmark results
cat benchmarks/baseline-5-model.json
```

---

### Area 7: Benchmark 20 Model

**Schema Ref**: N/A (performance scaling verification)
**Plan Ref**: Task 06 (`docs/plans/07-final-validation/06-benchmark-20-model.md`)
**Files**: N/A (benchmarking infrastructure)

**Criteria**:
- Linear scaling verification from 5→20 models
- Test workflow with 20 concurrent model references
- Verify execution time scales approximately linearly (4x 5-model time)
- Detect any N^2 bottlenecks (if time > 5x baseline, investigate)
- Verify memory usage scales linearly (should not exceed 2GB)
- Verify resource limits respected (no OOM, no connection pool exhaustion)
- Compare with baseline from 5-model benchmark
- Log all metrics to `benchmarks/scaling-20-model.json`

**Test Type**: Benchmark
**Priority**: P0

**Commands**:
```bash
# Run benchmark with 20 models
cargo test --bench benchmark_20_model -- --test-threads=1

# Check scaling metrics
cat benchmarks/scaling-20-model.json
```

---

### Area 8: Benchmark 50 Model

**Schema Ref**: N/A (stress testing with high concurrency)
**Plan Ref**: Task 07 (`docs/plans/07-final-validation/07-benchmark-50-model.md`)
**Files**: N/A (benchmarking infrastructure)

**Criteria**:
- Stress test with 50 concurrent LLM models
- Test workflow with 50 concurrent model references
- Verify parallel execution handles 50 concurrent models without deadlocks
- Verify memory pressure is manageable (target: < 5GB peak)
- Verify graceful degradation occurs if resources constrained
- Verify no model requests are dropped or lost
- Verify system remains responsive (CLI still responds)
- Monitor for connection pool exhaustion, thread pool starvation, OOM
- Log all metrics to `benchmarks/stress-50-model.json`

**Test Type**: Benchmark
**Priority**: P0

**Commands**:
```bash
# Run benchmark with 50 models
cargo test --bench benchmark_50_model -- --test-threads=1

# Check stress test results
cat benchmarks/stress-50-model.json
```

---

### Area 9: Benchmark 120 Model

**Schema Ref**: N/A (maximum stress test - CRITICAL from user requirement)
**Plan Ref**: Task 08 (`docs/plans/07-final-validation/08-benchmark-120-model.md`)
**Files**: N/A (benchmarking infrastructure)

**Criteria**:
- Maximum stress test with 120 concurrent models (user's explicit requirement)
- Test workflow with 120 model references in a single workflow
- Verify system handles this load without crashing
- Verify maximum throughput is measurable (tokens/second, requests/second)
- Identify any system limits (connection pool size, thread pool size, memory)
- Verify graceful failure if resources exhausted
- Verify recovery mechanism works (can resume after failure)
- This is CRITICAL test from user's explicit requirement
- Log all metrics to `benchmarks/max-120-model.json`
- Generate performance report comparing 5→20→50→120 scaling

**Test Type**: Benchmark
**Priority**: P0

**Commands**:
```bash
# Run benchmark with 120 models (CRITICAL user requirement)
cargo test --bench benchmark_120_model -- --test-threads=1

# Check max stress test results
cat benchmarks/max-120-model.json
```

---

### Area 10: Workflow Generation Test

**Schema Ref**: Lines 196-497 (agentic_workflow) + Lines 14-805 (full schema)
**Plan Ref**: Task 09 (`docs/plans/07-final-validation/09-workflow-generation-test.md`)
**Files**: N/A (meta-validation infrastructure)

**Criteria**:
- THE KEY TEST - Create a workflow that generates new workflows
- User's explicit requirement: "attempting other example workflows mainly focusing on workflow that makes other workflows that fit the schema and will produce results that will actually run and work in the engine"
- Test Steps:
  1. Create a "workflow generator" workflow that:
     - Takes a template description as input
     - Uses an LLM to generate a YAML workflow definition
     - Validates the generated workflow against schema
     - Saves the generated workflow to a file
  2. Execute workflow generator to create multiple test workflows:
     - Simple 1-step workflow
     - 3-step sequential workflow
     - Parallel 2-model workflow
     - Conditional workflow
     - Error-handling workflow
  3. For each generated workflow:
     - (a) Verify it passes schema validation: `cargo run --bin agentsdk validate <generated-workflow.yaml>`
     - (b) Execute it against a real backend: `cargo run --bin agentsdk run <generated-workflow.yaml>`
     - (c) Verify it produces meaningful results (not empty, not error)
- Success Criteria:
  - All generated workflows pass schema validation
  - All generated workflows execute successfully
  - All generated workflows produce meaningful results
  - Generation process is repeatable

**Test Type**: E2E
**Priority**: P0

**Commands**:
```bash
# Create workflow generator workflow
cat > examples/workflow-generator.yaml << 'EOF'
workflow_id: workflow_generator
name: "Workflow Generator"
agentic_workflow:
  - step: generate_workflow
      generative_entity: "${models.primary-analyzer}"
      prompt: "Generate a workflow from template: {{inputs.template_description}}"
      output:
        save_to: generated_workflow
EOF

# Execute workflow generator
cargo run --bin agentsdk --run examples/workflow-generator.yaml

# Validate and execute generated workflows
cargo run --bin agentsdk validate generated-workflow-simple.yaml
cargo run --bin agentsdk run generated-workflow-simple.yaml
cargo run --bin agentsdk validate generated-workflow-sequential.yaml
cargo run --bin agentsdk run generated-workflow-sequential.yaml
cargo run --bin agentsdk validate generated-workflow-parallel.yaml
cargo run --bin agentsdk run generated-workflow-parallel.yaml
cargo run --bin agentsdk validate generated-workflow-conditional.yaml
cargo run --bin agentsdk run generated-workflow-conditional.yaml
cargo run --bin agentsdk validate generated-workflow-error-handling.yaml
cargo run --bin agentsdk run generated-workflow-error-handling.yaml
```

---

### Area 11: Cross-Phase Regression

**Schema Ref**: Lines 14-805 (full unified workflow schema)
**Plan Ref**: Task 10 (`docs/plans/07-final-validation/10-cross-phase-regression.md`)
**Files**: All phase 00-06 test suites

**Criteria**:
- Re-run ALL tests from phases 0-6 to verify no regressions
- Test Steps:
  1. Phase 0 Tests: Verify project initialization, configuration
  2. Phase 1 Tests: Verify schema validation, YAML parsing
  3. Phase 2 Tests: Verify workflow execution engine
  4. Phase 3 Tests: Verify LLM backend integration
  5. Phase 4 Tests: Verify CLI implementation
  6. Phase 5 Tests: Verify queue management
  7. Phase 6 Tests: Verify scheduler implementation
- Upgrade Path Testing:
  - Load old state files from Phase 6
  - Verify state can be resumed
  - Verify backward compatibility with previous workflow formats
- Success Criteria:
  - All tests from phases 0-6 pass
  - No new warnings or errors
  - Upgrade path works correctly
  - Backward compatibility maintained

**Test Type**: Integration, Regression
**Priority**: P0

**Commands**:
```bash
# Run all prior phase tests
for phase in $(seq 0 6); do
  cargo test --test "phase_${phase}_integration" -- --test-threads=1
done

# Verify backward compatibility
# Load and resume from old state files
cargo run --bin agentsdk resume --from ./workspace/state/phase-06-state.json
```

---

### Area 12: Final Signoff

**Schema Ref**: Lines 14-805 (full unified workflow schema)
**Plan Ref**: Task 11 (`docs/plans/07-final-validation/11-final-signoff.md`)
**Files**: All previous validation files and test results

**Criteria**:
- Final acceptance criteria and signoff:
  1. ✅ All 12 validation files (00-11) pass completely
  2. ✅ All 50 example workflows execute successfully
  3. ✅ All benchmarks meet targets:
     - 5-model: < 60 seconds, < 500MB memory
     - 20-model: Linear scaling, < 2GB memory
     - 50-model: Handles 50 concurrent, < 5GB memory
     - 120-model: Handles 120 without crash, identifies limits
  4. ✅ No critical bugs or unhandled errors
  5. ✅ Documentation complete and accurate
  6. ✅ ADR compliance verified (all architectural decisions documented)
  7. ✅ Code coverage > 90%
  8. ✅ All unit tests pass
  9. ✅ All integration tests pass
  10. ✅ All CLI commands work correctly
- Performance Report:
  - Generate comprehensive performance report comparing all benchmarks
  - Document system limits discovered
  - Document bottlenecks and optimization opportunities
- Final Checklist completed

**Test Type**: Signoff
**Priority**: P0

**Commands**:
```bash
# Verify all validation files pass
for file in docs/plans/07-final-validation/00-*.md docs/plans/07-final-validation/01-*.md docs/plans/07-final-validation/02-*.md docs/plans/07-final-validation/03-*.md docs/plans/07-final-validation/04-*.md docs/plans/07-final-validation/05-*.md docs/plans/07-final-validation/06-*.md docs/plans/07-final-validation/07-*.md docs/plans/07-final-validation/08-*.md docs/plans/07-final-validation/09-*.md docs/plans/07-final-validation/10-*.md docs/plans/07-final-validation/11-*.md; do
  echo "Executing validation file: $file"
  # Execute validation file
done

# Verify all 50 workflows execute
for workflow in docs/reports/requirements/example-workflows/requirements-oriented-auto/*.yaml; do
  cargo run --bin agentsdk --run "$workflow" || exit 1
done

# Verify benchmarks meet targets
cat benchmarks/baseline-5-model.json | grep -q "execution_time.*< 60"
cat benchmarks/scaling-20-model.json | grep -q "memory.*< 2GB"
cat benchmarks/stress-50-model.json | grep -q "concurrent.*= 50"
cat benchmarks/max-120-model.json | grep -q "model_count.*= 120"
```

---

## Performance Targets

| Metric | Target | Validation Method |
|--------|--------|------------------|
| 5-model execution time | < 60 seconds | Benchmark test |
| 5-model memory usage | < 500MB peak | Benchmark test |
| 5-model token throughput | > 100 tokens/second | Benchmark test |
| 20-model scaling | Linear (4x baseline), < 2GB memory | Benchmark test |
| 50-model concurrency | Handles 50 concurrent, < 5GB memory | Benchmark test |
| 120-model stress | Handles 120 without crash, identifies limits | Benchmark test |
| Unit test coverage | > 90% | Coverage report |
| Integration test pass rate | 100% | Test execution |

---

## Phase Exit Criteria

Phase 07 is complete when:

1. **All 12 validation files** are created and documented
2. **All 12 validation files** execute without errors
3. **All 50 example workflows** execute successfully
4. **All benchmarks** meet their targets
5. **Performance report** is generated
6. **Final signoff checklist** is complete
7. **No critical bugs** remain
8. **ADR compliance** is verified
9. **Code coverage > 90%** is verified
10. **All unit tests pass**
11. **All integration tests pass**
12. **All CLI commands work correctly**

---

## Related Plan Tasks

| QA Area | Plan Task | Plan File |
|----------|------------|------------|
| Area 1: System Log Verification | Task 00 | [00-system-log-verification.md](../plans/07-final-validation/00-system-log-verification.md) |
| Area 2: Unit Test Verification | Task 01 | [01-unit-test-verification.md](../plans/07-final-validation/01-unit-test-verification.md) |
| Area 3: Integration Test Verification | Task 02 | [02-integration-test-verification.md](../plans/07-final-validation/02-integration-test-verification.md) |
| Area 4: CLI Live Testing | Task 03 | [03-cli-live-testing.md](../plans/07-final-validation/03-cli-live-testing.md) |
| Area 5: Workflow Execution Tests | Task 04 | [04-workflow-execution-tests.md](../plans/07-final-validation/04-workflow-execution-tests.md) |
| Area 6: Benchmark 5 Model | Task 05 | [05-benchmark-5-model.md](../plans/07-final-validation/05-benchmark-5-model.md) |
| Area 7: Benchmark 20 Model | Task 06 | [06-benchmark-20-model.md](../plans/07-final-validation/06-benchmark-20-model.md) |
| Area 8: Benchmark 50 Model | Task 07 | [07-benchmark-50-model.md](../plans/07-final-validation/07-benchmark-50-model.md) |
| Area 9: Benchmark 120 Model | Task 08 | [08-benchmark-120-model.md](../plans/07-final-validation/08-benchmark-120-model.md) |
| Area 10: Workflow Generation Test | Task 09 | [09-workflow-generation-test.md](../plans/07-final-validation/09-workflow-generation-test.md) |
| Area 11: Cross-Phase Regression | Task 10 | [10-cross-phase-regression.md](../plans/07-final-validation/10-cross-phase-regression.md) |
| Area 12: Final Signoff | Task 11 | [11-final-signoff.md](../plans/07-final-validation/11-final-signoff.md) |

---

**End of QA Criteria for Phase 07**
