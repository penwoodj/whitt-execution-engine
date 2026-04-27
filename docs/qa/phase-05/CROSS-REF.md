# Cross-References — Phase 05: Automation

**Phase**: 05 - Automation
**Schema Version**: 2.0.0
**Last Updated**: 2026-04-26

---

## Schema References

| QA Area | Schema Section | Line Range | Key Fields |
|----------|---------------|--------------|-------------|
| Automation CLI | N/A (new feature) | N/A | New CLI commands for automation |
| Automation UI | N/A (new feature) | N/A | React-based UI components |
| Cron Scheduler | N/A (new feature) | N/A | Cron expressions, tokio scheduler |
| Git Experiment | N/A (new feature) | N/A | Branch isolation, worktree management |
| Merge Proposals | N/A (new feature) | N/A | Diff generation, artifacts |
| Manual Refinement | N/A (new feature) | N/A | Event capture, immutable log |
| Result Tracking | N/A (new feature) | N/A | Experiment results, comparison |

---

## Plan References

| QA Area | Plan File | Line Range | Key Tasks |
|----------|-----------|--------------|-------------|
| Cron Scheduler | [tasks/00-cron-scheduler.md](../plans/05-automation/tasks/00-cron-scheduler.md) | N/A | Cron parser, scheduler, validation |
| Git Experiment Framework | [tasks/01-git-experiment-framework.md](../plans/05-automation/tasks/01-git-experiment-framework.md) | N/A | Branch isolation, worktree management |
| Merge Proposal Generation | [tasks/02-merge-proposal-generation.md](../plans/05-automation/tasks/02-merge-proposal-generation.md) | N/A | Diff generation, confidence scoring |
| Manual Refinement Capture | [tasks/03-manual-refinement-capture.md](../plans/05-automation/tasks/03-manual-refinement-capture.md) | N/A | Event capture, artifact linking |
| Experiment Result Tracking | [tasks/04-experiment-result-tracking.md](../plans/05-automation/tasks/04-experiment-result-tracking.md) | N/A | Result storage, comparison, visualization |
| Rollback & Cleanup | [tasks/05-rollback-cleanup.md](../plans/05-automation/tasks/05-rollback-cleanup.md) | N/A | Rollback procedures, cleanup operations |
| Scheduling Policy Compiler | [tasks/06-scheduling-policy-compiler.md](../plans/05-automation/tasks/06-scheduling-policy-compiler.md) | N/A | WorkflowIR compilation, cron compiler |
| Automation CLI | [tasks/07-automation-cli.md](../plans/05-automation/tasks/07-automation-cli.md) | N/A | CLI commands for all automation features |
| Automation UI Integration | [tasks/08-automation-ui-integration.md](../plans/05-automation/tasks/08-automation-ui-integration.md) | N/A | React UI, experiment tracking, proposals |

---

## Related QA Areas

| Related Phase | Related QA Area | Relationship |
|---------------|------------------|-------------|
| Phase 02 (CLI & LLM Backend) | CLI Implementation | Automation CLI extends CLI commands |
| Phase 03 (Quality Loops) | Validation | Experiment results validated in quality loops |
| Phase 04 (Memory & Search) | Provenance | Automation experiments tracked in provenance |
| Phase 06 (Autonomy & Metrics) | Metrics | Automation operations instrumented in metrics |
| Phase 07 (Final Validation) | Integration Tests | Automation included in E2E tests |

---

## ADR References

| ADR | Title | Relevance |
|------|-------|-----------|
| ADR-0007 | Automation Governance | Core ADR governing this phase |
| ADR-0006 | Memory & Search Architecture | Experiments use memory search |
| ADR-0008 | Autonomous Execution Safety | Automation can trigger autonomous workflows |

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
| 00 | [tasks/00-cron-scheduler.md](../plans/05-automation/tasks/00-cron-scheduler.md) | Cron scheduling infrastructure |
| 01 | [tasks/01-git-experiment-framework.md](../plans/05-automation/tasks/01-git-experiment-framework.md) | Git experiment isolation and worktree management |
| 02 | [tasks/02-merge-proposal-generation.md](../plans/05-automation/tasks/02-merge-proposal-generation.md) | Merge proposal generation and artifact management |
| 03 | [tasks/03-manual-refinement-capture.md](../plans/05-automation/tasks/03-manual-refinement-capture.md) | Manual refinement capture with immutable events |
| 04 | [tasks/04-experiment-result-tracking.md](../plans/05-automation/tasks/04-experiment-result-tracking.md) | Experiment result tracking and comparison |
| 05 | [tasks/05-rollback-cleanup.md](../plans/05-automation/tasks/05-rollback-cleanup.md) | Rollback and cleanup procedures |
| 06 | [tasks/06-scheduling-policy-compiler.md](../plans/05-automation/tasks/06-scheduling-policy-compiler.md) | Scheduling policy compilation to WorkflowIR |
| 07 | [tasks/07-automation-cli.md](../plans/05-automation/tasks/07-automation-cli.md) | Automation CLI commands |
| 08 | [tasks/08-automation-ui-integration.md](../plans/05-automation/tasks/08-automation-ui-integration.md) | React-based UI for automation |

---

## Test File References

| Test File | Purpose |
|-----------|---------|
| tests/integration/cron_scheduler_test.rs | Integration tests for cron scheduler |
| tests/integration/git_experiment_test.rs | Integration tests for git experiments |
| tests/integration/merge_proposal_test.rs | Integration tests for merge proposals |
| tests/integration/rollback_test.rs | Integration tests for rollback and cleanup |
| tests/mock/git_repo.rs | Mock git repository helper |
| tests/mock/cron_scheduler.rs | Mock cron scheduler helper |

---

## Validation File References

| Validation File | Purpose |
|----------------|---------|
| validation/00-cron-scheduler.md | Validation criteria for task 00 |
| validation/01-git-experiment-framework.md | Validation criteria for task 01 |
| validation/02-merge-proposal-generation.md | Validation criteria for task 02 |
| validation/03-manual-refinement-capture.md | Validation criteria for task 03 |
| validation/04-experiment-result-tracking.md | Validation criteria for task 04 |
| validation/05-rollback-cleanup.md | Validation criteria for task 05 |
| validation/06-scheduling-policy-compiler.md | Validation criteria for task 06 |
| validation/07-automation-cli.md | Validation criteria for task 07 |
| validation/08-automation-ui-integration.md | Validation criteria for task 08 |

---

## Implementation Checklist

- [ ] Task 00: Cron Scheduler implemented and passing tests
- [ ] Task 01: Git Experiment Framework implemented and passing tests
- [ ] Task 02: Merge Proposal Generation implemented and passing tests
- [ ] Task 03: Manual Refinement Capture implemented and passing tests
- [ ] Task 04: Experiment Result Tracking implemented and passing tests
- [ ] Task 05: Rollback & Cleanup implemented and passing tests
- [ ] Task 06: Scheduling Policy Compiler implemented and passing tests
- [ ] Task 07: Automation CLI implemented and passing tests
- [ ] Task 08: Automation UI Integration implemented and passing tests
- [ ] All unit tests passing (cargo test --lib)
- [ ] All integration tests passing (cargo test --test)
- [ ] ADR-0007 compliance verified
- [ ] Performance benchmarks meet targets
- [ ] Documentation complete
- [ ] LSP diagnostics clean on all changed files
- [ ] Build passes (cargo build --release --all-features)

---

**End of Cross-References for Phase 05**
