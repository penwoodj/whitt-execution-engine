//! Llama.cpp with Vulkan backend implementation.
//!
//! Implements the LlmBackend trait for llama.cpp servers with Vulkan GPU acceleration.
//!
//! HTTP client code is gated behind the "client" feature.

use crate::backend::llm_backend::{
    BackendCapabilities, ChatMessage, ChatResponse, HealthStatus, LlmBackend, LlmError, ModelInfo,
};
use crate::config::provider::LlamaCppVulkanProvider;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::time::Duration;

// ---------------------------------------------------------------------------
// Backend struct
// ---------------------------------------------------------------------------

/// Llama.cpp backend with Vulkan support.
#[derive(Debug, Clone)]
pub struct LlamaCppVulkanBackend {
    /// Provider configuration.
    #[allow(dead_code)]
    config: LlamaCppVulkanProvider,

    /// Base URL for API requests.
    base_url: String,

    /// Request timeout.
    timeout: Duration,

    /// Max retries.
    max_retries: u32,

    /// Backoff strategy.
    backoff_strategy: BackoffStrategy,
}

impl LlamaCppVulkanBackend {
    /// Create a new Llama.cpp Vulkan backend from configuration.
    pub fn from_config(config: LlamaCppVulkanProvider) -> Self {
        let connection_config = config.config.clone().unwrap_or_default();
        let requests_config = config.requests.clone().unwrap_or_default();
        let retry_config = requests_config.retry.unwrap_or_default();
        let backoff_config = retry_config.backoff.unwrap_or_default();

        let host = connection_config.host;
        let port = connection_config.port;
        let base_url = format!("http://{}:{}", host, port);
        let timeout = Duration::from_secs(connection_config.connection_timeout_secs);
        let max_retries = retry_config.max_retries;
        let backoff_strategy = match backoff_config.strategy.as_str() {
            "exponential" => BackoffStrategy::Exponential {
                initial_delay: Duration::from_secs(backoff_config.initial_delay_secs),
                max_delay: Duration::from_secs(backoff_config.max_delay_secs),
                multiplier: backoff_config.multiplier,
            },
            "linear" => BackoffStrategy::Linear {
                initial_delay: Duration::from_secs(backoff_config.initial_delay_secs),
                max_delay: Duration::from_secs(backoff_config.max_delay_secs),
                increment: Duration::from_secs(1),
            },
            "fixed" => BackoffStrategy::Fixed {
                delay: Duration::from_secs(backoff_config.initial_delay_secs),
            },
            _ => BackoffStrategy::Exponential {
                initial_delay: Duration::from_secs(backoff_config.initial_delay_secs),
                max_delay: Duration::from_secs(backoff_config.max_delay_secs),
                multiplier: backoff_config.multiplier,
            },
        };

        tracing::debug!(
            base_url = %base_url,
            timeout_secs = connection_config.connection_timeout_secs,
            max_retries,
            "Created LlamaCppVulkanBackend"
        );

        Self {
            config,
            base_url,
            timeout,
            max_retries,
            backoff_strategy,
        }
    }

    /// Calculate delay for a given retry attempt with jitter.
    fn calculate_retry_delay(&self, attempt: u32) -> Duration {
        let base_delay = match &self.backoff_strategy {
            BackoffStrategy::Exponential {
                initial_delay,
                max_delay,
                multiplier,
            } => {
                let delay = initial_delay.as_secs_f64() * multiplier.powi(attempt as i32 - 1);
                let delay_secs = delay.min(max_delay.as_secs_f64());
                Duration::from_secs_f64(delay_secs)
            }
            BackoffStrategy::Linear {
                initial_delay,
                max_delay,
                increment,
            } => {
                let delay = initial_delay.as_secs_f64() + (attempt as u64 * increment.as_secs()) as f64;
                let delay_secs = delay.min(max_delay.as_secs_f64());
                Duration::from_secs_f64(delay_secs)
            }
            BackoffStrategy::Fixed { delay } => *delay,
        };

        // Add jitter using fastrand (±20%)
        let jitter = fastrand::f64() * 0.4 - 0.2; // -0.2 to +0.2
        let delay_secs = base_delay.as_secs_f64() * (1.0 + jitter);
        Duration::from_secs_f64(delay_secs.max(0.0))
    }
}

// ---------------------------------------------------------------------------
// Backoff strategy
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum BackoffStrategy {
    Exponential {
        initial_delay: Duration,
        max_delay: Duration,
        multiplier: f64,
    },
    Linear {
        initial_delay: Duration,
        max_delay: Duration,
        increment: Duration,
    },
    Fixed {
        delay: Duration,
    },
}

// ---------------------------------------------------------------------------
// LlmBackend implementation (feature-gated)
// ---------------------------------------------------------------------------

#[async_trait::async_trait]
impl LlmBackend for LlamaCppVulkanBackend {
    async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
    ) -> Result<ChatResponse, LlmError> {
        #[cfg(feature = "client")]
        {
            tracing::debug!(model, message_count = messages.len(), "Sending chat request");

            let url = format!("{}/v1/chat/completions", self.base_url);
            let request_body = ChatCompletionRequest {
                model: model.to_string(),
                messages,
                stream: false,
            };

            let client = reqwest::Client::builder()
                .timeout(self.timeout)
                .build()
                .map_err(|e| LlmError::Internal(format!("Failed to create HTTP client: {}", e)))?;

            let mut last_error = None;

            for attempt in 1..=self.max_retries {
                let response = client
                    .post(&url)
                    .json(&request_body)
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            let text = resp
                                .text()
                                .await
                                .map_err(|e| LlmError::Parse(format!("Failed to read response: {}", e)))?;

                            tracing::debug!(response_len = text.len(), "Received chat response");

                            let chat_response: ChatCompletionResponse = serde_json::from_str(&text)
                                .map_err(|e| LlmError::Parse(format!("Failed to parse response: {}", e)))?;

                            return Ok(ChatResponse {
                                content: chat_response.choices[0].message.content.clone(),
                                model: chat_response.model,
                                usage: chat_response.usage,
                            });
                        } else if resp.status() == 429 {
                            last_error = Some(LlmError::RateLimited(
                                resp.text()
                                    .await
                                    .unwrap_or_else(|_| "Rate limited".into()),
                            ));
                        } else {
                            let status = resp.status();
                            let error_text = resp
                                .text()
                                .await
                                .unwrap_or_else(|_| "Unknown error".into());
                            last_error = Some(LlmError::Connection(format!(
                                "HTTP {}: {}",
                                status, error_text
                            )));
                        }
                    }
                    Err(e) if e.is_timeout() => {
                        last_error = Some(LlmError::Timeout(e.to_string()));
                    }
                    Err(e) if e.is_connect() => {
                        last_error = Some(LlmError::Connection(e.to_string()));
                    }
                    Err(e) => {
                        last_error = Some(LlmError::Internal(format!("Request failed: {}", e)));
                    }
                }

                if attempt < self.max_retries {
                    let delay = self.calculate_retry_delay(attempt);
                    tracing::debug!(attempt, delay_secs = delay.as_secs(), "Retrying after error");
                    tokio::time::sleep(delay).await;
                }
            }

            Err(last_error.unwrap_or_else(|| LlmError::Internal("Max retries exceeded".into())))
        }

        #[cfg(not(feature = "client"))]
        {
            Err(LlmError::Internal(
                "HTTP client not available (enable 'client' feature)".into(),
            ))
        }
    }

    async fn chat_stream(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, LlmError>> + Send>>, LlmError> {
        #[cfg(feature = "client")]
        {
            tracing::debug!(model, message_count = messages.len(), "Sending chat stream request");

            let url = format!("{}/v1/chat/completions", self.base_url);
            let request_body = ChatCompletionRequest {
                model: model.to_string(),
                messages,
                stream: true,
            };

            let client = reqwest::Client::builder()
                .timeout(self.timeout)
                .build()
                .map_err(|e| LlmError::Internal(format!("Failed to create HTTP client: {}", e)))?;

            let mut last_error = None;

            for attempt in 1..=self.max_retries {
                let response = client
                    .post(&url)
                    .json(&request_body)
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            tracing::debug!("Receiving SSE stream");
                            let byte_stream = resp.bytes_stream();
                            let text_stream = byte_stream.map(|bytes_result| {
                                let bytes = bytes_result.map_err(|e| {
                                    LlmError::Parse(format!("Failed to read bytes: {}", e))
                                })?;
                                Ok(String::from_utf8_lossy(&bytes).to_string())
                            });
                            return Ok(Box::pin(text_stream));
                        } else if resp.status() == 429 {
                            last_error = Some(LlmError::RateLimited(
                                resp.text()
                                    .await
                                    .unwrap_or_else(|_| "Rate limited".into()),
                            ));
                        } else {
                            let status = resp.status();
                            let error_text = resp
                                .text()
                                .await
                                .unwrap_or_else(|_| "Unknown error".into());
                            last_error = Some(LlmError::Connection(format!(
                                "HTTP {}: {}",
                                status, error_text
                            )));
                        }
                    }
                    Err(e) if e.is_timeout() => {
                        last_error = Some(LlmError::Timeout(e.to_string()));
                    }
                    Err(e) if e.is_connect() => {
                        last_error = Some(LlmError::Connection(e.to_string()));
                    }
                    Err(e) => {
                        last_error = Some(LlmError::Internal(format!("Request failed: {}", e)));
                    }
                }

                if attempt < self.max_retries {
                    let delay = self.calculate_retry_delay(attempt);
                    tracing::debug!(attempt, delay_secs = delay.as_secs(), "Retrying after error");
                    tokio::time::sleep(delay).await;
                }
            }

            Err(last_error.unwrap_or_else(|| LlmError::Internal("Max retries exceeded".into())))
        }

        #[cfg(not(feature = "client"))]
        {
            Err(LlmError::Internal(
                "HTTP client not available (enable 'client' feature)".into(),
            ))
        }
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        #[cfg(feature = "client")]
        {
            tracing::debug!("Listing models");

            let url = format!("{}/v1/models", self.base_url);

            let client = reqwest::Client::builder()
                .timeout(self.timeout)
                .build()
                .map_err(|e| LlmError::Internal(format!("Failed to create HTTP client: {}", e)))?;

            let mut last_error = None;

            for attempt in 1..=self.max_retries {
                let response = client.get(&url).send().await;

                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            let text = resp
                                .text()
                                .await
                                .map_err(|e| LlmError::Parse(format!("Failed to read response: {}", e)))?;

                            let models_response: ModelsResponse = serde_json::from_str(&text)
                                .map_err(|e| LlmError::Parse(format!("Failed to parse response: {}", e)))?;

                            tracing::debug!(model_count = models_response.data.len(), "Listed models");

                            return Ok(models_response.data);
                        } else {
                            let status = resp.status();
                            let error_text = resp
                                .text()
                                .await
                                .unwrap_or_else(|_| "Unknown error".into());
                            last_error = Some(LlmError::Connection(format!(
                                "HTTP {}: {}",
                                status, error_text
                            )));
                        }
                    }
                    Err(e) if e.is_timeout() => {
                        last_error = Some(LlmError::Timeout(e.to_string()));
                    }
                    Err(e) if e.is_connect() => {
                        last_error = Some(LlmError::Connection(e.to_string()));
                    }
                    Err(e) => {
                        last_error = Some(LlmError::Internal(format!("Request failed: {}", e)));
                    }
                }

                if attempt < self.max_retries {
                    let delay = self.calculate_retry_delay(attempt);
                    tracing::debug!(attempt, delay_secs = delay.as_secs(), "Retrying after error");
                    tokio::time::sleep(delay).await;
                }
            }

            Err(last_error.unwrap_or_else(|| LlmError::Internal("Max retries exceeded".into())))
        }

        #[cfg(not(feature = "client"))]
        {
            Err(LlmError::Internal(
                "HTTP client not available (enable 'client' feature)".into(),
            ))
        }
    }

    async fn health_check(&self) -> Result<HealthStatus, LlmError> {
        #[cfg(feature = "client")]
        {
            tracing::debug!("Performing health check");

            let url = format!("{}/health", self.base_url);

            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .map_err(|e| LlmError::Internal(format!("Failed to create HTTP client: {}", e)))?;

            match client.get(&url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        tracing::debug!("Health check passed");
                        Ok(HealthStatus::Healthy)
                    } else if resp.status().is_server_error() {
                        tracing::warn!("Health check: server error");
                        Ok(HealthStatus::Degraded)
                    } else {
                        tracing::warn!("Health check: unhealthy");
                        Ok(HealthStatus::Unhealthy)
                    }
                }
                Err(e) if e.is_timeout() || e.is_connect() => {
                    tracing::warn!(error = %e, "Health check failed");
                    Ok(HealthStatus::Unhealthy)
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Health check failed");
                    Ok(HealthStatus::Degraded)
                }
            }
        }

        #[cfg(not(feature = "client"))]
        {
            Err(LlmError::Internal(
                "HTTP client not available (enable 'client' feature)".into(),
            ))
        }
    }

    async fn load_model(&self, model_id: &str) -> Result<(), LlmError> {
        #[cfg(feature = "client")]
        {
            tracing::debug!(model_id, "Loading model");

            let url = format!("{}/v1/models/load", self.base_url);
            let request_body = ModelLoadRequest {
                model_id: model_id.to_string(),
            };

            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(300)) // 5 minutes for model loading
                .build()
                .map_err(|e| LlmError::Internal(format!("Failed to create HTTP client: {}", e)))?;

            let mut last_error = None;

            for attempt in 1..=self.max_retries {
                let response = client.post(&url).json(&request_body).send().await;

                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            tracing::debug!(model_id, "Model loaded successfully");
                            return Ok(());
                        } else {
                            let status = resp.status();
                            let error_text = resp
                                .text()
                                .await
                                .unwrap_or_else(|_| "Unknown error".into());
                            last_error = Some(LlmError::Model(format!(
                                "Failed to load model (HTTP {}): {}",
                                status, error_text
                            )));
                        }
                    }
                    Err(e) if e.is_timeout() => {
                        last_error = Some(LlmError::Timeout(e.to_string()));
                    }
                    Err(e) if e.is_connect() => {
                        last_error = Some(LlmError::Connection(e.to_string()));
                    }
                    Err(e) => {
                        last_error = Some(LlmError::Internal(format!("Request failed: {}", e)));
                    }
                }

                if attempt < self.max_retries {
                    let delay = self.calculate_retry_delay(attempt);
                    tracing::debug!(attempt, delay_secs = delay.as_secs(), "Retrying after error");
                    tokio::time::sleep(delay).await;
                }
            }

            Err(last_error.unwrap_or_else(|| LlmError::Internal("Max retries exceeded".into())))
        }

        #[cfg(not(feature = "client"))]
        {
            Err(LlmError::Internal(
                "HTTP client not available (enable 'client' feature)".into(),
            ))
        }
    }

    async fn unload_model(&self, model_id: &str) -> Result<(), LlmError> {
        #[cfg(feature = "client")]
        {
            tracing::debug!(model_id, "Unloading model");

            let url = format!("{}/v1/models/unload", self.base_url);
            let request_body = ModelUnloadRequest {
                model_id: model_id.to_string(),
            };

            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .map_err(|e| LlmError::Internal(format!("Failed to create HTTP client: {}", e)))?;

            let mut last_error = None;

            for attempt in 1..=self.max_retries {
                let response = client.post(&url).json(&request_body).send().await;

                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            tracing::debug!(model_id, "Model unloaded successfully");
                            return Ok(());
                        } else {
                            let status = resp.status();
                            let error_text = resp
                                .text()
                                .await
                                .unwrap_or_else(|_| "Unknown error".into());
                            last_error = Some(LlmError::Model(format!(
                                "Failed to unload model (HTTP {}): {}",
                                status, error_text
                            )));
                        }
                    }
                    Err(e) if e.is_timeout() => {
                        last_error = Some(LlmError::Timeout(e.to_string()));
                    }
                    Err(e) if e.is_connect() => {
                        last_error = Some(LlmError::Connection(e.to_string()));
                    }
                    Err(e) => {
                        last_error = Some(LlmError::Internal(format!("Request failed: {}", e)));
                    }
                }

                if attempt < self.max_retries {
                    let delay = self.calculate_retry_delay(attempt);
                    tracing::debug!(attempt, delay_secs = delay.as_secs(), "Retrying after error");
                    tokio::time::sleep(delay).await;
                }
            }

            Err(last_error.unwrap_or_else(|| LlmError::Internal("Max retries exceeded".into())))
        }

        #[cfg(not(feature = "client"))]
        {
            Err(LlmError::Internal(
                "HTTP client not available (enable 'client' feature)".into(),
            ))
        }
    }

    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            streaming: true,
            tools: false,
            function_calling: false,
        }
    }

    fn base_url(&self) -> String {
        self.base_url.clone()
    }
}

// ---------------------------------------------------------------------------
// Request/Response types (OpenAI-compatible)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatCompletionResponse {
    model: String,
    choices: Vec<ChatCompletionChoice>,
    usage: std::collections::HashMap<String, u64>,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatCompletionChoice {
    message: ChatCompletionMessage,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatCompletionMessage {
    content: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ModelsResponse {
    data: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize)]
struct ModelLoadRequest {
    model_id: String,
}

#[derive(Debug, Clone, Serialize)]
struct ModelUnloadRequest {
    model_id: String,
}
