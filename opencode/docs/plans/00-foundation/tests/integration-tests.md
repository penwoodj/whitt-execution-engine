# Integration Tests Specification

This document specifies the integration tests for Phase 0 Foundation components.

## Overview

Integration tests verify that components work together correctly. They test the full YAML→WorkflowSpec→WorkflowIR pipeline, including parsing, compilation, validation, and persistence.

## Test Structure

```
tests/
└── integration/
    ├── mod.rs                # Integration test module
    ├── pipeline_tests.rs      # Full pipeline tests
    ├── workflow_tests.rs     # End-to-end workflow tests
    └── fixture_tests.rs      # Test fixture validation
```

---

## Full Pipeline Tests

### Test Cases

#### YAML → WorkflowSpec → WorkflowIR Pipeline
```rust
#[test]
fn test_minimal_workflow_pipeline() {
    // 1. Parse YAML
    let spec = parse_workflow("tests/fixtures/workflows/minimal.yml").unwrap();

    // 2. Validate spec
    validate_workflow(&spec).unwrap();

    // 3. Compile to IR
    let ir = compile(&spec).unwrap();

    // 4. Validate IR
    assert_eq!(ir.id.as_str(), "minimal_workflow");
    assert_eq!(ir.name, "Minimal Workflow");
    assert!(ir.models.contains_key(&ModelId::new("primary")));
}

#[test]
fn test_complex_workflow_pipeline() {
    let spec = parse_workflow("tests/fixtures/workflows/complex.yml").unwrap();
    validate_workflow(&spec).unwrap();

    let ir = compile(&spec).unwrap();

    assert_eq!(ir.id.as_str(), "complex_workflow");
    assert_eq!(ir.version, "2.0.0");
    assert_eq!(ir.models.len(), 2);
    assert_eq!(ir.steps.len(), 2);

    // Validate models
    assert!(ir.models.contains_key(&ModelId::new("analyzer")));
    assert!(ir.models.contains_key(&ModelId::new("validator")));

    // Validate steps
    assert!(ir.steps.contains_key(&StepId::new("step_1_analyze")));
    assert!(ir.steps.contains_key(&StepId::new("step_2_validate")));
}
```

#### DAG Validation Integration
```rust
#[test]
fn test_pipeline_with_dag_validation() {
    let spec = parse_workflow("tests/fixtures/workflows/complex.yml").unwrap();
    let ir = compile(&spec).unwrap();

    // Validate DAG
    validate_dag(&ir).unwrap();

    // Validate step references
    validate_step_references(&ir).unwrap();
}

#[test]
fn test_pipeline_detects_circular_dependency() {
    let yaml = r#"
workflow_id: circular_test
name: "Circular Test"
description: "Test circular dependency detection"
version: "1.0.0"

models:
  primary:
    host:
      type: lmstudio

workspace:
  root_path: /workspace/test

features:
  categories:
    - ModelConfiguration
  compatibility_checks: true

execution:
  processing: serial
  load_unload: one_at_a_time

tool_permissions:
  file_operations:
    read:
      # presence = enabled
    write:
      disabled: true
    delete:
      disabled: true
  web_operations:
    fetch:
      disabled: true
    scrape:
      disabled: true
  shell_operations:
    exec:
      disabled: true

logging:
  # presence = enabled
  default: info
  output:
    console:
      # presence = enabled
    file:
      disabled: true
  errors:
    log_parsing_errors: true

agentic_workflow:
  steps:
    step_1:
      step: test_step_1
      model: "${models.primary}"
      prompt: "Test"
      depends_on:
        - step_3
    step_2:
      step: test_step_2
      model: "${models.primary}"
      prompt: "Test"
      depends_on:
        - step_1
    step_3:
      step: test_step_3
      model: "${models.primary}"
      prompt: "Test"
      depends_on:
        - step_2
"#;

    let spec = parse_workflow_str(yaml).unwrap();
    let ir = compile(&spec).unwrap();

    let result = validate_dag(&ir);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Circular dependency"));
}
```

#### Policy Compilation Integration
```rust
#[test]
fn test_pipeline_with_policy_compilation() {
    let spec = parse_workflow("tests/fixtures/workflows/complex.yml").unwrap();

    // Compile policies
    let policies = compile_policies(&spec).unwrap();

    // Verify logging policy
    assert_eq!(policies.logging.default_level, "info");
    assert_eq!(policies.logging.workflow_level, "info");
    assert_eq!(policies.logging.step_level, "debug");

    // Verify tool permissions policy
    assert_eq!(policies.tool_permissions.file_read_enabled, true);
    assert_eq!(policies.tool_permissions.file_write_enabled, true);
    assert_eq!(policies.tool_permissions.shell_exec_enabled, true);
}

#[test]
fn test_pipeline_with_defaults() {
    let spec = parse_workflow("tests/fixtures/workflows/minimal.yml").unwrap();

    // Apply defaults
    let defaults = DefaultValues::system_defaults();
    let mut spec = spec;
    defaults.apply_defaults(&mut spec).unwrap();

    // Verify defaults applied
    assert!(!spec.logging.levels.is_empty());
}
```

---

## End-to-End Workflow Tests

### Test Cases

#### Complete Workflow Execution
```rust
#[test]
fn test_complete_workflow_from_yaml_to_ir() {
    // 1. Load YAML file
    let spec = parse_workflow("tests/fixtures/workflows/complex.yml").unwrap();

    // 2. Validate workflow
    validate_workflow(&spec).unwrap();

    // 3. Apply defaults
    let defaults = DefaultValues::system_defaults();
    let mut spec = spec;
    defaults.apply_defaults(&mut spec).unwrap();

    // 4. Compile to IR
    let ir = compile(&spec).unwrap();

    // 5. Validate DAG
    validate_dag(&ir).unwrap();

    // 6. Compile policies
    let policies = compile_policies(&spec).unwrap();

    // 7. Store workflow
    let temp_dir = TempDir::new().unwrap();
    let storage = LocalStorage::open(temp_dir.path()).unwrap();
    storage.store_workflow(ir.id.as_str(), &ir).unwrap();

    // 8. Retrieve workflow
    let loaded = storage.load_workflow(ir.id.as_str()).unwrap();
    assert!(loaded.is_some());

    // 9. Initialize workspace
    let workspace = WorkspaceManager::new(spec.workspace.clone());
    workspace.initialize().unwrap();

    // 10. Verify complete pipeline
    assert_eq!(loaded.unwrap().id.as_str(), ir.id.as_str());
}

#[test]
fn test_workflow_with_all_features() {
    let yaml = r#"
workflow_id: full_feature_test
name: "Full Feature Test"
description: "Test all features together"
version: "1.0.0"
author: "Test Author"
tags: ["full", "test"]

models:
  analyzer:
    host:
      type: lmstudio
    max_allowed:
      ram: 13%
      vram: 3.7GB
      cpu: 49%
      gpu: 74%
      attention_tokens: 150000
    min_allowed:
      ram: 9%
      vram: 2.4GB
    allocation_strategy: dynamic
    execution:
      load_into_memory: 45s
      time_to_first_response: 1m
      total_time_to_response: 4h

  validator:
    host:
      type: ollama

workspace:
  root_path: /workspace/full_test
  output_path: /workspace/full_test/output
  checkpoint_path: /workspace/full_test/checkpoints
  log_path: /workspace/full_test/logs
  metrics_path: /workspace/full_test/metrics
  temp_path: /workspace/full_test/temp

features:
  categories:
    - ModelConfiguration
    - StepTypes
    - DataFlow
    - ParallelExecution
  compatibility_checks: true

execution:
  processing: parallel
  load_unload: adaptive
  memory:
    ram_allocation: adaptive
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
    # presence = enabled
    algorithm: round_robin
    max_threads: 4
    max_models: 3
    max_concurrent_requests: 2
    max_steps: 2
    max_sub_agents: 2

agentic_workflow:
  steps:
    step_1_analyze:
      step: analyze
      name: "Analyze Code"
      description: "Analyze the codebase"
      type: agent
      model: "${models.analyzer}"
      prompt: "Analyze the codebase"
      output:
        save_to: analysis
        format: json

    step_2_validate:
      step: validate
      name: "Validate Results"
      description: "Validate analysis results"
      type: agent
      model: "${models.validator}"
      depends_on:
        - step_1_analyze
      inputs:
        analysis: "${step.step_1_analyze.output}"
      retry:
        max_attempts: 3
        backoff: exponential
        delay_ms: 1000

tool_permissions:
  file_operations:
    read:
      # presence = enabled
      require_confirmation: false
      allowed_paths:
        - /workspace/full_test/src
      max_file_size_mb: 100
    write:
      # presence = enabled
      require_confirmation: true
      allowed_paths:
        - /workspace/full_test/output
      backup_existing: true
      max_file_size_mb: 500
    delete:
      # presence = enabled
      require_confirmation: true
      allowed_paths:
        - /workspace/full_test/temp
  web_operations:
    fetch:
      # presence = enabled
      require_confirmation: false
      max_concurrent_requests: 5
      timeout_secs: 30
    scrape:
      # presence = enabled
      respect_robots_txt: true
      max_pages_per_domain: 100
  shell_operations:
    exec:
      # presence = enabled
      require_confirmation: true
      timeout_seconds: 30
      allowed_commands:
        - cargo
        - rustc

logging:
  # presence = enabled
  default: info
  levels:
    workflow: info
    models: warning
    tools: info
    execution: debug
  scopes:
    workflow:
      # presence = enabled
      include_timestamps: true
  output:
    console:
      # presence = enabled
      color: true
      timestamps: true
      format: text
    file:
      # presence = enabled
      path: /workspace/full_test/logs/workflow.log
      format: json
      rotation:
        # presence = enabled
        max_size_mb: 100
        max_files: 10
  errors:
    log_parsing_errors: true
    hardware_limitation_errors: true
    runtime_errors: true
    validation_errors: true
    error_log_file: /workspace/full_test/logs/errors.log
"#;

    // Complete pipeline
    let spec = parse_workflow_str(yaml).unwrap();
    validate_workflow(&spec).unwrap();

    let defaults = DefaultValues::system_defaults();
    let mut spec = spec;
    defaults.apply_defaults(&mut spec).unwrap();

    let ir = compile(&spec).unwrap();
    validate_dag(&ir).unwrap();

    let policies = compile_policies(&spec).unwrap();

    // Verify all features
    assert_eq!(ir.models.len(), 2);
    assert_eq!(ir.steps.len(), 2);
    assert_eq!(ir.execution_mode, ExecutionMode::Parallel { .. });
    assert!(policies.tool_permissions.file_read_enabled);
    assert!(policies.tool_permissions.shell_exec_enabled);
}
```

---

## Test Fixture Validation

### Test Cases

#### Validate All Fixtures
```rust
#[test]
fn test_all_fixtures_parse_successfully() {
    let fixtures = vec![
        "tests/fixtures/workflows/minimal.yml",
        "tests/fixtures/workflows/complex.yml",
    ];

    for fixture in fixtures {
        let spec = parse_workflow(fixture).unwrap();
        validate_workflow(&spec).unwrap();

        let ir = compile(&spec).unwrap();
        validate_dag(&ir).unwrap();
    }
}

#[test]
fn test_fixtures_compile_to_ir() {
    let fixtures = vec![
        ("minimal", "tests/fixtures/workflows/minimal.yml"),
        ("complex", "tests/fixtures/workflows/complex.yml"),
    ];

    for (name, path) in fixtures {
        let spec = parse_workflow(path).unwrap();
        let ir = compile(&spec).unwrap();

        // Verify IR structure
        assert!(!ir.id.as_str().is_empty());
        assert!(!ir.name.is_empty());
        assert!(!ir.version.is_empty());
    }
}
```

---

## Error Scenario Tests

### Test Cases

#### Invalid Workflow Scenarios
```rust
#[test]
fn test_invalid_workflow_causes_error() {
    let yaml = r#"
workflow_id: ""
name: "Test"
description: "Test"
version: "1.0.0"

models:
  primary:
    host:
      type: lmstudio

workspace:
  root_path: /workspace/test

features:
  categories:
    - ModelConfiguration
  compatibility_checks: true

execution:
  processing: serial
  load_unload: one_at_a_time

tool_permissions:
  file_operations:
    read:
      # presence = enabled
    write:
      disabled: true
    delete:
      disabled: true
  web_operations:
    fetch:
      disabled: true
    scrape:
      disabled: true
  shell_operations:
    exec:
      disabled: true

logging:
  # presence = enabled
  default: info
  output:
    console:
      # presence = enabled
    file:
      disabled: true
  errors:
    log_parsing_errors: true
"#;

    let spec = parse_workflow_str(yaml).unwrap();
    let result = validate_workflow(&spec);

    assert!(result.is_err());
}

#[test]
fn test_invalid_threshold_causes_error() {
    let yaml = r#"
workflow_id: test
name: "Test"
description: "Test"
version: "1.0.0"

models:
  primary:
    host:
      type: lmstudio
    max_allowed:
      ram: 150%
    min_allowed:
      ram: 0%

workspace:
  root_path: /workspace/test

features:
  categories:
    - ModelConfiguration
  compatibility_checks: true

execution:
  processing: serial
  load_unload: one_at_a_time

tool_permissions:
  file_operations:
    read:
      # presence = enabled
    write:
      disabled: true
    delete:
      disabled: true
  web_operations:
    fetch:
      disabled: true
    scrape:
      disabled: true
  shell_operations:
    exec:
      disabled: true

logging:
  # presence = enabled
  default: info
  output:
    console:
      # presence = enabled
    file:
      disabled: true
  errors:
    log_parsing_errors: true
"#;

    let spec = parse_workflow_str(yaml).unwrap();
    let ir = compile(&spec).unwrap();

    for model in ir.models.values() {
        let result = validate_resource_limits(&model.max_resources, true);
        assert!(result.is_err());
    }
}
```

---

## Running Tests

```bash
# Run all integration tests
cargo test --test integration

# Run specific test file
cargo test integration::pipeline_tests

# Run with output
cargo test --test integration -- --nocapture

# Run with backtrace
cargo test --test integration -- --backtrace
```

---

## Coverage Goals

- **Full Pipeline Tests**: 90%+ coverage
- **End-to-End Workflow Tests**: 85%+ coverage
- **Test Fixture Validation**: 100% coverage
- **Error Scenario Tests**: 95%+ coverage

---

## Performance Requirements

- All integration tests should complete in < 30 seconds
- Full pipeline test should complete in < 5 seconds
- End-to-end workflow test should complete in < 10 seconds

---

**Document Version:** 1.0.0
**Last Updated:** 2026-04-06
**Status:** Complete
