use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::schema::{
    BackoffStrategy, CacheSize, CheckpointLevel, ConflictResolution,
    DependencyFailureAction, DependencyStrategy, EventPropagation, LoadUnloadStrategy,
    PressureStrategy, RamAllocationStrategy, TimeoutStrategy, ErrorAction,
};

/// Workflow execution strategy.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct WorkflowExecutionStrategy {
    #[serde(default)]
    pub load_unload: Option<LoadUnloadStrategy>,
    #[serde(default)]
    pub memory: Option<ExecutionMemoryConfig>,
    #[serde(default)]
    pub timeout: Option<TimeoutConfig>,
    #[serde(default)]
    pub error_handling: Option<ErrorHandlingConfig>,
    #[serde(default)]
    pub sub_workflow: Option<SubWorkflowExecutionConfig>,
    #[serde(default)]
    pub checkpointing: Option<CheckpointingConfig>,
    #[serde(default)]
    pub synchronization: Option<SynchronizationConfig>,
    #[serde(default)]
    pub dependency_resolution: Option<DependencyResolutionConfig>,
}

/// Execution memory configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExecutionMemoryConfig {
    #[serde(default)]
    pub ram_allocation: Option<RamAllocationConfig>,
    #[serde(default)]
    pub model_lifecycle: Option<ModelLifecycleConfig>,
    #[serde(default)]
    pub pressure_handling: Option<PressureHandlingConfig>,
}

/// RAM allocation configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct RamAllocationConfig {
    #[serde(default = "default_ram_strategy")]
    pub strategy: RamAllocationStrategy,
    #[serde(default)]
    pub max_allowed: Option<ResourceLimits>,
    #[serde(default)]
    pub min_allowed: Option<ResourceLimits>,
}

/// Resource limits.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ResourceLimits {
    #[serde(default)]
    pub ram: Option<String>,
    #[serde(default)]
    pub vram: Option<String>,
    #[serde(default)]
    pub cpu: Option<String>,
    #[serde(default)]
    pub gpu: Option<String>,
}

/// Model lifecycle configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ModelLifecycleConfig {
    #[serde(default = "default_load_unload_strategy")]
    pub load_unload_strategy: LoadUnloadStrategy,
    #[serde(default = "default_cache_size")]
    pub cache_size: CacheSize,
    #[serde(default)]
    pub swap_timeout_secs: Option<u64>,
    #[serde(default)]
    pub unload_unused: Option<bool>,
}

/// Pressure handling configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct PressureHandlingConfig {
    #[serde(default)]
    pub trigger_threshold_percent: Option<u32>,
    #[serde(default = "default_pressure_strategy")]
    pub strategy: PressureStrategy,
    #[serde(default)]
    pub throttle_factor: Option<f32>,
    #[serde(default = "default_pressure_strategy")]
    pub on_oom: PressureStrategy,
}

/// Timeout configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct TimeoutConfig {
    #[serde(default)]
    pub total: Option<String>,
    #[serde(default)]
    pub per_operation: Option<PerOperationTimeout>,
    #[serde(default = "default_timeout_strategy")]
    pub timeout_strategy: TimeoutStrategy,
}

/// Per-operation timeout.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct PerOperationTimeout {
    #[serde(default)]
    pub tool_call: Option<String>,
    #[serde(default)]
    pub step: Option<String>,
    #[serde(default)]
    pub sub_workflow: Option<String>,
    #[serde(default)]
    pub time_to_first_result_secs: Option<u64>,
}

/// Error handling configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ErrorHandlingConfig {
    #[serde(default)]
    pub default_action: Option<DefaultActionConfig>,
    #[serde(default)]
    pub retry: Option<RetryConfig>,
    #[serde(default)]
    pub escalation: Option<EscalationConfig>,
    #[serde(default)]
    pub handling_by_type: Option<HashMap<String, ErrorTypeHandling>>,
}

/// Default action configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct DefaultActionConfig {
    #[serde(default = "default_error_action")]
    pub r#type: ErrorAction,
}

/// Retry configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct RetryConfig {
    #[serde(default)]
    pub max_attempts_per_step: Option<u32>,
    #[serde(default)]
    pub max_attempts_per_workflow: Option<u32>,
    #[serde(default)]
    pub backoff: Option<BackoffStrategyConfig>,
}

/// Backoff strategy configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct BackoffStrategyConfig {
    #[serde(default = "default_backoff_strategy")]
    pub backoff: BackoffStrategy,
    #[serde(default)]
    pub initial_delay: Option<String>,
    #[serde(default)]
    pub max_delay: Option<String>,
    #[serde(default)]
    pub multiplier: Option<f32>,
    #[serde(default)]
    pub jitter: Option<bool>,
}

/// Escalation configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EscalationConfig {
    #[serde(default)]
    pub escalate_to_model_after: Option<u32>,
    #[serde(default)]
    pub notify_on_failure: Option<bool>,
}

/// Error type handling.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ErrorTypeHandling {
    #[serde(default)]
    pub action: Option<String>,
}

/// Sub-workflow execution configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct SubWorkflowExecutionConfig {
    #[serde(default)]
    pub reference_resolution: Option<ReferenceResolutionConfig>,
    #[serde(default)]
    pub inherit_policy: Option<InheritPolicyConfig>,
}

/// Reference resolution configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ReferenceResolutionConfig {
    #[serde(default = "default_sub_workflow_resolution")]
    pub strategy: super::schema::SubWorkflowResolution,
    #[serde(default)]
    pub cache_ttl_secs: Option<u64>,
    #[serde(default = "default_version_conflict_action")]
    pub on_version_conflict: super::schema::VersionConflictAction,
}

/// Inherit policy configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct InheritPolicyConfig {
    #[serde(default)]
    pub inherit_from_parent: Option<Vec<String>>,
    #[serde(default)]
    pub override_allowed: Option<Vec<String>>,
}

/// Checkpointing configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CheckpointingConfig {
    #[serde(default)]
    pub configuration: Option<CheckpointConfiguration>,
    #[serde(default)]
    pub triggers: Option<CheckpointTriggers>,
    #[serde(default)]
    pub cleanup: Option<CheckpointCleanup>,
}

/// Checkpoint configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CheckpointConfiguration {
    #[serde(default = "default_checkpoint_level")]
    pub level: CheckpointLevel,
    #[serde(default)]
    pub output_path: Option<String>,
    #[serde(default)]
    pub max_size: Option<CheckpointMaxSize>,
}

/// Checkpoint max size.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CheckpointMaxSize {
    #[serde(default)]
    pub lines: Option<u32>,
    #[serde(default)]
    pub bytes: Option<String>,
    #[serde(default)]
    pub tokens: Option<u32>,
}

/// Checkpoint triggers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CheckpointTriggers {
    #[serde(default)]
    pub interval: Option<CheckpointInterval>,
    #[serde(default)]
    pub on_failure: Option<String>,
    #[serde(default)]
    pub on_timeout: Option<String>,
}

/// Checkpoint interval.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CheckpointInterval {
    #[serde(default)]
    pub time: Option<String>,
}

/// Checkpoint cleanup.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CheckpointCleanup {
    #[serde(default)]
    pub max_checkpoints: Option<u32>,
    #[serde(default)]
    pub keep_last_n: Option<u32>,
    #[serde(default)]
    pub delete_older_than_hours: Option<u32>,
}

/// Synchronization configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct SynchronizationConfig {
    #[serde(default)]
    pub coordination: Option<CoordinationConfig>,
    #[serde(default)]
    pub conflict_handling: Option<ConflictHandlingConfig>,
    #[serde(default)]
    pub event_propagation: Option<EventPropagationConfig>,
}

/// Coordination configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CoordinationConfig {
    #[serde(default)]
    pub deadlock_detection: Option<bool>,
    #[serde(default)]
    pub deadlock_resolution_timeout_secs: Option<u64>,
}

/// Conflict handling configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ConflictHandlingConfig {
    #[serde(default = "default_conflict_resolution")]
    pub resolution_strategy: ConflictResolution,
}

/// Event propagation configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EventPropagationConfig {
    #[serde(default = "default_event_propagation")]
    pub mode: EventPropagation,
}

/// Dependency resolution configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct DependencyResolutionConfig {
    #[serde(default = "default_dependency_strategy")]
    pub strategy: DependencyStrategy,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    #[serde(default)]
    pub propagate_failure_to_dependents: Option<bool>,
    #[serde(default = "default_dependency_failure_action")]
    pub on_dependency_failure: DependencyFailureAction,
}

/// Step retry configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct StepRetryConfig {
    #[serde(default)]
    pub max_attempts: Option<u32>,
    #[serde(default)]
    pub backoff: Option<BackoffStrategy>,
    #[serde(default)]
    pub initial_delay: Option<String>,
    #[serde(default)]
    pub max_delay: Option<String>,
    #[serde(default)]
    pub multiplier: Option<f32>,
    #[serde(default)]
    pub jitter: Option<bool>,
    #[serde(default)]
    pub level: Option<super::schema::RetryLevel>,
    #[serde(default)]
    pub adjustment_strategy: Option<super::schema::RetryAdjustment>,
    #[serde(default)]
    pub tolerance_adjustment: Option<f32>,
    #[serde(default)]
    pub checkpoint_after_retry: Option<bool>,
}

// ============================================================================
// Default functions
// ============================================================================

fn default_ram_strategy() -> RamAllocationStrategy {
    RamAllocationStrategy::Dynamic
}

fn default_cache_size() -> CacheSize {
    CacheSize::Min
}

fn default_load_unload_strategy() -> LoadUnloadStrategy {
    LoadUnloadStrategy::OneAtATime
}

fn default_pressure_strategy() -> PressureStrategy {
    PressureStrategy::Throttle
}

fn default_timeout_strategy() -> TimeoutStrategy {
    TimeoutStrategy::Fail
}

fn default_error_action() -> ErrorAction {
    ErrorAction::Retry
}

fn default_backoff_strategy() -> BackoffStrategy {
    BackoffStrategy::Exponential
}

fn default_sub_workflow_resolution() -> super::schema::SubWorkflowResolution {
    super::schema::SubWorkflowResolution::LocalFirst
}

fn default_version_conflict_action() -> super::schema::VersionConflictAction {
    super::schema::VersionConflictAction::Error
}

fn default_checkpoint_level() -> CheckpointLevel {
    CheckpointLevel::Info
}

fn default_conflict_resolution() -> ConflictResolution {
    ConflictResolution::LastWriterWins
}

fn default_event_propagation() -> EventPropagation {
    EventPropagation::Bidirectional
}

fn default_dependency_strategy() -> DependencyStrategy {
    DependencyStrategy::WaitForAll
}

fn default_dependency_failure_action() -> DependencyFailureAction {
    DependencyFailureAction::FailWorkflow
}
