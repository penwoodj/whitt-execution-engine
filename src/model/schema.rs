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
}

// ============================================================================
// Model host configuration
// ============================================================================

/// Host type and connection settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Maximum resource limits for the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Minimum resource requirements for the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinAllowed {
    /// Minimum RAM (percentage like "20%" or absolute like "2GB").
    #[serde(default)]
    pub ram: ResourceLimit,

    /// Minimum VRAM (percentage or absolute).
    #[serde(default)]
    pub vram: ResourceLimit,
}

impl Default for MinAllowed {
    fn default() -> Self {
        Self {
            ram: ResourceLimit::Percentage("20%".into()),
            vram: ResourceLimit::Percentage("30%".into()),
        }
    }
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
#[serde(rename_all = "snake_case")]
#[allow(non_camel_case_types)]
pub enum KvCacheQuantization {
    #[default]
    Q8_0,
    F32,
    F16,
    Q4_0,
    Q4_K_M,
    Q5_K_M,
    Q6_K,
}

/// Attention context strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AttentionContext {
    #[default]
    Rolling,
    Static,
    Dynamic,
    Auto,
}

fn default_kv_quantization() -> KvCacheQuantization {
    KvCacheQuantization::Q8_0
}

fn default_attention_context() -> AttentionContext {
    AttentionContext::Rolling
}

// ============================================================================
// Execution configuration
// ============================================================================

/// Model execution configuration.
#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
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
pub struct TimeoutConfig {
    /// Total timeout (e.g., "120s", "2m").
    #[serde(default = "default_total_timeout")]
    pub total: String,

    /// Per-token timeout (e.g., "500ms").
    #[serde(default = "default_per_token_timeout")]
    pub per_token: String,

    /// Queue timeout (e.g., "30s").
    #[serde(default = "default_queue_timeout")]
    pub queue: String,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            total: default_total_timeout(),
            per_token: default_per_token_timeout(),
            queue: default_queue_timeout(),
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

fn default_total_timeout() -> String {
    "120s".into()
}

fn default_per_token_timeout() -> String {
    "500ms".into()
}

fn default_queue_timeout() -> String {
    "30s".into()
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

/// Tools configuration for the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    /// Default permission level for tools.
    #[serde(default = "default_permissions")]
    pub default_permissions: String,

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

impl Default for ToolsConfig {
    fn default() -> Self {
        Self {
            default_permissions: default_permissions(),
            allowed_tools: Vec::new(),
            forbidden_tools: Vec::new(),
            custom_tools: Vec::new(),
        }
    }
}

fn default_permissions() -> String {
    "ask".into()
}

/// Custom tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct GuardrailsConfig {
    /// Maximum response length in tokens.
    #[serde(default)]
    pub max_response_length: Option<u32>,

    /// Forbidden patterns to block.
    #[serde(default)]
    pub forbidden_patterns: Vec<String>,

    /// Actions that require approval.
    #[serde(default)]
    pub require_approval_for: Vec<String>,
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
"#;
        let config: ModelsConfig = serde_saphyr::from_str(yaml).expect("parse");
        let max = &config.models["test-model"].max_allowed;

        assert_eq!(max.attention_tokens, 10000);
        assert_eq!(max.concurrent_requests, 5);

        match &max.ram {
            ResourceLimit::Percentage(s) => assert_eq!(s, "80%"),
            _ => panic!("Expected percentage"),
        }

        match &max.vram {
            ResourceLimit::Absolute(s) => assert_eq!(s, "4GB"),
            _ => panic!("Expected absolute"),
        }
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
}
