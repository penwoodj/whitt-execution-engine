# QA Criteria — Phase 06: Autonomy & Metrics

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Phase Plan**: `docs/plans/06-autonomy-metrics/plan.md` (745 lines, 10 tasks)
**Date**: 2026-04-26
**Status**: 🟡 IN PROGRESS — Implementation not started

---

## Summary Table

| # | QA Area | Schema Ref | Status | Priority | Test Type |
|---|---------|------------|------------|
| 1 | Autonomous Loop Contracts (bounded goals, stop conditions) | N/A (new feature) | P0 | Unit, Integration |
| 2 | Metrics Collection (counters, gauges, histograms) | N/A (new feature) | P0 | Unit, Integration |
| 3 | Human Override Controls (pause, stop, modify, scope-change) | N/A (new feature) | P0 | Unit, Integration |
| 4 | Intervention Tracking (structured logging) | N/A (new feature) | P1 | Unit, Integration |
| 5 | Success Regression Dashboards (real-time, anomaly detection) | N/A (new feature) | P1 | Unit, Integration |
| 6 | Stop Condition Evaluation (time, iteration, quality, intervention) | N/A (new feature) | P0 | Unit, Integration |
| 7 | Checkpoint Generation (periodic, event-triggered) | N/A (new feature) | P1 | Unit, Integration |
| 8 | Autonomy Scope & Risk (boundary enforcement, risk models) | N/A (new feature) | P1 | Unit, Integration |
| 9 | Confidence Thresholds (computed from metrics) | N/A (new feature) | P1 | Unit, Integration |
| 10 | Autonomy CLI/UI Integration (autonomy controls, metrics viewing) | N/A (new feature) | P0 | Integration, E2E |

---

## Area Details

### Area 1: Autonomous Loop Contracts

**Schema Ref**: N/A (new feature - infrastructure for bounded autonomous execution)
**Plan Ref**: Task 00 (`docs/plans/06-autonomy-metrics/tasks/00-autonomous-loop-contracts.md`)
**Files**: `src/autonomy/contracts.rs`, `src/autonomy/contracts/parser.rs`, `src/autonomy/contracts/validator.rs`, `src/autonomy/contracts/enforcer.rs`, `src/autonomy/contracts/goal.rs`, `src/autonomy/contracts/level.rs`

**Criteria**:
- AutonomousLoopContract struct: Define contract with goal definition, stop conditions, autonomy levels
- ContractParser: Parse contract YAML (schema compatible)
- ContractValidator: Validate contract constraints (bounded goals, valid stop conditions)
- BoundedExecutionEnforcer: Enforce stop conditions (time, iteration, quality, intervention, resource limits)
- GoalTracker: Track progress toward goals
- AutonomyLevel enum: Low (every action confirmed), Medium (batch confirmation), High (bounded autonomous)
- StopCondition types: Time (max 24h), Iteration (max 1000), Quality (score < 0.8), Intervention (stop on any), Resource (budget)
- No autonomous loop may run forever: BoundedExecutionEnforcer enforces hard limits

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Unit tests for contract parsing
cargo test --lib autonomy::contracts::parser -- --test-threads=1

# Unit tests for contract validation
cargo test --lib autonomy::contracts::validator -- --test-threads=1

# Unit tests for bounded execution enforcer
cargo test --lib autonomy::contracts::enforcer -- --test-threads=1

# Integration tests
cargo test --test autonomous_loop_contracts -- --test-threads=1 --nocapture
```

---

### Area 2: Metrics Collection

**Schema Ref**: N/A (new feature - instrumentation integrated into tracing)
**Plan Ref**: Task 01 (`docs/plans/06-autonomy-metrics/tasks/01-metrics-collection.md`)
**Files**: `src/metrics/types.rs`, `src/metrics/instrumentation.rs`, `src/metrics/aggregation.rs`, `src/metrics/storage.rs`, `src/metrics/export.rs`

**Criteria**:
- Metric types: Counter (increment only), Gauge (current value), Histogram (duration distribution), Summary (quantiles)
- Instrumentation hooks: Hooks for workflow events (actions taken, interventions, errors)
- Aggregation logic: Sum, avg, p50, p95, p99 aggregation
- In-memory storage with TTL: Store metrics in memory with time-to-live
- Export formats: Prometheus (for Grafana), JSON (for development)
- Existing tracing integration: Use tokio::tracing for instrumentation (ADR-0008 constraint)
- Critical events instrumented: actions_taken, interventions, errors
- Current state instrumented: queue size, active workflows, resource usage
- Duration metrics instrumented: action duration, intervention response time
- Performance overhead: < 5% (ADR-0008 constraint)

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Unit tests for metric types
cargo test --lib metrics::types -- --test-threads=1

# Unit tests for instrumentation
cargo test --lib metrics::instrumentation -- --test-threads=1

# Unit tests for aggregation
cargo test --lib metrics::aggregation -- --test-threads=1

# Unit tests for export
cargo test --lib metrics::export -- --test-threads=1

# Integration tests
cargo test --test metrics_collection -- --test-threads=1 --nocapture

# Performance overhead benchmark
cargo test --bench metrics_overhead -- --test-threads=1
```

---

### Area 3: Human Override Controls

**Schema Ref**: N/A (new feature - always available at all autonomy levels)
**Plan Ref**: Task 02 (`docs/plans/06-autonomy-metrics/tasks/02-human-override-controls.md`)
**Files**: `src/override/controls.rs`, `src/override/handlers.rs`, `src/override/pause.rs`, `src/override/emergency.rs`

**Criteria**:
- OverrideType enum: Pause, Stop, Modify, ScopeChange (reduce/increase autonomy level)
- OverrideEvent: Override command with workflow ID, timestamp, reason
- OverrideHandler: Process overrides (higher priority than autonomous loop)
- PauseManager: Pause/resume state for autonomous workflows
- EmergencyStop: Immediate termination (response < 100ms)
- Available at all autonomy levels: Low, Medium, High (ADR-0008 constraint)
- Non-blocking: Override controls respond within 100ms
- Clear feedback: Provide clear feedback to operator
- Scope-change support: Reduce/increase autonomy level dynamically
- Separate tokio task: Override system runs as separate tokio task

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Unit tests for override controls
cargo test --lib override::controls -- --test-threads=1

# Unit tests for pause manager
cargo test --lib override::pause -- --test-threads=1

# Unit tests for emergency stop
cargo test --lib override::emergency -- --test-threads=1

# Integration tests with response time measurement
cargo test --test human_override_controls -- --test-threads=1 --nocapture

# Response time benchmark
cargo test --bench override_response_time -- --test-threads=1
```

---

### Area 4: Intervention Tracking

**Schema Ref**: N/A (new feature - structured logging of all human interventions)
**Plan Ref**: Task 03 (`docs/plans/06-autonomy-metrics/tasks/03-intervention-tracking.md`)
**Files**: `src/intervention/events.rs`, `src/intervention/context.rs`, `src/intervention/reason.rs`, `src/intervention/logging.rs`, `src/intervention/analysis.rs`

**Criteria**:
- InterventionEvent structure: Workflow ID, intervention type, timestamp, reason, context
- InterventionContext: Capture state (workflow state, action ID, metrics snapshot)
- InterventionReason: Reason tracking with what, why, who, when
- Structured logging: Log to `./workspace/interventions/`
- Analysis logic: Trends, frequency analysis
- Full context capture: No silent failures (ADR-0008 constraint)
- Immutable event log: Event log cannot be modified
- Analysis insights: Identify patterns in interventions (high-frequency actions, error-prone workflows)

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests for intervention events
cargo test --lib intervention::events -- --test-threads=1

# Unit tests for intervention context
cargo test --lib intervention::context -- --test-threads=1

# Unit tests for intervention reason
cargo test --lib intervention::reason -- --test-threads=1

# Unit tests for logging
cargo test --lib intervention::logging -- --test-threads=1

# Unit tests for analysis
cargo test --lib intervention::analysis -- --test-threads=1

# Integration tests
cargo test --test intervention_tracking -- --test-threads=1 --nocapture
```

---

### Area 5: Success Regression Dashboards

**Schema Ref**: N/A (new feature - real-time dashboards)
**Plan Ref**: Task 04 (`docs/plans/06-autonomy-metrics/tasks/04-success-regression-dashboards.md`)
**Files**: `src/dashboard/layout.rs`, `src/dashboard/success_metrics.rs`, `src/dashboard/regression.rs`, `src/dashboard/streaming.rs`, `src/dashboard/anomaly.rs`

**Criteria**:
- Dashboard layout: Success rate, intervention rate, action duration displays
- Success metrics visualization: Time-series charts for success rate
- Regression detection algorithm: EWMA control chart (exponentially weighted moving average)
- Real-time streaming updates: WebSocket or polling for live updates
- Anomaly detection alerts: Detect 10% drop in success rate
- Alert operators in real-time: Immediate notification on regression
- Drill-down into root cause: Link anomalies to specific workflows/actions
- Suggest corrective actions: Recommend fixes based on anomaly patterns
- Dashboard accessibility: HTTP endpoint for dashboard access

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests for dashboard layout
cargo test --lib dashboard::layout -- --test-threads=1

# Unit tests for success metrics
cargo test --lib dashboard::success_metrics -- --test-threads=1

# Unit tests for regression detection
cargo test --lib dashboard::regression -- --test-threads=1

# Unit tests for streaming
cargo test --lib dashboard::streaming -- --test-threads=1

# Integration tests with mock metrics
cargo test --test success_regression_dashboards -- --test-threads=1 --nocapture
```

---

### Area 6: Stop Condition Evaluation

**Schema Ref**: Lines 503-598 (workflow_execution_strategy - for stop conditions)
**Plan Ref**: Task 05 (`docs/plans/06-autonomy-metrics/tasks/05-stop-condition-evaluation.md`)
**Files**: `src/stop_conditions/types.rs`, `src/stop_conditions/parser.rs`, `src/stop_conditions/validator.rs`, `src/stop_conditions/evaluation.rs`

**Criteria**:
- StopCondition types: Time (max duration), Iteration (max actions), Quality (metric threshold), Intervention (stop on any), Resource (budget/memory/CPU)
- StopConditionParser: Parse conditions from contract
- StopConditionValidator: Validate conditions at load time
- StopConditionEvaluator: Runtime evaluation loop integrated into autonomous executor
- Time limits: Enforce max 24h execution time
- Iteration limits: Enforce max 1000 actions
- Quality gates: Stop if success rate < 80%
- Intervention stops: Stop on any human intervention (stop_on_any: true)
- Resource limits: Enforce max $100 budget, memory limits
- Evaluation frequency: Evaluate stop conditions on every iteration
- Early termination: Terminate loop when any stop condition met

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Unit tests for stop condition types
cargo test --lib stop_conditions::types -- --test-threads=1

# Unit tests for parser
cargo test --lib stop_conditions::parser -- --test-threads=1

# Unit tests for validator
cargo test --lib stop_conditions::validator -- --test-threads=1

# Unit tests for evaluator
cargo test --lib stop_conditions::evaluation -- --test-threads=1

# Integration tests
cargo test --test stop_condition_evaluation -- --test-threads=1 --nocapture
```

---

### Area 7: Checkpoint Generation

**Schema Ref**: Lines 568-583 (checkpointing in workflow_execution_strategy)
**Plan Ref**: Task 06 (`docs/plans/06-autonomy-metrics/tasks/06-checkpoint-generation.md`)
**Files**: `src/checkpoint/structure.rs`, `src/checkpoint/periodic.rs`, `src/checkpoint/triggered.rs`, `src/checkpoint/restoration.rs`

**Criteria**:
- Checkpoint structure: Workflow state, metrics, intervention history
- Periodic checkpointing: Generate checkpoints at configurable interval (e.g., every 5 minutes)
- Event-triggered checkpoints: On intervention, on stop condition, on error
- Storage in `./workspace/checkpoints/`: Persist checkpoints
- Restoration logic: Load checkpoint and resume autonomous loop
- Checkpoint key format: `{workflow_id}.{checkpoint_number}`
- State preservation: Capture complete autonomous loop state
- Checkpoint validation: Validate checkpoint integrity on save/load
- Max checkpoints limit: Keep last N checkpoints (e.g., 10)
- Automatic cleanup: Delete checkpoints older than X hours

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests for checkpoint structure
cargo test --lib checkpoint::structure -- --test-threads=1

# Unit tests for periodic checkpointing
cargo test --lib checkpoint::periodic -- --test-threads=1

# Unit tests for triggered checkpointing
cargo test --lib checkpoint::triggered -- --test-threads=1

# Unit tests for restoration
cargo test --lib checkpoint::restoration -- --test-threads=1

# Integration tests
cargo test --test checkpoint_generation -- --test-threads=1 --nocapture
```

---

### Area 8: Autonomy Scope & Risk

**Schema Ref**: N/A (new feature - boundary enforcement and risk assessment)
**Plan Ref**: Task 07 (`docs/plans/06-autonomy-metrics/tasks/07-autonomy-scope-risk.md`)
**Files**: `src/risk/scope.rs`, `src/risk/assessment.rs`, `src/risk/enforcement.rs`, `src/risk/alerting.rs`

**Criteria**:
- ScopeBoundary definition: Define boundaries for autonomous execution
- RiskAssessmentModel: Probability, impact, severity
- ScopeEnforcer: Enforce boundary restrictions
- RiskAlerting: Alert on high-risk actions
- Scope change support: Reduce/increase autonomy level dynamically
- Risk thresholds: Configurable thresholds for probability, impact, severity
- Boundary violations: Detect and block boundary violations
- High-risk action detection: Detect actions with high probability of failure
- Risk mitigation: Suggest mitigations for high-risk actions

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests for scope boundaries
cargo test --lib risk::scope -- --test-threads=1

# Unit tests for risk assessment
cargo test --lib risk::assessment -- --test-threads=1

# Unit tests for enforcement
cargo test --lib risk::enforcement -- --test-threads=1

# Unit tests for alerting
cargo test --lib risk::alerting -- --test-threads=1

# Integration tests
cargo test --test autonomy_scope_risk -- --test-threads=1 --nocapture
```

---

### Area 9: Confidence Thresholds

**Schema Ref**: N/A (new feature - computed from metrics)
**Plan Ref**: Task 08 (`docs/plans/06-autonomy-metrics/tasks/08-confidence-thresholds.md`)
**Files**: `src/confidence/threshold.rs`, `src/confidence/computation.rs`, `src/confidence/enforcement.rs`, `src/confidence/tuning.rs`

**Criteria**:
- ConfidenceThreshold structure: Threshold definition with metric, operator, value
- ConfidenceComputation: Compute confidence from metrics (success rate, error rate, duration)
- ConfidenceEnforcer: Enforce threshold-based decisions (continue/stop/escalate)
- ConfidenceTuning: Adjust thresholds based on performance data
- Logging: Log confidence decisions with context
- Confidence sources: Success rate, error rate, intervention rate, resource usage
- Confidence algorithms: Weighted average, exponential moving average, rule-based
- Threshold enforcement: Stop autonomous loop if confidence below threshold
- Tuning feedback: Adjust thresholds based on false positives/negatives

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests for confidence thresholds
cargo test --lib confidence::threshold -- --test-threads=1

# Unit tests for computation
cargo test --lib confidence::computation -- --test-threads=1

# Unit tests for enforcement
cargo test --lib confidence::enforcement -- --test-threads=1

# Unit tests for tuning
cargo test --lib confidence::tuning -- --test-threads=1

# Integration tests
cargo test --test confidence_thresholds -- --test-threads=1 --nocapture
```

---

### Area 10: Autonomy CLI/UI Integration

**Schema Ref**: N/A (new feature - CLI commands for autonomy control and metrics viewing)
**Plan Ref**: Task 09 (`docs/plans/06-autonomy-metrics/tasks/09-autonomy-cli-ui.md`)
**Files**: `src/cli/autonomy.rs`, `src/cli/metrics.rs`, UI components

**Criteria**:
- CLI commands: `whitt autonomy start/pause/stop`, `whitt metrics view/export`
- Autonomy control commands: Start autonomous workflow, pause/resume/stop, modify scope
- Metrics viewing commands: View current metrics, export metrics
- Interactive mode: Interactive prompts for confirmations
- Error messages: Clear error messages for invalid inputs
- Help text: Accurate help text for all commands
- Exit codes: Proper exit codes (0 for success, non-zero for errors)
- UI integration: Display autonomy status and metrics in UI
- Real-time metrics display: Live updates of metrics in UI
- Override controls in UI: Pause, stop, modify, scope-change buttons

**Test Type**: Integration, E2E
**Priority**: P0

**Commands**:
```bash
# Unit tests for CLI parsing
cargo test --lib cli::autonomy -- --test-threads=1

# Unit tests for CLI metrics
cargo test --lib cli::metrics -- --test-threads=1

# Integration tests for autonomy commands
cargo test --test cli_autonomy_commands -- --test-threads=1 --nocapture

# Integration tests for metrics commands
cargo test --test cli_metrics_commands -- --test-threads=1 --nocapture

# Live CLI testing
cargo run --bin whitt autonomy --help
cargo run --bin whitt metrics --help
```

---

## ADR-0008 Compliance Validation

- [ ] Bounded execution enforced (time, iteration, quality, intervention, resource limits)
- [ ] Human override available at all autonomy levels (Low, Medium, High)
- [ ] Metrics instrumented BEFORE any UI/UX experimentation begins
- [ ] Observability integrated into runtime (tokio::tracing), not bolted on
- [ ] Success regression detection (10% drop in success rate detected and alerted)
- [ ] All interventions logged with full context (workflow ID, action ID, state, metrics snapshot)
- [ ] Override controls responsive within 100ms
- [ ] Performance overhead < 5% for metrics collection

---

## Performance Targets

| Metric | Target | Validation Method |
|--------|--------|------------------|
| Override control response time | < 100ms | Benchmark test |
| Metrics collection overhead | < 5% | Benchmark test |
| Checkpoint save time | < 1s | Benchmark test |
| Stop condition evaluation | < 10ms | Benchmark test |
| Dashboard update latency | < 100ms | Benchmark test |

---

## Phase Exit Criteria

Phase 06 is complete when:

1. **All 10 tasks** are implemented and passing their validation criteria
2. **All tests** pass with proper mock strategies
3. **ADR-0008 compliance** verified through full checklist
4. **Integration tests** demonstrate end-to-end autonomy workflow
5. **Documentation** covers all APIs, configuration, and usage patterns
6. **Performance benchmarks** meet targets (override response < 100ms, metrics overhead < 5%)
7. **No silent failures**: All interventions, stop condition violations, metric anomalies logged

---

## Related Plan Tasks

| QA Area | Plan Task | Plan File |
|----------|------------|------------|
| Area 1: Autonomous Loop Contracts | Task 00 | [00-autonomous-loop-contracts.md](../plans/06-autonomy-metrics/tasks/00-autonomous-loop-contracts.md) |
| Area 2: Metrics Collection | Task 01 | [01-metrics-collection.md](../plans/06-autonomy-metrics/tasks/01-metrics-collection.md) |
| Area 3: Human Override Controls | Task 02 | [02-human-override-controls.md](../plans/06-autonomy-metrics/tasks/02-human-override-controls.md) |
| Area 4: Intervention Tracking | Task 03 | [03-intervention-tracking.md](../plans/06-autonomy-metrics/tasks/03-intervention-tracking.md) |
| Area 5: Success Regression Dashboards | Task 04 | [04-success-regression-dashboards.md](../plans/06-autonomy-metrics/tasks/04-success-regression-dashboards.md) |
| Area 6: Stop Condition Evaluation | Task 05 | [05-stop-condition-evaluation.md](../plans/06-autonomy-metrics/tasks/05-stop-condition-evaluation.md) |
| Area 7: Checkpoint Generation | Task 06 | [06-checkpoint-generation.md](../plans/06-autonomy-metrics/tasks/06-checkpoint-generation.md) |
| Area 8: Autonomy Scope & Risk | Task 07 | [07-autonomy-scope-risk.md](../plans/06-autonomy-metrics/tasks/07-autonomy-scope-risk.md) |
| Area 9: Confidence Thresholds | Task 08 | [08-confidence-thresholds.md](../plans/06-autonomy-metrics/tasks/08-confidence-thresholds.md) |
| Area 10: Autonomy CLI/UI Integration | Task 09 | [09-autonomy-cli-ui.md](../plans/06-autonomy-metrics/tasks/09-autonomy-cli-ui.md) |

---

**End of QA Criteria for Phase 06**
