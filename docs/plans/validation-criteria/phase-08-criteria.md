# Phase 08: Final Validation - Validation Criteria

**Phase Focus:** End-to-end validation of all 53 workflows, 5→120 model benchmarks, workflow generation
**Entry Criteria:** ALL prior phases complete (00-07)
**Estimated Duration:** 2-3 weeks
**Blocking for:** None (final phase)

---

## Phase Overview

Phase 08 is the final validation phase. This phase executes all 53 example workflows, runs model benchmarks from 5 to 120 models, validates workflow generation, performs cross-phase regression testing, and verifies system logs. This is the comprehensive validation that ensures the entire system works end-to-end.

**Critical Success Factors:**
1. All 53 workflows execute successfully
2. Model benchmarks pass (5→120 models)
3. Workflow generation works
4. Cross-phase regression clean
5. System logs verified

---

## Phase Entry Criteria

**Status:** Must be verified before starting Phase 08

**Verification Commands:**
```bash
# Verify ALL prior phases exit
cargo test --test phase_00_integration -- --test-threads=1
cargo test --test phase_01_integration -- --test-threads=1
cargo test --test phase_02_integration -- --test-threads=1
cargo test --test phase_04_integration -- --test-threads=1
cargo test --test phase_05_integration -- --test-threads=1
cargo test --test phase_06_integration -- --test-threads=1
cargo test --test phase_07_integration -- --test-threads=1

# Verify schema coverage for ALL prior phases
for phase in 0 1 2 4 5 6 7; do
  cargo run --bin schema_audit -- --phase $phase --output phase_${phase}_coverage.md
done
```

**Prerequisites:**
- [ ] Phases 00-07 exit criteria verified (all 7 layers)
- [ ] Schema coverage 100% for phases 00-07
- [ ] ADR compliance verified for phases 00-07
- [ ] Cross-phase regression clean (Phases 00-07)
- [ ] All 53 example workflows present
- [ ] Model benchmark infrastructure ready
- [ ] Workflow generation system ready

**Blocking Violations:**
- Unresolved Phase 00-07 failures
- Schema coverage < 100% for any phase
- Cross-phase regression detected
- Missing example workflows
- Model benchmark infrastructure not ready

---

## All 53 Workflows Execution

**Requirement:** All 53 example workflows execute successfully

### Workflow Categories

1. **Basic Workflows (15):**
   - Simple linear workflows
   - Single-pipeline workflows
   - Multi-step workflows

2. **Complex Control Flow (12):**
   - Nested loops
   - Conditional branches
   - Retry logic
   - Threshold gating

3. **Variable Interpolation (8):**
   - Simple variable references
   - Nested variable references
   - Scope inheritance
   - Default values

4. **Error Scenarios (10):**
   - Missing required fields
   - Invalid types
   - Out-of-range values
   - Circular dependencies

5. **Edge Cases (8):**
   - Empty workflows
   - Large workflows (100+ steps)
   - Unicode content
   - Comments and special characters

### Execution Commands

```bash
# Execute all 53 workflows
find examples/workflows -name "*.yaml" -o -name "*.yml" | while read file; do
  echo "Executing $file..."
  cargo run --bin agentsdk -- run "$file" || exit 1
done

# Count executed workflows
find examples/workflows -name "*.yaml" -o -name "*.yml" | wc -l
# Expected output: 53

# Verify all workflows completed successfully
for workflow in $(find examples/workflows -name "*.yaml" -o -name "*.yml"); do
  grep -q "Workflow completed successfully" "/tmp/${workflow}_output.log" || exit 1
done
```

### Pass Criteria

- [ ] All 53 workflows execute without crash
- [ ] All workflows complete successfully
- [ ] Execution time < 5 minutes per workflow
- [ ] No memory leaks (valgrind clean)
- [ ] All outputs valid

### Evidence Required

- Execution logs for all 53 workflows
- Execution time metrics
- Valgrind report (no memory leaks)
- Output validation results

---

## Model Benchmarks (5→120)

**Requirement:** Model benchmarks pass (5→120 models)

### Benchmark Models

**Initial Set (5 models):**
- gpt-4
- gpt-3.5-turbo
- claude-3-opus
- claude-3-sonnet
- llama-3-70b

**Expanded Set (120 models):**
- OpenAI models (20)
- Anthropic models (15)
- Meta models (10)
- Google models (15)
- Mistral models (10)
- Local models (30)
- Custom models (20)

### Benchmark Categories

1. **Latency:** Response time (p50, p95, p99)
2. **Throughput:** Requests per second
3. **Quality:** Response quality (simulated)
4. **Cost:** Cost per 1K tokens
5. **Reliability:** Success rate

### Benchmark Commands

```bash
# Run initial benchmarks (5 models)
cargo bench --bench model_benchmarks -- --benchmarks 5_models

# Run expanded benchmarks (120 models)
cargo bench --bench model_benchmarks -- --benchmarks 120_models

# Generate benchmark report
cargo run --bin benchmark_report -- --output benchmark_report.md
```

### Pass Criteria

- [ ] Initial benchmarks pass (5 models)
- [ ] Expanded benchmarks pass (120 models)
- [ ] Latency within SLA (p95 < 2x p50)
- [ ] Throughput acceptable (> 10 requests/second)
- [ ] Quality acceptable (> 90%)
- [ ] Cost within budget
- [ ] Reliability > 99%

### Evidence Required

- Benchmark results (5 models)
- Benchmark results (120 models)
- Benchmark report
- Performance metrics

---

## Workflow Generation

**Requirement:** Workflow generation works

### Generation Scenarios

1. **Simple Workflow:** Generate simple workflow
2. **Complex Workflow:** Generate complex workflow with loops/branches
3. **Sub-Workflow:** Generate sub-workflow
4. **Workflow from Template:** Generate workflow from template

### Generation Commands

```bash
# Test workflow generation
cargo test --lib generation::tests::simple_workflow_generation
cargo test --lib generation::tests::complex_workflow_generation
cargo test --lib generation::tests::sub_workflow_generation
cargo test --lib generation::tests::template_workflow_generation

# Test generated workflows
for workflow in generated_workflows/*.yaml; do
  echo "Validating $workflow..."
  cargo run --bin agentsdk -- validate "$workflow" || exit 1
  cargo run --bin agentsdk -- run "$workflow" || exit 1
done
```

### Pass Criteria

- [ ] All generation scenarios work
- [ ] Generated workflows valid
- [ ] Generated workflows execute successfully
- [ ] Generated workflows compilable
- [ ] No syntax errors in generated workflows

### Evidence Required

- Workflow generation test results
- Generated workflow samples
- Validation results for generated workflows
- Execution logs for generated workflows

---

## Cross-Phase Regression

**Requirement:** Cross-phase regression clean

### Regression Test Scope

**Phases 00-07 Regression:**
- Phase 00: Foundation (parsing, validation, IR)
- Phase 01: MVP Queue (scheduler, loops, branches)
- Phase 02: CLI & Backends (CLI, backends, tools, RAG)
- Phase 04: Quality Loops (GVR loops, benchmarks)
- Phase 05: Memory & Search (memory, search, scraping)
- Phase 06: Automation (cron, git experiments)
- Phase 07: Autonomy (autonomous loops, override)

### Regression Commands

```bash
# Run ALL prior phase tests
cargo test --test phase_00_integration -- --test-threads=1
cargo test --test phase_01_integration -- --test-threads=1
cargo test --test phase_02_integration -- --test-threads=1
cargo test --test phase_04_integration -- --test-threads=1
cargo test --test phase_05_integration -- --test-threads=1
cargo test --test phase_06_integration -- --test-threads=1
cargo test --test phase_07_integration -- --test-threads=1

# Verify schema coverage for ALL prior phases
for phase in 0 1 2 4 5 6 7; do
  cargo run --bin schema_audit -- --phase $phase --output phase_${phase}_regression.md
done
```

### Pass Criteria

- [ ] ALL Phase 00 tests still pass
- [ ] ALL Phase 01 tests still pass
- [ ] ALL Phase 02 tests still pass
- [ ] ALL Phase 04 tests still pass
- [ ] ALL Phase 05 tests still pass
- [ ] ALL Phase 06 tests still pass
- [ ] ALL Phase 07 tests still pass
- [ ] Schema coverage 100% for ALL phases

### Evidence Required

- Regression test results (all phases)
- Schema coverage reports (all phases)

---

## System Logs Verification

**Requirement:** System logs verified

### Log Scopes (9 scopes)

1. **pipeline**: Pipeline-level events
2. **step**: Step-level events
3. **model**: Model interactions
4. **tool**: Tool invocations
5. **backend**: Backend connections
6. **ui**: UI events
7. **system**: System events
8. **automation**: Automation triggers
9. **autonomy**: Autonomous decisions

### Log Structure Verification

```json
{
  "timestamp": "2026-04-06T10:00:00Z",
  "scope": "pipeline",
  "level": "info",
  "workflow_id": "uuid",
  "message": "Pipeline started",
  "metadata": {}
}
```

### Verification Commands

```bash
# Verify all 9 scopes emit logs
for scope in pipeline step model tool backend ui system automation autonomy; do
  echo "Testing $scope scope..."
  cargo run --bin agentsdk -- run examples/workflows/test.yaml 2>&1 | \
    jq -e "select(.scope == \"$scope\")" || exit 1
done

# Verify log structure
cargo run --bin agentsdk -- run examples/workflows/test.yaml 2>&1 | \
  jq -e '.timestamp and .scope and .level and .message'

# Verify no missing required fields
cargo run --bin agentsdk -- run examples/workflows/test.yaml 2>&1 | \
  jq -e 'select(.timestamp == null or .scope == null or .level == null or .message == null)'
# Expected: No output (no missing fields)
```

### Pass Criteria

- [ ] All 9 scopes emit logs
- [ ] Logs include all required fields
- [ ] Log structure valid JSON
- [ ] No missing required fields
- [ ] Log levels appropriate

### Evidence Required

- Log samples from all 9 scopes
- Log structure validation results
- Required field checklist

---

## Phase Exit Criteria

### Layer 1: Unit Tests

**Commands:**
```bash
cargo test --lib -- --test-threads=1
```

**Evidence:**
- All Phase 08 unit tests pass
- Test coverage >= 90%
- No clippy warnings

### Layer 2: Integration Tests

**Commands:**
```bash
cargo test --test '*' -- --test-threads=1
```

**Evidence:**
- All Phase 08 integration tests pass
- System integrates end-to-end

### Layer 3: Property Tests

**Commands:**
```bash
PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based
```

**Evidence:**
- Workflow generation invariants hold for 1000 iterations
- System-wide invariants hold

### Layer 4: E2E Tests

**Commands:**
```bash
# Execute all 53 workflows
find examples/workflows -name "*.yaml" -o -name "*.yml" | while read file; do
  cargo run --bin agentsdk -- run "$file" || exit 1
done

# Run model benchmarks
cargo bench --bench model_benchmarks -- --benchmarks 120_models

# Test workflow generation
cargo test --lib generation::tests::simple_workflow_generation
cargo test --lib generation::tests::complex_workflow_generation
```

**Evidence:**
- All 53 workflows execute successfully
- Model benchmarks pass (120 models)
- Workflow generation works

### Layer 5: System Log Validation

**Commands:**
```bash
# Verify all 9 scopes emit logs
for scope in pipeline step model tool backend ui system automation autonomy; do
  cargo run --bin agentsdk -- run examples/workflows/test.yaml 2>&1 | \
    jq -e "select(.scope == \"$scope\")" || exit 1
done
```

**Evidence:**
- All 9 scopes emit logs
- Logs include all required fields

### Layer 6: Live CLI Verification

**Commands:**
```bash
# Test all CLI commands
cargo run --bin agentsdk -- --help
cargo run --bin agentsdk -- run examples/workflows/simple.yaml
cargo run --bin agentsdk -- validate examples/workflows/simple.yaml
cargo run --bin agentsdk -- queue list
cargo run --bin agentsdk -- scheduler status
```

**Evidence:**
- All CLI commands work
- Error handling works

### Layer 7: Benchmark Performance

**Commands:**
```bash
cargo bench --bench phase_08_benchmarks
```

**Evidence:**
- End-to-end performance acceptable
- All 53 workflows execute within time limits
- No performance regression > 10%

---

## Cross-Phase Regression Tests

**Commands:**
```bash
# Run ALL prior phase tests
cargo test --test phase_00_integration -- --test-threads=1
cargo test --test phase_01_integration -- --test-threads=1
cargo test --test phase_02_integration -- --test-threads=1
cargo test --test phase_04_integration -- --test-threads=1
cargo test --test phase_05_integration -- --test-threads=1
cargo test --test phase_06_integration -- --test-threads=1
cargo test --test phase_07_integration -- --test-threads=1
```

**Pass Criteria:**
- [ ] ALL Phase 00 tests still pass
- [ ] ALL Phase 01 tests still pass
- [ ] ALL Phase 02 tests still pass
- [ ] ALL Phase 04 tests still pass
- [ ] ALL Phase 05 tests still pass
- [ ] ALL Phase 06 tests still pass
- [ ] ALL Phase 07 tests still pass

---

## Schema Coverage Audit

**Requirement:** 100% of ALL schema fields implemented

### ALL Phase-Owned Fields

**Phases 00-07:**
- All schema fields from all phases (00, 01, 02, 04, 05, 06, 07)

### Verification Commands

```bash
cargo run --bin schema_audit -- --phase all --output final_coverage_report.md
```

**Expected Output:**
- 100% coverage for ALL schema fields
- No unimplemented fields

---

## ADR Compliance

**Relevant ADRs:**
- ALL ADRs from phases 00-07

### Verification Commands

```bash
cargo run --bin adr_compliance -- --phase all
```

**Expected Output:**
- ALL ADR constraints satisfied
- No outstanding ADR TODOs

---

## Anti-Goal-Drift Checklist

**Run after phase completion:**

1. **Requirements Drift:**
   - [ ] Implementation matches all phase requirements
   - [ ] No unauthorized features added

2. **Architecture Drift:**
   - [ ] Architecture matches ALL ADRs
   - [ ] No unauthorized architectural changes

3. **Scope Creep:**
   - [ ] Only specified features implemented
   - [ ] No unauthorized scope expansion

---

## Final Acceptance Criteria

**Must ALL be true:**

1. [ ] All 53 workflows execute successfully
2. [ ] Model benchmarks pass (120 models)
3. [ ] Workflow generation works
4. [ ] Cross-phase regression clean (ALL phases)
5. [ ] System logs verified (all 9 scopes)
6. [ ] Schema coverage 100% (ALL fields)
7. [ ] ADR compliance verified (ALL ADRs)
8. [ ] Anti-goal-drift checklist complete
9. [ ] All 7 verification layers pass
10. [ ] Performance within SLA

---

## Evidence Storage

**Location:** `results/phase_08/`

**Contents:**
- `unit_test_results.json`
- `integration_test_output.log`
- `property_test_results.json`
- `e2e_execution_logs/` (all 53 workflows)
- `system_log_samples.json` (all 9 scopes)
- `cli_verification/` (all CLI commands)
- `benchmarks/` (end-to-end performance)
- `workflow_execution/` (53 workflow execution logs)
- `model_benchmarks/` (120 model benchmark results)
- `workflow_generation/` (generated workflows)
- `cross_phase_regression/` (all phase regression results)
- `final_coverage_report.md` (100% schema coverage)
- `final_adr_compliance.md` (all ADRs satisfied)
- `final_acceptance_checklist.md` (final acceptance criteria)
- `phase_00_regression/` through `phase_07_regression/`

---

## Blocking Issues

**Cannot exit Phase 08 if:**
- Any verification layer fails
- Any of 53 workflows fail to execute
- Model benchmarks fail
- Workflow generation doesn't work
- Cross-phase regression detected
- System logs not verified
- Schema coverage < 100%
- ADR compliance violations
- ANY prior phase regression detected

---

## Project Completion

**When Phase 08 completes:**
- All 8 phases validated
- All 53 workflows working
- All 120 models benchmarked
- System ready for production

---

**End of Phase 08 Criteria (FINAL PHASE)**
