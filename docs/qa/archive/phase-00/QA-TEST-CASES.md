# Phase 00: Test Cases

### P00-001: Cargo Project Setup
- **Area:** QA-00-01
- **Type:** Build
- **Command:** `cargo check && cargo build --release`
- **Setup:** None (new project)
- **Expected:** Cargo.toml exists, project structure created, CI configured
- **Pass Criteria:** ✅ `cargo build --release` exits with code 0, no compilation errors

### P00-002: Error Module Compilation
- **Area:** QA-00-02
- **Type:** Unit
- **Command:** `cargo test --lib error && cargo clippy --lib error`
- **Setup:** Error module with thiserror derives
- **Expected:** All error types compile, no warnings
- **Pass Criteria:** ✅ All error tests pass, zero clippy warnings

### P00-003: Schema Type Serialization
- **Area:** QA-00-03
- **Type:** Unit
- **Command:** `cargo test --lib schema && cargo build --release`
- **Setup:** Schema types with serde derives
- **Expected:** All schema structs serialize/deserialize correctly
- **Pass Criteria:** ✅ Schema tests pass, successful release build

### P00-004: Schema Validation with Garde
- **Area:** QA-00-03
- **Type:** Unit
- **Command:** `cargo test --lib schema_validation`
- **Setup:** Schema with garde validation rules
- **Expected:** All garde rules enforce constraints correctly
- **Pass Criteria:** ✅ All validation tests pass, invalid schemas rejected

### P00-005: YAML Parser - Basic
- **Area:** QA-00-04
- **Type:** Unit
- **Command:** `cargo test --lib parser_basic`
- **Setup:** Minimal YAML workflow file
- **Expected:** YAML parses to WorkflowSpec successfully
- **Pass Criteria:** ✅ Test passes, WorkflowSpec has correct values

### P00-006: YAML Parser - Merge Keys
- **Area:** QA-00-04
- **Type:** Unit
- **Command:** `cargo test --lib parser_merge_keys`
- **Setup:** YAML with merge anchor (<<: *default)
- **Expected:** Merge key resolved correctly in parsed structure
- **Pass Criteria:** ✅ Merge override applied correctly

### P00-007: YAML Parser - Error Reporting
- **Area:** QA-00-04
- **Type:** Unit
- **Command:** `cargo test --lib parser_error_reporting`
- **Setup:** Invalid YAML workflow file
- **Expected:** Parser returns error with line number and context
- **Pass Criteria:** ✅ Error message includes location and helpful description

### P00-008: Variable Interpolation - Parse-Time
- **Area:** QA-00-05
- **Type:** Unit
- **Command:** `cargo test --lib interpolation_parse_time`
- **Setup:** YAML with `${...}` variables
- **Expected:** Variables resolve at parse time correctly
- **Pass Criteria:** ✅ All variables substituted, no unknown references

### P00-009: Variable Interpolation - Runtime
- **Area:** QA-00-05
- **Type:** Unit
- **Command:** `cargo test --lib interpolation_runtime`
- **Setup:** WorkflowIR with `{{...}}` placeholders
- **Expected:** Runtime interpolation resolves correctly
- **Pass Criteria:** ✅ All runtime variables substituted correctly

### P00-010: Variable Interpolation - Scoping
- **Area:** QA-00-05
- **Type:** Unit
- **Command:** `cargo test --lib interpolation_scoping`
- **Setup:** Nested variable references (parent/child/step)
- **Expected:** Variables resolve in correct scope order
- **Pass Criteria:** ✅ Scope hierarchy respected, no shadowing

### P00-011: Workflow IR - Round-Trip
- **Area:** QA-00-06
- **Type:** Unit
- **Command:** `cargo test --lib ir_round_trip`
- **Setup:** WorkflowSpec → WorkflowIR → WorkflowSpec
- **Expected:** Round-trip preserves all data without loss
- **Pass Criteria:** ✅ All fields preserved, no data corruption

### P00-012: Workflow IR - Type Safety
- **Area:** QA-00-06
- **Type:** Unit
- **Command:** `cargo clippy --lib ir`
- **Setup:** IR module with typed structs
- **Expected:** Zero clippy warnings on IR types
- **Pass Criteria:** ✅ No type safety violations

### P00-013: IR Compiler - End-to-End
- **Area:** QA-00-07
- **Type:** Integration
- **Command:** `cargo test --test '*compiler*' -- compiler_pipeline`
- **Setup:** Example workflow YAML
- **Expected:** Full compilation pipeline produces valid WorkflowIR
- **Pass Criteria:** ✅ Pipeline executes, IR validates against schema

### P00-014: IR Compiler - Error Messages
- **Area:** QA-00-07
- **Type:** Integration
- **Command:** `cargo test --test compiler_error_messages`
- **Setup:** Invalid workflow YAML
- **Expected:** Compiler returns error with line number
- **Pass Criteria:** ✅ Error message is actionable with context

### P00-015: DAG Validator - Acyclic
- **Area:** QA-00-08
- **Type:** Property
- **Command:** `PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib dag_acyclic`
- **Setup:** Workflow with complex dependency graph
- **Expected:** Topological sort produces valid ordering
- **Pass Criteria:** ✅ All iterations pass, DAG is acyclic

### P00-016: DAG Validator - Circular Detection
- **Area:** QA-00-08
- **Type:** Property
- **Command:** `PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib dag_circular`
- **Setup:** Workflow with circular step references
- **Expected:** Circular reference detected and reported
- **Pass Criteria:** ✅ Circular dependency found and error raised

### P00-017: Policy Compiler - Inheritance
- **Area:** QA-00-09
- **Type:** Unit
- **Command:** `cargo test --lib policy_inheritance`
- **Setup:** Workflow with step, workflow, and default scopes
- **Expected:** Inheritance rules apply correct precedence
- **Pass Criteria:** ✅ Step-level override wins over workflow-level

### P00-018: Policy Compiler - Determinism
- **Area:** QA-00-09
- **Type:** Unit
- **Command:** `cargo test --lib policy_determinism`
- **Setup:** Same policy config compiled multiple times
- **Expected:** Identical inputs produce identical policy output
- **Pass Criteria:** ✅ Output is deterministic, no randomness

### P00-019: Local Storage - Sled Operations
- **Area:** QA-00-10
- **Type:** Integration
- **Command:** `cargo test --test '*storage*' -- sled_operations`
- **Setup:** In-memory sled instance for tests
- **Expected:** All KV operations (insert, get, delete) work correctly
- **Pass Criteria:** ✅ Storage operations complete, no data loss

### P00-020: Local Storage - ACID Transactions
- **Area:** QA-00-10
- **Type:** Unit
- **Command:** `cargo test --test storage_transactions`
- **Setup:** Sled transaction with multiple operations
- **Expected:** All-or-nothing rollback behavior
- **Pass Criteria:** ✅ Transaction semantics verified, atomic operations

### P00-021: Workspace Management - Path Resolution
- **Area:** QA-00-11
- **Type:** Unit
- **Command:** `cargo test --lib workspace_path_resolution`
- **Setup:** Test cases with relative and absolute paths
- **Expected:** Paths resolve correctly to workspace root
- **Pass Criteria:** ✅ All path types handled, no traversal vulnerabilities

### P00-022: Workspace Management - Directory Creation
- **Area:** QA-00-11
- **Type:** Integration
- **Command:** `cargo test --test workspace_creation`
- **Setup:** Workspace creation on fresh system
- **Expected:** All required directories created with correct permissions
- **Pass Criteria:** ✅ Directory structure matches spec, permissions are correct

### P00-023: Threshold Validation - Numeric Ranges
- **Area:** QA-00-12
- **Type:** Unit
- **Command:** `cargo test --lib threshold_numeric_ranges`
- **Setup:** Config with all 11 numeric thresholds
- **Expected:** All thresholds enforce range constraints (e.g., max_allowed.ram: 0-100%)
- **Pass Criteria:** ✅ All range checks pass, invalid values rejected

### P00-024: Threshold Validation - Percentage Parsing
- **Area:** QA-00-12
- **Type:** Unit
- **Command:** `cargo test --lib threshold_percentage_parsing`
- **Setup:** Threshold values with percentage strings
- **Expected:** Percentages parse to numeric values (e.g., "13%" → 0.13)
- **Pass Criteria:** ✅ All percentage formats handled correctly

### P00-025: Threshold Validation - Absolute Parsing
- **Area:** QA-00-12
- **Type:** Unit
- **Command:** `cargo test --lib threshold_absolute_parsing`
- **Setup:** Threshold values with absolute strings (e.g., "3.7GB")
- **Expected:** Absolute values parse to byte counts (e.g., "3.7GB" → 3968 MB)
- **Pass Criteria:** ✅ All absolute formats handled correctly

### P00-026: Integration - Full Pipeline
- **Area:** All Phase 0 components
- **Type:** Integration
- **Command:** `cargo test --test phase00_full_pipeline`
- **Setup:** Complete workflow from YAML to IR to storage
- **Expected:** All components work together seamlessly
- **Pass Criteria:** ✅ End-to-end workflow executes successfully, output saved

### P00-027: Schema Coverage Audit
- **Area:** All schema sections
- **Type:** Automated
- **Command:** `cargo run --bin schema_audit -- --phase 00`
- **Setup:** Audit tool comparing implementation to schema
- **Expected:** 100% coverage of Phase 0-owned schema fields
- **Pass Criteria:** ✅ All schema fields implemented and tested

---

## Test Execution Order

1. **Unit Tests** (fastest)
2. **Integration Tests** (medium)
3. **Property Tests** (slower)
4. **End-to-End Tests** (slowest)
5. **Build Verification** (after changes)

**Stop on first failure:** Fix issues before proceeding
