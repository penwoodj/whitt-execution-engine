# Task 1: Error Module

**Goal:** Extend existing `src/error.rs` (151 lines) with full error hierarchy for parsing, validation, IR compilation, and persistence.

**Estimated Time:** 3 hours

**Dependencies:** Task 0

**Files:**
- Modify: `src/error.rs` (extend from 151 to ~300 lines)
- Create: `tests/error_test.rs` (error tests)

---

## Step 1: Extend error types

Add these error variants to `src/error.rs`:

```rust
#[error("Parse error at line {line}, column {column}: {message}")]
Parse {
    line: usize,
    column: usize,
    message: String,
},

#[error("YAML syntax error: {message}")]
YamlSyntax { message: String },

#[error("Schema validation error: {field}: {message}")]
SchemaValidation {
    field: String,
    message: String,
    value: Option<String>>,
},

#[error("Type error: expected {expected}, found {found}")]
Type {
    expected: String,
    found: String,
    location: String,
},

#[error("Undefined reference: {reference}")]
UndefinedReference { reference: String },

#[error("Circular dependency detected: {cycle}")]
CircularDependency { cycle: String },

#[error("Policy inheritance error: {message}")]
PolicyInheritance { message: String },

#[error("Policy override error: {message}")]
PolicyOverride { message: String },

#[error("Variable interpolation error: {variable}")]
Interpolation { variable: String },

#[error("Unknown interpolation scope: {scope}")]
UnknownScope { scope: String },

#[error("Storage error: {0}")]
Storage(#[from] sled::Error),

#[error("Serialization error: {0}")]
Serialization(#[from] bincode::Error),

#[error("Database error: {operation}: {message}")]
Database {
    operation: String,
    message: String,
},

#[error("File system error: {operation}: {path}")]
FileSystem {
    operation: String,
    path: PathBuf,
    reason: String,
},

#[error("Threshold validation error: {threshold}: {message}")]
Threshold {
    threshold: String,
    message: String,
    actual: Option<f64>,
    expected: Option<String>>,
},

#[error("Default value error: {field}: {message}")]
Default {
    field: String,
    message: String,
    value: String,
},

#[error("DAG validation error: {message}")]
DagValidation { message: String },

#[error("IR compilation error: {message}")]
IrCompilation { message: String },

#[error("Invalid workflow version: {version}")]
InvalidVersion { version: String },
```

**Commit:** `feat: add Phase 0 error types`

---

## Step 2: Add error constructors

Add these convenience constructors to `src/error.rs`:

```rust
impl Error {
    pub fn parse(line: usize, column: usize, message: impl Into<String>>) -> Self {
        Self::Parse {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn yaml_syntax(message: impl Into<String>>) -> Self {
        Self::YamlSyntax {
            message: message.into(),
        }
    }

    pub fn schema_validation(
        field: impl Into<String>>,
        message: impl Into<String>>,
        value: Option<String>>,
    ) -> Self {
        Self::SchemaValidation {
            field: field.into(),
            message: message.into(),
            value,
        }
    }

    pub fn type_error(
        expected: impl Into<String>>,
        found: impl Into<String>>,
        location: impl Into<String>>,
    ) -> Self {
        Self::Type {
            expected: expected.into(),
            found: found.into(),
            location: location.into(),
        }
    }

    pub fn undefined_reference(reference: impl Into<String>>) -> Self {
        Self::UndefinedReference {
            reference: reference.into(),
        }
    }

    pub fn circular_dependency(cycle: impl Into<String>>) -> Self {
        Self::CircularDependency {
            cycle: cycle.into(),
        }
    }

    pub fn policy_inheritance(message: impl Into<String>>) -> Self {
        Self::PolicyInheritance {
            message: message.into(),
        }
    }

    pub fn policy_override(message: impl Into<String>>) -> Self {
        Self::PolicyOverride {
            message: message.into(),
        }
    }

    pub fn interpolation(variable: impl Into<String>>) -> Self {
        Self::Interpolation {
            variable: variable.into(),
        }
    }

    pub fn unknown_scope(scope: impl Into<String>>) -> Self {
        Self::UnknownScope {
            scope: scope.into(),
        }
    }

    pub fn database(operation: impl Into<String>>, message: impl Into<String>>) -> Self {
        Self::Database {
            operation: operation.into(),
            message: message.into(),
        }
    }

    pub fn file_system(
        operation: impl Into<String>>,
        path: PathBuf,
        reason: impl Into<String>>,
    ) -> Self {
        Self::FileSystem {
            operation,
            path,
            reason: reason.into(),
        }
    }

    pub fn threshold(
        threshold: impl Into<String>>,
        message: impl Into<String>>,
        actual: Option<f64>,
        expected: Option<String>>,
    ) -> Self {
        Self::Threshold {
            threshold: threshold.into(),
            message: message.into(),
            actual,
            expected,
        }
    }

    pub fn default(
        field: impl Into<String>>,
        message: impl Into<String>>,
        value: impl Into<String>>,
    ) -> Self {
        Self::Default {
            field: field.into(),
            message: message.into(),
            value: value.into(),
        }
    }

    pub fn dag_validation(message: impl Into<String>>) -> Self {
        Self::DagValidation {
            message: message.into(),
        }
    }

    pub fn ir_compilation(message: impl Into<String>>) -> Self {
        Self::IrCompilation {
            message: message.into(),
        }
    }

    pub fn invalid_version(version: impl Into<String>>) -> Self {
        Self::InvalidVersion {
            version: version.into(),
        }
    }
}
```

**Commit:** `feat: add error constructors`

---

## Step 3: Add bincode to Cargo.toml

Add bincode for serialization:

```toml
# Binary serialization for sled storage
bincode = "1.3"
```

**Commit:** `chore: add bincode dependency for storage serialization`

---

## Step 4: Write error tests

Create `tests/error_test.rs`:

```rust
use whitt_execution_engine::Error;

#[test]
fn test_parse_error() {
    let error = Error::parse(10, 5, "unexpected token");
    assert_eq!(
        error.to_string(),
        "Parse error at line 10, column 5: unexpected token"
    );
}

#[test]
fn test_type_error() {
    let error = Error::type_error("String", "Int", "field: models.primary.temperature");
    assert_eq!(
        error.to_string(),
        "Type error: expected String, found Int at field: models.primary.temperature"
    );
}

#[test]
fn test_undefined_reference() {
    let error = Error::undefined_reference("step.nonexistent.output");
    assert_eq!(
        error.to_string(),
        "Undefined reference: step.nonexistent.output"
    );
}

#[test]
fn test_circular_dependency() {
    let error = Error::circular_dependency("step_1 -> step_2 -> step_3 -> step_1");
    assert_eq!(
        error.to_string(),
        "Circular dependency detected: step_1 -> step_2 -> step_3 -> step_1"
    );
}

#[test]
fn test_threshold_error() {
    let error = Error::threshold(
        "max_allowed.ram",
        "value 120 exceeds maximum 100",
        Some(120.0),
        Some("0-100".to_string()),
    );
    assert_eq!(
        error.to_string(),
        "Threshold validation error: max_allowed.ram: value 120 exceeds maximum 100 (actual: 120, expected: 0-100)"
    );
}

#[test]
fn test_error_display() {
    let errors = vec![
        Error::yaml_syntax("invalid YAML syntax"),
        Error::schema_validation("workflow_id", "required field missing", None),
        Error::interpolation("${unknown.variable}"),
    ];

    for error in errors {
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn test_error_source() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error: Error = io_error.into();
    assert!(matches!(error, Error::Io(_))));
}

#[test]
fn test_error_debug() {
    let error = Error::parse(1, 1, "test");
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("Parse"));
}
```

**Commit:** `test: add error module tests`

---

## Step 5: Run tests

Verify all tests pass:

```bash
cargo test error_test

# Expected output:
# test result: ok. X passed in Y.ZZs
```

**Commit:** `fix: resolve any test failures`

---

## Verification

After completing all steps, verify:

```bash
# 1. Build passes
cargo build
# Expected: Finished dev [unoptimized + debuginfo] target(s)

# 2. All tests pass
cargo test error_test
# Expected: test result: ok. X passed

# 3. Error types are comprehensive
# Check that all error variants from plan.md are implemented

# 4. Error constructors work
# Test each constructor returns to expected error type
```

**Checkpoint Criteria:**
- ✅ All Phase 0 error types added (20+ variants)
- ✅ All error constructors implemented
- ✅ bincode dependency added
- ✅ Error tests created and passing
- ✅ Error display formatting is clear and helpful
- ✅ Error sources are properly wrapped (thiserror)

**Next:** Proceed to Task 2 (Schema Types)

---

## Implementation Status

**Status**: ⚠️ PARTIAL

### What Exists
- **[src/error.rs](../../src/error.rs)**: Basic error module implemented with:
  - `Error` enum with core error variants (60+ lines) ✅
  - `Result<T>` type alias ✅
  - Error variants: `Config`, `Model`, `Agent`, `Backend`, `Client`, `Tool`, `Storage`, `Serialization`, `Io`, `Validation` ✅
  - `Config` variants: `Missing`, `Parse`, `Validation`, `Merge` ✅
  - `Model` variants: `NotFound`, `AlreadyLoaded`, `LoadFailed`, `UnloadFailed`, `InvalidState` ✅
  - `Agent` variants: `ExecutionFailed`, `ToolFailed`, `MaxIterationsExceeded`, `InvalidStepReference`, `InvalidLoopCondition` ✅
  - `Backend` variants: `Connection`, `Timeout`, `RateLimited`, `ModelNotLoaded`, `InvalidRequest` ✅
  - `Client` variants: `RequestFailed`, `ResponseParse`, `DownloadFailed` ✅
  - `Tool` variants: `ExecutionFailed`, `PermissionDenied`, `InvalidArguments` ✅
  - `Storage`, `Serialization`, `Io`, `Validation` base variants ✅

- **Error constructors** (partial):
  - Some constructors implemented (e.g., `from_io`, `from_serialization`, `from_model`) ✅
  - `thiserror::Error` derive attribute applied ✅

- **Tests**: `tests/error_test.rs` exists with basic error tests ✅

- **Dependencies**: `bincode` present in [Cargo.toml](../../Cargo.toml) (line 22) ✅

### What's Missing
- **Error variants** not implemented (per plan):
  - `Parse` (line/column/message) - NOT IMPLEMENTED (plan lines 20-25)
  - `YamlSyntax` (message) - NOT IMPLEMENTED (plan lines 27-28)
  - `SchemaValidation` (field/message/value) - NOT IMPLEMENTED (plan lines 30-35)
  - `Type` (expected/found/location) - NOT IMPLEMENTED (plan lines 37-42)
  - `UndefinedReference` (reference) - NOT IMPLEMENTED (plan lines 44-45)
  - `CircularDependency` (cycle) - NOT IMPLEMENTED (plan lines 47-48)
  - `PolicyInheritance` (message) - NOT IMPLEMENTED (plan lines 50-51)
  - `PolicyOverride` (message) - NOT IMPLEMENTED (plan lines 53-54)
  - `Interpolation` (variable) - NOT IMPLEMENTED (plan lines 56-57)
  - `UnknownScope` (scope) - NOT IMPLEMENTED (plan lines 59-60)
  - `Threshold` (threshold/message/actual/expected) - NOT IMPLEMENTED (plan lines 81-87)
  - `Default` (field/message/value) - NOT IMPLEMENTED (plan lines 89-94)
  - `DagValidation` (message) - NOT IMPLEMENTED (plan lines 96-97)
  - `IrCompilation` (message) - NOT IMPLEMENTED (plan lines 99-100)
  - `InvalidVersion` (version) - NOT IMPLEMENTED (plan lines 102-103)

- **Error constructors** not implemented (per plan):
  - `parse(line, column, message)` - NOT IMPLEMENTED (plan lines 116-122)
  - `yaml_syntax(message)` - NOT IMPLEMENTED (plan lines 124-128)
  - `schema_validation(field, message, value)` - NOT IMPLEMENTED (plan lines 130-140)
  - `type_error(expected, found, location)` - NOT IMPLEMENTED (plan lines 142-152)
  - `undefined_reference(reference)` - NOT IMPLEMENTED (plan lines 154-158)
  - `circular_dependency(cycle)` - NOT IMPLEMENTED (plan lines 160-164)
  - `policy_inheritance(message)` - NOT IMPLEMENTED (plan lines 166-170)
  - `policy_override(message)` - NOT IMPLEMENTED (plan lines 172-176)
  - `interpolation(variable)` - NOT IMPLEMENTED (plan lines 178-182)
  - `unknown_scope(scope)` - NOT IMPLEMENTED (plan lines 184-188)
  - `database(operation, message)` - NOT IMPLEMENTED (plan lines 190-195)
  - `file_system(operation, path, reason)` - NOT IMPLEMENTED (plan lines 197-207)
  - `threshold(threshold, message, actual, expected)` - NOT IMPLEMENTED (plan lines 209-221)
  - `default(field, message, value)` - NOT IMPLEMENTED (plan lines 223-232)
  - `dag_validation(message)` - NOT IMPLEMENTED (plan lines 235-239)
  - `ir_compilation(message)` - NOT IMPLEMENTED (plan lines 241-245)
  - `invalid_version(version)` - NOT IMPLEMENTED (plan lines 247-251)

- **Error type differences**:
  - Plan expects 20+ error variants with specific names (`Parse`, `YamlSyntax`, `SchemaValidation`, etc.)
  - Current implementation has different organization (Config, Model, Agent, Backend, etc.)
  - Plan uses ` sled::Error` wrapper for `Storage` variant (line 63)
  - Plan uses `bincode::Error` wrapper for `Serialization` variant (line 65)
  - Current implementation may have `Database` variant but it's structured differently than plan

### QA Coverage
- **Status**: No dedicated QA tests for this task in EPOC findings
- **Coverage**: From EPOC Extended POC findings:
  - **AREA-20 BUILD HYGIENE** (Area 20) - ✅ PASS - 91/91 tests passing, 0 clippy warnings
- **Note**: Error tests exist in `tests/error_test.rs` but use current implementation's error structure, not plan's expected structure

### Schema Alignment
- **Schema Ref**: Lines 14-21 (identification), Lines 196-353 (agentic_workflow section with errors)
- **Coverage**: Not applicable - error types don't map directly to schema lines
- **Gaps**: Error handling infrastructure exists but doesn't match plan's expected error hierarchy for schema validation and YAML parsing

### Evidence
- **Build**: ✅ `cargo build` passes
- **Clippy**: ✅ `cargo clippy -- -D warnings` passes
- **Tests**: ✅ `cargo test --lib` passes (includes error tests)
- **Line Count**: `src/error.rs` has 60+ lines (plan expects 151-300 lines)

### Plan vs Reality Notes
- **Plan structure**: 20+ specific error variants with hierarchical organization (Parse → YamlSyntax, SchemaValidation, Type, etc.)
- **Current reality**: Different error organization (Config, Model, Agent, Backend, Client, Tool categories)
- **Incompatibility**: Tests in plan use error constructors like `Error::parse(10, 5, "unexpected token")` which don't exist in current implementation
- **Dependencies**: `bincode` is in Cargo.toml but current code doesn't use sled/SQLite serialization directly (uses JSON file storage in persistence.rs)

---

## QA Cross-References

- **QA Criteria**: [QA-00-02](../../qa/phase-00/QA-CRITERIA.md)
- **Test Cases**: [P00-002](../../qa/phase-00/QA-TEST-CASES.md)
- **Schema Ref**: N/A (error handling)
