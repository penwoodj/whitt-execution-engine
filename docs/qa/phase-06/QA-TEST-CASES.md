# QA Test Cases — Phase 06: Autonomy & Metrics

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Test ID Prefix**: P06
**Date**: 2026-04-26

---

## Test Case Summary Table

| Test ID | QA Area | Description | Command | Expected Result |
|----------|----------|-------------|----------|----------------|
| P06-001 | Autonomous Loop Contracts | Contract parsing | cargo test --lib autonomy::contracts::parser::test_contract_parsing | Contracts parsed correctly |
| P06-002 | Autonomous Loop Contracts | Contract validation | cargo test --lib autonomy::contracts::validator::test_validation | Invalid contracts rejected |
| P06-003 | Autonomous Loop Contracts | Bounded execution enforcer | cargo test --lib autonomy::contracts::enforcer::test_enforcement | Hard limits enforced |
| P06-004 | Autonomous Loop Contracts | Autonomy levels | cargo test --lib autonomy::contracts::level::test_levels | Low/Medium/High work correctly |
| P06-005 | Autonomous Loop Contracts | No infinite loops | cargo test --test autonomous_loop_no_infinite_loops -- --test-threads=1 --nocapture | All loops terminate within limits |
| P06-006 | Metrics Collection | Metric types (Counter/Gauge/Histogram/Summary) | cargo test --lib metrics::types::test_types | All types work correctly |
| P06-007 | Metrics Collection | Instrumentation hooks | cargo test --lib metrics::instrumentation::test_hooks | Workflow events instrumented |
| P06-008 | Metrics Collection | Aggregation logic | cargo test --lib metrics::aggregation::test_aggregation | Sum/avg/p50/p95/p99 computed correctly |
| P06-009 | Metrics Collection | Export formats (Prometheus/JSON) | cargo test --lib metrics::export::test_export | Export formats work correctly |
| P06-010 | Metrics Collection | Performance overhead < 5% (ADR-0008) | cargo test --bench metrics_overhead -- --test-threads=1 | Overhead < 5% |
| P06-011 | Human Override Controls | Override types | cargo test --lib override::controls::test_types | All override types work |
| P06-012 | Human Override Controls | Response time < 100ms (ADR-0008) | cargo test --bench override_response_time -- --test-threads=1 | p50 < 100ms |
| P06-013 | Human Override Controls | Available at all autonomy levels | cargo test --test override_available_at_all_levels -- --test-threads=1 --nocapture | Overrides work at Low/Medium/High |
| P06-014 | Human Override Controls | Emergency stop | cargo test --lib override::emergency::test_emergency_stop | Immediate termination works |
| P06-015 | Intervention Tracking | Intervention events | cargo test --lib intervention::events::test_events | All interventions logged |
| P06-016 | Intervention Tracking | Context capture (state snapshot) | cargo test --lib intervention::context::test_capture | Full context captured |
| P06-017 | Intervention Tracking | Reason tracking | cargo test --lib intervention::reason::test_reason | What/why/who/when captured |
| P06-018 | Intervention Tracking | Analysis logic (trends/frequency) | cargo test --lib intervention::analysis::test_analysis | Trends identified |
| P06-019 | Success Regression Dashboards | Regression detection (10% drop) | cargo test --lib dashboard::regression::test_regression | 10% drop detected |
| P06-020 | Success Regression Dashboards | Real-time streaming | cargo test --lib dashboard::streaming::test_streaming | Live updates work |
| P06-021 | Success Regression Dashboards | Alert operators | cargo test --test dashboards_alert_operators -- --test-threads=1 --nocapture | Operators alerted in real-time |
| P06-022 | Success Regression Dashboards | Dashboard accessibility | cargo test --test dashboards_accessibility -- --test-threads=1 --nocapture | HTTP endpoint accessible |
| P06-023 | Stop Condition Evaluation | Stop condition types | cargo test --lib stop_conditions::types::test_types | All types work correctly |
| P06-024 | Stop Condition Evaluation | Parser and validator | cargo test --lib stop_conditions::parser::test_parser | Conditions parsed/validated |
| P06-025 | Stop Condition Evaluation | Runtime evaluation | cargo test --lib stop_conditions::evaluation::test_evaluation | Conditions evaluated every iteration |
| P06-026 | Stop Condition Evaluation | Early termination | cargo test --test stop_condition_early_termination -- --test-threads=1 --nocapture | Loop terminates when condition met |
| P06-027 | Checkpoint Generation | Checkpoint structure | cargo test --lib checkpoint::structure::test_structure | State/metrics/interventions captured |
| P06-028 | Checkpoint Generation | Periodic checkpointing | cargo test --lib checkpoint::periodic::test_periodic | Checkpoints at interval |
| P06-029 | Checkpoint Generation | Event-triggered checkpointing | cargo test --lib checkpoint::triggered::test_triggered | Checkpoints on intervention/error |
| P06-030 | Checkpoint Generation | Restoration logic | cargo test --lib checkpoint::restoration::test_restoration | Checkpoint restores state |
| P06-031 | Autonomy Scope & Risk | Scope boundaries | cargo test --lib risk::scope::test_boundaries | Boundaries enforced |
| P06-032 | Autonomy Scope & Risk | Risk assessment model | cargo test --lib risk::assessment::test_model | Probability/impact/severity computed |
| P06-033 | Autonomy Scope & Risk | Enforcement and alerting | cargo test --lib risk::enforcement::test_enforcement | Boundaries enforced, high-risk alerts |
| P06-034 | Confidence Thresholds | Computation from metrics | cargo test --lib confidence::computation::test_computation | Confidence computed correctly |
| P06-035 | Confidence Thresholds | Enforcement | cargo test --lib confidence::enforcement::test_enforcement | Thresholds enforced correctly |
| P06-036 | Confidence Thresholds | Tuning feedback | cargo test --lib confidence::tuning::test_tuning | Thresholds adjusted based on performance |
| P06-037 | Autonomy CLI/UI Integration | Autonomy commands | cargo test --test cli_autonomy_commands -- --test-threads=1 --nocapture | Start/pause/stop/modify work |
| P06-038 | Autonomy CLI/UI Integration | Metrics commands | cargo test --test cli_metrics_commands -- --test-threads=1 --nocapture | View/export work |
| P06-039 | Integration | End-to-end autonomy workflow | cargo run --bin whitt autonomy examples/autonomy-workflow.yaml | Autonomy workflow executes successfully |

---

## Detailed Test Cases

### P06-001: Autonomous Loop Contracts - Contract Parsing

**Description**: Verify autonomous loop contracts are parsed correctly
**QA Area**: Area 1 - Autonomous Loop Contracts
**Priority**: P0
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib autonomy::contracts::parser::test_contract_parsing -- --test-threads=1
```

**Expected Result**:
- Contract YAML parsed correctly
- Goal definition extracted
- Stop conditions extracted (time, iteration, quality, intervention, resource)
- Autonomy level parsed (Low, Medium, High)
- Invalid contracts rejected with clear error
- Contract structure validated

**Verification**:
```bash
# Check test output
cargo test --lib autonomy::contracts::parser::test_contract_parsing -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P06-005: Autonomous Loop Contracts - No Infinite Loops

**Description**: Verify no autonomous loop may run forever
**QA Area**: Area 1 - Autonomous Loop Contracts
**Priority**: P0
**Test Type**: Integration

**Test Command**:
```bash
cargo test --test autonomous_loop_no_infinite_loops -- --test-threads=1 --nocapture
```

**Expected Result**:
- All loops terminate within defined limits
- Time limit enforced (max 24h)
- Iteration limit enforced (max 1000)
- Quality gate enforced (stop if success rate < 80%)
- Intervention stop enforced (stop_on_any: true)
- Resource limit enforced (max $100 budget)
- BoundedExecutionEnforcer terminates loop when any limit violated

**Verification**:
```bash
# Check test output
cargo test --test autonomous_loop_no_infinite_loops -- --test-threads=1 --nocapture 2>&1 | grep -q "test result: ok"
```

---

### P06-010: Metrics Collection - Performance Overhead < 5% (ADR-0008)

**Description**: Verify metrics collection overhead is less than 5%
**QA Area**: Area 2 - Metrics Collection
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench metrics_overhead -- --test-threads=1
```

**Expected Result**:
- Overhead < 5% compared to baseline (no instrumentation)
- p50 overhead < 3%
- p95 overhead < 5%
- No significant performance impact from instrumentation
- Observability integrated into runtime (tokio::tracing), not bolted on (ADR-0008 constraint)

**Verification**:
```bash
# Parse benchmark results
cargo test --bench metrics_overhead -- --test-threads=1 2>&1 | grep -q "overhead.*< 5%"
```

---

### P06-012: Human Override Controls - Response Time < 100ms (ADR-0008)

**Description**: Verify human override controls respond within 100ms
**QA Area**: Area 3 - Human Override Controls
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench override_response_time -- --test-threads=1
```

**Expected Result**:
- p50 response time < 100ms
- p95 response time < 150ms
- p99 response time < 200ms
- Override controls responsive to all override types
- Override system has higher priority than autonomous loop
- Separate tokio task ensures responsiveness (ADR-0008 constraint)

**Verification**:
```bash
# Parse benchmark results
cargo test --bench override_response_time -- --test-threads=1 2>&1 | grep -q "p50.*< 100ms"
```

---

### P06-013: Human Override Controls - Available at All Autonomy Levels

**Description**: Verify override controls are available at all autonomy levels (Low, Medium, High)
**QA Area**: Area 3 - Human Override Controls
**Priority**: P0
**Test Type**: Integration

**Test Command**:
```bash
cargo test --test override_available_at_all_levels -- --test-threads=1 --nocapture
```

**Expected Result**:
- Override controls work at Low autonomy (every action confirmed)
- Override controls work at Medium autonomy (batch confirmation)
- Override controls work at High autonomy (bounded autonomous with checkpoints)
- All override types (Pause, Stop, Modify, ScopeChange) available
- Human override ALWAYS available (ADR-0008 constraint)
- No mode where system operates without human oversight

**Verification**:
```bash
# Check test output
cargo test --test override_available_at_all_levels -- --test-threads=1 --nocapture 2>&1 | grep -q "test result: ok"
```

---

### P06-019: Success Regression Dashboards - Regression Detection (10% Drop)

**Description**: Verify 10% drop in success rate is detected and alerted
**QA Area**: Area 5 - Success Regression Dashboards
**Priority**: P1
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib dashboard::regression::test_regression -- --test-threads=1
```

**Expected Result**:
- EWMA control chart detects 10% drop in success rate
- Alert operators notified immediately on regression
- Drill-down into root cause links anomaly to specific workflows
- Corrective actions suggested based on anomaly patterns
- Real-time detection (not delayed batch processing)
- Configurable sensitivity (adjust EWMA parameters)

**Verification**:
```bash
# Check test output
cargo test --lib dashboard::regression::test_regression -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P06-037: Autonomy CLI/UI Integration - Autonomy Commands

**Description**: Verify autonomy control commands work correctly
**QA Area**: Area 10 - Autonomy CLI/UI Integration
**Priority**: P0
**Test Type**: Integration

**Test Command**:
```bash
cargo test --test cli_autonomy_commands -- --test-threads=1 --nocapture
```

**Expected Result**:
- `whitt autonomy start` starts autonomous workflow
- `whitt autonomy pause` pauses autonomous workflow
- `whitt autonomy stop` stops autonomous workflow immediately
- `whitt autonomy modify` modifies autonomy scope
- Commands validate arguments before execution
- Clear error messages for invalid inputs
- Accurate help text for all commands
- Proper exit codes (0 success, non-zero errors)

**Verification**:
```bash
# Check test output
cargo test --test cli_autonomy_commands -- --test-threads=1 --nocapture 2>&1 | grep -q "test result: ok"
```

---

### P06-039: Integration - End-to-End Autonomy Workflow

**Description**: Verify complete autonomy workflow executes end-to-end
**QA Area**: Area 10 - Autonomy CLI/UI Integration
**Priority**: P0
**Test Type**: E2E

**Test Command**:
```bash
cargo run --bin whitt autonomy examples/autonomy-workflow.yaml
```

**Expected Result**:
- Workflow loads successfully from YAML
- Autonomous loop contract parsed and validated
- Bounded execution enforced throughout workflow
- Override controls available and responsive
- Metrics collected (actions_taken, interventions, errors)
- Stop conditions evaluated and enforced
- Checkpoints generated at intervals and on events
- Success regression detected (if applicable) and alerted
- No infinite loops (all bounded execution enforced)
- No silent failures (all events logged)
- Exit code is 0

**Verification**:
```bash
# Check execution succeeded
if cargo run --bin whitt autonomy examples/autonomy-workflow.yaml; then
  echo "E2E test PASSED"
else
  echo "E2E test FAILED"
  exit 1
fi

# Check artifacts exist
ls -lh workspace/checkpoints/
ls -lh workspace/interventions/
ls -lh workspace/metrics/
```

---

## Performance Test Cases

### P06-PERF-001: Override Control Response Time

**Description**: Verify override control response time meets < 100ms target
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench override_response_time -- --test-threads=1
```

**Expected Result**:
- p50 response time < 100ms
- p95 response time < 150ms
- p99 response time < 200ms
- No significant regression from baseline

**Verification**:
```bash
# Parse benchmark results
cargo test --bench override_response_time -- --test-threads=1 2>&1 | grep -q "p50.*< 100ms"
```

---

### P06-PERF-002: Checkpoint Save Time

**Description**: Verify checkpoint save time meets < 1s target
**Priority**: P1
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench checkpoint_save_time -- --test-threads=1
```

**Expected Result**:
- p50 save time < 1s
- p95 save time < 2s
- No significant regression from baseline

**Verification**:
```bash
# Parse benchmark results
cargo test --bench checkpoint_save_time -- --test-threads=1 2>&1 | grep -q "p50.*< 1s"
```

---

### P06-PERF-003: Stop Condition Evaluation

**Description**: Verify stop condition evaluation time meets < 10ms target
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench stop_condition_evaluation -- --test-threads=1
```

**Expected Result**:
- p50 evaluation time < 10ms
- p95 evaluation time < 20ms
- No significant regression from baseline

**Verification**:
```bash
# Parse benchmark results
cargo test --bench stop_condition_evaluation -- --test-threads=1 2>&1 | grep -q "p50.*< 10ms"
```

---

### P06-PERF-004: Dashboard Update Latency

**Description**: Verify dashboard update latency meets < 100ms target
**Priority**: P1
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench dashboard_update_latency -- --test-threads=1
```

**Expected Result**:
- p50 latency < 100ms
- p95 latency < 150ms
- No significant regression from baseline

**Verification**:
```bash
# Parse benchmark results
cargo test --bench dashboard_update_latency -- --test-threads=1 2>&1 | grep -q "p50.*< 100ms"
```

---

## Mock Strategy Notes

### Autonomous Loops (bounded execution)
- Mock autonomous loop execution with deterministic behavior
- Test stop condition enforcement with time control
- Mock goal tracking progress

### Metrics Collection (instrumentation)
- Mock workflow events for testing instrumentation hooks
- Test aggregation logic with mock metrics data
- Test export formats without actual Prometheus server

### Human Override Controls (responsive)
- Mock override events with timing measurement
- Test separate tokio task with higher priority
- Measure response time accurately

### Success Regression Dashboards (real-time)
- Mock metrics data with regression patterns
- Test EWMA control chart algorithm
- Mock WebSocket for streaming updates

### Stop Condition Evaluation (early termination)
- Mock autonomous loop with configurable stop conditions
- Test early termination scenarios
- Test all stop condition types (time, iteration, quality, intervention, resource)

### Checkpoint Generation (state persistence)
- Mock checkpoint storage and loading
- Test periodic and event-triggered checkpointing
- Mock state restoration

---

**End of QA Test Cases for Phase 06**
