# Phase 0 Foundation Acceptance Criteria

This document defines the exit criteria for Phase 0 Foundation. All criteria must be met before proceeding to Phase 1.

## Overview

Phase 0 builds the foundational infrastructure for the AgentSDK Execution Engine:
- Schema types for Sections 1,2,13-18 of unified schema
- YAML parsing with accurate error reporting
- WorkflowIR compilation and validation
- Local persistence with sled
- Variable interpolation and scoping
- DAG validation and circular reference detection
- Policy compilation with inheritance
- Defaults and threshold validation

## Exit Criteria

### 1. Schema Coverage ✅

**Requirement:** All Sections 1,2,13-18 of unified schema implemented

**Verification:**
```bash
# Check all schema types exist
ls src/schema/
# Expected output:
# identification.rs
# model.rs
# workspace.rs
# features.rs
# step.rs
# loop.rs
# execution.rs
# mod.rs
```

**Acceptance:**
- ✅ Section 1: WorkflowIdentification complete
- ✅ Section 2: ModelConfig complete with all fields
- ✅ Section 13: WorkspaceConfig complete
- ✅ Section 14: FeaturesDemonstrated complete
- ✅ Section 15: Variable interpolation syntax supported
- ✅ Section 16: Default values implemented
- ✅ Section 17: Scope & inheritance rules implemented
- ✅ Section 18: Numeric thresholds defined (11 thresholds)

---

### 2. Parser Correctness ✅

**Requirement:** YAML→WorkflowSpec parsing with accurate error reporting

**Verification:**
```bash
# Parse all test fixtures
cargo test parser_test
# Expected: All tests pass

# Parse example workflows
for file in opencode/docs/reports/requirements/example-workflows/requirements-oriented-auto/**/*.yml; do
    if [ -f "$file" ]; then
        cargo run --bin parse-check -- "$file" || true
    fi
done
```

**Acceptance:**
- ✅ Minimal workflow parses successfully
- ✅ Complex workflow parses successfully
- ✅ Invalid YAML fails with helpful error
- ✅ Missing required fields fail validation
- ✅ Error messages include file path and line numbers
- ✅ Agentic workflow parsing works
- ✅ Pipeline format parsing works

---

### 3. IR Compilation ✅

**Requirement:** WorkflowSpec→WorkflowIR pipeline passes all tests

**Verification:**
```bash
# Test compilation
cargo test compiler_test
# Expected: All tests pass

# End-to-end compilation
cargo run --example compile-workflow -- tests/fixtures/workflows/complex.yml
# Expected: Successfully compiles to WorkflowIR
```

**Acceptance:**
- ✅ Minimal workflow compiles to IR
- ✅ Complex workflow compiles to IR
- ✅ Type checking catches schema violations
- ✅ Interpolation happens during compilation
- ✅ All models compile to ModelIR
- ✅ All steps compile to StepIR
- ✅ Execution mode compiles correctly
- ✅ Error messages include context
- ✅ No serde_yaml::Value in IR (strict typing)

---

### 4. Validation ✅

**Requirement:** DAG validation, threshold validation, policy compilation all pass

**Verification:**
```bash
# Test all validation
cargo test dag_test threshold_test policy_test
# Expected: All tests pass

# Test circular dependency detection
cargo test --test circular_dependency
# Expected: Circular dependencies detected and reported
```

**Acceptance:**

**DAG Validation:**
- ✅ Linear workflows validate successfully
- ✅ Parallel workflows validate successfully
- ✅ Circular dependencies detected
- ✅ Error messages include cycle path
- ✅ Step references validated

**Threshold Validation:**
- ✅ All 11 thresholds defined
- ✅ Range checks enforce min/max values
- ✅ Default values applied when not specified
- ✅ Error messages include actual vs expected values
- ✅ Validation happens at parse time

**Policy Compilation:**
- ✅ Policies compile deterministically
- ✅ Inheritance hierarchy enforced (step > workflow > system)
- ✅ Override rules enforced correctly
- ✅ Logging policies compile
- ✅ Tool permission policies compile

---

### 5. Persistence ✅

**Requirement:** Local ./workspace/ directory structure with sled persistence

**Verification:**
```bash
# Test storage
cargo test storage_test
# Expected: All tests pass

# Check directory structure
ls -la ./workspace/
# Expected: db/ directory present
```

**Acceptance:**
- ✅ ./workspace/ directory created on open
- ✅ sled database initialized with ACID transactions
- ✅ Workflow IR storage works
- ✅ Workflow IR retrieval works
- ✅ Execution state storage works
- ✅ UUID generation for execution IDs
- ✅ Storage persists across restarts

---

### 6. Test Coverage ✅

**Requirement:** All unit, integration, and property tests pass

**Verification:**
```bash
# Run all tests
cargo test --all

# Check test coverage (if coverage tool installed)
cargo tarpaulin --out Html --output-dir coverage/
# Expected: 80%+ coverage for Phase 0 code
```

**Acceptance:**

**Unit Tests:**
- ✅ Error module tests pass (error_test.rs)
- ✅ Schema type tests pass (schema_test.rs)
- ✅ Parser tests pass (parser_test.rs)
- ✅ Interpolation tests pass (interpolation_test.rs)
- ✅ IR type tests pass (ir_test.rs)
- ✅ Compiler tests pass (compiler_test.rs)
- ✅ DAG tests pass (dag_test.rs)
- ✅ Policy tests pass (policy_test.rs)
- ✅ Storage tests pass (storage_test.rs)
- ✅ Workspace tests pass (workspace_test.rs)
- ✅ Defaults tests pass (defaults_test.rs)
- ✅ Threshold tests pass (threshold_test.rs)

**Integration Tests:**
- ✅ YAML→WorkflowSpec→WorkflowIR pipeline works
- ✅ All test fixtures compile successfully
- ✅ End-to-end workflow validation works

**Property Tests:**
- ✅ Determinism tests pass (same input = same output)
- ✅ Round-trip tests pass (serialize → deserialize)
- ✅ Invariant tests pass (IR properties maintained)

---

### 7. Verification Layers ✅

**Requirement:** All 7 verification layers pass with zero errors

**Verification:**

**Layer 1: LSP Diagnostics**
```bash
lsp_diagnostics filePath=/home/jon/code/whitt-execution-engine/src/ severity=all
# Expected: No errors
```
**Acceptance:** ✅ No errors on all source files

**Layer 2: Unit Tests**
```bash
cargo test
# Expected: test result: ok. X passed in Y.ZZs
```
**Acceptance:** ✅ All tests pass, 100% defined tests run

**Layer 3: Integration Tests**
```bash
cargo test --test integration
# Expected: test result: ok. X passed
```
**Acceptance:** ✅ All integration tests pass

**Layer 4: Property Tests**
```bash
cargo test --test properties
# Expected: test result: ok. X passed (100+ test cases)
```
**Acceptance:** ✅ All property tests pass

**Layer 5: Build Verification**
```bash
cargo build --release
# Expected: Finished release [optimized] target(s)
```
**Acceptance:** ✅ Successful build, no warnings

**Layer 6: Schema Validation**
```bash
cargo test --test schema_validation
# Expected: test result: ok. X passed
```
**Acceptance:** ✅ All schema validation tests pass

**Layer 7: Memory Safety**
```bash
cargo clippy -- -D warnings
# Expected: No clippy warnings
```
**Acceptance:** ✅ No clippy warnings, no unsafe code (unless justified)

---

## Anti-Goal-Drift Verification

### Checkpoint 1 (After Task 2)
**Check:** Verify task 2 implements ONLY schema types
- ❌ No parser code
- ❌ No IR types
- ❌ No interpolation logic
- ❌ No validation logic
- ❌ No persistence code

### Checkpoint 2 (After Task 6)
**Check:** Verify task 6 compiles to IR only
- ❌ No DAG validation
- ❌ No policy compilation
- ❌ No persistence
- ❌ No workspace management

### Checkpoint 3 (After Task 11)
**Check:** Verify task 11 implements ONLY defaults and inheritance
- ❌ No persistence
- ❌ No workspace management
- ❌ No threshold validation

### Checkpoint 4 (After Task 12)
**Check:** Verify task 12 implements ONLY threshold validation
- ❌ All other validation already tested

---

## External Dependencies

**Requirement:** Zero external dependencies in Phase 0 (use in-memory test fixtures)

**Verification:**
```bash
# Check Cargo.toml for external dependencies
grep -E "lmstudio|ollama|llama.cpp" Cargo.toml
# Expected: No matches (except in comments)
```

**Acceptance:**
- ✅ No external service dependencies
- ✅ All providers mocked (LM Studio, Ollama, llama.cpp)
- ✅ Tool operations use in-memory test fixtures
- ✅ State persistence uses in-memory sled for tests
- ✅ Tests run in < 100ms without network I/O

---

## Documentation

**Requirement:** All code is documented with examples

**Verification:**
```bash
# Check documentation builds
cargo doc --no-deps

# Run documentation tests
cargo test --doc
```

**Acceptance:**
- ✅ All public types have documentation
- ✅ All public functions have documentation
- ✅ Complex logic has inline comments
- ✅ Documentation tests pass
- ✅ README updated with Phase 0 features

---

## Code Quality

**Requirement:** Code follows Rust best practices and conventions

**Verification:**
```bash
# Check formatting
cargo fmt -- --check

# Check clippy
cargo clippy -- -D warnings

# Check for unsafe code
grep -r "unsafe" src/
# Expected: No unsafe code (or justified with comments)
```

**Acceptance:**
- ✅ Code is formatted with rustfmt
- ✅ No clippy warnings (or justified)
- ✅ No unsafe code (or justified)
- ✅ Error handling uses thiserror
- ✅ Result types used consistently
- ✅ Naming conventions followed

---

## Performance

**Requirement:** Tests run quickly and efficiently

**Verification:**
```bash
# Time test execution
time cargo test
# Expected: < 30 seconds for all tests
```

**Acceptance:**
- ✅ All unit tests run in < 10 seconds
- ✅ All integration tests run in < 20 seconds
- ✅ No flaky tests
- ✅ Test fixtures are small and fast to parse

---

## Deliverables

### Source Code
- ✅ src/error.rs (extended error module)
- ✅ src/schema/*.rs (all schema types)
- ✅ src/parser/mod.rs (YAML parser)
- ✅ src/interpolation.rs (variable interpolation)
- ✅ src/ir/mod.rs (WorkflowIR types)
- ✅ src/compiler/mod.rs (IR compiler)
- ✅ src/validation/mod.rs (DAG validation)
- ✅ src/validation/thresholds.rs (threshold validation)
- ✅ src/storage/mod.rs (local storage)
- ✅ src/workspace.rs (workspace management)
- ✅ src/policy.rs (policy compilation)
- ✅ src/defaults.rs (defaults and inheritance)

### Tests
- ✅ tests/error_test.rs
- ✅ tests/schema_test.rs
- ✅ tests/parser_test.rs
- ✅ tests/interpolation_test.rs
- ✅ tests/ir_test.rs
- ✅ tests/compiler_test.rs
- ✅ tests/dag_test.rs
- ✅ tests/policy_test.rs
- ✅ tests/storage_test.rs
- ✅ tests/workspace_test.rs
- ✅ tests/defaults_test.rs
- ✅ tests/threshold_test.rs

### Fixtures
- ✅ tests/fixtures/workflows/minimal.yml
- ✅ tests/fixtures/workflows/complex.yml

### Documentation
- ✅ opencode/docs/plans/00-foundation/plan.md
- ✅ opencode/docs/plans/00-foundation/tasks/*.md (12 task files)
- ✅ opencode/docs/plans/00-foundation/validation/*.md
- ✅ opencode/docs/plans/00-foundation/tests/*.md

---

## Sign-Off

**Phase 0 Foundation is complete when:**

1. ✅ All 12 tasks complete with passing tests
2. ✅ All checkpoint criteria met (see checkpoint-criteria.md)
3. ✅ All 7 verification layers pass
4. ✅ All acceptance criteria in this document met
5. ✅ Zero external dependencies (all mocked)
6. ✅ Full integration test suite passes
7. ✅ Property tests demonstrate correctness invariants

**Ready for Phase 1 (MVP Queue & Scheduler):**
- WorkflowIR available for job execution context
- Persistence available for queue state storage
- Validation available for job acceptance criteria
- Error handling available for queue failures

---

**Plan Version:** 1.0.0
**Last Updated:** 2026-04-06
**Status:** Ready for Execution
