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
    value: Option<String>,
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
    expected: Option<String>,
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
    pub fn parse(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::Parse {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn yaml_syntax(message: impl Into<String>) -> Self {
        Self::YamlSyntax {
            message: message.into(),
        }
    }

    pub fn schema_validation(
        field: impl Into<String>,
        message: impl Into<String>,
        value: Option<String>,
    ) -> Self {
        Self::SchemaValidation {
            field: field.into(),
            message: message.into(),
            value,
        }
    }

    pub fn type_error(
        expected: impl Into<String>,
        found: impl Into<String>,
        location: impl Into<String>,
    ) -> Self {
        Self::Type {
            expected: expected.into(),
            found: found.into(),
            location: location.into(),
        }
    }

    pub fn undefined_reference(reference: impl Into<String>) -> Self {
        Self::UndefinedReference {
            reference: reference.into(),
        }
    }

    pub fn circular_dependency(cycle: impl Into<String>) -> Self {
        Self::CircularDependency {
            cycle: cycle.into(),
        }
    }

    pub fn policy_inheritance(message: impl Into<String>) -> Self {
        Self::PolicyInheritance {
            message: message.into(),
        }
    }

    pub fn policy_override(message: impl Into<String>) -> Self {
        Self::PolicyOverride {
            message: message.into(),
        }
    }

    pub fn interpolation(variable: impl Into<String>) -> Self {
        Self::Interpolation {
            variable: variable.into(),
        }
    }

    pub fn unknown_scope(scope: impl Into<String>) -> Self {
        Self::UnknownScope {
            scope: scope.into(),
        }
    }

    pub fn database(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Database {
            operation: operation.into(),
            message: message.into(),
        }
    }

    pub fn file_system(
        operation: impl Into<String>,
        path: PathBuf,
        reason: impl Into<String>,
    ) -> Self {
        Self::FileSystem {
            operation,
            path,
            reason: reason.into(),
        }
    }

    pub fn threshold(
        threshold: impl Into<String>,
        message: impl Into<String>,
        actual: Option<f64>,
        expected: Option<String>,
    ) -> Self {
        Self::Threshold {
            threshold: threshold.into(),
            message: message.into(),
            actual,
            expected,
        }
    }

    pub fn default(
        field: impl Into<String>,
        message: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self::Default {
            field: field.into(),
            message: message.into(),
            value: value.into(),
        }
    }

    pub fn dag_validation(message: impl Into<String>) -> Self {
        Self::DagValidation {
            message: message.into(),
        }
    }

    pub fn ir_compilation(message: impl Into<String>) -> Self {
        Self::IrCompilation {
            message: message.into(),
        }
    }

    pub fn invalid_version(version: impl Into<String>) -> Self {
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
    assert!(matches!(error, Error::Io(_)));
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
# Test each constructor returns the expected error type
```

**Checkpoint Criteria:**
- ✅ All Phase 0 error types added (20+ variants)
- ✅ All error constructors implemented
- ✅ bincode dependency added
- ✅ Error tests created and passing
- ✅ Error display formatting is clear and helpful
- ✅ Error sources are properly wrapped (thiserror)

**Next:** Proceed to Task 2 (Schema Types)
