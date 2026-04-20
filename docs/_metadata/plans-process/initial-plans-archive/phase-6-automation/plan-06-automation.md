# Plan-06: Cron Execution, Git-Branch Experimentation, and Refining Workflows

**Plan ID**: plan-06
**Phase**: Phase 6 - Automation
**Status**: Not Started
**Created**: 2026-03-27
**Related ADR**: ADR-0007 (Cron execution git-branch experimentation and refining workflows)
**Related Research Plan**: research-plan-04-automation-autonomy-metrics.yml
**Estimated Time**: 8-10 weeks
**Dependencies**: plan-00 (Foundation), plan-01 (MVP Queue), plan-02 (CLI & Backends), plan-03 (Glyphnova UI), plan-04 (Quality Loops), plan-05 (Memory & Search)

---

## Overview

This plan implements cron job scheduling, git-aware experiments, and manual refinement capture for automated recurring workflows with safety controls.

**Key Features**:
- **Cron Job Scheduling**: Scheduled experiments from workflow definitions
- **Versioned Workflow Runs**: Cron jobs instantiate versioned runs from stable specs
- **Persistent Experiment Manifests**: State tracking for all scheduled experiments
- **Git-Aware Experiments**: Run in isolated working copies or branches
- **Merge Policy Controls**: Explicit merge policies (auto-merge, require-approval, block)
- **Manual Refinement Capture**: Preserved as artifacted review events (not implicit side channel)
- **Merge Recommendation Generation**: Output recommendations (not auto-commits) until confidence thresholds mature
- **Experiment Result Tracking**: Comparable results across experiments
- **Rollback and Cleanup**: Failed experiment procedures
- **Scheduling in WorkflowIR**: Cron policies compiled into WorkflowIR (not interpreted at runtime)

**Key Deliverables**:
- Cron job scheduler
- Git experiment framework
- Merge proposal generation system
- Manual refinement capture system
- Experiment result tracking and comparison
- Rollback and cleanup procedures
- Scheduling policy compilation
- Automation CLI and UI integration

---

## Code Review

### ADR-0007 Summary

**Decision**: Treat scheduled experiments as governed workflow campaigns with transpiler integration.

**Key Requirements**:
1. **Cron Jobs**: Instantiate versioned workflow runs from stable specs
2. **Backed-Up Text Corpora**: Immutable inputs unless explicitly promoted
3. **Git-Aware Experiments**: Run in isolated working copies or branches with explicit merge policies
4. **Manual Refinement**: Preserved as artifacted review events (not implicit side channel)
5. **Merge Recommendations**: Outputs not auto-commits until confidence and validation thresholds are mature
6. **Scheduling in WorkflowIR**: Compiled into WorkflowIR (not interpreted at runtime)

**Scope**:
- **Included**: Cron job scheduling, persistent experiment manifests, git branch isolation, merge proposal generation, manual refinement capture, experiment result tracking, rollback and cleanup
- **Excluded**: Always-on autonomous loops without scheduling, direct main branch manipulation, broad cross-repo automation, unmanaged cron expressions without safety limits

**Positive Consequences**:
- Enables automated recurring workflows with safety
- Git experiments provide repeatable test environments
- Manual refinement is preserved and auditable
- Merge policies prevent accidental main branch corruption

**Negative Consequences**:
- More operational complexity in scheduling and git management
- Storage growth from experiment artifacts and branches
- Manual review gating may slow automation

### Requirements Review

From `requirements.md` and `schema-consolidated-report.md`:

**R17**: Multiple workflow archetypes and scheduling modes
**R24**: Queue semantics (worker pools, persistence, cancellation, result retrieval)
**R26**: Long-running behavior for background, semi-endless, endless, and resumable workflows
**R30**: Observability with logging levels, summaries, graphs, reports, and provenance

**Validation Focus for v0.1.0**:
- Cron jobs execute workflows with proper environment isolation
- Git experiments run in isolated branches without affecting main code
- Merge proposals are generated with clear diff recommendations
- Manual refinement events are captured as auditable artifacts
- Experiment results are tracked and comparable
- Rollback procedures exist for failed experiments

**Validation Focus for v0.1.x and Later**:
- Scheduling supports automated recurring workflows without manual triggers
- Git experimentation framework enables parallel test branches
- Merge automation with validation prevents breaking changes
- Manual refinement workflow integrates with artifact review system
- Cron policies include resource limits and failure handling
- Automation workflows maintain auditability through observability

### Transpiler Integration Points

- **WorkflowIR**: Should support scheduling modes for cron and long-running workflows
- **Cron Expressions**: Should compile into WorkflowIR scheduling nodes
- **Git-Aware Tool Nodes**: Require branch context and merge policy declarations
- **Experiment Artifacts**: Should reference workflow version and policy snapshot
- **Manual Refinement Operations**: Require review workflow integration

### Research Plan Review

**research-plan-04-automation-autonomy-metrics.yml** is **COMPLETED** with 3 research domains (plan-06 uses domain 1):

1. **Automation and Scheduling**: What scheduling patterns and git experimentation frameworks best support safe automation?

**Key Findings**:
- Report outputs: `automation-autonomy-metrics-research-report.md`, `automation-patterns.csv`, `git-experimentation-options.csv`
- Quality gates met: Recommendations include safety limits and auditability

---

## Web Research

### Research Area 1: Cron Job Scheduling Patterns

**Research Question**: What cron scheduling patterns and safety limits work best for automated workflows?

**Recommended Sources**:
- [Cron expression docs](https://crontab.guru) - Cron expression reference
- [tokio-cron-scheduler](https://docs.rs/tokio-cron-scheduler) - Rust cron scheduler
- [Saffron](https://github.com/samsquire/saffron) - Cron scheduler library
- [Airflow](https://airflow.apache.org) - Workflow scheduling patterns
- [GitHub Actions scheduling](https://docs.github.com/en/actions) - Scheduled workflow patterns

**Expected Findings**:
- Cron expression parsing and validation
- Scheduling engine patterns (tokio-based, thread-based)
- Safety limits (rate limiting, resource limits)
- Timezone handling
- Missed schedule handling
- Failure retry policies

**Status**: Not Started

---

### Research Area 2: Git Experimentation Frameworks

**Research Question**: What git branching and experimentation frameworks enable safe, isolated experiments?

**Recommended Sources**:
- [git-worktree](https://git-scm.com/docs/git-worktree) - Multiple working trees
- [git-branch](https://git-scm.com/docs/git-branch) - Branch management
- [GitHub Actions](https://github.com/features/actions) - CI experimentation patterns
- [GitLab CI](https://docs.gitlab.com/ee/ci/) - CI experimentation patterns
- [Dagger](https://dagger.io) - CI/CD experimentation

**Expected Findings**:
- Git worktree vs branch isolation
- Experiment branch naming and organization
- Merge policy enforcement (squash, rebase, merge commit)
- Branch cleanup strategies
- Conflict resolution patterns
- Parallel experiment management

**Status**: Not Started

---

### Research Area 3: Merge Policy and Validation

**Research Question**: What merge policies and validation patterns prevent breaking changes?

**Recommended Sources**:
- [GitHub Pull Request API](https://docs.github.com/en/pull-requests) - PR validation
- [GitLab Merge Request](https://docs.gitlab.com/ee/user/project/merge_requests/) - MR validation
- [pre-commit hooks](https://pre-commit.com) - Validation before merge
- [git hooks](https://githooks.com) - Git event-based validation
- [Semgrep](https://semgrep.dev) - Security and quality validation

**Expected Findings**:
- Merge policy types (auto-merge, require-approval, block)
- Validation criteria (tests pass, security scan, code review)
- Confidence thresholds for auto-merge
- Rollback triggers and procedures
- Merge conflict handling
- Notification and approval workflows

**Status**: Not Started

---

### Research Area 4: Manual Refinement Capture

**Research Question**: What patterns exist for capturing manual refinements as auditable artifacts?

**Recommended Sources**:
- [GitHub code review](https://docs.github.com/en/pull-requests/reviewing-changes) - Review comments
- [GitLab discussions](https://docs.gitlab.com/ee/user/discussions/) - Discussion artifacts
- [Change logs](https://keepachangelog.com) - Change tracking patterns
- [Audit logging](https://owasp.org) - Audit log patterns
- [Artifact versioning](https://provenance.io) - Provenance patterns

**Expected Findings**:
- Review artifact structures
- Comment and suggestion capture
- Refinement tracking (what changed, why)
- Approval and rejection workflows
- Refinement-to-artifact linking
- Historical query and analysis

**Status**: Not Started

---

### Research Area 5: Experiment Result Comparison

**Research Question**: What patterns enable effective comparison of experiment results?

**Recommended Sources**:
- [A/B testing frameworks](https://github.com) - Search for "ab testing" projects
- [Benchmark comparison](https://docs.rs/criterion) - Rust benchmark comparison
- [Experiment tracking](https://www.mlflow.org) - ML experiment tracking
- [Data visualization](https://d3js.org) - Result visualization
- [Statistical analysis](https://scipy.org) - Statistical comparison methods

**Expected Findings**:
- Result storage and indexing
- Comparison metrics (difference, ratio, statistical significance)
- Visualization patterns (side-by-side, diff, charts)
- Experiment groupings (by workflow, by time, by policy)
- Result ranking and selection
- Statistical significance testing

**Status**: Not Started

---

## Implementation Plan

### Phase 1: Cron Job Scheduler

**Description**: Implement cron job scheduling and execution

**Tasks**:
1. Define cron expression parser and validator
2. Implement scheduling engine (tokio-based)
3. Add safety limits (rate limiting, resource limits, max concurrent)
4. Implement schedule persistence (store scheduled jobs)
5. Add missed schedule handling
6. Implement failure retry policies
7. Add timezone support
8. Implement scheduling CLI commands (schedule, list, cancel)

**Related Requirements**: ADR-0007 (cron job scheduling)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

### Phase 2: Git Experimentation Framework

**Description**: Implement git-aware experiments in isolated branches/worktrees

**Tasks**:
1. Define experiment manifest structure (workflow version, branch, policy)
2. Implement git branch creation and isolation
3. Add git worktree support for parallel experiments
4. Implement experiment workspace management
5. Add merge policy definitions (auto-merge, require-approval, block)
6. Implement branch cleanup strategies
7. Add conflict detection and reporting
8. Implement git experiment CLI commands

**Related Requirements**: ADR-0007 (git branch isolation)
**Verification Layers**: 1, 2, 3, 4
**Status**: Not Started

---

### Phase 3: Merge Proposal Generation

**Description**: Generate merge proposals with validation and recommendations

**Tasks**:
1. Define merge proposal structure (diff, validation, recommendations)
2. Implement diff generation (experiment branch vs target)
3. Add validation criteria (tests pass, security scan, code review)
4. Implement confidence threshold evaluation
5. Generate merge recommendations (approve, request-changes, block)
6. Add merge proposal artifacts (stored in `.opencode/`)
7. Implement merge proposal CLI commands (create, view, approve, reject)
8. Add merge proposal UI integration

**Related Requirements**: ADR-0007 (merge proposal generation)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 4: Manual Refinement Capture

**Description**: Capture manual refinements as auditable artifacts

**Tasks**:
1. Define refinement event structure (what, why, who, when)
2. Implement refinement capture (comments, suggestions, manual edits)
3. Add refinement linking to artifacts
4. Implement refinement approval workflow
5. Add refinement rejection workflow
6. Implement refinement history tracking
7. Add refinement query interface
8. Implement refinement CLI commands (list, view, approve, reject)

**Related Requirements**: ADR-0007 (manual refinement capture)
**Verification Layers**: 1, 2
**Status**: Not Started

---

### Phase 5: Experiment Result Tracking

**Description**: Track and compare experiment results

**Tasks**:
1. Define experiment result structure (metrics, outputs, logs, timestamp)
2. Implement result storage (indexed by experiment ID)
3. Add result comparison logic (difference, ratio, significance)
4. Implement result visualization (side-by-side, diff, charts)
5. Add result ranking (by quality, by performance)
6. Implement result query interface (by workflow, by time, by policy)
7. Add result export (JSON, CSV)
8. Implement experiment result CLI commands

**Related Requirements**: ADR-0007 (experiment result tracking)
**Verification Layers**: 1, 2, 4
**Status**: Not Started

---

### Phase 6: Rollback and Cleanup

**Description**: Implement rollback and cleanup for failed experiments

**Tasks**:
1. Define rollback procedure structure
2. Implement branch deletion (cleanup failed experiments)
3. Add artifact cleanup (remove experiment artifacts)
4. Implement rollback to previous state
5. Add rollback verification (confirm rollback worked)
6. Implement cleanup CLI commands (cleanup, rollback)
7. Add cleanup scheduling (automatic after failed experiments)
8. Implement cleanup safety checks (confirm no active dependencies)

**Related Requirements**: ADR-0007 (rollback and cleanup)
**Verification Layers**: 1, 2
**Status**: Not Started

---

### Phase 7: Scheduling Policy Compilation

**Description**: Compile scheduling policies into WorkflowIR

**Tasks**:
1. Define scheduling policy nodes in WorkflowIR
2. Implement cron expression compiler (to IR scheduling nodes)
3. Add resource limit compilation
4. Implement failure policy compilation
5. Integrate scheduling nodes into WorkflowIR compiler
6. Add scheduling policy validation
7. Implement long-running workflow support
8. Add scheduling policy CLI commands (define, validate)

**Related Requirements**: ADR-0007 (scheduling in WorkflowIR)
**Verification Layers**: 1, 2, 3
**Status**: Not Started

---

## Verification Strategy

### Layer 1: Unit Tests

**Scope**: Individual component testing

**Coverage Areas**:
- Cron expression parsing and validation
- Scheduling engine (tick, schedule, execute)
- Git branch creation and isolation
- Merge proposal generation
- Refinement event structures
- Experiment result storage and comparison
- Rollback and cleanup procedures
- Scheduling policy compilation

**Test Framework**: `cargo test --lib`

**Success Criteria**:
- 90%+ code coverage on core modules
- All cron expressions tested (valid and invalid)
- All merge policies tested (auto-merge, require-approval, block)

---

### Layer 2: Integration Tests

**Scope**: Multi-component interaction testing

**Coverage Areas**:
- Cron scheduler with workflow execution
- Git experiments with branch isolation
- Merge proposals with validation
- Manual refinement capture and linking
- Experiment result tracking and comparison
- Rollback and cleanup workflows
- Scheduling policy compilation into WorkflowIR
- CLI and UI integration with all automation features

**Test Framework**: `cargo test --test '*'`

**Success Criteria**:
- Cron jobs execute workflows correctly
- Git experiments run in isolation
- Merge proposals generate correct diffs and recommendations
- Refinements are captured and linked
- Results are stored and comparable
- Rollback restores previous state
- Scheduling policies compile correctly
- UI displays automation state accurately

---

### Layer 3: Property-Based Tests

**Scope**: Edge cases and invariants

**Coverage Areas**:
- Cron expression parsing (always valid or always rejected)
- Scheduling determinism (same input = same schedule)
- Git branch isolation (no cross-branch pollution)
- Merge policy enforcement (policy always respected)
- Refinement event integrity (always linked to artifact)
- Experiment result comparison (deterministic ordering)
- Rollback correctness (always restores to exact state)

**Test Framework**: `proptest` with 1000 iterations each

**Success Criteria**:
- No crashes on random inputs
- Invariants always hold (schedule deterministic, isolation preserved, policy enforced)
- Properties verified with 1000+ iterations

---

### Layer 4: End-to-End Tests

**Scope**: Complete automation workflows

**Coverage Areas**:
- Schedule cron job with workflow
- Run scheduled workflow in git experiment branch
- Generate merge proposal with validation
- Capture manual refinement and link to proposal
- Track experiment results and compare
- Rollback failed experiment and cleanup
- Compile scheduling policy into WorkflowIR
- Execute WorkflowIR with scheduling nodes

**Test Framework**: `cargo test --test '*e2e*'`

**Success Criteria**:
- Complete automation workflows execute successfully
- Cron jobs run on schedule
- Git experiments are isolated and safe
- Merge proposals are generated with recommendations
- Refinements are captured and queryable
- Results are tracked and comparable
- Rollback restores state correctly
- Scheduling policies execute deterministically

---

## Verification Checkpoints

### Checkpoint 1: Cron Scheduler Working

**Target Date**: Week 2
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] Cron expressions parse correctly
- [ ] Scheduler executes jobs on schedule
- [ ] Safety limits are enforced
- [ ] Missed schedules are handled
- [ ] Failure retry policies work
- [ ] CLI schedule commands work
- [ ] Property tests verify scheduling determinism
- [ ] Integration tests pass with real workflows

**Status**: Not Started

---

### Checkpoint 2: Git Experiments Isolated

**Target Date**: Week 4
**Verification Layers**: 1, 2, 3, 4
**Sign-Off Criteria**:
- [ ] Git branches are created for experiments
- [ ] Branch isolation prevents cross-branch pollution
- [ ] Git worktrees support parallel experiments
- [ ] Merge policies are enforced
- [ ] Branch cleanup works
- [ ] Conflicts are detected and reported
- [ ] Property tests verify isolation invariants
- [ ] E2E test: complete experiment workflow in branch

**Status**: Not Started

---

### Checkpoint 3: Merge Proposals Generated

**Target Date**: Week 5
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Merge proposals are generated correctly
- [ ] Diffs are accurate
- [ ] Validation criteria are evaluated
- [ ] Confidence thresholds work
- [ ] Recommendations are clear
- [ ] Proposal artifacts are stored
- [ ] CLI proposal commands work
- [ ] E2E test: create, view, approve proposal

**Status**: Not Started

---

### Checkpoint 4: Manual Refinements Captured

**Target Date**: Week 6
**Verification Layers**: 1, 2
**Sign-Off Criteria**:
- [ ] Refinement events are captured correctly
- [ ] Refinements are linked to artifacts
- [ ] Approval workflow works
- [ ] Rejection workflow works
- [ ] History is tracked
- [ ] Query interface works
- [ ] CLI refinement commands work
- [ ] Integration tests pass with refinements

**Status**: Not Started

---

### Checkpoint 5: Experiment Results Tracked

**Target Date**: Week 7
**Verification Layers**: 1, 2, 4
**Sign-Off Criteria**:
- [ ] Experiment results are stored correctly
- [ ] Result comparison works
- [ ] Visualization renders correctly
- [ ] Ranking works by quality/performance
- [ ] Query interface returns correct results
- [ ] Export works
- [ ] CLI result commands work
- [ ] E2E test: track and compare experiment results

**Status**: Not Started

---

### Checkpoint 6: Rollback and Cleanup Working

**Target Date**: Week 8
**Verification Layers**: 1, 2
**Sign-Off Criteria**:
- [ ] Rollback restores previous state
- [ ] Branch deletion works
- [ ] Artifact cleanup removes experiment artifacts
- [ ] Rollback verification works
- [ ] CLI cleanup commands work
- [ ] Cleanup scheduling works
- [ ] Safety checks prevent dangerous cleanup
- [ ] Integration tests pass with rollback and cleanup

**Status**: Not Started

---

### Checkpoint 7: Scheduling Policy Compiled

**Target Date**: Week 10
**Verification Layers**: 1, 2, 3
**Sign-Off Criteria**:
- [ ] Scheduling policies compile to IR correctly
- [ ] Cron expressions compile to IR scheduling nodes
- [ ] Resource limits compile correctly
- [ ] Failure policies compile correctly
- [ ] Scheduling nodes integrate into WorkflowIR
- [ ] Validation works on compiled policies
- [ ] Long-running workflows execute correctly
- [ ] Property tests verify compilation correctness
- [ ] Integration tests pass with compiled policies

**Status**: Not Started

---

## Progress Tracking

### Overall Progress

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Phases Complete | 7 | 0 | 0% |
| Verification Layers | 4 | 0 | 0% |
| Checkpoints Passed | 7 | 0 | 0% |
| Web Research Areas | 5 | 0 | 0% |

**Overall Status**: Not Started

---

### Phase Progress

| Phase | Description | Status | Completion |
|-------|-------------|--------|------------|
| 1 | Cron Job Scheduler | Not Started | 0% |
| 2 | Git Experimentation Framework | Not Started | 0% |
| 3 | Merge Proposal Generation | Not Started | 0% |
| 4 | Manual Refinement Capture | Not Started | 0% |
| 5 | Experiment Result Tracking | Not Started | 0% |
| 6 | Rollback and Cleanup | Not Started | 0% |
| 7 | Scheduling Policy Compilation | Not Started | 0% |

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
| Cron Job Scheduling Patterns | Not Started | 0 | No |
| Git Experimentation Frameworks | Not Started | 0 | No |
| Merge Policy and Validation | Not Started | 0 | No |
| Manual Refinement Capture | Not Started | 0 | No |
| Experiment Result Comparison | Not Started | 0 | No |

---

## Dependencies

### Blocks

- plan-00 (Foundation): Needed for WorkflowIR and storage
- plan-01 (MVP Queue): Needed for workflow execution
- plan-02 (CLI & Backends): Needed for CLI automation commands
- plan-03 (Glyphnova UI): Needed for UI automation integration
- plan-04 (Quality Loops): Needed for merge validation and quality checks
- plan-05 (Memory & Search): Needed for experiment result storage

### Unblocks

- plan-07: Autonomy & Metrics (depends on automation framework)

### Integration Points

- **plan-00 (Foundation)**: Uses WorkflowIR, storage layer, observability
- **plan-01 (MVP Queue)**: Schedules workflows through cron
- **plan-02 (CLI & Backends)**: CLI automation commands
- **plan-03 (Glyphnova UI)**: Automation dashboard
- **plan-04 (Quality Loops)**: Merge validation, quality metrics
- **plan-05 (Memory & Search)**: Experiment result storage

---

## Quality Gates

### ADR-0007 Quality Gates

1. **Cron Execution Isolation**: Cron jobs execute workflows with proper environment isolation
2. **Git Experiment Safety**: Git experiments run in isolated branches without affecting main code
3. **Merge Proposal Quality**: Merge proposals are generated with clear diff recommendations
4. **Refinement Auditability**: Manual refinement events are captured as auditable artifacts
5. **Experiment Result Tracking**: Experiment results are tracked and comparable
6. **Rollback Procedures**: Rollback procedures exist for failed experiments

### Critical Review Upstream Factors

1. **Cron Scheduler Accuracy**: Jobs execute within 1 second of scheduled time
2. **Git Isolation Completeness**: No experiment can affect main branch or other experiments
3. **Merge Policy Enforcement**: No merge succeeds without meeting policy requirements
4. **Refinement Link Integrity**: All refinements are linked to correct artifacts
5. **Result Comparison Correctness**: Comparison metrics are mathematically correct and deterministic
6. **Rollback Safety**: Rollback never leaves system in inconsistent state
7. **Scheduling Policy Determinism**: Same scheduling policy always compiles to same IR

---

## Next Steps

### Workflow for Execution

1. **Code Review**: Read ADR-0007 and related research plan findings
2. **Web Research**: Complete 5 research areas with evidence collection
3. **Update Plan**: Incorporate research findings into implementation plan
4. **Write Tests**: Start with unit tests for Phase 1 (Cron Job Scheduler)
5. **Implement Phase 1**: Build cron scheduler with test-driven development
6. **Verify Checkpoint 1**: Run Layer 1, 2, 3 tests, update plan with results
7. **Continue Phases**: Repeat test-first approach for phases 2-7
8. **All Checkpoints**: Verify each checkpoint with all applicable layers
9. **Update Plan**: Document all working code and verification results
10. **Proceed**: Mark plan complete and move to plan-07

### Starting Point

Begin with Phase 1 (Cron Job Scheduler):
1. Define cron expression parser
2. Implement scheduling engine (tokio-based)
3. Write unit tests for parsing and scheduling
4. Write property tests for scheduling determinism
5. Implement safety limits and retry policies
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
# Verify Phase 1 (Cron Job Scheduler)
cargo test --lib cron
cargo test --test cron_integration
cargo test --test cron_proptest

# Verify Phase 2 (Git Experimentation Framework)
cargo test --lib git_experiment
cargo test --test git_experiment_integration
cargo test --test git_experiment_proptest
cargo test --test git_experiment_e2e
```

### Run Automation Operations

```bash
# Schedule cron job
whitt-execution-engine schedule --workflow workflow.yml --cron "0 9 * * 1-5" --branch experiment-001

# List scheduled jobs
whitt-execution-engine schedule --list

# Cancel scheduled job
whitt-execution-engine schedule --cancel <job-id>

# Create merge proposal
whitt-execution-engine merge-proposal --branch experiment-001 --target main

# View merge proposal
whitt-execution-engine merge-proposal --view <proposal-id>

# Approve merge proposal
whitt-execution-engine merge-proposal --approve <proposal-id>

# Capture manual refinement
whitt-execution-engine refinement --add --proposal <proposal-id> --comment "Improve error handling"

# View experiment results
whitt-execution-engine experiment --results --workflow <workflow-id> --compare

# Rollback experiment
whitt-execution-engine experiment --rollback <experiment-id>

# Cleanup experiment
whitt-execution-engine experiment --cleanup <experiment-id>

# Compile scheduling policy
whitt-execution-engine compile-schedule --policy scheduling.yml
```

### Use implement.sh Script

```bash
# Start from beginning
./implement.sh start plan-06

# Resume from checkpoint
./implement.sh resume plan-06 cp3

# Check progress
./implement.sh status plan-06

# Generate progress report
./implement.sh report plan-06
```

---

## References

### Related Documents

- **ADR-0007**: [Cron execution git-branch experimentation and refining workflows](../roadmap/adr-0006-cron-git-refinement.yml)
- **Research Plan 05**: [Automation, autonomy, and metrics research](../roadmap/research-plan-04-automation-autonomy-metrics.yml)
- **Plan 00**: [Foundation Compiler Contract](phase-0-foundation/plan-00-foundation.md)
- **Plan 01**: [MVP Queue & Scheduler](phase-1-mvp-queue/plan-01-mvp-queue.md)
- **Plan 02**: [CLI & Backends](phase-2-cli-backends/plan-02-cli-backends.md)
- **Plan 03**: [Glyphnova UI](phase-3-glyphnova-ui/plan-03-glyphnova-ui.md)
- **Plan 04**: [Quality Loops](phase-4-quality-loops/plan-03-quality-loops.md)
- **Plan 05**: [Memory & Search](phase-5-memory-search/plan-04-memory-search.md)
- **Requirements**: [schema-consolidated-report.md](../requirements/schema-consolidated-report.md)

### Implementation References

- **tokio-cron-scheduler**: [docs.rs/tokio-cron-scheduler](https://docs.rs/tokio-cron-scheduler) - Rust cron scheduler
- **git-worktree**: [git-scm.com/docs/git-worktree](https://git-scm.com/docs/git-worktree) - Multiple working trees
- **GitHub Actions**: [docs.github.com/en/actions](https://docs.github.com/en/actions) - CI patterns
- **criterion**: [docs.rs/criterion](https://docs.rs/criterion) - Benchmark comparison
- **MLflow**: [mlflow.org](https://www.mlflow.org) - Experiment tracking
- **pre-commit**: [pre-commit.com](https://pre-commit.com) - Validation before merge
- **Semgrep**: [semgrep.dev](https://semgrep.dev) - Security and quality validation

### Tool and Plugin References

- **OpenCode Tools**: File operations, git operations, shell execution
- **bash tool**: Command execution for git operations
- **lsp_diagnostics**: Type checking and lint verification
- **glob tool**: File pattern matching for experiment discovery

---

**Last Updated**: 2026-03-27
**Status**: Not Started
