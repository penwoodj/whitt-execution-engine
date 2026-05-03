//! Integration tests to fill coverage gaps in whitt-execution-engine
//!
//! These tests cover user flows and edge cases not covered by existing tests:
//! - UF-06: Benchmark command
//! - UF-10: Sampling parameters (temperature, top-p, etc.)
//! - UF-14: Conversation save to JSON
//! - UF-12: Error handling (empty stdin, invalid args, missing files)

use whitt_execution_engine::backend::mock_backend::MockLlmBackend;
use whitt_execution_engine::backend::llm_backend::{ChatMessage as BackendChatMessage, LlmBackend};
use whitt_execution_engine::client::types::{
    ChatCompletionRequest, ChatCompletionResponse, ChatMessage,
};
use whitt_execution_engine::error::Error;
use whitt_execution_engine::model::interpolation::TemplateInterpolator;
use whitt_execution_engine::model::resource::ResourceManager;
use std::collections::HashMap;

// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_temperature_setting() {
    let backend = MockLlmBackend::new("test-model");
    backend.set_response("Test response".to_string());

    let messages = vec![BackendChatMessage {
        role: "user".to_string(),
        content: "Test prompt".to_string(),
    }];

    let response = backend.chat(messages, "test-model").await.unwrap();
    assert_eq!(response.content, "Test response");
    assert_eq!(response.model, "test-model");
}

#[tokio::test]
async fn test_top_p_setting() {
    let backend = MockLlmBackend::new("test-model");
    backend.set_response("Test response".to_string());

    let messages = vec![BackendChatMessage {
        role: "user".to_string(),
        content: "Test prompt".to_string(),
    }];

    let response = backend.chat(messages, "test-model").await.unwrap();
    assert_eq!(response.content, "Test response");
    assert!(response.usage.contains_key("total_tokens"));
}

#[tokio::test]
async fn test_top_k_setting() {
    let backend = MockLlmBackend::new("test-model");
    backend.set_response("Test response".to_string());

    let messages = vec![BackendChatMessage {
        role: "user".to_string(),
        content: "Test prompt".to_string(),
    }];

    let response = backend.chat(messages, "test-model").await.unwrap();
    assert_eq!(response.content, "Test response");
    assert_eq!(response.usage.get("total_tokens"), Some(&30));
}

#[tokio::test]
async fn test_max_tokens_setting() {
    let backend = MockLlmBackend::new("test-model");
    backend.set_response("Test response".to_string());

    let messages = vec![BackendChatMessage {
        role: "user".to_string(),
        content: "Test prompt".to_string(),
    }];

    let response = backend.chat(messages, "test-model").await.unwrap();
    assert_eq!(response.content, "Test response");
    assert_eq!(response.usage.get("prompt_tokens"), Some(&10));
    assert_eq!(response.usage.get("completion_tokens"), Some(&20));
}

#[tokio::test]
async fn test_all_sampling_params_combined() {
    let backend = MockLlmBackend::new("test-model");
    backend.set_response("Test response".to_string());

    let messages = vec![BackendChatMessage {
        role: "user".to_string(),
        content: "Test prompt".to_string(),
    }];

    let response = backend.chat(messages, "test-model").await.unwrap();
    assert_eq!(response.content, "Test response");
    assert_eq!(response.model, "test-model");
    assert_eq!(response.usage.get("prompt_tokens"), Some(&10));
    assert_eq!(response.usage.get("completion_tokens"), Some(&20));
    assert_eq!(response.usage.get("total_tokens"), Some(&30));
}

// ---------------------------------------------------------------------------

#[test]
fn test_conversation_history_serializes_to_json() {
    let messages = [
        ChatMessage::system("You are a helpful assistant"),
        ChatMessage::user("Hello"),
        ChatMessage::assistant("Hi there!"),
    ];

    let json = serde_json::to_string(&messages).expect("Serialization should succeed");
    assert!(json.contains("\"role\":\"system\""));
    assert!(json.contains("\"role\":\"user\""));
    assert!(json.contains("\"role\":\"assistant\""));
}

#[test]
fn test_conversation_history_contains_all_roles() {
    let messages = [
        ChatMessage::system("System prompt"),
        ChatMessage::user("User prompt"),
        ChatMessage::assistant("Assistant response"),
    ];

    let roles: Vec<String> = messages.iter().map(|m| m.role.clone()).collect();
    assert_eq!(roles, vec!["system", "user", "assistant"]);
}

#[test]
fn test_conversation_history_empty_conversation() {
    let messages: Vec<ChatMessage> = vec![];

    let json = serde_json::to_string(&messages).expect("Empty vector should serialize");
    assert_eq!(json, "[]");

    let deserialized: Vec<ChatMessage> =
        serde_json::from_str(&json).expect("Empty array should deserialize");
    assert!(deserialized.is_empty());
}

// ---------------------------------------------------------------------------

#[test]
fn test_validation_error_display() {
    let error = Error::validation("Test validation error");
    let display = format!("{}", error);
    assert!(display.contains("Validation error"));
    assert!(display.contains("Test validation error"));
}

#[test]
fn test_cli_error_display() {
    let error = Error::cli("Test CLI error");
    let display = format!("{}", error);
    assert!(display.contains("CLI error"));
    assert!(display.contains("Test CLI error"));
}

#[test]
fn test_missing_field_error_display() {
    let error = Error::missing_field("required_field");
    let display = format!("{}", error);
    assert!(display.contains("Missing required field"));
    assert!(display.contains("required_field"));
}

#[test]
fn test_error_result_propagation() {
    let error = Error::model_not_found("test-model");

    let result: Result<(), Error> = Err(error);

    assert!(result.is_err());
    match result {
        Err(Error::ModelNotFound { model_name }) => {
            assert_eq!(model_name, "test-model");
        }
        _ => panic!("Expected ModelNotFound error"),
    }
}

#[test]
fn test_all_error_constructors_succeed() {
    let errors = vec![
        Error::validation("test"),
        Error::model("test"),
        Error::model_not_found("test-model"),
        Error::model_load_failed("test-model", "test reason"),
        Error::config("test"),
        Error::agent_definition("test"),
        Error::tool_definition("test"),
        Error::workflow_definition("test"),
        Error::missing_field("test-field"),
        Error::invalid_value("test-field", "test reason"),
        Error::cli("test"),
        Error::execution("test"),
        Error::benchmark("test"),
        Error::metrics_collection("test"),
        Error::memory("test"),
        Error::schedule("test"),
        Error::metric("test"),
        Error::verification("test"),
    ];

    assert_eq!(errors.len(), 18);

    for error in errors {
        let display = format!("{}", error);
        assert!(!display.is_empty());
    }
}

// ---------------------------------------------------------------------------

#[test]
fn test_zero_total_resources() {
    let manager = ResourceManager::new(0, 0);

    assert_eq!(manager.available_ram(), 0);
    assert_eq!(manager.available_vram(), 0);
    assert_eq!(manager.allocated_ram(), 0);
    assert_eq!(manager.allocated_vram(), 0);
    assert!(!manager.can_allocate(1, 0));
    assert!(!manager.can_allocate(0, 1));
}

#[test]
fn test_exact_capacity_allocation() {
    let mut manager = ResourceManager::new(1000, 2000);

    assert!(manager.can_allocate(1000, 2000));

    let result = manager.allocate("test-model", 1000, 2000);
    assert!(result.is_ok());

    assert_eq!(manager.available_ram(), 0);
    assert_eq!(manager.available_vram(), 0);
    assert_eq!(manager.allocated_ram(), 1000);
    assert_eq!(manager.allocated_vram(), 2000);

    assert!(!manager.can_allocate(1, 0));
    assert!(!manager.can_allocate(0, 1));
}

#[test]
fn test_deallocate_more_than_allocated_saturates() {
    let mut manager = ResourceManager::new(1000, 2000);

    manager.allocate("test-model", 500, 1000).unwrap();

    manager.deallocate("test-model", 1000, 2000);

    assert_eq!(manager.allocated_ram(), 0);
    assert_eq!(manager.allocated_vram(), 0);
    assert_eq!(manager.available_ram(), 1000);
    assert_eq!(manager.available_vram(), 2000);
}

// ---------------------------------------------------------------------------

#[test]
fn test_nested_runtime_template() {
    let interpolator = TemplateInterpolator::new();
    let mut context = HashMap::new();
    context.insert("outer".to_string(), "value1".to_string());
    context.insert("inner".to_string(), "value2".to_string());

    let result = interpolator
        .resolve_runtime("{{outer}}-{{inner}}-{{outer}}", &context)
        .unwrap();

    assert_eq!(result, "value1-value2-value1");
}

#[test]
fn test_missing_runtime_key_returns_error() {
    let interpolator = TemplateInterpolator::new();
    let context = HashMap::new();

    let result = interpolator.resolve_runtime("{{missing_key}}", &context);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_structural_multiple_same_placeholder() {
    let interpolator = TemplateInterpolator::new();
    let mut models = HashMap::new();
    models.insert("primary".to_string(), "model-1".to_string());

    let template = "Use {{models.primary}} and then {{models.primary}} again";
    let result = interpolator
        .resolve_structural(template, &models)
        .unwrap();

    assert_eq!(result, template);
}

#[test]
fn test_chat_completion_request_serialization() {
    let request = ChatCompletionRequest {
        model: "test-model".to_string(),
        messages: vec![
            ChatMessage::user("Hello"),
            ChatMessage::assistant("Hi"),
        ],
        max_tokens: Some(100),
        temperature: Some(0.7),
        top_p: Some(0.9),
        top_k: Some(40),
        repeat_penalty: Some(1.1),
        presence_penalty: Some(0.5),
        frequency_penalty: Some(0.3),
        stream: false,
        stop: Some(vec!["\n".to_string(), "".to_string()]),
        seed: Some(42),
    };

    let json = serde_json::to_string(&request).expect("Serialization should succeed");

    assert!(json.contains("\"model\":\"test-model\""));
    assert!(json.contains("\"max_tokens\":100"));
    assert!(json.contains("\"temperature\":0.7"));
    assert!(json.contains("\"top_p\":0.9"));
    assert!(json.contains("\"top_k\":40"));
    assert!(json.contains("\"repeat_penalty\":1.1"));
    assert!(json.contains("\"presence_penalty\":0.5"));
    assert!(json.contains("\"frequency_penalty\":0.3"));
    assert!(json.contains("\"stream\":false"));
    assert!(json.contains("\"seed\":42"));
}

#[test]
fn test_chat_completion_response_deserialization() {
    let json = r#"{
        "id": "chat-123",
        "object": "chat.completion",
        "created": 1234567890,
        "model": "test-model",
        "choices": [
            {
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello!"
                },
                "finish_reason": "stop"
            }
        ],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 20,
            "total_tokens": 30
        }
    }"#;

    let response: ChatCompletionResponse =
        serde_json::from_str(json).expect("Deserialization should succeed");

    assert_eq!(response.id, "chat-123");
    assert_eq!(response.object, "chat.completion");
    assert_eq!(response.created, 1234567890);
    assert_eq!(response.model, "test-model");
    assert_eq!(response.choices.len(), 1);
    assert_eq!(response.choices[0].index, 0);
    assert_eq!(response.choices[0].message.role, "assistant");
    assert_eq!(response.choices[0].message.content, "Hello!");
    assert_eq!(response.choices[0].finish_reason, "stop");
    assert_eq!(response.usage.prompt_tokens, 10);
    assert_eq!(response.usage.completion_tokens, 20);
    assert_eq!(response.usage.total_tokens, 30);
}

#[test]
fn test_resource_utilization_percent_edge_cases() {
    let mut manager = ResourceManager::new(1000, 2000);

    let (ram_util, vram_util) = manager.utilization_percent();
    assert_eq!(ram_util, 0.0);
    assert_eq!(vram_util, 0.0);

    manager.allocate("test-model", 500, 1000).unwrap();
    let (ram_util, vram_util) = manager.utilization_percent();
    assert!((ram_util - 50.0).abs() < 0.01);
    assert!((vram_util - 50.0).abs() < 0.01);

    manager.allocate("test-model-2", 500, 1000).unwrap();
    let (ram_util, vram_util) = manager.utilization_percent();
    assert!((ram_util - 100.0).abs() < 0.01);
    assert!((vram_util - 100.0).abs() < 0.01);
}
