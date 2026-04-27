# Task 4: Variable Interpolation

**Goal:** Implement structural (${...}) and runtime ({{...}) variable interpolation with scoping and validation.

**Estimated Time:** 5 hours

**Dependencies:** Task 2

**Files:**
- Create: `src/interpolation.rs` (interpolation implementation)
- Modify: `Cargo.toml` (add regex, minijinja dependencies)
- Create: `tests/interpolation_test.rs` (interpolation tests)

---

## Step 1: Define interpolation types

Create `src/interpolation.rs`:

```rust
//! Variable interpolation for workflow templates
//!
//! Supports two interpolation types:
//! - Structural references (${...}) — resolved at workflow parse time
//! - Runtime template values ({{...}}) — resolved at execution time

use crate::error::{Error, Result};
use regex::Regex;
use std::collections::HashMap;

/// Interpolation scope type
#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationScope {
    /// Structural reference: ${models.model_name}
    Structural,

    /// Runtime template value: {{step.step_name.output}}
    Runtime,
}

/// Interpolation context containing all available variables
#[derive(Debug, Clone)]
pub struct InterpolationContext {
    /// Structural variables (models, workspace, workflow)
    pub structural: HashMap<String, serde_yaml::Value>,
    /// Runtime variables (step outputs, inputs, loop iterations)
    pub runtime: HashMap<String, serde_yaml::Value>,
}

impl InterpolationContext {
    pub fn new() -> Self {
        Self {
            structural: HashMap::new(),
            runtime: HashMap::new(),
        }
    }

    pub fn with_structural(vars: HashMap<String, serde_yaml::Value>) -> Self {
        Self {
            structural: vars,
            runtime: HashMap::new(),
        }
    }

    pub fn with_runtime(vars: HashMap<String, serde_yaml::Value>) -> Self {
        Self {
            structural: HashMap::new(),
            runtime: vars,
        }
    }
}

/// Interpolation result
#[derive(Debug, Clone)]
pub struct InterpolationResult {
    pub interpolated: String,
    pub unresolved_references: Vec<String>,
    pub errors: Vec<Error>,
}
```

**Commit:** `feat: define interpolation types`

---

## Step 2: Implement structural interpolation

Add to `src/interpolation.rs`:

```rust
impl InterpolationContext {
    /// Resolve structural references (${models.model_name}, ${workspace.path_var}, etc.)
    pub fn resolve_structural(&self, template: &str) -> Result<InterpolationResult> {
        let mut result = String::from(template);
        let mut unresolved = Vec::new();
        let mut errors = Vec::new();

        // Match ${...} patterns
        let structural_regex = Regex::new(r"\$\{([^}]+)\}").unwrap();

        for cap in structural_regex.captures_iter(template) {
            if let Some(reference) = cap.and_then(|c| c.get(1)) {
                let ref_str = reference.as_str();

                // Split reference into parts: models.primary, workspace.root_path, etc.
                let parts: Vec<&str> = ref_str.split('.').collect();

                match parts.as_slice() {
                    ["models", model_name] => {
                        if let Some(value) = self.structural.get(*model_name) {
                            result = result.replace(&format!("${{{}}}", ref_str), &value_to_string(value));
                        } else {
                            unresolved.push(ref_str.to_string());
                            errors.push(Error::undefined_reference(format!("${{{}}}", ref_str)));
                        }
                    }

                    ["workspace", var_name] => {
                        if let Some(value) = self.structural.get(*var_name) {
                            result = result.replace(&format!("${{{}}}", ref_str), &value_to_string(value));
                        } else {
                            unresolved.push(ref_str.to_string());
                            errors.push(Error::unknown_scope(format!("${{{}}}", ref_str)));
                        }
                    }

                    ["workflow", field_name] => {
                        if let Some(value) = self.structural.get(*field_name) {
                            result = result.replace(&format!("${{{}}}", ref_str), &value_to_string(value));
                        } else {
                            unresolved.push(ref_str.to_string());
                            errors.push(Error::undefined_reference(format!("${{{}}}", ref_str)));
                        }
                    }

                    _ => {
                        unresolved.push(ref_str.to_string());
                        errors.push(Error::unknown_scope(format!("${{{}}}", ref_str)));
                    }
                }
            }
        }

        Ok(InterpolationResult {
            interpolated: result,
            unresolved_references: unresolved,
            errors,
        })
    }
}

fn value_to_string(value: &serde_yaml::Value) -> String {
    match value {
        serde_yaml::Value::String(s) => s.clone(),
        serde_yaml::Value::Number(n) => n.to_string(),
        serde_yaml::Value::Bool(b) => b.to_string(),
        serde_yaml::Value::Null => "null".to_string(),
        serde_yaml::Value::Mapping(m) => serde_yaml::to_string(m).unwrap_or_else(|_| "{}".to_string()),
        serde_yaml::Value::Sequence(seq) => serde_yaml::to_string(seq).unwrap_or_else(|_| "[]".to_string()),
    }
}
```

**Commit:** `feat: implement structural interpolation`

---

## Step 3: Implement runtime interpolation

Add to `src/interpolation.rs`:

```rust
impl InterpolationContext {
    /// Resolve runtime template values ({{step.step_name.output}}, {{inputs.field_name}}, etc.)
    pub fn resolve_runtime(&self, template: &str) -> Result<InterpolationResult> {
        let mut result = String::from(template);
        let mut unresolved = Vec::new();
        let mut errors = Vec::new();

        // Match {{...}} patterns
        let runtime_regex = Regex::new(r"\{\{([^}]+)\}\}").unwrap();

        for cap in runtime_regex.captures_iter(template) {
            if let Some(reference) = cap.and_then(|c| c.get(1)) {
                let ref_str = reference.as_str();

                // Split reference into parts: step.step_name.output, inputs.field_name, etc.
                let parts: Vec<&str> = ref_str.split('.').collect();

                match parts.as_slice() {
                    ["step", step_name, "output"] => {
                        let var_name = format!("step.{{}}.output", *step_name);
                        if let Some(value) = self.runtime.get(&var_name) {
                            result = result.replace(&format!("{{{{}}}}", ref_str), &value_to_string(value));
                        } else {
                            unresolved.push(ref_str.to_string());
                            errors.push(Error::undefined_reference(format!("{{{{}}}}", ref_str)));
                        }
                    }

                    ["step", step_name, "raw_text"] => {
                        let var_name = format!("step.{{}}.raw_text", *step_name);
                        if let Some(value) = self.runtime.get(&var_name) {
                            result = result.replace(&format!("{{{{}}}}", ref_str), &value_to_string(value));
                        } else {
                            unresolved.push(ref_str.to_string());
                            errors.push(Error::undefined_reference(format!("{{{{}}}}", ref_str)));
                        }
                    }

                    ["sub_workflow", name, "output"] => {
                        let var_name = format!("sub_workflow.{{}}.output", *name);
                        if let Some(value) = self.runtime.get(&var_name) {
                            result = result.replace(&format!("{{{{}}}}", ref_str), &value_to_string(value));
                        } else {
                            unresolved.push(ref_str.to_string());
                            errors.push(Error::undefined_reference(format!("{{{{}}}}", ref_str)));
                        }
                    }

                    ["inputs", field_name] => {
                        if let Some(value) = self.runtime.get(*field_name) {
                            result = result.replace(&format!("{{{{}}}}", ref_str), &value_to_string(value));
                        } else {
                            unresolved.push(ref_str.to_string());
                            errors.push(Error::undefined_reference(format!("{{{{}}}}", ref_str)));
                        }
                    }

                    ["loop", iteration_variable] => {
                        if let Some(value) = self.runtime.get(*iteration_variable) {
                            result = result.replace(&format!("{{{{}}}}", ref_str), &value_to_string(value));
                        } else {
                            unresolved.push(ref_str.to_string());
                            errors.push(Error::interpolation(format!("{{{{}}}}", ref_str)));
                        }
                    }

                    ["now"] | ["workflow_id"] | ["run", "number"] => {
                        // These are built-in runtime variables
                        let value = match parts.as_slice() {
                            ["now"] => chrono::Utc::now().to_rfc3339(),
                            ["workflow_id"] => self.structural.get("workflow_id")
                                .and_then(|v| v.as_str())
                                .unwrap_or(""),
                            ["run", "number"] => self.runtime.get("run.number")
                                .and_then(|v| v.as_str())
                                .unwrap_or("1"),
                            _ => unreachable!(),
                        };
                        result = result.replace(&format!("{{{{}}}}", ref_str), &value_to_string(&serde_yaml::Value::String(value)));
                    }

                    _ => {
                        unresolved.push(ref_str.to_string());
                        errors.push(Error::unknown_scope(format!("{{{{}}}}", ref_str)));
                    }
                }
            }
        }

        Ok(InterpolationResult {
            interpolated: result,
            unresolved_references: unresolved,
            errors,
        })
    }
}
```

**Commit:** `feat: implement runtime interpolation`

---

## Step 4: Add dependencies

Update `Cargo.toml`:

```toml
[dependencies]
# Already present from Task 0
regex = "1.10"

# Template engine
minijinja = { version = "2.3", features = ["loader", "urlencode"] }
```

**Commit:** `chore: add regex and minijinja dependencies`

---

## Step 5: Write interpolation tests

Create `tests/interpolation_test.rs`:

```rust
use whitt_execution_engine::interpolation::{InterpolationContext, InterpolationScope};

#[test]
fn test_structural_interpolation() {
    let mut structural = std::collections::HashMap::new();
    structural.insert("models.primary".to_string(), serde_yaml::Value::String("llama-3.2".to_string()));

    let ctx = InterpolationContext::with_structural(structural);
    let template = "Use model: ${models.primary}";
    let result = ctx.resolve_structural(template).unwrap();

    assert_eq!(result.interpolated, "Use model: llama-3.2");
    assert!(result.unresolved_references.is_empty());
}

#[test]
fn test_runtime_interpolation() {
    let mut runtime = std::collections::HashMap::new();
    runtime.insert("step.analyze.output".to_string(), serde_yaml::Value::String("analysis complete".to_string()));

    let ctx = InterpolationContext::with_runtime(runtime);
    let template = "Result: {{step.analyze.output}}";
    let result = ctx.resolve_runtime(template).unwrap();

    assert_eq!(result.interpolated, "Result: analysis complete");
    assert!(result.unresolved_references.is_empty());
}

#[test]
fn test_mixed_interpolation() {
    let mut structural = std::collections::HashMap::new();
    structural.insert("models.primary".to_string(), serde_yaml::Value::String("llama-3.2".to_string()));

    let mut runtime = std::collections::HashMap::new();
    runtime.insert("step.analyze.output".to_string(), serde_yaml::Value::String("complete".to_string()));

    let ctx = InterpolationContext {
        structural,
        runtime,
    };
    let template = "Model ${models.primary} gave result: {{step.analyze.output}}";
    let result = ctx.resolve_structural(template).unwrap();

    assert_eq!(result.interpolated, "Model llama-3.2 gave result: {{step.analyze.output}}");
}

#[test]
fn test_unresolved_reference() {
    let structural = std::collections::HashMap::new();
    let ctx = InterpolationContext::with_structural(structural);

    let template = "Model ${models.missing}";
    let result = ctx.resolve_structural(template).unwrap();

    assert_eq!(result.interpolated, "Model ${models.missing}");
    assert!(!result.unresolved_references.is_empty());
    assert_eq!(result.unresolved_references[0], "models.missing");
}
```

**Commit:** `test: add interpolation tests`

---

## Step 6: Run tests

Verify all tests pass:

```bash
cargo test interpolation_test

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

# 2. All interpolation tests pass
cargo test interpolation_test
# Expected: test result: ok. X passed

# 3. Both interpolation types work
# Test structural and runtime interpolation separately

# 4. Scoping rules enforced
# Test that structural and runtime scopes are properly separated
```

**Checkpoint Criteria:**
- ✅ Structural interpolation (${...}) implemented with scoping
- ✅ Runtime interpolation ({{...}}) implemented with scoping
- ✅ Error handling for undefined references and unknown scopes
- ✅ Regex-based pattern matching for both interpolation types
- ✅ Interpolation tests created and passing
- ✅ Dependencies added (regex, minijinja)

**Next:** Proceed to Task 5 (Defaults Module)

---

## Implementation Status

**Status**: ⚠️ PARTIAL

### What Exists
- **[src/model/interpolation.rs](../../src/model/interpolation.rs)**: Template interpolation implemented:
  - `TemplateInterpolator` struct ✅
  - `resolve_structural()` method for ${...} references ✅
  - `resolve_runtime()` method for {{...}} references ✅
  - String-based template parsing (not regex-based) ✅
  - Error handling for undefined variables ✅

- **Template engine**: Uses `minijinja` for template interpolation ✅

- **Scoping**: Supports models, inputs, steps, loops scoping ✅

- **Dependencies**: `minijinja` and `regex` in [Cargo.toml](../../Cargo.toml) ✅

### What's Missing
- **Interpolation module location**:
  - Plan expects `src/interpolation.rs` - Current implementation is in `src/model/interpolation.rs`
  - Plan expects `InterpolationContext` struct - Current implementation uses `TemplateInterpolator`

- **Type definitions** not implemented (per plan):
  - `InterpolationScope` enum (Structural, Runtime) - NOT IMPLEMENTED
  - `InterpolationContext` struct (structural HashMap, runtime HashMap) - NOT IMPLEMENTED
  - `InterpolationResult` struct (interpolated, unresolved_references, errors) - NOT IMPLEMENTED

- **Method signatures** differ:
  - Plan expects `resolve_structural(&self, template: &str)` - Current uses `resolve_structural(&self, template: &str, context: &Context)`
  - Plan expects `resolve_runtime(&self, template: &str)` - Current uses `resolve_runtime(&self, template: &str, context: &Context)`
  - Plan expects regex-based pattern matching - Current uses minijinja's template parsing

- **Test fixtures** not created:
  - `tests/interpolation_test.rs` - NOT CREATED (plan expects this file)
  - Current implementation has inline tests in `src/model/interpolation.rs`

### QA Coverage
- **Status**: No dedicated QA tests for this task
- **Coverage**: From EPOC Extended POC findings:
  - **AREA-09 TEMPLATE INTERPOLATION** (Area 9) - ✅ PASS - Template interpolation with minijinja works correctly
  - **Note**: QA confirms interpolation works, but uses minijinja not regex as plan expects

### Schema Alignment
- **Schema Ref**: Lines 726-739 (variable interpolation syntax section)
- **Coverage**: Partial — Interpolation works but implementation differs from plan
- **Gaps**:
  - Plan expects separate structural (${...}) and runtime ({{...}) interpolation with regex
  - Current implementation uses minijinja which handles both syntaxes differently
  - Plan's expected `InterpolationContext` with separate structural/runtime HashMaps not implemented

### Evidence
- **Implementation location**: [src/model/interpolation.rs](../../src/model/interpolation.rs) (500+ lines)
- **Template engine**: `minijinja` based interpolation ✅
- **Build**: ✅ `cargo build` passes
- **Tests**: ✅ Interpolation tests exist and pass (part of 91 tests)
- **Dependencies**: `minijinja = "2.3"` with `loader` and `urlencode` features ✅
- **Dependencies**: `regex = "1.10"` ✅

### Plan vs Reality Notes
- **Plan expects**: Regex-based interpolation with `resolve_structural()` and `resolve_runtime()` methods
- **Current reality**: minijinja-based interpolation with different API
- **Library choice**: Plan mentions `minijinja` but expects regex implementation. Current implementation uses minijinja as primary parser
- **Method signatures**: Plan's expected `InterpolationContext::with_structural()` and `with_runtime()` not implemented
- **Context passing**: Current implementation uses `Context` struct for passing variables, not plan's HashMap approach
- **Test location**: Plan expects `tests/interpolation_test.rs`, current implementation has inline tests in `src/model/interpolation.rs`

---

## QA Cross-References

- **QA Criteria**: [QA-00-05](../../qa/phase-00/QA-CRITERIA.md)
- **Test Cases**: [P00-008](../../qa/phase-00/QA-TEST-CASES.md), [P00-009](../../qa/phase-00/QA-TEST-CASES.md), [P00-010](../../qa/phase-00/QA-TEST-CASES.md)
- **Schema Ref**: Lines 726-740 (variable interpolation syntax section)
