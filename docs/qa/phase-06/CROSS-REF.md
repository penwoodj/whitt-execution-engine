# Cross-References — Phase 06: Autonomy & Metrics

**Phase**: 06 - Autonomy & Metrics
**Schema Version**: 2.0.0
**Last Updated**: 2026-04-26

---

## Schema References

| QA Area | Schema Section | Line Range | Key Fields |
|----------|---------------|--------------|-------------|
| Autonomous Loop Contracts | N/A (new feature) | N/A | Bounded goals, stop conditions, autonomy levels |
| Metrics Collection | N/A (new feature) | N/A | Counters, gauges, histograms, summaries |
| Human Override Controls | N/A (new feature) | N/A | Pause, stop, modify, scope-change |
| Intervention Tracking | N/A (new feature) | N/A | Structured logging, full context |
| Success Regression Dashboards | N/A (new feature) | N/A | Real-time visualization, anomaly detection |
| Stop Condition Evaluation | Lines 503-598 (workflow_execution_strategy) | Stop conditions, early termination |
| Checkpoint Generation | Lines 568-583 (checkpointing) | Periodic, event-triggered, storage |
| Autonomy Scope & Risk | N/A (new feature) | N/A | Boundary enforcement, risk models |
| Confidence Thresholds | N/A (new feature) | N/A | Computed from metrics, threshold enforcement |
| Autonomy CLI/UI | N/A (new feature) | N/A | Autonomy controls, metrics viewing |

---

## Plan References

| QA Area | Plan File | Line Range | Key Tasks |
|----------|-----------|--------------|-------------|
| Autonomous Loop Contracts | [tasks/00-autonomous-loop-contracts.md](../plans/06-autonomy-metrics/tasks/00-autonomous-loop-contracts.md) | N/A | Contracts, parser, validator, enforcer, levels |
| Metrics Collection | [tasks/01-metrics-collection.md](../plans/06-autonomy-metrics/tasks/01-metrics-collection.md) | N/A | Types, instrumentation, aggregation, storage, export |
| Human Override Controls | [tasks/02-human-override-controls.md](../plans/06-autonomy-metrics/tasks/02-human-override-controls.md) | N/A | Controls, handlers, pause, emergency |
| Intervention Tracking | [tasks/03-intervention-tracking.md](../plans/06-autonomy-metrics/tasks/03-intervention-tracking.md) | N/A | Events, context, reason, logging, analysis |
| Success Regression Dashboards | [tasks/04-success-regression-dashboards.md](../plans/06-autonomy-metrics/tasks/04-success-regression-dashboards.md) | N/A | Layout, success metrics, regression, streaming, anomaly |
| Stop Condition Evaluation | [tasks/05-stop-condition-evaluation.md](../plans/06-autonomy-metrics/tasks/05-stop-condition-evaluation.md) | N/A | Types, parser, validator, evaluation |
| Checkpoint Generation | [tasks/06-checkpoint-generation.md](../plans/06-autonomy-metrics/tasks/06-checkpoint-generation.md) | N/A | Structure, periodic, triggered, restoration |
| Autonomy Scope & Risk | [tasks/07-autonomy-scope-risk.md](../plans/06-autonomy-metrics/tasks/07-autonomy-scope-risk.md) | N/A | Scope, assessment, enforcement, alerting |
| Confidence Thresholds | [tasks/08-confidence-thresholds.md](../plans/06-autonomy-metrics/tasks/08-confidence-thresholds.md) | N/A | Threshold, computation, enforcement, tuning |
| Autonomy CLI/UI Integration | [tasks/09-autonomy-cli-ui.md](../plans/06-autonomy-metrics/tasks/09-autonomy-cli-ui.md) | N/A | CLI autonomy, CLI metrics, UI components |

---

## Related QA Areas

| Related Phase | Related QA Area | Relationship |
|---------------|------------------|-------------|
| Phase 02 (CLI & LLM Backend) | CLI Implementation | Autonomy CLI extends CLI commands |
| Phase 03 (Quality Loops) | Validation | Success regression detected in quality loops |
| Phase 04 (Memory & Search) | Provenance | Interventions tracked in provenance |
| Phase 05 (Automation) | Experiments | Autonomous workflows can trigger automation |
| Phase 07 (Final Validation) | Integration Tests | Autonomy included in E2E tests |

---

## ADR References

| ADR | Title | Relevance |
|------|-------|-----------|
| ADR-0008 | Autonomous Execution Safety | Core ADR governing this phase |
| ADR-0006 | Memory & Search Architecture | Autonomous workflows use memory search |
| ADR-0007 | Automation Governance | Autonomy can trigger automation |

---

## External Documentation

| Document | Path | Purpose |
|----------|--------|---------|
| Unified Workflow Schema | docs/schema/unified-workflow-schema.yml | Schema reference (805 lines) |
| Validation Framework | docs/plans/validation-criteria/framework.md | 7-layer validation system |
| Extended POC QA | docs/qa/extended-poc/QA-AREAS-EXTENDED-POC.md | QA format reference |
| AGENTS.md | AGENTS.md | QA area format, verification protocol |

---

## Task File References

| Task ID | Task File | Description |
|----------|-----------|-------------|
| 00 | [tasks/00-autonomous-loop-contracts.md](../plans/06-autonomy-metrics/tasks/00-autonomous-loop-contracts.md) | Autonomous loop contracts and bounded execution |
| 01 | [tasks/01-metrics-collection.md](../plans/06-autonomy-metrics/tasks/01-metrics-collection.md) | Metrics collection integrated into tracing |
| 02 | [tasks/02-human-override-controls.md](../plans/06-autonomy-metrics/tasks/02-human-override-controls.md) | Human override controls always available |
| 03 | [tasks/03-intervention-tracking.md](../plans/06-autonomy-metrics/tasks/03-intervention-tracking.md) | Structured logging of interventions |
| 04 | [tasks/04-success-regression-dashboards.md](../plans/06-autonomy-metrics/tasks/04-success-regression-dashboards.md) | Real-time dashboards with regression detection |
| 05 | [tasks/05-stop-condition-evaluation.md](../plans/06-autonomy-metrics/tasks/05-stop-condition-evaluation.md) | Stop condition evaluation and enforcement |
| 06 | [tasks/06-checkpoint-generation.md](../plans/06-autonomy-metrics/tasks/06-checkpoint-generation.md) | Checkpoint generation (periodic and event-triggered) |
| 07 | [tasks/07-autonomy-scope-risk.md](../plans/06-autonomy-metrics/tasks/07-autonomy-scope-risk.md) | Autonomy scope and risk assessment |
| 08 | [tasks/08-confidence-thresholds.md](../plans/06-autonomy-metrics/tasks/08-confidence-thresholds.md) | Confidence thresholds computed from metrics |
| 09 | [tasks/09-autonomy-cli-ui.md](../plans/06-autonomy-metrics/tasks/09-autonomy-cli-ui.md) | Autonomy CLI commands and UI integration |

---

## Test File References

| Test File | Purpose |
|-----------|---------|
| tests/autonomy/contracts_test.rs | Contract parsing, validation, enforcement tests |
| tests/autonomy/metrics_test.rs | Metrics collection, aggregation, export tests |
| tests/autonomy/override_test.rs | Override controls, pause manager tests |
| tests/autonomy/intervention_test.rs | Intervention tracking, logging, analysis tests |
| tests/integration/autonomy_integration_test.rs | End-to-end autonomy workflow tests |

---

## Validation File References

| Validation File | Purpose |
|----------------|---------|
| [validation/00-autonomous-loop-contracts.md](../plans/06-autonomy-metrics/tasks/00-autonomous-loop-contracts.md) | Validation criteria for task 00 |
| [validation/01-metrics-collection.md](../plans/06-autonomy-metrics/tasks/01-metrics-collection.md) | Validation criteria for task 01 |
| [validation/02-human-override-controls.md](../plans/06-autonomy-metrics/tasks/02-human-override-controls.md) | Validation criteria for task 02 |
| [validation/03-intervention-tracking.md](../plans/06-autonomy-metrics/tasks/03-intervention-tracking.md) | Validation criteria for task 03 |
| [validation/04-success-regression-dashboards.md](../plans/06-autonomy-metrics/tasks/04-success-regression-dashboards.md) | Validation criteria for task 04 |
| [validation/05-stop-condition-evaluation.md](../plans/06-autonomy-metrics/tasks/05-stop-condition-evaluation.md) | Validation criteria for task 05 |
| [validation/06-checkpoint-generation.md](../plans/06-autonomy-metrics/tasks/06-checkpoint-generation.md) | Validation criteria for task 06 |
| [validation/07-autonomy-scope-risk.md](../plans/06-autonomy-metrics/tasks/07-autonomy-scope-risk.md) | Validation criteria for task 07 |
| [validation/08-confidence-thresholds.md](../plans/06-autonomy-metrics/tasks/08-confidence-thresholds.md) | Validation criteria for task 08 |
| [validation/09-autonomy-cli-ui.md](../plans/06-autonomy-metrics/tasks/09-autonomy-cli-ui.md) | Validation criteria for task 09 |

---

## Implementation Checklist

- [ ] Task 00: Autonomous Loop Contracts implemented and passing tests
- [ ] Task 01: Metrics Collection implemented and passing tests (MUST BE COMPLETED FIRST)
- [ ] Task 02: Human Override Controls implemented and passing tests
- [ ] Task 03: Intervention Tracking implemented and passing tests
- [ ] Task 04: Success Regression Dashboards implemented and passing tests
- [ ] Task 05: Stop Condition Evaluation implemented and passing tests
- [ ] Task 06: Checkpoint Generation implemented and passing tests
- [ ] Task 07: Autonomy Scope & Risk implemented and passing tests
- [ ] Task 08: Confidence Thresholds implemented and passing tests
- [ ] Task 09: Autonomy CLI/UI Integration implemented and passing tests
- [ ] All unit tests passing (cargo test --lib)
- [ ] All integration tests passing (cargo test --test)
- [ ] ADR-0008 compliance verified
- [ ] Performance benchmarks meet targets (override < 100ms, metrics overhead < 5%)
- [ ] Documentation complete
- [ ] LSP diagnostics clean on all changed files
- [ ] Build passes (cargo build --release --all-features)

---

**End of Cross-References for Phase 06**
