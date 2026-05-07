use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::error::Error;
use crate::error::Result;
use crate::model::schema::ModelsConfig;

pub use super::execution::WorkflowExecutionStrategy;
pub use super::permissions::ToolPermissions;
pub use super::step::{AgenticWorkflow, SubWorkflowRef};

/// Top-level keys allowed by the unified-workflow-schema.yml.
/// Any key not in this set is rejected by [`WorkflowFile::validate_raw_keys`].
const ALLOWED_TOP_LEVEL_KEYS: &[&str] = &[
    "workflow_id",
    "name",
    "description",
    "version",
    "author",
    "tags",
    "providers",
    "models",
    "sub_workflows",
    "agentic_workflow",
    "workflow_execution_strategy",
    "tool_permissions",
    "memory",
    "workspace",
    "schema_version",
    "min_schema_version",
];

/// Top-level workflow file structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowFile {
    pub workflow_id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub providers: Option<ProvidersConfig>,
    #[serde(default)]
    pub models: Option<ModelsConfig>,
    #[serde(default)]
    pub sub_workflows: Option<HashMap<String, SubWorkflowRef>>,
    #[serde(default)]
    pub agentic_workflow: Option<AgenticWorkflow>,
    #[serde(default)]
    pub workflow_execution_strategy: Option<WorkflowExecutionStrategy>,
    #[serde(default)]
    pub tool_permissions: Option<ToolPermissions>,
    #[serde(default)]
    pub memory: Option<MemoryConfig>,
    #[serde(default)]
    pub workspace: Option<WorkspaceConfig>,
    #[serde(default)]
    pub schema_version: Option<String>,
    #[serde(default)]
    pub min_schema_version: Option<String>,
}

impl WorkflowFile {
    /// Validate raw YAML top-level keys against the schema.
    ///
    /// This must run *before* deserialization because `#[serde(flatten)]` on
    /// `ProvidersConfig` absorbs unknown keys into the provider HashMap,
    /// making `#[serde(deny_unknown_fields)]` impossible at the `WorkflowFile` level.
    pub fn validate_raw_keys(yaml: &str) -> Result<()> {
        let doc: serde_json::Value = serde_saphyr::from_str(yaml)
            .map_err(Error::YamlParse)?;
        let mapping = doc.as_object().ok_or_else(|| Error::Validation {
            message: "workflow YAML must be a mapping at the top level".into(),
        })?;
        let allowed: std::collections::HashSet<&str> =
            ALLOWED_TOP_LEVEL_KEYS.iter().copied().collect();
        let mut unknown: Vec<String> = Vec::new();
        for key in mapping.keys() {
            if !allowed.contains(key.as_str()) {
                unknown.push(key.clone());
            }
        }
        if !unknown.is_empty() {
            return Err(Error::Validation {
                message: format!(
                    "unknown top-level key(s) not in schema: {}. \
                     Allowed keys: {}",
                    unknown.join(", "),
                    ALLOWED_TOP_LEVEL_KEYS.join(", ")
                ),
            });
        }
        Ok(())
    }

    pub fn from_yaml(yaml: &str) -> Result<Self> {
        Self::validate_raw_keys(yaml)?;
        serde_saphyr::from_str(yaml).map_err(Error::YamlParse)
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path).map_err(Error::Io)?;
        Self::from_yaml(&contents)
    }

    pub fn validate(&self) -> Result<()> {
        if self.workflow_id.is_empty() {
            return Err(Error::Validation {
                message: "workflow_id must be non-empty".into(),
            });
        }
        if self.name.is_empty() {
            return Err(Error::Validation {
                message: "name must be non-empty".into(),
            });
        }
        if let Some(ref providers) = self.providers {
            for provider_name in providers.providers.keys() {
                match provider_name.as_str() {
                    "llama_cpp_with_vulkan" => {}
                    other => {
                        return Err(Error::Validation {
                            message: format!(
                                "unsupported provider '{}'. \
                                 Current POC scope only supports 'llama_cpp_with_vulkan'. \
                                 Per schema line 28: lmstudio | ollama | llama_cpp_with_vulkan",
                                other
                            ),
                        });
                    }
                }
            }
        }
        if let Some(ref models) = self.models {
            for (model_name, model_spec) in &models.models {
                let host_type = &model_spec.host.r#type;
                match host_type.as_str() {
                    "llama_cpp_with_vulkan" => {}
                    other => {
                        return Err(Error::Validation {
                            message: format!(
                                "model '{}' has unsupported host.type '{}'. \
                                 Current POC scope only supports 'llama_cpp_with_vulkan'. \
                                 Per schema line 71: lmstudio | ollama | llama_cpp_with_vulkan",
                                model_name, other
                            ),
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

/// Providers configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProvidersConfig {
    #[serde(flatten)]
    pub providers: HashMap<String, ProviderConfig>,
}

/// Provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderConfig {
    #[serde(default)]
    pub config: Option<ProviderConnectionConfig>,
    #[serde(default)]
    pub config_file: Option<String>,
    #[serde(default)]
    pub hosting: Option<ProviderHostingConfig>,
    #[serde(default)]
    pub requests: Option<ProviderRequestsConfig>,
}

/// Provider connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderConnectionConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub connection_timeout_secs: Option<u64>,
}

/// Provider hosting configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderHostingConfig {
    #[serde(default)]
    pub max_concurrent_models: Option<u32>,
    #[serde(default)]
    pub model_offload_timeout_secs: Option<u64>,
    #[serde(default)]
    pub gpu_allocation: Option<GpuAllocationConfig>,
    #[serde(default)]
    pub cpu_fallback: Option<CpuFallbackConfig>,
}

/// GPU allocation configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GpuAllocationConfig {
    #[serde(default)]
    pub vram_per_model_mb: Option<u64>,
}

/// CPU fallback configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuFallbackConfig {
    #[serde(default)]
    pub cpu_cores_per_model: Option<u32>,
}

/// Provider requests configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderRequestsConfig {
    #[serde(default)]
    pub max_concurrent_requests: Option<u32>,
    #[serde(default)]
    pub request_timeout_secs: Option<u64>,
    #[serde(default)]
    pub queue_timeout_secs: Option<u64>,
    #[serde(default)]
    pub rate_limit_per_minute: Option<u32>,
    #[serde(default)]
    pub retry: Option<ProviderRetryConfig>,
}

/// Provider retry configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRetryConfig {
    #[serde(default)]
    pub max_retries: Option<u32>,
    #[serde(default = "default_backoff_strategy")]
    pub backoff: BackoffStrategy,
    #[serde(default = "default_initial_delay")]
    pub initial_delay: String,
    #[serde(default = "default_max_delay")]
    pub max_delay: String,
    #[serde(default = "default_multiplier")]
    pub multiplier: f32,
    #[serde(default = "default_jitter")]
    pub jitter: bool,
}

impl Default for ProviderRetryConfig {
    fn default() -> Self {
        Self {
            max_retries: None,
            backoff: default_backoff_strategy(),
            initial_delay: default_initial_delay(),
            max_delay: default_max_delay(),
            multiplier: default_multiplier(),
            jitter: default_jitter(),
        }
    }
}

/// Memory configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryConfig {
    #[serde(default)]
    pub rag: Option<RagConfig>,
}

/// RAG configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RagConfig {
    #[serde(default)]
    pub knowledge_base: Option<KnowledgeBaseConfig>,
    #[serde(default)]
    pub embedding_model: Option<EmbeddingModelConfig>,
    #[serde(default)]
    pub retrieval: Option<RetrievalConfig>,
}

/// Knowledge base configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KnowledgeBaseConfig {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default = "default_rag_format")]
    pub format: RagFormat,
    #[serde(default)]
    pub chunk_size: Option<u32>,
    #[serde(default)]
    pub chunk_overlap: Option<u32>,
}

/// Embedding model configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmbeddingModelConfig {
    #[serde(default)]
    pub model_ref: Option<String>,
    #[serde(default)]
    pub dimension: Option<u32>,
    #[serde(default)]
    pub batch_size: Option<u32>,
}

/// Retrieval configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrievalConfig {
    #[serde(default)]
    pub max_results: Option<u32>,
    #[serde(default)]
    pub similarity_threshold: Option<f32>,
    #[serde(default)]
    pub include_sources: Option<bool>,
}

/// Workspace configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceConfig {
    #[serde(default)]
    pub root_path: Option<String>,
    #[serde(default)]
    pub directories: Option<WorkspaceDirectories>,
    #[serde(default)]
    pub permissions: Option<WorkspacePermissions>,
}

/// Workspace directories.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceDirectories {
    #[serde(default)]
    pub output: Option<String>,
    #[serde(default)]
    pub checkpoints: Option<String>,
    #[serde(default)]
    pub logs: Option<String>,
    #[serde(default)]
    pub metrics: Option<String>,
    #[serde(default)]
    pub backups: Option<String>,
    #[serde(default)]
    pub temp: Option<String>,
    #[serde(default)]
    pub rag_knowledge_base: Option<String>,
}

/// Workspace permissions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspacePermissions {
    #[serde(default = "default_permission_mode")]
    pub default_mode: PermissionMode,
}

// ============================================================================
// Enums
// ============================================================================

/// Backoff strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum BackoffStrategy {
    #[default]
    Exponential,
    Linear,
    Fixed,
}

/// Retry level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RetryLevel {
    #[default]
    WorkflowRestart,
    WorkflowUnload,
    StepRestart,
    StepUnload,
    PromptRestart,
    ToolsRetry,
}

/// Log level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    #[default]
    Info,
    Debug,
    Warning,
    Error,
    Critical,
}

/// Tool permission policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ToolPermissionPolicy {
    #[default]
    Ask,
    Trust,
    Block,
}

/// Checkpoint level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointLevel {
    #[default]
    Info,
    Debug,
    Git,
    Timetravel,
    CompressedSummary,
}

/// Cache size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CacheSize {
    #[default]
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "medium-min")]
    MediumMin,
    #[serde(rename = "medium-max")]
    MediumMax,
}

/// KV cache quantization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
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
    Q4Km,
    #[serde(rename = "q5_k_m")]
    Q5Km,
    #[serde(rename = "q5_0")]
    Q5_0,
    #[serde(rename = "q6_k")]
    Q6K,
}

/// Attention context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum AttentionContext {
    #[default]
    Auto,
    FromMaxAllowed,
    ManualOverride,
}

/// RAM allocation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RamAllocationStrategy {
    #[default]
    Static,
    Dynamic,
}

/// Pressure strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PressureStrategy {
    #[default]
    Throttle,
    Swap,
    FailFast,
    GracefulDegradation,
}

/// Timeout strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TimeoutStrategy {
    #[default]
    Fail,
    ContinueWithPartial,
    Retry,
}

/// Error action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ErrorAction {
    #[default]
    Retry,
    Skip,
    FailWorkflow,
}

/// User input type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum UserInputType {
    #[default]
    TextArea,
    TextLine,
    Select,
    MultiSelect,
    Confirm,
}

/// Execution mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
pub enum ExecutionMode {
    #[default]
    Automated,
    Interactive,
    Hybrid,
}

/// Comparison operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    #[serde(rename = ">=")]
    GreaterEqual,
    #[serde(rename = "<=")]
    LessEqual,
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = "<")]
    Less,
}

/// Conflict resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ConflictResolution {
    #[default]
    LastWriterWins,
    MergeResults,
    FailOnConflict,
}

/// Event propagation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EventPropagation {
    #[default]
    Bidirectional,
    BottomToTop,
    TopToBottom,
}

/// Dependency strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DependencyStrategy {
    #[default]
    WaitForAll,
    ContinueWithAvailable,
    SkipFailed,
}

/// Dependency failure action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DependencyFailureAction {
    #[default]
    FailWorkflow,
    SkipDependent,
    UseFallback,
}

/// RAG format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RagFormat {
    #[default]
    VectorDb,
    FileBased,
    MemoryBased,
}

/// Permission mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PermissionMode {
    #[default]
    OwnerFullGroupReadExecOtherReadExec,
    OwnerFull,
    OwnerReadOnly,
    OwnerReadWrite,
    OwnerFullGroupRead,
    OwnerFullGroupReadOtherRead,
    OwnerFullGroupReadOtherReadExec,
    OwnerFullGroupFullOtherRead,
    OwnerFullGroupFullOtherFull,
}

/// Sub-workflow resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SubWorkflowResolution {
    #[default]
    LocalFirst,
    Hierarchical,
    RegistryOnly,
}

/// Version conflict action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum VersionConflictAction {
    #[default]
    Error,
    WarnAndContinue,
    UseLatest,
}

/// Retry adjustment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RetryAdjustment {
    #[default]
    LoosenTolerance,
    IncreasePromptDetail,
    SimplifyTask,
}

/// Format validation format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FormatValidationFormat {
    #[default]
    Json,
    Xml,
    Markdown,
}

/// Guardrail enforcement policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum GuardrailEnforcementPolicy {
    #[default]
    Block,
    Warn,
    Allow,
}

/// Guardrail sensitivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum GuardrailSensitivity {
    #[default]
    Medium,
    Low,
    High,
}

/// Guardrail on match action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum GuardrailOnMatch {
    #[default]
    Block,
    Warn,
    Sanitize,
    LogOnly,
    Truncate,
    Error,
    Retry,
}

/// Load/unload strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LoadUnloadStrategy {
    #[default]
    OneAtATime,
    Lazy,
    Eager,
}

// ============================================================================
// Default functions
// ============================================================================

fn default_host() -> String {
    "localhost".to_string()
}

fn default_port() -> u16 {
    1234
}

fn default_backoff_strategy() -> BackoffStrategy {
    BackoffStrategy::Exponential
}

fn default_initial_delay() -> String {
    "1s".to_string()
}

fn default_max_delay() -> String {
    "30s".to_string()
}

fn default_multiplier() -> f32 {
    2.0
}

fn default_jitter() -> bool {
    true
}

fn default_rag_format() -> RagFormat {
    RagFormat::VectorDb
}

fn default_permission_mode() -> PermissionMode {
    PermissionMode::OwnerFullGroupReadExecOtherReadExec
}
