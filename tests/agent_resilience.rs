//! Agent resilience tests.
//!
//! Tests that verify the agent's behavior under adverse conditions:
//! - Timeout handling
//! - Rate limiting
//! - Connection errors
//! - Malformed responses
//! - Slow streaming
//! - Connection drops mid-stream
//! - Exhausted retries
//! - Recovery after temporary failures

use whitt_execution_engine::agent::react::ReactAgent;
use whitt_execution_engine::agent::tools::{FinalAnswerTool, ToolRegistry};
use whitt_execution_engine::backend::llm_backend::LlmError;
use whitt_execution_engine::backend::mock_backend::{MockError, MockLlmBackend};
use std::sync::Arc;
use std::time::Duration;

fn create_agent_with_mock() -> (Arc<MockLlmBackend>, ReactAgent) {
    let backend = Arc::new(MockLlmBackend::new("test-model"));
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FinalAnswerTool::new()));
    let agent = ReactAgent::new(
        backend.clone() as Arc<dyn whitt_execution_engine::backend::llm_backend::LlmBackend>,
        Arc::new(registry),
        "test-model".to_string(),
    );
    (backend, agent)
}

#[tokio::test]
async fn test_timeout_handling() {
    let (backend, agent) = create_agent_with_mock();
    backend.set_error(Some(MockError::Timeout("Request timed out".to_string())));

    let result = agent.run("What is 2+2?".to_string()).await;
    assert!(result.is_err(), "Agent should return error on timeout");
    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("timed out") || err_msg.contains("Timeout"),
        "Error should mention timeout: {}",
        err_msg
    );
}

#[tokio::test]
async fn test_rate_limit_error() {
    let (backend, agent) = create_agent_with_mock();
    backend.set_error(Some(MockError::RateLimited("Rate limit exceeded".to_string())));

    let result = agent.run("What is 2+2?".to_string()).await;
    assert!(result.is_err(), "Agent should return error on rate limit");
    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("Rate limit") || err_msg.contains("rate"),
        "Error should mention rate limit: {}",
        err_msg
    );
}

#[tokio::test]
async fn test_connection_error() {
    let (backend, agent) = create_agent_with_mock();
    backend.set_error(Some(MockError::Connection("Connection refused".to_string())));

    let result = agent.run("What is 2+2?".to_string()).await;
    assert!(result.is_err(), "Agent should return error on connection failure");
    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("Connection") || err_msg.contains("connection"),
        "Error should mention connection: {}",
        err_msg
    );
}

#[tokio::test]
async fn test_malformed_response_handling() {
    let (backend, agent) = create_agent_with_mock();
    // Set a response that is not valid JSON for a tool call, but not a final answer either.
    // The agent should treat it as a direct response.
    backend.set_response("I don't know the answer to that.".to_string());

    let result = agent.run("What is 2+2?".to_string()).await;
    assert!(result.is_ok(), "Agent should handle non-tool response gracefully");
    let response = result.unwrap();
    assert_eq!(
        response.final_answer, "I don't know the answer to that.",
        "Agent should return the direct response as final answer"
    );
    assert_eq!(response.iterations, 1, "Should complete in 1 iteration");
}

#[tokio::test]
async fn test_slow_streaming() {
    let backend = Arc::new(MockLlmBackend::new("test-model"));
    backend.set_delay(Duration::from_millis(100));
    backend.set_response("Slow streamed response".to_string());

    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FinalAnswerTool::new()));
    let agent = ReactAgent::new(
        backend.clone() as Arc<dyn whitt_execution_engine::backend::llm_backend::LlmBackend>,
        Arc::new(registry),
        "test-model".to_string(),
    );

    let start = std::time::Instant::now();
    let result = agent.run("Stream something slowly".to_string()).await;
    let elapsed = start.elapsed();

    assert!(result.is_ok(), "Slow streaming should succeed");
    assert!(
        elapsed >= Duration::from_millis(100),
        "Should take at least the configured delay: {:?}",
        elapsed
    );
}

#[tokio::test]
async fn test_connection_drop_mid_stream() {
    use whitt_execution_engine::backend::llm_backend::LlmBackend;
    use futures::StreamExt;

    let backend = Arc::new(MockLlmBackend::new("test-model"));
    backend.set_error(Some(MockError::Connection("Connection dropped".to_string())));

    let dyn_backend: Arc<dyn LlmBackend> = backend;
    let stream_result = dyn_backend.chat_stream(vec![], "test-model").await;
    assert!(stream_result.is_ok(), "Stream creation should succeed");

    let mut stream = stream_result.unwrap();
    let first_chunk: Option<Result<String, LlmError>> = stream.next().await;
    assert!(first_chunk.is_some(), "Should get first chunk");
    assert!(
        first_chunk.unwrap().is_err(),
        "First chunk should be an error due to connection drop"
    );
}

#[tokio::test]
async fn test_multiple_retries_exhaust() {
    let (backend, agent) = create_agent_with_mock();
    // Always return an error — agent should exhaust attempts and fail
    backend.set_error(Some(MockError::Internal("Permanent failure".to_string())));

    let result = agent.run("Try this".to_string()).await;
    assert!(
        result.is_err(),
        "Agent should fail when backend always errors"
    );
}

#[tokio::test]
async fn test_recovery_after_temporary_failure() {
    // First attempt fails
    let backend = Arc::new(MockLlmBackend::new("test-model"));
    backend.set_error(Some(MockError::Connection("Temporary failure".to_string())));

    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FinalAnswerTool::new()));
    let agent1 = ReactAgent::new(
        backend.clone() as Arc<dyn whitt_execution_engine::backend::llm_backend::LlmBackend>,
        Arc::new(registry),
        "test-model".to_string(),
    );

    let result1 = agent1.run("First query".to_string()).await;
    assert!(result1.is_err(), "First attempt should fail");

    // Clear error, set a valid response
    backend.set_error(None);
    backend.set_response("{\"tool\": \"final_answer\", \"arguments\": {\"answer\": \"Recovered!\"}}".to_string());

    let mut registry2 = ToolRegistry::new();
    registry2.register(Box::new(FinalAnswerTool::new()));
    let agent2 = ReactAgent::new(
        backend.clone() as Arc<dyn whitt_execution_engine::backend::llm_backend::LlmBackend>,
        Arc::new(registry2),
        "test-model".to_string(),
    );

    let result2 = agent2.run("Second query".to_string()).await;
    assert!(result2.is_ok(), "Second attempt should succeed after recovery");
    let response = result2.unwrap();
    assert_eq!(response.final_answer, "Recovered!");
}
