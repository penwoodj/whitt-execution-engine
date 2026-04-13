# Validation Criteria

Incremental validation framework preventing goal drift and ensuring systematic validation of AgentSDK Execution Engine implementation. Each phase must pass all 7 verification layers before completion.

## Framework Documents

- framework.md - 7-layer validation framework definition with evidence requirements and pass criteria
- anti-goal-drift-checklist.md - Drift detection mechanisms for requirements, architecture, scope, performance, testing, documentation, and dependencies
- cumulative-progress.md - Cumulative progress tracker across all phases

## Phase-Specific Criteria

- phase-00-criteria.md - Foundation phase measurable validation criteria
- phase-01-criteria.md - MVP Queue & Scheduler phase measurable validation criteria
- phase-02-criteria.md - CLI & Backends phase measurable validation criteria
- phase-03-criteria.md - Glyphnova UI phase measurable validation criteria
- phase-04-criteria.md - Quality Loops phase measurable validation criteria
- phase-05-criteria.md - Memory & Search phase measurable validation criteria
- phase-06-criteria.md - Automation phase measurable validation criteria
- phase-07-criteria.md - Autonomy & Metrics phase measurable validation criteria
- phase-08-criteria.md - Final Validation phase measurable validation criteria

## 7 Verification Layers

1. **Unit Tests** - Isolated component testing with >90% coverage
2. **Integration Tests** - Multi-component interaction validation
3. **Property Tests** - Invariants verified for 1000 iterations
4. **E2E Tests** - Complete workflow execution from YAML to output
5. **System Log Validation** - Structured logging, scope correctness, telemetry completeness
6. **Live CLI Verification** - User-facing command behavior and error handling
7. **Benchmark Performance** - Performance against baseline and resource utilization

## Related

- [Master Plan](../INDEX.md) - Phase definitions and execution order
- [Traceability Matrices](../traceability/) - Schema and requirement mapping to phases
- [Research Integration](../deep-research/00-research-master-plan.md) - Research-to-implementation pipeline
