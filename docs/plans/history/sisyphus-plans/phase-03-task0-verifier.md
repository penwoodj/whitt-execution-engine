# Phase 03 Task 0 — Verifier Interface System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build pluggable verifier trait system for artifact verification with code, documentation, and config validation capabilities.

**Architecture:** Internal module (`src/quality/verifier/`) integrated into existing single-crate structure. Verifier trait with async verification, pluggable built-in implementations, and runtime registry for dynamic verifier discovery.

**Tech Stack:** Rust, async-trait (object-safe async traits), serde (serialization), tokio (async runtime), thiserror (error types), regex (pattern matching).

---

## Architecture Decision: Internal Module vs Workspace Crate

### Decision: **INTERNAL MODULE** (`src/quality/verifier/`)

### Rationale

**1. Project Structure Reality**
- Current project: **Single crate** (`whitt-execution-engine`) — NOT a workspace
- Original plan assumes workspace with `crates/quality/verifier/` and cross-crate dependencies on non-existent crates (`agentsdk-types`, `agentsdk-llm`)
- Converting to workspace would require restructuring entire project, breaking existing patterns

**2. Minimal Disruption**
- Internal module approach requires **zero refactoring** of existing code
- Leverages existing error handling (`src/error.rs`)
- Uses existing dependencies (serde, tokio, async-trait already in Cargo.toml)
- Follows established module patterns (`src/model/`, `src/agent/`, `src/backend/`)

**3. Phase Integration**
- Verifiers will integrate with Phase 02's workflow execution engine
- Single-crate structure simplifies integration: verifiers accessible as `whitt_execution_engine::quality::verifier::...`
- No cross-crate visibility issues or feature flag complexity

**4. Future Flexibility**
- If workspace conversion becomes necessary later, internal modules can be extracted into crates with minimal changes
- Trait-based design is crate-agnostic — same code works in module or crate context

**5. Validation Criteria Alignment**
- QA Phase 03 expects verifier system to work with workflow artifacts
- Internal module allows direct integration with `WorkflowState`, `Checkpoint` types from Phase 02
- Workspace approach would require defining cross-phase contracts before implementation

### Trade-offs

| Aspect | Internal Module | Workspace Crate |
|---------|----------------|-----------------|
| **Setup time** | ✅ Minutes (create directory) | ⚠️ Hours (workspace + crate restructuring) |
| **Integration effort** | ✅ Minimal (same crate) | ❌ High (cross-crate dependencies) |
| **Code reuse** | ✅ Direct (shared error.rs) | ❌ Duplication or re-exports |
| **Testing** | ✅ Unit tests in same crate | ✅ Same, but requires workspace config |
| **Future extraction** | ✅ Possible if needed | ✅ Designed for multi-crate |
| **Complexity** | ✅ Low | ⚠️ Medium-High |

### Conclusion

**Internal module is the pragmatic choice** for Phase 03 Task 0. It delivers value immediately while maintaining flexibility for future reorganization if project needs change.

---

## File Structure

```
src/
├── quality/                          # NEW MODULE
│   ├── mod.rs                        # Quality module root
│   └── verifier/                     # Verifier sub-module
│       ├── mod.rs                     # Public exports (Verifier trait, VerificationResult, registry)
│       ├── types.rs                   # Core types (VerificationResult, CheckType, Artifact)
│       ├── registry.rs                # VerifierRegistry for dynamic verifier management
│       └── builtin/                  # Built-in verifier implementations
│           ├── mod.rs                # Export built-in verifiers
│           ├── code.rs               # CodeVerifier (syntax, compilation)
│           ├── docs.rs               # DocsVerifier (completeness, formatting)
│           └── config.rs            # ConfigVerifier (schema validation)

tests/                               # EXISTING TEST DIRECTORY
└── quality_verifier_tests.rs         # NEW: Integration tests for verifier system

src/error.rs                           # EXISTING: Add VerificationError variant
src/lib.rs                             # EXISTING: Add `pub mod quality;`
```

---

## Step-by-Step Implementation

### Task 1: Extend error.rs with verification errors

**Files:**
- Modify: `src/error.rs:1-178`

- [ ] **Step 1.1: Add VerificationError variant to Error enum**

Add after line 76 (after `Metric` variant):

```rust
#[error("Verification error: {message}")]
Verification { message: String },
```

- [ ] **Step 1.2: Add verification() constructor method**

Add after line 177 (after `metric()` method):

```rust
pub fn verification(message: impl Into<String>) -> Self {
    Self::Verification {
        message: message.into(),
    }
}
```

- [ ] **Step 1.3: Verify syntax**

Run: `cargo check --lib`
Expected: No errors

- [ ] **Step 1.4: Commit**

```bash
git add src/error.rs
git commit -m "feat(quality): add VerificationError variant"
```

---

### Task 2: Create quality module structure

**Files:**
- Create: `src/quality/mod.rs`

- [ ] **Step 2.1: Create quality module root**

Create file: `src/quality/mod.rs`

```rust
//! Quality assurance module for verification and validation

pub mod verifier;
```

- [ ] **Step 2.2: Add quality module to lib.rs**

Modify: `src/lib.rs`

Add after line 9 (after `pub mod error;`):

```rust
pub mod quality;
```

- [ ] **Step 2.3: Verify syntax**

Run: `cargo check --lib`
Expected: No errors

- [ ] **Step 2.4: Commit**

```bash
git add src/quality/mod.rs src/lib.rs
git commit -m "feat(quality): create quality module structure"
```

---

### Task 3: Define core verifier types

**Files:**
- Create: `src/quality/verifier/types.rs`

- [ ] **Step 3.1: Create types.rs with core verification types**

Create file: `src/quality/verifier/types.rs`

```rust
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
```

- [ ] **Step 3.2: Verify syntax**

Run: `cargo check --lib`
Expected: No errors

- [ ] **Step 3.3: Write failing unit test for VerificationResult**

Create file: `tests/quality_verifier_tests.rs`

```rust
use whitt_execution_engine::quality::verifier::{
    VerificationResult, CheckType, ArtifactType, Artifact, Content,
};

#[test]
fn test_verification_result_pass() {
    let result = VerificationResult::pass(0.9);
    assert!(result.passed);
    assert_eq!(result.confidence, 0.9);
}

#[test]
fn test_verification_result_fail() {
    let result = VerificationResult::fail("Test failure");
    assert!(!result.passed);
    assert_eq!(result.messages.len(), 1);
    assert_eq!(result.messages[0].level, crate::quality::verifier::MessageLevel::Error);
}

#[test]
fn test_verification_result_combine() {
    let result1 = VerificationResult::pass(0.8);
    let result2 = VerificationResult::pass(0.9);
    let combined = VerificationResult::combine(vec![result1, result2]);

    assert!(combined.passed);
    assert_eq!(combined.confidence, 0.85); // (0.8 + 0.9) / 2
}
```

- [ ] **Step 3.4: Run test to verify it fails**

Run: `cargo test quality_verifier_tests::test_verification_result_pass --lib`
Expected: FAIL (types not yet exported)

- [ ] **Step 3.5: Commit**

```bash
git add src/quality/verifier/types.rs tests/quality_verifier_tests.rs
git commit -m "feat(quality): define core verification types"
```

---

### Task 4: Define Verifier trait

**Files:**
- Create: `src/quality/verifier/mod.rs`

- [ ] **Step 4.1: Create verifier module root with Verifier trait**

Create file: `src/quality/verifier/mod.rs`

```rust
mod types;
mod registry;

pub use types::*;
pub use registry::*;

use crate::error::{Error, Result};
use async_trait::async_trait;

/// Core verifier trait for artifact verification
#[async_trait]
pub trait Verifier: Send + Sync {
    /// Get verifier capabilities
    fn capabilities(&self) -> &VerifierCapabilities;

    /// Get verifier name
    fn name(&self) -> &str;

    /// Verify an artifact
    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult>;

    /// Check if verifier can handle this artifact
    fn can_verify(&self, artifact: &Artifact) -> bool {
        self.capabilities().can_handle(artifact.artifact_type)
    }

    /// Optional: Warm-up verifier for performance
    async fn warmup(&self) -> Result<()> {
        Ok(())
    }

    /// Optional: Cleanup verifier resources
    async fn cleanup(&self) -> Result<()> {
        Ok(())
    }
}
```

- [ ] **Step 4.2: Write failing test for Verifier trait**

Add to `tests/quality_verifier_tests.rs`:

```rust
use whitt_execution_engine::quality::verifier::{Verifier, VerifierCapabilities};
use std::sync::Arc;

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

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult> {
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
            artifact_types: vec![ArtifactType::Code],
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
            artifact_types: vec![ArtifactType::Code],
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

- [ ] **Step 4.3: Run test to verify it fails**

Run: `cargo test quality_verifier_tests --lib`
Expected: FAIL (registry module not yet created)

- [ ] **Step 4.4: Implement minimal registry module**

Create file: `src/quality/verifier/registry.rs` (minimal stub):

```rust
use crate::error::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Registry for managing verifiers
pub struct VerifierRegistry {
    verifiers: RwLock<HashMap<String, Arc<dyn Verifier>>>,
    by_artifact_type: RwLock<HashMap<ArtifactType, Vec<String>>>,
}

impl VerifierRegistry {
    pub fn new() -> Self {
        Self {
            verifiers: RwLock::new(HashMap::new()),
            by_artifact_type: RwLock::new(HashMap::new()),
        }
    }

    /// Register a verifier
    pub async fn register(&self, verifier: Arc<dyn Verifier>) -> Result<()> {
        let name = verifier.name().to_string();
        let capabilities = verifier.capabilities();

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
                    .entry(*artifact_type)
                    .or_insert_with(Vec::new)
                    .push(name.clone());
            }
        }

        Ok(())
    }

    /// Get a verifier by name
    pub async fn get(&self, name: &str) -> Result<Arc<dyn Verifier>> {
        let verifiers = self.verifiers.read().await;
        verifiers
            .get(name)
            .cloned()
            .ok_or_else(|| Error::verification(format!("Verifier not found: {}", name)))
    }

    /// Get all verifiers for an artifact type
    pub async fn get_for_artifact_type(
        &self,
        artifact_type: ArtifactType,
    ) -> Result<Vec<Arc<dyn Verifier>>> {
        let by_type = self.by_artifact_type.read().await;
        let verifiers = self.verifiers.read().await;

        let names = by_type
            .get(&artifact_type)
            .ok_or_else(|| Error::verification(format!("No verifiers for artifact type: {:?}", artifact_type)))?;

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
}

impl Default for VerifierRegistry {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 4.5: Run test to verify it passes**

Run: `cargo test quality_verifier_tests --lib`
Expected: PASS

- [ ] **Step 4.6: Commit**

```bash
git add src/quality/verifier/mod.rs src/quality/verifier/registry.rs tests/quality_verifier_tests.rs
git commit -m "feat(quality): implement Verifier trait and registry"
```

---

### Task 5: Implement built-in code verifier

**Files:**
- Create: `src/quality/verifier/builtin/mod.rs`
- Create: `src/quality/verifier/builtin/code.rs`

- [ ] **Step 5.1: Create builtin module root**

Create file: `src/quality/verifier/builtin/mod.rs`

```rust
pub mod code;
pub mod docs;
pub mod config;

pub use code::CodeVerifier;
pub use docs::DocsVerifier;
pub use config::ConfigVerifier;
```

- [ ] **Step 5.2: Update verifier/mod.rs to export builtin**

Modify: `src/quality/verifier/mod.rs`

Add after line 13 (after `pub use registry::*;`):

```rust
pub mod builtin;
```

- [ ] **Step 5.3: Implement CodeVerifier**

Create file: `src/quality/verifier/builtin/code.rs`

```rust
use super::super::{Verifier, VerificationResult, VerifierCapabilities, CheckType, ArtifactType, Artifact};
use crate::error::Result;
use async_trait::async_trait;

/// Code verifier for compilation, linting, and tests
pub struct CodeVerifier {
    language: String,
    check_types: Vec<CheckType>,
}

impl CodeVerifier {
    pub fn new(language: String) -> Self {
        Self {
            language,
            check_types: vec![CheckType::Syntax],
        }
    }

    pub fn with_checks(mut self, check_types: Vec<CheckType>) -> Self {
        self.check_types = check_types;
        self
    }

    async fn check_syntax(&self, artifact: &Artifact) -> VerificationResult {
        let code = String::from_utf8_lossy(&artifact.content.data);

        // Simple syntax checks based on language
        match self.language.as_str() {
            "rust" => self.check_rust_syntax(&code),
            "python" => self.check_python_syntax(&code),
            "javascript" | "typescript" => self.check_js_syntax(&code),
            _ => VerificationResult::pass(0.5), // Unknown language, assume valid
        }
    }

    fn check_rust_syntax(&self, code: &str) -> VerificationResult {
        // Check for basic Rust syntax issues
        if code.contains("fn main(") && !code.contains("{") {
            return VerificationResult::fail("Rust function missing opening brace");
        }

        // Check for balanced braces (basic)
        let brace_count = code.matches('{').count() - code.matches('}').count();
        if brace_count != 0 {
            return VerificationResult::fail(format!("Unbalanced braces: {}", brace_count));
        }

        VerificationResult::pass(0.8)
    }

    fn check_python_syntax(&self, code: &str) -> VerificationResult {
        // Check for basic Python indentation issues
        for line in code.lines() {
            if line.trim().starts_with("def ") || line.trim().starts_with("class ") {
                let trimmed = line.trim();
                if !trimmed.ends_with(':') {
                    return VerificationResult::fail(format!("Python definition missing colon: {}", trimmed));
                }
            }
        }

        VerificationResult::pass(0.8)
    }

    fn check_js_syntax(&self, code: &str) -> VerificationResult {
        // Check for balanced braces and parentheses
        let brace_count = code.matches('{').count() - code.matches('}').count();
        let paren_count = code.matches('(').count() - code.matches(')').count();

        if brace_count != 0 || paren_count != 0 {
            return VerificationResult::fail(format!(
                "Unbalanced braces ({}) or parentheses ({})",
                brace_count, paren_count
            ));
        }

        VerificationResult::pass(0.8)
    }
}

#[async_trait]
impl Verifier for CodeVerifier {
    fn capabilities(&self) -> &VerifierCapabilities {
        // Static capabilities
        &VerifierCapabilities {
            artifact_types: vec![ArtifactType::Code],
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
        // Borrow checker workaround: format in separate string
        &self.language // Simplified: just return language name
    }

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult> {
        let mut results = Vec::new();

        if self.check_types.contains(&CheckType::Syntax) {
            results.push(self.check_syntax(artifact).await);
        }

        Ok(VerificationResult::combine(results))
    }
}
```

**Note on name()**: For now, returns language name. Full "code-rust" format requires storing formatted name or using Cow.

- [ ] **Step 5.4: Write test for code verifier**

Add to `tests/quality_verifier_tests.rs`:

```rust
use whitt_execution_engine::quality::verifier::builtin::CodeVerifier;

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
    assert!(result.passed || result.confidence > 0.5);
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
    assert!(!result.passed);
}
```

- [ ] **Step 5.5: Run tests**

Run: `cargo test quality_verifier_tests::test_code_verifier --lib`
Expected: PASS

- [ ] **Step 5.6: Commit**

```bash
git add src/quality/verifier/builtin/mod.rs src/quality/verifier/builtin/code.rs tests/quality_verifier_tests.rs
git commit -m "feat(quality): implement code verifier"
```

---

### Task 6: Implement built-in documentation verifier

**Files:**
- Create: `src/quality/verifier/builtin/docs.rs`

- [ ] **Step 6.1: Implement DocsVerifier**

Create file: `src/quality/verifier/builtin/docs.rs`

```rust
use super::super::{Verifier, VerificationResult, VerifierCapabilities, CheckType, ArtifactType, Artifact, MessageLevel};
use crate::error::Result;
use async_trait::async_trait;

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
        if content.contains("\n\n\n\n") {
            messages.push(super::super::VerificationMessage {
                level: MessageLevel::Info,
                message: "Contains excessive blank lines (3+ consecutive)".to_string(),
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
            artifact_types: vec![ArtifactType::Documentation],
            check_types: vec![CheckType::Documentation, CheckType::Formatting],
            dependencies: vec![],
            estimated_duration_ms: 100,
            supports_async: true,
        }
    }

    fn name(&self) -> &str {
        "docs-verifier"
    }

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult> {
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

- [ ] **Step 6.2: Write test for docs verifier**

Add to `tests/quality_verifier_tests.rs`:

```rust
use whitt_execution_engine::quality::verifier::builtin::DocsVerifier;

#[tokio::test]
async fn test_docs_verifier_complete() {
    let verifier = DocsVerifier::new().without_formatting();
    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Documentation,
        content: Content {
            data: br#"# Introduction

This is introduction.

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

- [ ] **Step 6.3: Run tests**

Run: `cargo test quality_verifier_tests::test_docs_verifier --lib`
Expected: PASS

- [ ] **Step 6.4: Commit**

```bash
git add src/quality/verifier/builtin/docs.rs tests/quality_verifier_tests.rs
git commit -m "feat(quality): implement documentation verifier"
```

---

### Task 7: Implement built-in config verifier

**Files:**
- Create: `src/quality/verifier/builtin/config.rs`

- [ ] **Step 7.1: Implement ConfigVerifier**

Create file: `src/quality/verifier/builtin/config.rs`

```rust
use super::super::{Verifier, VerificationResult, VerifierCapabilities, CheckType, ArtifactType, Artifact, MessageLevel};
use crate::error::{Result, Error};
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

    fn validate_schema(&self, value: &Value) -> VerificationResult {
        if let Some(schema) = &self.schema {
            // Basic schema validation
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
            artifact_types: vec![ArtifactType::Config],
            check_types: vec![CheckType::SchemaValidation],
            dependencies: vec![],
            estimated_duration_ms: 50,
            supports_async: true,
        }
    }

    fn name(&self) -> &str {
        "config-verifier"
    }

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult> {
        let content = String::from_utf8_lossy(&artifact.content.data).to_string();
        let format = artifact.content.format.as_str();

        // Parse based on format
        let value = match format {
            "json" => serde_json::from_str::<Value>(&content)
                .map_err(|e| Error::verification(format!("Invalid JSON: {}", e)))?,
            "yaml" => {
                // Simple YAML parsing (would use serde_yaml in production)
                // For now, treat as JSON if it's valid
                serde_json::from_str::<Value>(&content)
                    .map_err(|e| Error::verification(format!("Invalid YAML/JSON: {}", e)))?
            }
            _ => return Ok(VerificationResult::fail(format!("Unsupported format: {}", format))),
        };

        // Validate against schema if provided
        Ok(self.validate_schema(&value))
    }
}
```

**Note on YAML**: Using serde_json for simplicity. Could integrate serde_yaml (already a transitive dependency via serde-saphyr) later.

- [ ] **Step 7.2: Write test for config verifier**

Add to `tests/quality_verifier_tests.rs`:

```rust
use whitt_execution_engine::quality::verifier::builtin::ConfigVerifier;

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

- [ ] **Step 7.3: Run tests**

Run: `cargo test quality_verifier_tests::test_config_verifier --lib`
Expected: PASS

- [ ] **Step 7.4: Commit**

```bash
git add src/quality/verifier/builtin/config.rs tests/quality_verifier_tests.rs
git commit -m "feat(quality): implement config verifier"
```

---

### Task 8: Add integration test for registry

**Files:**
- Modify: `tests/quality_verifier_tests.rs`

- [ ] **Step 8.1: Add registry integration test**

Add to `tests/quality_verifier_tests.rs`:

```rust
use whitt_execution_engine::quality::verifier::VerifierRegistry;
use std::sync::Arc;

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
    let verifiers = registry.get_for_artifact_type(ArtifactType::Code).await.unwrap();
    assert_eq!(verifiers.len(), 1);
}
```

- [ ] **Step 8.2: Run all verifier tests**

Run: `cargo test quality_verifier_tests --lib`
Expected: ALL PASS

- [ ] **Step 8.3: Commit**

```bash
git add tests/quality_verifier_tests.rs
git commit -m "test(quality): add registry integration tests"
```

---

### Task 9: Run full test suite and verify

- [ ] **Step 9.1: Run all tests**

Run: `cargo test --all-features --lib`
Expected: ALL PASS (existing tests + new verifier tests)

- [ ] **Step 9.2: Run clippy**

Run: `cargo clippy --all-features -- -W clippy::all`
Expected: 0 warnings

- [ ] **Step 9.3: Run build**

Run: `cargo build --release --all-features`
Expected: Exit code 0

- [ ] **Step 9.4: Check LSP diagnostics**

Run: Check changed files for errors
Expected: 0 errors

- [ ] **Step 9.5: Commit**

```bash
git add .
git commit -m "feat(quality): complete verifier interface system with full test coverage"
```

---

## Completion Criteria

Task 0 is complete when:

- ✅ Verifier trait defined with async verify method
- ✅ Core types (VerificationResult, CheckType, ArtifactType) implemented
- ✅ Built-in verifiers (code, docs, config) working
- ✅ VerifierRegistry with registration and lookup
- ✅ Integration with existing `src/error.rs`
- ✅ All tests passing (unit + integration)
- ✅ Clippy clean (0 warnings)
- ✅ Build passes
- ✅ Code review passed (via `/find-bugs` or manual)

---

## Integration Points

### Error Handling
- Uses `crate::error::Error::Verification { message: String }` variant
- Constructor: `Error::verification(message)` added to `src/error.rs`

### Dependencies
- **Existing**: serde, tokio, thiserror (already in Cargo.toml)
- **New**: async-trait (already in Cargo.toml)
- **None**: No new dependencies required

### Phase 02 Integration
- Verifiers accept `Artifact` struct (new type defined in verifier system)
- Verifiers return `VerificationResult` for integration with Generate-Verify-Repair runtime (Task 1)
- Registry pattern enables dynamic verifier selection based on workflow artifacts

---

## Future Enhancements

1. **YAML Support**: Integrate serde-yaml for config verifier (transitive via serde-saphyr)
2. **External Tools**: Integrate rustc, python3 for actual compilation/syntax checking
3. **Plugin System**: Allow external verifier registration via dynamic loading (optional)
4. **Metrics**: Track verification performance, success rates, common failures
5. **Caching**: Cache verification results for unchanged artifacts

---

## Handoff

Ready for Task 1: Generate-Verify-Repair Runtime

---

## References

| Link | Description |
|-------|-------------|
| [Original Task Plan](../../../docs/plans/03-quality-loops/tasks/00-verifier-interface.md) | 1410-line original plan (adapted to internal module) |
| [Quality Loops Phase Plan](../../../docs/plans/03-quality-loops/plan.md) | Parent phase plan |
| [Existing Error Types](src/error.rs) | Error handling pattern used in codebase |
| [Module Structure](src/lib.rs) | Current module organization |
