# QA Test Cases — Phase 05: Automation

**Schema**: `docs/schema/unified-workflow-schema.yml` (805 lines)
**Test ID Prefix**: P05
**Date**: 2026-04-26

---

## Test Case Summary Table

| Test ID | QA Area | Description | Command | Expected Result |
|----------|----------|-------------|----------|----------------|
| P05-001 | Cron Scheduler | Cron expression parsing | cargo test --lib automation::cron::parser::test_cron_parsing | All cron expressions parsed correctly |
| P05-002 | Cron Scheduler | Schedule validation | cargo test --lib automation::cron::validator::test_validation | Invalid schedules rejected |
| P05-003 | Cron Scheduler | Tokio scheduler execution | cargo test --lib automation::cron::scheduler::test_execution | Workflows execute on schedule |
| P05-004 | Cron Scheduler | Schedule persistence | cargo test --lib automation::cron::persistence::test_persistence | Schedules persist across restarts |
| P05-005 | Cron Scheduler | Next run calculation | cargo test --lib automation::cron::scheduler::test_next_run | Next run calculated accurately |
| P05-006 | Git Experiment Framework | Branch isolation (ADR-0007) | cargo test --lib automation::experiment::isolation::test_branch_isolation | Experiments never run on main |
| P05-007 | Git Experiment Framework | Worktree management | cargo test --lib automation::experiment::git_operations::test_worktrees | Worktrees created and cleaned up |
| P05-008 | Git Experiment Framework | Experiment lifecycle | cargo test --test git_experiment_lifecycle -- --test-threads=1 --nocapture | Lifecycle complete without errors |
| P05-009 | Merge Proposal Generation | Diff generation | cargo test --lib automation::merge::generator::test_diff_generation | Accurate diffs generated |
| P05-010 | Merge Proposal Generation | Artifact location (ADR-0007) | cargo test --lib automation::merge::artifacts::test_location | Proposals written to `./workspace/merge-proposals/` |
| P05-011 | Merge Proposal Generation | No auto-commit (ADR-0007) | cargo test --test merge_proposal_no_auto_commit -- --test-threads=1 --nocapture | Proposals never auto-committed |
| P05-012 | Merge Proposal Generation | Confidence scoring | cargo test --lib automation::merge::confidence::test_scoring | Confidence computed from results |
| P05-013 | Manual Refinement Capture | Event capture | cargo test --lib automation::refinement::capture::test_capture | All refinements captured |
| P05-014 | Manual Refinement Capture | Immutable event log | cargo test --lib automation::refinement::events::test_immutability | Event log cannot be modified |
| P05-015 | Manual Refinement Capture | Artifact linking | cargo test --lib automation::refinement::linking::test_linking | Refinements linked to proposals |
| P05-016 | Experiment Result Tracking | Result storage | cargo test --lib automation::tracking::storage::test_storage | Results stored to `./workspace/experiments/` |
| P05-017 | Experiment Result Tracking | Result comparison | cargo test --lib automation::tracking::comparison::test_comparison | Results compared to baseline |
| P05-018 | Experiment Result Tracking | Visualization | cargo test --lib automation::tracking::visualization::test_visualization | Charts/tables generated |
| P05-019 | Rollback & Cleanup | Rollback procedures | cargo test --lib automation::rollback::procedures::test_rollback | Rollback to pre-experiment state |
| P05-020 | Rollback & Cleanup | Branch deletion | cargo test --lib automation::rollback::cleanup::test_branch_deletion | Experiment branches deleted |
| P05-021 | Rollback & Cleanup | Artifact cleanup | cargo test --lib automation::rollback::cleanup::test_artifact_cleanup | All artifacts cleaned up |
| P05-022 | Rollback & Cleanup | Cleanup logging | cargo test --lib automation::rollback::cleanup::test_logging | Cleanup operations logged |
| P05-023 | Scheduling Policy Compiler | Policy compilation | cargo test --lib automation::compiler::scheduling_node::test_compilation | Cron policies compiled to WorkflowIR |
| P05-024 | Scheduling Policy Compiler | Runtime no parsing | cargo test --test scheduling_policy_compiler_no_runtime_parsing -- --test-threads=1 --nocapture | Runtime uses pre-compiled metadata |
| P05-025 | Automation CLI | Schedule commands | cargo test --test cli_schedule_commands -- --test-threads=1 --nocapture | Schedule add/remove/list/pause/resume work |
| P05-026 | Automation CLI | Experiment commands | cargo test --test cli_experiment_commands -- --test-threads=1 --nocapture | Experiment start/stop/list work |
| P05-027 | Automation CLI | Merge/refine/rollback commands | cargo test --test cli_merge_refine_rollback -- --test-threads=1 --nocapture | All subcommands work correctly |
| P05-028 | Automation UI Integration | Experiment tracking UI | cargo test --test ui_experiment_tracking -- --test-threads=1 --nocapture | UI displays experiment status |
| P05-029 | Automation UI Integration | Merge proposal UI | cargo test --test ui_merge_proposals -- --test-threads=1 --nocapture | UI displays proposals with actions |
| P05-030 | Integration | End-to-end automation workflow | cargo run --bin whitt automation examples/automation-workflow.yaml | Automation workflow executes successfully |

---

## Detailed Test Cases

### P05-001: Cron Scheduler - Cron Expression Parsing

**Description**: Verify cron expressions are parsed correctly
**QA Area**: Area 1 - Cron Scheduler
**Priority**: P0
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib automation::cron::parser::test_cron_parsing -- --test-threads=1
```

**Expected Result**:
- Minute field parsed correctly (0-59)
- Hour field parsed correctly (0-23)
- Day of month field parsed correctly (1-31)
- Month field parsed correctly (1-12)
- Day of week field parsed correctly (0-6, 0=Sunday)
- Special cron expressions parsed (*, */5, 0,1-5)
- Invalid cron expressions rejected with clear error
- Next run times calculated correctly

**Verification**:
```bash
# Check test output
cargo test --lib automation::cron::parser::test_cron_parsing -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P05-006: Git Experiment Framework - Branch Isolation (ADR-0007)

**Description**: Verify experiments never run on main branch
**QA Area**: Area 2 - Git Experiment Framework
**Priority**: P0
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib automation::experiment::isolation::test_branch_isolation -- --test-threads=1
```

**Expected Result**:
- Experiment branches created from current HEAD
- Experiment branches never are "main"
- Worktrees create isolated environments
- Main branch remains unmodified during experiment
- ADR-0007 constraint enforced: branch isolation

**Verification**:
```bash
# Check test output
cargo test --lib automation::experiment::isolation::test_branch_isolation -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P05-010: Merge Proposal Generation - Artifact Location (ADR-0007)

**Description**: Verify merge proposals written to `./workspace/merge-proposals/`
**QA Area**: Area 3 - Merge Proposal Generation
**Priority**: P1
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib automation::merge::artifacts::test_location -- --test-threads=1
```

**Expected Result**:
- Proposals directory: `./workspace/merge-proposals/`
- Proposal files: proposal.json, diff.patch, metadata.json
- No proposals written elsewhere
- No auto-commit to git (ADR-0007 constraint)
- Artifact directory structure enforced

**Verification**:
```bash
# Check test output
cargo test --lib automation::merge::artifacts::test_location -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P05-011: Merge Proposal Generation - No Auto-Commit (ADR-0007)

**Description**: Verify merge proposals are outputs, not auto-committed
**QA Area**: Area 3 - Merge Proposal Generation
**Priority**: P1
**Test Type**: Integration

**Test Command**:
```bash
cargo test --test merge_proposal_no_auto_commit -- --test-threads=1 --nocapture
```

**Expected Result**:
- Merge proposal generation does NOT execute git commit
- Proposal written to `./workspace/merge-proposals/` only
- No automatic git operations performed
- Manual intervention required for merge
- ADR-0007 constraint enforced: outputs only, no auto-commit

**Verification**:
```bash
# Check test output
cargo test --test merge_proposal_no_auto_commit -- --test-threads=1 --nocapture 2>&1 | grep -q "test result: ok"
```

---

### P05-014: Manual Refinement Capture - Immutable Event Log

**Description**: Verify event log in `./workspace/refinements/` cannot be modified
**QA Area**: Area 4 - Manual Refinement Capture
**Priority**: P1
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib automation::refinement::events::test_immutability -- --test-threads=1
```

**Expected Result**:
- Event log location: `./workspace/refinements/`
- Event files: event.json with what, why, who, when, linked_artifacts
- Events cannot be modified after creation
- Events have write-once semantics
- Immutable timestamps recorded in events
- Event structure validated before writing

**Verification**:
```bash
# Check test output
cargo test --lib automation::refinement::events::test_immutability -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P05-023: Scheduling Policy Compiler - Policy Compilation

**Description**: Verify cron policies compiled to WorkflowIR, not interpreted at runtime
**QA Area**: Area 7 - Scheduling Policy Compiler
**Priority**: P0
**Test Type**: Unit

**Test Command**:
```bash
cargo test --lib automation::compiler::scheduling_node::test_compilation -- --test-threads=1
```

**Expected Result**:
- Cron policies compiled to SchedulingNode in WorkflowIR
- Cron expression parsed and validated at compilation time
- Runtime scheduler uses pre-compiled scheduling metadata
- No cron expression parsing at execution time
- WorkflowIR serialization includes compiled scheduling metadata
- Unit tests verify compilation output structure
- ADR-0007 constraint enforced: compiled, not interpreted

**Verification**:
```bash
# Check test output
cargo test --lib automation::compiler::scheduling_node::test_compilation -- --test-threads=1 2>&1 | grep -q "test result: ok"
```

---

### P05-024: Scheduling Policy Compiler - Runtime No Parsing

**Description**: Verify runtime scheduler never parses cron expressions
**QA Area**: Area 7 - Scheduling Policy Compiler
**Priority**: P0
**Test Type**: Integration

**Test Command**:
```bash
cargo test --test scheduling_policy_compiler_no_runtime_parsing -- --test-threads=1 --nocapture
```

**Expected Result**:
- Scheduler uses pre-compiled scheduling metadata
- No cron parser called at runtime
- Scheduling logic uses compiled data structures
- Runtime performance improved (no parsing overhead)
- ADR-0007 constraint enforced: compiled, not interpreted

**Verification**:
```bash
# Check test output
cargo test --test scheduling_policy_compiler_no_runtime_parsing -- --test-threads=1 --nocapture 2>&1 | grep -q "test result: ok"
```

---

### P05-025: Automation CLI - Schedule Commands

**Description**: Verify schedule management commands work correctly
**QA Area**: Area 8 - Automation CLI
**Priority**: P0
**Test Type**: Integration

**Test Command**:
```bash
cargo test --test cli_schedule_commands -- --test-threads=1 --nocapture
```

**Expected Result**:
- `whitt schedule add` adds new schedule
- `whitt schedule list` lists all schedules
- `whitt schedule remove <schedule-id>` removes schedule
- `whitt schedule pause <schedule-id>` pauses schedule
- `whitt schedule resume <schedule-id>` resumes schedule
- Commands validate arguments before execution
- Clear error messages for invalid inputs
- Accurate help text for all commands
- Proper exit codes (0 success, non-zero errors)

**Verification**:
```bash
# Check test output
cargo test --test cli_schedule_commands -- --test-threads=1 --nocapture 2>&1 | grep -q "test result: ok"
```

---

### P05-030: Integration - End-to-End Automation Workflow

**Description**: Verify complete automation workflow executes end-to-end
**QA Area**: Area 8/9 - Integration
**Priority**: P0
**Test Type**: E2E

**Test Command**:
```bash
cargo run --bin whitt automation examples/automation-workflow.yaml
```

**Expected Result**:
- Workflow loads successfully from YAML
- Cron schedule created and registered
- Experiment executes at scheduled time
- Merge proposal generated and written to `./workspace/merge-proposals/`
- Manual refinement captured when proposal reviewed
- All automation artifacts created in correct locations
- No errors or panics
- Exit code is 0
- Workflow completes successfully

**Verification**:
```bash
# Check execution succeeded
if cargo run --bin whitt automation examples/automation-workflow.yaml; then
  echo "E2E test PASSED"
else
  echo "E2E test FAILED"
  exit 1
fi

# Check artifacts exist
ls -lh workspace/merge-proposals/
ls -lh workspace/refinements/
ls -lh workspace/experiments/
```

---

## Performance Test Cases

### P05-PERF-001: Experiment Creation Time

**Description**: Verify experiment creation time meets < 5s target
**Priority**: P0
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench experiment_creation_time -- --test-threads=1
```

**Expected Result**:
- p50 creation time < 5s
- p95 creation time < 10s
- No significant regression from baseline

**Verification**:
```bash
# Parse benchmark results
cargo test --bench experiment_creation_time -- --test-threads=1 2>&1 | grep -q "p50.*< 5s"
```

---

### P05-PERF-002: Merge Proposal Generation

**Description**: Verify merge proposal generation time meets < 30s target
**Priority**: P1
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench merge_proposal_generation -- --test-threads=1
```

**Expected Result**:
- p50 generation time < 30s
- p95 generation time < 60s
- No significant regression from baseline

**Verification**:
```bash
# Parse benchmark results
cargo test --bench merge_proposal_generation -- --test-threads=1 2>&1 | grep -q "p50.*< 30s"
```

---

### P05-PERF-003: Rollback & Cleanup

**Description**: Verify rollback & cleanup time meets < 60s target
**Priority**: P1
**Test Type**: Benchmark

**Test Command**:
```bash
cargo test --bench rollback_cleanup -- --test-threads=1
```

**Expected Result**:
- p50 rollback time < 60s
- p95 rollback time < 120s
- No significant regression from baseline

**Verification**:
```bash
# Parse benchmark results
cargo test --bench rollback_cleanup -- --test-threads=1 2>&1 | grep -q "p50.*< 60s"
```

---

## Mock Strategy Notes

### Git Experiments (git2 crate)
- Use temporary git repository for testing git operations
- Mock branch creation, deletion, and worktree management
- Test branch isolation enforcement
- Test cleanup procedures

### HTTP Clients (reqwest)
- Mock HTTP responses for DuckDuckGo and Brave APIs
- Test rate limiting with time control
- Test policy gate enforcement without network calls
- Mock API rate limit headers

### Cron Scheduler (tokio-cron-scheduler)
- Mock tokio scheduler for deterministic testing
- Test schedule accuracy with time control
- Test missed schedule handling

### Merge Proposals (diff)
- Mock diff generation for testing
- Mock git diff output
- Test artifact location enforcement
- Test no-auto-commit constraint

---

**End of QA Test Cases for Phase 05**
