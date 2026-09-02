//! Unified workflow schema v2.0.0
//!
//! Provides strongly-typed deserialization for the unified workflow schema.
//!
//! # Schema Source
//!
//! The source of truth is `docs/schema/unified-workflow-schema.yml`.
//!
//! # Usage
//!
//! ```ignore
//! use whitt_execution_engine::workflow::WorkflowFile;
//!
//! let yaml = std::fs::read_to_string("workflow.yml")?;
//! let workflow = WorkflowFile::from_yaml(&yaml)?;
//! ```

mod execution;
pub mod hooks;
mod permissions;
mod schema;
mod step;
mod tests;

pub use schema::{
    CacheSize, CheckpointLevel, ComparisonOperator, ConflictResolution,
    DependencyFailureAction, DependencyStrategy, EventPropagation, FormatValidationFormat,
    GuardrailEnforcementPolicy, GuardrailOnMatch, GuardrailSensitivity, KvCacheQuantization,
    LogLevel, PermissionMode, ProvidersConfig, RamAllocationStrategy, RetryAdjustment,
    RetryLevel, SubWorkflowRef, SubWorkflowResolution, ToolPermissionPolicy,
    UserInputType, VersionConflictAction, WorkflowExecutionStrategy, WorkflowFile,
};

pub use step::{
    AgenticWorkflow, ExecutionMode, HookAction, LoopConfig, RequireCondition, UserInputConfig,
    UserInputPrompt, WorkflowInput, WorkflowStep,
    LogAction, GwtClause, AppendToAction, SaveToAction, RouteToAction,
    BookmarkAction, BookmarkActionDetail, NotifyAction, FailAction, ShellAction, ModelOverrides,
};

pub use execution::{
    CheckpointingConfig, DependencyResolutionConfig, ErrorHandlingConfig,
    ExecutionMemoryConfig, SubWorkflowExecutionConfig, TimeoutConfig,
    SynchronizationConfig, RetryConfig, StepRetryConfig,
    QualityConfig, TimingConfig, StreamingConfig, ResourceAdmissionConfig,
};

pub use permissions::{
    ContentOperations, FileOperations, ShellOperations, SystemOperations,
    ToolPermissions, WebOperations,
};
