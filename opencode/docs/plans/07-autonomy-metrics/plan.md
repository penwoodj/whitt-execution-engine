# Phase 7: Autonomy & Metrics Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build bounded autonomous execution with comprehensive metrics, human override controls, and success regression detection for the AgentSDK Execution Engine.

**Architecture:** Contract-based autonomous loops with three autonomy levels (low/medium/high), integrated metrics collection into the existing tracing infrastructure, human-override controls that are always available regardless of autonomy level, and stop conditions that enforce bounded execution. All observability is integrated into the runtime, not bolted on.

**Tech Stack:** Rust (existing AgentSDK codebase), tokio for async runtime, tracing for observability (already in use), prometheus for metrics export (or JSON for development), serde for serialization, chrono for timestamps, clap for CLI commands.

---

## Table of Contents

- [Overview](#overview)
- [ADR-0008 Compliance](#adr-0008-compliance)
- [Dependencies](#dependencies)
- [Estimated Timeline](#estimated-timeline)
- [File Structure](#file-structure)
- [Key Constraints](#key-constraints)
- [Task Breakdown](#task-breakdown)
- [Validation & Testing Strategy](#validation--testing-strategy)

---

## Overview

Phase 7 implements autonomous execution capabilities for the AgentSDK Execution Engine. This phase is the most complex in the entire roadmap because it integrates ALL prior systems (Phases 0-6) and introduces safety-critical autonomous behavior.

### What This Builds

1. **Autonomous Loop Contracts**: Typed contracts that define bounded goals, stop conditions, and autonomy levels
2. **Metrics Collection**: Integrated instrumentation for counters, gauges, histograms, and summaries
3. **Human Override Controls**: Pause, stop, modify, and scope-change capabilities that are ALWAYS available
4. **Intervention Tracking**: Structured logging of all human interventions with context and reason tracking
5. **Success Regression Dashboards**: Real-time visualization of success metrics with anomaly detection
6. **Stop Condition Evaluation**: Time, iteration, quality, intervention, and resource-based stop conditions
7. **Checkpoint Generation**: Periodic and event-triggered checkpointing for state preservation
8. **Autonomy Scope & Risk**: Boundary enforcement and risk assessment models
9. **Confidence Thresholds**: Computed from metrics with configurable thresholds
10. **CLI/UI Integration**: Commands for autonomy control and metrics viewing

### Autonomy Levels

- **Low**: Every action requires human confirmation
- **Medium**: Batch confirmation for groups of actions (configurable batch size)
- **High**: Bounded autonomous execution with periodic checkpoints and human-override available

### Critical Design Principle

**Human override is ALWAYS available at ALL autonomy levels.** There is no mode where the system operates without human oversight and intervention capability.

---

## ADR-0008 Compliance

This implementation strictly follows ADR-0008: Autonomous Execution Safety. Key requirements:

### 1. Bounded Autonomous Loops

All autonomous loops MUST:
- Have explicit contracts defining goals and stop conditions
- Enforce time-based limits (maximum execution time)
- Enforce iteration-based limits (maximum number of actions)
- Have quality gates that can trigger early termination
- Have intervention-based triggers (stop on human intervention)
- Have resource limits (memory, compute, budget)

**Implementation**: Task 00 (autonomous-loop-contracts) and Task 05 (stop-condition-evaluation)

### 2. Human Override Always Available

Human override controls MUST:
- Be available at all autonomy levels (low/medium/high)
- Support pause/resume/stop/modify operations
- Support scope-change operations (reduce/increase autonomy level)
- Be non-blocking and responsive (emergency stop within 100ms)
- Provide clear feedback to the operator

**Implementation**: Task 02 (human-override-controls) and Task 09 (autonomy-cli-ui)

### 3. Metrics Instrumented Before UX Experimentation

Metrics MUST:
- Be instrumented BEFORE any UI/UX experimentation begins
- Include counters for all critical events (actions taken, interventions, errors)
- Include gauges for current state (queue size, active workflows, resource usage)
- Include histograms for duration metrics (action duration, intervention response time)
- Include summaries for success rates (workflow success, intervention effectiveness)

**Implementation**: Task 01 (metrics-collection) - MUST COMPLETED FIRST

### 4. Observability Integrated Into Runtime

Observability MUST:
- Be integrated into the core runtime, not bolted on as an add-on
- Use the existing tracing infrastructure (tokio::tracing)
- Emit structured logs with correlation IDs
- Support multiple export formats (Prometheus, JSON)
- Be performant (no more than 5% overhead)

**Implementation**: Task 01 (metrics-collection) - integrates with existing tracing

### 5. Success Regression Detection

The system MUST:
- Detect regressions in success metrics (e.g., success rate dropping 10%)
- Alert operators in real-time
- Provide drill-down into the root cause
- Suggest corrective actions

**Implementation**: Task 04 (success-regression-dashboards)

---

## Dependencies

This phase DEPENDS on ALL prior phases being COMPLETE:

### Must Be Complete Before Starting Phase 7

- **Phase 0: Project Setup** - Repository structure, build system, basic CLI
- **Phase 1: Core Execution Engine** - Workflow execution, task scheduling, resource management
- **Phase 2: Tracing Infrastructure** - Distributed tracing, span collection, span export
- **Phase 3: Storage Layer** - Checkpoint storage, state persistence
- **Phase 4: Strategy Plugins** - Strategy interface, plugin loading, strategy execution
- **Phase 5: Event Bus** - Event publication/subscription, event filtering
- **Phase 6: CLI/UI Integration** - Command structure, interactive mode, progress display

### Critical Dependencies

1. **Core Execution Engine** - Autonomous loops use the same workflow execution engine
2. **Tracing Infrastructure** - Metrics collection integrates with existing spans
3. **Storage Layer** - Checkpoints are stored using the existing storage system
4. **Event Bus** - Intervention events and metrics events are published via event bus
5. **CLI/UI Integration** - Override controls are exposed through the existing CLI

**Verification**: Before starting Phase 7, verify that all prior phases pass their integration tests:

```bash
cd /home/jon/code/yaml-to-rust-agentsdk
cargo test --all-phases  # Assuming test organization by phase
```

---

## Estimated Timeline

**Total Estimated Time: 10-12 weeks**

### Week 1-2: Foundation (Tasks 00-01)
- Task 00: Autonomous Loop Contracts (5 days)
- Task 01: Metrics Collection (5 days)

### Week 3-4: Human Oversight (Tasks 02-03)
- Task 02: Human Override Controls (5 days)
- Task 03: Intervention Tracking (5 days)

### Week 5-6: Safety & Boundaries (Tasks 05-07)
- Task 05: Stop Condition Evaluation (5 days)
- Task 06: Checkpoint Generation (5 days)
- Task 07: Autonomy Scope & Risk (5 days)

### Week 7-8: Intelligence (Tasks 04, 08)
- Task 04: Success Regression Dashboards (5 days)
- Task 08: Confidence Thresholds (5 days)

### Week 9-10: Integration (Task 09)
- Task 09: Autonomy CLI/UI Integration (5 days)
- Integration Testing (5 days)

### Week 11-12: Validation & Polish
- Validation Suite (3 days)
- End-to-End Testing (3 days)
- Documentation (2 days)
- Buffer/Mitigation (2 days)

---

## File Structure

### Files to Create

```
glyphnova-engine/
├── src/
│   ├── autonomy/
│   │   ├── mod.rs
│   │   ├── contracts.rs          # Task 00
│   │   ├── contracts/parser.rs   # Task 00
│   │   ├── contracts/validator.rs # Task 00
│   │   ├── contracts/enforcer.rs  # Task 00
│   │   ├── contracts/goal.rs     # Task 00
│   │   └── contracts/level.rs    # Task 00
│   ├── metrics/
│   │   ├── mod.rs
│   │   ├── types.rs              # Task 01
│   │   ├── instrumentation.rs    # Task 01
│   │   ├── aggregation.rs        # Task 01
│   │   ├── storage.rs           # Task 01
│   │   └── export.rs            # Task 01
│   ├── override/
│   │   ├── mod.rs
│   │   ├── controls.rs          # Task 02
│   │   ├── handlers.rs          # Task 02
│   │   ├── pause.rs             # Task 02
│   │   └── emergency.rs         # Task 02
│   ├── intervention/
│   │   ├── mod.rs
│   │   ├── events.rs            # Task 03
│   │   ├── context.rs           # Task 03
│   │   ├── reason.rs            # Task 03
│   │   ├── logging.rs           # Task 03
│   │   └── analysis.rs          # Task 03
│   ├── dashboard/
│   │   ├── mod.rs
│   │   ├── layout.rs            # Task 04
│   │   ├── success_metrics.rs   # Task 04
│   │   ├── regression.rs       # Task 04
│   │   ├── streaming.rs         # Task 04
│   │   └── anomaly.rs           # Task 04
│   ├── stop_conditions/
│   │   ├── mod.rs
│   │   ├── types.rs             # Task 05
│   │   ├── parser.rs            # Task 05
│   │   ├── validator.rs         # Task 05
│   │   └── evaluation.rs        # Task 05
│   ├── checkpoint/
│   │   ├── mod.rs
│   │   ├── structure.rs         # Task 06
│   │   ├── periodic.rs          # Task 06
│   │   ├── triggered.rs         # Task 06
│   │   └── restoration.rs       # Task 06
│   ├── risk/
│   │   ├── mod.rs
│   │   ├── scope.rs             # Task 07
│   │   ├── assessment.rs       # Task 07
│   │   ├── enforcement.rs       # Task 07
│   │   └── alerting.rs          # Task 07
│   ├── confidence/
│   │   ├── mod.rs
│   │   ├── threshold.rs         # Task 08
│   │   ├── computation.rs       # Task 08
│   │   ├── enforcement.rs       # Task 08
│   │   └── tuning.rs            # Task 08
│   └── cli/
│       ├── autonomy.rs          # Task 09
│       └── metrics.rs           # Task 09
├── tests/
│   ├── autonomy/
│   │   ├── contracts_test.rs
│   │   ├── override_test.rs
│   │   └── intervention_test.rs
│   └── integration/
│       └── autonomy_integration_test.rs
├── adr/
│   └── ADR-0008-autonomous-execution-safety.md
└── docs/
    └── autonomy.md
```

### Files to Modify

```
glyphnova-engine/
├── src/
│   ├── lib.rs                   # Expose autonomy module
│   ├── execution/
│   │   ├── engine.rs           # Integrate with autonomous loops
│   │   └── workflow.rs         # Add autonomy context
│   └── tracing/
│       └── span.rs              # Add metrics instrumentation
└── Cargo.toml                  # Add dependencies
```

---

## Key Constraints

### 1. Bounded Execution

**Constraint**: No autonomous loop may run forever. Every loop MUST have:

- Time limit (e.g., max 24 hours)
- Iteration limit (e.g., max 1000 actions)
- Quality gate (e.g., stop if success rate < 80%)
- Resource limit (e.g., max $100 budget)

**Enforcement**: The `BoundedExecutionEnforcer` in Task 00 will enforce these limits and terminate the loop if any are violated.

### 2. Human Override Always Available

**Constraint**: Human override controls MUST be available at ALL autonomy levels, including HIGH autonomy.

**Implementation**: The override control system (Task 02) runs as a separate tokio task that listens for override commands. It has higher priority than the autonomous loop and can interrupt it at any time.

### 3. Metrics Before UX

**Constraint**: Metrics collection MUST be complete and validated BEFORE any UI/UX experimentation begins.

**Rationale**: Without metrics, you cannot determine whether UX changes improve or degrade the user experience.

**Verification**: The metrics collection system (Task 01) MUST have 100% test coverage and pass all validation tests before proceeding to Task 04 (dashboards).

### 4. Observability Integrated, Not Bolted On

**Constraint**: Observability MUST be integrated into the runtime, not added as an afterthought.

**Implementation**: Metrics collection uses the existing tracing infrastructure. Every span emitted by the workflow engine includes metric instrumentation. No separate metrics collection thread is spawned.

### 5. No Silent Failures

**Constraint**: All interventions, stop condition violations, and metric anomalies MUST be logged with full context.

**Implementation**: The intervention tracking system (Task 03) captures full context (workflow ID, action ID, state, metrics snapshot) for every intervention.

### 6. Performance Overhead < 5%

**Constraint**: Metrics collection and observability MUST NOT add more than 5% performance overhead.

**Verification**: Benchmark tests MUST show < 5% overhead for all instrumentation.

---

## Task Breakdown

### Task 0: Autonomous Loop Contracts

**File**: `tasks/00-autonomous-loop-contracts.md`

**Goal**: Define typed contracts for autonomous loops that specify goals, stop conditions, and autonomy levels.

**Key Deliverables**:
- `AutonomousLoopContract` struct with goal definition
- `ContractParser` for parsing contract YAML
- `ContractValidator` for validating contract constraints
- `BoundedExecutionEnforcer` for enforcing stop conditions
- `GoalTracker` for tracking progress toward goals
- `AutonomyLevel` enum (Low, Medium, High)

**Estimated Time**: 5 days

---

### Task 1: Metrics Collection

**File**: `tasks/01-metrics-collection.md`

**Goal**: Implement comprehensive metrics collection integrated into the tracing infrastructure.

**Key Deliverables**:
- Metric types: `Counter`, `Gauge`, `Histogram`, `Summary`
- Instrumentation hooks for workflow events
- Aggregation logic (sum, avg, p50, p95, p99)
- In-memory storage with TTL
- Export to Prometheus and JSON formats

**Estimated Time**: 5 days

**Critical Path**: MUST BE COMPLETED FIRST before any other task can begin.

---

### Task 2: Human Override Controls

**File**: `tasks/02-human-override-controls.md`

**Goal**: Implement pause, stop, modify, and scope-change controls that are always available.

**Key Deliverables**:
- `OverrideType` enum (Pause, Stop, Modify, ScopeChange)
- `OverrideEvent` for override commands
- `OverrideHandler` for processing overrides
- `PauseManager` for pause/resume state
- `EmergencyStop` for immediate termination

**Estimated Time**: 5 days

---

### Task 3: Intervention Tracking

**File**: `tasks/03-intervention-tracking.md`

**Goal**: Track all human interventions with full context and reason tracking.

**Key Deliverables**:
- `InterventionEvent` structure
- `InterventionContext` for capturing state
- `InterventionReason` for reason tracking
- Structured logging to `.glyphnova/interventions/`
- Analysis logic (trends, frequency)

**Estimated Time**: 5 days

---

### Task 4: Success Regression Dashboards

**File**: `tasks/04-success-regression-dashboards.md`

**Goal**: Build real-time dashboards for success metrics with regression detection.

**Key Deliverables**:
- Dashboard layout (success rate, intervention rate, action duration)
- Success metrics visualization
- Regression detection algorithm (e.g., EWMA control chart)
- Real-time streaming updates
- Anomaly detection alerts

**Estimated Time**: 5 days

**Dependencies**: Task 1 (metrics collection) MUST be complete.

---

### Task 5: Stop Condition Evaluation

**File**: `tasks/05-stop-condition-evaluation.md`

**Goal**: Evaluate stop conditions during autonomous loop execution.

**Key Deliverables**:
- `StopCondition` types: Time, Iteration, Quality, Intervention, Resource
- `StopConditionParser` for parsing conditions
- `StopConditionValidator` for validation
- `StopConditionEvaluator` for runtime evaluation
- Evaluation loop integrated into autonomous executor

**Estimated Time**: 5 days

---

### Task 6: Checkpoint Generation

**File**: `tasks/06-checkpoint-generation.md`

**Goal**: Generate periodic and event-triggered checkpoints for state preservation.

**Key Deliverables**:
- `Checkpoint` structure (workflow state, metrics, intervention history)
- Periodic checkpoint generation (configurable interval)
- Event-triggered checkpoints (on intervention, on stop condition)
- Storage in `.glyphnova/checkpoints/`
- Restoration logic

**Estimated Time**: 5 days

---

### Task 7: Autonomy Scope & Risk

**File**: `tasks/07-autonomy-scope-risk.md`

**Goal**: Define scope boundaries and implement risk assessment models.

**Key Deliverables**:
- `ScopeBoundary` definition
- `RiskAssessmentModel` (probability, impact, severity)
- `ScopeEnforcer` for boundary enforcement
- `RiskAlerting` for high-risk actions

**Estimated Time**: 5 days

---

### Task 8: Confidence Thresholds

**File**: `tasks/08-confidence-thresholds.md`

**Goal**: Compute confidence from metrics and enforce threshold-based decisions.

**Key Deliverables**:
- `ConfidenceThreshold` structure
- `ConfidenceComputation` from metrics
- `ConfidenceEnforcer` for threshold enforcement
- `ConfidenceTuning` for adjusting thresholds
- Logging of confidence decisions

**Estimated Time**: 5 days

---

### Task 9: Autonomy CLI/UI Integration

**File**: `tasks/09-autonomy-cli-ui.md`

**Goal**: Expose autonomy controls and metrics viewing through the CLI.

**Key Deliverables**:
- CLI commands: `glyphnova autonomy start`, `glyphnova autonomy pause`, `glyphnova autonomy stop`
- CLI commands: `glyphnova metrics view`, `glyphnova metrics export`
- UI integration for displaying autonomy status
- Interactive mode for override controls

**Estimated Time**: 5 days

---

## Validation & Testing Strategy

### Validation Files

The `validation/` directory contains mock strategies and test fixtures:

```
validation/
├── mock_autonomous_workflow.yaml  # Mock workflow for testing
├── mock_metrics_collector.yaml    # Mock metrics configuration
└── mock_override_events.yaml      # Mock override event scenarios
```

### Test Files

The `tests/` directory contains unit and integration tests:

```
tests/
├── autonomy/
│   ├── contracts_test.rs        # Contract parsing, validation, enforcement
│   ├── metrics_test.rs          # Metrics collection, aggregation, export
│   ├── override_test.rs         # Override controls, pause/resume
│   └── integration_test.rs      # End-to-end autonomy workflows
└── validation/
    ├── bounded_execution_test.rs  # Verify bounded execution enforcement
    └── human_override_test.rs     # Verify override always available
```

### Validation Criteria

1. **Bounded Execution**: All autonomous loops MUST terminate within defined limits
2. **Human Override**: Override controls MUST be responsive within 100ms
3. **Metrics Coverage**: All critical events MUST be instrumented
4. **Performance Overhead**: Metrics collection MUST NOT exceed 5% overhead
5. **No Silent Failures**: All interventions and violations MUST be logged

---

## Success Metrics

Phase 7 is considered complete when:

1. ✅ All 10 tasks are implemented and tested
2. ✅ All validation criteria pass
3. ✅ Integration tests for autonomy workflows pass
4. ✅ Performance overhead is verified to be < 5%
5. ✅ ADR-0008 compliance is verified
6. ✅ Documentation is complete

---

## Rollout Plan

### Week 1-2: Internal Testing
- Run unit tests for all tasks
- Run integration tests with mock strategies
- Verify performance benchmarks

### Week 3: Alpha Testing
- Deploy to a single test workflow
- Monitor metrics and interventions
- Tune confidence thresholds

### Week 4: Beta Testing
- Deploy to multiple test workflows
- Collect feedback on UX
- Fix any issues discovered

### Week 5: Production Rollout
- Gradual rollout with monitoring
- Continuous observation of success metrics
- Rollback plan ready if regressions detected

---

## Risk Mitigation

### Risk 1: Unbounded Autonomous Loops
**Mitigation**: Bounded execution enforcer with hard limits on time, iterations, and resources.

### Risk 2: Human Override Fails
**Mitigation**: Separate tokio task with higher priority, health checks every 100ms.

### Risk 3: Metrics Collection Overhead Too High
**Mitigation**: Aggressive testing and benchmarking, sampling for high-frequency metrics.

### Risk 4: Success Regression Not Detected
**Mitigation**: EWMA control charts with configurable sensitivity, real-time alerts.

### Risk 5: Checkpoint Corruption
**Mitigation**: Checkpoint validation on save/load, periodic integrity checks.

---

## Documentation Requirements

1. **ADR-0008**: Architectural Decision Record for autonomous execution safety
2. **API Documentation**: All public APIs must be documented
3. **User Guide**: How to use autonomous loops and override controls
4. **Operator Guide**: How to interpret metrics and dashboards
5. **Troubleshooting**: Common issues and how to resolve them

---

## Appendix A: ADR-0008 Reference

See `adr/ADR-0008-autonomous-execution-safety.md` for the full architectural decision record.

---

## Appendix B: Configuration Examples

### Autonomous Loop Contract Example

```yaml
version: "1.0"
autonomy_level: high
goal:
  type: "workflow_completion"
  target_workflow: "deploy_production"
  success_criteria:
    - type: "deployment_success"
      threshold: 0.95
stop_conditions:
  - type: "time"
    max_duration: "24h"
  - type: "iteration"
    max_actions: 1000
  - type: "quality"
    metric: "success_rate"
    threshold: 0.80
    operator: "below"
  - type: "intervention"
    stop_on_any: true
  - type: "resource"
    budget:
      max_cost_usd: 100
```

### Metrics Collection Configuration

```yaml
version: "1.0"
metrics:
  - name: "actions_taken"
    type: "counter"
    labels: ["workflow_id", "action_type"]
  - name: "active_workflows"
    type: "gauge"
  - name: "action_duration_ms"
    type: "histogram"
    buckets: [10, 50, 100, 500, 1000, 5000]
  - name: "success_rate"
    type: "summary"
    quantiles: [0.5, 0.95, 0.99]
export:
  prometheus:
    enabled: true
    port: 9090
  json:
    enabled: true
    path: ".glyphnova/metrics.json"
```

---

## Appendix C: Integration with Existing Code

### Tracing Integration

Metrics collection integrates with the existing tracing infrastructure:

```rust
use tracing::{span, Level};

let span = span!(Level::INFO, "workflow_execution", workflow_id = %id);
let _enter = span.enter();

// Metrics are automatically collected within the span
metrics::counter!("actions_taken", 1, "workflow_id" => id.to_string());
```

### Event Bus Integration

Intervention events are published via the event bus:

```rust
event_bus.publish(InterventionEvent {
    workflow_id,
    intervention_type: OverrideType::Stop,
    reason: "Manual termination by operator".to_string(),
    timestamp: Utc::now(),
    context: capture_intervention_context(),
});
```

### Storage Integration

Checkpoints are stored using the existing storage layer:

```rust
storage.save_checkpoint(&checkpoint_path, &checkpoint_data)?;
```

---

## Appendix D: Performance Benchmarks

Target performance metrics:

| Metric | Target | Measured |
|--------|--------|----------|
| Metrics collection overhead | < 5% | TBD |
| Override control response time | < 100ms | TBD |
| Checkpoint save time | < 1s | TBD |
| Stop condition evaluation | < 10ms | TBD |
| Dashboard update latency | < 100ms | TBD |

---

## Appendix E: Rollback Plan

If Phase 7 introduces issues:

1. Immediate rollback: Disable autonomous loops via configuration
2. Partial rollback: Revert to Low autonomy level (every action confirmed)
3. Data preservation: Keep all checkpoints and intervention logs for analysis
4. Hotfix: Apply patches without full rollback if possible

---

## Appendix F: Future Enhancements

Potential future enhancements (out of scope for Phase 7):

1. Reinforcement learning for adaptive autonomy levels
2. Predictive analytics for anticipating interventions
3. Multi-agent coordination with shared autonomy
4. Human-in-the-loop optimization of confidence thresholds
5. Autonomous self-healing (auto-tuning based on metrics)

---

**End of Phase 7 Implementation Plan**
