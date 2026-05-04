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

pub mod loop_config;
pub mod provider;
pub mod unified;

use garde::Validate;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
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

impl Default for LlamaConfig {
    fn default() -> Self {
        Self {
            model: ModelConfig {
                path: PathBuf::from(""),
                huggingface: None,
                quantization: default_quantization(),
                parameter_count: None,
            },
            context: ContextConfig::default(),
            hardware: HardwareConfig::default(),
            sampling: SamplingConfig::default(),
            server: ServerConfig::default(),
            cache: CacheConfig::default(),
            retry_config: RetryConfig::default(),
            docker: DockerConfig::default(),
            features: FeaturesConfig::default(),
            vulkan: VulkanConfig::default(),
        }
    }
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

    /// Validate config values. Returns error for out-of-range values.
    pub fn validate_config(&self) -> anyhow::Result<()> {
        if let Err(e) = Validate::validate(&self.sampling) {
            anyhow::bail!("Sampling config validation failed: {}", e);
        }
        if let Err(e) = Validate::validate(&self.context) {
            anyhow::bail!("Context config validation failed: {}", e);
        }
        Ok(())
    }

    /// Translate config to `LLAMA_ARG_*` environment variables.
    pub fn to_env_vars(&self) -> Vec<(String, String)> {
        let mut vars = vec![
            // Model
            (
                "LLAMA_ARG_MODEL_PATH".into(),
                self.model.path.display().to_string(),
            ),
            // Context
            ("LLAMA_ARG_CTX_SIZE".into(), self.context.size.to_string()),
            (
                "LLAMA_ARG_BATCH_SIZE".into(),
                self.context.batch_size.to_string(),
            ),
            (
                "LLAMA_ARG_UBATCH_SIZE".into(),
                self.context.ubatch_size.to_string(),
            ),
        ];

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
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
pub struct ContextConfig {
    #[serde(default = "default_ctx_size")]
    #[garde(range(min = 1))]
    pub size: usize,
    #[serde(default = "default_batch_size")]
    #[garde(range(min = 1))]
    pub batch_size: usize,
    #[serde(default)]
    #[garde(skip)]
    pub max_context_per_slot: Option<usize>,
    #[serde(default = "default_ubatch_size")]
    #[garde(range(min = 1))]
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
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
pub struct SamplingConfig {
    #[serde(default = "default_temperature")]
    #[garde(range(min = 0.0, max = 2.0))]
    pub temperature: f32,
    #[serde(default = "default_top_p")]
    #[garde(range(min = 0.0, max = 1.0))]
    pub top_p: f32,
    #[serde(default = "default_top_k")]
    #[garde(range(min = 0))]
    pub top_k: usize,
    #[serde(default)]
    #[garde(range(min = 0.0, max = 1.0))]
    pub min_p: Option<f32>,
    #[serde(default = "default_typical_p")]
    #[garde(skip)]
    pub typical_p: f32,
    #[serde(default = "default_repeat_penalty")]
    #[garde(range(min = 1.0))]
    pub repeat_penalty: f32,
    #[serde(default)]
    #[garde(range(min = 0.0))]
    pub presence_penalty: Option<f32>,
    #[serde(default)]
    #[garde(range(min = 0.0))]
    pub frequency_penalty: Option<f32>,
    #[serde(default = "default_repeat_last_n")]
    #[garde(range(min = 0))]
    pub repeat_last_n: usize,
    #[serde(default = "default_seed")]
    #[garde(skip)]
    pub seed: u32,
    #[serde(default = "default_max_tokens")]
    #[garde(range(min = 1))]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CacheType {
    F32,
    #[default]
    F16,
    Q8_0,
    Q4_0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
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

// ---------------------------------------------------------------------------
// Config Loader
// ---------------------------------------------------------------------------

/// Multi-source YAML config loader with priority-based merging.
///
/// Config resolution order (highest priority wins):
/// 1. Per-model override (configs/models/<model-name>.yml)
/// 2. Machine-wide config (~/.config/whitt/config.yml)
/// 3. Docker-mounted config (/config/config.yml)
/// 4. Struct defaults (Default trait impls)
pub struct ConfigLoader;

impl ConfigLoader {
    /// Machine-wide config directory.
    pub const MACHINE_WIDE_DIR: &'static str = ".config/whitt";
    pub const MACHINE_WIDE_CONFIG: &'static str = "config.yml";

    /// Docker-mounted config path (inside container).
    pub const DOCKER_CONFIG: &'static str = "/config/config.yml";

    /// Per-model config directory (relative to project root).
    pub const PER_MODEL_DIR: &'static str = "configs/models";

    /// Load and merge configs from all available sources.
    /// If `model_name` is provided, per-model overrides are applied last.
    pub fn load_merged(model_name: Option<&str>) -> anyhow::Result<LlamaConfig> {
        let mut config = LlamaConfig::default();

        let docker_path = std::path::Path::new(Self::DOCKER_CONFIG);
        if docker_path.exists() {
            match LlamaConfig::from_file(docker_path) {
                Ok(docker_config) => {
                    tracing::info!(path = %docker_path.display(), "Loaded docker config");
                    config = docker_config;
                }
                Err(e) => {
                    tracing::warn!(path = %docker_path.display(), error = %e, "Failed to load docker config, using defaults");
                }
            }
        }

        if let Some(home) = dirs::home_dir() {
            let machine_path = home
                .join(Self::MACHINE_WIDE_DIR)
                .join(Self::MACHINE_WIDE_CONFIG);
            if machine_path.exists() {
                match LlamaConfig::from_file(&machine_path) {
                    Ok(machine_config) => {
                        tracing::info!(path = %machine_path.display(), "Loaded machine-wide config");
                        config = Self::merge(config, machine_config);
                    }
                    Err(e) => {
                        tracing::warn!(path = %machine_path.display(), error = %e, "Failed to load machine-wide config, skipping");
                    }
                }
            }
        }

        if let Some(name) = model_name {
            let model_config_path =
                std::path::Path::new(Self::PER_MODEL_DIR).join(format!("{}.yml", name));
            if model_config_path.exists() {
                match LlamaConfig::from_file(&model_config_path) {
                    Ok(model_config) => {
                        tracing::info!(path = %model_config_path.display(), model = name, "Loaded per-model config override");
                        config = Self::merge(config, model_config);
                    }
                    Err(e) => {
                        tracing::warn!(path = %model_config_path.display(), error = %e, "Failed to load per-model config, skipping");
                    }
                }
            }
        }

        Ok(config)
    }

    /// Deep-merge two LlamaConfig instances. `override_config` wins over `base`.
    /// Model section: entirely replaced (model path + HF config are atomic).
    /// Other sections: field-by-field merge using serde.
    fn merge(base: LlamaConfig, override_config: LlamaConfig) -> LlamaConfig {
        let base_yaml = serde_saphyr::to_string(&base).unwrap_or_default();
        let override_yaml = serde_saphyr::to_string(&override_config).unwrap_or_default();

        let base_json: JsonValue = serde_saphyr::from_str(&base_yaml).unwrap_or_default();
        let override_json: JsonValue = serde_saphyr::from_str(&override_yaml).unwrap_or_default();

        let merged = Self::merge_json_values(base_json, override_json);
        let merged_yaml = serde_saphyr::to_string(&merged).unwrap_or_default();

        serde_saphyr::from_str(&merged_yaml).unwrap_or(base)
    }

    /// Recursively merge JSON values. For objects, override wins per key.
    /// For arrays, override replaces entirely. For scalars, override wins.
    fn merge_json_values(base: JsonValue, override_val: JsonValue) -> JsonValue {
        match (base, override_val) {
            (JsonValue::Object(mut base_map), JsonValue::Object(override_map)) => {
                for (key, value) in override_map {
                    if let Some(existing) = base_map.remove(&key) {
                        base_map.insert(key, Self::merge_json_values(existing, value));
                    } else {
                        base_map.insert(key, value);
                    }
                }
                JsonValue::Object(base_map)
            }
            (_, override_value) => override_value,
        }
    }
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

    #[test]
    fn config_loader_missing_files_use_defaults() {
        let config = ConfigLoader::load_merged(None).expect("load with no model");
        assert_eq!(config.sampling.temperature, 0.80);
        assert_eq!(config.server.port, 8080);
    }

    #[test]
    fn merge_yaml_override_wins() {
        let base_yaml = r#"
model:
  path: /models/base.gguf
sampling:
  temperature: 0.80
  max_tokens: 512
context:
  size: 2048
"#;
        let override_yaml = r#"
model:
  path: /models/override.gguf
sampling:
  temperature: 0.50
"#;
        let base: LlamaConfig = serde_saphyr::from_str(base_yaml).expect("parse base");
        let override_config: LlamaConfig =
            serde_saphyr::from_str(override_yaml).expect("parse override");
        let merged = ConfigLoader::merge(base, override_config);

        assert_eq!(merged.model.path, PathBuf::from("/models/override.gguf"));
        assert!((merged.sampling.temperature - 0.50).abs() < 0.001);
        assert_eq!(merged.sampling.max_tokens, 512);
        assert_eq!(merged.context.size, 2048);
    }

    #[test]
    fn merge_empty_override_preserves_base() {
        let base_yaml = r#"
model:
  path: /models/base.gguf
sampling:
  temperature: 0.80
"#;
        let override_yaml = r#"
model:
  path: /models/override.gguf
"#;
        let base: LlamaConfig = serde_saphyr::from_str(base_yaml).expect("parse base");
        let override_config: LlamaConfig =
            serde_saphyr::from_str(override_yaml).expect("parse override");
        let merged = ConfigLoader::merge(base, override_config);

        assert_eq!(merged.model.path, PathBuf::from("/models/override.gguf"));
        assert!((merged.sampling.temperature - 0.80).abs() < 0.001);
    }

    #[test]
    fn validate_rejects_invalid_temperature() {
        let yaml = r#"
model:
  path: /models/test.gguf
sampling:
  temperature: 5.0
"#;
        let config: LlamaConfig = serde_saphyr::from_str(yaml).expect("parse");
        let result = config.validate_config();
        assert!(result.is_err(), "Validation must reject temperature 5.0");
    }
}
