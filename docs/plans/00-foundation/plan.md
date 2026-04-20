# Phase 0 Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the foundational infrastructure for the AgentSDK Execution Engine, implementing YAML parsing, schema validation, WorkflowIR compilation, and persistence with zero external dependencies.

**Architecture:** Multi-layer architecture with strict separation of concerns:
- **Schema Layer**: Rust types implementing the unified workflow schema (Sections 1,2,13-18)
- **Parser Layer**: YAML→WorkflowSpec parsing with yaml_serde and error reporting
- **IR Layer**: WorkflowIR typed internal representation for execution
- **Compiler Layer**: WorkflowSpec→WorkflowIR compilation pipeline with validation
- **Policy Layer**: Deterministic policy field compilation and inheritance
- **Storage Layer**: Local ./workspace/ directory structure with sled persistence
- **Validation Layer**: DAG validation, circular reference detection, threshold validation

**Tech Stack:**
- **YAML Parsing**: yaml_serde (1.5x faster than serde_yaml, schema validation)
- **Serialization**: serde (derive macros for all types)
- **Schema Generation**: schemars (JSON schema for external tools)
- **Hashing**: sha2 (SHA256 for workflow fingerprints)
- **UUIDs**: uuid (v4 for execution IDs)
- **Time**: chrono (timestamps and durations)
- **Persistence**: sled (embedded key-value store, ACID transactions)
- **Testing**: Built-in with cargo test, property-based testing

**Estimated Time:** 6-8 weeks (12 tasks, 1.5-2 hours per checkpoint)

**ADR-0001 Compliance:**
- ✅ Compiler approach selected (8.7/10 score vs 3.8/10 for transpiler)
- ✅ Rust crate integration with yaml_serde, serde, schemars
- ✅ Zero external dependencies in Phase 0 (use in-memory test fixtures)
- ✅ Test-first approach with failing tests before implementation
- ✅ Frequent commits with clear, atomic changes
- ✅ Verification layers with measurable pass criteria

---

## Schema Domain Ownership

Phase 0 owns these sections of the unified schema:

### Section 1: Workflow Identification
- `workflow_id`: Unique identifier for this workflow
- `name`: Human-readable workflow name
- `description`: Detailed description of workflow purpose
- `version`: Workflow version (semantic versioning)
- `author`: Workflow author
- `tags`: Categorization tags for discovery

### Section 2: Model Configuration
- Provider types: lmstudio, ollama, llama_cpp_with_vulkan
- Parameter tuning: temperature, top_p, max_tokens
- Lifecycle: load_unload strategies, hooks
- Cost tracking: token counting, resource usage

### Section 13: Workspace Configuration
- `root_path`: Root path for workspace
- `directories`: Output, checkpoint, log directories
- `permissions`: File system permissions

### Section 14: Features Demonstrated
- 18 feature categories from example workflows
- Feature compatibility checks

### Section 15: Variable Interpolation Syntax
- `${...}`: Parse-time interpolation (YAML parsing phase)
- `{{...}}`: Runtime interpolation (execution phase)

### Section 16: Default Behavior Reference
- All defaults for optional fields
- Default values for model configuration
- Default values for execution settings

### Section 17: Scope & Inheritance Rules
- `tool_permissions`: Inheritance from parent scopes
- `logging`: Hierarchical logging configuration
- `retry`: Retry strategy inheritance
- `models`: Model override rules

### Section 18: Numeric Threshold Guidance
- 11 thresholds with ranges and defaults:
  1. `max_allowed.ram`: 0-100% (default: 13%)
  2. `max_allowed.vram`: 0-100% or absolute (default: 3.7GB)
  3. `max_allowed.cpu`: 0-100% (default: 49%)
  4. `max_allowed.gpu`: 0-100% (default: 74%)
  5. `max_allowed.attention_tokens`: 1-1,000,000 (default: 150,000)
  6. `min_allowed.ram`: 0-100% (default: 9%)
  7. `min_allowed.vram`: 0-100% or absolute (default: 2.4GB)
  8. `min_allowed.cpu`: 0-100% (default: 49%)
  9. `min_allowed.gpu`: 0-100% (default: 74%)
  10. `min_allowed.attention_tokens`: 1-1,000,000 (default: 73,500)
  # 11. `max_concurrent_requests`: 1-10 (default: 2) — kept for provider-level concurrency

---

## 📋 Task Breakdown

### Task 0: Cargo Project Setup
**File:** `tasks/00-cargo-project-setup.md`
**Dependencies:** None
**Estimated Time:** 2 hours

Initialize Cargo.toml with all dependencies, project structure, and CI configuration.

### Task 1: Error Module
**File:** `tasks/01-error-module.md`
**Dependencies:** Task 0
**Estimated Time:** 3 hours

Extend existing `src/error.rs` (151 lines) with full error hierarchy for parsing, validation, IR compilation, and persistence.

### Task 2: Schema Types
**File:** `tasks/02-schema-types.md`
**Dependencies:** Task 1
**Estimated Time:** 8 hours

Define ALL Rust structs for the unified schema (WorkflowSpec, ModelConfig, Step, Loop, etc.) with serde derives and garde validation.

### Task 3: YAML Parser
**File:** `tasks/03-yaml-parser.md`
**Dependencies:** Task 2
**Estimated Time:** 6 hours

Implement YAML→WorkflowSpec parsing with yaml_serde, error reporting, and line number tracking.

### Task 4: Variable Interpolation
**File:** `tasks/04-variable-interpolation.md`
**Dependencies:** Task 2
**Estimated Time:** 5 hours

Implement `${...}` (parse-time) and `{{...}}` (runtime) interpolation with proper scoping.

### Task 5: Workflow IR
**File:** `tasks/05-workflow-ir.md`
**Dependencies:** Task 2
**Estimated Time:** 6 hours

Define WorkflowIR typed internal representation with strict type checking.

### Task 6: IR Compiler
**File:** `tasks/06-ir-compiler.md`
**Dependencies:** Task 3, Task 4, Task 5
**Estimated Time:** 8 hours

Implement WorkflowSpec→WorkflowIR compilation pipeline with type checking and validation.

### Task 7: DAG Validator
**File:** `tasks/07-dag-validator.md`
**Dependencies:** Task 6
**Estimated Time:** 4 hours

Implement DAG validation and circular reference detection using topological sort.

### Task 8: Policy Compiler
**File:** `tasks/08-policy-compiler.md`
**Dependencies:** Task 6
**Estimated Time:** 5 hours

Implement deterministic policy field compilation with inheritance and override rules.

### Task 9: Local Storage
**File:** `tasks/09-local-storage.md`
**Dependencies:** Task 6
**Estimated Time:** 6 hours

Implement ./workspace/ directory structure and persistence with sled (ACID transactions).

### Task 10: Workspace Management
**File:** `tasks/10-workspace-management.md`
**Dependencies:** Task 9
**Estimated Time:** 4 hours

Implement workspace path resolution and directory creation with permission checks.

### Task 11: Defaults, Scope & Inheritance
**File:** `tasks/11-defaults-scope-inheritance.md`
**Dependencies:** Task 8, Task 10
**Estimated Time:** 5 hours

Implement default values, scope hierarchy, and inheritance rules for all policy fields.

### Task 12: Threshold Validation
**File:** `tasks/12-threshold-validation.md`
**Dependencies:** Task 2
**Estimated Time:** 4 hours

Implement numeric threshold validation for 11 thresholds with range checks and default enforcement.

---

## Verification Layers

### Layer 1: LSP Diagnostics
**Tool:** `lsp_diagnostics`
**Pass Criteria:** No errors on all modified source files
**Scope:** All Rust source files in src/
**Frequency:** After each task completion

### Layer 2: Unit Tests
**Tool:** `cargo test`
**Pass Criteria:** All tests pass, 100% of defined tests run
**Scope:** Individual module tests for each component
**Frequency:** After each task completion

### Layer 3: Integration Tests
**Tool:** `cargo test --test integration`
**Pass Criteria:** All integration tests pass
**Scope:** YAML→WorkflowSpec→WorkflowIR pipeline
**Frequency:** After tasks 3, 6, 9, 11

### Layer 4: Property Tests
**Tool:** `cargo test --test properties`
**Pass Criteria:** All property tests pass with 100+ test cases
**Scope:** Determinism, round-trips, invariants
**Frequency:** After tasks 6, 11

### Layer 5: Build Verification
**Tool:** `cargo build --release`
**Pass Criteria:** Successful build, no warnings
**Scope:** Full project compilation
**Frequency:** After tasks 2, 6, 11, 12

### Layer 6: Schema Validation
**Tool:** `cargo test --test schema_validation`
**Pass Criteria:** All schema validation tests pass
**Scope:** Unified schema compliance
**Frequency:** After task 2, task 6

### Layer 7: Memory Safety
**Tool:** `cargo clippy`
**Pass Criteria:** No clippy warnings, no unsafe code (unless justified)
**Scope:** All Rust source files
**Frequency:** After each task completion

---

## Mock Strategies

Phase 0 has NO external dependencies. All external services use in-memory test fixtures:

### Model Provider Mocks
- **LM Studio**: Mock connection settings, in-memory model registry
- **Ollama**: Mock API responses, pre-canned inference results
- **llama.cpp**: Mock GGUF model loading, simulated inference

### Tool Execution Mocks
- **File Operations**: In-memory file system for tests
- **Web Operations**: Mock HTTP responses with pre-canned data
- **Shell Operations**: Mock command execution with expected outputs

### State Persistence Mocks
- **sled Database**: In-memory sled instance for tests
- **Directory Structure**: Temporary directories with test fixtures
- **Checkpoints**: In-memory checkpoint storage for tests

### Why No External Dependencies?
1. **Speed**: Tests run in < 100ms without network I/O
2. **Reliability**: No flaky tests due to external service failures
3. **Isolation**: Tests run in CI without external dependencies
4. **Determinism**: Same inputs always produce same outputs
5. **Portability**: Tests run anywhere without setup

---

## Anti-Goal-Drift Checkpoints

### Checkpoint 1 (After Task 2)
- [ ] All schema types compile without errors
- [ ] serde derives generate correct Serialize/Deserialize implementations
- [ ] schemars generates valid JSON schema
- [ ] garde validation rules compile
- [ ] No unused fields or dead code warnings

**Anti-Drift Check:** Verify task 2 implements ONLY schema types (Section 1,2,13-18). No parser or IR code yet.

### Checkpoint 2 (After Task 6)
- [ ] YAML→WorkflowSpec→WorkflowIR pipeline works end-to-end
- [ ] All test fixtures from example-workflows/ parse correctly
- [ ] IR types are strictly typed (no `serde_yaml::Value` anywhere)
- [ ] Type checking catches all schema violations
- [ ] Error messages include line numbers and context

**Anti-Drift Check:** Verify task 6 compiles to IR only. No DAG validation, policy compilation, or persistence yet.

### Checkpoint 3 (After Task 11)
- [ ] All default values are applied correctly
- [ ] Scope inheritance works (parent→child→step)
- [ ] Policy fields compile deterministically (same input = same output)
- [ ] Override rules enforce precedence (step > workflow > default)
- [ ] No circular dependencies in inheritance graph

**Anti-Drift Check:** Verify task 11 implements ONLY defaults and inheritance. No persistence or workspace management yet.

### Checkpoint 4 (After Task 12)
- [ ] All 11 numeric thresholds validate correctly
- [ ] Range checks enforce minimum/maximum values
- [ ] Default values applied when threshold not specified
- [ ] Error messages include actual vs expected values
- [ ] Threshold validation happens at parse time, not runtime

**Anti-Drift Check:** Verify task 12 implements ONLY threshold validation. All other validation (DAG, policy) already tested.

### Final Checkpoint (After Task 12)
- [ ] All 7 verification layers pass
- [ ] All 12 tasks complete with passing tests
- [ ] All acceptance criteria met (see validation/acceptance-criteria.md)
- [ ] Zero external dependencies in code (all mocked)
- [ ] Full integration test suite passes
- [ ] Property tests demonstrate correctness invariants

---

## OpenCode Tool Specifications

### lsp_diagnostics
```bash
# Run on all modified files
lsp_diagnostics filePath=/home/jon/code/whitt-execution-engine/src/ severity=all

# Expected output:
# - No errors
# - Zero or minimal warnings (only justified)
```

### cargo test
```bash
# Run all tests
cargo test

# Expected output:
# - test result: ok. X passed in Y.ZZs
# - All unit tests pass
# - No test failures or panics
```

### cargo build
```bash
# Build in release mode
cargo build --release

# Expected output:
# - Compiling whitt-execution-engine v0.1.0
# - Finished release [optimized] target(s) in X.XXs
# - No warnings (or only justified ones)
```

### cargo clippy
```bash
# Run clippy with all lints
cargo clippy -- -D warnings

# Expected output:
# - warning: unused variable (if any)
# - No clippy warnings that indicate bugs
# - All unsafe code justified with comments
```

---

## ✅ Success Criteria

Phase 0 is complete when:

1. **Schema Coverage**: All Sections 1,2,13-18 of unified schema implemented
2. **Parser Correctness**: YAML→WorkflowSpec parsing with accurate error reporting
3. **IR Compilation**: WorkflowSpec→WorkflowIR pipeline passes all tests
4. **Validation**: DAG validation, threshold validation, policy compilation all pass
5. **Persistence**: Local ./workspace/ directory structure with sled persistence
6. **Test Coverage**: All unit, integration, and property tests pass
7. **Verification**: All 7 verification layers pass with zero errors

**Exit Criteria:** See `validation/acceptance-criteria.md` for detailed acceptance criteria.

---

## Next Phase Dependencies

Phase 1 (MVP Queue & Scheduler) depends on Phase 0:
- WorkflowIR for job execution context
- Persistence for queue state storage
- Validation for job acceptance criteria
- Error handling for queue failures

**Phase 0 must be complete and verified before starting Phase 1.**

---

## Getting Started

1. **Read the plan**: This document provides the complete roadmap
2. **Review tasks**: Read each task file in `tasks/` for detailed implementation steps
3. **Execute sequentially**: Tasks are ordered by dependencies
4. **Verify frequently**: Run verification layers after each task
5. **Check anti-drift**: Ensure tasks don't drift into later-phase work
6. **Commit often**: Each task ends with a clear commit message

**Total Estimated Effort:** 60-80 hours (12 tasks × 5-6.5 hours each)

---

## Appendix

### A. Schema Section Mapping

| Schema Section | Phase | Task | File |
|---------------|-------|------|------|
| Section 1: Workflow Identification | 0 | 2,3 | src/schema/identification.rs |
| Section 2: Model Configuration | 0 | 2,3 | src/schema/model.rs |
| Section 13: Workspace Configuration | 0 | 2,10 | src/schema/workspace.rs |
| Section 14: Features Demonstrated | 0 | 2 | src/schema/features.rs |
| Section 15: Variable Interpolation | 0 | 4 | src/interpolation.rs |
| Section 16: Default Behavior | 0 | 11 | src/defaults.rs |
| Section 17: Scope & Inheritance | 0 | 8,11 | src/policy.rs |
| Section 18: Numeric Thresholds | 0 | 12 | src/validation/thresholds.rs |

### B. Verification Matrix

| Task | Layer 1 | Layer 2 | Layer 3 | Layer 4 | Layer 5 | Layer 6 | Layer 7 |
|------|---------|---------|---------|---------|---------|---------|---------|
| 0 | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| 1 | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| 2 | ✅ | ✅ | ❌ | ❌ | ✅ | ✅ | ✅ |
| 3 | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ |
| 4 | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| 5 | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| 6 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| 7 | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ |
| 8 | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ |
| 9 | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ |
| 10 | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ |
| 11 | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| 12 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

**Legend:** ✅ = Run after task, ❌ = Not applicable yet

### C. Error Hierarchy

```
Error
├── ParseError (YAML parsing)
│   ├── SyntaxError (YAML syntax)
│   ├── ValidationError (Schema validation)
│   └── InterpolationError (Variable interpolation)
├── CompilationError (IR compilation)
│   ├── TypeError (Type checking)
│   ├── ReferenceError (Undefined references)
│   └── CircularDependencyError (Circular references)
├── PolicyError (Policy compilation)
│   ├── InheritanceError (Invalid inheritance)
│   └── OverrideError (Invalid override)
├── StorageError (Persistence)
│   ├── DatabaseError (Sled errors)
│   ├── FileSystemError (File system errors)
│   └── SerializationError (Serialization errors)
└── ValidationError (Threshold validation)
    ├── ThresholdError (Threshold out of range)
    └── DefaultError (Invalid default value)
```

### D. Test Fixture Locations

```
tests/
├── fixtures/
│   ├── workflows/          # YAML workflow fixtures
│   │   ├── minimal.yml     # Minimal valid workflow
│   │   ├── complex.yml     # Complex workflow with all features
│   │   └── invalid/        # Invalid workflows for error testing
│   ├── models/             # Model configuration fixtures
│   ├── policies/            # Policy configuration fixtures
│   └── thresholds/         # Threshold test fixtures
├── unit/                   # Unit tests
├── integration/            # Integration tests
└── properties/             # Property-based tests
```

---

**Plan Version:** 1.0.0
**Last Updated:** 2026-04-06
**Author:** AI Planning System
**Status:** Ready for Execution
