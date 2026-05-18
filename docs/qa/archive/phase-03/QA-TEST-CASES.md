# Phase 03: Test Cases

### P03-001: Verifier Interface Object-Safety
- **Area**: QA-03-01
- **Type**: Compile-time
- **Command**: `cargo check --lib 2>&1 | grep -i "object.*safe\|dyn.*Verifier"`
- **Setup**: N/A (compile check)
- **Expected**: No errors. Verifier trait compiles and can be used as `dyn Verifier`
- **Pass Criteria**: ✅ `cargo check` passes

### P03-002: Verifier Capabilities
- **Area**: QA-03-01
- **Type**: Unit
- **Command**: `cargo test verifier_capabilities --lib`
- **Setup**: Mock verifiers with different capabilities
- **Expected**: VerifierCapabilities correctly describes what each verifier can check
- **Pass Criteria**: ✅ All capability fields accessible and correct

### P03-003: Built-in Code Verifiers
- **Area**: QA-03-01
- **Type**: Integration
- **Command**: `cargo test code_verifiers --test`
- **Setup**: Code artifact, compilation verifier, linting verifier, test executor
- **Expected**: Each verifier runs correctly, results accurate
- **Pass Criteria**: ✅ All code verifiers pass on valid code, fail on invalid code

### P03-004: Built-in Documentation Verifiers
- **Area**: QA-03-01
- **Type**: Integration
- **Command**: `cargo test doc_verifiers --test`
- **Setup**: Documentation artifact, completeness verifier, structure verifier
- **Expected**: Documentation verified for completeness and structure
- **Pass Criteria**: ✅ Doc verifiers validate correctly

### P03-005: Verifier Registry
- **Area**: QA-03-01
- **Type**: Unit
- **Command**: `cargo test verifier_registry --lib`
- **Setup**: Built-in and custom verifiers
- **Expected**: Registry registers verifiers, discovery works
- **Pass Criteria**: ✅ Custom verifiers registered, discovered by name

### P03-006: Quality Loop Convergence
- **Area**: QA-03-02
- **Type**: Integration
- **Command**: `cargo test quality_loop_convergence --test`
- **Setup**: RepairLoop with max_iterations=5, epsilon=0.05
- **Expected**: Loop converges within max_iterations or quality >= threshold
- **Pass Criteria**: ✅ Convergence detected, loop terminates

### P03-007: Quality Loop State Tracking
- **Area**: QA-03-02
- **Type**: Integration
- **Command**: `cargo test quality_loop_state --test`
- **Setup**: Quality loop execution with artifacts
- **Expected**: State tracked correctly: iteration count, quality score, repair history
- **Pass Criteria**: ✅ State updates correctly, repair history preserved

### P03-008: Quality Loop Auto-Trigger
- **Area**: QA-03-02
- **Type**: Integration
- **Command**: `cargo test quality_loop_autotrigger --test`
- **Setup**: Verifier fails initially, then passes
- **Expected**: Repair step triggered automatically on verifier failure
- **Pass Criteria**: ✅ Auto-repair triggers without manual intervention

### P03-009: Benchmark Provenance Tracking
- **Area**: QA-03-03
- **Type**: Integration
- **Command**: `cargo test benchmark_provenance --test`
- **Setup**: Benchmark execution with workflow, policy, backend, file type
- **Expected**: All provenance fields captured: workflow_version, policy_snapshot, backend, file_type
- **Pass Criteria**: ✅ Provenance stored completely and accurately

### P03-010: Benchmark Storage and Retrieval
- **Area**: QA-03-03
- **Type**: Integration
- **Command**: `cargo test benchmark_storage --test`
- **Setup**: Benchmark store, multiple benchmarks
- **Expected**: Benchmarks stored correctly, retrieved by query
- **Pass Criteria**: ✅ Storage and retrieval work end-to-end

### P03-011: Benchmark Statistical Analysis
- **Area**: QA-03-03
- **Type**: Integration
- **Command**: `cargo test benchmark_statistics --test`
- **Setup**: Multiple benchmark runs with varying results
- **Expected**: Statistical metrics calculated: mean, median, std dev, percentiles (p50, p95, p99)
- **Pass Criteria**: ✅ Statistics accurate and complete

### P03-012: Capability Matrix Registration
- **Area**: QA-03-04
- **Type**: Unit
- **Command**: `cargo test capability_registration --lib`
- **Setup**: Workflow IDs, file types, quality thresholds
- **Expected**: Capabilities registered in matrix
- **Pass Criteria**: ✅ Matrix populated correctly

### P03-013: Capability Matrix Querying
- **Area**: QA-03-04
- **Type**: Unit
- **Command**: `cargo test capability_querying --lib`
- **Setup**: Capability matrix with multiple workflows and file types
- **Expected**: Query returns workflows supporting given file type
- **Pass Criteria**: ✅ Queries return correct workflow lists

### P03-014: Gap Analysis
- **Area**: QA-03-04
- **Type**: Unit
- **Command**: `cargo test gap_analysis --lib`
- **Setup**: Capability matrix, missing workflows for some file types
- **Expected**: Gap analysis identifies missing capabilities
- **Pass Criteria**: ✅ Gap report lists missing workflows and file types

### P03-015: Workflow Library Storage
- **Area**: QA-03-05
- **Type**: Integration
- **Command**: `cargo test workflow_library_storage --test`
- **Setup**: Artifact workflows with versions
- **Expected**: Workflows stored with versioning
- **Pass Criteria**: ✅ Storage preserves versions, retrieval works

### P03-016: Workflow Transformation
- **Area**: QA-03-05
- **Type**: Integration
- **Command**: `cargo test workflow_transformation --test`
- **Setup**: Workflow, transformation operations (optimize, refactor)
- **Expected**: Transformations applied correctly
- **Pass Criteria**: ✅ Transformed workflow valid and improved

### P03-017: HTML Report Generation
- **Area**: QA-03-06
- **Type**: Integration
- **Command**: `cargo test html_reports --test`
- **Setup**: Quality report data
- **Expected**: HTML report generated with charts and tables
- **Pass Criteria**: ✅ HTML renders correctly, interactive elements work

### P03-018: PDF Report Generation
- **Area**: QA-03-06
- **Type**: Integration
- **Command**: `cargo test pdf_reports --test`
- **Setup**: Quality report data
- **Expected**: PDF report generated for archiving
- **Pass Criteria**: ✅ PDF generated without errors, formatting correct

### P03-019: JSON Report Generation
- **Area**: QA-03-06
- **Type**: Integration
- **Command**: `cargo test json_reports --test`
- **Setup**: Quality report data
- **Expected**: JSON report generated for CI integration
- **Pass Criteria**: ✅ JSON valid, schema correct

### P03-020: Markdown Report Generation
- **Area**: QA-03-06
- **Type**: Integration
- **Command**: `cargo test markdown_reports --test`
- **Setup**: Quality report data
- **Expected**: Markdown report generated for documentation
- **Pass Criteria**: ✅ Markdown renders correctly, tables and charts included

### P03-021: Dashboard API Routes
- **Area**: QA-03-07
- **Type**: Integration
- **Command**: `cargo test dashboard_api --test`
- **Setup**: Dashboard server, test endpoints
- **Expected**: All API routes respond correctly
- **Pass Criteria**: ✅ API handlers return correct data and status codes

### P03-022: Dashboard Quality Trends
- **Area**: QA-03-07
- **Type**: Integration
- **Command**: `cargo test dashboard_trends --test`
- **Setup**: Quality metrics over time
- **Expected**: Trends calculated and displayed
- **Pass Criteria**: ✅ Trend charts render correctly

### P03-023: Dashboard Benchmark Visualization
- **Area**: QA-03-07
- **Type**: Integration
- **Command**: `cargo test dashboard_benchmarks --test`
- **Setup**: Benchmark history
- **Expected**: Benchmarks visualized with charts
- **Pass Criteria**: ✅ Benchmark charts render correctly

---

## Test Execution Order
**Phase 3 Group 1: Verifier System**
P03-001 → P03-002 → P03-003 → P03-004 → P03-005

**Phase 3 Group 2: Quality Loops**
P03-006 → P03-007 → P03-008

**Phase 3 Group 3: Benchmark Harness**
P03-009 → P03-010 → P03-011

**Phase 3 Group 4: Capability Matrix**
P03-012 → P03-013 → P03-014

**Phase 3 Group 5: Workflow Library**
P03-015 → P03-016

**Phase 3 Group 6: Report Generation**
P03-017 → P03-018 → P03-019 → P03-020

**Phase 3 Group 7: Dashboard Integration**
P03-021 → P03-022 → P03-023
