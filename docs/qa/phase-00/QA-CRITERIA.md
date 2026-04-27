# Phase 00: Foundation — QA Criteria

## Overview

Phase 0 implements the foundational infrastructure for the AgentSDK Execution Engine, including YAML parsing, schema validation, WorkflowIR compilation, and persistence with zero external dependencies. This phase establishes the core data structures and validation layer that all subsequent phases depend on.

**Dependencies:** None (Phase 0 is the foundation)
**Estimated Time:** 60-80 hours (12 tasks, 5-6.5 hours each)

---

## QA Areas

### QA-00-01: Cargo Project Setup
- **Plan Ref:** [Task 00](../../plans/00-foundation/tasks/00-cargo-project-setup.md)
- **Schema Ref:** N/A (infrastructure setup)
- **Priority:** P0
- **Test Type:** Build
- **Criteria:**
  1. Cargo.toml exists with all required dependencies
  2. Project structure matches specified layout (src/, tests/, docs/, etc.)
  3. CI/CD pipeline configured (GitHub Actions)
  4. Workspace configuration enables multi-crate project
- **Commands:**
  ```bash
  ls Cargo.toml
  cargo check
  cargo build --release
  ```

### QA-00-02: Error Module
- **Plan Ref:** [Task 01](../../plans/00-foundation/tasks/01-error-module.md)
- **Schema Ref:** N/A (error handling)
- **Priority:** P0
- **Test Type:** Unit
- **Criteria:**
  1. All error variants defined with proper `thiserror` derives
  2. Error hierarchy covers: ParseError, CompilationError, PolicyError, StorageError, ValidationError
  3. Error messages are descriptive and actionable
  4. `anyhow::Result<T>` used for application errors
  5. Error conversion chain preserves context
- **Commands:**
  ```bash
  cargo test --lib error
  cargo clippy --lib error
  ```

### QA-00-03: Schema Types
- **Plan Ref:** [Task 02](../../plans/00-foundation/tasks/02-schema-types.md)
- **Schema Ref:** Section 1 (Workflow Identification), Section 2 (Model Configuration), Section 13 (Workspace Configuration), Section 14 (Features Demonstrated), Section 15 (Variable Interpolation), Section 16 (Default Behavior), Section 17 (Scope & Inheritance), Section 18 (Numeric Thresholds)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. All Rust structs implement schema sections 1, 2, 13-18
  2. serde derives generate correct `Serialize` and `Deserialize` implementations
  3. garde validation rules compile and enforce schema constraints
  4. schemars generates valid JSON schema
  5. Optional fields have correct default values
  6. No unused fields or dead code warnings
- **Commands:**
  ```bash
  cargo test --lib schema
  cargo build --release
  schemars validate
  ```

### QA-00-04: YAML Parser
- **Plan Ref:** [Task 03](../../plans/00-foundation/tasks/03-yaml-parser.md)
- **Schema Ref:** Full unified workflow schema (all sections)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. YAML files parse correctly to WorkflowSpec
  2. Error messages include line numbers and context
  3. yaml_serde handles merge keys correctly
  4. Parsing validates against schema at parse time
  5. All test fixtures from example-workflows/ parse correctly
  6. Performance: parsing < 100ms for typical workflows
- **Commands:**
  ```bash
  cargo test --lib parser
  cargo test --test '*parser*'
  ```

### QA-00-05: Variable Interpolation
- **Plan Ref:** [Task 04](../../plans/00-foundation/tasks/04-variable-interpolation.md)
- **Schema Ref:** Section 15 (Variable Interpolation Syntax)
- **Priority:** P0
- **Test Type:** Unit
- **Criteria:**
  1. `${...}` syntax resolves at parse time (YAML parsing phase)
  2. `{{...}}` syntax works for runtime interpolation
  3. Scoping rules enforce proper variable resolution
  4. Model references resolve correctly (`${models.primary}`)
  5. Parse-time interpolation produces deterministic results
  6. Unknown variable references produce clear errors
  7. Circular dependencies detected and reported
- **Commands:**
  ```bash
  cargo test --lib interpolation
  cargo test --test '*interpolation*'
  ```

### QA-00-06: Workflow IR
- **Plan Ref:** [Task 05](../../plans/00-foundation/tasks/05-workflow-ir.md)
- **Schema Ref:** N/A (internal representation)
- **Priority:** P0
- **Test Type:** Unit
- **Criteria:**
  1. WorkflowIR structs are strictly typed (no `serde_yaml::Value` anywhere)
  2. All WorkflowSpec variants map to correct WorkflowIR types
  3. Type system prevents invalid IR states
  4. IR serialization/deserialization is lossless (round-trip works)
  5. IR contains all necessary execution metadata
  6. No type safety violations or unsafe code
- **Commands:**
  ```bash
  cargo test --lib ir
  cargo clippy --lib ir
  ```

### QA-00-07: IR Compiler
- **Plan Ref:** [Task 06](../../plans/00-foundation/tasks/06-ir-compiler.md)
- **Schema Ref:** Full schema validation
- **Priority:** P0
- **Test Type:** Integration + Unit
- **Criteria:**
  1. WorkflowSpec → WorkflowIR pipeline works end-to-end
  2. All test fixtures parse correctly and compile to valid IR
  3. Type checking catches all schema violations
  4. Error messages include line numbers and context
  5. Complex workflows with all features compile successfully
  6. IR types are strictly typed (no `serde_yaml::Value`)
  7. Performance: compilation < 500ms for typical workflows
- **Commands:**
  ```bash
  cargo test --test '*compiler*'
  cargo test --test '*integration*' -- compiler pipeline
  ```

### QA-00-08: DAG Validator
- **Plan Ref:** [Task 07](../../plans/00-foundation/tasks/07-dag-validator.md)
- **Schema Ref:** Section 4 (Pipeline Definition - dependencies, branches)
- **Priority:** P0
- **Test Type:** Unit + Property
- **Criteria:**
  1. Topological sort correctly orders steps
  2. Circular references detected and reported with error
  3. Dependency graph is acyclic
  4. All valid DAG structures validate correctly
  5. Invalid workflows produce clear DAG violation errors
  6. Property tests pass 1000 iterations without failures
- **Commands:**
  ```bash
  cargo test --lib dag
  cargo test --lib property_based -- --test-threads=1
  ```

### QA-00-09: Policy Compiler
- **Plan Ref:** [Task 08](../../plans/00-foundation/tasks/08-policy-compiler.md)
- **Schema Ref:** Section 17 (Scope & Inheritance Rules)
- **Priority:** P0
- **Test Type:** Unit
- **Criteria:**
  1. Policy fields compile deterministically (same input = same output)
  2. Inheritance rules enforce correct precedence (step > workflow > default)
  3. Override rules apply correctly
  4. Scope hierarchy works (parent → child → step)
  5. No circular dependencies in inheritance graph
  6. Policy values match expected defaults and overrides
- **Commands:**
  ```bash
  cargo test --lib policy
  cargo clippy --lib policy
  ```

### QA-00-10: Local Storage
- **Plan Ref:** [Task 09](../../plans/00-foundation/tasks/09-local-storage.md)
- **Schema Ref:** Section 13 (Workspace Configuration)
- **Priority:** P0
- **Test Type:** Integration + Unit
- **Criteria:**
  1. ./workspace/ directory structure created correctly
  2. sled KV store operations are atomic (ACID transactions)
  3. Storage operations are recoverable after crashes
  4. File system operations respect permission checks
  5. Checkpoint save/load works correctly
  6. Storage serialization is lossless
  7. Concurrent access is thread-safe (RwLock where needed)
- **Commands:**
  ```bash
  cargo test --lib storage
  cargo test --test '*storage*' -- sled operations
  ```

### QA-00-11: Workspace Management
- **Plan Ref:** [Task 10](../../plans/00-foundation/tasks/10-workspace-management.md)
- **Schema Ref:** Section 13 (Workspace Configuration)
- **Priority:** P0
- **Test Type:** Unit + Integration
- **Criteria:**
  1. Workspace path resolution works (relative and absolute paths)
  2. Directory creation respects configured permissions
  3. Missing directories are created with correct ownership
  4. Workspace cleanup works correctly
  5. Path traversal attacks are prevented
  6. Workspace configuration is loaded correctly
- **Commands:**
  ```bash
  cargo test --lib workspace
  cargo test --test '*workspace*' -- directory management
  ```

### QA-00-12: Threshold Validation
- **Plan Ref:** [Task 12](../../plans/00-foundation/tasks/12-threshold-validation.md)
- **Schema Ref:** Section 18 (Numeric Threshold Guidance)
- **Priority:** P0
- **Test Type:** Unit
- **Criteria:**
  1. All 11 numeric thresholds validate correctly with range checks
  2. Range checks enforce minimum/maximum values (e.g., max_allowed.ram: 0-100%)
  3. Default values applied when threshold not specified
  4. Error messages include actual vs expected values
  5. Percentage parsing works ("13%" → value 13)
  6. Absolute parsing works ("3.7GB" → 3968 MB)
  7. Threshold validation happens at parse time, not runtime
- **Commands:**
  ```bash
  cargo test --lib thresholds
  cargo test --test '*validation*' -- threshold checks
  ```

---

## Verification Layers

### Layer 1: Unit Tests
- **Scope:** All modules implemented in Phase 0
- **Evidence Required:** `cargo test --lib` output showing 100% pass rate
- **Validation Commands:**
  ```bash
  cargo test --lib
  ```
- **Pass Criteria:**
  - All 12 task groups pass
  - No panics in test output
  - Coverage >= 90% for new code

### Layer 2: Integration Tests
- **Scope:** YAML → WorkflowSpec → WorkflowIR pipeline
- **Evidence Required:** End-to-end test execution logs
- **Validation Commands:**
  ```bash
  cargo test --test 'integration*' -- --test-threads=1 --nocapture
  ```
- **Pass Criteria:**
  - Full pipeline test passes
  - All fixture workflows execute successfully
  - No race conditions or deadlocks

### Layer 3: Property Tests
- **Scope:** Invariants for DAG, IR compilation, and policy compilation
- **Evidence Required:** Proptest output with 1000 iterations
- **Validation Commands:**
  ```bash
  PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based
  ```
- **Pass Criteria:**
  - All properties hold for 1000 iterations
  - No shrinking required (no failures)
  - Round-trip invariants verified

### Layer 4: Build Verification
- **Scope:** Full project compilation
- **Evidence Required:** Clean build output
- **Validation Commands:**
  ```bash
  cargo build --release --all-features
  ```
- **Pass Criteria:**
  - Successful build with exit code 0
  - No warnings in release build
  - No compilation errors

### Layer 5: Schema Validation
- **Scope:** Unified schema compliance
- **Evidence Required:** schemars-generated JSON schema
- **Validation Commands:**
  ```bash
  cargo test --test schema_validation
  ```
- **Pass Criteria:**
  - All schema sections (1, 2, 13-18) implemented
  - schemars generates valid JSON schema
  - Schema validation tests pass

### Layer 6: Clippy / Code Quality
- **Scope:** All Rust source files
- **Evidence Required:** Zero clippy warnings
- **Validation Commands:**
  ```bash
  cargo clippy --all-features -- -W clippy::all
  ```
- **Pass Criteria:**
  - Zero warnings (or only justified warnings)
  - No unsafe code without comments
  - No `as any` type suppressions

### Layer 7: Memory Safety
- **Scope:** Rust unsafe code usage
- **Evidence Required:** Audit of unsafe blocks
- **Validation Commands:**
  ```bash
  cargo test --lib
  grep -r "unsafe" src/ | wc -l  # Should be minimal/zero
  ```
- **Pass Criteria:**
  - Zero or minimal unsafe code
  - All unsafe blocks justified with comments
  - No memory safety violations in tests

---

## Success Criteria

Phase 0 is complete when:

1. **Schema Coverage:** All Sections 1, 2, 13-18 of unified schema implemented
2. **Parser Correctness:** YAML → WorkflowSpec parsing with accurate error reporting
3. **IR Compilation:** WorkflowSpec → WorkflowIR pipeline passes all tests
4. **Validation:** DAG validation, threshold validation, and policy compilation all pass
5. **Persistence:** Local ./workspace/ directory structure with sled persistence
6. **Test Coverage:** All unit, integration, and property tests pass with >= 90% coverage
7. **Verification:** All 7 verification layers pass with zero errors
8. **Zero External Dependencies:** All external services use in-memory test fixtures

---

## Anti-Goal-Drift Prevention

### Requirements Drift
- Verify implementation matches only schema sections 1, 2, 13-18
- No features from later phases (3, 4, 5, 6, 7) implemented early
- All tasks stay within their specified scope

### Architecture Drift
- Schema → Parser → IR → Compiler pipeline matches architecture diagrams
- No deviation from ADR-0001 (Compiler approach decision)
- Module boundaries respected (schema/, parser/, ir/, storage/)

### Performance Drift
- Parsing < 100ms for typical workflows (established baseline)
- Compilation < 500ms for typical workflows
- No N^2 complexity in DAG validation

---

## Next Steps

Phase 0 complete when:
- ✅ All 12 QA areas validated with evidence
- ✅ All 7 verification layers pass
- ✅ All test fixtures parse and execute correctly
- ✅ Zero external dependencies verified
- ✅ Documentation updated

Proceed to Phase 1 (Core Execution Engine) implementation.
