# QA Criteria — Phase 05: Automation

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Phase Plan**: `docs/plans/05-automation/plan.md` (503 lines, 9 tasks)
**Date**: 2026-04-26
**Status**: 🟡 IN PROGRESS — Implementation not started

---

## Summary Table

| # | QA Area | Schema Ref | Status | Priority | Test Type |
|---|---------|------------|------------|
| 1 | Cron Scheduler (time-based scheduling) | N/A (new feature) | P0 | Unit, Integration |
| 2 | Git Experiment Framework (branch isolation) | N/A (new feature) | P0 | Unit, Integration |
| 3 | Merge Proposal Generation (diff generation) | N/A (new feature) | P1 | Unit, Integration |
| 4 | Manual Refinement Capture (approval/rejection events) | N/A (new feature) | P1 | Unit, Integration |
| 5 | Experiment Result Tracking (storage, comparison) | N/A (new feature) | P1 | Unit, Integration |
| 6 | Rollback & Cleanup (safe experiment rollback) | N/A (new feature) | P1 | Unit, Integration |
| 7 | Scheduling Policy Compiler (WorkflowIR compilation) | N/A (new feature) | P0 | Unit, Integration |
| 8 | Automation CLI (user-facing commands) | N/A (new feature) | P0 | Integration, E2E |
| 9 | Automation UI Integration (React-based UI) | N/A (new feature) | P2 | Integration, E2E |

---

## Area Details

### Area 1: Cron Scheduler

**Schema Ref**: N/A (new feature - not in schema yet, will be in workflow_execution_strategy)
**Plan Ref**: Task 00 (`docs/plans/05-automation/tasks/00-cron-scheduler.md`)
**Files**: `automation/cron/mod.rs`, `automation/cron/parser.rs`, `automation/cron/scheduler.rs`, `automation/cron/validator.rs`, `automation/cron/persistence.rs`

**Criteria**:
- Cron expression parsing: Parse standard cron expressions (minute, hour, day of month, month, day of week)
- Schedule validation: Validate cron expressions at load time
- Tokio-based scheduler: Async scheduler using tokio-cron-scheduler crate
- Schedule persistence: Persist schedules to `./workspace/schedules/`
- Schedule loading: Load schedules on startup
- Workflow execution: Execute workflows on schedule
- Safety limits: Enforce max concurrent experiments, resource limits
- Schedule management: Add, remove, list, pause, resume schedules
- Next run calculation: Calculate next run time accurately
- Missed schedules: Handle missed executions (system downtime)

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Unit tests for cron parser
cargo test --lib automation::cron::parser -- --test-threads=1

# Unit tests for scheduler
cargo test --lib automation::cron::scheduler -- --test-threads=1

# Integration tests
cargo test --test cron_scheduler_integration -- --test-threads=1 --nocapture
```

---

### Area 2: Git Experiment Framework

**Schema Ref**: N/A (new feature - infrastructure)
**Plan Ref**: Task 01 (`docs/plans/05-automation/tasks/01-git-experiment-framework.md`)
**Files**: `automation/experiment/mod.rs`, `automation/experiment/manifest.rs`, `automation/experiment/git_operations.rs`, `automation/experiment/merge_policy.rs`, `automation/experiment/isolation.rs`

**Criteria**:
- Branch isolation: All git experiments run in isolated branches, never on main (ADR-0007 constraint)
- Worktree management: Create isolated worktrees for experiments
- Branch creation: Create experiment branches from current HEAD
- Branch naming: Use consistent naming scheme (experiment-{timestamp})
- Manifest tracking: Track experiment manifest (id, branch, workflow, start_time)
- Isolation enforcement: Prevent modifications to main branch
- Experiment lifecycle: start → execute → complete → cleanup
- Worktree cleanup: Remove worktree after experiment completion
- Merge policies: Auto-merge/approval policies for experiment results

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Unit tests for git operations
cargo test --lib automation::experiment::git_operations -- --test-threads=1

# Unit tests for isolation
cargo test --lib automation::experiment::isolation -- --test-threads=1

# Integration tests with mock git repo
cargo test --test git_experiment_framework -- --test-threads=1 --nocapture

# Clippy check
cargo clippy --all-features -- -D warnings automation/experiment/
```

---

### Area 3: Merge Proposal Generation

**Schema Ref**: N/A (new feature - artifacts in `./workspace/merge-proposals/`)
**Plan Ref**: Task 02 (`docs/plans/05-automation/tasks/02-merge-proposal-generation.md`)
**Files**: `automation/merge/mod.rs`, `automation/merge/generator.rs`, `automation/merge/validator.rs`, `automation/merge/confidence.rs`, `automation/merge/artifacts.rs`

**Criteria**:
- Diff generation: Generate accurate diffs between experiment branch and main
- Artifact location: Write proposals to `./workspace/merge-proposals/` (ADR-0007 constraint)
- Proposal structure: proposal.json, diff.patch, metadata.json
- Diff format: Standard unified diff format
- Validation criteria: Validate merge proposals before writing
- Confidence scoring: Compute confidence based on test results, code changes
- Artifact linking: Link proposals to experiment manifests
- No auto-commit: Merge proposals are outputs, not auto-committed (ADR-0007 constraint)
- Conflict detection: Detect merge conflicts in proposals
- Metadata capture: Capture experiment_id, author, timestamp, change_summary

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests for diff generation
cargo test --lib automation::merge::generator -- --test-threads=1

# Unit tests for validator
cargo test --lib automation::merge::validator -- --test-threads=1

# Unit tests for confidence scoring
cargo test --lib automation::merge::confidence -- --test-threads=1

# Integration tests with mock git repo
cargo test --test merge_proposal_generation -- --test-threads=1 --nocapture
```

---

### Area 4: Manual Refinement Capture

**Schema Ref**: N/A (new feature - artifacts in `./workspace/refinements/`)
**Plan Ref**: Task 03 (`docs/plans/05-automation/tasks/03-manual-refinement-capture.md`)
**Files**: `automation/refinement/mod.rs`, `automation/refinement/events.rs`, `automation/refinement/capture.rs`, `automation/refinement/linking.rs`

**Criteria**:
- Event capture: Capture all manual refinements (approvals, rejections, modifications)
- Event structure: event.json with what, why, who, when, linked_artifacts
- Immutable event log: Event log in `./workspace/refinements/` cannot be modified
- Artifact linking: Link refinements to merge proposals and experiment results
- Refinement types: approval, rejection, modification, rollback
- Metadata capture: Capture user_id, timestamp, reason, context
- Approval workflow: CLI/UI commands for approving proposals
- Rejection workflow: CLI/UI commands for rejecting proposals
- Modification workflow: CLI/UI commands for modifying proposals
- Rollback support: Refinements can trigger rollback operations

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests for event capture
cargo test --lib automation::refinement::capture -- --test-threads=1

# Unit tests for linking
cargo test --lib automation::refinement::linking -- --test-threads=1

# Integration tests
cargo test --test manual_refinement_capture -- --test-threads=1 --nocapture
```

---

### Area 5: Experiment Result Tracking

**Schema Ref**: N/A (new feature - artifacts in `./workspace/experiments/`)
**Plan Ref**: Task 04 (`docs/plans/05-automation/tasks/04-experiment-result-tracking.md`)
**Files**: `automation/tracking/mod.rs`, `automation/tracking/storage.rs`, `automation/tracking/comparison.rs`, `automation/tracking/visualization.rs`, `automation/tracking/export.rs`

**Criteria**:
- Result storage: Store experiment results in `./workspace/experiments/<experiment-id>/`
- Result structure: manifest.json, results.json, artifacts/
- Result comparison: Compare experiment results with baseline
- Comparison metrics: success rate, execution time, resource usage, code quality
- Visualization helpers: Generate visualizations (charts, tables) for results
- Export functionality: Export results to CSV, JSON for analysis
- Baseline tracking: Track baseline results for comparison
- Trend analysis: Identify trends across multiple experiments
- Result validation: Validate results before storage
- Performance metrics: Capture performance metrics (latency, throughput, errors)

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests for storage
cargo test --lib automation::tracking::storage -- --test-threads=1

# Unit tests for comparison
cargo test --lib automation::tracking::comparison -- --test-threads=1

# Unit tests for visualization
cargo test --lib automation::tracking::visualization -- --test-threads=1

# Integration tests
cargo test --test experiment_result_tracking -- --test-threads=1 --nocapture
```

---

### Area 6: Rollback & Cleanup

**Schema Ref**: N/A (new feature - artifacts in `./workspace/rollbacks/`)
**Plan Ref**: Task 05 (`docs/plans/05-automation/tasks/05-rollback-cleanup.md`)
**Files**: `automation/rollback/mod.rs`, `automation/rollback/procedures.rs`, `automation/rollback/cleanup.rs`, `automation/rollback/verification.rs`

**Criteria**:
- Rollback procedures: Rollback to pre-experiment state safely
- Cleanup operations: Delete experiment artifacts, remove branches, clean worktrees
- Rollback verification: Post-rollback verification to confirm clean state
- Recovery mechanism: Restore deleted data if rollback enabled
- Branch deletion: Delete experiment branches after cleanup
- Worktree removal: Remove worktrees after experiment completion
- Cleanup logging: Log all cleanup operations with before/after state
- Artifact cleanup: Remove artifacts from `./workspace/merge-proposals/`, `./workspace/refinements/`
- Verification of clean state: Verify no artifacts remain after cleanup
- Failed experiment handling: Cleanup after failed experiments
- Safe rollback: Only rollback verified experiments

**Test Type**: Unit, Integration
**Priority**: P1

**Commands**:
```bash
# Unit tests for rollback procedures
cargo test --lib automation::rollback::procedures -- --test-threads=1

# Unit tests for cleanup
cargo test --lib automation::rollback::cleanup -- --test-threads=1

# Unit tests for verification
cargo test --lib automation::rollback::verification -- --test-threads=1

# Integration tests with mock git repo
cargo test --test rollback_cleanup -- --test-threads=1 --nocapture
```

---

### Area 7: Scheduling Policy Compiler

**Schema Ref**: N/A (new feature - compiles cron to WorkflowIR)
**Plan Ref**: Task 06 (`docs/plans/05-automation/tasks/06-scheduling-policy-compiler.md`)
**Files**: `automation/compiler/mod.rs`, `automation/compiler/scheduling_node.rs`, `automation/compiler/cron_compiler.rs`, `automation/compiler/resource_compiler.rs`

**Criteria**:
- Policy compilation: Compile cron policies to SchedulingNode in WorkflowIR
- Cron expression compilation: Parse and validate cron expressions at compilation time
- Runtime no parsing: Runtime scheduler uses pre-compiled scheduling metadata
- WorkflowIR integration: SchedulingNode variant in WorkflowIR
- Serialization: WorkflowIR serialization includes compiled scheduling metadata
- Resource limit compilation: Compile resource limits at policy definition time
- Validation: Validate compiled output structure
- Unit tests: Unit tests verify compilation output structure
- ADR-0007 compliance: Cron policies compiled to WorkflowIR, not interpreted at runtime

**Test Type**: Unit, Integration
**Priority**: P0

**Commands**:
```bash
# Unit tests for scheduling node compilation
cargo test --lib automation::compiler::scheduling_node -- --test-threads=1

# Unit tests for cron compiler
cargo test --lib automation::compiler::cron_compiler -- --test-threads=1

# Integration tests
cargo test --test scheduling_policy_compiler -- --test-threads=1 --nocapture
```

---

### Area 8: Automation CLI

**Schema Ref**: N/A (new feature - CLI commands)
**Plan Ref**: Task 07 (`docs/plans/05-automation/tasks/07-automation-cli.md`)
**Files**: `automation/cli/mod.rs`, `automation/cli/schedule.rs`, `automation/cli/experiment.rs`, `automation/cli/merge.rs`, `automation/cli/refine.rs`, `automation/cli/rollback.rs`

**Criteria**:
- Schedule management commands: `whitt schedule add/remove/list/pause/resume`
- Experiment commands: `whitt experiment start/stop/list`
- Merge proposal commands: `whitt merge approve/reject/view`
- Refinement commands: `whitt refine approve/reject/modify`
- Rollback commands: `whitt rollback <experiment-id>`
- Command validation: Validate command arguments before execution
- Error messages: Clear error messages for invalid inputs
- Help text: Accurate help text for all commands
- Interactive mode: Support interactive prompts for confirmations
- Exit codes: Proper exit codes (0 for success, non-zero for errors)
- Progress indicators: Progress indicators for long-running operations

**Test Type**: Integration, E2E
**Priority**: P0

**Commands**:
```bash
# Unit tests for CLI parsing
cargo test --lib automation::cli -- --test-threads=1

# Integration tests for schedule commands
cargo test --test cli_schedule_commands -- --test-threads=1 --nocapture

# Integration tests for experiment commands
cargo test --test cli_experiment_commands -- --test-threads=1 --nocapture

# Integration tests for merge/refine/rollback commands
cargo test --test cli_merge_refine_rollback -- --test-threads=1 --nocapture

# Live CLI testing
cargo run --bin whitt schedule --help
cargo run --bin whitt experiment --help
cargo run --bin whitt merge --help
cargo run --bin whitt rollback --help
```

---

### Area 9: Automation UI Integration

**Schema Ref**: N/A (new feature - React-based UI)
**Plan Ref**: Task 08 (`docs/plans/05-automation/tasks/08-automation-ui-integration.md`)
**Files**: `automation/ui/mod.rs`, `automation/ui/experiments.rs`, `automation/ui/proposals.rs`, `automation/ui/refinements.rs`

**Criteria**:
- React-based UI: React UI for experiment tracking and management
- Experiment tracking UI: Display experiment status, results, logs
- Merge proposal UI: Display proposals, diffs, approve/reject actions
- Refinement capture UI: Capture approvals, rejections, modifications
- Real-time updates: WebSocket or polling for real-time status updates
- Responsive design: UI works on desktop and tablet
- Error handling: Graceful error handling in UI
- Backend integration: UI uses CLI backend via API
- Authentication: User authentication for sensitive operations (approve/rollback)

**Test Type**: Integration, E2E
**Priority**: P2

**Commands**:
```bash
# Integration tests for UI
cargo test --test ui_integration -- --test-threads=1 --nocapture

# UI component tests (if applicable)
cargo test --lib ui_components -- --test-threads=1

# E2E tests with UI
# Start UI server and test with Playwright
cargo run --bin whitt-ui &
sleep 5
cargo test --test ui_e2e -- --test-threads=1 --nocapture
```

---

## ADR-0007 Compliance Validation

- [ ] Cron policies compiled to WorkflowIR, not interpreted at runtime
- [ ] Git experiments run in isolated branches only
- [ ] Merge proposals written to `./workspace/merge-proposals/` only, never auto-committed
- [ ] Manual refinements captured as immutable events
- [ ] Rollback procedures clean up all artifacts
- [ ] Scheduler enforces safety limits (max concurrent experiments, etc.)
- [ ] Git operations never modify main branch
- [ ] Failed experiments are fully cleaned up
- [ ] Rollback restores system to pre-experiment state

---

## Performance Targets

| Metric | Target | Validation Method |
|--------|--------|------------------|
| Cron scheduler accuracy | Cron triggers at correct times | Integration test |
| Experiment creation time | < 5s | Benchmark test |
| Merge proposal generation | < 30s | Benchmark test |
| Rollback & cleanup | < 60s | Benchmark test |
| Experiment result tracking | > 100 experiments tracked | Integration test |

---

## Phase Exit Criteria

Phase 05 is complete when:

1. **All 9 tasks** are implemented and passing their validation criteria
2. **All tests** pass with proper mock strategies
3. **ADR-0007 compliance** verified through full checklist
4. **Integration tests** demonstrate end-to-end automation workflow
5. **Documentation** covers all APIs, configuration, and usage patterns
6. **Performance benchmarks** meet targets
7. **Automation layer** integrates with existing WorkflowIR execution engine

---

**End of QA Criteria for Phase 05**

## Related Tasks

This QA criteria document covers the following plan tasks:

- **Task 00**: [Cron Scheduler](../plans/05-automation/tasks/00-cron-scheduler.md) — Implements cron-based scheduler for automated workflow execution
- **Task 01**: [Git Experiment Framework](../plans/05-automation/tasks/01-git-experiment-framework.md) — Builds framework for git-based experiments
- **Task 02**: [Merge Proposal Generation](../plans/05-automation/tasks/02-merge-proposal-generation.md) — Generates merge proposals based on experiment results
- **Task 03**: [Manual Refinement Capture](../plans/05-automation/tasks/03-manual-refinement-capture.md) — Captures manual refinements made during experiments
- **Task 04**: [Experiment Result Tracking](../plans/05-automation/tasks/04-experiment-result-tracking.md) — Tracks and aggregates experiment results
- **Task 05**: [Rollback & Cleanup](../plans/05-automation/tasks/05-rollback-cleanup.md) — Implements rollback and cleanup for failed experiments
- **Task 06**: [Scheduling Policy Compiler](../plans/05-automation/tasks/06-scheduling-policy-compiler.md) — Compiles scheduling policies for automated execution
- **Task 07**: [Automation CLI](../plans/05-automation/tasks/07-automation-cli.md) — Implements CLI commands for automation control
- **Task 08**: [Automation UI Integration](../plans/05-automation/tasks/08-automation-ui-integration.md) — Integrates automation controls into dashboard UI

