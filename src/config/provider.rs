//! Provider configuration for unified workflow schema.
//!
//! Parses the `providers` section from the unified YAML schema.
//! Supports multiple provider types (LM Studio, Ollama, llama.cpp with Vulkan).
//!
//! # YAML Structure
//!
//! ```yaml
//! providers:
//!   llama_cpp_with_vulkan:
//!     config:
//!       host: "localhost"
//!       port: 1234
//!       connection_timeout_secs: 30
//!     hosting:
//!       max_concurrent_models: 2
//!       model_offload_timeout_secs: 300
//!       gpu_allocation:
//!         strategy: "priority"
//!         device_ids: [0]
//!         vram_per_model_mb: 4096
//!         max_gpu_utilization: 0.95
//!       cpu_fallback:
//!         enabled: true
//!         cpu_cores_per_model: 2
//!         ram_reservation_mb: 1024
//!     requests:
//!       max_concurrent_requests: 10
//!       request_timeout_secs: 120
//!       queue_timeout_secs: 300
//!       rate_limit_per_minute: 60
//!       retry:
//!         max_retries: 3
//!         backoff: exponential
//!         initial_delay: 1
//!         max_delay: 60
//!         multiplier: 2.0
//!         jitter: true
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Top-level providers config
// ---------------------------------------------------------------------------

/// Collection of provider configurations keyed by provider name.
///
/// Supports multiple providers (lmstudio, ollama, llama_cpp_with_vulkan).
/// Uses `#[serde(flatten)]` to directly map provider keys (e.g., "lmstudio") to this struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvidersConfig {
    /// Flattened provider configurations - keys become top-level fields.
    #[serde(flatten)]
    pub providers: HashMap<String, ProviderConfig>,
}

/// Provider configuration type alias for API compatibility.
/// All providers currently use the same structure (LlamaCppVulkanProvider).
pub type ProviderConfig = LlamaCppVulkanProvider;

// ---------------------------------------------------------------------------
// Llama.cpp with Vulkan provider
// ---------------------------------------------------------------------------

/// Complete configuration for llama.cpp server with Vulkan backend.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
#[serde(deny_unknown_fields)]
pub struct LlamaCppVulkanProvider {
    /// Connection settings (host, port, timeouts).
    #[serde(default)]
    #[garde(skip)]
    pub config: Option<LlamaCppConfig>,

    /// Hosting configuration (model management, GPU/CPU resources).
    #[serde(default)]
    #[garde(skip)]
    pub hosting: Option<HostingConfig>,

    /// Request handling (concurrency, timeouts, rate limiting, retry).
    #[serde(default)]
    #[garde(skip)]
    pub requests: Option<RequestsConfig>,
}

/// Llama.cpp connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
#[serde(deny_unknown_fields)]
pub struct LlamaCppConfig {
    /// Server host address.
    #[serde(default = "default_host")]
    #[garde(skip)]
    pub host: String,

    /// Server port number.
    #[serde(default = "default_port")]
    #[garde(range(min = 1, max = 65535))]
    pub port: u32,

    /// Connection timeout in seconds.
    #[serde(default = "default_connection_timeout")]
    #[garde(range(min = 1))]
    pub connection_timeout_secs: u64,
}

impl Default for LlamaCppConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            connection_timeout_secs: default_connection_timeout(),
        }
    }
}

/// Hosting configuration for model management and resource allocation.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
#[serde(deny_unknown_fields)]
pub struct HostingConfig {
    /// Maximum number of models loaded concurrently.
    #[serde(default = "default_max_concurrent_models")]
    #[garde(skip)]
    pub max_concurrent_models: usize,

    /// Timeout for offloading models (in seconds).
    #[serde(default = "default_model_offload_timeout")]
    #[garde(skip)]
    pub model_offload_timeout_secs: u64,

    /// GPU layers to offload (0 = CPU only, 999 = all layers). Sets LLAMA_ARG_N_GPU_LAYERS env var.
    #[serde(default = "default_gpu_layers")]
    #[garde(range(min = 0))]
    pub gpu_layers: usize,

    /// GPU allocation settings.
    #[serde(default)]
    #[garde(skip)]
    pub gpu_allocation: Option<GpuAllocation>,

    /// CPU fallback settings.
    #[serde(default)]
    #[garde(skip)]
    pub cpu_fallback: Option<CpuFallback>,
}

impl Default for HostingConfig {
    fn default() -> Self {
        Self {
            max_concurrent_models: default_max_concurrent_models(),
            model_offload_timeout_secs: default_model_offload_timeout(),
            gpu_layers: default_gpu_layers(),
            gpu_allocation: None,
            cpu_fallback: None,
        }
    }
}

/// GPU allocation strategy and limits.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
#[serde(deny_unknown_fields)]
pub struct GpuAllocation {
    /// Allocation strategy: priority, round_robin, fixed.
    #[serde(default = "default_gpu_strategy")]
    #[garde(skip)]
    pub strategy: String,

    /// GPU device IDs to use.
    #[serde(default = "default_device_ids")]
    #[garde(skip)]
    pub device_ids: Vec<u32>,

    /// VRAM allocation per model in MB.
    #[serde(default = "default_vram_per_model")]
    #[garde(range(min = 1))]
    pub vram_per_model_mb: usize,

    /// Maximum GPU utilization (0.0 to 1.0).
    #[serde(default = "default_max_gpu_utilization")]
    #[garde(range(min = 0.0, max = 1.0))]
    pub max_gpu_utilization: f64,
}

impl Default for GpuAllocation {
    fn default() -> Self {
        Self {
            strategy: default_gpu_strategy(),
            device_ids: default_device_ids(),
            vram_per_model_mb: default_vram_per_model(),
            max_gpu_utilization: default_max_gpu_utilization(),
        }
    }
}

/// CPU fallback settings when GPU is unavailable or overloaded.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
#[serde(deny_unknown_fields)]
pub struct CpuFallback {
    /// Enable CPU fallback.
    #[serde(default = "default_cpu_fallback_enabled")]
    #[garde(skip)]
    pub enabled: bool,

    /// CPU cores allocated per model.
    #[serde(default = "default_cpu_cores_per_model")]
    #[garde(range(min = 1))]
    pub cpu_cores_per_model: usize,

    /// RAM reservation per model in MB.
    #[serde(default = "default_ram_reservation")]
    #[garde(range(min = 1))]
    pub ram_reservation_mb: usize,
}

impl Default for CpuFallback {
    fn default() -> Self {
        Self {
            enabled: default_cpu_fallback_enabled(),
            cpu_cores_per_model: default_cpu_cores_per_model(),
            ram_reservation_mb: default_ram_reservation(),
        }
    }
}

/// Request handling configuration.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
#[serde(deny_unknown_fields)]
pub struct RequestsConfig {
    /// Maximum concurrent requests.
    #[serde(default = "default_max_concurrent_requests")]
    #[garde(skip)]
    pub max_concurrent_requests: usize,

    /// Request timeout in seconds.
    #[serde(default = "default_request_timeout")]
    #[garde(skip)]
    pub request_timeout_secs: u64,

    /// Queue timeout in seconds.
    #[serde(default = "default_queue_timeout")]
    #[garde(skip)]
    pub queue_timeout_secs: u64,

    /// Rate limit: max requests per minute.
    #[serde(default = "default_rate_limit")]
    #[garde(range(min = 1))]
    pub rate_limit_per_minute: usize,

    /// Retry policy configuration.
    #[serde(default)]
    #[garde(skip)]
    pub retry: Option<RetryPolicyConfig>,
}

impl Default for RequestsConfig {
    fn default() -> Self {
        Self {
            max_concurrent_requests: default_max_concurrent_requests(),
            request_timeout_secs: default_request_timeout(),
            queue_timeout_secs: default_queue_timeout(),
            rate_limit_per_minute: default_rate_limit(),
            retry: None,
        }
    }
}

/// Retry policy configuration.
///
/// Flattened structure matching schema: retry has flat keys, not nested backoff object.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
#[serde(deny_unknown_fields)]
pub struct RetryPolicyConfig {
    /// Maximum number of retry attempts.
    #[serde(default = "default_max_retries")]
    #[garde(skip)]
    pub max_retries: u32,

    /// Backoff strategy: exponential, linear, fixed.
    #[serde(default = "default_backoff_strategy")]
    #[garde(skip)]
    pub backoff: String,

    /// Initial delay in seconds.
    #[serde(default = "default_initial_delay")]
    #[garde(skip)]
    pub initial_delay: u64,

    /// Maximum delay in seconds.
    #[serde(default = "default_max_delay")]
    #[garde(skip)]
    pub max_delay: u64,

    /// Backoff multiplier (for exponential strategy).
    #[serde(default = "default_multiplier")]
    #[garde(skip)]
    pub multiplier: f64,

    /// Add random jitter to retry delays (prevents thundering herd).
    #[serde(default = "default_jitter")]
    #[garde(skip)]
    pub jitter: bool,
}

impl Default for RetryPolicyConfig {
    fn default() -> Self {
        Self {
            max_retries: default_max_retries(),
            backoff: default_backoff_strategy(),
            initial_delay: default_initial_delay(),
            max_delay: default_max_delay(),
            multiplier: default_multiplier(),
            jitter: default_jitter(),
        }
    }
}

// ---------------------------------------------------------------------------
// Default functions (matching schema defaults)
// ---------------------------------------------------------------------------

fn default_host() -> String {
    "localhost".into()
}

fn default_port() -> u32 {
    1234
}

fn default_connection_timeout() -> u64 {
    30
}

fn default_max_concurrent_models() -> usize {
    2
}

fn default_model_offload_timeout() -> u64 {
    300
}

fn default_gpu_layers() -> usize {
    999
}

fn default_gpu_strategy() -> String {
    "priority".into()
}

fn default_device_ids() -> Vec<u32> {
    vec![0]
}

fn default_vram_per_model() -> usize {
    4096
}

fn default_max_gpu_utilization() -> f64 {
    0.95
}

fn default_cpu_fallback_enabled() -> bool {
    true
}

fn default_cpu_cores_per_model() -> usize {
    2
}

fn default_ram_reservation() -> usize {
    1024
}

fn default_max_concurrent_requests() -> usize {
    10
}

fn default_request_timeout() -> u64 {
    120
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

fn default_backoff_strategy() -> String {
    "exponential".into()
}

fn default_initial_delay() -> u64 {
    1
}

fn default_max_delay() -> u64 {
    60
}

fn default_multiplier() -> f64 {
    2.0
}

fn default_jitter() -> bool {
    true
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_provider() {
        let yaml = r#"
llama_cpp_with_vulkan:
  config:
    host: localhost
    port: 1234
"#;
        let config: ProvidersConfig = serde_saphyr::from_str(yaml).expect("parse");
        assert!(config.providers.contains_key("llama_cpp_with_vulkan"));
    }

    #[test]
    fn parse_full_provider() {
        let yaml = r#"
llama_cpp_with_vulkan:
  config:
    host: localhost
    port: 1234
    connection_timeout_secs: 30
  hosting:
    max_concurrent_models: 2
    model_offload_timeout_secs: 300
    gpu_allocation:
      strategy: priority
      device_ids: [0]
      vram_per_model_mb: 4096
      max_gpu_utilization: 0.95
    cpu_fallback:
      enabled: true
      cpu_cores_per_model: 2
      ram_reservation_mb: 1024
  requests:
    max_concurrent_requests: 10
    request_timeout_secs: 120
    queue_timeout_secs: 300
    rate_limit_per_minute: 60
    retry:
      max_retries: 3
      backoff: exponential
      initial_delay: 1
      max_delay: 60
      multiplier: 2.0
      jitter: true
"#;
        let config: ProvidersConfig = serde_saphyr::from_str(yaml).expect("parse");
        assert!(config.providers.contains_key("llama_cpp_with_vulkan"));
    }

    #[test]
    fn defaults_apply() {
        let config = LlamaCppConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 1234);
        assert_eq!(config.connection_timeout_secs, 30);
    }

    #[test]
    fn validate_port_range() {
        let yaml = r#"
host: localhost
port: 99999
connection_timeout_secs: 30
"#;
        let config: LlamaCppConfig = serde_saphyr::from_str(yaml).expect("parse");
        let result = garde::Validate::validate(&config);
        assert!(result.is_err(), "Port > 65535 should fail validation");
    }
}
