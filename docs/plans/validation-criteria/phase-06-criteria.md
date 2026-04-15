# Phase 06: Automation - Validation Criteria

**Phase Focus:** Cron jobs, git experiments, merge proposals, refinement artifacts, results tracking, rollback
**Entry Criteria:** Phases 00, 01, 02, 03, 04, and 05 complete
**Estimated Duration:** 3-4 weeks
**Blocking for:** Phases 07, 08

---

## Phase Overview

Phase 06 implements automation capabilities. This phase provides cron job scheduling, git experiment isolation, merge proposal generation, refinement artifact capture, results tracking and comparison, and rollback functionality. Automation enables workflows to run periodically and experiment with changes safely.

**Critical Success Factors:**
1. Cron jobs execute on schedule
2. Git experiments are isolated
3. Merge proposals are generated
4. Refinement captured as artifacts
5. Results tracked and comparable
6. Rollback works correctly

---

## Phase Entry Criteria

**Status:** Must be verified before starting Phase 06

**Verification Commands:**
```bash
# Verify Phases 00-05 exit
cargo test --test phase_00_integration -- --test-threads=1
cargo test --test phase_01_integration -- --test-threads=1
cargo test --test phase_02_integration -- --test-threads=1
cargo test --test phase_04_integration -- --test-threads=1
cargo test --test phase_05_integration -- --test-threads=1
```

**Prerequisites:**
- [ ] Phases 00-05 exit criteria verified (all 7 layers)
- [ ] Schema coverage 100% for phases 00-05
- [ ] ADR compliance verified for phases 00-05
- [ ] Cross-phase regression clean (Phases 00-05)
- [ ] Git repository initialized
- [ ] Cron scheduler ready

**Blocking Violations:**
- Unresolved Phase 00-05 failures
- Schema coverage < 100% for any phase
- Cross-phase regression detected

---

## Cron Jobs

**Requirement:** Cron jobs execute on schedule

### Cron Schedule Syntax

```
* * * * * command
│ │ │ │ │
│ │ │ │ └─ Day of week (0-6)
│ │ │ └─── Month (1-12)
│ │ └───── Day of month (1-31)
│ └─────── Hour (0-23)
└───────── Minute (0-59)
```

### Cron Features

1. **Fixed Schedule:** Execute at specific times
2. **Interval Schedule:** Execute every N minutes/hours/days
3. **Complex Schedule:** Combinations of schedules
4. **Timezone Support:** Execute in specific timezone
5. **Job Management:** Create, list, delete cron jobs

### Verification Commands

```bash
# Test cron jobs
cargo test --lib automation::tests::fixed_schedule
cargo test --lib automation::tests::interval_schedule
cargo test --lib automation::tests::complex_schedule
cargo test --lib automation::tests::timezone_support

# Test job management
cargo test --lib automation::tests::cron_job_create
cargo test --lib automation::tests::cron_job_list
cargo test --lib automation::tests::cron_job_delete

# Test execution
cargo test --lib automation::tests::cron_job_execution
```

### Pass Criteria

- [ ] All schedule types work correctly
- [ ] Jobs execute on schedule
- [ ] Timezone support works
- [ ] Job management works
- [ ] No missed executions

### Evidence Required

- Cron job test results
- Schedule execution logs
- Timezone test logs

---

## Git Experiments

**Requirement:** Git experiments are isolated

### Experiment Isolation

1. **Branch Creation:** Create experiment branch
2. **Workspace Isolation:** Isolate experiment workspace
3. **Revert on Failure:** Revert experiment on failure
4. **Merge on Success:** Merge experiment on success

### Experiment Workflow

```
1. Create experiment branch from main
2. Make changes in experiment branch
3. Run tests in experiment branch
4. If tests pass → merge to main
5. If tests fail → revert experiment
```

### Verification Commands

```bash
# Test git experiments
cargo test --lib automation::tests::experiment_branch_creation
cargo test --lib automation::tests::workspace_isolation
cargo test --lib automation::tests::revert_on_failure
cargo test --lib automation::tests::merge_on_success

# Test experiment safety
cargo test --lib automation::tests::experiment_safety
cargo test --lib automation::tests::experiment_rollback
```

### Pass Criteria

- [ ] Experiment branches created correctly
- [ ] Workspaces isolated
- [ ] Revert on failure works
- [ ] Merge on success works
- [ ] No corruption of main branch

### Evidence Required

- Git experiment test results
- Branch creation logs
- Isolation test logs

---

## Merge Proposals

**Requirement:** Merge proposals are generated

### Proposal Content

1. **Title:** Summary of changes
2. **Description:** Detailed description of changes
3. **Diff:** Diff of changes
4. **Test Results:** Test execution results
5. **Review Status:** Review status

### Proposal Generation

```markdown
# Merge Proposal: Add Feature X

## Description
Add feature X to improve Y

## Changes
- Added feature X
- Updated tests for feature X

## Test Results
- Unit tests: PASS
- Integration tests: PASS
- E2E tests: PASS

## Review Status
Pending review
```

### Verification Commands

```bash
# Test merge proposals
cargo test --lib automation::tests::proposal_generation
cargo test --lib automation::tests::proposal_content
cargo test --lib automation::tests::proposal_diff

# Test proposal submission
cargo test --lib automation::tests::proposal_submission
```

### Pass Criteria

- [ ] Proposals generated correctly
- [ ] Proposals include all required content
- [ ] Diff accurately reflects changes
- [ ] Test results included
- [ ] Proposals submitted successfully

### Evidence Required

- Merge proposal test results
- Proposal content samples
- Diff verification logs

---

## Refinement Artifacts

**Requirement:** Refinement captured as artifacts

### Artifact Types

1. **Code Artifacts:** Generated code
2. **Config Artifacts:** Configuration files
3. **Document Artifacts:** Documentation
4. **Result Artifacts:** Execution results
5. **Log Artifacts:** Execution logs

### Artifact Storage

```
.artifacts/
├── code/
│   ├── generated_code_1.rs
│   └── generated_code_2.rs
├── config/
│   └── config_1.yaml
├── document/
│   └── doc_1.md
├── result/
│   └── result_1.json
└── log/
    └── log_1.txt
```

### Verification Commands

```bash
# Test artifact capture
cargo test --lib automation::tests::code_artifact_capture
cargo test --lib automation::tests::config_artifact_capture
cargo test --lib automation::tests::document_artifact_capture
cargo test --lib automation::tests::result_artifact_capture
cargo test --lib automation::tests::log_artifact_capture

# Test artifact storage
cargo test --lib automation::tests::artifact_storage
cargo test --lib automation::tests::artifact_retrieval
```

### Pass Criteria

- [ ] All artifact types captured
- [ ] Artifacts stored correctly
- [ ] Artifacts retrievable
- [ ] No artifact loss
- [ ] Storage organized correctly

### Evidence Required

- Artifact capture test results
- Storage test logs
- Retrieval test logs

---

## Results Tracking

**Requirement:** Results tracked and comparable

### Result Data

1. **Execution ID:** Unique execution identifier
2. **Timestamp:** Execution timestamp
3. **Workflow:** Workflow executed
4. **Success:** Success/failure
5. **Duration:** Execution duration
6. **Output:** Execution output
7. **Metrics:** Execution metrics

### Result Comparison

```markdown
## Execution Comparison

| Metric | Execution A | Execution B | Delta |
|--------|-------------|-------------|-------|
| Success | true | true | - |
| Duration (s) | 10 | 12 | +20% |
| Output | "result1" | "result2" | Changed |
```

### Verification Commands

```bash
# Test result tracking
cargo test --lib automation::tests::result_tracking
cargo test --lib automation::tests::result_storage
cargo test --lib automation::tests::result_retrieval

# Test result comparison
cargo test --lib automation::tests::result_comparison
cargo test --lib automation::tests::result_delta_calculation
```

### Pass Criteria

- [ ] Results tracked correctly
- [ ] Results stored
- [ ] Results retrievable
- [ ] Results comparable
- [ ] Delta calculations correct

### Evidence Required

- Result tracking test results
- Comparison test logs
- Delta calculation logs

---

## Rollback

**Requirement:** Rollback works correctly

### Rollback Scenarios

1. **Manual Rollback:** User triggers rollback
2. **Automatic Rollback:** System triggers rollback on failure
3. **Partial Rollback:** Rollback specific changes
4. **Full Rollback:** Rollback all changes

### Rollback Process

```
1. Identify changes to rollback
2. Revert changes in git
3. Restore artifacts
4. Restore configuration
5. Verify rollback success
```

### Verification Commands

```bash
# Test rollback
cargo test --lib automation::tests::manual_rollback
cargo test --lib automation::tests::automatic_rollback
cargo test --lib automation::tests::partial_rollback
cargo test --lib automation::tests::full_rollback

# Test rollback safety
cargo test --lib automation::tests::rollback_safety
cargo test --lib automation::tests::rollback_verification
```

### Pass Criteria

- [ ] All rollback types work
- [ ] Rollback reverts changes correctly
- [ ] Artifacts restored
- [ ] Configuration restored
- [ ] Rollback verification succeeds
- [ ] No data loss during rollback

### Evidence Required

- Rollback test results
- Rollback verification logs
- Safety test logs

---

## Phase Exit Criteria

### Layer 1: Unit Tests

**Commands:**
```bash
cargo test --lib -- --test-threads=1
```

**Evidence:**
- All Phase 06 unit tests pass
- Test coverage >= 90%
- No clippy warnings

### Layer 2: Integration Tests

**Commands:**
```bash
cargo test --test '*' -- --test-threads=1
```

**Evidence:**
- All Phase 06 integration tests pass
- Automation integrates correctly

### Layer 3: Property Tests

**Commands:**
```bash
PROPTEST_NUMBER_OF_TESTS=100 cargo test --lib property_based
```

**Evidence:**
- Experiment isolation invariants hold
- Rollback safety invariants hold

### Layer 4: E2E Tests

**Commands:**
```bash
# Execute automation workflows
cargo run --bin agentsdk -- run examples/workflows/cron_job.yaml
cargo run --bin agentsdk -- run examples/workflows/git_experiment.yaml
cargo run --bin agentsdk -- run examples/workflows/rollback.yaml
```

**Evidence:**
- Cron jobs execute on schedule
- Git experiments isolated
- Rollback works correctly

### Layer 5: System Log Validation

**Commands:**
```bash
# Verify automation scope logs
cargo run --bin agentsdk -- run examples/workflows/automation.yaml 2>&1 | \
  jq -e 'select(.scope == "automation")'
```

**Evidence:**
- Automation scope logs emitted
- Logs include timestamp, scope, event, message

### Layer 6: Live CLI Verification

**Commands:**
```bash
# Test automation commands
cargo run --bin agentsdk -- automation cron create --schedule "*/5 * * * *" --workflow test.yaml
cargo run --bin agentsdk -- automation cron list
cargo run --bin agentsdk -- automation cron delete <job_id>
cargo run --bin agentsdk -- automation experiment start --name test_exp
cargo run --bin agentsdk -- automation rollback <experiment_id>
```

**Evidence:**
- Automation commands work
- Error handling works

### Layer 7: Benchmark Performance

**Commands:**
```bash
cargo bench --bench phase_06_benchmarks
```

**Evidence:**
- Automation performance acceptable
- Rollback completes in reasonable time
- No performance regression > 10%

---

## Cross-Phase Regression Tests

**Commands:**
```bash
# Run all prior phase tests
cargo test --test phase_00_integration -- --test-threads=1
cargo test --test phase_01_integration -- --test-threads=1
cargo test --test phase_02_integration -- --test-threads=1
cargo test --test phase_04_integration -- --test-threads=1
cargo test --test phase_05_integration -- --test-threads=1
```

**Pass Criteria:**
- [ ] All Phase 00 tests still pass
- [ ] All Phase 01 tests still pass
- [ ] All Phase 02 tests still pass
- [ ] All Phase 04 tests still pass
- [ ] All Phase 05 tests still pass

---

## Schema Coverage Audit

**Requirement:** 100% of Phase 06-owned fields implemented

### Phase 06-Owned Fields

**CronSchema:**
- schedule
- timezone
- workflow
- enabled

**GitExperimentSchema:**
- experiment_id
- branch_name
- base_branch
- changes

**MergeProposalSchema:**
- proposal_id
- title
- description
- diff
- test_results

**RefinementSchema:**
- artifact_id
- artifact_type
- content
- created_at

**ResultSchema:**
- execution_id
- timestamp
- workflow
- success
- duration
- output
- metrics

**RollbackSchema:**
- rollback_id
- experiment_id
- changes_reverted
- artifacts_restored

### Verification Commands

```bash
cargo run --bin schema_audit -- --phase 6 --output coverage_report.md
```

**Expected Output:**
- 100% coverage for Phase 06-owned fields
- No unimplemented fields

---

## ADR Compliance

**Relevant ADRs:**
- ADR-011: Automation Architecture
- ADR-012: Experiment Safety

### Verification Commands

```bash
cargo run --bin adr_compliance -- --phase 6
```

**Expected Output:**
- All Phase 06-relevant ADR constraints satisfied
- No outstanding ADR TODOs

---

## Anti-Goal-Drift Checklist

**Run after phase completion:**

1. **Requirements Drift:**
   - [ ] Implementation matches Phase 06 requirements
   - [ ] No features from later phases added

2. **Architecture Drift:**
   - [ ] Automation matches ADR-011
   - [ ] Experiments match ADR-012

3. **Scope Creep:**
   - [ ] Only automation features implemented
   - [ ] No autonomy features added

---

## Evidence Storage

**Location:** `results/phase_06/`

**Contents:**
- `unit_test_results.json`
- `integration_test_output.log`
- `property_test_results.json`
- `e2e_execution_logs/` (automation workflows)
- `system_log_samples.json` (automation scope)
- `cli_verification/` (automation commands)
- `benchmarks/` (automation performance)
- `cron_logs/` (cron job execution logs)
- `git_experiments/` (experiment logs)
- `merge_proposals/` (generated proposals)
- `refinement_artifacts/` (captured artifacts)
- `results_tracking/` (result comparisons)
- `rollback_logs/` (rollback execution logs)
- `schema_coverage_report.md`
- `adr_compliance_report.md`
- `anti_goal_drift_checklist.md`
- `phase_00_regression/`
- `phase_01_regression/`
- `phase_02_regression/`
- `phase_04_regression/`
- `phase_05_regression/`

---

## Blocking Issues

**Cannot exit Phase 06 if:**
- Any verification layer fails
- Cron jobs don't execute on schedule
- Git experiments not isolated
- Merge proposals not generated
- Refinement not captured as artifacts
- Results not tracked
- Rollback doesn't work
- Any prior phase regression detected

---

**End of Phase 06 Criteria**
