use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Verification result with detailed information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationResult {
    /// Overall pass/fail status
    pub passed: bool,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Detailed messages explaining the result
    pub messages: Vec<VerificationMessage>,
    /// Metrics collected during verification
    pub metrics: HashMap<String, serde_json::Value>,
    /// Time taken to verify (in milliseconds)
    pub duration_ms: u64,
}

impl VerificationResult {
    /// Create a passing result
    pub fn pass(confidence: f64) -> Self {
        Self {
            passed: true,
            confidence,
            messages: vec![],
            metrics: HashMap::new(),
            duration_ms: 0,
        }
    }

    /// Create a failing result
    pub fn fail(message: impl Into<String>) -> Self {
        Self {
            passed: false,
            confidence: 1.0,
            messages: vec![VerificationMessage {
                level: MessageLevel::Error,
                message: message.into(),
                location: None,
            }],
            metrics: HashMap::new(),
            duration_ms: 0,
        }
    }

    /// Add a message to the result
    pub fn with_message(mut self, level: MessageLevel, message: impl Into<String>) -> Self {
        self.messages.push(VerificationMessage {
            level,
            message: message.into(),
            location: None,
        });
        self
    }

    /// Add a metric to the result
    pub fn with_metric(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metrics.insert(key.into(), value);
        self
    }

    /// Combine multiple results (AND logic)
    pub fn combine(results: Vec<Self>) -> Self {
        let passed = results.iter().all(|r| r.passed);
        let confidence = results.iter().map(|r| r.confidence).sum::<f64>() / results.len() as f64;
        let messages: Vec<_> = results.iter().flat_map(|r| r.messages.clone()).collect();
        let metrics: HashMap<_, _> = results
            .iter()
            .flat_map(|r| r.metrics.clone())
            .collect();

        Self {
            passed,
            confidence,
            messages,
            metrics,
            duration_ms: 0,
        }
    }
}

/// Individual verification message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationMessage {
    pub level: MessageLevel,
    pub message: String,
    pub location: Option<SourceLocation>,
}

/// Message severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageLevel {
    Info,
    Warning,
    Error,
    Critical,
}

/// Source location for verification messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceLocation {
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

/// Verifier capabilities descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierCapabilities {
    /// Artifact types this verifier can handle
    pub artifact_types: Vec<ArtifactType>,
    /// Check types this verifier performs
    pub check_types: Vec<CheckType>,
    /// Required tool dependencies
    pub dependencies: Vec<String>,
    /// Estimated verification time (in milliseconds)
    pub estimated_duration_ms: u64,
    /// Whether this verifier is async-capable
    pub supports_async: bool,
}

impl VerifierCapabilities {
    /// Check if verifier can handle a specific artifact type
    pub fn can_handle(&self, artifact_type: ArtifactType) -> bool {
        self.artifact_types.contains(&artifact_type)
    }

    /// Check if verifier can perform a specific check
    pub fn can_check(&self, check_type: CheckType) -> bool {
        self.check_types.contains(&check_type)
    }
}

/// Types of verification checks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheckType {
    Syntax,
    Compilation,
    Linting,
    Tests,
    Documentation,
    SchemaValidation,
    Formatting,
    Style,
    Security,
    Performance,
}

/// Artifact type for verification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtifactType {
    Code,
    Documentation,
    Config,
}

/// Artifact to be verified
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub artifact_type: ArtifactType,
    pub content: Content,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Artifact content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub data: Vec<u8>,
    pub format: String, // e.g., "rust", "python", "markdown", "json", "yaml"
}
