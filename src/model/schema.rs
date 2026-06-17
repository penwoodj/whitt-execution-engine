use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Top-level models config
// ============================================================================

/// Complete models section configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsConfig {
    /// Path to global model configuration file.
    #[serde(default = "default_global_config_path")]
    pub global_config_path: String,

    /// Default routing strategy for model selection.
    #[serde(default = "default_router")]
    pub default_router: String,

    /// Model specifications keyed by model name.
    #[serde(flatten)]
    pub models: HashMap<String, ModelSpec>,
}

fn default_global_config_path() -> String {
    "./configs/models".into()
}

fn default_router() -> String {
    "automatic".into()
}

// ============================================================================
// Model specification
// ============================================================================

/// Complete model specification.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate, Default)]
#[serde(deny_unknown_fields)]
pub struct ModelSpec {
    /// Human-readable model name.
    #[serde(default)]
    #[garde(skip)]
    pub name: String,

    /// Host configuration.
    #[serde(default)]
    #[garde(skip)]
    pub host: ModelHost,

    /// RAM allocation strategy.
    #[serde(default)]
    #[garde(skip)]
    pub ram_allocation: RamAllocation,

    /// Maximum allowed resources.
    #[serde(default)]
    #[garde(skip)]
    pub max_allowed: MaxAllowed,

    /// Minimum required resources.
    #[serde(default)]
    #[garde(skip)]
    pub min_allowed: MinAllowed,

    /// Model memory configuration.
    #[serde(default)]
    #[garde(skip)]
    pub model_memory: ModelMemory,

    /// Execution configuration.
    #[serde(default)]
    #[garde(skip)]
    pub execution: ExecutionConfig,

    /// Thinking mode configuration.
    #[serde(default)]
    #[garde(skip)]
    pub thinking: ThinkingConfig,

    /// Tools configuration.
    #[serde(default)]
    #[garde(skip)]
    pub tools: ToolsConfig,

    /// Guardrails configuration.
    #[serde(default)]
    #[garde(skip)]
    pub guardrails: GuardrailsConfig,

    /// LLM server load parameters (context size, cache types, GPU layers, etc.).
    /// Maps directly to llama.cpp server startup flags.
    #[serde(default)]
    #[garde(skip)]
    pub load_params: LoadParams,

    /// Default sampling parameters for inference.
    #[serde(default)]
    #[garde(skip)]
    pub sampling: SamplingConfig,
}

// ============================================================================
// Model host configuration
// ============================================================================

/// Host type and connection settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelHost {
    /// Host provider type.
    #[serde(default = "default_host_type")]
    pub r#type: String,

    /// Provider-specific connection settings.
    #[serde(default)]
    pub connection_settings: HashMap<String, String>,
}

impl Default for ModelHost {
    fn default() -> Self {
        Self {
            r#type: default_host_type(),
            connection_settings: HashMap::new(),
        }
    }
}

fn default_host_type() -> String {
    "llama_cpp_with_vulkan".into()
}

// ============================================================================
// RAM allocation configuration
// ============================================================================

/// RAM allocation strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RamAllocation {
    /// Allocation strategy (static | dynamic | priority).
    #[serde(default = "default_ram_strategy")]
    pub strategy: String,
}

impl Default for RamAllocation {
    fn default() -> Self {
        Self {
            strategy: default_ram_strategy(),
        }
    }
}

fn default_ram_strategy() -> String {
    "dynamic".into()
}

// ============================================================================
// Maximum allowed resources
// ============================================================================

/// Maximum resource limits for model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaxAllowed {
    /// Maximum RAM (percentage like "80%" or absolute like "4GB").
    #[serde(default)]
    pub ram: ResourceLimit,

    /// Maximum VRAM (percentage or absolute).
    #[serde(default)]
    pub vram: ResourceLimit,

    /// Maximum CPU (percentage).
    #[serde(default = "default_cpu_limit")]
    pub cpu: String,

    /// Maximum GPU (percentage).
    #[serde(default = "default_gpu_limit")]
    pub gpu: String,

    /// Maximum attention tokens.
    #[serde(default = "default_attention_tokens")]
    pub attention_tokens: u64,

    /// Maximum concurrent requests.
    #[serde(default = "default_concurrent_requests")]
    pub concurrent_requests: u32,
}

impl Default for MaxAllowed {
    fn default() -> Self {
        Self {
            ram: ResourceLimit::Percentage("80%".into()),
            vram: ResourceLimit::Percentage("90%".into()),
            cpu: default_cpu_limit(),
            gpu: default_gpu_limit(),
            attention_tokens: default_attention_tokens(),
            concurrent_requests: default_concurrent_requests(),
        }
    }
}

fn default_cpu_limit() -> String {
    "50%".into()
}

fn default_gpu_limit() -> String {
    "80%".into()
}

fn default_attention_tokens() -> u64 {
    10000
}

fn default_concurrent_requests() -> u32 {
    5
}

// ============================================================================
// Minimum allowed resources
// ============================================================================

/// Minimum resource requirements for model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinAllowed {
    /// Minimum RAM (percentage like "20%" or absolute like "2GB").
    #[serde(default)]
    pub ram: ResourceLimit,

    /// Minimum VRAM (percentage or absolute).
    #[serde(default)]
    pub vram: ResourceLimit,

    /// Minimum CPU (percentage).
    #[serde(default = "default_min_cpu_limit")]
    pub cpu: String,

    /// Minimum GPU (percentage).
    #[serde(default = "default_min_gpu_limit")]
    pub gpu: String,

    /// Minimum attention tokens.
    #[serde(default = "default_min_attention_tokens")]
    pub attention_tokens: u64,
}

impl Default for MinAllowed {
    fn default() -> Self {
        Self {
            ram: ResourceLimit::Percentage("20%".into()),
            vram: ResourceLimit::Percentage("30%".into()),
            cpu: default_min_cpu_limit(),
            gpu: default_min_gpu_limit(),
            attention_tokens: default_min_attention_tokens(),
        }
    }
}

fn default_min_cpu_limit() -> String {
    "25%".into()
}

fn default_min_gpu_limit() -> String {
    "40%".into()
}

fn default_min_attention_tokens() -> u64 {
    5000
}

// ============================================================================
// Resource limit enum
// ============================================================================

/// Resource limit can be specified as percentage or absolute value.
#[derive(Debug, Clone, Serialize)]
pub enum ResourceLimit {
    /// Percentage value (e.g., "80%", "50%").
    Percentage(String),

    /// Absolute value (e.g., "4GB", "8192MB").
    Absolute(String),
}

impl<'de> Deserialize<'de> for ResourceLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.ends_with('%') {
            Ok(ResourceLimit::Percentage(s))
        } else {
            Ok(ResourceLimit::Absolute(s))
        }
    }
}

impl Default for ResourceLimit {
    fn default() -> Self {
        ResourceLimit::Percentage("100%".to_string())
    }
}

impl ResourceLimit {
    /// Parse resource limit string and determine type.
    pub fn parse_resource_limit(s: &str) -> Self {
        if s.ends_with('%') {
            ResourceLimit::Percentage(s.to_string())
        } else {
            ResourceLimit::Absolute(s.to_string())
        }
    }
    /// Parse a percentage resource limit (e.g., "80%" -> 80).
    pub fn parse_percentage(&self) -> Option<u64> {
        match self {
            ResourceLimit::Percentage(s) => {
                let trimmed = s.trim_end_matches('%');
                trimmed.parse::<u64>().ok()
            }
            ResourceLimit::Absolute(_) => None,
        }
    }

    /// Parse an absolute resource limit (e.g., "4GB" -> 4096 MB).
    /// Supports GB, MB, KB units.
    pub fn parse_absolute_mb(&self) -> Option<u64> {
        match self {
            ResourceLimit::Absolute(s) => {
                let s = s.to_lowercase();
                if s.ends_with("gb") {
                    let num = s.trim_end_matches("gb");
                    num.parse::<u64>().ok().map(|v| v * 1024)
                } else if s.ends_with("mb") {
                    s.trim_end_matches("mb").parse::<u64>().ok()
                } else if s.ends_with("kb") {
                    s.trim_end_matches("kb")
                        .parse::<u64>()
                        .ok()
                        .map(|v| v / 1024)
                } else {
                    // Assume MB if no unit specified
                    s.parse::<u64>().ok()
                }
            }
            ResourceLimit::Percentage(_) => None,
        }
    }
}

// ============================================================================
// Model memory configuration
// ============================================================================

/// Model memory and cache configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelMemory {
    /// Cache size (e.g., "2GB", "min", "max", "medium").
    #[serde(default = "default_cache_size")]
    pub cache_size: String,

    /// KV cache quantization format.
    #[serde(default = "default_kv_quantization")]
    pub kv_cache_quantization: KvCacheQuantization,

    /// Attention context strategy.
    #[serde(default = "default_attention_context")]
    pub attention_context: AttentionContext,
}

impl Default for ModelMemory {
    fn default() -> Self {
        Self {
            cache_size: default_cache_size(),
            kv_cache_quantization: default_kv_quantization(),
            attention_context: default_attention_context(),
        }
    }
}

fn default_cache_size() -> String {
    "2GB".into()
}

/// KV cache quantization formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[allow(non_camel_case_types)]
pub enum KvCacheQuantization {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "q8_0")]
    Q8_0,
    #[serde(rename = "q4_0")]
    Q4_0,
    #[serde(rename = "q4_k_m")]
    Q4_K_M,
    #[serde(rename = "q5_k_m")]
    Q5_K_M,
    #[serde(rename = "q5_0")]
    Q5_0,
    #[serde(rename = "q6_k")]
    Q6_K,
}

/// Attention context strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[allow(non_camel_case_types)]
pub enum AttentionContext {
    #[default]
    Auto,
    FromMaxAllowed,
    ManualOverride,
}

fn default_kv_quantization() -> KvCacheQuantization {
    KvCacheQuantization::Auto
}

fn default_attention_context() -> AttentionContext {
    AttentionContext::Auto
}

// ============================================================================
// Execution configuration
// ============================================================================

/// Model execution configuration.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
#[serde(deny_unknown_fields)]
pub struct ExecutionConfig {
    /// Timeout configuration.
    #[serde(default)]
    #[garde(skip)]
    pub timeout: TimeoutConfig,

    /// Maximum number of turns.
    #[serde(default = "default_max_turns")]
    #[garde(range(min = 1))]
    pub max_turns: u32,

    /// Stop on tool failure.
    #[serde(default = "default_stop_on_tool_failure")]
    #[garde(skip)]
    pub stop_on_tool_failure: bool,

    /// Accumulate tool results.
    #[serde(default = "default_accumulate_tool_results")]
    #[garde(skip)]
    pub accumulate_tool_results: bool,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            timeout: TimeoutConfig::default(),
            max_turns: default_max_turns(),
            stop_on_tool_failure: default_stop_on_tool_failure(),
            accumulate_tool_results: default_accumulate_tool_results(),
        }
    }
}

/// Timeout configuration for execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimeoutConfig {
    /// Time to load model into memory (e.g., "30s", "1m").
    #[serde(default = "default_load_into_memory_timeout")]
    pub load_into_memory: String,

    /// Time to first response (e.g., "10s", "30s").
    #[serde(default = "default_time_to_first_response_timeout")]
    pub time_to_first_response: String,

    /// Total time to respond (e.g., "120s", "2m").
    #[serde(default = "default_total_time_to_response_timeout")]
    pub total_time_to_response: String,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            load_into_memory: default_load_into_memory_timeout(),
            time_to_first_response: default_time_to_first_response_timeout(),
            total_time_to_response: default_total_time_to_response_timeout(),
        }
    }
}

fn default_max_turns() -> u32 {
    10
}

fn default_stop_on_tool_failure() -> bool {
    false
}

fn default_accumulate_tool_results() -> bool {
    true
}

fn default_load_into_memory_timeout() -> String {
    "30s".into()
}

fn default_time_to_first_response_timeout() -> String {
    "10s".into()
}

fn default_total_time_to_response_timeout() -> String {
    "120s".into()
}

/// Parse a duration string (e.g., "120s", "500ms", "2m") into milliseconds.
pub fn parse_duration_string(s: &str) -> anyhow::Result<u64> {
    let s = s.to_lowercase();
    let s = s.trim();

    if s.ends_with("ms") {
        let num = s.trim_end_matches("ms");
        num.parse::<u64>()
            .map_err(|e| anyhow::anyhow!("Invalid duration: {}", e))
    } else if s.ends_with("s") {
        let num = s.trim_end_matches("s");
        num.parse::<u64>()
            .map(|v| v * 1000)
            .map_err(|e| anyhow::anyhow!("Invalid duration: {}", e))
    } else if s.ends_with("m") {
        let num = s.trim_end_matches("m");
        num.parse::<u64>()
            .map(|v| v * 60 * 1000)
            .map_err(|e| anyhow::anyhow!("Invalid duration: {}", e))
    } else if s.ends_with("h") {
        let num = s.trim_end_matches("h");
        num.parse::<u64>()
            .map(|v| v * 60 * 60 * 1000)
            .map_err(|e| anyhow::anyhow!("Invalid duration: {}", e))
    } else {
        anyhow::bail!("Unknown duration unit: {}", s);
    }
}

// ============================================================================
// Thinking configuration
// ============================================================================

/// Thinking mode configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThinkingConfig {
    /// Budget tokens for thinking mode.
    #[serde(default)]
    pub budget_tokens: Option<u32>,

    /// Capture thinking in output.
    #[serde(default = "default_capture_in_output")]
    pub capture_in_output: bool,

    /// Capture thinking in events.
    #[serde(default = "default_capture_in_events")]
    pub capture_in_events: bool,
}

impl Default for ThinkingConfig {
    fn default() -> Self {
        Self {
            budget_tokens: None,
            capture_in_output: default_capture_in_output(),
            capture_in_events: default_capture_in_events(),
        }
    }
}

fn default_capture_in_output() -> bool {
    true
}

fn default_capture_in_events() -> bool {
    false
}

// ============================================================================
// Tools configuration
// ============================================================================

/// Tools configuration for model.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ToolsConfig {
    /// Default permissions for tools.
    #[serde(default)]
    pub default_permissions: DefaultPermissions,

    /// List of allowed tools.
    #[serde(default)]
    pub allowed_tools: Vec<String>,

    /// List of forbidden tools.
    #[serde(default)]
    pub forbidden_tools: Vec<String>,

    /// Custom tool definitions.
    #[serde(default)]
    pub custom_tools: Vec<CustomTool>,
}

/// Default permissions for tools.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DefaultPermissions {
    /// Web access permission.
    #[serde(default = "default_web_access")]
    pub web_access: bool,

    /// File read permission.
    #[serde(default = "default_file_read")]
    pub file_read: bool,

    /// File write permission.
    #[serde(default = "default_file_write")]
    pub file_write: bool,

    /// Shell execution permission.
    #[serde(default = "default_shell_exec")]
    pub shell_exec: bool,
}

impl Default for DefaultPermissions {
    fn default() -> Self {
        Self {
            web_access: default_web_access(),
            file_read: default_file_read(),
            file_write: default_file_write(),
            shell_exec: default_shell_exec(),
        }
    }
}

fn default_web_access() -> bool {
    true
}

fn default_file_read() -> bool {
    true
}

fn default_file_write() -> bool {
    false
}

fn default_shell_exec() -> bool {
    false
}

/// Custom tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomTool {
    /// Tool name.
    pub name: String,

    /// Tool description.
    pub description: String,

    /// Tool parameters schema.
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
}

// ============================================================================
// Guardrails configuration
// ============================================================================

/// Guardrails and safety configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct GuardrailsConfig {
    /// Enforcement policy (block | warn | allow).
    #[serde(default = "default_enforcement_policy")]
    pub enforcement_policy: String,

    /// Input guardrails configuration.
    #[serde(default)]
    pub input: InputGuardrails,

    /// Output guardrails configuration.
    #[serde(default)]
    pub output: OutputGuardrails,
}

fn default_enforcement_policy() -> String {
    "block".into()
}

/// Input guardrails configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct InputGuardrails {
    /// List of enabled input guards.
    #[serde(default)]
    pub guards: Vec<String>,

    /// Prompt injection guard configuration.
    #[serde(default)]
    pub prompt_injection: PromptInjectionConfig,

    /// PII redaction guard configuration.
    #[serde(default)]
    pub pii_redaction: PiiRedactionConfig,

    /// Maximum length guard configuration.
    #[serde(default)]
    pub max_length: MaxLengthConfig,
}

/// Output guardrails configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct OutputGuardrails {
    /// List of enabled output guards.
    #[serde(default)]
    pub guards: Vec<String>,

    /// Toxicity filter guard configuration.
    #[serde(default)]
    pub toxicity_filter: ToxicityFilterConfig,

    /// Format validation guard configuration.
    #[serde(default)]
    pub format_validation: FormatValidationConfig,
}

/// Prompt injection guard configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromptInjectionConfig {
    /// Sensitivity level (low | medium | high).
    #[serde(default = "default_sensitivity")]
    pub sensitivity: String,

    /// Action on match (block | warn | sanitize).
    #[serde(default = "default_on_match_block")]
    pub on_match: String,
}

impl Default for PromptInjectionConfig {
    fn default() -> Self {
        Self {
            sensitivity: default_sensitivity(),
            on_match: default_on_match_block(),
        }
    }
}

fn default_sensitivity() -> String {
    "medium".into()
}

fn default_on_match_block() -> String {
    "block".into()
}

/// PII redaction guard configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PiiRedactionConfig {
    /// Patterns to detect ([] = built-in, or custom regex strings).
    #[serde(default)]
    pub patterns: Vec<String>,

    /// Replacement text for redacted content.
    #[serde(default = "default_pii_replacement")]
    pub replacement: String,

    /// Action on match (sanitize | block | log_only).
    #[serde(default = "default_on_match_sanitize")]
    pub on_match: String,
}

impl Default for PiiRedactionConfig {
    fn default() -> Self {
        Self {
            patterns: vec![],
            replacement: default_pii_replacement(),
            on_match: default_on_match_sanitize(),
        }
    }
}

fn default_pii_replacement() -> String {
    "[REDACTED]".into()
}

fn default_on_match_sanitize() -> String {
    "sanitize".into()
}

/// Maximum length guard configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaxLengthConfig {
    /// Maximum number of tokens allowed.
    #[serde(default = "default_max_length_tokens")]
    pub max_tokens: u64,

    /// Action on match (block | truncate | error).
    #[serde(default = "default_max_length_action")]
    pub on_match: String,
}

impl Default for MaxLengthConfig {
    fn default() -> Self {
        Self {
            max_tokens: default_max_length_tokens(),
            on_match: default_max_length_action(),
        }
    }
}

fn default_max_length_tokens() -> u64 {
    8000
}

fn default_max_length_action() -> String {
    "truncate".into()
}

/// Toxicity filter guard configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToxicityFilterConfig {
    /// Token patterns that trigger the filter.
    #[serde(default)]
    pub trigger_tokens: Vec<String>,

    /// Action on match (block | warn | log_only).
    #[serde(default = "default_on_match_block")]
    pub on_match: String,
}

impl Default for ToxicityFilterConfig {
    fn default() -> Self {
        Self {
            trigger_tokens: vec![],
            on_match: default_on_match_block(),
        }
    }
}

/// Format validation guard configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormatValidationConfig {
    /// Expected format (json | xml | markdown).
    #[serde(default = "default_format")]
    pub format: String,

    /// Whether to enforce strict validation.
    #[serde(default = "default_strict")]
    pub strict: bool,

    /// Action on match (block | retry | error).
    #[serde(default = "default_on_match_error")]
    pub on_match: String,
}

impl Default for FormatValidationConfig {
    fn default() -> Self {
        Self {
            format: default_format(),
            strict: default_strict(),
            on_match: default_on_match_error(),
        }
    }
}

fn default_format() -> String {
    "json".into()
}

fn default_strict() -> bool {
    true
}

fn default_on_match_error() -> String {
    "error".into()
}

// ============================================================================
// Load parameters (llama.cpp server startup flags)
// ============================================================================

/// LLM server load parameters mapped to llama.cpp startup flags.
/// These control how the model is loaded into memory on the server side.
///
/// Vulkan-specific constraints (enforced by AGENTS.md):
/// - `flash_attn: true` is safe and recommended
/// - `no_cache_prompt: true` MUST be used (Vulkan cannot serialize KV cache)
/// - `cont_batching: false` MUST NOT be enabled (triggers KV cache serialization)
/// - `cache_type_k/v`: Q8_0 enabled for larger context (test with your Vulkan build)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct LoadParams {
    /// Context window size in tokens (maps to `--ctx-size`).
    #[serde(default = "default_context_size")]
    pub context_size: usize,

    /// Logical batch size for prompt processing (maps to `--batch-size`).
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

    /// Physical batch size for prompt processing (maps to `--ubatch-size`).
    #[serde(default = "default_ubatch_size")]
    pub ubatch_size: usize,

    /// KV cache type for K tensor (maps to `--cache-type-k`).
    /// Q8_0 by default; test with your Vulkan build.
    #[serde(default = "default_cache_type_k")]
    pub cache_type_k: String,

    /// KV cache type for V tensor (maps to `--cache-type-v`).
    /// Q8_0 by default; test with your Vulkan build.
    #[serde(default = "default_cache_type_v")]
    pub cache_type_v: String,

    /// Number of GPU layers to offload (maps to `--n-gpu-layers`).
    /// Use 0 for CPU-only inference; 99+ for full GPU offload.
    #[serde(default = "default_gpu_layers")]
    pub gpu_layers: usize,

    /// CPU threads for inference (maps to `--threads`, 0 = auto-detect).
    #[serde(default = "default_threads")]
    pub threads: usize,

    /// Use memory-mapped files for faster loading (maps to `--mmap`).
    #[serde(default = "default_use_mmap")]
    pub use_mmap: bool,

    /// Enable flash attention (maps to `--flash-attn`).
    /// Safe and recommended for Vulkan.
    #[serde(default = "default_flash_attn")]
    pub flash_attn: bool,

    /// Enable continuous batching (maps to `--cont-batching`).
    /// MUST be `false` for Vulkan (triggers KV cache serialization on slot release).
    #[serde(default)]
    pub cont_batching: bool,

    /// Disable prompt caching (maps to `--no-cache-prompt`).
    /// MUST be `true` for Vulkan (cannot serialize KV cache state).
    #[serde(default = "default_no_cache_prompt")]
    pub no_cache_prompt: bool,

    /// Number of parallel sequences (slots) to process concurrently (maps to `--parallel`).
    /// Set to 1 for single-shot deterministic generation.
    #[serde(default = "default_parallel")]
    pub parallel: usize,
}

impl LoadParams {
    pub fn to_env_vars(&self) -> Vec<(String, String)> {
        vec![
            ("LLAMA_ARG_CTX_SIZE".into(), self.context_size.to_string()),
            ("LLAMA_ARG_BATCH_SIZE".into(), self.batch_size.to_string()),
            ("LLAMA_ARG_UBATCH_SIZE".into(), self.ubatch_size.to_string()),
            ("LLAMA_ARG_CACHE_TYPE_K".into(), self.cache_type_k.clone()),
            ("LLAMA_ARG_CACHE_TYPE_V".into(), self.cache_type_v.clone()),
            ("LLAMA_ARG_N_GPU_LAYERS".into(), self.gpu_layers.to_string()),
            ("LLAMA_ARG_N_THREADS".into(), self.threads.to_string()),
            ("LLAMA_ARG_USE_MMAP".into(), if self.use_mmap { "1".into() } else { "0".into() }),
            ("LLAMA_ARG_FLASH_ATTN".into(), if self.flash_attn { "1".into() } else { "0".into() }),
            ("LLAMA_ARG_CONT_BATCHING".into(), if self.cont_batching { "1".into() } else { "0".into() }),
            ("LLAMA_ARG_NO_CACHE_PROMPT".into(), if self.no_cache_prompt { "1".into() } else { "0".into() }),
            ("LLAMA_ARG_PARALLEL".into(), self.parallel.to_string()),
        ]
    }
}

fn default_context_size() -> usize {
    262144
}

fn default_batch_size() -> usize {
    2048
}

fn default_ubatch_size() -> usize {
    512
}

fn default_cache_type_k() -> String {
    "q8_0".into()
}

fn default_cache_type_v() -> String {
    "q8_0".into()
}

fn default_gpu_layers() -> usize {
    0
}

fn default_threads() -> usize {
    5
}

fn default_use_mmap() -> bool {
    true
}

fn default_flash_attn() -> bool {
    true
}

fn default_no_cache_prompt() -> bool {
    true
}

fn default_parallel() -> usize {
    1
}

// ============================================================================
// Sampling config (per-model inference defaults)
// ============================================================================

/// Per-model default sampling parameters for inference.
/// All fields are optional — unset fields fall back to server defaults.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct SamplingConfig {
    /// Sampling temperature (0.0–2.0).
    #[serde(default)]
    pub temperature: Option<f32>,

    /// Nucleus sampling probability (0.0–1.0).
    #[serde(default)]
    pub top_p: Option<f32>,

    /// Top-k sampling (0 = disabled).
    #[serde(default)]
    pub top_k: Option<usize>,

    /// Minimum probability threshold (0.0–1.0).
    #[serde(default)]
    pub min_p: Option<f32>,

    /// Maximum tokens to generate.
    #[serde(default)]
    pub max_tokens: Option<usize>,

    /// Repetition penalty (1.0 = no penalty).
    #[serde(default)]
    pub repeat_penalty: Option<f32>,

    /// Random seed (0 = random).
    #[serde(default)]
    pub seed: Option<u32>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_model_spec() {
        let yaml = r#"
my-model:
  name: "My Model"
  host:
    type: "llama_cpp_with_vulkan"
    connection_settings:
      host: "localhost"
      port: "8080"
"#;
        let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse");
        assert_eq!(config.models.len(), 1);
        assert_eq!(config.models["my-model"].name, "My Model");
    }

    #[test]
    fn parse_resource_limits() {
        let yaml = r#"
test-model:
  max_allowed:
    ram: "80%"
    vram: "4GB"
    cpu: "50%"
    gpu: "80%"
    attention_tokens: 10000
    concurrent_requests: 5
  min_allowed:
    ram: "20%"
    vram: "2GB"
    cpu: "25%"
    gpu: "40%"
    attention_tokens: 5000
"#;
        let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse");
        let max = &config.models["test-model"].max_allowed;
        let min = &config.models["test-model"].min_allowed;

        assert_eq!(max.attention_tokens, 10000);
        assert_eq!(max.concurrent_requests, 5);
        assert_eq!(min.attention_tokens, 5000);

        match &max.ram {
            ResourceLimit::Percentage(s) => assert_eq!(s, "80%"),
            _ => panic!("Expected percentage"),
        }

        match &max.vram {
            ResourceLimit::Absolute(s) => assert_eq!(s, "4GB"),
            _ => panic!("Expected absolute"),
        }

        assert_eq!(max.cpu, "50%");
        assert_eq!(max.gpu, "80%");
        assert_eq!(min.cpu, "25%");
        assert_eq!(min.gpu, "40%");
    }

    #[test]
    fn parse_timeout_config() {
        let yaml = r#"
test-model:
  execution:
    timeout:
      load_into_memory: "30s"
      time_to_first_response: "10s"
      total_time_to_response: "120s"
"#;
        let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse");
        let timeout = &config.models["test-model"].execution.timeout;

        assert_eq!(timeout.load_into_memory, "30s");
        assert_eq!(timeout.time_to_first_response, "10s");
        assert_eq!(timeout.total_time_to_response, "120s");
    }

    #[test]
    fn parse_kv_cache_quantization() {
        let yaml = r#"
test-model:
  model_memory:
    kv_cache_quantization: auto
"#;
        let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse");
        assert_eq!(
            config.models["test-model"].model_memory.kv_cache_quantization,
            KvCacheQuantization::Auto
        );
    }

    #[test]
    fn parse_attention_context() {
        let yaml = r#"
test-model:
  model_memory:
    attention_context: from_max_allowed
"#;
        let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse");
        assert_eq!(
            config.models["test-model"].model_memory.attention_context,
            AttentionContext::FromMaxAllowed
        );
    }

    #[test]
    fn parse_default_permissions() {
        let yaml = r#"
test-model:
  tools:
    default_permissions:
      web_access: true
      file_read: true
      file_write: false
      shell_exec: false
"#;
        let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse");
        let perms = &config.models["test-model"].tools.default_permissions;

        assert_eq!(perms.web_access, true);
        assert_eq!(perms.file_read, true);
        assert_eq!(perms.file_write, false);
        assert_eq!(perms.shell_exec, false);
    }

    #[test]
    fn parse_guardrails_config() {
        let yaml = r#"
test-model:
  guardrails:
    enforcement_policy: block
    input:
      guards: [prompt_injection, pii_redaction]
      prompt_injection:
        sensitivity: high
        on_match: block
      pii_redaction:
        patterns: []
        replacement: "[REDACTED]"
        on_match: sanitize
      max_length:
        max_tokens: 8000
        on_match: truncate
    output:
      guards: [toxicity_filter]
      toxicity_filter:
        trigger_tokens: ["ignore previous instructions"]
        on_match: block
      format_validation:
        format: json
        strict: true
        on_match: retry
"#;
        let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse");
        let guardrails = &config.models["test-model"].guardrails;

        assert_eq!(guardrails.enforcement_policy, "block");
        assert_eq!(guardrails.input.guards.len(), 2);
        assert_eq!(guardrails.input.prompt_injection.sensitivity, "high");
        assert_eq!(guardrails.input.prompt_injection.on_match, "block");
        assert_eq!(guardrails.input.pii_redaction.patterns.len(), 0);
        assert_eq!(guardrails.input.pii_redaction.replacement, "[REDACTED]");
        assert_eq!(guardrails.input.pii_redaction.on_match, "sanitize");
        assert_eq!(guardrails.input.max_length.max_tokens, 8000);
        assert_eq!(guardrails.input.max_length.on_match, "truncate");
        assert_eq!(guardrails.output.toxicity_filter.trigger_tokens.len(), 1);
        assert_eq!(guardrails.output.toxicity_filter.on_match, "block");
        assert_eq!(guardrails.output.format_validation.format, "json");
        assert_eq!(guardrails.output.format_validation.strict, true);
        assert_eq!(guardrails.output.format_validation.on_match, "retry");
    }

    #[test]
    fn parse_percentage_resource_limit() {
        let limit = ResourceLimit::parse_resource_limit("80%");
        assert_eq!(limit.parse_percentage(), Some(80));
        assert_eq!(limit.parse_absolute_mb(), None);
    }

    #[test]
    fn parse_absolute_resource_limit() {
        let limit = ResourceLimit::parse_resource_limit("4GB");
        assert_eq!(limit.parse_absolute_mb(), Some(4096));
        assert_eq!(limit.parse_percentage(), None);
    }

    #[test]
    fn parse_duration_ms() {
        assert_eq!(parse_duration_string("500ms").unwrap(), 500);
    }

    #[test]
    fn parse_duration_seconds() {
        assert_eq!(parse_duration_string("120s").unwrap(), 120_000);
    }

    #[test]
    fn parse_duration_minutes() {
        assert_eq!(parse_duration_string("2m").unwrap(), 120_000);
    }

    #[test]
    fn parse_duration_hours() {
        assert_eq!(parse_duration_string("1h").unwrap(), 3_600_000);
    }

    #[test]
    fn default_model_spec() {
        let spec = ModelSpec::default();
        assert_eq!(spec.name, "");
        assert_eq!(spec.ram_allocation.strategy, "dynamic");
        assert_eq!(spec.execution.max_turns, 10);
        assert_eq!(spec.thinking.capture_in_output, true);
    }

    #[test]
    fn kv_cache_quantization_variants() {
        // Test all enum variants parse correctly
        let variants = ["auto", "q8_0", "q4_0", "q4_k_m", "q5_k_m", "q5_0", "q6_k"];
        for variant in variants {
            let yaml = format!(r#"
test-model:
  model_memory:
    kv_cache_quantization: {}
"#, variant);
            let config: ModelsConfig = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", variant));
            // Just verify it parses without error
            assert!(config.models.contains_key("test-model"));
        }
    }

    #[test]
    fn attention_context_variants() {
        // Test all enum variants parse correctly
        let variants = ["auto", "from_max_allowed", "manual_override"];
        for variant in variants {
            let yaml = format!(r#"
test-model:
  model_memory:
    attention_context: {}
"#, variant);
            let config: ModelsConfig = serde_saphyr::from_str(&yaml).expect(&format!("parse {}", variant));
            // Just verify it parses without error
            assert!(config.models.contains_key("test-model"));
        }
    }

    #[test]
    fn timeout_field_names() {
        // Verify old field names don't exist and new ones do
        let yaml = r#"
test-model:
  execution:
    timeout:
      load_into_memory: "30s"
      time_to_first_response: "10s"
      total_time_to_response: "120s"
"#;
        let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse");
        let timeout = &config.models["test-model"].execution.timeout;

        // Verify new fields exist
        assert_eq!(timeout.load_into_memory, "30s");
        assert_eq!(timeout.time_to_first_response, "10s");
        assert_eq!(timeout.total_time_to_response, "120s");
    }
}
