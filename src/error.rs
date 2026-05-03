use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("YAML parsing error: {0}")]
    YamlParse(#[from] serde_saphyr::Error),

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Template rendering error: {0}")]
    Template(#[from] askama::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("Model error: {0}")]
    Model(String),

    #[error("Model not found: {model_name}")]
    ModelNotFound { model_name: String },

    #[error("Model load failed: {model_name}: {reason}")]
    ModelLoadFailed { model_name: String, reason: String },

    #[error("Invalid model format: {format}")]
    InvalidModelFormat { format: String },

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Agent definition error: {message}")]
    AgentDefinition { message: String },

    #[error("Tool definition error: {message}")]
    ToolDefinition { message: String },

    #[error("Workflow definition error: {message}")]
    WorkflowDefinition { message: String },

    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Invalid value for field '{field}': {reason}")]
    InvalidValue { field: String, reason: String },

    #[error("CLI error: {0}")]
    Cli(String),

    #[error("Execution error: {message}")]
    Execution { message: String },

    #[error("Benchmark error: {message}")]
    Benchmark { message: String },

    #[error("Metrics collection failed: {reason}")]
    MetricsCollection { reason: String },

    #[error("Invalid output path: {path}")]
    InvalidOutputPath { path: PathBuf },

    #[error("Memory error: {message}")]
    Memory { message: String },

    #[error("Schedule error: {message}")]
    Schedule { message: String },

    #[error("Metric error: {message}")]
    Metric { message: String },

    #[error("Verification error: {message}")]
    Verification { message: String },
}

impl Error {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    pub fn model(message: impl Into<String>) -> Self {
        Self::Model(message.into())
    }

    pub fn model_not_found(model_name: impl Into<String>) -> Self {
        Self::ModelNotFound {
            model_name: model_name.into(),
        }
    }

    pub fn model_load_failed(model_name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ModelLoadFailed {
            model_name: model_name.into(),
            reason: reason.into(),
        }
    }

    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    pub fn agent_definition(message: impl Into<String>) -> Self {
        Self::AgentDefinition {
            message: message.into(),
        }
    }

    pub fn tool_definition(message: impl Into<String>) -> Self {
        Self::ToolDefinition {
            message: message.into(),
        }
    }

    pub fn workflow_definition(message: impl Into<String>) -> Self {
        Self::WorkflowDefinition {
            message: message.into(),
        }
    }

    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    pub fn invalid_value(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            field: field.into(),
            reason: reason.into(),
        }
    }

    pub fn cli(message: impl Into<String>) -> Self {
        Self::Cli(message.into())
    }

    pub fn execution(message: impl Into<String>) -> Self {
        Self::Execution {
            message: message.into(),
        }
    }

    pub fn benchmark(message: impl Into<String>) -> Self {
        Self::Benchmark {
            message: message.into(),
        }
    }

    pub fn metrics_collection(reason: impl Into<String>) -> Self {
        Self::MetricsCollection {
            reason: reason.into(),
        }
    }

    pub fn memory(message: impl Into<String>) -> Self {
        Self::Memory {
            message: message.into(),
        }
    }

    pub fn schedule(message: impl Into<String>) -> Self {
        Self::Schedule {
            message: message.into(),
        }
    }

    pub fn metric(message: impl Into<String>) -> Self {
        Self::Metric {
            message: message.into(),
        }
    }

    pub fn verification(message: impl Into<String>) -> Self {
        Self::Verification {
            message: message.into(),
        }
    }
}
