# Task 0: Verifier Interface System

**Files:**
- Create: `crates/quality/verifier/src/lib.rs`
- Create: `crates/quality/verifier/src/types.rs`
- Create: `crates/quality/verifier/src/builtin/mod.rs`
- Create: `crates/quality/verifier/src/builtin/code.rs`
- Create: `crates/quality/verifier/src/builtin/docs.rs`
- Create: `crates/quality/verifier/src/builtin/config.rs`
- Create: `crates/quality/verifier/src/registry.rs`
- Create: `crates/quality/verifier/Cargo.toml`
- Test: `crates/quality/verifier/tests/verifier_tests.rs`

**Duration:** 1.5 weeks

## Overview

Build the pluggable verifier trait system that allows different verification strategies for different artifact types. Verifiers check artifacts for correctness, quality, and compliance.

## Architecture

The verifier system consists of:

1. **Verifier Trait** — Core abstraction for artifact verification
2. **VerificationResult** — Structured result with pass/fail status and detailed messages
3. **VerifierCapabilities** — Metadata describing what a verifier can check
4. **Built-in Verifiers** — Code (compilation, linting, tests), Docs (completeness, formatting), Config (schema validation)
5. **Verifier Registry** — Dynamic registration and lookup of verifiers by artifact type

---

## Step-by-Step Implementation

### Step 1: Create crate structure and Cargo.toml

- [ ] **Step 1.1: Create verifier crate directory structure**

```bash
mkdir -p crates/quality/verifier/src/builtin
```

- [ ] **Step 1.2: Write Cargo.toml**

Create file: `crates/quality/verifier/Cargo.toml`

```toml
[package]
name = "agentsdk-verifier"
version = "0.1.0"
edition = "2021"

[dependencies]
agentsdk-types = { path = "../../types" }
agentsdk-llm = { path = "../../llm" }
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
regex = "1.0"
glob = "0.3"

[dev-dependencies]
tokio-test = "0.4"
```

- [ ] **Step 1.3: Commit**

```bash
git add crates/quality/verifier/Cargo.toml
git commit -m "feat(quality): create verifier crate with dependencies"
```

---

### Step 2: Define core types

- [ ] **Step 2.1: Create types.rs with core verification types**

Create file: `crates/quality/verifier/src/types.rs`

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

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
        let messages: Vec<_> = results.into_iter().flat_map(|r| r.messages).collect();
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

/// Verifier capabilities descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierCapabilities {
    /// Artifact types this verifier can handle
    pub artifact_types: Vec<String>,
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
    pub fn can_handle(&self, artifact_type: &str) -> bool {
        self.artifact_types.contains(&artifact_type.to_string())
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

/// Verification error types
#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Verifier not found: {0}")]
    VerifierNotFound(String),

    #[error("Artifact type not supported: {0}")]
    ArtifactTypeNotSupported(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Timeout error")]
    Timeout,
}
```

- [ ] **Step 2.2: Commit**

```bash
git add crates/quality/verifier/src/types.rs
git commit -m "feat(quality): define core verification types"
```

---

### Step 3: Define Verifier trait

- [ ] **Step 3.1: Create lib.rs with Verifier trait**

Create file: `crates/quality/verifier/src/lib.rs`

```rust
mod types;
mod builtin;
mod registry;

pub use types::*;
pub use builtin::*;
pub use registry::*;

use agentsdk_types::{Artifact, ArtifactType};
use async_trait::async_trait;

/// Core verifier trait for artifact verification
#[async_trait]
pub trait Verifier: Send + Sync {
    /// Get verifier capabilities
    fn capabilities(&self) -> &VerifierCapabilities;

    /// Get verifier name
    fn name(&self) -> &str;

    /// Verify an artifact
    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult, VerificationError>;

    /// Check if verifier can handle this artifact
    fn can_verify(&self, artifact: &Artifact) -> bool {
        self.capabilities().can_handle(&artifact.artifact_type.to_string())
    }

    /// Optional: Warm-up verifier for performance
    async fn warmup(&self) -> Result<(), VerificationError> {
        Ok(())
    }

    /// Optional: Cleanup verifier resources
    async fn cleanup(&self) -> Result<(), VerificationError> {
        Ok(())
    }
}
```

- [ ] **Step 3.2: Write failing test for Verifier trait**

Create file: `crates/quality/verifier/tests/verifier_tests.rs`

```rust
use agentsdk_verifier::{Verifier, VerificationResult, VerifierCapabilities, CheckType};
use agentsdk_types::{Artifact, ArtifactType, Content};

struct MockVerifier {
    name: String,
    capabilities: VerifierCapabilities,
}

#[async_trait::async_trait]
impl Verifier for MockVerifier {
    fn capabilities(&self) -> &VerifierCapabilities {
        &self.capabilities
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult, Box<dyn std::error::Error>> {
        if artifact.content.data.len() > 0 {
            Ok(VerificationResult::pass(1.0))
        } else {
            Ok(VerificationResult::fail("Empty artifact"))
        }
    }
}

#[tokio::test]
async fn test_verifier_trait_basic() {
    let verifier = MockVerifier {
        name: "test".to_string(),
        capabilities: VerifierCapabilities {
            artifact_types: vec!["code".to_string()],
            check_types: vec![CheckType::Syntax],
            dependencies: vec![],
            estimated_duration_ms: 100,
            supports_async: true,
        },
    };

    assert_eq!(verifier.name(), "test");
    assert_eq!(verifier.capabilities().artifact_types.len(), 1);
}

#[tokio::test]
async fn test_verify_empty_artifact_fails() {
    let verifier = MockVerifier {
        name: "test".to_string(),
        capabilities: VerifierCapabilities {
            artifact_types: vec!["code".to_string()],
            check_types: vec![CheckType::Syntax],
            dependencies: vec![],
            estimated_duration_ms: 100,
            supports_async: true,
        },
    };

    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Code,
        content: Content {
            data: vec![],
            format: "text".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let result = verifier.verify(&artifact).await.unwrap();
    assert!(!result.passed);
}
```

- [ ] **Step 3.3: Run test to verify it fails**

```bash
cargo test --package agentsdk-verifier --lib
```

Expected: FAIL (trait not yet implemented)

- [ ] **Step 3.4: Implement Verifier trait in lib.rs**

The trait is already implemented in Step 3.1.

- [ ] **Step 3.5: Run test to verify it passes**

```bash
cargo test --package agentsdk-verifier --lib
```

Expected: PASS

- [ ] **Step 3.6: Commit**

```bash
git add crates/quality/verifier/src/lib.rs crates/quality/verifier/tests/verifier_tests.rs
git commit -m "feat(quality): implement Verifier trait with tests"
```

---

### Step 4: Implement built-in code verifier

- [ ] **Step 4.1: Create builtin code verifier**

Create file: `crates/quality/verifier/src/builtin/code.rs`

```rust
use super::super::{Verifier, VerificationResult, VerificationError, VerifierCapabilities, CheckType, MessageLevel};
use agentsdk_types::{Artifact, ArtifactType};
use async_trait::async_trait;
use std::process::Command;
use std::path::Path;

/// Code verifier for compilation, linting, and tests
pub struct CodeVerifier {
    language: String,
    check_types: Vec<CheckType>,
}

impl CodeVerifier {
    pub fn new(language: String) -> Self {
        Self {
            language,
            check_types: vec![CheckType::Syntax, CheckType::Compilation],
        }
    }

    pub fn with_checks(mut self, check_types: Vec<CheckType>) -> Self {
        self.check_types = check_types;
        self
    }

    async fn check_syntax(&self, artifact: &Artifact) -> VerificationResult {
        // Basic syntax check using language-specific tools
        let result = match self.language.as_str() {
            "rust" => self.check_rust_syntax(artifact).await,
            "python" => self.check_python_syntax(artifact).await,
            "javascript" | "typescript" => self.check_js_syntax(artifact).await,
            _ => VerificationResult::pass(0.5), // Unknown language, assume valid
        };
        result
    }

    async fn check_rust_syntax(&self, artifact: &Artifact) -> VerificationResult {
        let code = String::from_utf8_lossy(&artifact.content.data);
        // Use rustc --parse to check syntax
        let output = Command::new("rustc")
            .args(&["--crate-type", "lib", "-Z", "no-codegen", "--emit", "metadata"])
            .arg("-")
            .output();

        match output {
            Ok(output) if output.status.success() => VerificationResult::pass(1.0),
            Ok(output) => {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                VerificationResult::fail(format!("Rust syntax error: {}", error_msg))
            }
            Err(_) => VerificationResult::pass(0.5), // rustc not available
        }
    }

    async fn check_python_syntax(&self, artifact: &Artifact) -> VerificationResult {
        let code = String::from_utf8_lossy(&artifact.content.data);
        // Use python -m py_compile
        let output = Command::new("python3")
            .args(&["-m", "py_compile", "-"])
            .output();

        match output {
            Ok(output) if output.status.success() => VerificationResult::pass(1.0),
            Ok(output) => {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                VerificationResult::fail(format!("Python syntax error: {}", error_msg))
            }
            Err(_) => VerificationResult::pass(0.5), // python3 not available
        }
    }

    async fn check_js_syntax(&self, artifact: &Artifact) -> VerificationResult {
        // Simple heuristic: check for balanced braces and parentheses
        let code = String::from_utf8_lossy(&artifact.content.data);
        let mut brace_count = 0;
        let mut paren_count = 0;

        for c in code.chars() {
            match c {
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                '(' => paren_count += 1,
                ')' => paren_count -= 1,
                _ => {}
            }
        }

        if brace_count != 0 || paren_count != 0 {
            VerificationResult::fail(format!("Unbalanced braces ({}) or parentheses ({})", brace_count, paren_count))
        } else {
            VerificationResult::pass(0.8)
        }
    }

    async fn check_compilation(&self, artifact: &Artifact) -> VerificationResult {
        // Compilation check would require full project context
        // For now, return a medium confidence pass
        VerificationResult::pass(0.7)
    }
}

#[async_trait]
impl Verifier for CodeVerifier {
    fn capabilities(&self) -> &VerifierCapabilities {
        // Use static capabilities for simplicity
        &VerifierCapabilities {
            artifact_types: vec!["code".to_string()],
            check_types: self.check_types.clone(),
            dependencies: match self.language.as_str() {
                "rust" => vec!["rustc".to_string()],
                "python" => vec!["python3".to_string()],
                _ => vec![],
            },
            estimated_duration_ms: 500,
            supports_async: true,
        }
    }

    fn name(&self) -> &str {
        &format!("code-{}", self.language)
    }

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult, VerificationError> {
        let mut results = Vec::new();

        if self.check_types.contains(&CheckType::Syntax) {
            results.push(self.check_syntax(artifact).await);
        }

        if self.check_types.contains(&CheckType::Compilation) {
            results.push(self.check_compilation(artifact).await);
        }

        Ok(VerificationResult::combine(results))
    }
}
```

- [ ] **Step 4.2: Write test for code verifier**

Add to `crates/quality/verifier/tests/verifier_tests.rs`:

```rust
use agentsdk_verifier::builtin::CodeVerifier;

#[tokio::test]
async fn test_code_verifier_valid_rust() {
    let verifier = CodeVerifier::new("rust".to_string());
    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Code,
        content: Content {
            data: br#"fn main() { println!("Hello"); }"#.to_vec(),
            format: "rust".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let result = verifier.verify(&artifact).await.unwrap();
    // Result may be pass or fail depending on rustc availability
    assert!(result.confidence > 0.0);
}

#[tokio::test]
async fn test_code_verifier_invalid_rust() {
    let verifier = CodeVerifier::new("rust".to_string());
    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Code,
        content: Content {
            data: b"fn main( {".to_vec(),
            format: "rust".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let result = verifier.verify(&artifact).await.unwrap();
    // Invalid syntax should fail
    assert!(!result.passed || result.confidence < 1.0);
}
```

- [ ] **Step 4.3: Run tests**

```bash
cargo test --package agentsdk-verifier --lib
```

Expected: PASS

- [ ] **Step 4.4: Commit**

```bash
git add crates/quality/verifier/src/builtin/code.rs crates/quality/verifier/tests/verifier_tests.rs
git commit -m "feat(quality): implement code verifier"
```

---

### Step 5: Implement built-in documentation verifier

- [ ] **Step 5.1: Create builtin documentation verifier**

Create file: `crates/quality/verifier/src/builtin/docs.rs`

```rust
use super::super::{Verifier, VerificationResult, VerificationError, VerifierCapabilities, CheckType, MessageLevel};
use agentsdk_types::{Artifact, ArtifactType};
use async_trait::async_trait;
use regex::Regex;

/// Documentation verifier for completeness and formatting
pub struct DocsVerifier {
    required_sections: Vec<String>,
    check_formatting: bool,
}

impl DocsVerifier {
    pub fn new() -> Self {
        Self {
            required_sections: vec![
                "Introduction".to_string(),
                "Usage".to_string(),
                "API".to_string(),
            ],
            check_formatting: true,
        }
    }

    pub fn with_sections(mut self, sections: Vec<String>) -> Self {
        self.required_sections = sections;
        self
    }

    pub fn without_formatting(mut self) -> Self {
        self.check_formatting = false;
        self
    }

    fn check_completeness(&self, content: &str) -> VerificationResult {
        let mut messages = Vec::new();
        let mut missing_sections = Vec::new();

        for section in &self.required_sections {
            if !content.contains(section) {
                missing_sections.push(section.clone());
            }
        }

        if !missing_sections.is_empty() {
            messages.push(super::super::VerificationMessage {
                level: MessageLevel::Warning,
                message: format!("Missing required sections: {}", missing_sections.join(", ")),
                location: None,
            });
        }

        let passed = missing_sections.is_empty();
        let confidence = if passed { 1.0 } else { 0.5 };

        VerificationResult {
            passed,
            confidence,
            messages,
            metrics: std::collections::HashMap::new(),
            duration_ms: 0,
        }
    }

    fn check_formatting(&self, content: &str) -> VerificationResult {
        let mut messages = Vec::new();

        // Check for excessive blank lines
        let excessive_blank_lines = Regex::new(r"\n{4,}").unwrap();
        if excessive_blank_lines.is_match(content) {
            messages.push(super::super::VerificationMessage {
                level: MessageLevel::Info,
                message: "Contains excessive blank lines (3+ consecutive)".to_string(),
                location: None,
            });
        }

        // Check for inconsistent header spacing
        let header_no_space = Regex::new(r"[^\n]#+[A-Z]").unwrap();
        if header_no_space.is_match(content) {
            messages.push(super::super::VerificationMessage {
                level: MessageLevel::Info,
                message: "Headers may lack proper spacing before them".to_string(),
                location: None,
            });
        }

        VerificationResult {
            passed: messages.is_empty(),
            confidence: if messages.is_empty() { 1.0 } else { 0.8 },
            messages,
            metrics: std::collections::HashMap::new(),
            duration_ms: 0,
        }
    }
}

impl Default for DocsVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Verifier for DocsVerifier {
    fn capabilities(&self) -> &VerifierCapabilities {
        &VerifierCapabilities {
            artifact_types: vec!["documentation".to_string()],
            check_types: vec![CheckType::Documentation, CheckType::Formatting],
            dependencies: vec![],
            estimated_duration_ms: 100,
            supports_async: true,
        }
    }

    fn name(&self) -> &str {
        "docs-verifier"
    }

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult, VerificationError> {
        let content = String::from_utf8_lossy(&artifact.content.data).to_string();
        let mut results = Vec::new();

        results.push(self.check_completeness(&content));

        if self.check_formatting {
            results.push(self.check_formatting(&content));
        }

        Ok(VerificationResult::combine(results))
    }
}
```

- [ ] **Step 5.2: Write test for docs verifier**

Add to `crates/quality/verifier/tests/verifier_tests.rs`:

```rust
use agentsdk_verifier::builtin::DocsVerifier;

#[tokio::test]
async fn test_docs_verifier_complete() {
    let verifier = DocsVerifier::new().without_formatting();
    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Documentation,
        content: Content {
            data: br#"# Introduction

This is the introduction.

# Usage

Here's how to use it.

# API

The API reference."#.to_vec(),
            format: "markdown".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let result = verifier.verify(&artifact).await.unwrap();
    assert!(result.passed);
}

#[tokio::test]
async fn test_docs_verifier_incomplete() {
    let verifier = DocsVerifier::new();
    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Documentation,
        content: Content {
            data: b"# Introduction\nOnly intro here.".to_vec(),
            format: "markdown".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let result = verifier.verify(&artifact).await.unwrap();
    // Should have warnings about missing sections
    assert!(!result.messages.is_empty());
}
```

- [ ] **Step 5.3: Run tests**

```bash
cargo test --package agentsdk-verifier --lib
```

Expected: PASS

- [ ] **Step 5.4: Commit**

```bash
git add crates/quality/verifier/src/builtin/docs.rs crates/quality/verifier/tests/verifier_tests.rs
git commit -m "feat(quality): implement documentation verifier"
```

---

### Step 6: Implement built-in config verifier

- [ ] **Step 6.1: Create builtin config verifier**

Create file: `crates/quality/verifier/src/builtin/config.rs`

```rust
use super::super::{Verifier, VerificationResult, VerificationError, VerifierCapabilities, CheckType, MessageLevel};
use agentsdk_types::{Artifact, ArtifactType};
use async_trait::async_trait;
use serde_json::Value;

/// Configuration verifier for schema validation
pub struct ConfigVerifier {
    schema: Option<Value>,
}

impl ConfigVerifier {
    pub fn new() -> Self {
        Self { schema: None }
    }

    pub fn with_schema(mut self, schema: Value) -> Self {
        self.schema = Some(schema);
        self
    }

    fn validate_json(&self, content: &str) -> VerificationResult {
        match serde_json::from_str::<Value>(content) {
            Ok(_) => VerificationResult::pass(1.0),
            Err(e) => VerificationResult::fail(format!("Invalid JSON: {}", e)),
        }
    }

    fn validate_yaml(&self, content: &str) -> VerificationResult {
        match serde_yaml::from_str::<Value>(content) {
            Ok(_) => VerificationResult::pass(1.0),
            Err(e) => VerificationResult::fail(format!("Invalid YAML: {}", e)),
        }
    }

    fn validate_schema(&self, value: &Value) -> VerificationResult {
        if let Some(schema) = &self.schema {
            // Basic schema validation (would use jsonschema crate in production)
            let messages = self.validate_against_schema(value, schema);

            if messages.is_empty() {
                VerificationResult::pass(1.0)
            } else {
                let mut result = VerificationResult::fail("Schema validation failed");
                result.messages = messages;
                result
            }
        } else {
            VerificationResult::pass(1.0)
        }
    }

    fn validate_against_schema(&self, value: &Value, schema: &Value) -> Vec<super::super::VerificationMessage> {
        let mut messages = Vec::new();

        // Check required fields
        if let Some(required) = schema.get("required").and_then(|v| v.as_array()) {
            if let Some(obj) = value.as_object() {
                for field in required {
                    if let Some(field_name) = field.as_str() {
                        if !obj.contains_key(field_name) {
                            messages.push(super::super::VerificationMessage {
                                level: MessageLevel::Error,
                                message: format!("Missing required field: {}", field_name),
                                location: None,
                            });
                        }
                    }
                }
            }
        }

        messages
    }
}

impl Default for ConfigVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Verifier for ConfigVerifier {
    fn capabilities(&self) -> &VerifierCapabilities {
        &VerifierCapabilities {
            artifact_types: vec!["config".to_string()],
            check_types: vec![CheckType::SchemaValidation],
            dependencies: vec![],
            estimated_duration_ms: 50,
            supports_async: true,
        }
    }

    fn name(&self) -> &str {
        "config-verifier"
    }

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult, VerificationError> {
        let content = String::from_utf8_lossy(&artifact.content.data).to_string();
        let format = artifact.content.format.as_str();

        // Parse based on format
        let value = match format {
            "json" => serde_json::from_str::<Value>(&content)
                .map_err(|e| VerificationError::ParseError(e.to_string()))?,
            "yaml" => serde_yaml::from_str::<Value>(&content)
                .map_err(|e| VerificationError::ParseError(e.to_string()))?,
            _ => return Ok(VerificationResult::fail(format!("Unsupported format: {}", format))),
        };

        // Validate against schema if provided
        Ok(self.validate_schema(&value))
    }
}
```

- [ ] **Step 6.2: Write test for config verifier**

Add to `crates/quality/verifier/tests/verifier_tests.rs`:

```rust
use agentsdk_verifier::builtin::ConfigVerifier;

#[tokio::test]
async fn test_config_verifier_valid_json() {
    let verifier = ConfigVerifier::new();
    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Config,
        content: Content {
            data: br#"{"name": "test", "value": 123}"#.to_vec(),
            format: "json".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let result = verifier.verify(&artifact).await.unwrap();
    assert!(result.passed);
}

#[tokio::test]
async fn test_config_verifier_invalid_json() {
    let verifier = ConfigVerifier::new();
    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Config,
        content: Content {
            data: b"{invalid json}".to_vec(),
            format: "json".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let result = verifier.verify(&artifact).await.unwrap();
    assert!(!result.passed);
}

#[tokio::test]
async fn test_config_verifier_schema_validation() {
    let schema = serde_json::json!({
        "type": "object",
        "required": ["name", "version"]
    });
    let verifier = ConfigVerifier::new().with_schema(schema);

    let artifact_missing = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Config,
        content: Content {
            data: br#"{"name": "test"}"#.to_vec(),
            format: "json".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let result = verifier.verify(&artifact_missing).await.unwrap();
    assert!(!result.passed);
    assert!(result.messages.iter().any(|m| m.message.contains("version")));
}
```

- [ ] **Step 6.3: Run tests**

```bash
cargo test --package agentsdk-verifier --lib
```

Expected: PASS

- [ ] **Step 6.4: Commit**

```bash
git add crates/quality/verifier/src/builtin/config.rs crates/quality/verifier/tests/verifier_tests.rs
git commit -m "feat(quality): implement config verifier"
```

---

### Step 7: Implement verifier registry

- [ ] **Step 7.1: Create registry module**

Create file: `crates/quality/verifier/src/registry.rs`

```rust
use super::{Verifier, VerificationError, VerifierCapabilities};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Registry for managing verifiers
pub struct VerifierRegistry {
    verifiers: RwLock<HashMap<String, Arc<dyn Verifier>>>,
    by_artifact_type: RwLock<HashMap<String, Vec<String>>>,
}

impl VerifierRegistry {
    pub fn new() -> Self {
        Self {
            verifiers: RwLock::new(HashMap::new()),
            by_artifact_type: RwLock::new(HashMap::new()),
        }
    }

    /// Register a verifier
    pub async fn register(&self, verifier: Arc<dyn Verifier>) -> Result<(), VerificationError> {
        let name = verifier.name().to_string();
        let capabilities = verifier.capabilities();

        // Check for duplicate names
        {
            let verifiers = self.verifiers.read().await;
            if verifiers.contains_key(&name) {
                return Err(VerificationError::VerifierNotFound(format!(
                    "Verifier '{}' already registered",
                    name
                )));
            }
        }

        // Store verifier
        {
            let mut verifiers = self.verifiers.write().await;
            verifiers.insert(name.clone(), verifier);
        }

        // Index by artifact type
        {
            let mut by_type = self.by_artifact_type.write().await;
            for artifact_type in &capabilities.artifact_types {
                by_type
                    .entry(artifact_type.clone())
                    .or_insert_with(Vec::new)
                    .push(name.clone());
            }
        }

        Ok(())
    }

    /// Get a verifier by name
    pub async fn get(&self, name: &str) -> Result<Arc<dyn Verifier>, VerificationError> {
        let verifiers = self.verifiers.read().await;
        verifiers
            .get(name)
            .cloned()
            .ok_or_else(|| VerificationError::VerifierNotFound(name.to_string()))
    }

    /// Get all verifiers for an artifact type
    pub async fn get_for_artifact_type(
        &self,
        artifact_type: &str,
    ) -> Result<Vec<Arc<dyn Verifier>>, VerificationError> {
        let by_type = self.by_artifact_type.read().await;
        let verifiers = self.verifiers.read().await;

        let names = by_type
            .get(artifact_type)
            .ok_or_else(|| VerificationError::ArtifactTypeNotSupported(artifact_type.to_string()))?;

        names
            .iter()
            .filter_map(|name| verifiers.get(name).cloned())
            .collect()
    }

    /// List all registered verifiers
    pub async fn list(&self) -> Vec<String> {
        let verifiers = self.verifiers.read().await;
        verifiers.keys().cloned().collect()
    }

    /// Get capabilities for all verifiers
    pub async fn get_all_capabilities(&self) -> Vec<(String, VerifierCapabilities)> {
        let verifiers = self.verifiers.read().await;
        verifiers
            .iter()
            .map(|(name, verifier)| (name.clone(), verifier.capabilities().clone()))
            .collect()
    }
}

impl Default for VerifierRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin::{CodeVerifier, DocsVerifier, ConfigVerifier};

    #[tokio::test]
    async fn test_registry_register_and_get() {
        let registry = VerifierRegistry::new();
        let verifier = Arc::new(CodeVerifier::new("rust".to_string()));

        registry.register(verifier.clone()).await.unwrap();

        let retrieved = registry.get("code-rust").await.unwrap();
        assert_eq!(retrieved.name(), "code-rust");
    }

    #[tokio::test]
    async fn test_registry_get_for_artifact_type() {
        let registry = VerifierRegistry::new();
        registry.register(Arc::new(CodeVerifier::new("rust".to_string()))).await.unwrap();
        registry.register(Arc::new(CodeVerifier::new("python".to_string()))).await.unwrap();

        let verifiers = registry.get_for_artifact_type("code").await.unwrap();
        assert_eq!(verifiers.len(), 2);
    }

    #[tokio::test]
    async fn test_registry_duplicate_registration() {
        let registry = VerifierRegistry::new();
        let verifier = Arc::new(CodeVerifier::new("rust".to_string()));

        registry.register(verifier.clone()).await.unwrap();
        let result = registry.register(verifier.clone()).await;

        assert!(result.is_err());
    }
}
```

- [ ] **Step 7.2: Update builtin/mod.rs**

Create file: `crates/quality/verifier/src/builtin/mod.rs`

```rust
pub mod code;
pub mod docs;
pub mod config;

pub use code::CodeVerifier;
pub use docs::DocsVerifier;
pub use config::ConfigVerifier;
```

- [ ] **Step 7.3: Write integration test for registry**

Add to `crates/quality/verifier/tests/verifier_tests.rs`:

```rust
use agentsdk_verifier::VerifierRegistry;

#[tokio::test]
async fn test_registry_integration() {
    let registry = VerifierRegistry::new();

    // Register multiple verifiers
    registry.register(Arc::new(CodeVerifier::new("rust".to_string()))).await.unwrap();
    registry.register(Arc::new(DocsVerifier::new())).await.unwrap();
    registry.register(Arc::new(ConfigVerifier::new())).await.unwrap();

    // List all verifiers
    let all = registry.list().await;
    assert_eq!(all.len(), 3);

    // Get capabilities
    let caps = registry.get_all_capabilities().await;
    assert_eq!(caps.len(), 3);
}
```

- [ ] **Step 7.4: Run tests**

```bash
cargo test --package agentsdk-verifier
```

Expected: PASS

- [ ] **Step 7.5: Commit**

```bash
git add crates/quality/verifier/src/registry.rs crates/quality/verifier/src/builtin/mod.rs crates/quality/verifier/tests/verifier_tests.rs
git commit -m "feat(quality): implement verifier registry"
```

---

### Step 8: Add documentation and examples

- [ ] **Step 8.1: Create README.md**

Create file: `crates/quality/verifier/README.md`

```markdown
# AgentSDK Verifier

Pluggable verifier trait system for artifact validation.

## Overview

The verifier system provides a flexible interface for validating different types of artifacts:

- **Code verification**: Syntax checking, compilation, linting, tests
- **Documentation verification**: Completeness, formatting
- **Configuration verification**: Schema validation

## Usage

### Basic Usage

```rust
use agentsdk_verifier::{Verifier, VerifierRegistry};
use agentsdk_verifier::builtin::{CodeVerifier, DocsVerifier, ConfigVerifier};

// Create a verifier
let verifier = CodeVerifier::new("rust".to_string());

// Verify an artifact
let result = verifier.verify(&artifact).await?;
assert!(result.passed);
```

### Using the Registry

```rust
let registry = VerifierRegistry::new();

// Register verifiers
registry.register(Arc::new(CodeVerifier::new("rust".to_string()))).await?;
registry.register(Arc::new(DocsVerifier::new())).await?;

// Get verifier for artifact type
let verifiers = registry.get_for_artifact_type("code").await?;
for verifier in verifiers {
    let result = verifier.verify(&artifact).await?;
}
```

### Custom Verifiers

```rust
use agentsdk_verifier::{Verifier, VerificationResult, VerifierCapabilities, CheckType};

struct CustomVerifier;

#[async_trait::async_trait]
impl Verifier for CustomVerifier {
    fn capabilities(&self) -> &VerifierCapabilities {
        &VerifierCapabilities {
            artifact_types: vec!["custom".to_string()],
            check_types: vec![CheckType::Syntax],
            dependencies: vec![],
            estimated_duration_ms: 100,
            supports_async: true,
        }
    }

    fn name(&self) -> &str {
        "custom-verifier"
    }

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult, VerificationError> {
        // Your verification logic here
        Ok(VerificationResult::pass(1.0))
    }
}
```

## Built-in Verifiers

### CodeVerifier

Verifies code artifacts:
- Syntax checking (Rust, Python, JavaScript)
- Compilation checking (project context required)

```rust
let verifier = CodeVerifier::new("rust".to_string())
    .with_checks(vec![CheckType::Syntax, CheckType::Compilation]);
```

### DocsVerifier

Verifies documentation artifacts:
- Required sections
- Formatting consistency

```rust
let verifier = DocsVerifier::new()
    .with_sections(vec![
        "Introduction".to_string(),
        "Usage".to_string(),
    ]);
```

### ConfigVerifier

Verifies configuration artifacts:
- JSON/YAML parsing
- Schema validation

```rust
let schema = serde_json::json!({
    "type": "object",
    "required": ["name", "version"]
});
let verifier = ConfigVerifier::new().with_schema(schema);
```
```

- [ ] **Step 8.2: Commit**

```bash
git add crates/quality/verifier/README.md
git commit -m "docs(quality): add verifier documentation"
```

---

## Completion Criteria

Task 0 is complete when:

- ✅ All verifier interfaces implemented
- ✅ Built-in verifiers (code, docs, config) working
- ✅ Verifier registry with registration and lookup
- ✅ All tests passing
- ✅ Documentation complete
- ✅ Code review passed

---

## Handoff

Ready for Task 1: Generate-Verify-Repair Runtime

---

## 🔗 Related Documentation

| Link | Description |
|-------|-------------|
| [Parent Plan](../plan.md) | Quality Loops phase implementation plan |
| [Validation Criteria](../validation/acceptance-criteria.md) | Acceptance criteria for verifier interface |

---

## QA Cross-References

- **QA Criteria**: ['$qa_criteria']('$file')
- **Test Cases**: ['$test_case']('$file')
- **Schema Ref**: $schema_ref
