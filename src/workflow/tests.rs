#[cfg(test)]
mod tests {
    use crate::workflow::WorkflowFile;

    #[test]
    fn parse_minimal_workflow() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
schema_version: "2.0.0"
min_schema_version: "2.0.0"
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert_eq!(workflow.workflow_id, "test-workflow");
        assert_eq!(workflow.name, "Test Workflow");
    }

    #[test]
    fn parse_with_all_fields() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
description: "Test description"
version: "1.0.0"
author: "test-author"
tags: [test, example]
schema_version: "2.0.0"
min_schema_version: "2.0.0"
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert_eq!(workflow.workflow_id, "test-workflow");
        assert_eq!(workflow.description, Some("Test description".to_string()));
        assert_eq!(workflow.version, Some("1.0.0".to_string()));
        assert_eq!(workflow.author, Some("test-author".to_string()));
        assert_eq!(workflow.tags, Some(vec!["test".to_string(), "example".to_string()]));
    }

    #[test]
    fn parse_enum_values() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
workflow_execution_strategy:
  load_unload: one_at_a_time
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert!(workflow.workflow_execution_strategy.is_some());
        let strategy = workflow.workflow_execution_strategy.unwrap();
        assert!(strategy.load_unload.is_some());
    }

    #[test]
    fn parse_log_levels() {
        let levels = ["debug", "info", "warning", "error", "critical"];
        for level in levels {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps: {{}}
  when:
    after_step_fails:
      - log:
          level: {}
"#, level);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", level));
            assert!(workflow.agentic_workflow.is_some());
        }
    }

    #[test]
    fn parse_user_input_types() {
        let types = ["text_area", "text_line", "select", "multi_select", "confirm"];
        for input_type in types {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  user_inputs:
    execution_mode: interactive
    workflow_level:
      inputs:
        - id: test-input
          type: {}
"#, input_type);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", input_type));
            assert!(workflow.agentic_workflow.is_some());
        }
    }

    #[test]
    fn parse_comparison_operators() {
        let ops = [">=", "<=", "==", "!=", ">", "<"];
        for op in ops {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test-step:
      loop:
        validation:
          exact_criteria:
            - metric: quality
              operator: "{}"
              target: 0.9
"#, op);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", op));
            assert!(workflow.agentic_workflow.is_some());
        }
    }

    #[test]
    fn parse_tool_permissions() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
tool_permissions:
  file_operations:
    read:
      allowed_paths: ["/workspace/src"]
      forbidden_paths: ["/etc"]
    write:
      require_confirmation: true
  shell_operations:
    exec:
      allowed_commands: [cargo, rustc]
      timeout_seconds: 30
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert!(workflow.tool_permissions.is_some());
        let perms = workflow.tool_permissions.unwrap();
        assert!(perms.file_operations.is_some());
        assert!(perms.shell_operations.is_some());
    }

    #[test]
    fn parse_hook_actions() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test-step:
      when:
        after_step_succeeds:
          - log:
              to_file_path: "./test.log"
              event_fields: [step_name]
          - save_to: output_var
          - bookmark: true
          - notify:
              message: "Step completed"
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn parse_retry_config() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  retry:
    max_attempts_per_step: 10
    max_attempts_per_workflow: 100
    backoff_strategy:
      backoff: exponential
      initial_delay: "1s"
      max_delay: "30s"
      multiplier: 2.0
      jitter: true
  steps: {}
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert!(workflow.agentic_workflow.is_some());
        let aw = workflow.agentic_workflow.unwrap();
        assert!(aw.retry.is_some());
        let retry = aw.retry.unwrap();
        assert_eq!(retry.max_attempts_per_step, Some(10));
        assert_eq!(retry.max_attempts_per_workflow, Some(100));
    }

    #[test]
    fn parse_memory_config() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
memory:
  rag:
    knowledge_base:
      path: "./workspace/rag"
      format: vector_db
      chunk_size: 512
    embedding_model:
      model_ref: "test-model"
      dimension: 768
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert!(workflow.memory.is_some());
        let memory = workflow.memory.unwrap();
        assert!(memory.rag.is_some());
    }

    #[test]
    fn parse_workspace_config() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
workspace:
  root_path: "./workspace"
  directories:
    output: "./workspace/output"
    logs: "./workspace/logs"
  permissions:
    default_mode: owner_full_group_read_exec_other_read_exec
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert!(workflow.workspace.is_some());
        let workspace = workflow.workspace.unwrap();
        assert!(workspace.directories.is_some());
    }

    #[test]
    fn from_yaml_method() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
schema_version: "2.0.0"
"#;
        let workflow = WorkflowFile::from_yaml(yaml).expect("parse from_yaml");
        assert_eq!(workflow.workflow_id, "test-workflow");
    }

    #[test]
    fn missing_required_fields() {
        let yaml = r#"
name: "Test Workflow"
"#;
        let result: Result<WorkflowFile, _> = serde_saphyr::from_str(yaml);
        assert!(result.is_err(), "Should fail without workflow_id");
    }

    #[test]
    fn parse_kv_cache_quantization() {
        let variants = ["auto", "q4_k_m", "q4_0", "q5_k_m", "q5_0", "q6_k", "q8_0"];
        for variant in variants {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
models:
  test-model:
    model_memory:
      kv_cache_quantization: {}
"#, variant);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", variant));
            assert!(workflow.models.is_some());
        }
    }

    #[test]
    fn parse_cache_sizes() {
        let sizes = ["min", "max", "medium", "medium-min", "medium-max"];
        for size in sizes {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
workflow_execution_strategy:
  memory:
    model_lifecycle:
      cache_size: {}
"#, size);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", size));
            assert!(workflow.workflow_execution_strategy.is_some());
        }
    }

    #[test]
    fn parse_permission_modes() {
        let modes = [
            "owner_full",
            "owner_read_only",
            "owner_full_group_read",
            "owner_full_group_read_exec_other_read_exec",
        ];
        for mode in modes {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
workspace:
  permissions:
    default_mode: {}
"#, mode);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", mode));
            assert!(workflow.workspace.is_some());
        }
    }

    #[test]
    fn parse_checkpoint_levels() {
        let levels = ["info", "debug", "git", "timetravel", "compressed_summary"];
        for level in levels {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
workflow_execution_strategy:
  checkpointing:
    configuration:
      level: {}
"#, level);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", level));
            assert!(workflow.workflow_execution_strategy.is_some());
        }
    }

    #[test]
    fn parse_dependency_strategies() {
        let strategies = ["wait_for_all", "continue_with_available", "skip_failed"];
        for strategy in strategies {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
workflow_execution_strategy:
  dependency_resolution:
    strategy: {}
"#, strategy);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", strategy));
            assert!(workflow.workflow_execution_strategy.is_some());
        }
    }

    #[test]
    fn parse_sub_workflow_ref() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
sub_workflows:
  external_ref: "./workflows/external.yml"
  inline_ref:
    inputs:
      test_input:
        type: string
    steps:
      test-step:
        prompt: "test"
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert!(workflow.sub_workflows.is_some());
        let sub_workflows = workflow.sub_workflows.unwrap();
        assert_eq!(sub_workflows.len(), 2);
    }

    #[test]
    fn parse_gwt_clauses() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test-step:
      when:
        after_step_succeeds:
          gwt:
            - given: "result.success == true"
              then: { route_to: success_step }
            - given: "result.success == false"
              then: { route_to: fail_step }
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn parse_execution_modes() {
        let modes = ["interactive", "automated", "hybrid"];
        for mode in modes {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  user_inputs:
    execution_mode: {}
"#, mode);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", mode));
            assert!(workflow.agentic_workflow.is_some());
        }
    }

    #[test]
    fn parse_loop_config() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test-step:
      loop:
        count:
          max_iterations: 10
          iteration_variable: current_item
        validation:
          tolerance: 0.05
          max_iterations: 5
          exact_criteria:
            - metric: quality
              operator: ">="
              target: 0.9
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn parse_requires_conditions() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test-step:
      requires:
        - step: previous-step
          condition: "result.success == true"
        - step: another-step
          condition: "result.value > 100"
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn parse_model_overrides() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test-step:
      model_overrides:
        max_turns: 20
        temperature: 0.7
        top_p: 0.9
        top_k: 50
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn parse_pressure_strategies() {
        let strategies = ["throttle", "swap", "fail_fast", "graceful_degradation"];
        for strategy in strategies {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
workflow_execution_strategy:
  memory:
    pressure_handling:
      strategy: {}
"#, strategy);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", strategy));
            assert!(workflow.workflow_execution_strategy.is_some());
        }
    }

    #[test]
    fn parse_timeout_strategies() {
        let strategies = ["fail", "continue_with_partial", "retry"];
        for strategy in strategies {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
workflow_execution_strategy:
  timeout:
    timeout_strategy: {}
"#, strategy);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", strategy));
            assert!(workflow.workflow_execution_strategy.is_some());
        }
    }

    #[test]
    fn parse_backoff_strategies() {
        let strategies = ["exponential", "linear", "fixed"];
        for strategy in strategies {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  retry:
    backoff: {}
"#, strategy);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", strategy));
            assert!(workflow.agentic_workflow.is_some());
        }
    }

    #[test]
    fn parse_rag_formats() {
        let formats = ["vector_db", "file_based", "memory_based"];
        for format in formats {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
memory:
  rag:
    knowledge_base:
      format: {}
"#, format);
            let workflow: WorkflowFile = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", format));
            assert!(workflow.memory.is_some());
        }
    }

    #[test]
    fn validate_accepts_llama_cpp_with_vulkan_provider() {
        let yaml = r#"
workflow_id: test-benchmark
name: "Test Benchmark"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        workflow.validate().expect("validation should pass for llama_cpp_with_vulkan");
    }

    #[test]
    fn validate_rejects_lmstudio_provider() {
        let yaml = r#"
workflow_id: test-benchmark
name: "Test Benchmark"
providers:
  lmstudio:
    config:
      host: localhost
      port: 1234
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        let result = workflow.validate();
        assert!(result.is_err(), "should reject lmstudio provider");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("unsupported provider 'lmstudio'"),
            "error should mention unsupported provider, got: {}", err_msg
        );
    }

    #[test]
    fn validate_rejects_ollama_provider() {
        let yaml = r#"
workflow_id: test-benchmark
name: "Test Benchmark"
providers:
  ollama:
    config:
      host: localhost
      port: 11434
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        let result = workflow.validate();
        assert!(result.is_err(), "should reject ollama provider");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("unsupported provider 'ollama'"),
            "error should mention unsupported provider, got: {}", err_msg
        );
    }

    #[test]
    fn validate_rejects_lmstudio_model_host_type() {
        let yaml = r#"
workflow_id: test-benchmark
name: "Test Benchmark"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: lmstudio
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        let result = workflow.validate();
        assert!(result.is_err(), "should reject lmstudio host.type");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("unsupported host.type 'lmstudio'"),
            "error should mention unsupported host.type, got: {}", err_msg
        );
    }

    #[test]
    fn validate_rejects_empty_workflow_id() {
        let yaml = r#"
workflow_id: ""
name: "Test"
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        let result = workflow.validate();
        assert!(result.is_err(), "should reject empty workflow_id");
    }

    #[test]
    fn validate_rejects_empty_name() {
        let yaml = r#"
workflow_id: test
name: ""
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        let result = workflow.validate();
        assert!(result.is_err(), "should reject empty name");
    }

    #[test]
    fn validate_raw_keys_rejects_benchmark_section() {
        let yaml = r#"
workflow_id: test
name: "Test"
benchmark:
  compare_modes: true
  max_tokens: 128
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject unknown 'benchmark' key");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("unknown top-level key") && err_msg.contains("benchmark"),
            "error should mention benchmark key, got: {}", err_msg
        );
    }

    #[test]
    fn validate_raw_keys_rejects_model_list() {
        let yaml = r#"
workflow_id: test
name: "Test"
model_list:
  - /path/to/model.gguf
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject unknown 'model_list' key");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("model_list"),
            "error should mention model_list, got: {}", err_msg
        );
    }

    #[test]
    fn validate_raw_keys_rejects_multiple_unknown_keys() {
        let yaml = r#"
workflow_id: test
name: "Test"
benchmark:
  max_tokens: 128
model_list:
  - /path/to/model.gguf
logging:
  level: debug
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject multiple unknown keys");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("benchmark") && err_msg.contains("model_list") && err_msg.contains("logging"),
            "error should list all unknown keys, got: {}", err_msg
        );
    }

    #[test]
    fn validate_raw_keys_accepts_all_schema_keys() {
        let yaml = r#"
workflow_id: test
name: "Test"
description: "desc"
version: "1.0"
author: "test"
tags: [test]
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
workflow_execution_strategy:
  load_unload: one_at_a_time
schema_version: "2.0.0"
min_schema_version: "2.0.0"
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_ok(), "all schema keys should be accepted");
    }

    #[test]
    fn validate_raw_keys_rejects_execution_section() {
        let yaml = r#"
workflow_id: test
name: "Test"
execution:
  mode: serial
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject unknown 'execution' key");
    }

    #[test]
    fn from_file_validates_raw_keys() {
        let yaml = r#"
workflow_id: test
name: "Test"
benchmark:
  max_tokens: 128
"#;
        let dir = std::env::temp_dir().join("whitt_test_validate");
        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("test_workflow.yml");
        std::fs::write(&file_path, yaml).unwrap();
        let result = WorkflowFile::from_file(&file_path);
        assert!(result.is_err(), "from_file should also validate raw keys");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("benchmark"), "error should mention benchmark, got: {}", err_msg);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
