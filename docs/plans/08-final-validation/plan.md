# Phase 8: Final Validation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Comprehensive validation of the complete AgentSDK Execution Engine through systematic testing, benchmarking, and final signoff.

**Architecture:** 7-layer verification framework (Logs → Unit → Integration → CLI → Workflows → Benchmarks → Meta-Validation) with progressive complexity scaling (5→20→50→120 models).

**Tech Stack:** Rust (cargo test, cargo bench), Python (pytest), Shell (bash), LLM Backends (LM Studio/Ollama/llama.cpp), Custom benchmarking harness.

---

## Dependencies

**PREREQUISITES:**
- Phase 0-7 complete with all tests passing
- All 53 example workflows in `opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/`
- At least one LLM backend running (LM Studio, Ollama, or llama.cpp)
- Configuration file properly set with backend credentials
- System log directories created and writable
- Database/MQTT initialized (if applicable)

**Verification of Dependencies:**
```bash
# Check all phases are complete
ls opencode/docs/plans/
git status  # Clean working tree

# Verify workflows exist
find opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/ -name "*.yaml" | wc -l  # Should be 53

# Verify backend is accessible
cargo run --bin agentsdk config get  # Should show backend config
curl -s http://localhost:11434/api/tags  # Ollama health check OR
curl -s http://localhost:1234/v1/models  # LM Studio health check

# Verify build
cargo build --release 2>&1 | grep -i "error"  # Should be empty
```

---

## 7-Layer Verification Framework

### Layer 1: System Log Verification
- **What:** Validate logging infrastructure produces correct structured output
- **Why:** Logs are the primary observability mechanism
- **How:** Execute `00-system-log-verification.md`

### Layer 2: Unit Test Verification
- **What:** Run all unit tests, verify coverage, verify no warnings
- **Why:** Ensures individual components work correctly
- **How:** Execute `01-unit-test-verification.md`

### Layer 3: Integration Test Verification
- **What:** Run all integration tests, verify cross-module data flows
- **Why:** Ensures modules work together correctly
- **How:** Execute `02-integration-test-verification.md`

### Layer 4: CLI Live Testing
- **What:** Test ALL CLI commands against real backends
- **Why:** Validates user-facing interface works end-to-end
- **How:** Execute `03-cli-live-testing.md`

### Layer 5: Workflow Execution Tests
- **What:** Execute ALL 53 example workflows, verify correct output
- **Why:** Validates the engine handles real-world workflows
- **How:** Execute `04-workflow-execution-tests.md`

### Layer 6: Benchmark Scaling Tests
- **What:** 5→20→50→120 model progressive scaling benchmarks
- **Why:** Validates system handles increasing load
- **How:** Execute `05-benchmark-5-model.md`, `06-benchmark-20-model.md`, `07-benchmark-50-model.md`, `08-benchmark-120-model.md`

### Layer 7: Meta-Validation
- **What:** Workflow-generating-workflow test, cross-phase regression, final signoff
- **Why:** Validates system can generate and run its own workflows
- **How:** Execute `09-workflow-generation-test.md`, `10-cross-phase-regression.md`, `11-final-signoff.md`

---

## Task Structure

### Task 1: Create Phase 8 Directory Structure

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/plan.md`

- [ ] **Step 1: Create the Phase 8 directory**

```bash
mkdir -p /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation
```

- [ ] **Step 2: Write master plan.md**

```bash
# (This is the current file being written)
```

- [ ] **Step 3: Verify directory structure**

```bash
ls -la /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/
```

- [ ] **Step 4: Commit Phase 8 initialization**

```bash
git add opencode/docs/plans/08-final-validation/
git commit -m "feat: initialize Phase 8 Final Validation plan directory"
```

---

### Task 2: Write Validation File 00 - System Log Verification

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/00-system-log-verification.md`

- [ ] **Step 1: Create validation file 00**

Write the complete validation file with all test procedures, expected results, and verification commands for system log verification.

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/00-system-log-verification.md
```

- [ ] **Step 3: Commit validation file 00**

```bash
git add opencode/docs/plans/08-final-validation/00-system-log-verification.md
git commit -m "feat: add validation file 00 - system log verification"
```

---

### Task 3: Write Validation File 01 - Unit Test Verification

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/01-unit-test-verification.md`

- [ ] **Step 1: Create validation file 01**

Write the complete validation file with:
- Run all unit tests across all modules
- Verify 90%+ code coverage using `tarpaulin` or `cargo-llvm-cov`
- Verify no compiler warnings
- Verify proptest passes with 1000 iterations
- Test procedures and expected results

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/01-unit-test-verification.md
```

- [ ] **Step 3: Commit validation file 01**

```bash
git add opencode/docs/plans/08-final-validation/01-unit-test-verification.md
git commit -m "feat: add validation file 01 - unit test verification"
```

---

### Task 4: Write Validation File 02 - Integration Test Verification

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/02-integration-test-verification.md`

- [ ] **Step 1: Create validation file 02**

Write the complete validation file with:
- Run all integration tests
- Verify cross-module data flows (workflow → execution → model → backend)
- Verify API contracts (request/response formats match schema)
- Verify state machine transitions (initialized → running → paused → completed)
- Test procedures and expected results

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/02-integration-test-verification.md
```

- [ ] **Step 3: Commit validation file 02**

```bash
git add opencode/docs/plans/08-final-validation/02-integration-test-verification.md
git commit -m "feat: add validation file 02 - integration test verification"
```

---

### Task 5: Write Validation File 03 - CLI Live Testing

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/03-cli-live-testing.md`

- [ ] **Step 1: Create validation file 03**

Write the complete validation file with:
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
- Test procedures, expected results, and verification commands

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/03-cli-live-testing.md
```

- [ ] **Step 3: Commit validation file 03**

```bash
git add opencode/docs/plans/08-final-validation/03-cli-live-testing.md
git commit -m "feat: add validation file 03 - CLI live testing"
```

---

### Task 6: Write Validation File 04 - Workflow Execution Tests

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/04-workflow-execution-tests.md`

- [ ] **Step 1: Create validation file 04**

Write the complete validation file with:
- Execute ALL 53 example workflows from `opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/`
- Verify correct output for each of 19 categories:
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
- Test procedures, expected results, and verification commands

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/04-workflow-execution-tests.md
```

- [ ] **Step 3: Commit validation file 04**

```bash
git add opencode/docs/plans/08-final-validation/04-workflow-execution-tests.md
git commit -m "feat: add validation file 04 - workflow execution tests"
```

---

### Task 7: Write Validation File 05 - Benchmark 5 Model

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/05-benchmark-5-model.md`

- [ ] **Step 1: Create validation file 05**

Write the complete validation file with:
- **Objective:** Establish baseline performance with 5 concurrent LLM models
- **Test workflow definition with 5 different model references**
- **Measure:**
  - Execution time (target: < 60 seconds for simple queries)
  - Memory usage (target: < 500MB peak)
  - Token throughput (target: > 100 tokens/second)
  - CPU utilization (target: < 80%)
- **Log all metrics to `benchmarks/baseline-5-model.json`**
- **Test procedures and expected results**

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/05-benchmark-5-model.md
```

- [ ] **Step 3: Commit validation file 05**

```bash
git add opencode/docs/plans/08-final-validation/05-benchmark-5-model.md
git commit -m "feat: add validation file 05 - benchmark 5 model"
```

---

### Task 8: Write Validation File 06 - Benchmark 20 Model

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/06-benchmark-20-model.md`

- [ ] **Step 1: Create validation file 06**

Write the complete validation file with:
- **Objective:** Verify linear scaling from 5→20 models
- **Test workflow with 20 concurrent model references**
- **Verify scaling:**
  - Execution time scales approximately linearly (4x the 5-model time)
  - Detect any N^2 bottlenecks (if time > 5x baseline, investigate)
  - Memory usage scales linearly (should not exceed 2GB)
  - Resource limits are respected (no OOM, no connection pool exhaustion)
- **Compare with baseline from 05-benchmark-5-model**
- **Log all metrics to `benchmarks/scaling-20-model.json`**
- **Test procedures and expected results**

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/06-benchmark-20-model.md
```

- [ ] **Step 3: Commit validation file 06**

```bash
git add opencode/docs/plans/08-final-validation/06-benchmark-20-model.md
git commit -m "feat: add validation file 06 - benchmark 20 model"
```

---

### Task 9: Write Validation File 07 - Benchmark 50 Model

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/07-benchmark-50-model.md`

- [ ] **Step 1: Create validation file 07**

Write the complete validation file with:
- **Objective:** Stress test with 50 concurrent models
- **Test workflow with 50 concurrent model references**
- **Verify:**
  - Parallel execution handles 50 concurrent models without deadlocks
  - Memory pressure is manageable (target: < 5GB peak)
  - Graceful degradation occurs if resources are constrained
  - No model requests are dropped or lost
  - System remains responsive (CLI still responds)
- **Monitor for connection pool exhaustion, thread pool starvation, OOM**
- **Log all metrics to `benchmarks/stress-50-model.json`**
- **Test procedures and expected results**

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/07-benchmark-50-model.md
```

- [ ] **Step 3: Commit validation file 07**

```bash
git add opencode/docs/plans/08-final-validation/07-benchmark-50-model.md
git commit -m "feat: add validation file 07 - benchmark 50 model"
```

---

### Task 10: Write Validation File 08 - Benchmark 120 Model

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/08-benchmark-120-model.md`

- [ ] **Step 1: Create validation file 08**

Write the complete validation file with:
- **Objective:** Maximum stress test with 120 concurrent models
- **Test workflow with 120 model references in a single workflow**
- **Verify:**
  - System handles this load without crashing
  - Maximum throughput is measurable (tokens/second, requests/second)
  - Identify any system limits (connection pool size, thread pool size, memory)
  - Verify graceful failure if resources are exhausted
  - Recovery mechanism works (can resume after failure)
- **This is the critical test from the user's explicit requirement**
- **Log all metrics to `benchmarks/max-120-model.json`**
- **Generate performance report comparing 5→20→50→120 scaling**
- **Test procedures and expected results**

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/08-benchmark-120-model.md
```

- [ ] **Step 3: Commit validation file 08**

```bash
git add opencode/docs/plans/08-final-validation/08-benchmark-120-model.md
git commit -m "feat: add validation file 08 - benchmark 120 model"
```

---

### Task 11: Write Validation File 09 - Workflow Generation Test

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/09-workflow-generation-test.md`

- [ ] **Step 1: Create validation file 09**

Write the complete validation file with:
- **Objective:** THE KEY TEST - Create a workflow that generates new workflows
- **This is the user's explicit requirement: "attempting other example workflows mainly focusing on the workflow that makes other workflows that fit the schema"**

**Test Steps:**
1. Create a "workflow generator" workflow that:
   - Takes a template description as input
   - Uses an LLM to generate a YAML workflow definition
   - Validates the generated workflow against the schema
   - Saves the generated workflow to a file

2. Execute the workflow generator to create multiple test workflows:
   - Simple 1-step workflow
   - 3-step sequential workflow
   - Parallel 2-model workflow
   - Conditional workflow
   - Error-handling workflow

3. For each generated workflow:
   - (a) Verify it passes schema validation: `cargo run --bin agentsdk validate <generated-workflow.yaml>`
   - (b) Execute it against a real backend: `cargo run --bin agentsdk run <generated-workflow.yaml>`
   - (c) Verify it produces meaningful results (not empty, not error)

4. Test with multiple generation templates:
   - Template 1: "Create a workflow that answers a simple question"
   - Template 2: "Create a workflow that processes data through 3 models in sequence"
   - Template 3: "Create a workflow that uses conditional logic"
   - Template 4: "Create a workflow that handles errors gracefully"

**Success Criteria:**
- All generated workflows pass schema validation
- All generated workflows execute successfully
- All generated workflows produce meaningful results
- Generation process is repeatable

**Test procedures, expected results, and verification commands**

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/09-workflow-generation-test.md
```

- [ ] **Step 3: Commit validation file 09**

```bash
git add opencode/docs/plans/08-final-validation/09-workflow-generation-test.md
git commit -m "feat: add validation file 09 - workflow generation test"
```

---

### Task 12: Write Validation File 10 - Cross-Phase Regression

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/10-cross-phase-regression.md`

- [ ] **Step 1: Create validation file 10**

Write the complete validation file with:
- **Objective:** Re-run ALL tests from phases 0-7 to verify no regressions

**Test Steps:**
1. **Phase 0 Tests:** Verify project initialization, configuration
2. **Phase 1 Tests:** Verify schema validation, YAML parsing
3. **Phase 2 Tests:** Verify workflow execution engine
4. **Phase 3 Tests:** Verify LLM backend integration
5. **Phase 4 Tests:** Verify CLI implementation
6. **Phase 5 Tests:** Verify queue management
7. **Phase 6 Tests:** Verify scheduler implementation
8. **Phase 7 Tests:** Verify metrics and observability

**Upgrade Path Testing:**
- Load old state files from Phase 7
- Verify state can be resumed
- Verify backward compatibility with previous workflow formats

**Success Criteria:**
- All tests from phases 0-7 pass
- No new warnings or errors
- Upgrade path works correctly
- Backward compatibility maintained

**Test procedures, expected results, and verification commands**

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/10-cross-phase-regression.md
```

- [ ] **Step 3: Commit validation file 10**

```bash
git add opencode/docs/plans/08-final-validation/10-cross-phase-regression.md
git commit -m "feat: add validation file 10 - cross-phase regression"
```

---

### Task 13: Write Validation File 11 - Final Signoff

**Files:**
- Create: `/home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/11-final-signoff.md`

- [ ] **Step 1: Create validation file 11**

Write the complete validation file with:
- **Objective:** Final acceptance criteria and signoff

**Final Acceptance Criteria:**
1. ✅ All 12 validation files (00-11) pass completely
2. ✅ All 53 example workflows execute successfully
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

**Performance Report:**
- Generate comprehensive performance report comparing all benchmarks
- Document system limits discovered
- Document bottlenecks and optimization opportunities

**Final Checklist:**
- [ ] All 12 validation files execute without errors
- [ ] All 53 workflows verified
- [ ] All benchmarks meet targets
- [ ] No critical bugs remain
- [ ] Documentation complete
- [ ] ADR compliance verified
- [ ] Performance report generated
- [ ] Signoff approved

**Test procedures, expected results, and verification commands**

- [ ] **Step 2: Verify file was created**

```bash
ls -l /home/jon/code/whitt-execution-engine/opencode/docs/plans/08-final-validation/11-final-signoff.md
```

- [ ] **Step 3: Commit validation file 11**

```bash
git add opencode/docs/plans/08-final-validation/11-final-signoff.md
git commit -m "feat: add validation file 11 - final signoff"
```

---

## Execution Sequence

**Execute validation files in order:**

1. `00-system-log-verification.md` - Foundation layer
2. `01-unit-test-verification.md` - Component validation
3. `02-integration-test-verification.md` - Cross-component validation
4. `03-cli-live-testing.md` - User interface validation
5. `04-workflow-execution-tests.md` - Real-world workload validation
6. `05-benchmark-5-model.md` - Baseline performance
7. `06-benchmark-20-model.md` - Scaling verification
8. `07-benchmark-50-model.md` - Stress testing
9. `08-benchmark-120-model.md` - Maximum load testing
10. `09-workflow-generation-test.md` - Meta-validation
11. `10-cross-phase-regression.md` - Regression testing
12. `11-final-signoff.md` - Final acceptance

**Stop and fix any failures before proceeding to the next validation file.**

---

## Failure Recovery

**If a validation file fails:**
1. Review the failure output
2. Identify the root cause
3. Implement a fix
4. Re-run the failed validation file
5. Only proceed to the next validation file after the current one passes

**Document all fixes in the validation file itself or in a `FIXES.md` alongside the validation files.**

---

## Success Metrics

**Phase 8 is complete when:**
- ✅ All 12 validation files have been created
- ✅ All 12 validation files execute without errors
- ✅ All 53 example workflows execute successfully
- ✅ All benchmarks meet their targets
- ✅ Performance report is generated
- ✅ Final signoff checklist is complete

---

## Notes

- **Critical User Requirements:**
  - "ramp up from a 5 model benchmarking workflow then going up to 120 model single workflow test run" → Covered in files 05-08
  - "attempting other example workflows mainly focusing on the workflow that makes other workflows that fit the schema and will produce results that will actually run and work in the engine" → Covered in file 09

- **Testing Strategy:**
  - Bottom-up: Start with foundation (logs), build up to complex workflows
  - Progressive complexity: 5→20→50→120 models to identify scaling limits
  - Meta-validation: System validates its own generated workflows

- **Performance Expectations:**
  - Linear scaling is ideal, but may not be achievable for all operations
  - Document where scaling deviates from linear and why
  - Focus on identifying system limits and graceful degradation

- **Documentation:**
  - Keep detailed logs of all validation runs
  - Capture performance metrics for all benchmarks
  - Document any unexpected behaviors or edge cases discovered

---

## References

- **Phase 0-7 Plans:** `opencode/docs/plans/00-phase-0/` through `07-phase-7/`
- **Example Workflows:** `opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/`
- **Schema:** `agentsdk-core/src/schema/workflow.schema.json`
- **ADR:** `opencode/docs/adr/`
- **CI/CD:** `.github/workflows/`

---

**End of Phase 8 Final Validation Implementation Plan**
