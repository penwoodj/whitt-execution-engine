# Incremental Validation Criteria Framework

**Version:** 2.0
**Last Updated:** 2026-04-06
**Purpose:** Prevent goal drift and ensure systematic validation of AgentSDK Execution Engine implementation

---

## Overview

This framework defines a rigorous, multi-layered validation system for the AgentSDK Execution Engine. Each phase must pass all 7 verification layers before being considered complete. This prevents partial implementations, scope creep, and architectural drift.

### Core Principles

1. **Evidence-First Validation**: Every claim must have test output, logs, or metrics as evidence
2. **Zero-Drift Enforcement**: Continuous comparison against original ADRs and schema
3. **Cumulative Validation**: Each phase includes regression tests from all prior phases
4. **Measurable Acceptance**: Binary pass/fail criteria with clear test commands

---

## 7 Verification Layers

### Layer 1: Unit Tests

**Scope:** Individual functions, methods, and data structures in isolation

**Evidence Required:**
- `cargo test --lib` output showing 100% of phase-specific unit tests passing
- Code coverage report (`cargo tarpaulin --out Html`) showing >90% coverage for modified modules
- No panics in unit test output

**Validation Commands:**
```bash
cargo test --lib -- --test-threads=1
cargo tarpaulin --out Html --exclude-files '*/tests/*' --workspace
```

**Pass Criteria:**
- All unit tests in phase scope pass
- No warnings from clippy on tested modules (`cargo clippy -- -D warnings`)
- Coverage >= 90% for new/modified code

---

### Layer 2: Integration Tests

**Scope:** Module interactions, component communication, data flow between units

**Evidence Required:**
- `cargo test --test '*'` output showing all integration tests passing
- Test logs showing successful component communication
- No race conditions or deadlocks detected

**Validation Commands:**
```bash
cargo test --test '*' -- --test-threads=1 --nocapture
RUST_LOG=debug cargo test --test 'integration_*' -- --nocapture
```

**Pass Criteria:**
- All integration tests in phase scope pass
- Communication patterns match ADR specifications
- No thread panic or deadlock in concurrent tests

---

### Layer 3: Property-Based Tests

**Scope:** Invariants that must hold for all possible inputs

**Evidence Required:**
- Proptest output showing 1000 successful iterations per property
- Shrunk failure cases (if any) analyzed and resolved
- Coverage of edge cases beyond example inputs

**Validation Commands:**
```bash
cargo test --lib property_based -- --test-threads=1
PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based
```

**Pass Criteria:**
- All property tests pass for 1000 iterations
- No shrinking required (no failures)
- Invariants cover all data structure variants used in phase

**Critical Invariants to Test:**
- Round-trip serialization: `parse(serialize(x)) == x`
- Scope closure: variables resolve to correct scope
- DAG validity: no cycles after any mutation
- State machine: only valid state transitions possible

---

### Layer 4: End-to-End (E2E) Tests

**Scope:** Complete workflows from YAML input to CLI/backend output

**Evidence Required:**
- Execution logs showing workflow completion
- Output artifacts matching expected results
- System resource usage within acceptable bounds

**Validation Commands:**
```bash
# Execute example workflows
for workflow in examples/workflows/*.yaml; do
  cargo run --bin agentsdk -- run "$workflow" > /tmp/e2e_output.log
  grep -q "Workflow completed successfully" /tmp/e2e_output.log || exit 1
done
```

**Pass Criteria:**
- All phase-specific example workflows execute successfully
- Output files exist and are non-empty
- No memory leaks in long-running workflows
- Execution time within 2x expected baseline

---

### Layer 5: System Log Validation

**Scope:** Structural logging, scope correctness, telemetry completeness

**Evidence Required:**
- Log samples from all 9 scopes showing correct formatting
- Metrics export showing all required fields populated
- No log level violations (debug logs in release builds)

**Validation Commands:**
```bash
# Check log structure
cargo run --bin agentsdk -- run examples/workflows/test.yaml 2>&1 | \
  jq -e 'select(.scope == "pipeline" or .scope == "step" or .scope == "model")'

# Verify metrics export
curl -s http://localhost:9090/metrics | grep -q 'agentsdk_pipeline_duration_seconds'
```

**Pass Criteria:**
- All 9 scopes (pipeline, step, model, tool, backend, ui, system, automation, autonomy) emit logs
- Logs include timestamp, scope, workflow_id, and message
- Metrics include pipeline_duration, step_duration, model_tokens, tool_calls
- No missing required fields in structured logs

---

### Layer 6: Live CLI Verification

**Scope:** User-facing command behavior, error handling, help text

**Evidence Required:**
- Screenshots or terminal output showing all CLI subcommands working
- Error messages displayed for invalid inputs
- Help text is current and accurate

**Validation Commands:**
```bash
# Test all subcommands
cargo run --bin agentsdk -- --help
cargo run --bin agentsdk run --help
cargo run --bin agentsdk queue --help
cargo run --bin agentsdk validate --help

# Test error handling
cargo run --bin agentsdk run nonexistent.yaml
! cargo run --bin agentsdk run invalid.yaml
```

**Pass Criteria:**
- All subcommands display help without crashing
- Invalid inputs produce clear error messages with exit code != 0
- Error messages reference relevant documentation
- Progress indicators work for long-running operations

---

### Layer 7: Benchmark Performance

**Scope:** Performance against baseline, resource utilization, scalability

**Evidence Required:**
- Criterion benchmark results showing <=10% regression from baseline
- Memory profiles showing no leaks
- Latency percentiles (p50, p95, p99) within SLA

**Validation Commands:**
```bash
cargo bench --bench phase_X_benchmarks
cargo flamegraph --bin agentsdk -- run examples/workflows/stress_test.yaml

# Check for regressions
cargo bench --bench phase_X_benchmarks | grep -q "no significant change detected"
```

**Pass Criteria:**
- No performance regression >10% from established baseline
- Memory usage grows linearly with workflow size, not exponentially
- p95 latency < 2x p50 latency (consistent performance)
- Benchmarks complete within timeout (5 minutes max)

---

## Phase Entry Criteria

**Definition:** What must be true BEFORE starting a phase

**Checklist:**
- [ ] Previous phase exit criteria verified
- [ ] Cross-phase regression tests from all prior phases passing
- [ ] ADRs reviewed and understood for phase scope
- [ ] Development environment updated to latest dependencies
- [ ] Test infrastructure ready (fixtures, mocks, harnesses)
- [ ] Documentation templates created for phase artifacts

**Blocking Violations (cannot proceed):**
- Unresolved failures from previous phase
- Undefined ADR constraints for phase requirements
- Missing test infrastructure for verification layers

---

## Task-Level Acceptance Criteria

**Definition:** Measurable criteria for individual tasks within a phase

**Per-Task Checklist:**
- [ ] Unit tests for the task pass
- [ ] Integration tests for the task pass
- [ ] Code review approved
- [ ] Clippy warnings resolved
- [ ] Documentation updated
- [ ] Commit message follows conventional commit format

**Evidence Collection:**
```bash
# After each task completion
git log -1 --format='%H %s'
cargo test --lib ${task_name}::tests
cargo clippy -- -D warnings
```

---

## Checkpoint Gate Criteria

**Definition:** Must pass after completing task groups (typically 3-5 tasks)

**Checkpoint Actions:**
1. Run full verification layer suite
2. Update cumulative-progress.md with current status
3. Run anti-goal-drift checklist
4. Architect review of architecture drift

**Blocking Checkpoint Failures:**
- Any verification layer fails
- Architecture drift detected without ADR update
- Schema coverage < 90% for phase-owned fields

---

## Phase Exit Criteria

**Definition:** Must pass ALL 7 layers before phase is "complete"

**Mandatory Evidence:**
- [ ] **Layer 1 (Unit)**: All unit tests pass, coverage >= 90%
- [ ] **Layer 2 (Integration)**: All integration tests pass, no deadlocks
- [ ] **Layer 3 (Property)**: All properties hold for 1000 iterations
- [ ] **Layer 4 (E2E)**: All example workflows execute successfully
- [ ] **Layer 5 (System Log)**: All 9 scopes emit valid structured logs
- [ ] **Layer 6 (Live CLI)**: All CLI commands work with valid/invalid inputs
- [ ] **Layer 7 (Benchmark)**: Performance within 10% of baseline

**Cumulative Evidence:**
- [ ] Cross-phase regression tests (all prior phases) pass
- [ ] Schema coverage audit: 100% of phase-owned fields implemented
- [ ] ADR compliance audit: all phase-relevant ADRs satisfied
- [ ] Anti-goal-drift checklist: no drift detected
- [ ] Documentation: phase documentation complete and reviewed

**Blocking Exit Violations:**
- Any single verification layer fails (no partial phase completions)
- Unapproved architecture drift from ADRs
- Missing schema field implementations
- Incomplete documentation

---

## Cross-Phase Regression Tests

**Definition:** Tests from prior phases that must still pass

**Regression Test Suite:**
```bash
# Run all prior phase tests after each phase completion
for phase in $(seq 0 $((CURRENT_PHASE - 1))); do
  cargo test --test "phase_${phase}_integration" -- --test-threads=1
done
```

**Regression Categories:**
1. **Schema Regression**: Existing fields still parse correctly
2. **Behavior Regression**: Existing workflows still execute correctly
3. **Performance Regression**: No slowdown in existing features
4. **Log Regression**: Existing scopes still emit logs

**Failure Handling:**
- Any regression failure blocks phase exit
- Must fix regression before proceeding
- Document regression fix in commit message

---

## Schema Coverage Audit

**Definition:** Ensure all schema fields owned by phase are implemented

**Audit Process:**
1. Extract schema fields for phase from domain definition
2. Search codebase for field implementations
3. Mark field as "implemented" if:
   - Field parses from YAML
   - Field validates according to constraints
   - Field is used in execution logic
   - Field is tested

**Audit Command:**
```bash
# Generate audit report
cargo run --bin schema_audit -- --phase $PHASE_NUMBER --output coverage_report.md
```

**Pass Criteria:**
- 100% of phase-owned fields implemented
- No "unimplemented" fields in codebase
- All fields have corresponding tests

**Audit Report Format:**
```markdown
| Field | Schema Domain | Implemented | Tested | Line Count |
|-------|--------------|-------------|--------|------------|
| step.retry.max_attempts | RetrySchema | ✅ | ✅ | 23 |
| step.retry.backoff_ms | RetrySchema | ✅ | ✅ | 18 |
| workflow.env | WorkflowSchema | ❌ | ❌ | 0 |
```

---

## ADR Constraint Compliance

**Definition:** Ensure all ADR constraints are satisfied

**ADR Compliance Categories:**
1. **Architectural Constraints**: Module boundaries, dependency rules
2. **Performance Constraints**: Latency limits, resource caps
3. **Security Constraints**: Input validation, sandboxing
4. **Usability Constraints**: Error messages, logging scopes

**Compliance Verification:**
```bash
# Run ADR compliance checks
cargo run --bin adr_compliance -- --phase $PHASE_NUMBER
```

**Pass Criteria:**
- All phase-relevant ADR constraints satisfied
- No "TODO" or "FIXME" comments for ADR requirements
- Architecture matches ADR diagrams

---

## Anti-Goal-Drift Detection Methodology

**Definition:** Systematic detection of scope and architectural drift

### Drift Detection Triggers

**Run anti-goal-drift checklist when:**
- Completing a phase
- Completing a checkpoint
- Adding new ADR
- Modifying schema
- Performance regression detected

### Drift Categories

**1. Requirements Drift:**
- Symptom: Building features not in original spec
- Detection: Compare implementation against phase requirements
- Action: Document drift, get approval, or remove feature

**2. Architecture Drift:**
- Symptom: Module structure deviates from ADR
- Detection: Architecture review, dependency analysis
- Action: Update ADR or refactor to match

**3. Scope Creep:**
- Symptom: Adding features for "future" phases
- Detection: Phase boundary review
- Action: Remove or defer to correct phase

**4. Performance Drift:**
- Symptom: Failing to meet performance targets
- Detection: Benchmark regression
- Action: Optimize or update performance ADR

**5. Testing Drift:**
- Symptom: Test coverage decreasing
- Detection: Coverage reports, test counts
- Action: Add tests to meet coverage targets

**6. Documentation Drift:**
- Symptom: Code changes not reflected in docs
- Detection: Doc review, code-to-doc sync
- Action: Update documentation

**7. Dependency Drift:**
- Symptom: Adding unnecessary dependencies
- Detection: Cargo.lock review
- Action: Remove or justify dependency

### Drift Response Protocol

**Minor Drift (<5% impact):**
- Document in drift log
- Get architectural approval
- Update ADR if necessary

**Major Drift (>5% impact):**
- Schedule architecture review
- Create drift remediation plan
- Get stakeholder approval

**Critical Drift (blocks phase exit):**
- Immediate rollback
- Root cause analysis
- Process review to prevent recurrence

---

## Evidence Requirements

**Rule:** No assertion without evidence

### Evidence Collection

**Per Verification Layer:**

1. **Unit Tests:**
   - Save: `cargo test --lib -- --test-threads=1 --output-format json`
   - Store: `results/phase_$PHASE/unit_test_results.json`

2. **Integration Tests:**
   - Save: `cargo test --test '*' -- --test-threads=1 --nocapture`
   - Store: `results/phase_$PHASE/integration_test_output.log`

3. **Property Tests:**
   - Save: `PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based`
   - Store: `results/phase_$PHASE/property_test_results.json`

4. **E2E Tests:**
   - Save: Workflow execution logs
   - Store: `results/phase_$PHASE/e2e_execution_logs/`

5. **System Logs:**
   - Save: JSON log samples
   - Store: `results/phase_$PHASE/system_log_samples.json`

6. **CLI Verification:**
   - Save: Terminal output screenshots
   - Store: `results/phase_$PHASE/cli_verification/`

7. **Benchmarks:**
   - Save: Criterion HTML reports
   - Store: `results/phase_$PHASE/benchmarks/`

### Evidence Storage Structure

```
results/
  phase_00/
    unit_test_results.json
    integration_test_output.log
    property_test_results.json
    e2e_execution_logs/
    system_log_samples.json
    cli_verification/
    benchmarks/
    schema_coverage_report.md
    adr_compliance_report.md
    anti_goal_drift_checklist.md
  phase_01/
    ...
```

### Evidence Retention

**Retention Period:**
- All evidence retained for project lifetime
- Archived after phase completion to save disk space
- Compressed logs older than 6 months

**Evidence Format:**
- Machine-readable JSON for automated analysis
- Human-readable markdown for review
- Raw logs for debugging

---

## Verification Execution Order

**Recommended Order:** Run verification layers in this order

1. **Unit Tests** (fastest, catches basic issues)
2. **Integration Tests** (fast, catches interaction issues)
3. **Property Tests** (slower, catches invariant violations)
4. **System Log Validation** (medium, requires runtime)
5. **Live CLI Verification** (medium, requires manual verification)
6. **E2E Tests** (slower, catches end-to-end issues)
7. **Benchmarks** (slowest, catches performance issues)

**Stop on First Failure:**
- Abort layer execution on first test failure
- Fix issues before proceeding to next layer
- Reduces wasted time on downstream layers

---

## Quick Reference

**Phase Complete When:**
- All 7 verification layers pass
- Cross-phase regression clean
- Schema coverage 100%
- ADR compliance verified
- Anti-goal-drift checklist clean
- Documentation complete

**Blocking Issues (Cannot Proceed):**
- Any verification layer fails
- Architecture drift without ADR update
- Schema fields unimplemented
- Performance regression >10%
- Test coverage < 90%

**Evidence Storage:**
- `results/phase_$PHASE/` directory
- JSON for automation, logs for debugging
- Retained for project lifetime

---

## Appendix: Verification Layer Cheat Sheet

| Layer | Command | Time | Evidence |
|-------|---------|------|----------|
| Unit | `cargo test --lib` | 1-2 min | JSON results |
| Integration | `cargo test --test '*'` | 2-5 min | Log output |
| Property | `cargo test --lib property_based` | 5-10 min | JSON results |
| E2E | Workflow execution loop | 5-15 min | Execution logs |
| System Log | Log parsing validation | 1-2 min | JSON samples |
| CLI | Manual command testing | 10-20 min | Screenshots |
| Benchmark | `cargo bench` | 10-30 min | HTML reports |

**Total Estimated Time:** 34-84 minutes per phase validation

---

**End of Framework Definition**
