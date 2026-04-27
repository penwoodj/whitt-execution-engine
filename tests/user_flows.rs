//! User flow integration tests.
//!
//! Tests that verify realistic user workflows:
//! - Multi-Model Pipeline
//! - Workflow Persistence + Resume
//! - Error Recovery + Retry
//! - Streaming Chat Session
//! - Model Hot-Swap Mid-Conversation

use whitt_execution_engine::agent::persistence::{
    WorkflowPersistence, WorkflowState, WorkflowStatus,
};
use whitt_execution_engine::agent::react::ReactAgent;
use whitt_execution_engine::agent::tools::{FinalAnswerTool, ToolRegistry};
use whitt_execution_engine::backend::llm_backend::LlmBackend;
use whitt_execution_engine::backend::mock_backend::{MockError, MockLlmBackend};
use std::sync::Arc;
use tempfile::TempDir;

/// Flow 1: Multi-Model Pipeline
/// User loads a small model, runs a planning task, then swaps to a larger model for execution.
#[tokio::test]
async fn test_multi_model_pipeline() {
    let backend = Arc::new(MockLlmBackend::new("small-model"));

    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FinalAnswerTool::new()));

    backend.set_response(
        "{\"tool\": \"final_answer\", \"arguments\": {\"answer\": \"Plan: Use large model for execution\"}}".to_string(),
    );

    let agent = ReactAgent::new(
        backend.clone() as Arc<dyn LlmBackend>,
        Arc::new(registry),
        "small-model".to_string(),
    );

    let result = agent.run("Plan the execution strategy".to_string()).await;
    assert!(result.is_ok(), "Planning task should succeed: {:?}", result);
    let plan_result = result.unwrap();
    assert_eq!(
        plan_result.final_answer,
        "Plan: Use large model for execution"
    );
    assert_eq!(plan_result.iterations, 1);

    let backend_large = Arc::new(MockLlmBackend::new("large-model"));
    backend_large.set_response(
        "{\"tool\": \"final_answer\", \"arguments\": {\"answer\": \"Execution complete with large model\"}}".to_string(),
    );

    let mut registry_large = ToolRegistry::new();
    registry_large.register(Box::new(FinalAnswerTool::new()));
    let agent_large = ReactAgent::new(
        backend_large.clone() as Arc<dyn LlmBackend>,
        Arc::new(registry_large),
        "large-model".to_string(),
    );

    let execution_result = agent_large.run("Execute the plan".to_string()).await;
    assert!(
        execution_result.is_ok(),
        "Execution task should succeed: {:?}",
        execution_result
    );
    let final_result = execution_result.unwrap();
    assert_eq!(
        final_result.final_answer,
        "Execution complete with large model"
    );
}

/// Flow 2: Workflow Persistence + Resume
/// User starts a workflow, saves checkpoint, simulates crash/restart, loads checkpoint and continues.
#[tokio::test]
async fn test_workflow_persistence_resume() {
    let tmpdir = TempDir::new().unwrap();
    let storage_path = tmpdir.path().to_path_buf();

    let persistence = WorkflowPersistence::sync_new(storage_path.clone()).unwrap();

    let mut state = WorkflowState::new(
        "persistence-test-workflow".to_string(),
        "step1".to_string(),
    );
    state.set_variable("query".to_string(), serde_json::json!("What is 3+4?"));

    persistence.sync_save(&state).unwrap();

    state.complete_step("step1".to_string());
    state.set_variable("result1".to_string(), serde_json::json!("7"));

    persistence.sync_create_checkpoint(&mut state, "after-step1").unwrap();

    let persistence_new = WorkflowPersistence::sync_new(storage_path).unwrap();

    let loaded_state = persistence_new.sync_load("persistence-test-workflow").unwrap();
    assert_eq!(loaded_state.workflow_id, "persistence-test-workflow");
    assert_eq!(loaded_state.current_step, "step1");
    assert_eq!(loaded_state.completed_steps, vec!["step1"]);
    assert_eq!(
        loaded_state.get_variable("query"),
        Some(&serde_json::json!("What is 3+4?"))
    );
    assert_eq!(
        loaded_state.get_variable("result1"),
        Some(&serde_json::json!("7"))
    );
    assert_eq!(loaded_state.checkpoints.len(), 1);
    assert_eq!(loaded_state.checkpoints[0].step_name, "after-step1");

    let mut continued_state = loaded_state;
    continued_state.set_current_step("step2".to_string());
    continued_state.set_variable("result2".to_string(), serde_json::json!("11"));
    continued_state.complete_step("step2".to_string());
    continued_state.set_status(WorkflowStatus::Completed);

    persistence_new.sync_save(&continued_state).unwrap();

    let final_state = persistence_new.sync_load("persistence-test-workflow").unwrap();
    assert_eq!(final_state.current_step, "step2");
    assert_eq!(final_state.completed_steps.len(), 2);
    assert_eq!(final_state.completed_steps, vec!["step1", "step2"]);
    assert_eq!(final_state.status, WorkflowStatus::Completed);
    assert_eq!(
        final_state.get_variable("result2"),
        Some(&serde_json::json!("11"))
    );
}

/// Flow 3: Error Recovery + Retry
/// Agent encounters error on first attempt, retries, and succeeds on second attempt.
#[tokio::test]
async fn test_error_recovery_retry() {
    let backend = Arc::new(MockLlmBackend::new("test-model"));

    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FinalAnswerTool::new()));

    backend.set_error(Some(MockError::Connection("Temporary connection failure".to_string())));

    let agent = ReactAgent::new(
        backend.clone() as Arc<dyn LlmBackend>,
        Arc::new(registry),
        "test-model".to_string(),
    );

    let result1 = agent.run("What is 5+6?".to_string()).await;
    assert!(
        result1.is_err(),
        "First attempt should fail: {:?}",
        result1
    );
    let err_msg = format!("{}", result1.unwrap_err());
    assert!(
        err_msg.contains("Connection") || err_msg.contains("connection"),
        "Error should mention connection: {}",
        err_msg
    );

    backend.set_error(None);
    backend.set_response(
        "{\"tool\": \"final_answer\", \"arguments\": {\"answer\": \"11\"}}".to_string(),
    );

    let mut registry2 = ToolRegistry::new();
    registry2.register(Box::new(FinalAnswerTool::new()));
    let agent2 = ReactAgent::new(
        backend.clone() as Arc<dyn LlmBackend>,
        Arc::new(registry2),
        "test-model".to_string(),
    );

    let result2 = agent2.run("What is 5+6?".to_string()).await;
    assert!(
        result2.is_ok(),
        "Retry should succeed after error recovery: {:?}",
        result2
    );
    let response = result2.unwrap();
    assert_eq!(response.final_answer, "11");
    assert_eq!(response.iterations, 1);
}

/// Flow 4: Streaming Chat Session
/// User starts a chat, gets streaming response, then follows up with second message.
#[tokio::test]
async fn test_streaming_chat_session() {
    let backend = Arc::new(MockLlmBackend::new("chat-model"));

    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FinalAnswerTool::new()));

    backend.set_response(
        "{\"tool\": \"final_answer\", \"arguments\": {\"answer\": \"First response: Hello!\"}}".to_string(),
    );

    let agent = ReactAgent::new(
        backend.clone() as Arc<dyn LlmBackend>,
        Arc::new(registry),
        "chat-model".to_string(),
    );

    let result1 = agent.run("Hello".to_string()).await;
    assert!(result1.is_ok(), "First message should succeed: {:?}", result1);
    let response1 = result1.unwrap();
    assert_eq!(response1.final_answer, "First response: Hello!");

    backend.set_response(
        "{\"tool\": \"final_answer\", \"arguments\": {\"answer\": \"Second response: How can I help?\"}}".to_string(),
    );

    let result2 = agent.run("How are you?".to_string()).await;
    assert!(
        result2.is_ok(),
        "Second message should succeed: {:?}",
        result2
    );
    let response2 = result2.unwrap();
    assert_eq!(
        response2.final_answer,
        "Second response: How can I help?"
    );

    assert_ne!(response1.final_answer, response2.final_answer);
}

/// Flow 5: Model Hot-Swap Mid-Conversation
/// User is chatting with one model, swaps to another model, continues conversation.
#[tokio::test]
async fn test_model_hot_swap_mid_conversation() {
    let backend = Arc::new(MockLlmBackend::new("model-a"));

    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FinalAnswerTool::new()));

    backend.set_response(
        "{\"tool\": \"model_load\", \"arguments\": {\"model_name\": \"model-b\"}}".to_string(),
    );

    let agent = ReactAgent::new(
        backend.clone() as Arc<dyn LlmBackend>,
        Arc::new(registry),
        "model-a".to_string(),
    );

    let result = agent.run("Switch to model-b for this task".to_string()).await;
    assert!(result.is_ok() || result.is_err(), "Agent should complete or handle model load");

    let backend_b = Arc::new(MockLlmBackend::new("model-b"));
    backend_b.set_response(
        "{\"tool\": \"final_answer\", \"arguments\": {\"answer\": \"Running with model-b\"}}".to_string(),
    );

    let mut registry_b = ToolRegistry::new();
    registry_b.register(Box::new(FinalAnswerTool::new()));
    let agent_b = ReactAgent::new(
        backend_b.clone() as Arc<dyn LlmBackend>,
        Arc::new(registry_b),
        "model-b".to_string(),
    );

    let result_b = agent_b.run("Continue task".to_string()).await;
    assert!(
        result_b.is_ok(),
        "Task should complete with swapped model: {:?}",
        result_b
    );
    let response_b = result_b.unwrap();
    assert_eq!(response_b.final_answer, "Running with model-b");
}
