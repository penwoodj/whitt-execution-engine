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
};

const MAX_RETRIES: usize = 5;
const RETRY_DELAY: Duration = Duration::from_secs(1);

fn is_retryable(err: &anyhow::Error) -> bool {
    let targets = [
        "connection closed before message completed",
        "ConnectionRefused",
        "connect error",
        "503",
        "IncompleteMessage",
    ];
    let mut chain = err.chain();
    while let Some(source) = chain.next() {
        let msg = source.to_string();
        if targets.iter().any(|t| msg.contains(t)) {
            return true;
        }
    }
    false
}

/// HTTP client for llama-server.
pub struct LlamaHttpClient {
    client: Client,
    base_url: String,
}

impl LlamaHttpClient {
    /// Create a new HTTP client.
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let client = ClientBuilder::new()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(300))
            .pool_idle_timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.into(),
        })
    }

    fn url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint)
    }

    // -----------------------------------------------------------------------
    // Health / info
    // -----------------------------------------------------------------------

    /// Health check.
    pub async fn health(&self) -> Result<HealthResponse> {
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
