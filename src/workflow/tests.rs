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
    fn parse_resource_admission_contract() {
        let yaml = r#"
workflow_id: resource-admission
name: "Resource Admission"
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 6GiB
      vram: 6GiB
      swap_free: 4GiB
    model_estimate:
      kv_cache: 2.1GiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 900
    telemetry:
      write_profile: true
"#;
        let workflow: WorkflowFile = serde_saphyr::from_str(yaml).expect("parse");
        let admission = workflow
            .workflow_execution_strategy
            .expect("execution strategy")
            .resource_admission
            .expect("resource admission");
        assert_eq!(admission.minimum_available.ram, "6GiB");
        assert_eq!(admission.minimum_available.vram, "6GiB");
        assert_eq!(admission.minimum_available.swap_free, "4GiB");
        assert_eq!(admission.model_estimate.kv_cache, "2.1GiB");
        assert_eq!(admission.model_estimate.expected_runtime_secs, 900);
        assert!(admission.telemetry.write_profile);
    }

    #[test]
    fn validate_rejects_invalid_resource_admission_values() {
        for (field, value) in [("ram", "0GiB"), ("vram", "6G"), ("expected_runtime_secs", "0")] {
            let minimum_available = match field {
                "ram" => format!("ram: {value}\n      vram: 6GiB\n      swap_free: 4GiB"),
                "vram" => format!("ram: 6GiB\n      vram: {value}\n      swap_free: 4GiB"),
                _ => "ram: 6GiB\n      vram: 6GiB\n      swap_free: 4GiB".to_string(),
            };
            let runtime = if field == "expected_runtime_secs" { value } else { "900" };
            let yaml = format!(r#"
workflow_id: invalid-resource-admission
name: "Invalid Resource Admission"
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      {minimum_available}
    model_estimate:
      kv_cache: 2.1GiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: {runtime}
    telemetry:
      write_profile: true
"#);
            assert!(
                WorkflowFile::from_yaml(&yaml).is_err(),
                "{field}={value} must be rejected"
            );
        }
    }

    #[test]
    fn validate_rejects_resource_admission_model_without_source_path() {
        let yaml = r#"
workflow_id: missing-resource-source
name: "Missing Resource Source"
models:
  worker:
    name: "model-a.gguf"
workflow_execution_strategy:
  resource_admission:
    enforcement_policy: block
    minimum_available:
      ram: 6GiB
      vram: 6GiB
      swap_free: 4GiB
    model_estimate:
      kv_cache: 288MiB
      compute_buffer: 512MiB
      host_runtime: 700MiB
      expected_runtime_secs: 60
    telemetry:
      write_profile: true
"#;

        assert!(
            WorkflowFile::from_yaml(yaml).is_err(),
            "resource admission must require a host-stattable model source"
        );
    }

    #[test]
    fn parse_log_levels() {
        let levels = ["debug", "info", "warning", "error", "critical"];
        for level in levels {
            let yaml = format!(r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  when:
    after_step_fails:
      - log:
          level: {}
          event_fields: [step_name]
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
    backoff:
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
  when:
    after_step_succeeds:
      - gwt:
          - given: "result.success == true"
            then: success_step
          - given: "result.success == false"
            then: fail_step
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
    backoff:
      backoff: {}
      initial_delay: "1s"
      max_delay: "30s"
      multiplier: 2.0
      jitter: true
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

    #[test]
    fn validate_nested_rejects_redundant_connection_settings() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
      connection_settings:
        host: localhost
        port: "8080"
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject redundant connection_settings");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("connection_settings duplicates provider config"),
            "error should mention redundant config, got: {}", err_msg
        );
    }

    #[test]
    fn validate_nested_rejects_redundant_load_unload_strategy() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
workflow_execution_strategy:
  load_unload: one_at_a_time
  memory:
    model_lifecycle:
      load_unload_strategy: one_at_a_time
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject redundant load_unload_strategy");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("load_unload_strategy duplicates load_unload"),
            "error should mention redundant key, got: {}", err_msg
        );
    }

    #[test]
    fn validate_nested_accepts_valid_yaml_no_false_positives() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
models:
  primary:
    host:
      type: llama_cpp_with_vulkan
      connection_settings:
        host: different-host
        port: "9090"
workflow_execution_strategy:
  load_unload: one_at_a_time
  memory:
    model_lifecycle:
      load_unload_strategy: lazy
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_ok(), "should accept valid YAML with no redundancies");
    }

    #[test]
    fn validate_workspace_benchmark_3_models() {
        use std::path::Path;
        let path = Path::new("docs/benchmarks/workflows/benchmark-3-models.yml");
        let result = WorkflowFile::from_file(path);
        assert!(result.is_ok(), "benchmark-3-models.yml should be schema-compliant: {:?}", result.err());
    }

    #[test]
    fn validate_workspace_benchmark_5_models() {
        use std::path::Path;
        let path = Path::new("docs/benchmarks/workflows/benchmark-5-models.yml");
        let result = WorkflowFile::from_file(path);
        assert!(result.is_ok(), "benchmark-5-models.yml should be schema-compliant: {:?}", result.err());
    }

    #[test]
    fn validate_workspace_benchmark_15_models() {
        use std::path::Path;
        let path = Path::new("docs/benchmarks/workflows/benchmark-15-models.yml");
        let result = WorkflowFile::from_file(path);
        assert!(result.is_ok(), "benchmark-15-models.yml should be schema-compliant: {:?}", result.err());
    }

    #[test]
    fn validate_workspace_benchmark_50_models() {
        use std::path::Path;
        let path = Path::new("docs/benchmarks/workflows/benchmark-50-models.yml");
        let result = WorkflowFile::from_file(path);
        assert!(result.is_ok(), "benchmark-50-models.yml should be schema-compliant: {:?}", result.err());
    }

    #[test]
    fn resource_admission_workspace_fixtures_parse() {
        for path in [
            "docs/benchmarks/workflows/resource-admission-reject.yml",
            "docs/benchmarks/workflows/resource-admission-simple.yml",
            "docs/benchmarks/workflows/resource-admission-long.yml",
        ] {
            let result = WorkflowFile::from_file(std::path::Path::new(path));
            assert!(result.is_ok(), "{path} should be schema-compliant: {:?}", result.err());
        }
    }

    // ── Exhaustive validation tests for WorkflowFile::from_yaml ──────────────

    #[test]
    fn require_condition_string_shorthand() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  step_b:
    requires: [step_a]
    prompt: "do something"
"#;
        let workflow = WorkflowFile::from_yaml(yaml).expect("shorthand requires should parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn require_condition_mixed_shorthand_and_object() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  step_c:
    requires:
      - step_a
      - step: step_b
        condition: "result.success == true"
    prompt: "do something"
"#;
        let workflow = WorkflowFile::from_yaml(yaml).expect("mixed requires should parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn hook_action_append_to_string() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  when:
    after_step_succeeds:
      - append_to: "./workspace/output/results.yaml"
"#;
        let workflow = WorkflowFile::from_yaml(yaml).expect("append_to string should parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn hook_action_append_to_list() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  when:
    after_step_succeeds:
      - append_to:
          - "./workspace/output/results.yaml"
          - result_collection
"#;
        let workflow = WorkflowFile::from_yaml(yaml).expect("append_to list should parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn hook_action_route_to() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  when:
    after_step_succeeds:
      - gwt:
          - given: "quality_score >= 0.9"
            then: success_step
          - given: "true"
            then: [fail_step, cleanup_step]
"#;
        let workflow = WorkflowFile::from_yaml(yaml).expect("route_to list should parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn hook_action_fail_and_skip() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  when:
    after_step_fails:
      - fail:
          message: "Critical error occurred"
      - skip_step: true
    after_all_retries_exhausted:
      - skip_remaining: true
"#;
        let workflow = WorkflowFile::from_yaml(yaml).expect("fail/skip actions should parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn step_with_on_requires_failed() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  step_b:
    requires: [step_a]
    on_requires_failed:
      log_failure:
        - log:
            to_file_path: "./workspace/logs/dep_fail.log"
            event_fields: [failed_step, reason]
    prompt: "do something"
"#;
        let workflow = WorkflowFile::from_yaml(yaml).expect("on_requires_failed should parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn step_retry_config() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test_step:
      prompt: "do something"
      retry:
        max_attempts: 5
        backoff: exponential
        initial_delay: "1s"
        max_delay: "30s"
        multiplier: 2.0
        jitter: true
        level: step_restart
        adjustment_strategy: loosen_tolerance
        tolerance_adjustment: 0.05
        checkpoint_after_retry: true
"#;
        let workflow = WorkflowFile::from_yaml(yaml).expect("step retry config should parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn full_workflow_pipeline_validation() {
        let yaml = r#"
workflow_id: full-pipeline-test
name: "Full Pipeline Test"
description: "Tests complete validation pipeline"
version: "1.0.0"
author: "Test Author"
tags: [test, validation]
schema_version: "2.0.0"
min_schema_version: "2.0.0"
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
  memory:
    ram_allocation:
      strategy: dynamic
agentic_workflow:
  when:
    after_step_fails:
      - log:
          to_file_path: "./logs/errors.log"
          event_fields: [step_name, error_message]
          level: error
  steps:
    step_one:
      generative_entity: "${models.primary}"
      prompt: "Analyze this code"
      model_overrides:
        max_turns: 10
        temperature: 0.7
      when:
        before_step_starts:
          - log:
              to_file_path: "./logs/steps.log"
              event_fields: [step_name]
        after_step_succeeds:
          - append_to: "./output/analysis.yaml"
    step_two:
      requires: [step_one]
      tool: file_read
      input:
        file_path: "./config.yml"
      when:
        after_step_succeeds:
          - save_to: config_data
    step_three:
      requires:
        - step_one
        - step: step_two
          condition: "result.success == true"
      prompt: "Generate report from {{step.step_two.output}}"
      loop:
        validation:
          tolerance: 0.05
          max_iterations: 5
          exact_criteria:
            - metric: quality
              operator: ">="
              target: 0.9
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_ok(), "full pipeline should validate: {:?}", result.err());
        let workflow = result.unwrap();
        assert_eq!(workflow.workflow_id, "full-pipeline-test");
        assert!(workflow.providers.is_some());
        assert!(workflow.models.is_some());
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn deny_unknown_fields_on_workflow_step() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test_step:
      prompt: "do something"
      unknown_field: "should fail"
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject unknown field on WorkflowStep");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("unknown_field"),
            "error should mention unknown field, got: {}", err_msg
        );
    }

    #[test]
    fn deny_unknown_fields_on_model_overrides() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test_step:
      model_overrides:
        max_turns: 10
        unknown_param: 42
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject unknown field on ModelOverrides");
    }

    #[test]
    fn deny_unknown_fields_on_loop_config() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test_step:
      loop:
        unknown_loop_field: true
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject unknown field on LoopConfig");
    }

    #[test]
    fn deny_unknown_fields_on_count_loop_config() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test_step:
      loop:
        count:
          max_iterations: 10
          unknown_count_field: true
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject unknown field on CountLoopConfig");
    }

    #[test]
    fn deny_unknown_fields_on_step_retry_config() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test_step:
      retry:
        max_attempts: 3
        unknown_retry_field: true
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject unknown field on StepRetryConfig");
    }

    #[test]
    fn deny_unknown_fields_on_exact_criteria() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test Workflow"
agentic_workflow:
  steps:
    test_step:
      loop:
        validation:
          exact_criteria:
            - metric: quality
              operator: ">="
              target: 0.9
              unknown_criteria: true
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err(), "should reject unknown field on ExactCriteria");
    }

    #[test]
    fn from_yaml_rejects_unknown_top_level_and_validates_nested() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test"
benchmark:
  mode: gpu
logging:
  level: debug
"#;
        let result = WorkflowFile::from_yaml(yaml);
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("benchmark") && err_msg.contains("logging"),
            "should report both unknown keys, got: {}", err_msg
        );
    }

    #[test]
    fn tool_step_with_input_json() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test"
agentic_workflow:
  run_tests:
    tool: shell_exec
    input:
      command: "cargo test"
      timeout_seconds: 120
      env:
        RUST_BACKTRACE: "1"
"#;
        let workflow = WorkflowFile::from_yaml(yaml).expect("tool step with input should parse");
        assert!(workflow.agentic_workflow.is_some());
    }

    #[test]
    fn user_input_prompt_step() {
        let yaml = r#"
workflow_id: test-workflow
name: "Test"
agentic_workflow:
  confirm_step:
    user_input:
      type: confirm
      message: "Proceed with deployment?"
      default: true
"#;
        let workflow = WorkflowFile::from_yaml(yaml).expect("user_input prompt should parse");
        assert!(workflow.agentic_workflow.is_some());
    }
}
