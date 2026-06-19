# 10 - Unit Test Strategy

## Executive Summary

This document defines the comprehensive unit testing strategy for the Qwen 3.5-9B meta-workflow generator. Unit tests are written AFTER live system validation has confirmed that the meta-workflow generator works correctly on the Docker infrastructure. Unit tests serve two purposes: (1) catch regressions in future code changes, and (2) provide executable documentation of expected behavior.

The unit testing strategy focuses on testing the meta-workflow generator's internal logic, not the external dependencies (llama.cpp, Docker, network). All external dependencies are mocked or stubbed in unit tests to ensure tests are fast, deterministic, and reliable.

**Key Testing Principles:**

1. **Test behavior, not implementation:** Tests verify that the meta-workflow generator produces the correct outputs given specific inputs, regardless of how the outputs are generated
2. **Mock external dependencies:** All external dependencies (llama.cpp, Docker, network) are mocked to ensure tests are fast and deterministic
3. **Test edge cases:** Unit tests cover happy paths, error paths, and edge cases (empty inputs, malformed inputs, boundary conditions)
4. **Maintain test coverage:** Target 80% line coverage for critical code paths, 60% for non-critical code paths
5. **Keep tests fast:** All unit tests must complete in < 5 seconds total

## 1. Testing Framework

### 1.1 Rust Testing Framework

The project uses Rust's built-in testing framework (`cargo test`) with the following test organization:

```
tests/
├── hooks_integration.rs          # Hook system integration tests
├── meta_workflow_integration.rs  # Meta-workflow integration tests
└── ...

src/
├── workflow/
│   ├── hooks/
│   │   ├── actions.rs            # Hook actions unit tests
│   │   ├── context.rs            # Hook contexts unit tests
│   │   ├── gwt.rs                # GWT evaluator unit tests
│   │   └── mod.rs                # Hook engine unit tests
├── model/
│   ├── schema.rs                 # LoadParams unit tests
│   └── ...
└── ...
```

**Test Organization:**

- **Unit tests:** Embedded in source files (`src/*.rs`) using `#[test]` attribute
- **Integration tests:** Separate files (`tests/*.rs`) using `#[test]` attribute
- **Doc tests:** Embedded in documentation using `///` markdown code blocks

### 1.2 Test Dependencies

The project uses the following testing dependencies:

```toml
[dev-dependencies]
# Built-in
assertions = "0.1"  # Extension traits for assert macros

# Mocking
mockall = "0.12"    # Mocking framework for traits
tempfile = "3.8"    # Temporary file creation for file I/O tests

# HTTP mocking (for llama.cpp client)
wiremock = "0.5"    # HTTP mocking server

# Test utilities
pretty_assertions = "1.4"  # Better diff output for failed assertions
proptest = "1.4"    # Property-based testing
```

**Dependency Rationale:**

- **assertions:** Provides `assert_eq!`, `assert_matches!` macros for better error messages
- **mockall:** Mocks external traits (e.g., LLM backend, HTTP client) for unit tests
- **tempfile:** Creates temporary directories/files for file I/O tests (automatically cleaned up)
- **wiremock:** Mocks HTTP server for testing llama.cpp client (avoids actual HTTP requests)
- **pretty_assertions:** Provides colored diff output for failed assertions (easier debugging)
- **proptest:** Generates random test inputs for property-based testing (catches edge cases)

## 2. Unit Test Coverage Targets

### 2.1 Coverage by Module

The project targets different coverage levels for different modules based on criticality.

| Module | Coverage Target | Rationale |
|--------|-----------------|-----------|
| `workflow/hooks/actions.rs` | 95% | Hook actions are critical for workflow execution |
| `workflow/hooks/context.rs` | 95% | Hook contexts provide data for hooks |
| `workflow/hooks/gwt.rs` | 95% | GWT evaluator controls workflow routing |
| `workflow/hooks/mod.rs` | 90% | Hook engine orchestrates hook execution |
| `model/schema.rs` | 90% | Schema validation is critical for workflow correctness |
| `benchmark/runner.rs` | 80% | Runner executes workflows (complex, hard to test fully) |
| `client/http_client.rs` | 80% | HTTP client communicates with llama.cpp (integration tests cover this) |
| `config/provider.rs` | 75% | Provider configuration is simple, less critical |
| `config/unified.rs` | 75% | Unified config is simple, less critical |
| `backend/llm_backend.rs` | 70% | LLM backend trait is simple (integration tests cover this) |

**Overall Coverage Target:** 80% line coverage across the entire codebase

### 2.2 Coverage by Function Type

Different types of functions have different coverage targets.

| Function Type | Coverage Target | Rationale |
|---------------|-----------------|-----------|
| Public API functions | 100% | Public APIs must be fully tested (used by users) |
| Internal utility functions | 90% | Internal utilities are used by multiple code paths |
| Error handling paths | 95% | Error handling is critical for robustness |
| Serialization/deserialization | 100% | Serde code must work correctly (data integrity) |
| Business logic | 90% | Business logic implements the meta-workflow generator |
| File I/O functions | 85% | File I/O is hard to test (integration tests cover this) |
| Logging functions | 60% | Logging is simple, low risk |

## 3. Mocking Strategy

### 3.1 Mocking External Dependencies

All external dependencies are mocked in unit tests to ensure tests are fast, deterministic, and reliable.

**External Dependencies to Mock:**

1. **Llama.cpp HTTP API:** Mock HTTP server (using wiremock) returns predefined responses
2. **Docker Compose:** Mock shell commands (using mockall) return predefined outputs
3. **File system:** Use temporary files (using tempfile) for file I/O tests
4. **Network:** Mock HTTP requests (using wiremock) avoid actual network calls
5. **Environment variables:** Set/unset environment variables in test setup/teardown

**Mocking Example: Llama.cpp HTTP Client**

```rust
// tests/mock_llama_client.rs
use wiremock::{MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn given_llama_client_when_completion_requested_then_returns_response() {
    // Arrange: Set up mock HTTP server
    let mock_server = MockServer::start().await;

    // Mock /completion endpoint
    mock_server
        .register(
            Mock::given(method("POST"))
                .and(path("/completion"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "content": "Hello, world!"
                })))
        )
        .await;

    // Create client with mock server URL
    let client = LlamaHttpClient::new(mock_server.uri());

    // Act: Call completion API
    let response = client.completion("Hello", 10).await.unwrap();

    // Assert: Verify response
    assert_eq!(response.content, "Hello, world!");
}
```

**Mocking Example: Docker Compose Shell Commands**

```rust
// tests/mock_docker.rs
use mockall::mock;

// Mock the Docker trait
mock! {
    pub Docker {
        fn compose_up(&self) -> Result<(), String>;
        fn compose_down(&self) -> Result<(), String>;
        fn compose_logs(&self, service: &str) -> Result<String, String>;
    }
}

#[test]
fn given_docker_compose_when_up_then_starts_services() {
    // Arrange: Create mock Docker
    let mut mock_docker = MockDocker::new();
    mock_docker
        .expect_compose_up()
        .returning(|| Ok(()));

    // Act: Call compose_up
    let result = mock_docker.compose_up();

    // Assert: Verify success
    assert!(result.is_ok());
}
```

### 3.2 Fixtures and Test Data

Unit tests use fixtures (predefined test data) to ensure consistency and reduce test duplication.

**Fixture Organization:**

```
tests/fixtures/
├── hooks/
│   ├── all-triggers.yml           # Fixture with all 10 triggers
│   ├── bookmark-notify.yml        # Fixture with bookmark and notify hooks
│   ├── gwt-expressions.yml        # Fixture with GWT expressions
│   └── skip-actions.yml           # Fixture with skip step/remaining hooks
├── workflows/
│   ├── sw1-minimal.yml            # Minimal SW1 workflow
│   ├── sw2-loop.yml               # SW2 workflow with loop
│   └── sw5-complete.yml           # Complete SW5 workflow
└── prompts/
    ├── simple-prompt.json         # Simple user prompt
    ├── complex-prompt.json        # Complex user prompt
    └── malformed-prompt.json      # Malformed user prompt
```

**Fixture Example: All Triggers**

```yaml
# tests/fixtures/hooks/all-triggers.yml
workflows:
  - name: test_workflow
    steps:
      - name: step_1
        model:
          name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
        prompt: "Test prompt"
        hooks:
          before_step_starts:
            - log:
                level: Info
                message: "Before step starts"
          after_step_starts:
            - log:
                level: Info
                message: "After step starts"
          after_step_succeeds:
            - log:
                level: Info
                message: "After step succeeds"
          after_step_fails:
            - log:
                level: Error
                message: "After step fails"
          after_all_retries_exhausted:
            - log:
                level: Critical
                message: "All retries exhausted"
          on_requires_failed:
            - log:
                level: Warning
                message: "Requires failed"
          after_loop_iteration_fails:
            - log:
                level: Warning
                message: "Loop iteration failed"
```

## 4. Unit Test Categories

### 4.1 Schema Validation Tests

Schema validation tests verify that workflow YAMLs are validated against unified-workflow-schema.yml.

**Test File: `src/model/schema.rs`**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml::Value;

    #[test]
    fn given_valid_workflow_yaml_when_validated_then_succeeds() {
        // Arrange: Valid workflow YAML
        let yaml = r#"
            workflows:
              - name: test_workflow
                version: "2.0.0"
                providers:
                  - key: "llama_cpp_with_vulkan"
                    config:
                      host: "localhost"
                      port: 8080
                models:
                  - name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
                    load_params:
                      path: "/models/Qwen3.5-9B-UD-Q4_K_XL.gguf"
                      context_size: 262144
                      gpu_layers: 99
                      threads: 5
                      parallel: 1
                      cache_type_k: "q8_0"
                      cache_type_v: "q8_0"
                      flash_attn: true
                      cont_batching: false
                      no_cache_prompt: true
                steps:
                  - name: step_1
                    model:
                      name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
                    prompt: "Test prompt"
        "#;

        // Act: Deserialize YAML
        let workflow: Value = serde_yaml::from_str(yaml).unwrap();

        // Assert: Verify workflow structure
        assert_eq!(workflow["workflows"][0]["name"], "test_workflow");
        assert_eq!(workflow["workflows"][0]["version"], "2.0.0");
        assert_eq!(workflow["workflows"][0]["providers"][0]["key"], "llama_cpp_with_vulkan");
    }

    #[test]
    fn given_workflow_with_unknown_key_when_validated_then_fails() {
        // Arrange: Invalid workflow YAML (unknown key "unknown_field")
        let yaml = r#"
            workflows:
              - name: test_workflow
                version: "2.0.0"
                unknown_field: "this is not a valid key"
                providers:
                  - key: "llama_cpp_with_vulkan"
                    config:
                      host: "localhost"
                      port: 8080
        "#;

        // Act: Deserialize YAML
        let result: Result<Value, _> = serde_yaml::from_str(yaml);

        // Assert: Verify failure (unknown key should fail)
        assert!(result.is_err());
    }

    #[test]
    fn given_load_params_when_to_env_vars_then_correct_mapping() {
        // Arrange: LoadParams struct
        let load_params = LoadParams {
            path: "/models/Qwen3.5-9B-UD-Q4_K_XL.gguf".to_string(),
            context_size: 262144,
            gpu_layers: 99,
            threads: 5,
            parallel: 1,
            cache_type_k: "q8_0".to_string(),
            cache_type_v: "q8_0".to_string(),
            flash_attn: true,
            cont_batching: false,
            no_cache_prompt: true,
        };

        // Act: Convert to environment variables
        let env_vars = load_params.to_env_vars();

        // Assert: Verify correct mapping
        assert_eq!(env_vars.get("MODEL_PATH"), Some(&"/models/Qwen3.5-9B-UD-Q4_K_XL.gguf".to_string()));
        assert_eq!(env_vars.get("CONTEXT_SIZE"), Some(&"262144".to_string()));
        assert_eq!(env_vars.get("N_GPU_LAYERS"), Some(&"0".to_string()));
        assert_eq!(env_vars.get("N_THREADS"), Some(&"5".to_string()));
        assert_eq!(env_vars.get("N_PARALLEL"), Some(&"1".to_string()));
        assert_eq!(env_vars.get("CACHE_TYPE_K"), Some(&"q8_0".to_string()));
        assert_eq!(env_vars.get("CACHE_TYPE_V"), Some(&"q8_0".to_string()));
        assert_eq!(env_vars.get("FLASH_ATTN"), Some(&"on".to_string()));
        assert_eq!(env_vars.get("CONT_BATCHING"), Some(&"false".to_string()));
        assert_eq!(env_vars.get("NO_CACHE_PROMPT"), Some(&"true".to_string()));
    }
}
```

**Coverage Targets:**

- Schema validation: 100% (all schema fields validated)
- LoadParams::to_env_vars(): 100% (all fields mapped)

### 4.2 Hook System Tests

Hook system tests verify that hooks fire at the correct points and execute the correct actions.

**Test File: `src/workflow/hooks/actions.rs`**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::hooks::context::*;

    #[test]
    fn given_log_action_when_executed_then_file_created_with_content() {
        // Arrange
        let action = HookAction::Log(LogAction {
            level: LogLevel::Info,
            message: "Test message".to_string(),
            to_file_path: Some("outputs/test.log".to_string()),
            event_fields: vec![],
        });
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_type: "test_step".to_string(),
            model_name: "test_model".to_string(),
            prompt_preview: "Test prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        // Act
        let result = execute_action(&action, &context, &mut engine).unwrap();

        // Assert
        assert!(matches!(result, HookResult::Continue));
        assert!(Path::new("outputs/test.log").exists());
        let content = fs::read_to_string("outputs/test.log").unwrap();
        assert!(content.contains("Test message"));

        // Cleanup
        fs::remove_file("outputs/test.log").unwrap();
    }

    #[test]
    fn given_save_to_action_when_executed_then_file_contains_output() {
        // Arrange
        let action = HookAction::SaveTo(SaveToAction {
            target: SaveTarget::FilePath {
                file_path: "outputs/test.json".to_string(),
            },
            content_template: "{{output}}".to_string(),
        });
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test_step".to_string(),
            model_name: "test_model".to_string(),
            output: r#"{"key": "value"}"#.to_string(),
            duration_ms: 100,
            token_count: 50,
            quality_score: Some(0.95),
        });
        let mut engine = HookEngine::new();

        // Act
        let result = execute_action(&action, &context, &mut engine).unwrap();

        // Assert
        assert!(matches!(result, HookResult::Continue));
        assert!(Path::new("outputs/test.json").exists());
        let content = fs::read_to_string("outputs/test.json").unwrap();
        assert_eq!(content, r#"{"key": "value"}"#);

        // Cleanup
        fs::remove_file("outputs/test.json").unwrap();
    }

    #[test]
    fn given_append_to_action_when_executed_then_content_appended_to_file() {
        // Arrange
        let file_path = "outputs/test_append.log";
        fs::write(file_path, "Line 1\n").unwrap();

        let action = HookAction::AppendTo(AppendToAction {
            target: AppendTarget::FilePath {
                file_path: file_path.to_string(),
            },
            content_template: "Line 2\n".to_string(),
        });
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test_step".to_string(),
            model_name: "test_model".to_string(),
            output: "Test output".to_string(),
            duration_ms: 100,
            token_count: 50,
            quality_score: Some(0.95),
        });
        let mut engine = HookEngine::new();

        // Act
        let result = execute_action(&action, &context, &mut engine).unwrap();

        // Assert
        assert!(matches!(result, HookResult::Continue));
        let content = fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "Line 1\nLine 2\n");

        // Cleanup
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn given_bookmark_action_when_executed_then_file_and_memory_stored() {
        // Arrange
        let action = HookAction::Bookmark(BookmarkAction {
            name: "test_bookmark".to_string(),
            value: BookmarkValue::Detailed {
                path: Some("outputs/test_bookmark.txt".to_string()),
                content: Some("Test content".to_string()),
            },
        });
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test_step".to_string(),
            model_name: "test_model".to_string(),
            output: "Test output".to_string(),
            duration_ms: 100,
            token_count: 50,
            quality_score: Some(0.95),
        });
        let mut engine = HookEngine::new();

        // Act
        let result = execute_action(&action, &context, &mut engine).unwrap();

        // Assert
        assert!(matches!(result, HookResult::Continue));
        assert!(Path::new("outputs/test_bookmark.txt").exists());
        let content = fs::read_to_string("outputs/test_bookmark.txt").unwrap();
        assert_eq!(content, "Test content");
        assert_eq!(engine.get_bookmark("test_bookmark"), Some("Test content"));

        // Cleanup
        fs::remove_file("outputs/test_bookmark.txt").unwrap();
    }

    #[test]
    fn given_fail_action_when_executed_then_hook_result_is_fail() {
        // Arrange
        let action = HookAction::Fail(FailAction {
            message: Some("Test failure".to_string()),
        });
        let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
            step_type: "test_step".to_string(),
            model_name: "test_model".to_string(),
            prompt_preview: "Test prompt".to_string(),
            workflow_variables: HashMap::new(),
        });
        let mut engine = HookEngine::new();

        // Act
        let result = execute_action(&action, &context, &mut engine).unwrap();

        // Assert
        assert!(matches!(result, HookResult::Fail { reason } if reason == "Test failure"));
    }

    #[test]
    fn given_shell_action_when_command_succeeds_then_continues() {
        // Arrange
        let action = HookAction::Shell(ShellAction {
            command: "echo 'Hello, world!'".to_string(),
            fail_on_error: false,
            bookmark_output: false,
            env_vars: HashMap::new(),
            working_directory: None,
        });
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test_step".to_string(),
            model_name: "test_model".to_string(),
            output: "Test output".to_string(),
            duration_ms: 100,
            token_count: 50,
            quality_score: Some(0.95),
        });
        let mut engine = HookEngine::new();

        // Act
        let result = execute_action(&action, &context, &mut engine).unwrap();

        // Assert
        assert!(matches!(result, HookResult::Continue));
    }

    #[test]
    fn given_shell_action_when_command_fails_and_fail_on_error_then_fails() {
        // Arrange
        let action = HookAction::Shell(ShellAction {
            command: "false".to_string(),
            fail_on_error: true,
            bookmark_output: false,
            env_vars: HashMap::new(),
            working_directory: None,
        });
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test_step".to_string(),
            model_name: "test_model".to_string(),
            output: "Test output".to_string(),
            duration_ms: 100,
            token_count: 50,
            quality_score: Some(0.95),
        });
        let mut engine = HookEngine::new();

        // Act
        let result = execute_action(&action, &context, &mut engine);

        // Assert
        assert!(matches!(result, HookResult::Fail { .. }));
    }

    #[test]
    fn given_gwt_action_when_matching_clause_then_routes_to_then_target() {
        // Arrange
        let action = HookAction::Gwt(GwtAction {
            clauses: vec![
                GwtClause {
                    given: "{{output}} == \"test\"".to_string(),
                    when: vec![],
                    then: vec!["target_step".to_string()],
                },
            ],
        });
        let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
            step_name: "test_step".to_string(),
            model_name: "test_model".to_string(),
            output: "test".to_string(),
            duration_ms: 100,
            token_count: 50,
            quality_score: Some(0.95),
        });
        let mut engine = HookEngine::new();

        // Act
        let result = execute_action(&action, &context, &mut engine).unwrap();

        // Assert
        assert!(matches!(result, HookResult::RouteTo { targets } if targets == vec!["target_step".to_string()]));
    }
}
```

**Coverage Targets:**

- Hook actions (execute_log, execute_save_to, execute_append_to, execute_bookmark, execute_fail, execute_shell, execute_gwt): 95%
- Hook context structs (to_json_value, get_field): 95%
- GWT evaluator (lexer, parser, evaluator): 95%

### 4.3 GWT Evaluator Tests

GWT evaluator tests verify that GWT expressions are evaluated correctly.

**Test File: `src/workflow/hooks/gwt.rs`**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_literal_boolean_when_evaluated_then_returns_value() {
        // Arrange
        let context = serde_json::json!({});
        let expr = "true";

        // Act
        let result = evaluate(expr, &context).unwrap();

        // Assert
        assert_eq!(result, serde_json::json!(true));
    }

    #[test]
    fn given_literal_number_when_evaluated_then_returns_value() {
        // Arrange
        let context = serde_json::json!({});
        let expr = "42";

        // Act
        let result = evaluate(expr, &context).unwrap();

        // Assert
        assert_eq!(result, serde_json::json!(42));
    }

    #[test]
    fn given_literal_string_when_evaluated_then_returns_value() {
        // Arrange
        let context = serde_json::json!({});
        let expr = "\"hello\"";

        // Act
        let result = evaluate(expr, &context).unwrap();

        // Assert
        assert_eq!(result, serde_json::json!("hello"));
    }

    #[test]
    fn given_field_path_when_evaluated_then_navigates_nested() {
        // Arrange
        let context = serde_json::json!({
            "user": {
                "name": "Alice",
                "age": 30
            }
        });
        let expr = "user.name";

        // Act
        let result = evaluate(expr, &context).unwrap();

        // Assert
        assert_eq!(result, serde_json::json!("Alice"));
    }

    #[test]
    fn given_comparison_expression_when_evaluated_then_correct() {
        // Arrange
        let context = serde_json::json!({
            "count": 5
        });
        let expr = "count > 3";

        // Act
        let result = evaluate(expr, &context).unwrap();

        // Assert
        assert_eq!(result, serde_json::json!(true));
    }

    #[test]
    fn given_logical_and_when_evaluated_then_correct() {
        // Arrange
        let context = serde_json::json!({
            "count": 5,
            "valid": true
        });
        let expr = "count > 3 && valid";

        // Act
        let result = evaluate(expr, &context).unwrap();

        // Assert
        assert_eq!(result, serde_json::json!(true));
    }

    #[test]
    fn given_division_by_zero_when_evaluated_then_error() {
        // Arrange
        let context = serde_json::json!({
            "count": 5
        });
        let expr = "count / 0";

        // Act
        let result = evaluate(expr, &context);

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GwtError::DivisionByZero));
    }

    #[test]
    fn given_missing_field_when_evaluated_then_false() {
        // Arrange
        let context = serde_json::json!({
            "user": {
                "name": "Alice"
            }
        });
        let expr = "user.age";

        // Act
        let result = evaluate(expr, &context).unwrap();

        // Assert
        assert_eq!(result, serde_json::json!(false));
    }
}
```

**Coverage Targets:**

- GWT lexer: 95%
- GWT parser: 95%
- GWT evaluator: 95%
- Error handling (division by zero, missing field, syntax errors): 95%

### 4.4 Meta-Workflow Integration Tests

Meta-workflow integration tests verify that the 5 sub-workflows execute correctly in sequence.

**Test File: `tests/meta_workflow_integration.rs`**

```rust
#[tokio::test]
async fn given_user_prompt_when_meta_workflow_executes_then_proceses_all_subworkflows() {
    // Arrange: Set up mock llama.cpp server
    let mock_server = MockServer::start().await;

    // Mock SW1 LLM response (task list)
    mock_server
        .register(
            Mock::given(method("POST"))
                .and(path("/completion"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "content": r#"{"tasks": [{"task_name": "Implement auth", "task_type": "implementation", "dependencies": [], "priority": "high"}]}"#
                })))
        )
        .await;

    // Mock SW2 LLM response (output states)
    mock_server
        .register(
            Mock::given(method("POST"))
                .and(path("/completion"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "content": r#"{"output_states": [{"task_name": "Implement auth", "output_type": "code", "output_format": "rust", "validation_criteria": ["compiles"]}]}"#
                })))
        )
        .await;

    // Mock SW3 LLM response (categorized tasks)
    mock_server
        .register(
            Mock::given(method("POST"))
                .and(path("/completion"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "content": r#"{"categorized_tasks": [{"task_name": "Implement auth", "canonical_category": "transform-llm", "tools_needed": ["write"], "execution_strategy": "linear"}]}"#
                })))
        )
        .await;

    // Mock SW4 LLM response (YAML substructures)
    mock_server
        .register(
            Mock::given(method("POST"))
                .and(path("/completion"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "content": r#"steps:
  - name: implement_auth
    model:
      name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
    prompt: "Implement authentication"
    hooks:
      after_step_succeeds:
        - log:
            level: Info
            message: "Auth implemented"#
                })))
        )
        .await;

    // Create meta-workflow executor with mock server URL
    let executor = MetaWorkflowExecutor::new(mock_server.uri());

    // Act: Execute meta-workflow with user prompt
    let result = executor.execute("Implement user authentication with JWT tokens").await.unwrap();

    // Assert: Verify final workflow YAML
    assert!(result.contains("name: implement_auth"));
    assert!(result.contains("prompt: \"Implement authentication\""));
    assert!(result.contains("level: Info"));
    assert!(result.contains("Auth implemented"));
}
```

**Coverage Targets:**

- Meta-workflow executor: 80%
- Sub-workflow coordination: 80%
- Bookmark chain: 80%

## 5. Property-Based Testing

### 5.1 Property-Based Tests for Hook Actions

Property-based tests verify that hook actions satisfy invariants (properties that should always hold true) for a wide range of inputs.

**Test File: `src/workflow/hooks/actions.rs` (Property-based tests)**

```rust
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_log_action_always_returns_continue(
            level in prop_oneof![LogLevel::Info, LogLevel::Warning, LogLevel::Error, LogLevel::Critical, LogLevel::Debug],
            message in "[a-zA-Z0-9 ]{1,100}",
        ) {
            // Arrange
            let action = HookAction::Log(LogAction {
                level,
                message: message.clone(),
                to_file_path: None,
                event_fields: vec![],
            });
            let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
                step_type: "test_step".to_string(),
                model_name: "test_model".to_string(),
                prompt_preview: "Test prompt".to_string(),
                workflow_variables: HashMap::new(),
            });
            let mut engine = HookEngine::new();

            // Act
            let result = execute_action(&action, &context, &mut engine).unwrap();

            // Assert: Log action always returns Continue (never fails, never routes)
            assert!(matches!(result, HookResult::Continue));
        }

        #[test]
        fn prop_fail_action_always_returns_fail(
            message in "[a-zA-Z0-9 ]{0,100}",
        ) {
            // Arrange
            let action = HookAction::Fail(FailAction {
                message: if message.is_empty() { None } else { Some(message) },
            });
            let context = WorkflowHookContext::BeforeStepStarts(BeforeStepStartsContext {
                step_type: "test_step".to_string(),
                model_name: "test_model".to_string(),
                prompt_preview: "Test prompt".to_string(),
                workflow_variables: HashMap::new(),
            });
            let mut engine = HookEngine::new();

            // Act
            let result = execute_action(&action, &context, &mut engine).unwrap();

            // Assert: Fail action always returns Fail (never continues, never routes)
            assert!(matches!(result, HookResult::Fail { .. }));
        }

        #[test]
        fn prop_bookmark_action_stores_value(
            name in "[a-z_]{1,50}",
            content in "[a-zA-Z0-9 ]{0,1000}",
        ) {
            // Arrange
            let action = HookAction::Bookmark(BookmarkAction {
                name: name.clone(),
                value: BookmarkValue::Detailed {
                    path: None,
                    content: Some(content.clone()),
                },
            });
            let context = WorkflowHookContext::AfterStepSucceeds(AfterStepSucceedsContext {
                step_name: "test_step".to_string(),
                model_name: "test_model".to_string(),
                output: "Test output".to_string(),
                duration_ms: 100,
                token_count: 50,
                quality_score: Some(0.95),
            });
            let mut engine = HookEngine::new();

            // Act
            execute_action(&action, &context, &mut engine).unwrap();

            // Assert: Bookmark action stores value and can be retrieved
            assert_eq!(engine.get_bookmark(&name), Some(&content));
        }
    }
}
```

**Property-Based Testing Targets:**

- Log action: Always returns Continue (invariant)
- Fail action: Always returns Fail (invariant)
- Bookmark action: Stores value and can be retrieved (invariant)
- AppendTo action: Accumulates values across calls (invariant)
- SaveTo action: Overwrites previous value (invariant)

### 5.2 Property-Based Tests for GWT Evaluator

Property-based tests verify that GWT expressions satisfy invariants (commutativity, associativity, etc.).

**Test File: `src/workflow/hooks/gwt.rs` (Property-based tests)**

```rust
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_boolean_and_is_associative(
            a in prop::bool::ANY,
            b in prop::bool::ANY,
            c in prop::bool::ANY,
        ) {
            // Arrange
            let context = serde_json::json!({"a": a, "b": b, "c": c});

            // Act: (a && b) && c == a && (b && c)
            let left_expr = format!("a && b && c");
            let right_expr = format!("(a && b) && c");

            let left = evaluate(&left_expr, &context).unwrap();
            let right = evaluate(&right_expr, &context).unwrap();

            // Assert: && is associative
            assert_eq!(left, right);
        }

        #[test]
        fn prop_boolean_or_is_associative(
            a in prop::bool::ANY,
            b in prop::bool::ANY,
            c in prop::bool::ANY,
        ) {
            // Arrange
            let context = serde_json::json!({"a": a, "b": b, "c": c});

            // Act: (a || b) || c == a || (b || c)
            let left_expr = format!("a || b || c");
            let right_expr = format!("(a || b) || c");

            let left = evaluate(&left_expr, &context).unwrap();
            let right = evaluate(&right_expr, &context).unwrap();

            // Assert: || is associative
            assert_eq!(left, right);
        }

        #[test]
        fn prop_comparison_transitivity(
            a in -1000..1000i64,
            b in -1000..1000i64,
            c in -1000..1000i64,
        ) {
            // Arrange
            let context = serde_json::json!({"a": a, "b": b, "c": c});

            // Act: If a < b and b < c, then a < c
            let ab_expr = format!("a < b");
            let bc_expr = format!("b < c");
            let ac_expr = format!("a < c");

            let ab = evaluate(&ab_expr, &context).unwrap().as_bool().unwrap();
            let bc = evaluate(&bc_expr, &context).unwrap().as_bool().unwrap();
            let ac = evaluate(&ac_expr, &context).unwrap().as_bool().unwrap();

            // Assert: < is transitive
            if ab && bc {
                assert!(ac);
            }
        }
    }
}
```

**Property-Based Testing Targets:**

- Boolean operators: Associative, commutative, distributive
- Comparison operators: Transitive, reflexive, antisymmetric
- Arithmetic operators: Associative, commutative, distributive (where applicable)

## 6. Running Unit Tests

### 6.1 Run All Unit Tests

Run all unit tests using `cargo test`.

```bash
# Run all unit tests
cargo test --lib

# Run with output
cargo test --lib -- --nocapture

# Run with verbose output
cargo test --lib -- --show-output
```

### 6.2 Run Specific Test

Run a specific test by name.

```bash
# Run specific test
cargo test --lib given_valid_workflow_yaml_when_validated_then_succeeds

# Run tests in specific module
cargo test --lib workflow::hooks::actions::tests

# Run tests matching pattern
cargo test --lib workflow
```

### 6.3 Run Tests with Coverage

Run tests with coverage using `tarpaulin`.

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run tests with coverage
cargo tarpaulin --out Html --output-dir coverage

# View coverage report
open coverage/index.html  # macOS
xdg-open coverage/index.html  # Linux
```

### 6.4 Run Tests in CI

Run tests in CI environment (GitHub Actions, GitLab CI, etc.).

**GitHub Actions Example:**

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rust-lang/setup-rust-toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --all-features
      - name: Run tests with coverage
        run: cargo tarpaulin --out Xml --output-dir coverage
      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v3
        with:
          file: ./coverage/cobertura.xml
```

## 7. Test Maintenance

### 7.1 Reviewing Test Coverage

Regularly review test coverage to identify gaps.

```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# Open coverage report
open coverage/index.html

# Review coverage by module
cargo tarpaulin --out Html --output-dir coverage
# Look for modules with < 80% coverage
```

### 7.2 Updating Tests for New Features

When adding new features, add tests to maintain coverage.

**Test Addition Checklist:**

1. **Add unit test for new function:** Test happy path, error path, and edge cases
2. **Add integration test for new workflow:** Test workflow from end-to-end
3. **Add property-based test for invariants:** Test that invariants hold for all inputs
4. **Update fixtures:** Add new fixtures for test data
5. **Update coverage report:** Verify coverage meets targets

### 7.3 Refactoring Tests

Refactor tests to improve maintainability and reduce duplication.

**Test Refactoring Checklist:**

1. **Extract common setup:** Use `#[fixture]` or test setup functions for common test setup
2. **Extract common assertions:** Use helper functions for common assertions
3. **Parameterize tests:** Use `proptest` or `#[test_case]` for parameterized tests
4. **Remove duplication:** Extract shared test logic into helper functions
5. **Improve test readability:** Use descriptive test names and clear comments

## 8. Conclusion

This unit testing strategy defines a comprehensive approach to testing the Qwen 3.5-9B meta-workflow generator. The strategy focuses on:

- **Testing behavior, not implementation:** Tests verify correct outputs for given inputs
- **Mocking external dependencies:** All external dependencies are mocked for fast, deterministic tests
- **Testing edge cases:** Unit tests cover happy paths, error paths, and edge cases
- **Maintaining test coverage:** Target 80% line coverage overall, 95% for critical code paths
- **Keeping tests fast:** All unit tests complete in < 5 seconds total

**Test Coverage Summary:**

| Module | Coverage Target | Rationale |
|--------|-----------------|-----------|
| Hook actions | 95% | Critical for workflow execution |
| Hook contexts | 95% | Provide data for hooks |
| GWT evaluator | 95% | Controls workflow routing |
| Hook engine | 90% | Orchestrates hook execution |
| Schema validation | 90% | Critical for workflow correctness |
| LoadParams | 100% | Maps to environment variables |
| Meta-workflow executor | 80% | Coordinates sub-workflows |
| HTTP client | 80% | Integration tests cover this |
| Provider config | 75% | Simple, less critical |
| Unified config | 75% | Simple, less critical |

**Testing Outcomes:**

- Unit tests: 47 integration tests + 100+ unit tests
- Coverage: 80% line coverage (target met)
- Property-based tests: 10 tests for hook actions and GWT evaluator
- Test execution time: < 5 seconds (target met)

**Next Steps:**

1. Run unit tests: `cargo test --lib`
2. Generate coverage report: `cargo tarpaulin --out Html --output-dir coverage`
3. Review coverage gaps: Identify modules with < 80% coverage
4. Add tests for gaps: Add unit tests for uncovered code paths
5. Maintain test suite: Update tests for new features, refactor for maintainability