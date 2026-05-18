# Provider Configuration Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement provider configuration layer mapping unified-workflow-schema.yml `providers.llama_cpp_with_vulkan` to Rust structs with serde-saphyr parsing (with DoS budgets, merge keys), garde validation, and LlmBackend trait.

**Architecture:** Provider section → Rust structs → LlmBackend trait → LlamaCppVulkanBackend implementation. Replace flat config.yml with providers-based hierarchy.

**Tech Stack:** serde-saphyr (parsing), garde (validation), tokio (async), anyhow (errors).

**Prerequisites:** None (first sub-plan)

---

## Scope

### Unified Schema Sections (unified-workflow-schema.yml lines 27-51)

```yaml
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
      connection_timeout_secs: 30
    hosting:
      max_concurrent_models: 1
      model_offload_timeout_secs: 60
      gpu_allocation:
        vram_per_model_mb: 4096
      cpu_fallback:
        cpu_cores_per_model: 4
    requests:
      max_concurrent_requests: 1
      request_timeout_secs: 600
      queue_timeout_secs: 300
      rate_limit_per_minute: 60
      retry:
        max_retries: 3
        backoff: exponential
        initial_delay: "1s"
        max_delay: "30s"
        multiplier: 2.0
        jitter: true
```

### IN Scope
- Only `llama_cpp_with_vulkan` provider type
- Parse providers section with serde-saphyr
- Validate with garde (range constraints, required fields)
- Config resolution: providers → per-model overrides → defaults
- LlmBackend trait definition
- LlamaCppVulkanBackend implementation
- Refactor LlamaHttpClient to use backend trait

### OUT of Scope
- Other provider types (lmstudio, ollama)
- Provider config files (config_file: "./path.yml")
- Inline config vs file config selection
- Provider-level tools or permissions

---

## Files to Create

### 1. src/config/provider.rs

**Purpose:** Provider configuration structs mapping to unified schema.

```rust
use garde::Validate;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Provider configuration mapping to unified schema providers section.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
pub struct ProviderConfig {
    /// Schema version (from unified-workflow-schema.yml line 804).
    #[serde(default = "default_schema_version")]
    #[garde(skip)]
    pub schema_version: String,

    /// Provider type (only llama_cpp_with_vulkan in POC).
    #[serde(rename = "llama_cpp_with_vulkan")]
    pub llama_cpp_with_vulkan: Option<LlamaCppVulkanProvider>,
}

/// llama.cpp with Vulkan provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlamaCppVulkanProvider {
    /// Inline config OR config_file: "./path.yml" (POC: inline only).
    pub config: Option<LlamaCppConfig>,
    pub config_file: Option<String>,
}

/// llama.cpp connection and server configuration.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
pub struct LlamaCppConfig {
    /// Connection settings.
    #[serde(default)]
    pub config: ConnectionConfig,

    /// Hosting settings.
    #[serde(default)]
    pub hosting: HostingConfig,

    /// Request settings.
    #[serde(default)]
    pub requests: RequestsConfig,
}

/// Connection configuration (host, port, timeout).
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
pub struct ConnectionConfig {
    #[serde(default = "default_host")]
    #[garde(skip)]
    pub host: String,

    #[serde(default = "default_port")]
    #[garde(range(min = 1, max = 65535))]
    pub port: u16,

    #[serde(default = "default_connection_timeout")]
    #[garde(range(min = 1))]
    pub connection_timeout_secs: u64,
}

/// Hosting configuration (concurrency, GPU allocation, CPU fallback).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostingConfig {
    #[serde(default = "default_max_concurrent_models")]
    #[garde(range(min = 1))]
    pub max_concurrent_models: usize,

    #[serde(default = "default_model_offload_timeout")]
    #[garde(range(min = 1))]
    pub model_offload_timeout_secs: u64,

    /// Presence = GPU allocation enabled.
    pub gpu_allocation: Option<GpuAllocation>,

    /// Presence = CPU fallback enabled.
    pub cpu_fallback: Option<CpuFallback>,
}

/// GPU allocation settings.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
pub struct GpuAllocation {
    #[serde(default = "default_vram_per_model")]
    #[garde(range(min = 1))]
    pub vram_per_model_mb: usize,
}

/// CPU fallback settings.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
pub struct CpuFallback {
    #[serde(default = "default_cpu_cores")]
    #[garde(range(min = 1))]
    pub cpu_cores_per_model: usize,
}

/// Request configuration (concurrency, timeouts, rate limiting, retry).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestsConfig {
    #[serde(default = "default_max_concurrent_requests")]
    #[garde(range(min = 1))]
    pub max_concurrent_requests: usize,

    #[serde(default = "default_request_timeout")]
    #[garde(range(min = 1))]
    pub request_timeout_secs: u64,

    #[serde(default = "default_queue_timeout")]
    #[garde(range(min = 1))]
    pub queue_timeout_secs: u64,

    #[serde(default = "default_rate_limit")]
    #[garde(range(min = 1))]
    pub rate_limit_per_minute: usize,

    /// Presence = retry enabled.
    pub retry: Option<RetryConfig>,
}

/// Retry configuration (backoff strategy).
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
pub struct RetryConfig {
    #[serde(default = "default_max_retries")]
    #[garde(range(min = 0))]
    pub max_retries: u32,

    #[serde(default = "default_backoff")]
    #[garde(skip)]
    pub backoff: BackoffStrategy,

    #[serde(default = "default_initial_delay")]
    pub initial_delay: String,

    #[serde(default = "default_max_delay")]
    pub max_delay: String,

    #[serde(default = "default_multiplier")]
    #[garde(range(min = 1.0))]
    pub multiplier: f64,

    #[serde(default = "default_jitter")]
    pub jitter: bool,
}

/// Backoff strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackoffStrategy {
    Exponential,
    Linear,
    Fixed,
}

// Defaults (matching unified schema defaults)
fn default_schema_version() -> String {
    "2.0.0".into()
}

fn default_host() -> String {
    "localhost".into()
}

fn default_port() -> u16 {
    8080
}

fn default_connection_timeout() -> u64 {
    30
}

fn default_max_concurrent_models() -> usize {
    1
}

fn default_model_offload_timeout() -> u64 {
    60
}

fn default_vram_per_model() -> usize {
    4096
}

fn default_cpu_cores() -> usize {
    4
}

fn default_max_concurrent_requests() -> usize {
    1
}

fn default_request_timeout() -> u64 {
    600
}

fn default_queue_timeout() -> u64 {
    300
}

fn default_rate_limit() -> usize {
    60
}

fn default_max_retries() -> u32 {
    3
}

fn default_backoff() -> BackoffStrategy {
    BackoffStrategy::Exponential
}

fn default_initial_delay() -> String {
    "1s".into()
}

fn default_max_delay() -> String {
    "30s".into()
}

fn default_multiplier() -> f64 {
    2.0
}

fn default_jitter() -> bool {
    true
}
```

### 2. src/backend/llm_backend.rs

**Purpose:** LlmBackend trait definition for provider abstraction.

```rust
use async_trait::async_trait;
use std::pin::Pin;
use std::time::Duration;

use super::client::types::{ChatCompletionRequest, ChatCompletionResponse, Message};

/// LLM backend capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendCapabilities {
    pub streaming: bool,
    pub tools: bool,
    pub function_calling: bool,
}

/// Health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// LLM backend errors.
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Model error: {0}")]
    Model(String),

    #[error("Rate limited: retry after {0:?}")]
    RateLimited(Option<Duration>),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Trait for LLM backend implementations.
#[async_trait]
pub trait LlmBackend: Send + Sync {
    /// Send chat completion request.
    async fn chat(&self, request: ChatCompletionRequest)
        -> Result<ChatCompletionResponse, LlmError>;

    /// Send streaming chat completion request.
    async fn chat_stream(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<
        Pin<Box<dyn futures::Stream<Item = Result<String, LlmError>> + Send>>,
        LlmError,
    >;

    /// List available models.
    async fn list_models(&self) -> Result<Vec<String>, LlmError>;

    /// Check backend health.
    async fn health_check(&self) -> Result<HealthStatus, LlmError>;

    /// Load model into memory.
    async fn load_model(&self, model_id: &str) -> Result<(), LlmError>;

    /// Unload model from memory.
    async fn unload_model(&self, model_id: &str) -> Result<(), LlmError>;

    /// Get backend capabilities.
    fn capabilities(&self) -> BackendCapabilities;

    /// Get base URL.
    fn base_url(&self) -> &str;
}
```

### 3. src/backend/llama_vulkan.rs

**Purpose:** LlamaCppVulkanBackend implementation (refactor from LlamaHttpClient).

```rust
use super::client::types::{ChatCompletionRequest, ChatCompletionResponse};
use super::llm_backend::{BackendCapabilities, HealthStatus, LlmBackend, LlmError};
use super::config::provider::{LlamaCppConfig, RetryConfig, BackoffStrategy};

use async_trait::async_trait;
use reqwest::Client;
use std::pin::Pin;
use std::time::Duration;

/// llama.cpp with Vulkan backend implementation.
pub struct LlamaCppVulkanBackend {
    base_url: String,
    client: Client,
    timeout: Duration,
    model: String,
}

impl LlamaCppVulkanBackend {
    /// Create new backend from provider config.
    pub fn new(config: &LlamaCppConfig, model: String) -> Self {
        let base_url = format!(
            "http://{}:{}",
            config.config.host, config.config.port
        );

        let timeout = Duration::from_secs(config.requests.request_timeout_secs);
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            base_url,
            client,
            timeout,
            model,
        }
    }

    /// Parse duration string (e.g., "1s", "30s").
    fn parse_duration(s: &str) -> Result<Duration, LlmError> {
        let s = s.trim().to_lowercase();
        let num: u64 = s
            .trim_end_matches('s')
            .parse()
            .map_err(|e| LlmError::Parse(format!("Invalid duration: {}", e)))?;

        Ok(Duration::from_secs(num))
    }

    /// Calculate retry delay based on backoff strategy.
    fn calculate_delay(
        retry_config: &RetryConfig,
        attempt: u32,
    ) -> Duration {
        let base_delay =
            Self::parse_duration(&retry_config.initial_delay)
                .unwrap_or_else(|_| Duration::from_secs(1));

        let max_delay =
            Self::parse_duration(&retry_config.max_delay)
                .unwrap_or_else(|_| Duration::from_secs(30));

        let delay_secs = match retry_config.backoff {
            BackoffStrategy::Exponential => {
                base_delay.as_secs() * retry_config.multiplier.powi(attempt as i32 - 1) as u64
            }
            BackoffStrategy::Linear => {
                base_delay.as_secs() * attempt as u64
            }
            BackoffStrategy::Fixed => base_delay.as_secs(),
        };

        let delay = Duration::from_secs(delay_secs.min(max_delay.as_secs()));

        // Add jitter if enabled
        if retry_config.jitter {
            let jitter_ms = fastrand::u64(0..=delay.as_millis() as u64 / 10);
            Duration::from_millis(delay.as_millis() as u64 + jitter_ms)
        } else {
            delay
        }
    }
}

#[async_trait]
impl LlmBackend for LlamaCppVulkanBackend {
    async fn chat(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmError> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let mut request_body = request;
        request_body.model = self.model.clone();

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| LlmError::Connection(e.to_string()))?;

        if response.status().is_server_error() || response.status()is_429() {
            return Err(LlmError::RateLimited(None));
        }

        response
            .json()
            .await
            .map_err(|e| LlmError::Parse(e.to_string()))
    }

    async fn chat_stream(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<
        Pin<Box<dyn futures::Stream<Item = Result<String, LlmError>> + Send>>,
        LlmError,
    > {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let mut request_body = request;
        request_body.model = self.model.clone();
        request_body.stream = Some(true);

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| LlmError::Connection(e.to_string()))?;

        use futures::StreamExt;
        use sseer::Event;

        let stream = response
            .bytes_stream()
            .map(|result| {
                result
                    .map_err(|e| LlmError::Connection(e.to_string()))
                    .and_then(|bytes| {
                        let text = std::str::from_utf8(&bytes)
                            .map_err(|e| LlmError::Parse(e.to_string()))?;

                        match Event::parse(text) {
                            Ok(Event::Message(msg)) => Ok(msg.data),
                            Ok(Event::Done) => Ok(String::new()),
                            Err(e) => Err(LlmError::Parse(e.to_string())),
                        }
                    })
            });

        Ok(Box::pin(stream))
    }

    async fn list_models(&self) -> Result<Vec<String>, LlmError> {
        let url = format!("{}/v1/models", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| LlmError::Connection(e.to_string()))?;

        #[derive(serde::Deserialize)]
        struct ModelsResponse {
            data: Vec<ModelEntry>,
        }

        #[derive(serde::Deserialize)]
        struct ModelEntry {
            id: String,
        }

        let models: ModelsResponse = response
            .json()
            .await
            .map_err(|e| LlmError::Parse(e.to_string()))?;

        Ok(models.data.into_iter().map(|m| m.id).collect())
    }

    async fn health_check(&self) -> Result<HealthStatus, LlmError> {
        let url = format!("{}/health", self.base_url);

        let response = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await;

        match response {
            Ok(r) if r.status().is_success() => Ok(HealthStatus::Healthy),
            Ok(r) if r.status().is_server_error() => Ok(HealthStatus::Degraded),
            Ok(_) => Ok(HealthStatus::Unhealthy),
            Err(e) if e.is_timeout() => Ok(HealthStatus::Degraded),
            Err(e) => Err(LlmError::Connection(e.to_string())),
        }
    }

    async fn load_model(&self, model_id: &str) -> Result<(), LlmError> {
        let url = format!("{}/v1/models/load", self.base_url);

        #[derive(serde::Serialize)]
        struct LoadRequest {
            model: String,
        }

        let response = self
            .client
            .post(&url)
            .json(&LoadRequest {
                model: model_id.to_string(),
            })
            .send()
            .await
            .map_err(|e| LlmError::Connection(e.to_string()))?;

        if !response.status().is_success() {
            return Err(LlmError::Model(format!(
                "Failed to load model {}: {}",
                model_id,
                response.status()
            )));
        }

        Ok(())
    }

    async fn unload_model(&self, model_id: &str) -> Result<(), LlmError> {
        let url = format!("{}/v1/models/unload", self.base_url);

        #[derive(serde::Serialize)]
        struct UnloadRequest {
            model: String,
        }

        let response = self
            .client
            .post(&url)
            .json(&UnloadRequest {
                model: model_id.to_string(),
            })
            .send()
            .await
            .map_err(|e| LlmError::Connection(e.to_string()))?;

        if !response.status().is_success() {
            return Err(LlmError::Model(format!(
                "Failed to unload model {}: {}",
                model_id,
                response.status()
            )));
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

    fn base_url(&self) -> &str {
        &self.base_url
    }
}
```

### 4. tests/provider_config_test.rs

**Purpose:** Unit tests for provider config parsing and validation.

```rust
use whitt::config::provider::{ProviderConfig, RetryConfig, BackoffStrategy};

#[test]
fn parse_minimal_provider_config() {
    let yaml = r#"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080
      connection_timeout_secs: 30
    hosting:
      max_concurrent_models: 1
    requests:
      max_concurrent_requests: 1
      request_timeout_secs: 600
"#;

    let config: ProviderConfig = serde_saphyr::from_str(yaml).expect("parse");
    assert!(config.llama_cpp_with_vulkan.is_some());
}

#[test]
fn parse_retry_config_with_backoff() {
    let yaml = r#"
providers:
  llama_cpp_with_vulkan:
    config:
      requests:
        retry:
          max_retries: 3
          backoff: exponential
          initial_delay: "1s"
          max_delay: "30s"
          multiplier: 2.0
          jitter: true
"#;

    let config: ProviderConfig = serde_saphyr::from_str(yaml).expect("parse");
    let provider = config.llama_cpp_with_vulkan.unwrap();
    let requests = provider.config.unwrap().requests;
    let retry = requests.retry.unwrap();

    assert_eq!(retry.max_retries, 3);
    assert_eq!(retry.backoff, BackoffStrategy::Exponential);
    assert_eq!(retry.initial_delay, "1s");
    assert_eq!(retry.max_delay, "30s");
    assert_eq!(retry.multiplier, 2.0);
    assert!(retry.jitter);
}

#[test]
fn garde_validation_rejects_invalid_port() {
    let yaml = r#"
providers:
  llama_cpp_with_vulkan:
    config:
      config:
        port: 99999
"#;

    let result: Result<ProviderConfig, _> = serde_saphyr::from_str(yaml).map_err(Into::into);
    assert!(result.is_err(), "Should reject port > 65535");
}

#[test]
fn garde_validation_rejects_negative_timeout() {
    let yaml = r#"
providers:
  llama_cpp_with_vulkan:
    config:
      config:
        connection_timeout_secs: -1
"#;

    let result: Result<ProviderConfig, _> = serde_saphyr::from_str(yaml).map_err(Into::into);
    assert!(result.is_err(), "Should reject negative timeout");
}
```

---

## Files to Modify

### 1. src/config/mod.rs

**Purpose:** Export provider config, keep backward compat with LlamaConfig.

```rust
pub mod provider;
pub mod unified;

pub use provider::{ProviderConfig, LlamaCppVulkanProvider, RetryConfig, BackoffStrategy};
pub use unified::{WorkflowSpec, ModelsConfig, StepConfig};

// Keep for migration path
#[deprecated(note = "Use ProviderConfig instead")]
pub use self::LlamaConfig;
```

### 2. src/client/mod.rs

**Purpose:** Keep OpenAI-compatible types, export backend trait.

```rust
pub mod types;
pub mod llama_http_client;

pub use types::{ChatCompletionRequest, ChatCompletionResponse, Message, Tool, ToolCall};

// Backend trait for provider abstraction
pub use crate::backend::llm_backend::{LlmBackend, LlmError, BackendCapabilities, HealthStatus};
```

---

## Implementation Tasks

### Task 1: Create provider config structs

**Files:**
- Create: `src/config/provider.rs`
- Create: `src/config/mod.rs` (if not exists)

- [ ] **Step 1: Write provider.rs with all structs**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Add garde dependency to Cargo.toml**

Run: `cargo add garde async_trait futures sseer fastrand`
Expected: Add dependencies

- [ ] **Step 3: Verify provider.rs compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 4: Run unit tests**

Run: `cargo test provider_config_test`
Expected: All PASS

- [ ] **Step 5: Commit**

```bash
git add src/config/provider.rs src/config/mod.rs Cargo.toml tests/provider_config_test.rs
git commit -m "feat: add provider config layer with serde-saphyr and garde validation"
```

### Task 2: Define LlmBackend trait

**Files:**
- Create: `src/backend/llm_backend.rs`
- Modify: `src/client/mod.rs` (export trait)

- [ ] **Step 1: Write llm_backend.rs with trait definition**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Export trait in client/mod.rs**

```rust
pub use crate::backend::llm_backend::{LlmBackend, LlmError, BackendCapabilities, HealthStatus};
```

- [ ] **Step 3: Verify trait compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/backend/llm_backend.rs src/client/mod.rs
git commit -m "feat: define LlmBackend trait for provider abstraction"
```

### Task 3: Implement LlamaCppVulkanBackend

**Files:**
- Create: `src/backend/llama_vulkan.rs`
- Create: `src/backend/mod.rs`

- [ ] **Step 1: Write llama_vulkan.rs with implementation**

```rust
// Copy from Files to Create section above
```

- [ ] **Step 2: Create backend/mod.rs**

```rust
pub mod llm_backend;
pub mod llama_vulkan;

pub use llm_backend::{LlmBackend, LlmError, BackendCapabilities, HealthStatus};
pub use llama_vulkan::LlamaCppVulkanBackend;
```

- [ ] **Step 3: Verify implementation compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 4: Run backend tests**

Run: `cargo test llm_backend`
Expected: All PASS

- [ ] **Step 5: Commit**

```bash
git add src/backend/llama_vulkan.rs src/backend/mod.rs
git commit -m "feat: implement LlamaCppVulkanBackend from LlamaHttpClient refactor"
```

### Task 4: Refactor config loader

**Files:**
- Modify: `src/config/mod.rs` (keep ConfigLoader)

- [ ] **Step 1: Add provider config resolution to ConfigLoader**

```rust
// In src/config/mod.rs ConfigLoader impl
impl ConfigLoader {
    /// Load provider config from unified YAML.
    pub fn load_provider(yaml: &str) -> anyhow::Result<ProviderConfig> {
        let provider: ProviderConfig = serde_saphyr::from_str(yaml)?;
        Ok(provider)
    }
}
```

- [ ] **Step 2: Verify config loader compiles**

Run: `cargo check --lib`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/config/mod.rs
git commit -m "feat: add provider config resolution to ConfigLoader"
```

---

## Success Criteria

- [ ] Provider config structs match unified schema lines 27-51
- [ ] All structs use garde validation annotations
- [ ] LlmBackend trait defined with all methods (chat, chat_stream, list_models, health_check, load, unload)
- [ ] LlamaCppVulkanBackend implements LlmBackend trait
- [ ] All values from YAML, no hardcoded defaults in backend impl
- [ ] Config resolution order: providers → per-model → defaults
- [ ] Unit tests pass (cargo test provider_config_test)
- [ ] LSP diagnostics clean on all changed files
- [ ] Build passes (cargo build)
