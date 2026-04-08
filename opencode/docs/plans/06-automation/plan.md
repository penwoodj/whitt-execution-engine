# Phase 6: Automation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a comprehensive automation layer for the AgentSDK Execution Engine that supports cron-based scheduling, git experiment management, merge proposal generation, manual refinement capture, experiment result tracking, and rollback/cleanup procedures, all while maintaining strict ADR-0007 compliance.

**Architecture:** The automation layer integrates with the existing WorkflowIR execution engine by compiling cron scheduling policies into scheduling nodes, running git experiments in isolated worktrees, and artifacting all merge recommendations and manual refinements without auto-committing. The system follows a multi-component architecture with clear separation between scheduling logic, experiment management, and UI/CLI interfaces.

**Tech Stack:** Rust (tokio for async, chrono for time handling, git2 for git operations, cron for expression parsing), WorkflowIR (existing), tokio-cron-scheduler for job scheduling, serde for serialization.

---

## Table of Contents

- [Overview](#overview)
- [Dependencies](#dependencies)
- [ADR-0007 Compliance](#adr-0007-compliance)
- [Architecture Overview](#architecture-overview)
- [File Structure](#file-structure)
- [Task Breakdown](#task-breakdown)
- [Testing Strategy](#testing-strategy)
- [Validation Criteria](#validation-criteria)
- [Timeline](#timeline)

---

## Overview

Phase 6 Automation adds two critical capabilities to the AgentSDK Execution Engine:

1. **Time-Based Scheduling**: Execute workflows on cron schedules, compiled into WorkflowIR scheduling nodes
2. **Git Experiment Management**: Run parallel experiments in isolated branches, generate merge proposals, track results, and manage rollbacks

This automation layer enables automated workflow execution while maintaining strict governance around git operations and manual refinement tracking.

### Key Principles

- **Safety-First**: All automation runs in isolated contexts (worktrees, branches, containers)
- **Auditability**: Every automated action produces artifacts in `.glyphnova/`
- **Manual Control**: Merge proposals are outputs, not auto-commits (ADR-0007)
- **Recoverability**: All experiments can be rolled back with complete cleanup

---

## Dependencies

This phase **requires** completion of Phases 0-5:

- **Phase 0**: Project setup, basic infrastructure
- **Phase 1**: WorkflowIR core execution engine
- **Phase 2**: Workflow validation and transformation
- **Phase 3**: Agent execution framework
- **Phase 4**: Tool execution and resource management
- **Phase 5**: Error handling, logging, and observability

### Critical Dependencies

- `workflow_ir::ExecutionEngine` - For executing scheduled workflows
- `workflow_ir::Node` - Scheduling nodes will be added here
- `git2` crate - For git operations
- `tokio` - Async runtime for scheduler

---

## ADR-0007 Compliance

ADR-0007 governs automation in the AgentSDK. This implementation must follow these constraints:

### Cron Policies Compile to WorkflowIR (Not Interpreted)

**Constraint:** Cron scheduling policies are compiled into WorkflowIR scheduling nodes at policy definition time, not interpreted at runtime.

**Implementation:**
- Task 06: Scheduling Policy Compiler creates `SchedulingNode` variants in WorkflowIR
- Cron expressions are parsed and validated once during compilation
- Runtime scheduler uses pre-compiled scheduling metadata
- No cron expression parsing at execution time

**Validation:**
- WorkflowIR serialization includes compiled scheduling metadata
- Runtime engine never calls cron parser
- Unit tests verify compilation output structure

### Git Experiments Run in Isolated Branches

**Constraint:** All git experiments must execute in isolated branches, never on the main branch.

**Implementation:**
- Task 01: Git Experiment Framework enforces branch isolation
- Task 05: Rollback and Cleanup ensures branch cleanup
- Scheduler validates experiment targets are branches, not main
- Worktree management prevents workspace contamination

**Validation:**
- Integration tests verify branch creation before experiment execution
- Cleanup tests verify no artifacts on main branch
- Scheduler rejects experiments targeting main branch

### Merge Recommendations Are Outputs (Not Auto-Commits)

**Constraint:** Merge proposals are generated as artifacts in `.glyphnova/`, not automatically committed.

**Implementation:**
- Task 02: Merge Proposal Generation writes diff files to `.glyphnova/merge-proposals/`
- Task 03: Manual Refinement Capture links proposals to approval/rejection events
- Task 07: Automation CLI provides `approve` and `reject` commands for manual action
- No auto-merge logic in the automation layer

**Validation:**
- Tests verify proposals are written to artifact directory only
- Integration tests verify no automatic commits occur
- Approval workflow tests verify manual intervention required

### Manual Refinement Preserved as Artifacted Events

**Constraint:** All manual changes (approvals, rejections, refinements) are captured as events in `.glyphnova/`.

**Implementation:**
- Task 03: Manual Refinement Capture records all refinement events
- Events include: what, why, who, when, linked artifacts
- Immutable event log in `.glyphnova/refinements/`
- Rollback can undo refinements by reversing event log

**Validation:**
- Event log tests verify immutability
- Rollback tests verify refinement reversal
- Audit trail tests verify complete history

---

## Architecture Overview

### Component Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    Automation CLI (Task 07)                     │
└───────────────────────────┬─────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│  Cron         │  │  Git          │  │  Result       │
│  Scheduler    │  │  Experiment   │  │  Tracker      │
│  (Task 00)    │  │  Framework    │  │  (Task 04)    │
└───────┬───────┘  └───────┬───────┘  └───────┬───────┘
        │                  │                  │
        │                  ▼                  │
        │         ┌─────────────────┐          │
        │         │  Merge          │          │
        │         │  Proposal       │          │
        │         │  Generation     │          │
        │         │  (Task 02)      │          │
        │         └───────┬─────────┘          │
        │                 │                    │
        │                 ▼                    │
        │         ┌─────────────────┐          │
        │         │  Manual          │          │
        │         │  Refinement      │          │
        │         │  Capture         │          │
        │         │  (Task 03)       │          │
        │         └───────┬─────────┘          │
        │                 │                    │
        └─────────────────┼────────────────────┘
                          │
                          ▼
                ┌─────────────────┐
                │  Rollback &     │
                │  Cleanup        │
                │  (Task 05)      │
                └─────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│          Scheduling Policy Compiler (Task 06)                   │
│  Compiles cron policies → SchedulingNode in WorkflowIR          │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│          Automation UI Integration (Task 08)                    │
│  React-based UI for experiment tracking and management         │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│              WorkflowIR Execution Engine                        │
│  (Existing - Phases 0-5)                                         │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow

1. **Schedule Definition**: User defines cron schedule via CLI
2. **Policy Compilation**: Cron policy compiled to SchedulingNode in WorkflowIR
3. **Execution**: Cron scheduler triggers workflow execution
4. **Experiment**: Workflow runs experiment in isolated branch/worktree
5. **Result Capture**: Experiment results captured and compared
6. **Merge Proposal**: Diff generated and written to `.glyphnova/merge-proposals/`
7. **Manual Refinement**: User reviews, approves/rejects/refines via CLI or UI
8. **Rollback**: If needed, experiment rolled back and branch deleted

### Artifact Structure

```
.glyphnova/
├── experiments/
│   ├── <experiment-id>/
│   │   ├── manifest.json
│   │   ├── results.json
│   │   └── artifacts/
├── merge-proposals/
│   ├── <proposal-id>/
│   │   ├── proposal.json
│   │   ├── diff.patch
│   │   └── metadata.json
├── refinements/
│   ├── <event-id>/
│   │   ├── event.json
│   │   └── linked-artifacts.json
└── rollbacks/
    ├── <rollback-id>/
    │   ├── rollback.json
    │   └── cleanup-log.txt
```

---

## File Structure

### New Files to Create

```
automation/
├── cron/
│   ├── mod.rs                    # Cron scheduler module
│   ├── parser.rs                 # Cron expression parser
│   ├── scheduler.rs              # Tokio-based cron scheduler
│   ├── validator.rs              # Safety limits validation
│   └── persistence.rs            # Schedule persistence
├── experiment/
│   ├── mod.rs                    # Experiment framework module
│   ├── manifest.rs               # Experiment manifest types
│   ├── git_operations.rs        # Git worktree management
│   ├── merge_policy.rs          # Auto-merge/approval policies
│   └── isolation.rs             # Branch/worktree isolation
├── merge/
│   ├── mod.rs                    # Merge proposal module
│   ├── generator.rs             # Diff generation
│   ├── validator.rs             # Validation criteria
│   ├── confidence.rs            # Confidence scoring
│   └── artifacts.rs             # Proposal artifact management
├── refinement/
│   ├── mod.rs                    # Refinement capture module
│   ├── events.rs                # Refinement event types
│   ├── capture.rs               # Event capture logic
│   └── linking.rs               # Artifact linking
├── tracking/
│   ├── mod.rs                    # Result tracking module
│   ├── storage.rs               # Result storage
│   ├── comparison.rs            # Result comparison logic
│   ├── visualization.rs         # Visualization helpers
│   └── export.rs                # Export functionality
├── rollback/
│   ├── mod.rs                    # Rollback module
│   ├── procedures.rs            # Rollback procedures
│   ├── cleanup.rs               # Cleanup operations
│   └── verification.rs          # Post-rollback verification
├── compiler/
│   ├── mod.rs                    # Policy compiler module
│   ├── scheduling_node.rs       # SchedulingNode compilation
│   ├── cron_compiler.rs         # Cron expression compilation
│   └── resource_compiler.rs     # Resource limit compilation
├── cli/
│   ├── mod.rs                    # Automation CLI module
│   ├── schedule.rs              # Schedule management commands
│   ├── experiment.rs            # Experiment commands
│   ├── merge.rs                 # Merge proposal commands
│   ├── refine.rs                # Refinement commands
│   └── rollback.rs              # Rollback commands
└── ui/
    ├── mod.rs                    # UI integration module
    ├── experiments.rs           # Experiment tracking UI
    ├── proposals.rs             # Merge proposal UI
    └── refinements.rs           # Refinement capture UI

workflow_ir/
├── node.rs                      # MODIFIED: Add SchedulingNode variant
└── execution.rs                 # MODIFIED: Integrate scheduling nodes

src/
└── main.rs                      # MODIFIED: Initialize automation modules

tests/
├── integration/
│   ├── cron_scheduler_test.rs
│   ├── git_experiment_test.rs
│   ├── merge_proposal_test.rs
│   └── rollback_test.rs
└── mock/
    ├── git_repo.rs              # Mock git repository helper
    └── cron_scheduler.rs         # Mock cron scheduler helper
```

### Files to Modify

- `workflow_ir/node.rs` - Add `SchedulingNode` variant
- `workflow_ir/execution.rs` - Integrate scheduling node execution
- `src/main.rs` - Initialize automation modules
- `Cargo.toml` - Add dependencies (tokio-cron-scheduler, cron, git2)

---

## Task Breakdown

The implementation is divided into 8 major tasks, each building on the previous ones:

1. **Task 00**: Cron Scheduler - Core time-based scheduling infrastructure
2. **Task 01**: Git Experiment Framework - Branch isolation and worktree management
3. **Task 02**: Merge Proposal Generation - Diff generation and artifact management
4. **Task 03**: Manual Refinement Capture - Event capture and linking
5. **Task 04**: Experiment Result Tracking - Storage, comparison, visualization
6. **Task 05**: Rollback & Cleanup - Safe experiment rollback procedures
7. **Task 06**: Scheduling Policy Compiler - Compile cron policies to WorkflowIR
8. **Task 07**: Automation CLI - User-facing automation commands
9. **Task 08**: Automation UI Integration - Web UI for experiment management

Each task is broken down into bite-sized steps (2-5 minutes each) following TDD principles.

### Inter-Task Dependencies

```
Task 00 (Cron Scheduler)
    │
    ├── Task 06 (Scheduling Policy Compiler)
    │       └── Compiles to SchedulingNode for use in Cron Scheduler
    │
    ├── Task 01 (Git Experiment Framework)
    │       └── Scheduler triggers experiments
    │
    └── Task 07 (Automation CLI)
            └── CLI manages schedules

Task 01 (Git Experiment Framework)
    │
    ├── Task 02 (Merge Proposal Generation)
    │       └── Generates proposals from experiment results
    │
    ├── Task 04 (Experiment Result Tracking)
    │       └── Captures experiment results
    │
    └── Task 05 (Rollback & Cleanup)
            └── Cleans up failed experiments

Task 02 (Merge Proposal Generation)
    │
    ├── Task 03 (Manual Refinement Capture)
    │       └── Captures approvals/rejections of proposals
    │
    └── Task 07 (Automation CLI)
            └── CLI commands for proposal management

Task 03 (Manual Refinement Capture)
    │
    ├── Task 04 (Experiment Result Tracking)
    │       └── Links refinements to results
    │
    └── Task 07 (Automation CLI)
            └── CLI commands for refinement

Task 04 (Experiment Result Tracking)
    │
    ├── Task 05 (Rollback & Cleanup)
    │       └── Uses results for rollback decisions
    │
    └── Task 08 (Automation UI Integration)
            └── UI displays tracking results

Task 05 (Rollback & Cleanup)
    │
    └── Task 07 (Automation CLI)
            └── CLI commands for rollback

Task 06 (Scheduling Policy Compiler)
    │
    └── Task 07 (Automation CLI)
            └── CLI uses compiled policies

Task 07 (Automation CLI)
    │
    └── Task 08 (Automation UI Integration)
            └── UI uses CLI backend

Task 08 (Automation UI Integration)
    │
    └── (Final Task - integrates everything)
```

---

## Testing Strategy

### Unit Tests

Each module will have comprehensive unit tests covering:
- Happy path scenarios
- Error conditions
- Edge cases
- ADR-0007 compliance (no auto-commits, branch isolation, etc.)

### Integration Tests

Integration tests will cover:
- End-to-end scheduling → execution → result → proposal workflow
- Git experiment lifecycle (create → execute → cleanup)
- Merge proposal generation and approval/rejection
- Rollback procedures

### Mock Strategy

See `tests/mock/` for mock implementations:
- `git_repo.rs` - Temporary git repository for testing git operations
- `cron_scheduler.rs` - Mock cron scheduler for deterministic testing

### Validation Tests

See `validation/` for validation criteria tests:
- ADR-0007 compliance validation
- Schedule safety validation
- Experiment isolation validation

---

## Validation Criteria

### ADR-0007 Compliance Validation

- [ ] Cron policies compiled to WorkflowIR, not interpreted at runtime
- [ ] Git experiments run in isolated branches only
- [ ] Merge proposals written to `.glyphnova/` only, never auto-committed
- [ ] Manual refinements captured as immutable events

### Functional Validation

- [ ] Cron scheduler executes workflows on schedule
- [ ] Git experiments create isolated worktrees
- [ ] Merge proposals generate accurate diffs
- [ ] Experiment results are tracked and comparable
- [ ] Rollback procedures clean up all artifacts

### Integration Validation

- [ ] End-to-end workflow: schedule → experiment → proposal → refinement → merge/rollback
- [ ] CLI commands function correctly
- [ ] UI displays accurate experiment status

### Safety Validation

- [ ] Scheduler enforces safety limits (max concurrent experiments, etc.)
- [ ] Git operations never modify main branch
- [ ] Failed experiments are fully cleaned up
- [ ] Rollback restores system to pre-experiment state

---

## Timeline

**Estimated Duration:** 8-10 weeks

**Sprint Breakdown:**

- **Weeks 1-2**: Task 00 (Cron Scheduler) + Task 06 (Scheduling Policy Compiler)
- **Weeks 3-4**: Task 01 (Git Experiment Framework) + Task 05 (Rollback & Cleanup)
- **Weeks 5-6**: Task 02 (Merge Proposal Generation) + Task 03 (Manual Refinement Capture)
- **Weeks 7-8**: Task 04 (Experiment Result Tracking) + Task 07 (Automation CLI)
- **Weeks 9-10**: Task 08 (Automation UI Integration) + integration testing

### Parallelization Opportunities

- Tasks 00 and 06 can be developed in parallel (separate teams)
- Tasks 01 and 05 can be developed in parallel (separate teams)
- Tasks 02 and 03 can be developed in parallel (separate teams)
- Task 07 can be developed alongside tasks 01-06 (CLI interfaces)
- Task 08 depends on all other tasks (UI integration)

---

## Next Steps

This plan provides a comprehensive roadmap for implementing Phase 6 Automation. Each task file in `tasks/` contains detailed implementation steps.

**Execution Options:**

1. **Subagent-Driven (recommended)** - Fresh subagent per task, review between tasks, fast iteration
2. **Inline Execution** - Execute in this session using executing-plans, batch execution with checkpoints

See individual task files for detailed implementation steps.

---

**Document History:**
- Created: 2026-04-06
- Status: Draft
- Reviewer: TBD
- Approved: TBD
