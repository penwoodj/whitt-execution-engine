# Plan-07: Autonomous Loops and Metrics-Driven FE and BE UX Expansion

**Plan ID**: plan-07
**Phase**: Phase 7 - Autonomy & Metrics
**Status**: Not Started
**Created**: 2026-03-27
**Related ADR**: ADR-0008 (Autonomous loops and metrics-driven FE and BE UX expansion)
**Related Research Plan**: research-plan-05-automation-autonomy-metrics.yml
**Estimated Time**: 10-12 weeks
**Dependencies**: plan-00 (Foundation), plan-01 (MVP Queue), plan-02 (CLI & Backends), plan-03 (Glyphnova UI), plan-04 (Quality Loops), plan-05 (Memory & Search), plan-06 (Automation)

---

## Overview

This plan implements bounded autonomous loops with comprehensive metrics instrumentation, enabling useful local autonomy while maintaining safety, inspectability, and human override capabilities.

**Key Features**:
- **Autonomous Loop Contracts**: Declared workflow modes with bounded goals, checkpoints, and stop conditions
- **Metrics Instrumentation**: Comprehensive metrics for workflows and model performance
- **Objective Measurements**: Usefulness, time-to-usefulness, intervention rate, repair rate, benchmark quality, operator trust signals
- **Human Override Controls**: Pause, stop, and scope visibility available at all times
- **Stop Conditions and Checkpoints**: Explicit bounds and intermediate state capture
- **Success and Regression Dashboards**: Real-time visibility into autonomous operations
- **Intervention Logging**: Track all human interventions with context
- **Confidence and Validation Thresholds**: Prevent unsafe autonomous decisions
- **Autonomy Scope Boundaries**: Risk assessment and boundary enforcement
- **Observability Integration**: Integrated into transpiler runtime (not bolted on)

**Key Deliverables**:
- Autonomous loop contract system
- Metrics collection and aggregation
- Success and regression dashboards
- Human override controls
- Intervention tracking and logging
- Stop condition evaluation
- Checkpoint generation and restoration
- Autonomy scope and risk assessment
- Confidence and validation thresholds

---

## Code Review

### ADR-0008 Summary

**Decision**: Enable autonomy only on top of measured, reviewable foundations with transpiler observability.

**Key Requirements**:
1. **Autonomous Loop Contracts**: Declared workflow modes with bounded goals, checkpoints, and stop conditions
2. **Metrics Instrumentation**: Instrumented before large-scale interface experimentation
3. **Objective Measurements**: Usefulness, time-to-usefulness, intervention rate, repair rate, benchmark quality, operator trust signals
4. **Human Override**: Pause and scope visibility remain available at all times
5. **Provenance Inheritance**: Autonomy inherits provenance policy and artifact requirements from earlier ADRs
6. **Observability Integration**: Integrated into transpiler runtime (not bolted on)

**Scope**:
- **Included**: Autonomous loop contracts, stop conditions, metrics instrumentation, dashboards, human override, intervention logging, confidence thresholds, autonomy scope boundaries
- **Excluded**: Fully autonomous always-on loops without bounds, opaque self-modifying behavior without artifacts, unmeasured autonomous decisions, removal of human override or pause capabilities

**Positive Consequences**:
- Progress toward useful local autonomy without losing auditability
- Objective metrics drive interface improvements
- Human operators maintain control while automation improves
- Bounded autonomy enables safe experimentation

**Negative Consequences**:
- Autonomy pressure can tempt premature expansion
- Instrumentation overhead is real
- Scope boundaries may need adjustment over time

### Requirements Review

From `requirements.md` and `schema-consolidated-report.md`:

**R12**: Runtime review, rigor, scope, concurrency, efficiency, logging, context-budget, policy layer
**R25**: Generate to verify to repair review loop semantics
**R30**: Observability with logging levels, summaries, graphs, reports, and provenance
**R04**: Depth-over-speed and capability-normalized outcomes
**R26**: Long-running behavior for background, semi-endless, endless, and resumable workflows

**Validation Focus for v0.1.0**:
- Autonomous loops respect bounded goals and stop conditions
- Metrics collection captures all relevant performance and quality signals
- Human override and pause controls are always functional
- Dashboards provide real-time visibility into autonomous operations
- Observability integrates with `.opencode/` artifact system
- Validation thresholds prevent unsafe autonomous expansion

**Validation Focus for v0.1.x and Later**:
- Bounded autonomy enables safe experimentation with clear limits
- Metrics-driven decisions are based on objective data, not guesses
- Human operators can intervene at any point with full context
- Dashboards enable trend analysis and anomaly detection
- Autonomy features integrate with queue scheduler and artifact tracking
- Progressive autonomy is possible through validated threshold adjustments

### Transpiler Integration Points

- **WorkflowIR**: Should support autonomous loop modes and bounded execution
- **Observability Hooks**: Must capture autonomous loop events
- **Metrics Collection**: Should be integrated into WorkflowIR execution
- **Stop Conditions and Checkpoints**: Should be first-class WorkflowIR nodes
- **Human Override Controls**: Must override autonomous workflow execution
- **Dashboard Queries**: Should aggregate autonomous workflow metrics
- **Autonomy Policies**: Should be compiled into WorkflowIR (not dynamic)

### Research Plan Review

**research-plan-05-automation-autonomy-metrics.yml** is **COMPLETED** with 3 research domains (plan-07 uses domain 2):

2. **Autonomy and Metrics**: What autonomy models and metrics frameworks best support bounded, safe autonomous loops?

**Key Findings**:
- Report outputs: `automation-autonomy-metrics-research-report.md`, `autonomy-patterns.csv`, `metrics-frameworks.csv`
- Quality gates met: Recommendations include safety, observability, and human control

---

## Web Research

### Research Area 1: Autonomous Loop Contract Models

**Research Question**: What autonomous loop contract models support bounded, safe autonomy?

**Recommended Sources**:
- [RL literature](https://arxiv.org) - Reinforcement learning safety
- [OpenAI autonomy](https://github.com/openai) - Safety patterns
- [AutoGPT patterns](https://github.com) - Autonomous agent patterns
- [BabyAGI](https://github.com) - Autonomous task management
- [LangChain autonomy](https://python.langchain.com/) - Autonomous workflow patterns

**Expected Findings**:
- Autonomous loop contract definitions
- Bounded execution patterns
- Goal and checkpoint structures
- Stop condition evaluation
- Risk assessment models
- Autonomy level definitions

**Status**: Not Started

---

### Research Area 2: Metrics Frameworks for Autonomous Systems

**Research Question**: What metrics frameworks and instrumentation patterns support autonomous system monitoring?

**Recommended Sources**:
- [OpenTelemetry](https://opentelemetry.io) - Observability framework
- [Prometheus](https://prometheus.io) - Metrics collection
- [Grafana](https://grafana.com) - Visualization
- [Dapr](https://dapr.io) - Distributed application runtime
- [MLflow](https://mlflow.org) - ML experiment tracking

**Expected Findings**:
- Metrics instrumentation patterns
- Metric types (counter, gauge, histogram, summary)
- Metrics aggregation and querying
- Real-time dashboard patterns
- Anomaly detection
- Metric correlation and analysis

**Status**: Not Started

---

### Research Area 3: Human Override and Intervention Logging

**Research Question**: What human override and intervention logging patterns maintain safety while enabling autonomy?

**Recommended Sources**:
- [GitHub Actions](https://docs.github.com/en/actions) - Manual approval patterns
- [CI/CD safety](https://owasp.org) - Human intervention patterns
- [Audit logging](https://nvlpubs.nist.gov) - Audit log standards
- [Incident response](https://en.wikipedia.org/wiki/Incident_response) - Intervention patterns
- [Human-in-the-loop](https://dl.acm.org) - Research on human autonomy interaction

**Expected Findings**:
- Override control patterns (pause, stop, modify)
- Intervention logging structures
- Context capture for interventions
- Intervention reason tracking
- Post-intervention analysis
- Safety lock mechanisms

**Status**: Not Started

---

### Research Area 4: Success and Regression Dashboards

**Research Question**: What dashboard patterns provide real-time visibility into autonomous operations?

**Recommended Sources**:
- [Grafana dashboards](https://grafana.com/grafana/dashboards) - Dashboard examples
- [Observable](https://observablehq.com) - Interactive dashboards
- [Datadog](https://www.datadoghq.com) - Monitoring dashboards
- [Elasticsearch Kibana](https://www.elastic.co/kibana) - Data visualization
- [D3.js](https://d3js.org) - Custom visualization

**Expected Findings**:
- Dashboard layout patterns
- Real-time data streaming
- Success metric visualization
- Regression detection and alerting
- Trend analysis charts
- Anomaly highlighting
- Drill-down capabilities

**Status**: Not Started

---

### Research Area 5: Confidence and Validation Thresholds

**Research Question**: What confidence and validation threshold patterns prevent unsafe autonomous decisions?

**Recommended Sources**:
- [Confidence intervals](https://en.wikipedia.org/wiki/Confidence_interval) - Statistical confidence
- [Statistical hypothesis testing](https://en.wikipedia.org/wiki/Statistical_hypothesis_testing) - Validation
- [Safety-critical systems](https://nvlpubs.nist.gov) - Safety patterns
- [Risk assessment](https://www.iso.org) - ISO risk standards
- [Autonomous vehicle safety](https://www.nhtsa.gov) - Safety threshold patterns

**Expected Findings**:
- Confidence threshold computation
- Validation criteria design
- Risk assessment models
- Threshold tuning strategies
- Conservative vs aggressive tradeoffs
- Threshold failure handling

**Status**: Not Started

---

## Implementation Plan

### Phase 1: Autonomous Loop Contract System

**Description**: Implement autonomous loop contract definition and enforcement

**Tasks**:
1. Define autonomous loop contract structure (goals, checkpoints, stop conditions)
2. Implement contract parser and validator
3. Add autonomy level definitions (low, medium, high)
4. Implement bounded execution enforcer
5. Add goal tracking and evaluation
6. Implement checkpoint generation (intermediate state capture)
7. Add stop condition evaluation
8. Implement contract CLI commands (define, validate, activate)

**Related Requirements**: ADR-0008 (autonomous loop contracts)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 2: Metrics Collection and Aggregation

**Description**: Implement comprehensive metrics collection for autonomous workflows

**Tasks**:
1. Define metric types (counter, gauge, histogram, summary)
2. Implement metrics instrumentation hooks in WorkflowIR execution
3. Add metrics for autonomous workflows:
   - Usefulness
   - Time-to-usefulness
   - Intervention rate
   - Repair rate
   - Benchmark quality
   - Operator trust signals
4. Implement metrics aggregation (sum, avg, p50, p95, p99)
5. Add metrics storage (time-series database)
6. Implement metrics query interface
7. Add metrics export (Prometheus format, JSON)
8. Implement metrics CLI commands

**Related Requirements**: ADR-0008 (metrics instrumentation)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 3: Human Override Controls

**Description**: Implement human override and pause controls

**Tasks**:
1. Define override control types (pause, stop, modify, scope-change)
2. Implement override event handlers
3. Add pause and resume functionality
4. Implement stop condition override (emergency stop)
5. Add scope visibility during override
6. Implement override reason capture
7. Add override logging (with context and state)
8. Implement override CLI commands (pause, resume, stop, status)
9. Add override UI integration

**Related Requirements**: ADR-0008 (human override)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 4: Intervention Tracking and Logging

**Description**: Track all human interventions with full context

**Tasks**:
1. Define intervention event structure (type, reason, context, timestamp)
2. Implement intervention capture for all override types
3. Add intervention context capture (workflow state, metrics, decisions)
4. Implement intervention reason tracking
5. Add intervention logging (structured logs in `.opencode/`)
6. Implement intervention query interface
7. Add intervention analysis (trends, frequency, reasons)
8. Implement intervention CLI commands (list, view, analyze)

**Related Requirements**: ADR-0008 (intervention logging)
**Verification Layers**: 1, 2
**Status**: Not Started

---

### Phase 5: Success and Regression Dashboards

**Description**: Create dashboards for real-time visibility into autonomous operations

**Tasks**:
1. Design dashboard layout (autonomous operations overview)
2. Implement success metrics visualization (usefulness, repair rate)
3. Add regression detection (quality degradation, increased interventions)
4. Implement real-time data streaming
5. Add trend analysis charts (time-series metrics)
6. Implement anomaly detection and alerting
7. Add drill-down capabilities (from summary to details)
8. Implement dashboard UI integration
9. Add dashboard export (screenshots, reports)

**Related Requirements**: ADR-0008 (dashboards)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 6: Stop Condition Evaluation

**Description**: Implement stop condition evaluation and enforcement

**Tasks**:
1. Define stop condition types (time, iteration, quality, intervention, resource)
2. Implement stop condition parser and validator
3. Add stop condition evaluation loop (periodic checks)
4. Implement time-based stop conditions (max duration)
5. Add iteration-based stop conditions (max loops)
6. Implement quality-based stop conditions (quality threshold)
7. Add intervention-based stop conditions (intervention count)
8. Implement resource-based stop conditions (memory, CPU, GPU)
9. Add stop condition CLI commands

**Related Requirements**: ADR-0008 (stop conditions)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 7: Checkpoint Generation and Restoration

**Description**: Implement intermediate checkpoint generation and restoration

**Tasks**:
1. Define checkpoint structure (state, metrics, decisions, timestamp)
2. Implement periodic checkpoint generation
3. Add checkpoint-on-event (intervention, milestone, error)
4. Implement checkpoint storage (in `.opencode/checkpoints/`)
5. Add checkpoint versioning and compression
6. Implement checkpoint restoration
7. Add checkpoint validation (integrity check)
8. Implement checkpoint CLI commands (list, view, restore)

**Related Requirements**: ADR-0008 (checkpoints)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 8: Autonomy Scope and Risk Assessment

**Description**: Implement autonomy scope boundaries and risk assessment

**Tasks**:
1. Define autonomy scope boundaries (allowed operations, resource limits)
2. Implement risk assessment model (probability, impact, severity)
3. Add risk evaluation (pre-execution, during execution)
4. Implement scope boundary enforcement
5. Add risk-based autonomy level adjustment
6. Implement risk alerting (high risk notifications)
7. Add risk logging (assessment history)
8. Implement scope and risk CLI commands (assess, set-boundaries, view-risk)

**Related Requirements**: ADR-0008 (autonomy scope boundaries)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 9: Confidence and Validation Thresholds

**Description**: Implement confidence thresholds and validation for autonomous decisions

**Tasks**:
1. Define confidence threshold structure (metric, threshold, action)
2. Implement confidence computation (from metrics, quality scores)
3. Add validation criteria (must-meet thresholds)
4. Implement threshold enforcement (block action if below threshold)
5. Add threshold tuning (manual adjustment, auto-tuning)
6. Implement threshold validation (test before deployment)
7. Add threshold logging (changes, justifications)
8. Implement threshold CLI commands (set, validate, adjust)

**Related Requirements**: ADR-0008 (confidence thresholds)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

## Verification Strategy

### Layer 1: Unit Tests

**Scope**: Individual component testing

**Coverage Areas**:
- Autonomous loop contract parsing and validation
- Metrics instrumentation and aggregation
- Override control handlers
- Intervention event structures
- Stop condition evaluation
- Checkpoint generation and restoration
- Risk assessment computation
- Confidence threshold enforcement

**Test Framework**: `cargo test --lib`

**Success Criteria**:
- 90%+ code coverage on core modules
- All contract types tested (valid and invalid)
- All stop condition types tested
- All override types tested

---

### Layer 2: Integration Tests

**Scope**: Multi-component interaction testing

**Coverage Areas**:
- Autonomous loop execution with contracts
- Metrics collection from autonomous workflows
- Human override with full context capture
- Intervention tracking and analysis
- Dashboard data streaming and visualization
- Stop condition enforcement during execution
- Checkpoint generation and restoration
- Risk assessment with scope enforcement
- CLI and UI integration with all autonomy features

**Test Framework**: `cargo test --test '*'`

**Success Criteria**:
- Autonomous loops execute with bounded contracts
- Metrics are collected and aggregated correctly
- Human override works at any point
- Interventions are captured with full context
- Dashboards display real-time data
- Stop conditions are enforced
- Checkpoints are generated and restorable
- Risk assessments are computed and logged

---

### Layer 3: Property-Based Tests

**Scope**: Edge cases and invariants

**Coverage Areas**:
- Autonomous loop contract validity (always valid or always rejected)
- Metrics aggregation correctness (deterministic aggregation)
- Override enforcement (override always takes precedence)
- Intervention tracking completeness (all interventions captured)
- Stop condition correctness (always evaluates deterministically)
- Checkpoint integrity (always restores exact state)
- Risk assessment bounds (risk always in [0,1])
- Confidence threshold enforcement (threshold always respected)

**Test Framework**: `proptest` with 1000 iterations each

**Success Criteria**:
- No crashes on random inputs
- Invariants always hold (metrics deterministic, override enforced, thresholds respected)
- Properties verified with 1000+ iterations

---

### Layer 4: End-to-End Tests

**Scope**: Complete autonomous workflows with safety controls

**Coverage Areas**:
- Execute autonomous workflow with bounded contract
- Override autonomous workflow during execution
- Track interventions and view dashboard
- Evaluate stop conditions and trigger
- Generate and restore checkpoints
- Assess risk and adjust scope boundaries
- Monitor metrics and detect regressions
- Validate confidence thresholds

**Test Framework**: `cargo test --test '*e2e*'`

**Success Criteria**:
- Complete autonomous workflows execute successfully
- Contracts enforce bounded execution
- Human override works at any point with full context
- Dashboards provide real-time visibility
- Stop conditions prevent runaway autonomy
- Checkpoints restore state correctly
- Risk assessments are accurate
- Confidence thresholds prevent unsafe decisions

---

## Verification Checkpoints

### Checkpoint 1: Autonomous Loop Contracts Working

**Target Date**: Week 2
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] Autonomous loop contracts parse correctly
- [ ] Contracts are validated
- [ ] Bounded execution is enforced
- [ ] Goals are tracked and evaluated
- [ ] Checkpoints are generated periodically
- [ ] Stop conditions are evaluated
- [ ] CLI contract commands work
- [ ] Property tests verify contract validity
- [ ] Integration tests pass with real workflows

**Status**: Not Started

---

### Checkpoint 2: Metrics Collection Functional

**Target Date**: Week 4
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] All metric types are collected
- [ ] Metrics are aggregated correctly
- [ ] Metrics storage works
- [ ] Query interface returns correct results
- [ ] Export works (Prometheus, JSON)
- [ ] CLI metrics commands work
- [ ] Integration tests pass with real autonomous workflows
- [ ] E2E test: metrics collection during autonomous workflow

**Status**: Not Started

---

### Checkpoint 3: Human Override Controls Complete

**Target Date**: Week 5
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] All override types work (pause, stop, modify)
- [ ] Pause and resume functionality works
- [ ] Scope visibility is maintained during override
- [ ] Override reasons are captured
- [ ] Override logging works
- [ ] CLI override commands work
- [ ] UI override controls work
- [ ] E2E test: override autonomous workflow at multiple points

**Status**: Not Started

---

### Checkpoint 4: Intervention Tracking Working

**Target Date**: Week 6
**Verification Layers**: 1, 2
**Sign-Off Criteria**:
- [ ] All intervention types are captured
- [ ] Intervention context is captured correctly
- [ ] Intervention reasons are tracked
- [ ] Logging works (structured logs)
- [ ] Query interface works
- [ ] Analysis produces correct trends
- [ ] CLI intervention commands work
- [ ] Integration tests pass with interventions

**Status**: Not Started

---

### Checkpoint 5: Dashboards Functional

**Target Date**: Week 7
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Dashboard layout renders correctly
- [ ] Success metrics are visualized
- [ ] Regression detection works
- [ ] Real-time data streaming works
- [ ] Trend analysis charts work
- [ ] Anomaly detection and alerting works
- [ ] Drill-down capabilities work
- [ ] E2E test: complete dashboard workflow

**Status**: Not Started

---

### Checkpoint 6: Stop Conditions Enforced

**Target Date**: Week 8
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] All stop condition types work
- [ ] Stop conditions are evaluated correctly
- [ ] Time-based stop conditions trigger
- [ ] Iteration-based stop conditions trigger
- [ ] Quality-based stop conditions trigger
- [ ] Intervention-based stop conditions trigger
- [ ] Resource-based stop conditions trigger
- [ ] Property tests verify stop condition determinism
- [ ] Integration tests pass with stop conditions

**Status**: Not Started

---

### Checkpoint 7: Checkpoints Generated and Restored

**Target Date**: Week 9
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] Checkpoints are generated periodically
- [ ] Event-based checkpoints work
- [ ] Checkpoint storage works
- [ ] Checkpoint versioning works
- [ ] Restoration works correctly
- [ ] Integrity check passes
- [ ] CLI checkpoint commands work
- [ ] Property tests verify restoration correctness
- [ ] Integration tests pass with checkpoints

**Status**: Not Started

---

### Checkpoint 8: Risk Assessment and Confidence Thresholds Working

**Target Date**: Week 10
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] Risk assessment computes correctly
- [ ] Scope boundaries are enforced
- [ ] Risk-based autonomy adjustment works
- [ ] Risk alerting works
- [ ] Confidence thresholds are computed
- [ ] Threshold enforcement blocks unsafe actions
- [ ] Threshold tuning works
- [ ] CLI scope and threshold commands work
- [ ] Property tests verify risk bounds and threshold enforcement
- [ ] Integration tests pass with risk and thresholds

**Status**: Not Started

---

### Checkpoint 9: Full Autonomy System Integration Complete

**Target Date**: Week 12
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] All autonomy features work together
- [ ] Contracts execute with bounded goals
- [ ] Metrics drive dashboards
- [ ] Human override works with full context
- [ ] Interventions are tracked and analyzed
- [ ] Dashboards provide visibility
- [ ] Stop conditions prevent runaway
- [ ] Risk assessments are accurate
- [ ] Confidence thresholds prevent unsafe decisions
- [ ] E2E test: complete autonomous workflow with all features

**Status**: Not Started

---

## Progress Tracking

### Overall Progress

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Phases Complete | 9 | 0 | 0% |
| Verification Layers | 4 | 0 | 0% |
| Checkpoints Passed | 9 | 0 | 0% |
| Web Research Areas | 5 | 0 | 0% |

**Overall Status**: Not Started

---

### Phase Progress

| Phase | Description | Status | Completion |
|-------|-------------|--------|------------|
| 1 | Autonomous Loop Contract System | Not Started | 0% |
| 2 | Metrics Collection and Aggregation | Not Started | 0% |
| 3 | Human Override Controls | Not Started | 0% |
| 4 | Intervention Tracking and Logging | Not Started | 0% |
| 5 | Success and Regression Dashboards | Not Started | 0% |
| 6 | Stop Condition Evaluation | Not Started | 0% |
| 7 | Checkpoint Generation and Restoration | Not Started | 0% |
| 8 | Autonomy Scope and Risk Assessment | Not Started | 0% |
| 9 | Confidence and Validation Thresholds | Not Started | 0% |

---

### Verification Layer Progress

| Layer | Description | Tests Written | Tests Passing | Coverage |
|-------|-------------|----------------|---------------|----------|
| 1 | Unit Tests | 0 | 0 | 0% |
| 2 | Integration Tests | 0 | 0 | 0% |
| 3 | Property-Based Tests | 0 | 0 | 0% |
| 4 | End-to-End Tests | 0 | 0 | 0% |

---

### Research Progress

| Research Area | Status | Evidence Collected | Synthesized |
|--------------|--------|-------------------|-------------|
| Autonomous Loop Contract Models | Not Started | 0 | No |
| Metrics Frameworks for Autonomous Systems | Not Started | 0 | No |
| Human Override and Intervention Logging | Not Started | 0 | No |
| Success and Regression Dashboards | Not Started | 0 | No |
| Confidence and Validation Thresholds | Not Started | 0 | No |

---

## Dependencies

### Blocks

- plan-00 (Foundation): Needed for WorkflowIR and storage
- plan-01 (MVP Queue): Needed for autonomous workflow execution
- plan-02 (CLI & Backends): Needed for CLI autonomy commands
- plan-03 (Glyphnova UI): Needed for UI dashboards
- plan-04 (Quality Loops): Needed for quality metrics and benchmark integration
- plan-05 (Memory & Search): Needed for intervention logging and artifact tracking
- plan-06 (Automation): Needed for cron scheduling and git automation

### Unblocks

None (final plan)

### Integration Points

- **plan-00 (Foundation)**: Uses WorkflowIR, storage layer, observability
- **plan-01 (MVP Queue)**: Executes autonomous workflows
- **plan-02 (CLI & Backends)**: CLI autonomy commands
- **plan-03 (Glyphnova UI)**: Autonomy dashboards, override controls
- **plan-04 (Quality Loops)**: Quality metrics, benchmark integration
- **plan-05 (Memory & Search)**: Intervention logging, artifact tracking
- **plan-06 (Automation)**: Cron scheduling, git automation

---

## Quality Gates

### ADR-0008 Quality Gates

1. **Autonomous Loop Boundedness**: Autonomous loops respect bounded goals and stop conditions
2. **Metrics Completeness**: Metrics collection captures all relevant performance and quality signals
3. **Human Override Availability**: Human override and pause controls are always functional
4. **Dashboard Visibility**: Dashboards provide real-time visibility into autonomous operations
5. **Observability Integration**: Observability integrates with `.opencode/` artifact system
6. **Validation Threshold Safety**: Validation thresholds prevent unsafe autonomous expansion

### Critical Review Upstream Factors

1. **Contract Enforcement Correctness**: No autonomous action violates contract bounds
2. **Metrics Accuracy**: Metrics accurately reflect autonomous system behavior (no gaps, no errors)
3. **Override Responsiveness**: Human override takes effect within 100ms
4. **Intervention Completeness**: All interventions are captured with full context
5. **Stop Condition Reliability**: Stop conditions always trigger before unsafe state
6. **Checkpoint Integrity**: Checkpoints always restore exact state (verified with hashes)
7. **Risk Assessment Accuracy**: Risk scores correlate with actual outcomes (high risk = high failure rate)
8. **Threshold Enforcement**: No autonomous decision succeeds below confidence threshold

---

## Next Steps

### Workflow for Execution

1. **Code Review**: Read ADR-0008 and related research plan findings
2. **Web Research**: Complete 5 research areas with evidence collection
3. **Update Plan**: Incorporate research findings into implementation plan
4. **Write Tests**: Start with unit tests for Phase 1 (Autonomous Loop Contract System)
5. **Implement Phase 1**: Build contract system with test-driven development
6. **Verify Checkpoint 1**: Run Layer 1, 2, 3 tests, update plan with results
7. **Continue Phases**: Repeat test-first approach for phases 2-9
8. **All Checkpoints**: Verify each checkpoint with all applicable layers
9. **Update Plan**: Document all working code and verification results
10. **Complete**: Mark plan complete (all 8 plans done!)

### Starting Point

Begin with Phase 1 (Autonomous Loop Contract System):
1. Define autonomous loop contract structure
2. Implement contract parser and validator
3. Write unit tests for contract parsing
4. Write property tests for contract validity
5. Implement bounded execution enforcer
6. Verify Checkpoint 1

---

## Execution Commands

### Verify All Tests

```bash
# Run all test layers
cargo test --lib              # Layer 1: Unit tests
cargo test --test '*'           # Layer 2: Integration tests
cargo test --test '*proptest*'  # Layer 3: Property-based tests
cargo test --test '*e2e*'       # Layer 4: End-to-end tests
```

### Verify Single Phase

```bash
# Verify Phase 1 (Autonomous Loop Contract System)
cargo test --lib autonomy_contract
cargo test --test autonomy_contract_integration
cargo test --test autonomy_contract_proptest

# Verify Phase 2 (Metrics Collection and Aggregation)
cargo test --lib metrics
cargo test --test metrics_integration
cargo test --test metrics_proptest
cargo test --test metrics_e2e
```

### Run Autonomy Operations

```bash
# Define autonomous loop contract
whitt-execution-engine autonomy --define --contract contract.yml

# Validate autonomous loop contract
whitt-execution-engine autonomy --validate --contract contract.yml

# Activate autonomous workflow
whitt-execution-engine autonomy --activate --workflow workflow.yml --contract contract.yml

# Override autonomous workflow (pause)
whitt-execution-engine override --pause --workflow <workflow-id> --reason "Manual review needed"

# Override autonomous workflow (stop)
whitt-execution-engine override --stop --workflow <workflow-id> --reason "Critical safety concern"

# Resume paused workflow
whitt-execution-engine override --resume --workflow <workflow-id>

# View interventions
whitt-execution-engine interventions --list --workflow <workflow-id>

# View metrics
whitt-execution-engine metrics --workflow <workflow-id> --export prometheus

# List checkpoints
whitt-execution-engine checkpoints --list --workflow <workflow-id>

# Restore checkpoint
whitt-execution-engine checkpoints --restore <checkpoint-id>

# Assess autonomy risk
whitt-execution-engine autonomy --assess --workflow <workflow-id>

# Set autonomy scope boundaries
whitt-execution-engine autonomy --set-boundaries --max-duration 1h --max-iterations 100

# View dashboard
# (Access via Glyphnova UI)
```

### Use implement.sh Script

```bash
# Start from beginning
./implement.sh start plan-07

# Resume from checkpoint
./implement.sh resume plan-07 cp5

# Check progress
./implement.sh status plan-07

# Generate progress report
./implement.sh report plan-07
```

---

## References

### Related Documents

- **ADR-0008**: [Autonomous loops and metrics-driven FE and BE UX expansion](../roadmap/adr-0008-autonomy-and-metrics.yml)
- **Research Plan 05**: [Automation, autonomy, and metrics research](../roadmap/research-plan-05-automation-autonomy-metrics.yml)
- **Plan 00**: [Foundation Compiler Contract](phase-0-foundation/plan-00-foundation.md)
- **Plan 01**: [MVP Queue & Scheduler](phase-1-mvp-queue/plan-01-mvp-queue.md)
- **Plan 02**: [CLI & Backends](phase-2-cli-backends/plan-02-cli-backends.md)
- **Plan 03**: [Glyphnova UI](phase-3-glyphnova-ui/plan-03-glyphnova-ui.md)
- **Plan 04**: [Quality Loops](phase-4-quality-loops/plan-04-quality-loops.md)
- **Plan 05**: [Memory & Search](phase-5-memory-search/plan-05-memory-search.md)
- **Plan 06**: [Automation](phase-6-automation/plan-06-automation.md)
- **Requirements**: [schema-consolidated-report.md](../requirements/schema-consolidated-report.md)

### Implementation References

- **OpenTelemetry**: [opentelemetry.io](https://opentelemetry.io) - Observability framework
- **Prometheus**: [prometheus.io](https://prometheus.io) - Metrics collection
- **Grafana**: [grafana.com](https://grafana.com) - Visualization
- **MLflow**: [mlflow.org](https://mlflow.org) - Experiment tracking
- **NIST Audit Logging**: [nvlpubs.nist.gov](https://nvlpubs.nist.gov) - Audit standards
- **LangChain**: [python.langchain.com/](https://python.langchain.com/) - Autonomous workflow patterns
- **D3.js**: [d3js.org](https://d3js.org) - Data visualization

### Tool and Plugin References

- **OpenCode Tools**: File operations, grep search, web browsing, shell execution
- **bash tool**: Command execution for autonomy commands
- **lsp_diagnostics**: Type checking and lint verification
- **webapp-testing**: UI testing for dashboards
- **glob tool**: File pattern matching for contract discovery

---

**Last Updated**: 2026-03-27
**Status**: Not Started
