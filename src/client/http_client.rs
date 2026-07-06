//! HTTP client for llama-server with SSE streaming support.
//!
//! Uses reqwest 0.13 with rustls backend. Handles SSE streaming with
//! chunk-split buffering (llama.cpp PR #9519).

use anyhow::{Context, Result};
use futures::StreamExt;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;
use tokio::time::sleep;

use super::types::{
    ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse, HealthResponse,
    ModelInfo, ModelListResponse, ModelLoadRequest, ModelLoadResponse, ModelUnloadRequest,
};

/// Maximum retry attempts for transient HTTP errors (connection refused, 503, etc.).
const MAX_RETRIES: usize = 5;
/// Delay between retry attempts.
const RETRY_DELAY: Duration = Duration::from_secs(1);

fn is_retryable(err: &anyhow::Error) -> bool {
    let targets = [
        "connection closed before message completed",
        "ConnectionRefused",
        "connect error",
        "503",
        "IncompleteMessage",
    ];
    let chain = err.chain();
    for source in chain {
        let msg = source.to_string();
        if targets.iter().any(|t| msg.contains(t)) {
            return true;
        }
    }
    false
}

/// Which kind of OpenAI-compatible server the client is talking to.
///
/// - `LlamaCpp`: llama.cpp server (router mode) — has `/health`,
///   `models/load`, `models/unload`, and per-model status in `/v1/models`.
/// - `LmStudio`: LM Studio local server — OpenAI-compatible
///   (`/v1/models`, `/v1/chat/completions`) but no `/health` and no explicit
///   load/unload endpoints; models are loaded just-in-time when named in a
///   chat completion request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BackendKind {
    #[default]
    LlamaCpp,
    LmStudio,
}

/// HTTP client for llama-server.
#[derive(Clone)]
pub struct LlamaHttpClient {
    client: Client,
    base_url: String,
    kind: BackendKind,
}

impl LlamaHttpClient {
    /// Create a new HTTP client (llama.cpp semantics).
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        Self::new_with_kind(base_url, BackendKind::LlamaCpp)
    }

    /// Create a new HTTP client for a specific backend kind.
    pub fn new_with_kind(base_url: impl Into<String>, kind: BackendKind) -> Result<Self> {
        let client = ClientBuilder::new()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(3600))
            .pool_idle_timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.into(),
            kind,
        })
    }

    /// The backend kind this client targets.
    pub fn kind(&self) -> BackendKind {
        self.kind
    }

    fn url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint)
    }

    // -----------------------------------------------------------------------
    // Health / info
    // -----------------------------------------------------------------------

    /// Health check.
    ///
    /// LM Studio has no `/health` endpoint, so reachability of `/v1/models`
    /// is used as the health signal and a synthetic response is returned.
    pub async fn health(&self) -> Result<HealthResponse> {
        if self.kind == BackendKind::LmStudio {
            let resp = self
                .client
                .get(self.url("v1/models"))
                .send()
                .await
                .context("Failed to reach LM Studio server (/v1/models)")?;
            if !resp.status().is_success() {
                anyhow::bail!("LM Studio health probe failed: HTTP {}", resp.status());
            }
            return Ok(HealthResponse {
                status: "ok".into(),
                slots_idle: 1,
                slots_processing: 0,
            });
        }
        self.client
            .get(self.url("health"))
            .send()
            .await
            .context("Failed to send health check")?
            .json()
            .await
            .context("Failed to parse health response")
    }

    /// Wait for server to report healthy.
    pub async fn wait_for_healthy(&self, max_wait: Duration) -> Result<()> {
        let start = std::time::Instant::now();
        while start.elapsed() < max_wait {
            match self.health().await {
                Ok(resp) if resp.status == "ok" => return Ok(()),
                _ => sleep(Duration::from_secs(2)).await,
            }
        }
        anyhow::bail!("Server not healthy within {:?}", max_wait);
    }

    // -----------------------------------------------------------------------
    // Model management (router mode)
    // -----------------------------------------------------------------------

    pub async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let resp = self
            .client
            .get(self.url("v1/models"))
            .send()
            .await
            .context("Failed to list models")?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("List models failed: {} – {}", status, body);
        }
        let models: ModelListResponse = resp.json().await.context("Failed to parse model list")?;
        Ok(models.data)
    }

    pub async fn load_model(&self, model_id: impl Into<String>) -> Result<()> {
        let model_id = model_id.into();
        if self.kind == BackendKind::LmStudio {
            // LM Studio loads models just-in-time when they are named in a
            // chat completion request. Verify the model is known, then no-op.
            match self.list_models().await {
                Ok(models) => {
                    if models.iter().any(|m| m.id == model_id) {
                        eprintln!("[MODEL] {} available in LM Studio (JIT load on first request)", model_id);
                    } else {
                        eprintln!(
                            "[MODEL] WARNING: {} not listed by LM Studio /v1/models — \
                             first request will fail unless JIT loading resolves it",
                            model_id
                        );
                    }
                }
                Err(e) => eprintln!("[MODEL] could not list LM Studio models (continuing): {}", e),
            }
            return Ok(());
        }
        eprintln!("[MODEL] loading model: {}", model_id);
        let resp = self
            .client
            .post(self.url("models/load"))
            .json(&ModelLoadRequest { model: model_id.clone() })
            .send()
            .await
            .context("Failed to send load request")?;

        if resp.status().as_u16() == 400 {
            let body = resp.text().await.unwrap_or_default();
            if body.contains("already running") {
                eprintln!("[MODEL] already loaded, skipping");
                return Ok(());
            }
            anyhow::bail!("Load model failed: 400 – {}", body);
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Load model failed: {} – {}", status, body);
        }
        let result: ModelLoadResponse = resp.json().await.unwrap_or(ModelLoadResponse { success: true, error: None });
        if !result.success {
            anyhow::bail!("Load model rejected: {:?}", result.error);
        }
        eprintln!("[MODEL] load accepted, waiting for ready...");
        self.wait_for_model_status(&model_id, "loaded", Duration::from_secs(3600)).await
    }

    pub async fn unload_model(&self, model_id: impl Into<String>) -> Result<()> {
        let model_id = model_id.into();
        if self.kind == BackendKind::LmStudio {
            // LM Studio has no unload endpoint; it evicts models via its own
            // JIT auto-unload/TTL policy. Treat unload as a successful no-op.
            eprintln!("[MODEL] skipping unload of {} (LM Studio manages model lifecycle)", model_id);
            return Ok(());
        }
        eprintln!("[MODEL] unloading model: {}", model_id);
        let resp = self
            .client
            .post(self.url("models/unload"))
            .json(&ModelUnloadRequest { model: model_id.clone() })
            .send()
            .await
            .context("Failed to send unload request")?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Unload model failed: {} – {}", status, body);
        }
        self.wait_for_model_status(&model_id, "unloaded", Duration::from_secs(60)).await
    }

    pub async fn wait_for_model_status(
        &self,
        model_id: &str,
        expected_status: &str,
        timeout: Duration,
    ) -> Result<()> {
        if self.kind == BackendKind::LmStudio {
            // LM Studio's /v1/models has no per-model status field.
            return Ok(());
        }
        let start = std::time::Instant::now();
        let poll_interval = Duration::from_millis(500);
        while start.elapsed() < timeout {
            match self.list_models().await {
                Ok(models) => {
                    if let Some(model) = models.iter().find(|m| m.id == model_id) {
                        if model.status.value == expected_status {
                            eprintln!("[MODEL] {} is now {} (took {:?})", model_id, expected_status, start.elapsed());
                            return Ok(());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[MODEL] poll error (ignoring): {}", e);
                }
            }
            sleep(poll_interval).await;
        }
        anyhow::bail!("Model {} did not reach status '{}' within {:?}", model_id, expected_status, timeout);
    }

    // -----------------------------------------------------------------------
    // Completions
    // -----------------------------------------------------------------------

    /// Non-streaming chat completion.
    pub async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let mut attempt = 0;
        loop {
            match self.chat_completion_inner(request.clone()).await {
                Ok(resp) => return Ok(resp),
                Err(e) if is_retryable(&e) && attempt < MAX_RETRIES => {
                    attempt += 1;
                    eprintln!("[RETRY {}/{}] chat_completion: {}", attempt, MAX_RETRIES, e);
                    sleep(RETRY_DELAY).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    async fn chat_completion_inner(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let resp = self
            .client
            .post(self.url("v1/chat/completions"))
            .json(&request)
            .send()
            .await
            .context("Failed to send completion request")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "<no body>".into());
            anyhow::bail!("Completion failed: {} – {}", status, body);
        }

        resp.json()
            .await
            .context("Failed to parse completion response")
    }

    /// Streaming chat completion.
    ///
    /// Returns a stream of `ChatCompletionChunk` items. Handles the
    /// chunk-split buffering issue (llama.cpp PR #9519) by buffering
    /// incomplete JSON across SSE events.
    pub async fn chat_completion_stream(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<std::pin::Pin<Box<dyn futures::Stream<Item = Result<ChatCompletionChunk>>>>> {
        let mut req = request.clone();
        req.stream = true;

        let resp = self
            .client
            .post(self.url("v1/chat/completions"))
            .json(&req)
            .send()
            .await
            .context("Failed to send streaming request")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "<no body>".into());
            anyhow::bail!("Streaming failed: {} – {}", status, body);
        }

        Ok(parse_sse_stream(resp.bytes_stream()))
    }
}

// ---------------------------------------------------------------------------
// SSE parsing with chunk-split buffering
// ---------------------------------------------------------------------------

/// Parse an SSE byte stream into `ChatCompletionChunk` items.
///
/// Handles:
/// - `data: [DONE]` termination signal
/// - Incomplete JSON across SSE events (PR #9519)
/// - Multi-line SSE data
fn parse_sse_stream(
    byte_stream: impl futures::Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Send + 'static,
) -> std::pin::Pin<Box<dyn futures::Stream<Item = Result<ChatCompletionChunk>> + Send>> {
    let mut lines = Box::pin(byte_stream);

    async_stream::stream! {
        let mut buffer = String::new();

        while let Some(chunk_result) = lines.next().await {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(e) => {
                    yield Err(anyhow::anyhow!("Stream read error: {}", e));
                    return;
                }
            };

            let text = match std::str::from_utf8(&chunk) {
                Ok(t) => t,
                Err(e) => {
                    yield Err(anyhow::anyhow!("Invalid UTF-8 in stream: {}", e));
                    return;
                }
            };
            buffer.push_str(text);

            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].trim().to_string();
                buffer = buffer[pos + 1..].to_string();

                if line.is_empty() || line.starts_with(':') {
                    continue;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    let data = data.trim();
                    if data == "[DONE]" {
                        return;
                    }

                    match serde_json::from_str::<ChatCompletionChunk>(data) {
                        Ok(chunk) => yield Ok(chunk),
                        Err(e) => {
                            yield Err(anyhow::anyhow!(
                                "Failed to parse SSE chunk: {} (data: {})",
                                e,
                                data
                            ));
                        }
                    }
                }
            }
        }
    }
    .boxed()
}
