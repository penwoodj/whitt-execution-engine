# Task 3: YAML Parser

**Goal:** Implement YAML→WorkflowSpec parsing with yaml_serde, error reporting, and line number tracking.

**Estimated Time:** 6 hours

**Dependencies:** Task 2

**Files:**
- Modify: `src/parser/mod.rs` (implement YAML parsing)
- Create: `tests/fixtures/workflows/minimal.yml` (test fixture)
- Create: `tests/fixtures/workflows/complex.yml` (test fixture)
- Create: `tests/parser_test.rs` (parser tests)

---

## Step 1: Implement YAML parsing

Implement `src/parser/mod.rs`:

```rust
use crate::error::{Error, Result};
use crate::schema::WorkflowSpec;
use std::path::Path;

/// Parse a YAML workflow file into a WorkflowSpec
pub fn parse_workflow<P: AsRef<Path>>(path: P) -> Result<WorkflowSpec> {
    let yaml_content = std::fs::read_to_string(path.as_ref())
        .map_err(|e| Error::file_system("read", path.as_ref().to_path_buf(), e.to_string()))?;

    parse_workflow_str(&yaml_content)
        .map_err(|e| add_file_context(e, path.as_ref()))
}

/// Parse YAML string into a WorkflowSpec
pub fn parse_workflow_str(yaml: &str) -> Result<WorkflowSpec> {
    yaml_serde::from_str(yaml)
        .map_err(|e| Error::parse(0, 0, format!("YAML parsing error: {}", e)))
}

/// Add file context to error
fn add_file_context(mut error: Error, path: &Path) -> Error {
    // Enhance error message with file path
    match &mut error {
        Error::Parse { message, .. } => {
            *message = format!("{} in file {}", message, path.display());
        }
        Error::YamlSyntax { message } => {
            *message = format!("{} in file {}", message, path.display());
        }
        _ => {}
    }
    error
}

/// Validate WorkflowSpec after parsing
pub fn validate_workflow(spec: &WorkflowSpec) -> Result<()> {
    use garde::Validate;

    spec.validate()
        .map_err(|e| Error::schema_validation("workflow", format!("Validation failed: {}", e), None))
}
```

**Commit:** `feat: implement YAML parser with yaml_serde`

---

## Step 2: Create minimal workflow fixture

Create `tests/fixtures/workflows/minimal.yml`:

```yaml
workflow_id: minimal_workflow
name: "Minimal Workflow"
description: "A minimal valid workflow for testing"
version: "1.0.0"

models:
  primary:
    host:
      type: lmstudio
    ram_allocation:
      strategy: dynamic

workspace:
  root_path: /workspace/test

execution:
  processing: serial
  load_unload: one_at_a_time
```

**Commit:** `test: add minimal workflow fixture`

---

## Step 3: Create complex workflow fixture

Create `tests/fixtures/workflows/complex.yml`:

```yaml
workflow_id: complex_workflow
name: "Complex Workflow"
description: "A complex workflow with all features"
version: "2.0.0"
author: "Test Author"
tags: ["complex", "test", "example"]

models:
  analyzer:
    host:
      type: lmstudio
    ram_allocation:
      strategy: dynamic
    max_allowed:
      ram: 13%
      vram: 3.7GB
      cpu: 49%
      gpu: 74%
      attention_tokens: 150000
    min_allowed:
      ram: 9%
      vram: 2.4GB
    execution:
      load_into_memory: 45s
      time_to_first_response: 1m
      total_time_to_response: 4h

  validator:
    host:
      type: ollama

workspace:
  root_path: /workspace/complex
  directories:
    output: /workspace/complex/output
    checkpoints: /workspace/complex/checkpoints
    logs: /workspace/complex/logs
    metrics: /workspace/complex/metrics
    temp: /workspace/complex/temp

execution:
  processing: parallel
  load_unload: adaptive
  memory:
    ram_allocation:
      strategy: dynamic
    max_allowed:
      ram: 13%
      vram: 3.7GB
    min_allowed:
      ram: 9%
      vram: 2.4GB
    model_lifecycle:
      load_unload_strategy: one_at_a_time
      cache_size: min
      swap_timeout_secs: 30
      unload_unused: true
      gc_interval_secs: 300
  parallel:
    algorithm: round_robin
    max_threads: 4
    max_models: 3
    max_concurrent_requests: 2
    max_steps: 2

agentic_workflow:
  steps:
    step_1_analyze:
      generative_entity: "${models.analyzer}"
      prompt: "Analyze codebase"

    step_2_validate:
      generative_entity: "${models.validator}"
      input:
        analysis: "${step.step_1_analyze.output}"
      depends_on:
        - step: step_1_analyze
```

**Commit:** `test: add complex workflow fixture`

---

## Step 4: Write parser tests

Create `tests/parser_test.rs`:

```rust
use whitt_execution_engine::parser::{parse_workflow, parse_workflow_str, validate_workflow};

#[test]
fn test_parse_minimal_workflow() {
    let yaml = r#"
workflow_id: minimal_workflow
name: "Minimal Workflow"
description: "A minimal valid workflow"
version: "1.0.0"

models:
  primary:
    host:
      type: lmstudio
    ram_allocation:
      strategy: dynamic

workspace:
  root_path: /workspace/test

execution:
  processing: serial
  load_unload: one_at_a_time
"#;

    let spec = parse_workflow_str(yaml).unwrap();
    assert_eq!(spec.identification.workflow_id, "minimal_workflow");
    assert_eq!(spec.identification.name, "Minimal Workflow");
    assert_eq!(spec.identification.version, "1.0.0");
}

#[test]
fn test_parse_from_file() {
    let spec = parse_workflow("tests/fixtures/workflows/minimal.yml").unwrap();
    assert_eq!(spec.identification.workflow_id, "minimal_workflow");
}

#[test]
fn test_parse_complex_workflow() {
    let spec = parse_workflow("tests/fixtures/workflows/complex.yml").unwrap();
    assert_eq!(spec.identification.workflow_id, "complex_workflow");
    assert_eq!(spec.identification.version, "2.0.0");
    assert_eq!(spec.identification.author, "Test Author");
    assert!(spec.identification.tags.contains(&"complex".to_string()));
}

#[test]
fn test_invalid_yaml() {
    let invalid_yaml = "invalid: yaml: content:";

    let result = parse_workflow_str(invalid_yaml);
    assert!(result.is_err());
}

#[test]
fn test_missing_required_field() {
    let missing_id = r#"
name: "Test Workflow"
description: "Test"
version: "1.0.0"
"#;

    let result = parse_workflow_str(missing_id);
    // Should fail due to missing workflow_id
    assert!(result.is_err());
}

#[test]
fn test_validate_workflow() {
    let yaml = r#"
workflow_id: test_workflow
name: "Test Workflow"
description: "Test"
version: "1.0.0"

models:
  primary:
    host:
      type: lmstudio
    ram_allocation:
      strategy: dynamic

workspace:
  root_path: /workspace/test

execution:
  processing: serial
  load_unload: one_at_a_time
"#;

    let spec = parse_workflow_str(yaml).unwrap();
    let result = validate_workflow(&spec);
    assert!(result.is_ok());
}

#[test]
fn test_agentic_workflow_parsing() {
    let yaml = r#"
workflow_id: test_workflow
name: "Test Workflow"
description: "Test"
version: "1.0.0"

models:
  primary:
    host:
      type: lmstudio
    ram_allocation:
      strategy: dynamic

workspace:
  root_path: /workspace/test

execution:
  processing: serial
  load_unload: one_at_a_time

agentic_workflow:
  steps:
    step_1:
      generative_entity: "${models.primary}"
      prompt: "Test prompt"
"#;

    let spec = parse_workflow_str(yaml).unwrap();
    assert!(spec.agentic_workflow.is_some());
    let steps = spec.agentic_workflow.unwrap().steps;
    assert!(steps.contains_key("step_1"));
}
```

**Commit:** `test: add parser tests`

---

## Step 5: Run tests

Verify all tests pass:

```bash
cargo test parser_test

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

# 2. All parser tests pass
cargo test parser_test
# Expected: test result: ok. X passed

# 3. Parse example workflows
# Parse a few workflows from ../../requirements/example-workflows/
```

**Checkpoint Criteria:**
- ✅ YAML→WorkflowSpec parsing works with yaml_serde
- ✅ Error messages include file context and line numbers
- ✅ Parse-time validation catches schema errors
- ✅ Parser tests created and passing
- ✅ Minimal and complex workflow fixtures work
- ✅ Agentic workflow parsing works

**Anti-Drift Check:** Verify task 3 implements ONLY parsing. No IR compilation, DAG validation, or persistence yet.

**Next:** Proceed to Task 4 (Variable Interpolation)
