//! End-to-end integration tests.
//!
//! Tests that exercise the full pipeline:
//! - Unified YAML → config → provider → model → agent → output
//! - Model lifecycle management through agent tools
//! - Workflow persistence across agent execution

use whitt_execution_engine::agent::persistence::{WorkflowPersistence, WorkflowState, WorkflowStatus};
use whitt_execution_engine::agent::react::ReactAgent;
use whitt_execution_engine::agent::tools::{
    ChatTool, FinalAnswerTool, ModelListTool, ModelLoadTool, ToolRegistry,
};
use whitt_execution_engine::backend::llm_backend::LlmBackend;
use whitt_execution_engine::backend::mock_backend::MockLlmBackend;
use whitt_execution_engine::config::unified::UnifiedConfig;
use whitt_execution_engine::model::registry::{ModelLifecycle, ThreadSafeModelRegistry};
use std::sync::{Arc, Mutex};
use tempfile::TempDir;

#[tokio::test]
async fn test_full_agent_pipeline() {
    // Create mock backend configured to return a final answer via tool call
    let backend = Arc::new(MockLlmBackend::new("test-model"));
    backend.set_response(
        "{\"tool\": \"final_answer\", \"arguments\": {\"answer\": \"42\"}}".to_string(),
    );

    // Create tool registry with FinalAnswerTool
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FinalAnswerTool::new()));

    // Create agent
    let agent = ReactAgent::new(
        backend.clone() as Arc<dyn LlmBackend>,
        Arc::new(registry),
        "test-model".to_string(),
    );

    // Run agent
    let result = agent.run("What is the meaning of life?".to_string()).await;

    assert!(result.is_ok(), "Agent should succeed: {:?}", result);
    let response = result.unwrap();
    assert_eq!(response.final_answer, "42");
    assert_eq!(response.iterations, 1);
    assert!(!response.tool_calls.is_empty());
    assert_eq!(response.tool_calls[0].name, "final_answer");
}

#[tokio::test]
async fn test_yaml_to_agent_pipeline() {
    // Create a minimal unified YAML config
    let yaml = r#"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: "http://localhost"
      port: 1234
models:
  global_config_path: "./configs/models"
  default_router: automatic
  test-model:
    name: "Test Model"
    host:
      type: llama_cpp_with_vulkan
"#;

    // Parse unified config
    let config = UnifiedConfig::from_yaml(yaml);
    assert!(config.is_ok(), "YAML should parse: {:?}", config);
    let config = config.unwrap();

    // Verify config resolution
    assert_eq!(config.schema_version, "2.0.0");
    assert!(
        config.providers.providers.contains_key("llama_cpp_with_vulkan"),
        "Should have llama_cpp_with_vulkan provider"
    );
    assert!(
        config.models.models.contains_key("test-model"),
        "Should have test-model"
    );

    // Resolve model config through the hierarchy
    let resolved = config.resolve_model_config("test-model", None);
    assert!(resolved.is_ok(), "Should resolve model config: {:?}", resolved);
    let resolved = resolved.unwrap();
    assert_eq!(resolved.host, "http://localhost");
    assert_eq!(resolved.port, 1234);

    // Create mock backend with the resolved model name
    let backend = Arc::new(MockLlmBackend::new("test-model"));
    backend.set_response(
        "{\"tool\": \"final_answer\", \"arguments\": {\"answer\": \"Pipeline works!\"}}".to_string(),
    );

    // Create agent and run
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FinalAnswerTool::new()));
    let agent = ReactAgent::new(
        backend.clone() as Arc<dyn LlmBackend>,
        Arc::new(registry),
        "test-model".to_string(),
    );

    let result = agent.run("Test query".to_string()).await;
    assert!(result.is_ok(), "Agent should succeed: {:?}", result);
    assert_eq!(result.unwrap().final_answer, "Pipeline works!");
}

#[tokio::test]
async fn test_model_lifecycle_with_agent() {
    // Create registry and backend
    let models_config = whitt_execution_engine::model::schema::ModelsConfig {
        global_config_path: "./configs/models".to_string(),
        default_router: "automatic".to_string(),
        models: {
            let mut map = std::collections::HashMap::new();
            map.insert(
                "test-model".to_string(),
                whitt_execution_engine::model::schema::ModelSpec::default(),
            );
            map
        },
    };
    let registry = Arc::new(Mutex::new(ThreadSafeModelRegistry::new(models_config)));

    let backend = Arc::new(MockLlmBackend::new("test-model"));

    // Register model tools
    let mut tool_registry = ToolRegistry::new();
    tool_registry.register(Box::new(ModelListTool::new(registry.clone())));
    tool_registry.register(Box::new(ModelLoadTool::new(
        registry.clone(),
        backend.clone() as Arc<dyn LlmBackend>,
    )));
    tool_registry.register(Box::new(ChatTool::new(
        backend.clone() as Arc<dyn LlmBackend>,
    )));
    tool_registry.register(Box::new(FinalAnswerTool::new()));

    // Configure backend to return tool calls in sequence
    // First call: load model, second call: chat, third call: final answer
    backend.set_response(
        "{\"tool\": \"model_load\", \"arguments\": {\"model_name\": \"test-model\"}}".to_string(),
    );

    // Verify initial state
    {
        let reg = registry.lock().unwrap();
        assert_eq!(
            reg.get_state("test-model"),
            Some(ModelLifecycle::Unloaded),
            "Model should start unloaded"
        );
    }

    // Create agent and run
    let agent = ReactAgent::new(
        backend.clone() as Arc<dyn LlmBackend>,
        Arc::new(tool_registry),
        "test-model".to_string(),
    )
    .with_max_iterations(1);

    let result = agent.run("Load the test model".to_string()).await;
    assert!(result.is_ok(), "Agent should succeed: {:?}", result);

    // Verify model state was updated to Loaded
    {
        let reg = registry.lock().unwrap();
        assert_eq!(
            reg.get_state("test-model"),
            Some(ModelLifecycle::Loaded),
            "Model should be loaded after agent calls model_load"
        );
    }
}

#[tokio::test]
async fn test_workflow_persistence_e2e() {
    let tmpdir = TempDir::new().unwrap();
    let storage_path = tmpdir.path().to_path_buf();

    // Create persistence layer
    let persistence = WorkflowPersistence::sync_new(storage_path).unwrap();

    // Create initial workflow state
    let mut state = WorkflowState::new(
        "e2e-workflow-001".to_string(),
        "step1".to_string(),
    );
    state.set_variable("query".to_string(), serde_json::json!("What is 2+2?"));

    // Save state
    persistence.sync_save(&state).unwrap();

    // Create mock backend and agent
    let backend = Arc::new(MockLlmBackend::new("test-model"));
    backend.set_response(
        "{\"tool\": \"final_answer\", \"arguments\": {\"answer\": \"4\"}}".to_string(),
    );

    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FinalAnswerTool::new()));
    let agent = ReactAgent::new(
        backend.clone() as Arc<dyn LlmBackend>,
        Arc::new(registry),
        "test-model".to_string(),
    );

    // Run agent
    let result = agent.run("What is 2+2?".to_string()).await;
    assert!(result.is_ok(), "Agent should succeed: {:?}", result);
    let response = result.unwrap();
    assert_eq!(response.final_answer, "4");

    // Update workflow state with result
    state.set_variable("result".to_string(), serde_json::json!("4"));
    state.complete_step("step1".to_string());
    state.set_current_step("step2".to_string());
    state.set_status(WorkflowStatus::Completed);

    // Save updated state
    persistence.sync_save(&state).unwrap();

    // Reload and verify
    let loaded = persistence.sync_load("e2e-workflow-001").unwrap();
    assert_eq!(loaded.workflow_id, "e2e-workflow-001");
    assert_eq!(loaded.status, WorkflowStatus::Completed);
    assert_eq!(loaded.completed_steps, vec!["step1"]);
    assert_eq!(loaded.current_step, "step2");
    assert_eq!(
        loaded.get_variable("result"),
        Some(&serde_json::json!("4"))
    );
    assert_eq!(
        loaded.get_variable("query"),
        Some(&serde_json::json!("What is 2+2?"))
    );
}
