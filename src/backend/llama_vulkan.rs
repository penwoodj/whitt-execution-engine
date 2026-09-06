//! Llama.cpp with Vulkan backend implementation.
//!
//! Implements the LlmBackend trait for llama.cpp servers with Vulkan GPU acceleration.
//!
//! HTTP client code is gated behind the "client" feature.

use crate::backend::llm_backend::{
    BackendCapabilities, ChatMessage, ChatResponse, HealthStatus, LlmBackend, LlmError, ModelInfo,
};
use crate::config::provider::LlamaCppVulkanProvider;
#[cfg(feature = "client")]
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
#[cfg(feature = "client")]
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

        let host = connection_config.host;
        let port = connection_config.port;
        let base_url = format!("http://{}:{}", host, port);
        let timeout = Duration::from_secs(connection_config.connection_timeout_secs);
        let max_retries = retry_config.max_retries;
        let backoff_strategy = match retry_config.backoff.as_str() {
            "exponential" => BackoffStrategy::Exponential {
                initial_delay: Duration::from_secs(retry_config.initial_delay),
                max_delay: Duration::from_secs(retry_config.max_delay),
                multiplier: retry_config.multiplier,
            },
            "linear" => BackoffStrategy::Linear {
                initial_delay: Duration::from_secs(retry_config.initial_delay),
                max_delay: Duration::from_secs(retry_config.max_delay),
                increment: Duration::from_secs(1),
            },
            "fixed" => BackoffStrategy::Fixed {
                delay: Duration::from_secs(retry_config.initial_delay),
            },
            _ => BackoffStrategy::Exponential {
                initial_delay: Duration::from_secs(retry_config.initial_delay),
                max_delay: Duration::from_secs(retry_config.max_delay),
                multiplier: retry_config.multiplier,
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

        let use_jitter = self.config
            .requests
            .as_ref()
            .and_then(|r| r.retry.as_ref())
            .map(|r| r.jitter)
            .unwrap_or(true);

        // Add jitter if enabled using fastrand (±20%)
        if use_jitter {
            let jitter = fastrand::f64() * 0.4 - 0.2; // -0.2 to +0.2
            let delay_secs = base_delay.as_secs_f64() * (1.0 + jitter);
            Duration::from_secs_f64(delay_secs.max(0.0))
        } else {
            base_delay
        }
    }

    /// Verify a load/unload actually completed by polling `/v1/models`.
    ///
    /// The router-mode POST endpoints are fire-and-forget ACKs (~13ms
    /// `{"success":true}`), not completion confirmations
    /// (reasoning-enhancer-plus/docs/07-TRACKING.md:23-25). For loads the
    /// entry must report `status=loaded`; for unloads, either
    /// `status=unloaded` or absence from the list counts (servers drop
    /// unloaded entries). Status field is a whitt-server extension, hence
    /// the untyped JSON.
    #[cfg(feature = "client")]
    async fn wait_until_model_status(
        base_url: &str,
        model_id: &str,
        loaded: bool,
    ) -> Result<(), LlmError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|e| LlmError::Internal(format!("Failed to create HTTP client: {}", e)))?;
        let deadline = std::time::Instant::now() + Duration::from_secs(240);
        while std::time::Instant::now() < deadline {
            let resp = client.get(format!("{}/v1/models", base_url)).send().await;
            if let Ok(resp) = resp {
                if resp.status().is_success() {
                    if let Ok(list) = resp.json::<serde_json::Value>().await {
                        if let Some(entries) = list.get("data").and_then(|d| d.as_array()) {
                            let entry = entries.iter().find(|e| {
                                e.get("id").and_then(|i| i.as_str()) == Some(model_id)
                            });
                            match entry {
                                None if !loaded => return Ok(()),
                                None => {}
                                Some(e) => {
                                    let status = e
                                        .get("status")
                                        .and_then(|s| s.get("value"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("");
                                    if loaded && status == "loaded" {
                                        return Ok(());
                                    }
                                    if !loaded && status == "unloaded" {
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
        Err(LlmError::Timeout(format!(
            "model {} did not reach status '{}' within 240s (async load/unload verification)",
            model_id,
            if loaded { "loaded" } else { "unloaded" }
        )))
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

    #[cfg(feature = "client")]
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
                            tracing::debug!(model_id, "Load ACK received, verifying actual load");
                            return Self::wait_until_model_status(&self.base_url, model_id, true).await;
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
                            tracing::debug!(model_id, "Unload ACK received, verifying actual unload");
                            return Self::wait_until_model_status(&self.base_url, model_id, false).await;
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

#[cfg(all(test, feature = "client"))]
mod async_load_tests {
    use super::*;
    use std::io::{Read, Write};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn read_request(stream: &mut std::net::TcpStream) -> std::io::Result<(String, String)> {
        let mut buf = [0u8; 4096];
        let mut raw = Vec::new();
        loop {
            let n = stream.read(&mut buf)?;
            if n == 0 {
                break;
            }
            raw.extend_from_slice(&buf[..n]);
            let s = String::from_utf8_lossy(&raw);
            if let Some(header_end) = s.find("\r\n\r\n") {
                let body_start = header_end + 4;
                for line in s[..header_end].lines() {
                    if let Some(v) = line.split(':').next().map(|k| k.eq_ignore_ascii_case("content-length")) {
                        if v {
                            if let Some(len) = line.split(':').nth(1).and_then(|v| v.trim().parse::<usize>().ok()) {
                                if raw.len() >= body_start + len {
                                    return Ok((
                                        s[..header_end].lines().next().unwrap_or_default().to_string(),
                                        s[body_start..].to_string(),
                                    ));
                                }
                            }
                        }
                    }
                }
                return Ok((
                    s[..header_end].lines().next().unwrap_or_default().to_string(),
                    String::new(),
                ));
            }
        }
        Ok((String::new(), String::new()))
    }

    fn respond(stream: &mut std::net::TcpStream, status: &str, body: &str) -> std::io::Result<()> {
        let resp = format!(
            "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            status,
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes())
    }

    fn backend_for(port: u16) -> LlamaCppVulkanBackend {
        let provider = crate::config::provider::LlamaCppVulkanProvider {
            config: Some(crate::config::provider::LlamaCppConfig {
                host: "127.0.0.1".to_string(),
                port: port as u32,
                connection_timeout_secs: 5,
            }),
            hosting: None,
            requests: None,
        };
        LlamaCppVulkanBackend::from_config(provider)
    }

    // Regression (Issue P): POST /v1/models/load is a fire-and-forget ACK
    // ("~13ms {"success":true} — NOT load confirmation",
    // reasoning-enhancer-plus/docs/07-TRACKING.md:23-25). The backend
    // trusted the ACK and returned Ok before the model was loaded.
    // Server runs on a plain std::thread: blocking accept on the
    // current_thread test runtime would starve the client future.
    #[tokio::test]
    async fn given_loading_status_when_load_ack_received_then_waits_until_loaded() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let status_polls = Arc::new(AtomicUsize::new(0));
        let polls_seen = status_polls.clone();

        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let (req_line, _body) = read_request(&mut stream).unwrap();
                if req_line.starts_with("POST /v1/models/load") {
                    respond(&mut stream, "200 OK", r#"{"success":true}"#).unwrap();
                } else if req_line.starts_with("GET /v1/models") {
                    let n = polls_seen.fetch_add(1, Ordering::SeqCst) + 1;
                    let status = if n < 3 { "loading" } else { "loaded" };
                    respond(
                        &mut stream,
                        "200 OK",
                        &format!(r#"{{"data":[{{"id":"m","status":{{"value":"{}"}}}}]}}"#, status),
                    )
                    .unwrap();
                } else {
                    respond(&mut stream, "404 Not Found", "{}").unwrap();
                }
            }
        });

        let backend = backend_for(port);
        backend.load_model("m").await.expect("load must succeed");

        assert!(
            status_polls.load(Ordering::SeqCst) >= 3,
            "load_model must poll /v1/models until status=loaded, saw {} polls",
            status_polls.load(Ordering::SeqCst)
        );
    }

    // Regression (Issue M engine-side half): POST /v1/models/unload returns
    // before unload completes (atomic-reasoning/multi-model/atom-v1.yml:55
    // sleeps 3/15 around raw curls). Backend must verify — absent from the
    // model list counts as unloaded.
    #[tokio::test]
    async fn given_still_listed_when_unload_ack_received_then_waits_until_gone() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let status_polls = Arc::new(AtomicUsize::new(0));
        let polls_seen = status_polls.clone();

        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let (req_line, _body) = read_request(&mut stream).unwrap();
                if req_line.starts_with("POST /v1/models/unload") {
                    respond(&mut stream, "200 OK", r#"{"success":true}"#).unwrap();
                } else if req_line.starts_with("GET /v1/models") {
                    let n = polls_seen.fetch_add(1, Ordering::SeqCst) + 1;
                    let body = if n < 2 {
                        r#"{"data":[{"id":"m","status":{"value":"loaded"}}]}"#.to_string()
                    } else {
                        r#"{"data":[]}"#.to_string()
                    };
                    respond(&mut stream, "200 OK", &body).unwrap();
                } else {
                    respond(&mut stream, "404 Not Found", "{}").unwrap();
                }
            }
        });

        let backend = backend_for(port);
        backend.unload_model("m").await.expect("unload must succeed");

        assert!(
            status_polls.load(Ordering::SeqCst) >= 2,
            "unload_model must poll /v1/models until the model is gone, saw {} polls",
            status_polls.load(Ordering::SeqCst)
        );
    }
}
