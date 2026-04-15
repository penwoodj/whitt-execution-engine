# Task 2: Schema Types

**Goal:** Define ALL Rust structs for the unified schema (WorkflowSpec, ModelConfig, Step, Loop, etc.) with serde derives and garde validation.

**Estimated Time:** 8 hours

**Dependencies:** Task 1

**Files:**
- Create: `src/schema/identification.rs` (Section 1: Workflow Identification)
- Create: `src/schema/model.rs` (Section 2: Model Configuration)
- Create: `src/schema/workspace.rs` (Section 13: Workspace Configuration)
- Create: `src/schema/step.rs` (Step definitions)
- Create: `src/schema/loop.rs` (Loop configurations)
- Create: `src/schema/execution.rs` (Execution strategy)
- Create: `src/schema/mod.rs` (Module exports)
- Modify: `src/schema/mod.rs` (in lib.rs)

---

## Step 1: Create src/schema/identification.rs

Define workflow identification types (Section 1):

```rust
use serde::{Deserialize, Serialize};
use garde::Validate;
use schemars::JsonSchema;
use crate::error::{Error, Result};

/// Workflow identification (Section 1)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct WorkflowIdentification {
    /// Unique identifier for this workflow
    #[garde(skip)]
    pub workflow_id: String,

    /// Human-readable workflow name
    #[garde(length(min = 1))]
    pub name: String,

    /// Detailed description of workflow purpose
    #[garde(length(min = 1))]
    pub description: String,

    /// Workflow version (semantic versioning)
    #[garde(skip)]
    #[serde(default = "default_version")]
    pub version: String,

    /// Workflow author
    #[serde(default)]
    pub author: String,

    /// Categorization tags for discovery
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}
```

**Commit:** `feat: add workflow identification schema types`

---

## Step 2: Create src/schema/model.rs

Define model configuration types (Section 2):

```rust
use serde::{Deserialize, Serialize};
use garde::Validate;
use schemars::JsonSchema;
use std::collections::HashMap;

/// Model provider types
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelProvider {
    LmStudio,
    Ollama,
    LlamaCppWithVulkan,
}

/// Resource limit types
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ResourceLimit {
    Percentage(f64),
    Absolute { value: u64, unit: String },
}

impl Default for ResourceLimit {
    fn default() -> Self {
        ResourceLimit::Percentage(0.0)
    }
}

/// Model resource limits
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ResourceLimits {
    #[serde(default)]
    pub ram: ResourceLimit,

    #[serde(default)]
    pub vram: ResourceLimit,

    #[serde(default)]
    pub cpu: ResourceLimit,

    #[serde(default)]
    pub gpu: ResourceLimit,

    #[serde(default)]
    pub attention_tokens: u64,

    #[serde(default)]
    pub concurrent_requests: u32,
}

/// Model host configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModelHost {
    #[serde(rename = "type")]
    pub provider_type: ModelProvider,

    #[serde(default)]
    pub connection_settings: HashMap<String, serde_json::Value>,
}

/// Model memory configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModelMemory {
    #[serde(default = "default_cache_size")]
    pub cache_size: String,

    #[serde(default = "default_kv_cache_quantization")]
    pub kv_cache_quantization: String,

    #[serde(default = "default_attention_context")]
    pub attention_context: String,
}

fn default_cache_size() -> String {
    "min".to_string()
}

fn default_kv_cache_quantization() -> String {
    "auto".to_string()
}

fn default_attention_context() -> String {
    "auto".to_string()
}

/// Execution timeouts
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionTimeouts {
    #[serde(default = "default_load_timeout")]
    pub load_into_memory: String,

    #[serde(default = "default_first_response")]
    pub time_to_first_response: String,

    #[serde(default = "default_total_time")]
    pub total_time_to_response: String,
}

fn default_load_timeout() -> String {
    "45s".to_string()
}

fn default_first_response() -> String {
    "1m".to_string()
}

fn default_total_time() -> String {
    "4h".to_string()
}

/// Thinking configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ThinkingConfig {
    #[serde(default = "default_thinking_budget")]
    pub budget_tokens: u32,

    #[serde(default)]
    pub capture_in_output: bool,

    #[serde(default)]
    pub capture_in_events: bool,

    #[serde(default)]
    pub stream_to_log: bool,
}

fn default_thinking_budget() -> u32 {
    4096
}

/// Tool permission levels
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PermissionLevel {
    Ask,
    Trust,
    Block,
    AlwaysAsk,
}

impl Default for PermissionLevel {
    fn default() -> Self {
        PermissionLevel::Ask
    }
}

/// Tool permissions
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ToolPermissions {
    #[serde(default)]
    pub web_access: PermissionLevel,

    #[serde(default)]
    pub file_read: PermissionLevel,

    #[serde(default)]
    pub file_write: PermissionLevel,

    #[serde(default)]
    pub shell_exec: PermissionLevel,

    #[serde(default)]
    pub allowed_tools: Vec<String>,

    #[serde(default)]
    pub forbidden_tools: Vec<String>,

    #[serde(default)]
    pub custom_tools: Vec<serde_json::Value>,
}

/// Guardrails enforcement policy
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementPolicy {
    Block,
    Warn,
    Allow,
}

/// Guardrails configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Guardrails {
    #[serde(default = "default_enforcement_policy")]
    pub enforcement_policy: EnforcementPolicy,

    #[serde(default)]
    pub input: InputGuards,

    #[serde(default)]
    pub output: OutputGuards,

    #[serde(default)]
    pub tool_use: ToolUseGuards,
}

fn default_enforcement_policy() -> EnforcementPolicy {
    EnforcementPolicy::Block
}

/// Input guards
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct InputGuards {
    #[serde(default)]
    pub guards: Vec<String>,

    #[serde(default)]
    pub prompt_injection: PromptInjectionGuard,

    #[serde(default)]
    pub pii_redaction: PiiRedactionGuard,

    #[serde(default)]
    pub max_length: MaxLengthGuard,
}

/// Prompt injection guard
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct PromptInjectionGuard {
    #[serde(default = "default_sensitivity")]
    pub sensitivity: String,

    #[serde(default = "default_on_match")]
    pub on_match: String,

    #[serde(default)]
    pub log: bool,
}

fn default_sensitivity() -> String {
    "high".to_string()
}

fn default_on_match() -> String {
    "block".to_string()
}

/// PII redaction guard
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct PiiRedactionGuard {
    #[serde(default)]
    pub patterns: Vec<String>,

    #[serde(default = "default_replacement")]
    pub replacement: String,

    #[serde(default = "default_pii_on_match")]
    pub on_match: String,
}

fn default_replacement() -> String {
    "[REDACTED]".to_string()
}

fn default_pii_on_match() -> String {
    "sanitize".to_string()
}

/// Max length guard
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MaxLengthGuard {
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,

    #[serde(default = "default_max_length_on_match")]
    pub on_match: String,

    #[serde(default = "default_truncate_to")]
    pub truncate_to: u32,
}

impl Default for MaxLengthGuard {
    fn default() -> Self {
        Self {
            max_tokens: default_max_tokens(),
            on_match: default_max_length_on_match(),
            truncate_to: default_truncate_to(),
        }
    }
}

fn default_max_tokens() -> u32 {
    8000
}

fn default_max_length_on_match() -> String {
    "truncate".to_string()
}

fn default_truncate_to() -> u32 {
    7500
}

/// Output guards
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct OutputGuards {
    #[serde(default)]
    pub guards: Vec<String>,

    #[serde(default)]
    pub toxicity_filter: ToxicityFilterGuard,

    #[serde(default)]
    pub pii_redaction: PiiRedactionGuard,

    #[serde(default)]
    pub format_validation: FormatValidationGuard,

    #[serde(default)]
    pub max_length: MaxLengthGuard,
}

/// Toxicity filter guard
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ToxicityFilterGuard {
    #[serde(default)]
    pub trigger_tokens: Vec<String>,

    #[serde(default)]
    pub on_match: String,

    #[serde(default)]
    pub log: bool,
}

/// Format validation guard
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FormatValidationGuard {
    #[serde(default = "default_format")]
    pub format: String,

    #[serde(default)]
    pub strict: bool,

    #[serde(default)]
    pub on_match: String,

    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
}

impl Default for FormatValidationGuard {
    fn default() -> Self {
        Self {
            format: default_format(),
            strict: false,
            on_match: "retry".to_string(),
            max_retries: default_max_retries(),
        }
    }
}

fn default_format() -> String {
    "json".to_string()
}

fn default_max_retries() -> u32 {
    2
}

/// Tool use guards
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ToolUseGuards {
    #[serde(default)]
    pub guards: Vec<String>,
}

/// Lifecycle hooks
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct LifecycleHooks {
    #[serde(default)]
    pub on_create: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_run_start: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_run_complete: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_run_error: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_retry: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_turn_start: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_turn_complete: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_turn_error: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_thinking: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_tool_call: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_tool_start: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_tool_result: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_tool_error: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_stream_chunk: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_stream_complete: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_memory_recall: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub on_memory_store: HashMap<String, serde_json::Value>,
}

/// Model configuration (Section 2)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct ModelConfig {
    /// Model identifier (use in workflow: "${models.model_name}")
    #[garde(skip)]
    pub name: String,

    /// Host configuration
    pub host: ModelHost,

    /// RAM allocation (presence = RAM allocation configured)
    #[serde(default)]
    pub ram_allocation: RamAllocation,

    /// Maximum resource limits
    #[serde(default)]
    pub max_allowed: ResourceLimits,

    /// Minimum resource requirements
    #[serde(default)]
    pub min_allowed: ResourceLimits,

    /// Model memory configuration
    #[serde(default)]
    pub model_memory: ModelMemory,

    /// Execution timeouts
    #[serde(default)]
    pub execution: ExecutionTimeouts,

    /// Thinking configuration
    #[serde(default)]
    pub thinking: ThinkingConfig,

    /// Tool permissions
    #[serde(default)]
    pub tools: ToolPermissions,

    /// Lifecycle hooks
    #[serde(default)]
    pub hooks: LifecycleHooks,

    /// Guardrails (presence = guardrails enabled)
    #[serde(default)]
    pub guardrails: Option<Guardrails>,
}

/// RAM allocation configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RamAllocation {
    /// Allocation strategy
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
    "dynamic".to_string()
}
```

**Commit:** `feat: add model configuration schema types`

---

## Step 3: Create src/schema/workspace.rs

Define workspace configuration types (Section 13):

```rust
use serde::{Deserialize, Serialize};
use garde::Validate;
use schemars::JsonSchema;

/// Workspace configuration (Section 13)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct WorkspaceConfig {
    /// Root path for workspace
    #[garde(length(min = 1))]
    pub root_path: String,

    /// Output directory
    #[serde(default = "default_output_dir")]
    pub output_path: String,

    /// Checkpoint directory
    #[serde(default = "default_checkpoint_dir")]
    pub checkpoint_path: String,

    /// Log directory
    #[serde(default = "default_log_dir")]
    pub log_path: String,

    /// Metrics directory
    #[serde(default = "default_metrics_dir")]
    pub metrics_path: String,

    /// Temporary directory
    #[serde(default = "default_temp_dir")]
    pub temp_path: String,
}

fn default_output_dir() -> String {
    "./output".to_string()
}

fn default_checkpoint_dir() -> String {
    "./checkpoints".to_string()
}

fn default_log_dir() -> String {
    "./logs".to_string()
}

fn default_metrics_dir() -> String {
    "./metrics".to_string()
}

fn default_temp_dir() -> String {
    "./temp".to_string()
}
```

**Commit:** `feat: add workspace configuration schema types`
---

## Step 4: Create src/schema/step.rs

Define step configuration types:

```rust
use serde::{Deserialize, Serialize};
use garde::Validate;
use schemars::JsonSchema;
use std::collections::HashMap;

/// Output format
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Json,
    Yaml,
    Text,
    Markdown,
}

/// Backoff strategy
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BackoffStrategy {
    Exponential,
    Linear,
    Fixed,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RetryConfig {
    #[serde(default)]
    pub condition: Option<String>,

    #[serde(default = "default_max_attempts")]
    pub max_attempts: u32,

    #[serde(default = "default_backoff")]
    pub backoff: BackoffStrategy,

    #[serde(default = "default_delay_ms")]
    pub delay_ms: u64,

    #[serde(default)]
    pub base_ms: Option<u64>,

    #[serde(default)]
    pub max_ms: Option<u64>,

    #[serde(default)]
    pub jitter: Option<f64>,

    #[serde(default = "default_retry_level")]
    pub level: String,

    #[serde(default)]
    pub adjustment_strategy: Option<String>,

    #[serde(default)]
    pub tolerance_adjustment: Option<f64>,

    #[serde(default)]
    pub checkpoint_after_retry: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            condition: None,
            max_attempts: default_max_attempts(),
            backoff: default_backoff(),
            delay_ms: default_delay_ms(),
            base_ms: None,
            max_ms: None,
            jitter: None,
            level: default_retry_level(),
            adjustment_strategy: None,
            tolerance_adjustment: None,
            checkpoint_after_retry: false,
        }
    }
}

fn default_max_attempts() -> u32 {
    3
}

fn default_backoff() -> BackoffStrategy {
    BackoffStrategy::Exponential
}

fn default_delay_ms() -> u64 {
    1000
}

fn default_retry_level() -> String {
    "step_restart".to_string()
}

/// Step configuration (step type inferred from keys present)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct Step {
    /// Unique step identifier (used as variable name)
    #[garde(length(min = 1))]
    pub step: String,

    /// Generative entity reference (e.g., "${models.primary}")
    /// Step type = agent when this field is present
    #[serde(default)]
    pub generative_entity: Option<String>,

    /// Prompt for agent steps
    #[serde(default)]
    pub prompt: Option<String>,

    /// Tool name (e.g., "file_read")
    /// Step type = tool when this field is present
    #[serde(default)]
    pub tool: Option<String>,

    /// Tool input
    #[serde(default)]
    pub input: Option<serde_json::Value>,

    /// Sub-workflow reference (e.g., "validate_workflow")
    /// Step type = sub-workflow when this field is present
    #[serde(default)]
    pub sub_workflow: Option<String>,

    /// Loop configuration
    /// Step type = loop when this field is present
    #[serde(default)]
    pub loop_config: Option<LoopConfig>,

    /// GWT (Given When Then) configuration
    /// Step type = control when this field is present
    #[serde(default)]
    pub when: Option<WhenConfig>,

    /// Model overrides
    #[serde(default)]
    pub model_overrides: Option<ModelOverrides>,

    /// User input prompt
    #[serde(default)]
    pub user_input: Option<UserInput>,

    /// > **Note**: Parallel execution features have moved to the [agent-queue](https://github.com/penwoodj/agent-queue) project. The following structs describe historical parallel execution capabilities.

    /// Step dependencies (require before execution)
    #[serde(default)]
    pub depends_on: Option<Vec<DependencyConfig>>,

    /// Parallel group name
    #[serde(default)]
    pub parallel_group: Option<String>,

    /// Retry configuration
    #[serde(default)]
    pub retry: Option<RetryConfig>,

    /// Timeout in seconds
    #[serde(default)]
    pub timeout_secs: Option<u64>,
}

/// Model overrides
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModelOverrides {
    #[serde(default)]
    pub max_turns: Option<u32>,
}

/// User input prompt configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserInput {
    #[serde(rename = "type")]
    pub input_type: String,

    #[serde(default)]
    pub message: Option<String>,

    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

/// Dependency configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DependencyConfig {
    /// Step name
    pub step: String,

    /// Optional condition for dependency
    #[serde(default)]
    pub condition: Option<String>,
}
```

**Commit:** `feat: add step configuration schema types`
---

## Step 5: Create src/schema/loop.rs

Define loop configuration types:

```rust
use serde::{Deserialize, Serialize};
use garde::Validate;
use schemars::JsonSchema;
use std::collections::HashMap;

/// Loop types
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LoopType {
    Count,
    Time,
    Validation,
    Retry,
    Infinite,
}

/// Count-based loop configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CountLoopConfig {
    #[serde(default = "default_max_iterations")]
    pub max_iterations: u32,

    #[serde(default)]
    pub iteration_variable: Option<String>,

    #[serde(default)]
    pub initial_value: Option<i32>,

    #[serde(default)]
    pub stop_condition: Option<String>,

    #[serde(default = "default_on_stop")]
    pub on_stop: String,
}

impl Default for CountLoopConfig {
    fn default() -> Self {
        Self {
            max_iterations: default_max_iterations(),
            iteration_variable: None,
            initial_value: None,
            stop_condition: None,
            on_stop: default_on_stop(),
        }
    }
}

fn default_max_iterations() -> u32 {
    5
}

fn default_on_stop() -> String {
    "return_result".to_string()
}

/// Time-based loop configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TimeLoopConfig {
    #[serde(default = "default_duration_seconds")]
    pub duration_seconds: u64,

    #[serde(default = "default_check_interval")]
    pub check_interval_seconds: u64,

    #[serde(default)]
    pub start_time: Option<String>,

    #[serde(default)]
    pub stop_time: Option<String>,

    #[serde(default)]
    pub interval_variable: Option<String>,

    #[serde(default)]
    pub track_progress: bool,

    #[serde(default = "default_log_interval")]
    pub log_interval: u32,
}

impl Default for TimeLoopConfig {
    fn default() -> Self {
        Self {
            duration_seconds: default_duration_seconds(),
            check_interval_seconds: default_check_interval(),
            start_time: None,
            stop_time: None,
            interval_variable: None,
            track_progress: false,
            log_interval: default_log_interval(),
        }
    }
}

fn default_duration_seconds() -> u64 {
    60
}

fn default_check_interval() -> u64 {
    10
}

fn default_log_interval() -> u32 {
    5
}

/// Exact criteria for validation loop
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExactCriteria {
    pub metric: String,

    pub operator: String,

    pub target: f64,

    #[serde(default)]
    pub description: String,
}

/// On convergence behavior
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OnConvergence {
    Continue,
    ExitLoop,
    ReturnBest,
}

/// Validation loop configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ValidationLoopConfig {
    #[serde(default)]
    pub abstract_criteria: Option<String>,

    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    #[serde(default = "default_validation_max_iterations")]
    pub max_iterations: u32,

    #[serde(default)]
    pub exact_criteria: Option<Vec<ExactCriteria>>,

    #[serde(default = "default_on_convergence")]
    pub on_convergence: OnConvergence,

    #[serde(default = "default_on_max_iterations")]
    pub on_max_iterations: String,
}

impl Default for ValidationLoopConfig {
    fn default() -> Self {
        Self {
            abstract_criteria: None,
            tolerance: default_tolerance(),
            max_iterations: default_validation_max_iterations(),
            exact_criteria: None,
            on_convergence: default_on_convergence(),
            on_max_iterations: default_on_max_iterations(),
        }
    }
}

fn default_tolerance() -> f64 {
    0.03
}

fn default_validation_max_iterations() -> u32 {
    4
}

fn default_on_convergence() -> OnConvergence {
    OnConvergence::Continue
}

fn default_on_max_iterations() -> String {
    "use_best_result".to_string()
}

/// Retry loop configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RetryLoopConfig {
    #[serde(default = "default_retry_max_attempts")]
    pub max_attempts: u32,

    #[serde(default = "default_base_delay")]
    pub base_delay_ms: u64,

    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,

    #[serde(default)]
    pub retry_strategies: Option<Vec<RetryStrategy>>,
}

impl Default for RetryLoopConfig {
    fn default() -> Self {
        Self {
            max_attempts: default_retry_max_attempts(),
            base_delay_ms: default_base_delay(),
            timeout_ms: default_timeout(),
            retry_strategies: None,
        }
    }
}

fn default_retry_max_attempts() -> u32 {
    5
}

fn default_base_delay() -> u64 {
    1000
}

fn default_timeout() -> u64 {
    30000
}

/// Retry strategy
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RetryStrategy {
    #[serde(default)]
    pub error_type: Option<String>,

    #[serde(default = "default_retry_backoff")]
    pub backoff_strategy: BackoffStrategy,

    #[serde(default)]
    pub delay_multiplier: Option<f64>,
}

fn default_retry_backoff() -> BackoffStrategy {
    BackoffStrategy::Exponential
}

/// Infinite loop configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InfiniteLoopConfig {
    #[serde(default = "default_infinite_max_iterations")]
    pub max_iterations: u32,

    #[serde(default = "default_max_memory_mb")]
    pub max_memory_mb: u64,

    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,

    #[serde(default)]
    pub iteration_variable: Option<String>,

    #[serde(default = "default_checkpoint_interval")]
    pub checkpoint_interval: u32,

    #[serde(default = "default_log_interval")]
    pub log_interval: u32,

    #[serde(default)]
    pub safety_limits: Option<SafetyLimits>,
}

impl Default for InfiniteLoopConfig {
    fn default() -> Self {
        Self {
            max_iterations: default_infinite_max_iterations(),
            max_memory_mb: default_max_memory_mb(),
            timeout_seconds: default_timeout_seconds(),
            iteration_variable: None,
            checkpoint_interval: default_checkpoint_interval(),
            log_interval: default_log_interval(),
            safety_limits: None,
        }
    }
}

fn default_infinite_max_iterations() -> u32 {
    100
}

fn default_max_memory_mb() -> u64 {
    2048
}

fn default_timeout_seconds() -> u64 {
    30
}

fn default_checkpoint_interval() -> u32 {
    5
}

fn default_log_interval() -> u32 {
    1
}

/// Safety limits
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SafetyLimits {
    #[serde(default)]
    pub max_duration_seconds: Option<u64>,

    #[serde(default)]
    pub max_tool_calls: Option<u32>,

    #[serde(default)]
    pub abort_on_oom: Option<bool>,
}

/// Loop configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct LoopConfig {
    #[serde(rename = "type")]
    pub loop_type: LoopType,

    #[serde(default)]
    pub count_config: Option<CountLoopConfig>,

    #[serde(default)]
    pub time_config: Option<TimeLoopConfig>,

    #[serde(default)]
    pub validation_config: Option<ValidationLoopConfig>,

    #[serde(default)]
    pub retry_config: Option<RetryLoopConfig>,

    #[serde(default)]
    pub infinite_config: Option<InfiniteLoopConfig>,
}
```

**Commit:** `feat: add loop configuration schema types`
---

## Step 6: Create src/schema/execution.rs

Define execution strategy types:

```rust
use serde::{Deserialize, Serialize};
use garde::Validate;
use schemars::JsonSchema;
use std::collections::HashMap;

/// Processing modes
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingMode {
    Serial,
    Parallel,
    Hybrid,
}

impl Default for ProcessingMode {
    fn default() -> Self {
        ProcessingMode::Serial
    }
}

/// Load/unload strategies
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LoadUnloadStrategy {
    OneAtATime,
    Lazy,
    Eager,
    Adaptive,
}

impl Default for LoadUnloadStrategy {
    fn default() -> Self {
        LoadUnloadStrategy::OneAtATime
    }
}

/// Allocation strategies
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AllocationStrategy {
    Static,
    Dynamic,
    Adaptive,
}

/// Pressure handling strategies
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PressureHandlingStrategy {
    Throttle,
    Swap,
    FailFast,
    GracefulDegradation,
}

/// Timeout strategy
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TimeoutStrategy {
    Fail,
    ContinueWithPartial,
    Retry,
}

/// On timeout behavior
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OnTimeout {
    Fail,
    RetryWithBackoff,
    ContinueWithPartial,
}

/// Memory configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemoryConfig {
    #[serde(default)]
    pub ram_allocation: RamAllocation,

    #[serde(default)]
    pub max_allowed: ResourceLimits,

    #[serde(default)]
    pub min_allowed: ResourceLimits,

    #[serde(default)]
    pub model_lifecycle: ModelLifecycleConfig,

    #[serde(default)]
    pub pressure_handling: PressureHandlingConfig,
}

/// Model lifecycle configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModelLifecycleConfig {
    #[serde(default = "default_load_unload_strategy")]
    pub load_unload_strategy: LoadUnloadStrategy,

    #[serde(default = "default_cache_size")]
    pub cache_size: String,

    #[serde(default = "default_swap_timeout")]
    pub swap_timeout_secs: u64,

    #[serde(default)]
    pub unload_unused: bool,

    #[serde(default = "default_gc_interval")]
    pub gc_interval_secs: u64,
}

impl Default for ModelLifecycleConfig {
    fn default() -> Self {
        Self {
            load_unload_strategy: default_load_unload_strategy(),
            cache_size: default_cache_size(),
            swap_timeout_secs: default_swap_timeout(),
            unload_unused: true,
            gc_interval_secs: default_gc_interval(),
        }
    }
}

fn default_load_unload_strategy() -> LoadUnloadStrategy {
    LoadUnloadStrategy::OneAtATime
}

fn default_cache_size() -> String {
    "min".to_string()
}

fn default_swap_timeout() -> u64 {
    30
}

fn default_gc_interval() -> u64 {
    300
}

/// Pressure handling configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PressureHandlingConfig {
    #[serde(default = "default_trigger_threshold")]
    pub trigger_threshold_percent: u32,

    #[serde(default = "default_pressure_strategy")]
    pub strategy: PressureHandlingStrategy,

    #[serde(default = "default_throttle_factor")]
    pub throttle_factor: f64,

    #[serde(default = "default_on_oom")]
    pub on_oom: PressureHandlingStrategy,
}

impl Default for PressureHandlingConfig {
    fn default() -> Self {
        Self {
            trigger_threshold_percent: default_trigger_threshold(),
            strategy: default_pressure_strategy(),
            throttle_factor: default_throttle_factor(),
            on_oom: default_on_oom(),
        }
    }
}

fn default_trigger_threshold() -> u32 {
    85
}

fn default_pressure_strategy() -> PressureHandlingStrategy {
    PressureHandlingStrategy::Throttle
}

fn default_throttle_factor() -> f64 {
    0.5
}

fn default_on_oom() -> PressureHandlingStrategy {
    PressureHandlingStrategy::Swap
}

/// Timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TimeoutConfig {
    #[serde(default)]
    pub total: Option<String>,

    #[serde(default)]
    pub per_operation: Option<PerOperationTimeout>,

    #[serde(default = "default_timeout_strategy")]
    pub timeout_strategy: TimeoutStrategy,

    #[serde(default)]
    pub model: Option<ModelTimeoutConfig>,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            total: None,
            per_operation: None,
            timeout_strategy: default_timeout_strategy(),
            model: None,
        }
    }
}

fn default_timeout_strategy() -> TimeoutStrategy {
    TimeoutStrategy::ContinueWithPartial
}

/// Per-operation timeout
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PerOperationTimeout {
    #[serde(default)]
    pub tool_call: Option<String>,

    #[serde(default)]
    pub step: Option<String>,

    #[serde(default)]
    pub operation_timeout_secs: Option<u64>,

    #[serde(default)]
    pub sub_workflow: Option<u64>,

    #[serde(default)]
    pub time_to_first_result_secs: Option<u64>,
}

/// Model timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModelTimeoutConfig {
    #[serde(default)]
    pub load_into_memory: Option<String>,

    #[serde(default)]
    pub time_to_processing_after_loaded: Option<u64>,

    #[serde(default)]
    pub time_to_processing: Option<String>,

    #[serde(default)]
    pub time_to_responding: Option<String>,

    #[serde(default = "default_model_on_timeout")]
    pub on_timeout: OnTimeout,
}

impl Default for ModelTimeoutConfig {
    fn default() -> Self {
        Self {
            load_into_memory: None,
            time_to_processing_after_loaded: None,
            time_to_processing: None,
            time_to_responding: None,
            on_timeout: default_model_on_timeout(),
        }
    }
}

fn default_model_on_timeout() -> OnTimeout {
    OnTimeout::RetryWithBackoff
}

/// Parallel configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParallelConfig {
    #[serde(default = "default_algorithm")]
    pub algorithm: String,

    #[serde(default)]
    pub load_balancing: Option<LoadBalancingConfig>,

    #[serde(default = "default_max_threads")]
    pub max_threads: u32,

    #[serde(default = "default_max_models")]
    pub max_models: u32,

    #[serde(default = "default_max_concurrent_requests")]
    pub max_concurrent_requests: u32,

    #[serde(default = "default_max_steps")]
    pub max_steps: u32,

    #[serde(default = "default_max_sub_agents")]
    pub max_sub_agents: u32,

    #[serde(default = "default_parallel_group_timeout")]
    pub parallel_group_timeout_secs: u64,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: default_algorithm(),
            load_balancing: None,
            max_threads: default_max_threads(),
            max_models: default_max_models(),
            max_concurrent_requests: default_max_concurrent_requests(),
            max_steps: default_max_steps(),
            max_sub_agents: default_max_sub_agents(),
            parallel_group_timeout_secs: default_parallel_group_timeout(),
        }
    }
}

fn default_algorithm() -> String {
    "round_robin".to_string()
}

fn default_max_threads() -> u32 {
    4
}

fn default_max_models() -> u32 {
    3
}

fn default_max_concurrent_requests() -> u32 {
    2
}

fn default_max_steps() -> u32 {
    2
}

fn default_max_sub_agents() -> u32 {
    2
}

fn default_parallel_group_timeout() -> u64 {
    600
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LoadBalancingConfig {
    #[serde(default = "default_lb_strategy")]
    pub strategy: String,

    #[serde(default = "default_worker_selection_timeout")]
    pub worker_selection_timeout_secs: u64,
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            strategy: default_lb_strategy(),
            worker_selection_timeout_secs: default_worker_selection_timeout(),
            enabled: false,
        }
    }
}

fn default_lb_strategy() -> String {
    "least_loaded".to_string()
}

fn default_worker_selection_timeout() -> u64 {
    10
}

/// Workflow execution strategy
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct WorkflowExecutionStrategy {
    #[serde(default = "default_processing_mode")]
    pub processing: ProcessingMode,

    #[serde(default = "default_load_unload")]
    pub load_unload: LoadUnloadStrategy,

    #[serde(default)]
    pub memory: Option<MemoryConfig>,

    #[serde(default)]
    pub timeout: Option<TimeoutConfig>,

    #[serde(default)]
    pub parallel: Option<ParallelConfig>,
}

fn default_processing_mode() -> ProcessingMode {
    ProcessingMode::Serial
}

fn default_load_unload() -> LoadUnloadStrategy {
    LoadUnloadStrategy::OneAtATime
}
```

**Commit:** `feat: add execution strategy schema types`
---

## Step 7: Create src/schema/mod.rs

Create module exports and main WorkflowSpec:

```rust
use serde::{Deserialize, Serialize};
use garde::Validate;
use schemars::JsonSchema;
use std::collections::HashMap;

pub mod identification;
pub mod model;
pub mod workspace;
pub mod step;
pub mod loop_config;
pub mod execution;

pub use identification::WorkflowIdentification;
pub use model::{
    ModelConfig, ModelProvider, ResourceLimit, ResourceLimits,
    ModelHost, ModelMemory, ExecutionTimeouts, ThinkingConfig,
    ToolPermissions, PermissionLevel, Guardrails, EnforcementPolicy,
    InputGuards, OutputGuards, ToolUseGuards,
    PromptInjectionGuard, PiiRedactionGuard, MaxLengthGuard,
    ToxicityFilterGuard, FormatValidationGuard,
    LifecycleHooks, RamAllocation,
};
pub use workspace::WorkspaceConfig;
pub use step::{
    Step, OutputFormat, BackoffStrategy,
    RetryConfig, ModelOverrides, UserInput, DependencyConfig,
};
pub use loop_config::{
    LoopConfig, LoopType, CountLoopConfig, TimeLoopConfig,
    ValidationLoopConfig, ExactCriteria, OnConvergence,
    RetryLoopConfig, RetryStrategy, InfiniteLoopConfig, SafetyLimits,
};
pub use execution::{
    WorkflowExecutionStrategy, ProcessingMode, LoadUnloadStrategy,
    AllocationStrategy, PressureHandlingStrategy, TimeoutStrategy, OnTimeout,
    MemoryConfig, ModelLifecycleConfig, PressureHandlingConfig,
    TimeoutConfig, PerOperationTimeout, ModelTimeoutConfig,
    ParallelConfig, LoadBalancingConfig,
};

/// User input types
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserInputType {
    TextArea,
    TextLine,
    Select,
    MultiSelect,
    Confirm,
}

/// User input validation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserInputValidation {
    #[serde(default)]
    pub required: bool,

    #[serde(default)]
    pub pattern: Option<String>,

    #[serde(default)]
    pub error_message: Option<String>,
}

/// User input option (for select/multi-select)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserInputOption {
    pub value: String,

    #[serde(default)]
    pub label: String,

    #[serde(default)]
    pub description: Option<String>,
}

/// User input definition
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserInput {
    pub id: String,

    #[serde(rename = "type")]
    pub input_type: UserInputType,

    #[serde(default)]
    pub label: String,

    #[serde(default)]
    pub placeholder: Option<String>,

    #[serde(default)]
    pub default_value: Option<serde_json::Value>,

    #[serde(default)]
    pub multiline: bool,

    #[serde(default)]
    pub validation: Option<UserInputValidation>,

    #[serde(default)]
    pub options: Option<Vec<UserInputOption>>,

    #[serde(default)]
    pub required: bool,

    #[serde(default)]
    pub keyboard_shortcut: Option<String>,

    #[serde(default)]
    pub cancel_shortcut: Option<String>,

    #[serde(default)]
    pub message: Option<String>,
}

/// User inputs configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserInputs {
    #[serde(default)]
    pub execution_mode: String,

    #[serde(default)]
    pub workflow_level: WorkflowLevelInputs,

    #[serde(default)]
    pub step_level: StepLevelInputs,

    #[serde(default)]
    pub ui_configuration: Option<UiConfiguration>,
}

impl Default for UserInputs {
    fn default() -> Self {
        Self {
            execution_mode: "interactive".to_string(),
            workflow_level: WorkflowLevelInputs::default(),
            step_level: StepLevelInputs::default(),
            ui_configuration: None,
        }
    }
}

/// Workflow-level inputs
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WorkflowLevelInputs {
    #[serde(default)]
    pub ask_at_start: bool,

    #[serde(default)]
    pub inputs: Vec<UserInput>,
}

impl Default for WorkflowLevelInputs {
    fn default() -> Self {
        Self {
            ask_at_start: true,
            inputs: Vec::new(),
        }
    }
}

/// Step-level inputs
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StepLevelInputs {
    #[serde(default)]
    pub ask_at_step_start: bool,

    #[serde(default)]
    pub inputs_by_step: HashMap<String, Vec<UserInput>>,
}

impl Default for StepLevelInputs {
    fn default() -> Self {
        Self {
            ask_at_step_start: true,
            inputs_by_step: HashMap::new(),
        }
    }
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UiConfiguration {
    #[serde(default = "default_ui_layout")]
    pub layout: String,

    #[serde(default)]
    pub panels: Vec<UiPanel>,

    #[serde(default)]
    pub notifications: Option<UiNotifications>,
}

impl Default for UiConfiguration {
    fn default() -> Self {
        Self {
            layout: default_ui_layout(),
            panels: Vec::new(),
            notifications: None,
        }
    }
}

fn default_ui_layout() -> String {
    "tabbed".to_string()
}

/// UI panel
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UiPanel {
    pub id: String,

    #[serde(default)]
    pub label: String,

    #[serde(default)]
    pub inputs: Vec<String>,
}

/// UI notifications
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UiNotifications {
    #[serde(default)]
    pub sound: bool,

    #[serde(default)]
    pub position: String,
}

/// Hardcoded values accessible via interpolation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HardcodedValues {
    #[serde(default)]
    pub values: HashMap<String, serde_json::Value>,
}

impl Default for HardcodedValues {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}

/// Agentic workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct AgenticWorkflow {
    #[serde(default)]
    pub hardcoded_values: HardcodedValues,

    #[serde(default)]
    pub user_inputs: UserInputs,

    #[serde(default)]
    pub steps: HashMap<String, Step>,
}

impl Default for AgenticWorkflow {
    fn default() -> Self {
        Self {
            hardcoded_values: HardcodedValues::default(),
            user_inputs: UserInputs::default(),
            steps: HashMap::new(),
        }
    }
}

/// Pipeline step (array format)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct PipelineStep {
    #[serde(flatten)]
    pub step: Step,
}

/// Pipeline configuration (array format)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct Pipeline {
    #[serde(default)]
    pub steps: Vec<PipelineStep>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
        }
    }
}

/// Models configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct ModelsConfig {
    #[serde(default)]
    pub global_config_path: Option<String>,

    #[serde(default = "default_default_router")]
    pub default_router: String,

    #[serde(flatten)]
    pub models: HashMap<String, ModelConfig>,
}

impl Default for ModelsConfig {
    fn default() -> Self {
        Self {
            global_config_path: None,
            default_router: default_default_router(),
            models: HashMap::new(),
        }
    }
}

fn default_default_router() -> String {
    "automatic".to_string()
}

/// Logging level
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub default: LogLevel,

    #[serde(default)]
    pub levels: HashMap<String, LogLevel>,

    #[serde(default)]
    pub scopes: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub output: LoggingOutput,

    #[serde(default)]
    pub errors: LoggingErrors,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            default: default_log_level(),
            levels: HashMap::new(),
            scopes: HashMap::new(),
            output: LoggingOutput::default(),
            errors: LoggingErrors::default(),
        }
    }
}

fn default_log_level() -> LogLevel {
    LogLevel::Info
}

/// Logging output configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LoggingOutput {
    #[serde(default)]
    pub console: ConsoleOutputConfig,

    #[serde(default)]
    pub file: FileOutput,

    #[serde(default)]
    pub remote: Option<RemoteOutput>,
}

impl Default for LoggingOutput {
    fn default() -> Self {
        Self {
            console: ConsoleOutputConfig::default(),
            file: FileOutput::default(),
            remote: None,
        }
    }
}

/// Console output configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConsoleOutputConfig {
    #[serde(default)]
    pub color: bool,

    #[serde(default)]
    pub timestamps: bool,

    #[serde(default = "default_format")]
    pub format: String,
}

impl Default for ConsoleOutputConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            color: true,
            timestamps: true,
            format: default_format(),
        }
    }
}

fn default_format() -> String {
    "text".to_string()
}

/// File output configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileOutput {
    #[serde(default)]
    pub path: String,

    #[serde(default = "default_log_format")]
    pub format: String,

    #[serde(default)]
    pub rotation: Option<LogRotation>,
}

impl Default for FileOutput {
    fn default() -> Self {
        Self {
            enabled: false,
            path: "./logs/workflow.log".to_string(),
            format: default_log_format(),
            rotation: None,
        }
    }
}

fn default_log_format() -> String {
    "json".to_string()
}

/// Log rotation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LogRotation {
    #[serde(default = "default_max_size_mb")]
    pub max_size_mb: u64,

    #[serde(default = "default_max_files")]
    pub max_files: u32,
}

impl Default for LogRotation {
    fn default() -> Self {
        Self {
            enabled: false,
            max_size_mb: default_max_size_mb(),
            max_files: default_max_files(),
        }
    }
}

fn default_max_size_mb() -> u64 {
    100
}

fn default_max_files() -> u32 {
    10
}

/// Remote output configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RemoteOutput {
    #[serde(default)]
    pub webhook_url: String,

    #[serde(default)]
    pub headers: HashMap<String, String>,
}

/// Logging errors configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LoggingErrors {
    #[serde(default)]
    pub log_parsing_errors: bool,

    #[serde(default)]
    pub hardware_limitation_errors: bool,

    #[serde(default)]
    pub runtime_errors: bool,

    #[serde(default)]
    pub validation_errors: bool,

    #[serde(default = "default_error_log_file")]
    pub error_log_file: String,
}

impl Default for LoggingErrors {
    fn default() -> Self {
        Self {
            log_parsing_errors: true,
            hardware_limitation_errors: true,
            runtime_errors: true,
            validation_errors: true,
            error_log_file: default_error_log_file(),
        }
    }
}

fn default_error_log_file() -> String {
    "./logs/errors.log".to_string()
}

/// Tool permissions configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolPermissionsConfig {
    #[serde(default)]
    pub file_operations: FileOperationsPermissions,

    #[serde(default)]
    pub web_operations: WebOperationsPermissions,

    #[serde(default)]
    pub shell_operations: ShellOperationsPermissions,

    #[serde(default)]
    pub ai_operations: AiOperationsPermissions,

    #[serde(default)]
    pub system_operations: SystemOperationsPermissions,
}

/// File operations permissions
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileOperationsPermissions {
    #[serde(default)]
    pub read: ReadPermission,

    #[serde(default)]
    pub write: WritePermission,

    #[serde(default)]
    pub delete: DeletePermission,
}

impl Default for FileOperationsPermissions {
    fn default() -> Self {
        Self {
            read: ReadPermission::default(),
            write: WritePermission::default(),
            delete: DeletePermission::default(),
        }
    }
}

/// Read permission
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReadPermission {
    #[serde(default)]
    pub require_confirmation: bool,

    #[serde(default)]
    pub allowed_paths: Vec<String>,

    #[serde(default)]
    pub allowed_patterns: Vec<String>,

    #[serde(default)]
    pub forbidden_paths: Vec<String>,

    #[serde(default = "default_max_file_size_mb")]
    pub max_file_size_mb: u64,
}

impl Default for ReadPermission {
    fn default() -> Self {
        Self {
            enabled: false,
            require_confirmation: false,
            allowed_paths: Vec::new(),
            allowed_patterns: Vec::new(),
            forbidden_paths: Vec::new(),
            max_file_size_mb: default_max_file_size_mb(),
        }
    }
}

fn default_max_file_size_mb() -> u64 {
    100
}

/// Write permission
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WritePermission {
    #[serde(default)]
    pub require_confirmation: bool,

    #[serde(default)]
    pub allowed_paths: Vec<String>,

    #[serde(default)]
    pub backup_existing: bool,

    #[serde(default = "default_write_max_file_size_mb")]
    pub max_file_size_mb: u64,

    #[serde(default)]
    pub create_parent_directories: bool,
}

impl Default for WritePermission {
    fn default() -> Self {
        Self {
            enabled: false,
            require_confirmation: true,
            allowed_paths: Vec::new(),
            backup_existing: true,
            max_file_size_mb: default_write_max_file_size_mb(),
            create_parent_directories: true,
        }
    }
}

fn default_write_max_file_size_mb() -> u64 {
    500
}

/// Delete permission
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeletePermission {
    #[serde(default)]
    pub require_confirmation: bool,

    #[serde(default)]
    pub allowed_paths: Vec<String>,

    #[serde(default = "default_confirm_delete_count")]
    pub confirm_delete_count: u32,

    #[serde(default)]
    pub skip_confirmation_for_patterns: Vec<String>,
}

impl Default for DeletePermission {
    fn default() -> Self {
        Self {
            enabled: false,
            require_confirmation: true,
            allowed_paths: Vec::new(),
            confirm_delete_count: default_confirm_delete_count(),
            skip_confirmation_for_patterns: Vec::new(),
        }
    }
}

fn default_confirm_delete_count() -> u32 {
    5
}

/// Web operations permissions
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebOperationsPermissions {
    #[serde(default)]
    pub fetch: WebFetchPermission,

    #[serde(default)]
    pub scrape: WebScrapePermission,
}

impl Default for WebOperationsPermissions {
    fn default() -> Self {
        Self {
            fetch: WebFetchPermission::default(),
            scrape: WebScrapePermission::default(),
        }
    }
}

/// Web fetch permission
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebFetchPermission {
    #[serde(default)]
    pub require_confirmation: bool,

    #[serde(default = "default_max_concurrent_requests")]
    pub max_concurrent_requests: u32,

    #[serde(default = "default_web_timeout_secs")]
    pub timeout_secs: u64,

    #[serde(default)]
    pub allowed_domains: Vec<String>,

    #[serde(default)]
    pub forbidden_domains: Vec<String>,

    #[serde(default)]
    pub follow_redirects: bool,

    #[serde(default)]
    pub validate_ssl: bool,
}

impl Default for WebFetchPermission {
    fn default() -> Self {
        Self {
            enabled: false,
            require_confirmation: false,
            max_concurrent_requests: default_max_concurrent_requests(),
            timeout_secs: default_web_timeout_secs(),
            allowed_domains: Vec::new(),
            forbidden_domains: Vec::new(),
            follow_redirects: true,
            validate_ssl: true,
        }
    }
}

fn default_max_concurrent_requests() -> u32 {
    5
}

fn default_web_timeout_secs() -> u64 {
    30
}

/// Web scrape permission
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebScrapePermission {
    #[serde(default)]
    pub require_confirmation: bool,

    #[serde(default)]
    pub respect_robots_txt: bool,

    #[serde(default = "default_max_pages_per_domain")]
    pub max_pages_per_domain: u32,

    #[serde(default)]
    pub follow_links: FollowLinksConfig,
}

impl Default for WebScrapePermission {
    fn default() -> Self {
        Self {
            enabled: false,
            require_confirmation: false,
            respect_robots_txt: true,
            max_pages_per_domain: default_max_pages_per_domain(),
            follow_links: FollowLinksConfig::default(),
        }
    }
}

fn default_max_pages_per_domain() -> u32 {
    100
}

/// Follow links configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FollowLinksConfig {
}

    #[serde(default = "default_max_depth")]
    pub max_depth: u32,
}

impl Default for FollowLinksConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_depth: default_max_depth(),
        }
    }
}

fn default_max_depth() -> u32 {
    2
}

/// Shell operations permissions
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShellOperationsPermissions {
    #[serde(default)]
    pub exec: ShellExecPermission,
}

impl Default for ShellOperationsPermissions {
    fn default() -> Self {
        Self {
            exec: ShellExecPermission::default(),
        }
    }
}

/// Shell exec permission
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShellExecPermission {
    #[serde(default)]
    pub require_confirmation: bool,

    #[serde(default = "default_shell_timeout_seconds")]
    pub timeout_seconds: u64,

    #[serde(default)]
    pub allowed_commands: Vec<String>,

    #[serde(default)]
    pub forbidden_commands: Vec<String>,

    #[serde(default)]
    pub working_directories: Vec<String>,

    #[serde(default)]
    pub require_shell_for: Vec<String>,
}

impl Default for ShellExecPermission {
    fn default() -> Self {
        Self {
            enabled: false,
            require_confirmation: true,
            timeout_seconds: default_shell_timeout_seconds(),
            allowed_commands: Vec::new(),
            forbidden_commands: Vec::new(),
            working_directories: Vec::new(),
            require_shell_for: Vec::new(),
        }
    }
}

fn default_shell_timeout_seconds() -> u64 {
    30
}

/// AI operations permissions
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AiOperationsPermissions {
    #[serde(default)]
    pub web_search: WebSearchPermission,

    #[serde(default)]
    pub content_generation: ContentGenerationPermission,
}

impl Default for AiOperationsPermissions {
    fn default() -> Self {
        Self {
            web_search: WebSearchPermission::default(),
            content_generation: ContentGenerationPermission::default(),
        }
    }
}

/// Web search permission
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchPermission {
    #[serde(default)]
    pub require_confirmation: bool,

    #[serde(default = "default_max_searches_per_hour")]
    pub max_searches_per_hour: u32,
}

impl Default for WebSearchPermission {
    fn default() -> Self {
        Self {
            enabled: false,
            require_confirmation: false,
            max_searches_per_hour: default_max_searches_per_hour(),
        }
    }
}

fn default_max_searches_per_hour() -> u32 {
    100
}

/// Content generation permission
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContentGenerationPermission {
    #[serde(default)]
    pub require_confirmation: bool,

    #[serde(default = "default_max_tokens_per_hour")]
    pub max_tokens_per_hour: u32,
}

impl Default for ContentGenerationPermission {
    fn default() -> Self {
        Self {
            enabled: false,
            require_confirmation: false,
            max_tokens_per_hour: default_max_tokens_per_hour(),
        }
    }
}

fn default_max_tokens_per_hour() -> u32 {
    50000
}

/// System operations permissions
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SystemOperationsPermissions {
    #[serde(default)]
    pub process_management: ProcessManagementPermission,

    #[serde(default)]
    pub network_operations: NetworkOperationsPermission,
}

impl Default for SystemOperationsPermissions {
    fn default() -> Self {
        Self {
            process_management: ProcessManagementPermission::default(),
            network_operations: NetworkOperationsPermission::default(),
        }
    }
}

/// Process management permission
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessManagementPermission {
}

impl Default for ProcessManagementPermission {
    fn default() -> Self {
        Self { enabled: false }
    }
}

/// Network operations permission
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NetworkOperationsPermission {
}

impl Default for NetworkOperationsPermission {
    fn default() -> Self {
        Self { enabled: false }
    }
}

/// Complete workflow specification
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct WorkflowSpec {
    /// Workflow identification
    pub identification: WorkflowIdentification,

    /// Model configuration
    #[serde(default)]
    pub models: ModelsConfig,

    /// Sub-workflow definitions
    #[serde(default)]
    pub sub_workflows: SubWorkflowsConfig,

    /// Agentic workflow (preferred format)
    #[serde(default)]
    pub agentic_workflow: AgenticWorkflow,

    /// Workflow execution strategy
    #[serde(default)]
    pub workflow_execution_strategy: WorkflowExecutionStrategy,

    /// Tool permissions
    #[serde(default)]
    pub tool_permissions: ToolPermissionsConfig,

    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,

    /// Memory configuration (RAG)
    #[serde(default)]
    pub memory: Option<MemoryConfig>,

    /// Workspace configuration
    #[serde(default)]
    pub workspace: WorkspaceConfig,
}

/// Sub-workflows configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubWorkflowsConfig {
    #[serde(flatten)]
    pub workflows: HashMap<String, serde_json::Value>,
}

impl Default for SubWorkflowsConfig {
    fn default() -> Self {
        Self {
            workflows: HashMap::new(),
        }
    }
}

/// Memory configuration (RAG)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemoryConfig {
    /// RAG configuration (presence = RAG enabled)
    #[serde(default)]
    pub rag: Option<RagConfig>,
}

/// RAG configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RagConfig {
    #[serde(default)]
    pub knowledge_base: KnowledgeBaseConfig,

    #[serde(default)]
    pub embedding_model: EmbeddingModelConfig,

    #[serde(default)]
    pub retrieval: RetrievalConfig,
}

/// Knowledge base configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct KnowledgeBaseConfig {
    #[serde(default)]
    pub path: String,

    #[serde(default = "default_format")]
    pub format: String,

    #[serde(default = "default_chunk_size")]
    pub chunk_size: u32,

    #[serde(default = "default_chunk_overlap")]
    pub chunk_overlap: u32,
}

fn default_format() -> String {
    "vector_db".to_string()
}

fn default_chunk_size() -> u32 {
    512
}

fn default_chunk_overlap() -> u32 {
    50
}

/// Embedding model configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EmbeddingModelConfig {
    #[serde(default)]
    pub model_ref: String,

    #[serde(default = "default_dimension")]
    pub dimension: u32,

    #[serde(default = "default_batch_size")]
    pub batch_size: u32,
}

fn default_dimension() -> u32 {
    768
}

fn default_batch_size() -> u32 {
    32
}

/// Retrieval configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RetrievalConfig {
    #[serde(default = "default_max_results")]
    pub max_results: u32,

    #[serde(default = "default_similarity_threshold")]
    pub similarity_threshold: f64,

    #[serde(default)]
    pub include_sources: bool,
}

impl Default for RetrievalConfig {
    fn default() -> Self {
        Self {
            max_results: default_max_results(),
            similarity_threshold: default_similarity_threshold(),
            include_sources: true,
        }
    }
}

fn default_max_results() -> u32 {
    10
}

fn default_similarity_threshold() -> f64 {
    0.7
}

/// When configuration (hooks)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WhenConfig {
    #[serde(default)]
    pub before_step_starts: Vec<HookAction>,

    #[serde(default)]
    pub during_step_streaming: Vec<HookAction>,

    #[serde(default)]
    pub after_step_succeeds: Vec<HookAction>,

    #[serde(default)]
    pub after_step_fails: Vec<HookAction>,

    #[serde(default)]
    pub after_all_retries_exhausted: Vec<HookAction>,

    #[serde(default)]
    pub gwt: Option<Vec<GwtRule>>,

    #[serde(default)]
    pub requires: Vec<DependencyConfig>,

    #[serde(default)]
    pub on_requires_failed: Vec<HookAction>,
}

/// Hook action
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HookAction {
    #[serde(flatten)]
    pub action_type: HookActionType,
}

/// Hook action type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum HookActionType {
    Log { to_file_path: String, event_fields: Vec<String> },
    AppendTo(String),
    SaveTo(String),
    Notify { message: String },
    Fail(String),
    Bookmark { },
}

/// GWT (Given When Then) rule
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GwtRule {
    pub given: String,
    #[serde(default)]
    pub when: Option<String>,
    pub then: ThenClause,
}

/// Then clause
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ThenClause {
    RouteTo(String),
    RouteToMultiple(Vec<String>),
    Custom(serde_json::Value),
}
```

**Commit:** `feat: add complete WorkflowSpec with all schema types`

---

## Step 9: Update src/lib.rs

Export schema module:

```rust
pub mod schema;

pub use schema::WorkflowSpec;
```

**Commit:** `chore: export schema module from lib.rs`
---

## Step 8: Write schema tests

Create `tests/schema_test.rs`:

```rust
use yaml_to_rust_agentsdk::schema::*;

#[test]
fn test_workflow_identification() {
    let id = WorkflowIdentification {
        workflow_id: "test_workflow".to_string(),
        name: "Test Workflow".to_string(),
        description: "A test workflow".to_string(),
        version: "1.0.0".to_string(),
        author: "Test Author".to_string(),
        tags: vec!["test".to_string(), "example".to_string()],
    };

    assert_eq!(id.workflow_id, "test_workflow");
    assert_eq!(id.version, "1.0.0");
}

#[test]
fn test_model_config() {
    let model = ModelConfig {
        name: "test_model".to_string(),
        host: ModelHost {
            provider_type: ModelProvider::LmStudio,
            connection_settings: Default::default(),
        },
        ram_allocation: RamAllocation::default(),
        max_allowed: ResourceLimits::default(),
        min_allowed: ResourceLimits::default(),
        model_memory: ModelMemory::default(),
        execution: ExecutionTimeouts::default(),
        thinking: ThinkingConfig::default(),
        tools: ToolPermissions::default(),
        hooks: LifecycleHooks::default(),
        guardrails: None,
    };

    assert_eq!(model.name, "test_model");
    assert_eq!(model.ram_allocation.strategy, "dynamic");
}

#[test]
fn test_step_config() {
    let step = Step {
        step: "step_1".to_string(),
        generative_entity: Some("${models.primary}".to_string()),
        prompt: Some("Test prompt".to_string()),
        tool: None,
        input: None,
        sub_workflow: None,
        loop_config: None,
        when: None,
        model_overrides: None,
        user_input: None,
        depends_on: None,
        parallel_group: None,
        retry: None,
        timeout_secs: None,
    };

    assert_eq!(step.step, "step_1");
    assert!(step.generative_entity.is_some());
}

#[test]
fn test_retry_config_defaults() {
    let retry = RetryConfig::default();
    assert_eq!(retry.max_attempts, 3);
    assert_eq!(retry.backoff, BackoffStrategy::Exponential);
    assert_eq!(retry.delay_ms, 1000);
}

#[test]
fn test_resource_limit() {
    let percentage = ResourceLimit::Percentage(50.0);
    let absolute = ResourceLimit::Absolute {
        value: 8,
        unit: "GB".to_string(),
    };

    match percentage {
        ResourceLimit::Percentage(p) => assert_eq!(p, 50.0),
        _ => panic!("Expected Percentage"),
    }

    match absolute {
        ResourceLimit::Absolute { value, unit } => {
            assert_eq!(value, 8);
            assert_eq!(unit, "GB");
        }
        _ => panic!("Expected Absolute"),
    }
}

#[test]
fn test_workflow_spec_serialization() {
    let spec = WorkflowSpec {
        identification: WorkflowIdentification {
            workflow_id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test".to_string(),
            version: "1.0.0".to_string(),
            author: "".to_string(),
            tags: vec![],
        },
        models: ModelsConfig::default(),
        sub_workflows: SubWorkflowsConfig::default(),
        agentic_workflow: AgenticWorkflow::default(),
        workflow_execution_strategy: WorkflowExecutionStrategy::default(),
        tool_permissions: ToolPermissionsConfig::default(),
        logging: LoggingConfig::default(),
        memory: None,
        workspace: WorkspaceConfig {
            root_path: "/test".to_string(),
            ..Default::default()
        },
    };

    // Test serialization to JSON
    let json = serde_json::to_string(&spec).unwrap();
    assert!(json.contains("workflow_id"));

    // Test deserialization
    let deserialized: WorkflowSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.identification.workflow_id, "test");
}
```

**Commit:** `test: add schema type tests`
---

## Step 9: Run tests

Verify all tests pass:

```bash
cargo test schema_test

# Expected output:
# test result: ok. X passed in Y.ZZs
```

**Commit:** `fix: resolve any test failures`

---

## Verification

After completing all steps, verify:

```bash
# 1. Build passes
cargo build
# Expected: Finished dev [unoptimized + debuginfo] target(s)

# 2. All tests pass
cargo test schema_test
# Expected: test result: ok. X passed

# 3. Schema generation works
cargo run --example generate_schema
# Expected: Generates JSON schema for WorkflowSpec

# 4. All schema types are covered
# Check that all Sections 1,2,13-18 are implemented
```

**Checkpoint Criteria:**
- ✅ All schema types defined (Sections 1,2,13-18)
- ✅ serde derives generate correct Serialize/Deserialize
- ✅ garde validation rules compile
- ✅ schemars generates valid JSON schema
- ✅ All default values implemented
- ✅ Schema tests created and passing
- ✅ WorkflowSpec compiles successfully
- ✅ No unused fields or dead code warnings

**Anti-Drift Check:** Verify task 2 implements ONLY schema types. No parser, IR, interpolation, or validation code yet.

**Next:** Proceed to Task 3 (YAML Parser)
