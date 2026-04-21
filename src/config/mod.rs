//! YAML configuration schema for llama.cpp server.
//!
//! Provides strongly-typed configuration structs that map to llama.cpp
//! CLI flags and `LLAMA_ARG_*` environment variables.
//!
//! # Translation Layer
//!
//! ```text
//! YAML config → LlamaConfig struct → LLAMA_ARG_* env vars → llama.cpp CLI flags
//! ```
//!
//! # Defaults
//!
//! All defaults match llama.cpp upstream defaults where applicable.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Top-level config
// ---------------------------------------------------------------------------

/// Complete llama.cpp configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlamaConfig {
    pub model: ModelConfig,
    #[serde(default)]
    pub context: ContextConfig,
    #[serde(default)]
    pub hardware: HardwareConfig,
    #[serde(default)]
    pub sampling: SamplingConfig,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub cache: CacheConfig,
    #[serde(default)]
    pub retry_config: RetryConfig,
    #[serde(default)]
    pub docker: DockerConfig,
    #[serde(default)]
    pub features: FeaturesConfig,
    #[serde(default)]
    pub vulkan: VulkanConfig,
}

impl LlamaConfig {
    /// Load config from a YAML string.
    pub fn from_yaml(yaml: &str) -> anyhow::Result<Self> {
        Ok(serde_saphyr::from_str(yaml)?)
    }

    /// Load config from a file.
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        Self::from_yaml(&contents)
    }

    /// Translate config to `LLAMA_ARG_*` environment variables.
    pub fn to_env_vars(&self) -> Vec<(String, String)> {
        let mut vars = Vec::new();

        // Model
        vars.push((
            "LLAMA_ARG_MODEL_PATH".into(),
            self.model.path.display().to_string(),
        ));

        // Context
        vars.push(("LLAMA_ARG_CTX_SIZE".into(), self.context.size.to_string()));
        vars.push((
            "LLAMA_ARG_BATCH_SIZE".into(),
            self.context.batch_size.to_string(),
        ));
        vars.push((
            "LLAMA_ARG_UBATCH_SIZE".into(),
            self.context.ubatch_size.to_string(),
        ));

        // Hardware
        if self.hardware.threads > 0 {
            vars.push((
                "LLAMA_ARG_N_THREADS".into(),
                self.hardware.threads.to_string(),
            ));
        }
        vars.push((
            "LLAMA_ARG_N_GPU_LAYERS".into(),
            self.hardware.gpu_layers.to_string(),
        ));
        if !self.hardware.use_mmap {
            vars.push(("LLAMA_ARG_USE_MMAP".into(), "0".into()));
        }
        if self.hardware.lock_memory {
            vars.push(("LLAMA_ARG_LOCK_MEMORY".into(), "1".into()));
        }

        // Sampling
        vars.push((
            "LLAMA_ARG_TEMP".into(),
            format!("{:.2}", self.sampling.temperature),
        ));
        vars.push((
            "LLAMA_ARG_TOP_P".into(),
            format!("{:.2}", self.sampling.top_p),
        ));
        if self.sampling.top_k > 0 {
            vars.push(("LLAMA_ARG_TOP_K".into(), self.sampling.top_k.to_string()));
        }
        if let Some(min_p) = self.sampling.min_p {
            vars.push(("LLAMA_ARG_MIN_P".into(), format!("{:.2}", min_p)));
        }
        if self.sampling.repeat_penalty != 1.0 {
            vars.push((
                "LLAMA_ARG_REPEAT_PENALTY".into(),
                format!("{:.2}", self.sampling.repeat_penalty),
            ));
        }
        if let Some(pp) = self.sampling.presence_penalty {
            vars.push(("LLAMA_ARG_PRESENCE_PENALTY".into(), format!("{:.2}", pp)));
        }
        if let Some(fp) = self.sampling.frequency_penalty {
            vars.push(("LLAMA_ARG_FREQUENCY_PENALTY".into(), format!("{:.2}", fp)));
        }
        if self.sampling.repeat_last_n > 0 {
            vars.push((
                "LLAMA_ARG_REPEAT_LAST_N".into(),
                self.sampling.repeat_last_n.to_string(),
            ));
        }
        if self.sampling.seed > 0 {
            vars.push(("LLAMA_ARG_SEED".into(), self.sampling.seed.to_string()));
        }
        vars.push((
            "LLAMA_ARG_N_PREDICT".into(),
            self.sampling.max_tokens.to_string(),
        ));

        // Server
        vars.push(("LLAMA_ARG_HOST".into(), self.server.host.clone()));
        vars.push(("LLAMA_ARG_PORT".into(), self.server.port.to_string()));
        if self.server.parallel {
            vars.push(("LLAMA_ARG_PARALLEL".into(), "1".into()));
        }
        vars.push(("LLAMA_ARG_TIMEOUT".into(), self.server.timeout.to_string()));
        if self.server.max_slots > 0 {
            vars.push(("LLAMA_ARG_N_SLOT".into(), self.server.max_slots.to_string()));
        }
        if self.server.metrics {
            vars.push(("LLAMA_ARG_METRICS".into(), "1".into()));
        }
        if self.server.slots_endpoint {
            vars.push(("LLAMA_ARG_SLOTS_ENDPOINT".into(), "1".into()));
        }

        // Cache
        vars.push((
            "LLAMA_ARG_CACHE_TYPE_K".into(),
            format!("{:?}", self.cache.cache_type_k).to_lowercase(),
        ));
        vars.push((
            "LLAMA_ARG_CACHE_TYPE_V".into(),
            format!("{:?}", self.cache.cache_type_v).to_lowercase(),
        ));

        // Features
        vars.push((
            "LLAMA_ARG_LOG_LEVEL".into(),
            match self.features.log_level {
                LogLevel::Trace => "0",
                LogLevel::Debug => "1",
                LogLevel::Info => "2",
                LogLevel::Warn => "3",
                LogLevel::Error => "4",
            }
            .into(),
        ));

        // Vulkan (GGML_VK_* prefix)
        vars.push((
            "GGML_VK_VISIBLE_DEVICES".into(),
            self.vulkan.visible_devices.clone(),
        ));
        if self.vulkan.disable_debug {
            vars.push(("GGML_VK_DISABLE_DEBUG".into(), "1".into()));
        }

        vars
    }
}

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// Model configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Local path to GGUF model file.
    pub path: PathBuf,
    /// HuggingFace download specification (optional).
    #[serde(default)]
    pub huggingface: Option<HuggingFaceConfig>,
    /// Quantization format (for validation).
    #[serde(default = "default_quantization")]
    pub quantization: String,
    /// Expected parameter count (for validation).
    #[serde(default)]
    pub parameter_count: Option<u64>,
}

/// HuggingFace download configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuggingFaceConfig {
    /// HuggingFace repository (org/model).
    pub repo: String,
    /// Filename in repository.
    pub filename: String,
    /// Git branch (default: main).
    #[serde(default = "default_branch")]
    pub branch: String,
    /// SHA256 checksum for validation (optional).
    #[serde(default)]
    pub sha256: Option<String>,
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

/// Context window configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    /// Token context size (default from llama.cpp: 2048).
    #[serde(default = "default_ctx_size")]
    pub size: usize,
    /// Batch size (default from llama.cpp: 2048).
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    /// Maximum context per slot.
    #[serde(default)]
    pub max_context_per_slot: Option<usize>,
    /// Micro-batch size (default from llama.cpp: 512).
    #[serde(default = "default_ubatch_size")]
    pub ubatch_size: usize,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            size: default_ctx_size(),
            batch_size: default_batch_size(),
            max_context_per_slot: None,
            ubatch_size: default_ubatch_size(),
        }
    }
}

// ---------------------------------------------------------------------------
// Hardware
// ---------------------------------------------------------------------------

/// Hardware configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfig {
    /// CPU threads (0 = auto-detect).
    #[serde(default = "default_threads")]
    pub threads: usize,
    /// GPU layers to offload (0 = CPU only, 999 = all layers).
    #[serde(default = "default_gpu_layers")]
    pub gpu_layers: usize,
    /// Use mmap for model loading.
    #[serde(default = "default_use_mmap")]
    pub use_mmap: bool,
    /// Lock memory (prevent swap).
    #[serde(default)]
    pub lock_memory: bool,
}

impl Default for HardwareConfig {
    fn default() -> Self {
        Self {
            threads: default_threads(),
            gpu_layers: default_gpu_layers(),
            use_mmap: default_use_mmap(),
            lock_memory: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Sampling
// ---------------------------------------------------------------------------

/// Sampling parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingConfig {
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    #[serde(default = "default_top_k")]
    pub top_k: usize,
    #[serde(default)]
    pub min_p: Option<f32>,
    #[serde(default = "default_typical_p")]
    pub typical_p: f32,
    #[serde(default = "default_repeat_penalty")]
    pub repeat_penalty: f32,
    #[serde(default)]
    pub presence_penalty: Option<f32>,
    #[serde(default)]
    pub frequency_penalty: Option<f32>,
    #[serde(default = "default_repeat_last_n")]
    pub repeat_last_n: usize,
    #[serde(default = "default_seed")]
    pub seed: u32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
}

impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            temperature: default_temperature(),
            top_p: default_top_p(),
            top_k: default_top_k(),
            min_p: None,
            typical_p: default_typical_p(),
            repeat_penalty: default_repeat_penalty(),
            presence_penalty: None,
            frequency_penalty: None,
            repeat_last_n: default_repeat_last_n(),
            seed: default_seed(),
            max_tokens: default_max_tokens(),
        }
    }
}

/// Server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_parallel")]
    pub parallel: bool,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    #[serde(default = "default_max_slots")]
    pub max_slots: usize,
    #[serde(default = "default_metrics")]
    pub metrics: bool,
    #[serde(default = "default_slots_endpoint")]
    pub slots_endpoint: bool,
    #[serde(default = "default_cors_origins")]
    pub cors_origins: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub access_log: Option<PathBuf>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            parallel: default_parallel(),
            timeout: default_timeout(),
            max_slots: default_max_slots(),
            metrics: default_metrics(),
            slots_endpoint: default_slots_endpoint(),
            cors_origins: default_cors_origins(),
            api_key: None,
            access_log: None,
        }
    }
}

/// KV cache configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default = "default_cache_type")]
    pub cache_type_k: CacheType,
    #[serde(default = "default_cache_type")]
    pub cache_type_v: CacheType,
    #[serde(default)]
    pub kv_cache_size: Option<usize>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_type_k: default_cache_type(),
            cache_type_v: default_cache_type(),
            kv_cache_size: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheType {
    F32,
    F16,
    Q8_0,
    Q4_0,
}

impl Default for CacheType {
    fn default() -> Self {
        CacheType::F16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

/// Retry configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_retry_delay_seconds")]
    pub retry_delay_seconds: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: default_timeout_seconds(),
            max_retries: default_max_retries(),
            retry_delay_seconds: default_retry_delay_seconds(),
        }
    }
}

/// Docker resource limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerConfig {
    #[serde(default)]
    pub memory_limit: Option<usize>,
    #[serde(default = "default_shm_size")]
    pub shm_size: usize,
    #[serde(default)]
    pub cpu_count: Option<usize>,
}

impl Default for DockerConfig {
    fn default() -> Self {
        Self {
            memory_limit: None,
            shm_size: default_shm_size(),
            cpu_count: None,
        }
    }
}

/// Feature flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,
    #[serde(default)]
    pub profiling: bool,
    #[serde(default = "default_print_system_info")]
    pub print_system_info: bool,
    #[serde(default)]
    pub verbose: bool,
    #[serde(default = "default_color")]
    pub color: bool,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            profiling: false,
            print_system_info: default_print_system_info(),
            verbose: false,
            color: default_color(),
        }
    }
}

// ---------------------------------------------------------------------------
// Vulkan
// ---------------------------------------------------------------------------

/// Vulkan-specific configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulkanConfig {
    #[serde(default = "default_visible_devices")]
    pub visible_devices: String,
    #[serde(default)]
    pub force_max_allocation: Option<usize>,
    #[serde(default = "default_disable_debug")]
    pub disable_debug: bool,
    #[serde(default)]
    pub enable_validation: bool,
    /// Enable flash attention.
    /// NOTE: Disable for AMD Polaris GPUs (issue #20465).
    #[serde(default)]
    pub flash_attention: Option<bool>,
}

impl Default for VulkanConfig {
    fn default() -> Self {
        Self {
            visible_devices: default_visible_devices(),
            force_max_allocation: None,
            disable_debug: default_disable_debug(),
            enable_validation: false,
            flash_attention: None,
        }
    }
}

// ===========================================================================
// Defaults (matching llama.cpp upstream)
// ===========================================================================

fn default_quantization() -> String {
    "Q4_K_M".into()
}
fn default_branch() -> String {
    "main".into()
}
fn default_ctx_size() -> usize {
    2048
}
fn default_batch_size() -> usize {
    2048
}
fn default_ubatch_size() -> usize {
    512
}
fn default_threads() -> usize {
    4
}
fn default_gpu_layers() -> usize {
    999
}
fn default_use_mmap() -> bool {
    true
}
fn default_temperature() -> f32 {
    0.80
}
fn default_top_p() -> f32 {
    0.95
}
fn default_top_k() -> usize {
    40
}
fn default_typical_p() -> f32 {
    1.0
}
fn default_repeat_penalty() -> f32 {
    1.00
}
fn default_repeat_last_n() -> usize {
    64
}
fn default_seed() -> u32 {
    0
}
fn default_max_tokens() -> usize {
    512
}
fn default_host() -> String {
    "127.0.0.1".into()
}
fn default_port() -> u16 {
    8080
}
fn default_parallel() -> bool {
    false
}
fn default_timeout() -> u64 {
    600
}
fn default_max_slots() -> usize {
    8
}
fn default_metrics() -> bool {
    true
}
fn default_slots_endpoint() -> bool {
    true
}
fn default_cache_type() -> CacheType {
    CacheType::F16
}
fn default_log_level() -> LogLevel {
    LogLevel::Info
}
fn default_print_system_info() -> bool {
    true
}
fn default_color() -> bool {
    true
}
fn default_visible_devices() -> String {
    "0".into()
}
fn default_disable_debug() -> bool {
    true
}
fn default_cors_origins() -> String {
    "http://localhost:8080".into()
}
fn default_timeout_seconds() -> u64 {
    300
}
fn default_max_retries() -> u32 {
    3
}
fn default_retry_delay_seconds() -> u64 {
    5
}
fn default_shm_size() -> usize {
    8
}

// ===========================================================================
// Unit tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_config() {
        let yaml = r#"
model:
  path: /models/test.gguf
"#;
        let config: LlamaConfig = serde_saphyr::from_str(yaml).expect("parse");
        assert_eq!(config.model.path, PathBuf::from("/models/test.gguf"));
        assert_eq!(config.context.size, 2048);
        assert_eq!(config.sampling.temperature, 0.80);
        assert_eq!(config.server.port, 8080);
    }

    #[test]
    fn env_var_translation() {
        let yaml = r#"
model:
  path: /models/test.gguf
context:
  size: 4096
sampling:
  temperature: 0.7
"#;
        let config: LlamaConfig = serde_saphyr::from_str(yaml).expect("parse");
        let vars = config.to_env_vars();

        let get = |key: &str| -> Option<String> {
            vars.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
        };

        assert_eq!(
            get("LLAMA_ARG_MODEL_PATH"),
            Some("/models/test.gguf".into())
        );
        assert_eq!(get("LLAMA_ARG_CTX_SIZE"), Some("4096".into()));
        assert_eq!(get("LLAMA_ARG_TEMP"), Some("0.70".into()));
    }
}
