use crate::backend::llm_backend::{
    BackendCapabilities, ChatMessage, ChatResponse, HealthStatus, LlmBackend, LlmError, ModelInfo,
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
#[cfg(test)]
use futures::StreamExt;

#[derive(Debug, Clone)]
pub enum MockError {
    Connection(String),
    Timeout(String),
    Parse(String),
    Model(String),
    RateLimited(String),
    Internal(String),
}

impl From<MockError> for LlmError {
    fn from(err: MockError) -> Self {
        match err {
            MockError::Connection(msg) => LlmError::Connection(msg),
            MockError::Timeout(msg) => LlmError::Timeout(msg),
            MockError::Parse(msg) => LlmError::Parse(msg),
            MockError::Model(msg) => LlmError::Model(msg),
            MockError::RateLimited(msg) => LlmError::RateLimited(msg),
            MockError::Internal(msg) => LlmError::Internal(msg),
        }
    }
}

#[derive(Debug, Clone)]
struct MockConfig {
    model_name: String,
    response_content: String,
    error: Option<MockError>,
    delay: Duration,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            model_name: "mock-model".to_string(),
            response_content: "Mock response".to_string(),
            error: None,
            delay: Duration::ZERO,
        }
    }
}

pub struct MockLlmBackend {
    config: Arc<Mutex<MockConfig>>,
}

impl MockLlmBackend {
    pub fn new(model_name: &str) -> Self {
        Self {
            config: Arc::new(Mutex::new(MockConfig {
                model_name: model_name.to_string(),
                ..MockConfig::default()
            })),
        }
    }

    pub fn set_response(&self, content: String) {
        if let Ok(mut config) = self.config.lock() {
            config.response_content = content;
        }
    }

    pub fn set_error(&self, error: Option<MockError>) {
        if let Ok(mut config) = self.config.lock() {
            config.error = error;
        }
    }

    pub fn set_delay(&self, delay: Duration) {
        if let Ok(mut config) = self.config.lock() {
            config.delay = delay;
        }
    }

    pub fn model_name(&self) -> String {
        self.config
            .lock()
            .map(|config| config.model_name.clone())
            .unwrap_or_else(|_| "mock-model".to_string())
    }
}

impl Default for MockLlmBackend {
    fn default() -> Self {
        Self::new("mock-model")
    }
}

#[async_trait]
impl LlmBackend for MockLlmBackend {
    async fn chat(
        &self,
        _messages: Vec<ChatMessage>,
        _model: &str,
    ) -> Result<ChatResponse, LlmError> {
        let delay = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .delay;

        let response_content = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .response_content
            .clone();

        let model_name = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .model_name
            .clone();

        let error = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .error
            .clone();

        tokio::time::sleep(delay).await;

        if let Some(err) = error {
            return Err(LlmError::from(err));
        }

        let mut usage = HashMap::new();
        usage.insert("prompt_tokens".to_string(), 10u64);
        usage.insert("completion_tokens".to_string(), 20u64);
        usage.insert("total_tokens".to_string(), 30u64);

        Ok(ChatResponse {
            content: response_content,
            model: model_name,
            usage,
        })
    }

    async fn chat_stream(
        &self,
        _messages: Vec<ChatMessage>,
        _model: &str,
    ) -> Result<Pin<Box<dyn futures::Stream<Item = Result<String, LlmError>> + Send>>, LlmError> {
        let delay = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .delay;

        let content = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .response_content
            .clone();

        let error = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .error
            .clone();

        let stream = futures::stream::unfold(
            (content, error, delay, 0),
            |(content, error, delay, pos)| async move {
                if pos == 0 {
                    tokio::time::sleep(delay).await;
                }

                if pos == 0 {
                    if let Some(ref err) = error {
                        return Some((Err(LlmError::from(err.clone())), (content, error, delay, pos)));
                    }
                }

                let chars: Vec<char> = content.chars().collect();
                let chunk_size = 5;
                let start_pos = pos * chunk_size;
                let end_pos = (start_pos + chunk_size).min(chars.len());

                if start_pos >= chars.len() {
                    return None;
                }

                let chunk_str: String = chars[start_pos..end_pos].iter().collect();
                tokio::time::sleep(Duration::from_millis(10)).await;

                Some((Ok(chunk_str), (content, error, delay, pos + 1)))
            },
        );

        Ok(Box::pin(stream))
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        let delay = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .delay;

        let model_name = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .model_name
            .clone();

        let error = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .error
            .clone();

        tokio::time::sleep(delay).await;

        if let Some(err) = error {
            return Err(LlmError::from(err));
        }

        Ok(vec![ModelInfo {
            id: model_name,
            object: "model".to_string(),
            owned_by: "mock".to_string(),
        }])
    }

    async fn health_check(&self) -> Result<HealthStatus, LlmError> {
        let delay = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .delay;

        let error = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .error
            .clone();

        tokio::time::sleep(delay).await;

        if let Some(err) = error {
            return Err(LlmError::from(err));
        }

        Ok(HealthStatus::Healthy)
    }

    async fn load_model(&self, _model_id: &str) -> Result<(), LlmError> {
        let delay = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .delay;

        let error = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .error
            .clone();

        tokio::time::sleep(delay).await;

        if let Some(err) = error {
            return Err(LlmError::from(err));
        }

        Ok(())
    }

    async fn unload_model(&self, _model_id: &str) -> Result<(), LlmError> {
        let delay = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .delay;

        let error = self
            .config
            .lock()
            .map_err(|e| LlmError::Internal(format!("Lock error: {}", e)))?
            .error
            .clone();

        tokio::time::sleep(delay).await;

        if let Some(err) = error {
            return Err(LlmError::from(err));
        }

        Ok(())
    }

    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            streaming: true,
            tools: true,
            function_calling: true,
        }
    }

    fn base_url(&self) -> String {
        "http://mock-backend".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_backend_basic_chat() {
        let backend = MockLlmBackend::new("test-model");
        backend.set_response("Test response".to_string());

        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        }];

        let response = backend.chat(messages, "test-model").await.unwrap();
        assert_eq!(response.content, "Test response");
        assert_eq!(response.model, "test-model");
        assert_eq!(response.usage.get("total_tokens"), Some(&30));
    }

    #[tokio::test]
    async fn test_mock_backend_error_injection() {
        let backend = MockLlmBackend::new("test-model");
        backend.set_error(Some(MockError::Timeout("Test timeout".to_string())));

        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        }];

        let result = backend.chat(messages, "test-model").await;
        assert!(result.is_err());
        assert!(matches!(result, Err(LlmError::Timeout(_))));
    }

    #[tokio::test]
    async fn test_mock_backend_delay() {
        let backend = MockLlmBackend::new("test-model");
        backend.set_delay(Duration::from_millis(50));

        let start = std::time::Instant::now();
        let _ = backend
            .chat(vec![], "test-model")
            .await
            .expect("chat should succeed");
        let elapsed = start.elapsed();

        assert!(elapsed >= Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_mock_backend_list_models() {
        let backend = MockLlmBackend::new("test-model");

        let models = backend.list_models().await.unwrap();
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id, "test-model");
        assert_eq!(models[0].object, "model");
        assert_eq!(models[0].owned_by, "mock");
    }

    #[tokio::test]
    async fn test_mock_backend_health_check() {
        let backend = MockLlmBackend::new("test-model");

        let status = backend.health_check().await.unwrap();
        assert_eq!(status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_mock_backend_load_unload_model() {
        let backend = MockLlmBackend::new("test-model");

        backend.load_model("test-model").await.unwrap();
        backend.unload_model("test-model").await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_backend_streaming() {
        let backend = MockLlmBackend::new("test-model");
        backend.set_response("Hello, world!".to_string());

        let mut stream = backend
            .chat_stream(vec![], "test-model")
            .await
            .expect("stream should succeed");

        let mut result = String::new();
        while let Some(chunk) = stream.next().await {
            result.push_str(&chunk.unwrap());
        }

        assert_eq!(result, "Hello, world!");
    }

    #[tokio::test]
    async fn test_mock_backend_streaming_error() {
        let backend = MockLlmBackend::new("test-model");
        backend.set_error(Some(MockError::Connection("Test connection error".to_string())));

        let mut stream = backend
            .chat_stream(vec![], "test-model")
            .await
            .expect("stream creation should succeed");

        let result = stream.next().await;
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), Err(LlmError::Connection(_))));
    }

    #[tokio::test]
    async fn test_mock_backend_capabilities() {
        let backend = MockLlmBackend::new("test-model");

        let caps = backend.capabilities();
        assert!(caps.streaming);
        assert!(caps.tools);
        assert!(caps.function_calling);
    }

    #[tokio::test]
    async fn test_mock_backend_base_url() {
        let backend = MockLlmBackend::new("test-model");

        assert_eq!(backend.base_url(), "http://mock-backend");
    }
}
