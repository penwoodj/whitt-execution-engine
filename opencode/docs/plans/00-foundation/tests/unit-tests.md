# Unit Tests Specification

This document specifies the unit tests for Phase 0 Foundation components.

## Overview

Unit tests verify individual components in isolation with mocked dependencies. Each module has its own test file focusing on the specific functionality of that module.

## Test Structure

```
tests/
├── error_test.rs          # Error module tests
├── schema_test.rs          # Schema type tests
├── parser_test.rs          # Parser tests
├── interpolation_test.rs   # Interpolation tests
├── ir_test.rs             # IR type tests
├── compiler_test.rs        # Compiler tests
├── dag_test.rs            # DAG validation tests
├── policy_test.rs         # Policy compilation tests
├── storage_test.rs        # Storage tests
├── workspace_test.rs      # Workspace management tests
├── defaults_test.rs       # Defaults and inheritance tests
└── threshold_test.rs      # Threshold validation tests
```

---

## Error Module Tests (error_test.rs)

### Test Cases

#### Error Type Tests
```rust
#[test]
fn test_parse_error() {
    let error = Error::parse(10, 5, "unexpected token");
    assert!(error.to_string().contains("line 10"));
    assert!(error.to_string().contains("column 5"));
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
    assert!(error.to_string().contains("Undefined reference"));
}

#[test]
fn test_circular_dependency() {
    let error = Error::circular_dependency("step_1 -> step_2 -> step_3 -> step_1");
    assert!(error.to_string().contains("Circular dependency"));
}

#[test]
fn test_threshold_error() {
    let error = Error::threshold(
        "max_allowed.ram",
        "value 120 exceeds maximum 100",
        Some(120.0),
        Some("0-100".to_string()),
    );
    assert!(error.to_string().contains("Threshold validation error"));
    assert!(error.to_string().contains("120"));
}
```

#### Error Constructor Tests
```rust
#[test]
fn test_yaml_syntax() {
    let error = Error::yaml_syntax("invalid YAML syntax");
    assert!(matches!(error, Error::YamlSyntax { .. }));
}

#[test]
fn test_schema_validation() {
    let error = Error::schema_validation("workflow_id", "required field missing", None);
    assert!(matches!(error, Error::SchemaValidation { .. }));
}

#[test]
fn test_interpolation() {
    let error = Error::interpolation("${unknown.variable}");
    assert!(matches!(error, Error::Interpolation { .. }));
}

#[test]
fn test_database() {
    let error = Error::database("open", "database locked");
    assert!(matches!(error, Error::Database { .. }));
}

#[test]
fn test_file_system() {
    let error = Error::file_system("read", PathBuf::from("/test.txt"), "not found");
    assert!(matches!(error, Error::FileSystem { .. }));
}
```

#### Error Display Tests
```rust
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
fn test_error_debug() {
    let error = Error::parse(1, 1, "test");
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("Parse"));
}

#[test]
fn test_error_source() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error: Error = io_error.into();
    assert!(matches!(error, Error::Io(_)));
}
```

---

## Schema Type Tests (schema_test.rs)

### Test Cases

#### Workflow Identification Tests
```rust
#[test]
fn test_workflow_identification() {
    let id = WorkflowIdentification {
        workflow_id: "test_workflow".to_string(),
        name: "Test Workflow".to_string(),
        description: "A test workflow".to_string(),
        version: "1.0.0".to_string(),
        author: "Test Author".to_string(),
        tags: vec!["test".to_string(), "example".to_string()],
    };

    assert_eq!(id.workflow_id, "test_workflow");
    assert_eq!(id.version, "1.0.0");
    assert_eq!(id.tags.len(), 2);
}
```

#### Model Configuration Tests
```rust
#[test]
fn test_model_config() {
    let model = ModelConfig {
        name: "test_model".to_string(),
        var_name: Some("primary".to_string()),
        host: ModelHost {
            provider_type: ModelProvider::LmStudio,
            connection_settings: Default::default(),
        },
        max_allowed: ResourceLimits::default(),
        min_allowed: ResourceLimits::default(),
        allocation_strategy: "dynamic".to_string(),
        model_memory: ModelMemory::default(),
        execution: ExecutionTimeouts::default(),
        thinking: ThinkingConfig::default(),
        tools: ToolPermissions::default(),
        hooks: LifecycleHooks::default(),
        guardrails: None,
        framework: FrameworkType::Agentsdk,
        custom_executor_name: None,
    };

    assert_eq!(model.name, "test_model");
    assert_eq!(model.framework, FrameworkType::Agentsdk);
    assert_eq!(model.allocation_strategy, "dynamic");
}

#[test]
fn test_model_provider_variants() {
    let lmstudio = ModelProvider::LmStudio;
    let ollama = ModelProvider::Ollama;
    let llama = ModelProvider::LlamaCppWithVulkan;

    assert!(matches!(lmstudio, ModelProvider::LmStudio));
    assert!(matches!(ollama, ModelProvider::Ollama));
    assert!(matches!(llama, ModelProvider::LlamaCppWithVulkan));
}

#[test]
fn test_resource_limit_variants() {
    let percentage = ResourceLimit::Percentage(50.0);
    let absolute = ResourceLimit::Absolute {
        value: 8,
        unit: "GB".to_string(),
    };

    match percentage {
        ResourceLimit::Percentage(p) => assert_eq!(p, 50.0),
        _ => panic!("Expected Percentage"),
    }

    match absolute {
        ResourceLimit::Absolute { value, unit } => {
            assert_eq!(value, 8);
            assert_eq!(unit, "GB");
        }
        _ => panic!("Expected Absolute"),
    }
}
```

#### Step Configuration Tests
```rust
#[test]
fn test_step_config() {
    let step = Step {
        step: "step_1".to_string(),
        id: Some("unique_id".to_string()),
        name: Some("Test Step".to_string()),
        description: Some("A test step".to_string()),
        step_type: StepType::Agent,
        generative_entity: None,
        model: Some("${models.primary}".to_string()),
        model_overrides: None,
        prompt: Some("Test prompt".to_string()),
        input_variables: None,
        file_operations: None,
        context: None,
        output: None,
        retry: None,
        branches: None,
        parallel_group: None,
        max_parallel: None,
        timeout_secs: None,
        depends_on: None,
        pre_hooks: None,
        post_hooks: None,
    };

    assert_eq!(step.step, "step_1");
    assert_eq!(step.step_type, StepType::Agent);
    assert_eq!(step.model, Some("${models.primary}".to_string()));
}

#[test]
fn test_retry_config_defaults() {
    let retry = RetryConfig::default();
    assert_eq!(retry.max_attempts, 3);
    assert_eq!(retry.backoff, BackoffStrategy::Exponential);
    assert_eq!(retry.delay_ms, 1000);
}

#[test]
fn test_step_type_variants() {
    assert!(matches!(StepType::Agent, StepType::Agent));
    assert!(matches!(StepType::Tool, StepType::Tool));
    assert!(matches!(StepType::SubWorkflow, StepType::SubWorkflow));
    assert!(matches!(StepType::Control, StepType::Control));
    assert!(matches!(StepType::Loop, StepType::Loop));
}
```

#### Serialization Tests
```rust
#[test]
fn test_workflow_spec_serialization() {
    let spec = WorkflowSpec {
        identification: WorkflowIdentification {
            workflow_id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test".to_string(),
            version: "1.0.0".to_string(),
            author: "".to_string(),
            tags: vec![],
        },
        models: ModelsConfig::default(),
        workspace: WorkspaceConfig {
            root_path: "/test".to_string(),
            ..Default::default()
        },
        features: FeaturesDemonstrated::default(),
        execution: WorkflowExecutionStrategy::default(),
        tool_permissions: ToolPermissionsConfig::default(),
        logging: LoggingConfig::default(),
        agentic_workflow: None,
        pipeline: None,
    };

    // Test serialization to JSON
    let json = serde_json::to_string(&spec).unwrap();
    assert!(json.contains("workflow_id"));

    // Test deserialization
    let deserialized: WorkflowSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.identification.workflow_id, "test");
}

#[test]
fn test_round_trip_serialization() {
    let spec = create_test_workflow_spec();

    let yaml = serde_yaml::to_string(&spec).unwrap();
    let deserialized: WorkflowSpec = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(deserialized.identification.workflow_id, spec.identification.workflow_id);
    assert_eq!(deserialized.identification.name, spec.identification.name);
}
```

---

## Parser Tests (parser_test.rs)

### Test Cases

#### Parse Minimal Workflow
```rust
#[test]
fn test_parse_minimal_workflow() {
    let yaml = include_str!("../tests/fixtures/workflows/minimal.yml");
    let spec = parse_workflow_str(yaml).unwrap();

    assert_eq!(spec.identification.workflow_id, "minimal_workflow");
    assert_eq!(spec.identification.name, "Minimal Workflow");
    assert_eq!(spec.identification.version, "1.0.0");
}
```

#### Parse Complex Workflow
```rust
#[test]
fn test_parse_complex_workflow() {
    let spec = parse_workflow("tests/fixtures/workflows/complex.yml").unwrap();

    assert_eq!(spec.identification.workflow_id, "complex_workflow");
    assert_eq!(spec.identification.version, "2.0.0");
    assert_eq!(spec.identification.author, "Test Author");
    assert!(spec.models.models.len(), 2);
}
```

#### Parse Agentic Workflow
```rust
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
      step: test_step
      model: "${models.primary}"
      prompt: "Test prompt"
"#;

    let spec = parse_workflow_str(yaml).unwrap();
    assert!(spec.agentic_workflow.is_some());
    let steps = spec.agentic_workflow.unwrap().steps;
    assert!(steps.contains_key("step_1"));
}
```

#### Error Handling Tests
```rust
#[test]
fn test_invalid_yaml() {
    let invalid_yaml = "invalid: yaml: content:";

    let result = parse_workflow_str(invalid_yaml);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("YAML"));
}

#[test]
fn test_missing_required_field() {
    let missing_id = r#"
name: "Test Workflow"
description: "Test"
version: "1.0.0"
"#;

    let result = parse_workflow_str(missing_id);
    assert!(result.is_err());
}

#[test]
fn test_parse_from_nonexistent_file() {
    let result = parse_workflow("/nonexistent/path/workflow.yml");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Io(_)));
}
```

#### Validation Tests
```rust
#[test]
fn test_validate_workflow() {
    let yaml = include_str!("../tests/fixtures/workflows/minimal.yml");
    let spec = parse_workflow_str(yaml).unwrap();
    let result = validate_workflow(&spec);

    assert!(result.is_ok());
}

#[test]
fn test_validate_invalid_workflow() {
    let yaml = r#"
workflow_id: test_workflow
name: "Test Workflow"
description: "Test"
version: "1.0.0"

models:
  primary:
    host:
      type: lmstudio

workspace:
  root_path: ""

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
```

---

## Running Tests

```bash
# Run all unit tests
cargo test

# Run specific test file
cargo test error_test

# Run with output
cargo test -- --nocapture

# Run with backtrace
cargo test -- --backtrace
```

---

## Coverage Goals

- **Error Module**: 100% coverage (all variants and constructors)
- **Schema Types**: 95%+ coverage (all types and serialization)
- **Parser**: 90%+ coverage (parse paths and error handling)
- **Interpolation**: 100% coverage (all syntaxes and scopes)
- **IR Types**: 100% coverage (all types and conversions)
- **Compiler**: 85%+ coverage (compilation paths and errors)
- **DAG Validation**: 95%+ coverage (all graph patterns)
- **Policy Compiler**: 90%+ coverage (all policies and inheritance)
- **Storage**: 85%+ coverage (all operations)
- **Workspace**: 90%+ coverage (all operations)
- **Defaults**: 95%+ coverage (all defaults and inheritance)
- **Threshold Validation**: 100% coverage (all thresholds)

---

**Document Version:** 1.0.0
**Last Updated:** 2026-04-06
**Status:** Complete
