# Phase 03: Quality Loops - Validation Criteria

**Phase Focus:** Generate-verify-repair loops, benchmarks, file-type matrix, quality reports
**Entry Criteria:** Phases 00, 01, and 02 complete (Phase 03 optional)
**Estimated Duration:** 3-4 weeks
**Blocking for:** Phases 04, 05, 06, 07

---

## Phase Overview

Phase 03 implements quality assurance loops. This phase provides generate-verify-repair workflows for iterative improvement, benchmarks with metadata, file-type quality tracking, and actionable quality reports. Quality loops automatically detect issues and suggest or apply repairs.

**Critical Success Factors:**
1. Generate-verify-repair loops converge (no infinite loops)
2. Benchmarks store with complete metadata
3. File-type matrix tracks quality accurately
4. Reports provide actionable insights

---

## Phase Entry Criteria

**Status:** Must be verified before starting Phase 03

**Verification Commands:**
```bash
# Verify Phases 00-02 exit
cargo test --test phase_00_integration -- --test-threads=1
cargo test --test phase_01_integration -- --test-threads=1
cargo test --test phase_02_integration -- --test-threads=1

# Verify schema coverage
cargo run --bin schema_audit -- --phase 0 --output phase_00_coverage.md
cargo run --bin schema_audit -- --phase 1 --output phase_01_coverage.md
cargo run --bin schema_audit -- --phase 2 --output phase_02_coverage.md
```

**Prerequisites:**
- [ ] Phases 00, 01, 02 exit criteria verified (all 7 layers)
- [ ] Schema coverage 100% for phases 00, 01, 02
- [ ] ADR compliance verified for phases 00, 01, 02
- [ ] Cross-phase regression clean (Phases 00+01+02)
- [ ] Benchmark infrastructure ready

**Blocking Violations:**
- Unresolved Phase 00, 01, or 02 failures
- Schema coverage < 100% for any phase
- Cross-phase regression detected

---

## Generate-Verify-Repair Loops

**Requirement:** Generate-verify-repair loops converge (no infinite loops)

### Loop Structure

```
Generate → Verify → Pass?
               ↓
           Repair → Generate
```

1. **Generate:** Produce artifact (code, config, document)
2. **Verify:** Validate artifact against criteria
3. **Pass?** If verification passes, exit loop
4. **Repair:** If verification fails, generate repair
5. **Repeat:** Go back to Generate

### Convergence Criteria

1. **Max Iterations:** Loop terminates after N iterations
2. **Convergence Threshold:** Verification score >= threshold
3. **Stability:** No improvement for K iterations
4. **Timeout:** Maximum time limit exceeded

### Verification Commands

```bash
# Test generate-verify-repair loops
cargo test --lib quality::tests::basic_gvr_loop
cargo test --lib quality::tests::gvr_convergence
cargo test --lib quality::tests::gvr_max_iterations
cargo test --lib quality::tests::gvr_convergence_threshold
cargo test --lib quality::tests::gvr_stability_check
cargo test --lib quality::tests::gvr_timeout

# Test infinite loop prevention
cargo test --lib quality::tests::infinite_loop_prevention
```

### Pass Criteria

- [ ] Generate-verify-repair loops execute correctly
- [ ] Loops converge (terminate) in all test cases
- [ ] Max iterations respected
- [ ] Convergence thresholds work
- [ ] Stability checks work
- [ ] Timeout enforced
- [ ] No infinite loops

### Evidence Required

- Generate-verify-repair loop test results
- Convergence metrics
- Timeout test logs
- Infinite loop prevention test logs

---

## Benchmark Storage

**Requirement:** Benchmarks store with complete metadata

### Benchmark Metadata

1. **Workflow Metadata:**
   - Workflow ID
   - Workflow name
   - Workflow version
   - Workflow type (generation, verification, repair)

2. **Execution Metadata:**
   - Timestamp
   - Execution time
   - Success/failure
   - Error message (if failed)

3. **Resource Metadata:**
   - CPU usage
   - Memory usage
   - Network usage
   - Disk I/O

4. **Quality Metadata:**
   - Verification score
   - Repair count
   - Iteration count
   - Convergence metric

### Storage Format

```json
{
  "benchmark_id": "uuid",
  "workflow_id": "workflow_uuid",
  "workflow_name": "test_workflow",
  "workflow_version": "1.0.0",
  "workflow_type": "generation",
  "timestamp": "2026-04-06T10:00:00Z",
  "execution_time_ms": 1234,
  "success": true,
  "error": null,
  "cpu_usage_percent": 75.5,
  "memory_usage_mb": 512,
  "network_usage_bytes": 1024,
  "disk_io_bytes": 2048,
  "verification_score": 0.95,
  "repair_count": 2,
  "iteration_count": 5,
  "convergence_metric": 0.98
}
```

### Verification Commands

```bash
# Test benchmark storage
cargo test --lib quality::tests::benchmark_storage
cargo test --lib quality::tests::benchmark_retrieval
cargo test --lib quality::tests::benchmark_metadata_completeness

# Test benchmark querying
cargo test --lib quality::tests::benchmark_query_by_workflow
cargo test --lib quality::tests::benchmark_query_by_type
cargo test --lib quality::tests::benchmark_query_by_score
```

### Pass Criteria

- [ ] Benchmarks stored with complete metadata
- [ ] Metadata retrieval works correctly
- [ ] All required fields present
- [ ] Benchmark queries work correctly
- [ ] No missing or null metadata fields

### Evidence Required

- Benchmark storage test results
- Metadata completeness verification
- Benchmark query test logs

---

## File-Type Matrix

**Requirement:** File-type matrix tracks quality

### File Types

1. **YAML Files:** Workflow specifications
2. **Rust Files:** Generated code
3. **Markdown Files:** Documentation
4. **JSON Files:** Configuration and metadata
5. **Text Files:** Logs and reports

### Quality Metrics

1. **Syntax Validity:** File parses correctly
2. **Semantic Correctness:** Content is semantically valid
3. **Style Compliance:** Follows style guidelines
4. **Coverage:** Test coverage (for code)
5. **Documentation:** Documentation completeness

### Matrix Structure

| File Type | Total | Valid | Invalid | Quality Score | Avg Iterations |
|-----------|-------|-------|---------|----------------|----------------|
| YAML      | 53    | 50    | 3       | 0.94           | 2.1            |
| Rust      | 25    | 23    | 2       | 0.92           | 1.8            |
| Markdown  | 12    | 12    | 0       | 1.00           | 1.0            |
| JSON      | 8     | 8     | 0       | 1.00           | 1.0            |
| Text      | 5     | 5     | 0       | 1.00           | 1.0            |

### Verification Commands

```bash
# Test file-type matrix
cargo test --lib quality::tests::file_type_matrix_tracking
cargo test --lib quality::tests::file_type_matrix_aggregation
cargo test --lib quality::tests::file_type_matrix_quality_scores

# Test matrix updates
cargo test --lib quality::tests::matrix_update_on_valid_file
cargo test --lib quality::tests::matrix_update_on_invalid_file
```

### Pass Criteria

- [ ] File-type matrix tracks all file types
- [ ] Quality scores calculated correctly
- [ ] Matrix updates on valid/invalid files
- [ ] Aggregation statistics accurate
- [ ] Matrix queryable by file type

### Evidence Required

- File-type matrix test results
- Quality score calculations
- Matrix aggregation logs

---

## Quality Reports

**Requirement:** Reports provide actionable insights

### Report Types

1. **Overall Quality Report:** Summary of all workflows
2. **Workflow Quality Report:** Detailed report for single workflow
3. **File-Type Quality Report:** Quality by file type
4. **Trend Report:** Quality over time

### Report Content

#### Overall Quality Report

```markdown
# Overall Quality Report

## Summary
- Total Workflows: 53
- Valid Workflows: 50 (94.3%)
- Invalid Workflows: 3 (5.7%)
- Average Quality Score: 0.94
- Average Iterations: 1.8

## Issues
1. workflow_04.yaml: Invalid syntax (line 42)
2. workflow_12.yaml: Missing required field
3. workflow_27.yaml: Schema validation error

## Recommendations
1. Fix syntax errors in workflow_04.yaml
2. Add missing fields to workflow_12.yaml
3. Resolve schema validation in workflow_27.yaml
```

### Verification Commands

```bash
# Test quality reports
cargo test --lib quality::tests::overall_quality_report
cargo test --lib quality::tests::workflow_quality_report
cargo test --lib quality::tests::file_type_quality_report
cargo test --lib quality::tests::trend_report

# Test report actionability
cargo test --lib quality::tests::report_actionability
cargo test --lib quality::tests::report_recommendations
```

### Pass Criteria

- [ ] All report types generated correctly
- [ ] Reports include actionable insights
- [ ] Recommendations specific and actionable
- [ ] Trend data accurate
- [ ] Report formatting correct

### Evidence Required

- Quality report test results
- Sample quality reports
- Actionability verification

---

## Phase Exit Criteria

### Layer 1: Unit Tests

**Commands:**
```bash
cargo test --lib -- --test-threads=1
```

**Evidence:**
- All Phase 03 unit tests pass
- Test coverage >= 90%
- No clippy warnings

### Layer 2: Integration Tests

**Commands:**
```bash
cargo test --test '*' -- --test-threads=1
```

**Evidence:**
- All Phase 03 integration tests pass
- Quality loops integrate with scheduler

### Layer 3: Property Tests

**Commands:**
```bash
PROPTEST_NUMBER_OF_TESTS=100 cargo test --lib property_based
```

**Evidence:**
- Loop convergence invariants hold for 100 iterations
- Metadata completeness invariants hold

### Layer 4: E2E Tests

**Commands:**
```bash
# Execute quality loop workflow
cargo run --bin agentsdk -- run examples/workflows/quality_loop.yaml

# Verify benchmark storage
cargo run --bin quality -- list-benchmarks

# Generate quality report
cargo run --bin quality -- report --output quality_report.md
```

**Evidence:**
- Quality loops execute and converge
- Benchmarks stored with metadata
- Quality reports generated

### Layer 5: System Log Validation

**Commands:**
```bash
# Verify automation scope logs
cargo run --bin agentsdk -- run examples/workflows/quality_loop.yaml 2>&1 | \
  jq -e 'select(.scope == "automation")'
```

**Evidence:**
- Automation scope logs emitted
- Logs include timestamp, scope, event, message

### Layer 6: Live CLI Verification

**Commands:**
```bash
# Test quality commands
cargo run --bin agentsdk -- quality list-benchmarks
cargo run --bin agentsdk -- quality report
cargo run --bin agentsdk -- quality matrix
```

**Evidence:**
- Quality commands work
- Output formatted correctly

### Layer 7: Benchmark Performance

**Commands:**
```bash
cargo bench --bench phase_04_benchmarks
```

**Evidence:**
- Quality loop performance acceptable
- Benchmark storage performant
- No performance regression > 10%

---

## Cross-Phase Regression Tests

**Commands:**
```bash
# Run all prior phase tests
cargo test --test phase_00_integration -- --test-threads=1
cargo test --test phase_01_integration -- --test-threads=1
cargo test --test phase_02_integration -- --test-threads=1
```

**Pass Criteria:**
- [ ] All Phase 00 tests still pass
- [ ] All Phase 01 tests still pass
- [ ] All Phase 02 tests still pass

---

## Schema Coverage Audit

**Requirement:** 100% of Phase 03-owned fields implemented

### Phase 03-Owned Fields

**QualitySchema:**
- verification_criteria
- repair_strategy
- max_iterations
- convergence_threshold

**BenchmarkSchema:**
- benchmark_id
- execution_time_ms
- verification_score
- repair_count

**QualityReportSchema:**
- report_type
- summary
- issues
- recommendations

### Verification Commands

```bash
cargo run --bin schema_audit -- --phase 4 --output coverage_report.md
```

**Expected Output:**
- 100% coverage for Phase 03-owned fields
- No unimplemented fields

---

## ADR Compliance

**Relevant ADRs:**
- ADR-008: Quality Loop Architecture

### Verification Commands

```bash
cargo run --bin adr_compliance -- --phase 4
```

**Expected Output:**
- All Phase 03-relevant ADR constraints satisfied
- No outstanding ADR TODOs

---

## Anti-Goal-Drift Checklist

**Run after phase completion:**

1. **Requirements Drift:**
   - [ ] Implementation matches Phase 03 requirements
   - [ ] No features from later phases added

2. **Architecture Drift:**
   - [ ] Quality loops match ADR-008

3. **Scope Creep:**
   - [ ] Only quality loop features implemented
   - [ ] No memory or search features added

---

## Evidence Storage

**Location:** `results/phase_04/`

**Contents:**
- `unit_test_results.json`
- `integration_test_output.log`
- `property_test_results.json`
- `e2e_execution_logs/` (quality loop workflows)
- `system_log_samples.json` (automation scope)
- `cli_verification/` (quality commands)
- `benchmarks/` (quality loop performance)
- `benchmark_metadata/` (stored benchmark metadata)
- `quality_reports/` (generated quality reports)
- `file_type_matrix/` (file-type quality matrix)
- `schema_coverage_report.md`
- `adr_compliance_report.md`
- `anti_goal_drift_checklist.md`
- `phase_00_regression/`
- `phase_01_regression/`
- `phase_02_regression/`

---

## Blocking Issues

**Cannot exit Phase 03 if:**
- Any verification layer fails
- Generate-verify-repair loops don't converge
- Benchmark metadata incomplete
- File-type matrix doesn't track quality
- Reports not actionable
- Any prior phase regression detected

---

**End of Phase 03 Criteria**
