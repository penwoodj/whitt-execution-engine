# Phase 07: Autonomy - Validation Criteria

**Phase Focus:** Autonomous loops, metrics, human override, dashboards, confidence thresholds
**Entry Criteria:** ALL prior phases complete (00-06)
**Estimated Duration:** 4-5 weeks
**Blocking for:** Phase 08

---

## Phase Overview

Phase 07 implements autonomy capabilities. This phase provides autonomous workflow execution, comprehensive metrics, human override mechanisms, real-time dashboards, and confidence threshold enforcement. Autonomy enables workflows to run with minimal human intervention while maintaining safety.

**Critical Success Factors:**
1. Autonomous loops bounded (no infinite loops)
2. Metrics comprehensive (all execution components)
3. Human override available
4. Dashboards real-time
5. Confidence thresholds enforced

---

## Phase Entry Criteria

**Status:** Must be verified before starting Phase 07

**Verification Commands:**
```bash
# Verify ALL prior phases exit
cargo test --test phase_00_integration -- --test-threads=1
cargo test --test phase_01_integration -- --test-threads=1
cargo test --test phase_02_integration -- --test-threads=1
cargo test --test phase_04_integration -- --test-threads=1
cargo test --test phase_05_integration -- --test-threads=1
cargo test --test phase_06_integration -- --test-threads=1

# Verify schema coverage for ALL prior phases
for phase in 0 1 2 4 5 6; do
  cargo run --bin schema_audit -- --phase $phase --output phase_${phase}_coverage.md
done
```

**Prerequisites:**
- [ ] Phases 00-06 exit criteria verified (all 7 layers)
- [ ] Schema coverage 100% for phases 00-06
- [ ] ADR compliance verified for phases 00-06
- [ ] Cross-phase regression clean (Phases 00-06)
- [ ] Dashboard infrastructure ready
- [ ] Override mechanism ready

**Blocking Violations:**
- Unresolved Phase 00-06 failures
- Schema coverage < 100% for any phase
- Cross-phase regression detected
- Dashboard infrastructure not ready

---

## Autonomous Loops

**Requirement:** Autonomous loops bounded (no infinite loops)

### Loop Types

1. **Continuous Loop:** Run indefinitely (with bounds)
2. **Periodic Loop:** Run at intervals
3. **Conditional Loop:** Run while condition true
4. **Event-Driven Loop:** Run on events

### Loop Bounds

1. **Max Iterations:** Terminate after N iterations
2. **Max Duration:** Terminate after T seconds
3. **Max Resources:** Terminate after R resources used
4. **Manual Stop:** Terminate on manual stop

### Verification Commands

```bash
# Test autonomous loops
cargo test --lib autonomy::tests::continuous_loop
cargo test --lib autonomy::tests::periodic_loop
cargo test --lib autonomy::tests::conditional_loop
cargo test --lib autonomy::tests::event_driven_loop

# Test loop bounds
cargo test --lib autonomy::tests::max_iterations_bound
cargo test --lib autonomy::tests::max_duration_bound
cargo test --lib autonomy::tests::max_resources_bound
cargo test --lib autonomy::tests::manual_stop

# Test infinite loop prevention
cargo test --lib autonomy::tests::infinite_loop_prevention
```

### Pass Criteria

- [ ] All loop types work correctly
- [ ] All bounds enforced
- [ ] Loops terminate on bounds
- [ ] Manual stop works
- [ ] No infinite loops

### Evidence Required

- Autonomous loop test results
- Bound enforcement logs
- Infinite loop prevention logs

---

## Comprehensive Metrics

**Requirement:** Metrics comprehensive (all execution components)

### Metric Categories

1. **Workflow Metrics:**
   - workflow_duration_seconds
   - workflow_success_count
   - workflow_failure_count
   - workflow_retry_count

2. **Pipeline Metrics:**
   - pipeline_duration_seconds
   - pipeline_step_count
   - pipeline_success_count

3. **Step Metrics:**
   - step_duration_seconds
   - step_success_count
   - step_failure_count

4. **Model Metrics:**
   - model_request_count
   - model_response_tokens
   - model_latency_seconds

5. **Tool Metrics:**
   - tool_invocation_count
   - tool_success_count
   - tool_failure_count

6. **Autonomy Metrics:**
   - autonomous_loop_count
   - autonomous_iteration_count
   - autonomous_override_count

### Metrics Export

**Prometheus Format:**
```
agentsdk_workflow_duration_seconds{workflow_id="uuid"} 10.5
agentsdk_pipeline_duration_seconds{pipeline_id="uuid"} 5.2
agentsdk_step_duration_seconds{step_id="uuid"} 2.1
agentsdk_model_response_tokens{model_id="gpt-4"} 1500
agentsdk_tool_invocation_count{tool_id="search"} 3
agentsdk_autonomous_loop_count{loop_id="uuid"} 100
```

### Verification Commands

```bash
# Test comprehensive metrics
cargo test --lib autonomy::tests::workflow_metrics
cargo test --lib autonomy::tests::pipeline_metrics
cargo test --lib autonomy::tests::step_metrics
cargo test --lib autonomy::tests::model_metrics
cargo test --lib autonomy::tests::tool_metrics
cargo test --lib autonomy::tests::autonomy_metrics

# Test metrics export
cargo test --lib autonomy::tests::metrics_prometheus_export
```

### Pass Criteria

- [ ] All metric categories collected
- [ ] All metrics accurate
- [ ] Metrics export correctly
- [ ] No missing metrics

### Evidence Required

- Metrics collection test results
- Metrics export samples
- Metrics completeness checklist

---

## Human Override

**Requirement:** Human override available

### Override Types

1. **Manual Stop:** Manually stop autonomous loop
2. **Pause:** Pause autonomous loop (resumeable)
3. **Modify:** Modify loop parameters
4. **Rollback:** Rollback loop execution

### Override Workflow

```
1. Human requests override
2. System validates override
3. Override applied
4. Execution stops/pauses/modifies
5. Override logged
```

### Verification Commands

```bash
# Test human override
cargo test --lib autonomy::tests::manual_stop
cargo test --lib autonomy::tests::pause
cargo test --lib autonomy::tests::modify
cargo test --lib autonomy::tests::rollback

# Test override validation
cargo test --lib autonomy::tests::override_validation
cargo test --lib autonomy::tests::override_logging
```

### Pass Criteria

- [ ] All override types work
- [ ] Override validated before application
- [ ] Override applied correctly
- [ ] Override logged
- [ ] No data loss on override

### Evidence Required

- Override test results
- Validation logs
- Override logs

---

## Real-Time Dashboards

**Requirement:** Dashboards real-time

### Dashboard Components

1. **Workflow Status:** Real-time workflow status
2. **Queue Status:** Real-time queue status
3. **Metrics Dashboard:** Real-time metrics
4. **Logs Dashboard:** Real-time logs
5. **Alerts Dashboard:** Real-time alerts

### Dashboard Updates

1. **WebSocket:** Real-time updates via WebSocket
2. **Polling:** Fallback to polling if WebSocket unavailable
3. **Caching:** Cache dashboard data for performance

### Verification Commands

```bash
# Test dashboard components
cargo test --lib autonomy::tests::workflow_status_dashboard
cargo test --lib autonomy::tests::queue_status_dashboard
cargo test --lib autonomy::tests::metrics_dashboard
cargo test --lib autonomy::tests::logs_dashboard
cargo test --lib autonomy::tests::alerts_dashboard

# Test real-time updates
cargo test --lib autonomy::tests::websocket_updates
cargo test --lib autonomy::tests::polling_updates
cargo test --lib autonomy::tests::caching
```

### Pass Criteria

- [ ] All dashboard components work
- [ ] Real-time updates work
- [ ] WebSocket preferred over polling
- [ ] Caching improves performance
- [ ] Dashboard updates within 1 second

### Evidence Required

- Dashboard test results
- Real-time update logs
- Performance metrics

---

## Confidence Thresholds

**Requirement:** Confidence thresholds enforced

### Threshold Types

1. **Action Threshold:** Minimum confidence to take action
2. **Override Threshold:** Minimum confidence to override human
3. **Stop Threshold:** Minimum confidence to continue autonomous loop

### Threshold Enforcement

```yaml
autonomy:
  confidence_thresholds:
    action_threshold: 0.8
    override_threshold: 0.9
    stop_threshold: 0.7
```

### Verification Commands

```bash
# Test confidence thresholds
cargo test --lib autonomy::tests::action_threshold
cargo test --lib autonomy::tests::override_threshold
cargo test --lib autonomy::tests::stop_threshold

# Test threshold enforcement
cargo test --lib autonomy::tests::threshold_enforcement
cargo test --lib autonomy::tests::threshold_logging
```

### Pass Criteria

- [ ] All thresholds enforced
- [ ] Actions below threshold blocked
- [ ] Overrides below threshold blocked
- [ ] Loops stop below threshold
- [ ] Threshold violations logged

### Evidence Required

- Threshold test results
- Enforcement logs
- Threshold violation logs

---

## Phase Exit Criteria

### Layer 1: Unit Tests

**Commands:**
```bash
cargo test --lib -- --test-threads=1
```

**Evidence:**
- All Phase 07 unit tests pass
- Test coverage >= 90%
- No clippy warnings

### Layer 2: Integration Tests

**Commands:**
```bash
cargo test --test '*' -- --test-threads=1
```

**Evidence:**
- All Phase 07 integration tests pass
- Autonomy integrates correctly

### Layer 3: Property Tests

**Commands:**
```bash
PROPTEST_NUMBER_OF_TESTS=100 cargo test --lib property_based
```

**Evidence:**
- Loop bound invariants hold for 100 iterations
- Override safety invariants hold

### Layer 4: E2E Tests

**Commands:**
```bash
# Execute autonomy workflows
cargo run --bin agentsdk -- run examples/workflows/autonomous_loop.yaml
cargo run --bin agentsdk -- run examples/workflows/human_override.yaml
cargo run --bin agentsdk -- run examples/workflows/confidence_threshold.yaml
```

**Evidence:**
- Autonomous loops bounded
- Human override works
- Confidence thresholds enforced

### Layer 5: System Log Validation

**Commands:**
```bash
# Verify autonomy scope logs
cargo run --bin agentsdk -- run examples/workflows/autonomy.yaml 2>&1 | \
  jq -e 'select(.scope == "autonomy")'
```

**Evidence:**
- Autonomy scope logs emitted
- Logs include timestamp, scope, event, message

### Layer 6: Live CLI Verification

**Commands:**
```bash
# Test autonomy commands
cargo run --bin agentsdk -- autonomy start --workflow test.yaml --mode autonomous
cargo run --bin agentsdk -- autonomy stop --loop_id <loop_id>
cargo run --bin agentsdk -- autonomy pause --loop_id <loop_id>
cargo run --bin agentsdk -- autonomy resume --loop_id <loop_id>
```

**Evidence:**
- Autonomy commands work
- Error handling works

### Layer 7: Benchmark Performance

**Commands:**
```bash
cargo bench --bench phase_07_benchmarks
```

**Evidence:**
- Autonomy performance acceptable
- Dashboard updates fast
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
```

**Pass Criteria:**
- [ ] ALL Phase 00 tests still pass
- [ ] ALL Phase 01 tests still pass
- [ ] ALL Phase 02 tests still pass
- [ ] ALL Phase 04 tests still pass
- [ ] ALL Phase 05 tests still pass
- [ ] ALL Phase 06 tests still pass

---

## Schema Coverage Audit

**Requirement:** 100% of Phase 07-owned fields implemented

### Phase 07-Owned Fields

**AutonomySchema:**
- mode
- loop_type
- max_iterations
- max_duration_seconds
- max_resources_mb

**ConfidenceThresholdSchema:**
- action_threshold
- override_threshold
- stop_threshold

**OverrideSchema:**
- override_id
- type
- user_id
- timestamp
- reason

**DashboardSchema:**
- dashboard_id
- components
- update_interval_ms
- cache_ttl_ms

### Verification Commands

```bash
cargo run --bin schema_audit -- --phase 7 --output coverage_report.md
```

**Expected Output:**
- 100% coverage for Phase 07-owned fields
- No unimplemented fields

---

## ADR Compliance

**Relevant ADRs:**
- ADR-013: Autonomy Architecture
- ADR-014: Human Override Safety

### Verification Commands

```bash
cargo run --bin adr_compliance -- --phase 7
```

**Expected Output:**
- All Phase 07-relevant ADR constraints satisfied
- No outstanding ADR TODOs

---

## Anti-Goal-Drift Checklist

**Run after phase completion:**

1. **Requirements Drift:**
   - [ ] Implementation matches Phase 07 requirements
   - [ ] No features from later phases added

2. **Architecture Drift:**
   - [ ] Autonomy matches ADR-013
   - [ ] Override matches ADR-014

3. **Scope Creep:**
   - [ ] Only autonomy features implemented
   - [ ] No final validation features added

---

## Evidence Storage

**Location:** `results/phase_07/`

**Contents:**
- `unit_test_results.json`
- `integration_test_output.log`
- `property_test_results.json`
- `e2e_execution_logs/` (autonomy workflows)
- `system_log_samples.json` (autonomy scope)
- `cli_verification/` (autonomy commands)
- `benchmarks/` (autonomy performance)
- `autonomous_loops/` (loop execution logs)
- `metrics_export/` (comprehensive metrics)
- `override_logs/` (human override logs)
- `dashboard_snapshots/` (dashboard screenshots)
- `confidence_logs/` (threshold enforcement logs)
- `schema_coverage_report.md`
- `adr_compliance_report.md`
- `anti_goal_drift_checklist.md`
- `phase_00_regression/`
- `phase_01_regression/`
- `phase_02_regression/`
- `phase_04_regression/`
- `phase_05_regression/`
- `phase_06_regression/`

---

## Blocking Issues

**Cannot exit Phase 07 if:**
- Any verification layer fails
- Autonomous loops not bounded
- Metrics not comprehensive
- Human override not available
- Dashboards not real-time
- Confidence thresholds not enforced
- ANY prior phase regression detected

---

**End of Phase 07 Criteria**
