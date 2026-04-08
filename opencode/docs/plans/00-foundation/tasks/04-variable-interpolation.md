# Task 4: Variable Interpolation

**Goal:** Implement `${...}` (parse-time) and `{{...}}` (runtime) interpolation with proper scoping.

**Estimated Time:** 5 hours

**Dependencies:** Task 2

**Files:**
- Modify: `src/interpolation.rs` (implement interpolation)
- Create: `tests/interpolation_test.rs` (interpolation tests)

---

## Step 1: Define interpolation types

Update `src/interpolation.rs`:

```rust
use crate::error::{Error, Result};
use crate::schema::WorkflowSpec;
use std::collections::HashMap;

/// Interpolation scope for resolving variables
#[derive(Debug, Clone)]
pub enum InterpolationScope {
    Workflow,
    Step,
    Model,
    Run,
    Now,
}

impl InterpolationScope {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "workflow" => Ok(Self::Workflow),
            "step" => Ok(Self::Step),
            "model" => Ok(Self::Model),
            "run" => Ok(Self::Run),
            "now" => Ok(Self::Now),
            _ => Err(Error::unknown_scope(s)),
        }
    }
}

/// Parse-time interpolation context (for ${...} syntax)
#[derive(Debug, Clone)]
pub struct ParseContext {
    pub workflow: HashMap<String, serde_json::Value>,
    pub models: HashMap<String, serde_json::Value>,
}

impl ParseContext {
    pub fn new() -> Self {
        Self {
            workflow: HashMap::new(),
            models: HashMap::new(),
        }
    }

    pub fn from_spec(spec: &WorkflowSpec) -> Self {
        let mut ctx = Self::new();

        // Add workflow identification
        ctx.workflow.insert(
            "id".to_string(),
            serde_json::Value::String(spec.identification.workflow_id.clone()),
        );
        ctx.workflow.insert(
            "name".to_string(),
            serde_json::Value::String(spec.identification.name.clone()),
        );
        ctx.workflow.insert(
            "version".to_string(),
            serde_json::Value::String(spec.identification.version.clone()),
        );

        // Add workspace path
        ctx.workflow.insert(
            "workspace".to_string(),
            serde_json::json!({
                "root_path": spec.workspace.root_path,
                "output_path": spec.workspace.output_path,
            }),
        );

        // Add models
        for (name, config) in spec.models.models.iter() {
            ctx.models.insert(
                name.clone(),
                serde_json::json!({
                    "name": config.name,
                    "host": config.host.provider_type,
                }),
            );
        }

        ctx
    }
}

/// Runtime interpolation context (for {{...}} syntax)
#[derive(Debug, Clone)]
pub struct RuntimeContext {
    pub workflow: HashMap<String, serde_json::Value>,
    pub steps: HashMap<String, serde_json::Value>,
    pub models: HashMap<String, serde_json::Value>,
    pub run: HashMap<String, serde_json::Value>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self {
            workflow: HashMap::new(),
            steps: HashMap::new(),
            models: HashMap::new(),
            run: HashMap::new(),
        }
    }

    pub fn with_step(mut self, step_id: &str, step_output: serde_json::Value) -> Self {
        self.steps.insert(step_id.to_string(), step_output);
        self
    }
}

/// Parse-time interpolation (${...})
pub fn interpolate_parse(input: &str, ctx: &ParseContext) -> Result<String> {
    let mut result = input.to_string();

    // Find all ${...} patterns
    let start_pattern = "${";
    let end_pattern = "}";

    let mut pos = 0;
    while pos < result.len() {
        if let Some(start) = result[pos..].find(start_pattern) {
            let start_pos = pos + start;
            if let Some(end) = result[start_pos + start_pattern.len()..].find(end_pattern) {
                let end_pos = start_pos + start_pattern.len() + end;
                let var_expr = &result[start_pos + start_pattern.len()..end_pos];

                let replacement = resolve_parse_var(var_expr, ctx)?;
                result.replace_range(start_pos..=end_pos, &replacement);

                pos = start_pos + replacement.len();
            } else {
                pos = result.len();
            }
        } else {
            break;
        }
    }

    Ok(result)
}

/// Runtime interpolation ({{...}})
pub fn interpolate_runtime(input: &str, ctx: &RuntimeContext) -> Result<String> {
    let mut result = input.to_string();

    // Find all {{...}} patterns
    let start_pattern = "{{";
    let end_pattern = "}}";

    let mut pos = 0;
    while pos < result.len() {
        if let Some(start) = result[pos..].find(start_pattern) {
            let start_pos = pos + start;
            if let Some(end) = result[start_pos + start_pattern.len()..].find(end_pattern) {
                let end_pos = start_pos + start_pattern.len() + end;
                let var_expr = &result[start_pos + start_pattern.len()..end_pos];

                let replacement = resolve_runtime_var(var_expr, ctx)?;
                result.replace_range(start_pos..=end_pos, &replacement);

                pos = start_pos + replacement.len();
            } else {
                pos = result.len();
            }
        } else {
            break;
        }
    }

    Ok(result)
}

/// Resolve parse-time variable reference
fn resolve_parse_var(var_expr: &str, ctx: &ParseContext) -> Result<String> {
    let parts: Vec<&str> = var_expr.split('.').collect();

    match parts.as_slice() {
        ["workflow", key] => {
            ctx.workflow
                .get(*key)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| Error::interpolation(var_expr))
        }
        ["models", model_name, key] => {
            ctx.models
                .get(*model_name)
                .and_then(|m| m.get(*key))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| Error::interpolation(var_expr))
        }
        _ => Err(Error::interpolation(var_expr)),
    }
}

/// Resolve runtime variable reference
fn resolve_runtime_var(var_expr: &str, ctx: &RuntimeContext) -> Result<String> {
    let parts: Vec<&str> = var_expr.split('.').collect();

    match parts.as_slice() {
        ["workflow", key] => {
            ctx.workflow
                .get(*key)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| Error::interpolation(var_expr))
        }
        ["step", step_id, key] => {
            ctx.steps
                .get(*step_id)
                .and_then(|s| s.get(*key))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| Error::interpolation(var_expr))
        }
        ["model", model_name, key] => {
            ctx.models
                .get(*model_name)
                .and_then(|m| m.get(*key))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| Error::interpolation(var_expr))
        }
        ["run", key] => {
            ctx.run
                .get(*key)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| Error::interpolation(var_expr))
        }
        ["now"] => {
            // Return current timestamp
            let now = chrono::Utc::now().to_rfc3339();
            Ok(now)
        }
        _ => Err(Error::interpolation(var_expr)),
    }
}

/// Check if string contains interpolation markers
pub fn has_interpolation(input: &str) -> bool {
    input.contains("${") || input.contains("{{")
}

/// Extract all variable references from a string
pub fn extract_variables(input: &str) -> Vec<String> {
    let mut variables = Vec::new();

    // Extract ${...} variables
    let mut pos = 0;
    while pos < input.len() {
        if let Some(start) = input[pos..].find("${") {
            let start_pos = pos + start;
            if let Some(end) = input[start_pos + 2..].find("}") {
                let end_pos = start_pos + 2 + end;
                let var_expr = &input[start_pos + 2..end_pos];
                variables.push(var_expr.to_string());
                pos = end_pos + 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // Extract {{...}} variables
    pos = 0;
    while pos < input.len() {
        if let Some(start) = input[pos..].find("{{") {
            let start_pos = pos + start;
            if let Some(end) = input[start_pos + 2..].find("}}") {
                let end_pos = start_pos + 2 + end;
                let var_expr = &input[start_pos + 2..end_pos];
                variables.push(var_expr.to_string());
                pos = end_pos + 2;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    variables
}
```

**Commit:** `feat: implement parse-time and runtime variable interpolation`

---

## Step 2: Write interpolation tests

Create `tests/interpolation_test.rs`:

```rust
use yaml_to_rust_agentsdk::interpolation::*;

#[test]
fn test_parse_time_interpolation() {
    let mut ctx = ParseContext::new();
    ctx.workflow.insert(
        "id".to_string(),
        serde_json::Value::String("test_workflow".to_string()),
    );
    ctx.models.insert(
        "primary".to_string(),
        serde_json::json!({"name": "llama-3.2-3b-instruct"}),
    );

    let input = "Workflow ${workflow.id} uses model ${models.primary.name}";
    let result = interpolate_parse(input, &ctx).unwrap();

    assert_eq!(result, "Workflow test_workflow uses model llama-3.2-3b-instruct");
}

#[test]
fn test_runtime_interpolation() {
    let mut ctx = RuntimeContext::new();
    ctx.steps.insert(
        "step_1".to_string(),
        serde_json::json!({"output": "result_data"}),
    );
    ctx.run.insert(
        "number".to_string(),
        serde_json::Value::Number(1.into()),
    );

    let input = "Step output: {{step.step_1.output}}, Run: {{run.number}}";
    let result = interpolate_runtime(input, &ctx).unwrap();

    assert_eq!(result, "Step output: result_data, Run: 1");
}

#[test]
fn test_now_interpolation() {
    let ctx = RuntimeContext::new();

    let input = "Current time: {{now}}";
    let result = interpolate_runtime(input, &ctx).unwrap();

    assert!(result.contains("20")); // Contains year (202X)
    assert!(result.contains(":")); // Contains time separator
}

#[test]
fn test_undefined_variable() {
    let ctx = ParseContext::new();

    let input = "Reference ${workflow.nonexistent}";
    let result = interpolate_parse(input, &ctx);

    assert!(result.is_err());
}

#[test]
fn test_has_interpolation() {
    assert!(has_interpolation("${workflow.id}"));
    assert!(has_interpolation("{{step.output}}"));
    assert!(!has_interpolation("plain text"));
}

#[test]
fn test_extract_variables() {
    let input = "${workflow.id} {{step.output}} ${models.primary.name}";
    let vars = extract_variables(input);

    assert_eq!(vars.len(), 3);
    assert!(vars.contains(&"workflow.id".to_string()));
    assert!(vars.contains(&"step.output".to_string()));
    assert!(vars.contains(&"models.primary.name".to_string()));
}

#[test]
fn test_nested_interpolation() {
    let mut ctx = ParseContext::new();
    ctx.workflow.insert(
        "path".to_string(),
        serde_json::Value::String("/workspace".to_string()),
    );

    let input = "Full path: ${workflow.path}/output";
    let result = interpolate_parse(input, &ctx).unwrap();

    assert_eq!(result, "Full path: /workspace/output");
}

#[test]
fn test_runtime_context_builder() {
    let ctx = RuntimeContext::new()
        .with_step("step_1", serde_json::json!({"output": "test"}));

    assert_eq!(ctx.steps.get("step_1").unwrap().get("output").unwrap(), "test");
}
```

**Commit:** `test: add interpolation tests`

---

## Step 3: Run tests

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
# Test ${...} and {{...}} separately
```

**Checkpoint Criteria:**
- ✅ Parse-time interpolation (${...}) implemented
- ✅ Runtime interpolation ({{...}}) implemented
- ✅ Proper scoping (workflow, step, model, run, now)
- ✅ Interpolation tests created and passing
- ✅ Error handling for undefined variables
- ✅ Variable extraction works

**Anti-Drift Check:** Verify task 4 implements ONLY interpolation. No IR compilation, DAG validation, or persistence yet.

**Next:** Proceed to Task 5 (Workflow IR)
