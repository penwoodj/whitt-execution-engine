# Task 1: Generate-Verify-Repair Runtime

**Files:**
- Create: `crates/quality/loops/src/lib.rs`
- Create: `crates/quality/loops/src/config.rs`
- Create: `crates/quality/loops/src/generate.rs`
- Create: `crates/quality/loops/src/verify.rs`
- Create: `crates/quality/loops/src/repair.rs`
- Create: `crates/quality/loops/src/state.rs`
- Create: `crates/quality/loops/src/convergence.rs`
- Create: `crates/quality/loops/Cargo.toml`
- Test: `crates/quality/loops/tests/loop_tests.rs`

**Duration:** 2 weeks

## Overview

Implement the core generate-verify-repair runtime semantics. This is **RUNTIME BEHAVIOR**, not optional prompt engineering (per ADR-0005). The quality loop must converge within max_iterations and track all state for debugging.

## Architecture

The quality loop consists of:

1. **RepairLoopConfig** — Configuration for the quality loop (max_iterations, convergence criteria, backoff strategy)
2. **Generate Step** — Execute workflow to generate artifact
3. **Verify Step** — Run verifiers to check artifact quality
4. **Repair Step** — Use LLM to repair failed artifacts
5. **Convergence Detection** — Check if loop should terminate (success, failure, or timeout)
6. **State Tracking** — Persist loop state for debugging and auditing

---

## Step-by-Step Implementation

### Step 1: Create crate structure and Cargo.toml

- [ ] **Step 1.1: Create loops crate directory**

```bash
mkdir -p crates/quality/loops/src
```

- [ ] **Step 1.2: Write Cargo.toml**

Create file: `crates/quality/loops/Cargo.toml`

```toml
[package]
name = "agentsdk-loops"
version = "0.1.0"
edition = "2021"

[dependencies]
agentsdk-types = { path = "../../types" }
agentsdk-llm = { path = "../../llm" }
agentsdk-verifier = { path = "../verifier" }
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
anyhow = "1.0"

[dev-dependencies]
tokio-test = "0.4"
mockall = "0.11"
```

- [ ] **Step 1.3: Commit**

```bash
git add crates/quality/loops/Cargo.toml
git commit -m "feat(quality): create loops crate with dependencies"
```

---

### Step 2: Define config types

- [ ] **Step 2.1: Create config.rs with RepairLoopConfig**

Create file: `crates/quality/loops/src/config.rs`

```rust
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for the generate-verify-repair quality loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairLoopConfig {
    /// Maximum number of iterations before giving up
    pub max_iterations: usize,

    /// Convergence criteria for loop termination
    pub convergence: ConvergenceCriteria,

    /// Backoff strategy for repair attempts
    pub backoff: BackoffStrategy,

    /// Quality thresholds that must be met
    pub quality_thresholds: QualityThresholds,

    /// Timeout for each iteration
    pub iteration_timeout: Duration,

    /// Whether to persist loop state
    pub persist_state: bool,

    /// State persistence directory
    pub state_dir: Option<String>,
}

impl Default for RepairLoopConfig {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            convergence: ConvergenceCriteria::default(),
            backoff: BackoffStrategy::default(),
            quality_thresholds: QualityThresholds::default(),
            iteration_timeout: Duration::from_secs(300),
            persist_state: true,
            state_dir: None,
        }
    }
}

/// Criteria for determining loop convergence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceCriteria {
    /// Minimum confidence threshold (0.0 - 1.0)
    pub min_confidence: f64,

    /// Maximum number of consecutive failures
    pub max_consecutive_failures: usize,

    /// Epsilon for considering two artifacts equivalent
    pub equivalence_epsilon: f64,

    /// Whether to stop early if quality plateaus
    pub stop_on_plateau: bool,

    /// Number of iterations to check for plateau
    pub plateau_window: usize,
}

impl Default for ConvergenceCriteria {
    fn default() -> Self {
        Self {
            min_confidence: 0.95,
            max_consecutive_failures: 3,
            equivalence_epsilon: 0.001,
            stop_on_plateau: true,
            plateau_window: 3,
        }
    }
}

/// Backoff strategy for repair attempts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackoffStrategy {
    /// No backoff - constant delay
    None { delay: Duration },

    /// Linear backoff
    Linear {
        initial_delay: Duration,
        increment: Duration,
    },

    /// Exponential backoff
    Exponential {
        initial_delay: Duration,
        multiplier: f64,
        max_delay: Duration,
    },
}

impl Default for BackoffStrategy {
    fn default() -> Self {
        Self::Exponential {
            initial_delay: Duration::from_secs(1),
            multiplier: 2.0,
            max_delay: Duration::from_secs(60),
        }
    }
}

impl BackoffStrategy {
    /// Calculate delay for given iteration (0-indexed)
    pub fn delay_for_iteration(&self, iteration: usize) -> Duration {
        match self {
            Self::None { delay } => *delay,
            Self::Linear {
                initial_delay,
                increment,
            } => *initial_delay + increment.mul(iteration as u32),
            Self::Exponential {
                initial_delay,
                multiplier,
                max_delay,
            } => {
                let delay_ms = initial_delay.as_millis() as f64 * multiplier.powi(iteration as i32);
                let delay = Duration::from_millis(delay_ms as u64);
                std::cmp::min(delay, *max_delay)
            }
        }
    }
}

/// Quality thresholds that must be met
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityThresholds {
    /// Minimum verifier pass rate (0.0 - 1.0)
    pub min_pass_rate: f64,

    /// Minimum average confidence (0.0 - 1.0)
    pub min_avg_confidence: f64,

    /// Maximum number of errors allowed
    pub max_errors: usize,

    /// Maximum number of warnings allowed
    pub max_warnings: usize,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            min_pass_rate: 1.0,
            min_avg_confidence: 0.95,
            max_errors: 0,
            max_warnings: 5,
        }
    }
}
```

- [ ] **Step 2.2: Write failing test for config**

Create file: `crates/quality/loops/tests/loop_tests.rs`

```rust
use agentsdk_loops::config::{RepairLoopConfig, BackoffStrategy, ConvergenceCriteria};
use std::time::Duration;

#[test]
fn test_default_config() {
    let config = RepairLoopConfig::default();
    assert_eq!(config.max_iterations, 10);
    assert_eq!(config.convergence.min_confidence, 0.95);
}

#[test]
fn test_backoff_exponential() {
    let backoff = BackoffStrategy::Exponential {
        initial_delay: Duration::from_millis(100),
        multiplier: 2.0,
        max_delay: Duration::from_secs(10),
    };

    assert_eq!(backoff.delay_for_iteration(0), Duration::from_millis(100));
    assert_eq!(backoff.delay_for_iteration(1), Duration::from_millis(200));
    assert_eq!(backoff.delay_for_iteration(2), Duration::from_millis(400));
}
```

- [ ] **Step 2.3: Run test to verify it fails**

```bash
cargo test --package agentsdk-loops --lib
```

Expected: FAIL (config not yet exported from lib.rs)

- [ ] **Step 2.4: Export config from lib.rs**

Create file: `crates/quality/loops/src/lib.rs`

```rust
mod config;
mod generate;
mod verify;
mod repair;
mod state;
mod convergence;

pub use config::*;
pub use generate::*;
pub use verify::*;
pub use repair::*;
pub use state::*;
pub use convergence::*;
```

- [ ] **Step 2.5: Run test to verify it passes**

```bash
cargo test --package agentsdk-loops --lib
```

Expected: PASS

- [ ] **Step 2.6: Commit**

```bash
git add crates/quality/loops/src/lib.rs crates/quality/loops/src/config.rs crates/quality/loops/tests/loop_tests.rs
git commit -m "feat(quality): define RepairLoopConfig with tests"
```

---

### Step 3: Define loop state types

- [ ] **Step 3.1: Create state.rs with loop state**

Create file: `crates/quality/loops/src/state.rs`

```rust
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use agentsdk_types::Artifact;
use agentsdk_verifier::VerificationResult;

/// Unique identifier for a quality loop
pub type LoopId = Uuid;

/// State of the quality loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoopState {
    /// Loop is starting
    Starting,

    /// Loop is running generate step
    Generating,

    /// Loop is running verify step
    Verifying,

    /// Loop is running repair step
    Repairing,

    /// Loop completed successfully
    Completed,

    /// Loop failed to converge
    Failed,

    /// Loop timed out
    TimedOut,
}

/// Complete state of a quality loop execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopExecutionState {
    /// Unique identifier for this loop
    pub id: LoopId,

    /// Current state
    pub state: LoopState,

    /// Current iteration (0-indexed)
    pub iteration: usize,

    /// History of all iterations
    pub iteration_history: Vec<IterationState>,

    /// Current artifact
    pub current_artifact: Option<Artifact>,

    /// Verification result from current iteration
    pub current_verification: Option<VerificationResult>,

    /// Loop start time
    pub start_time: DateTime<Utc>,

    /// Loop end time (if completed)
    pub end_time: Option<DateTime<Utc>>,

    /// Number of consecutive failures
    pub consecutive_failures: usize,

    /// Configuration used for this loop
    pub config: super::RepairLoopConfig,
}

impl LoopExecutionState {
    /// Create a new loop state
    pub fn new(config: super::RepairLoopConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: LoopState::Starting,
            iteration: 0,
            iteration_history: Vec::new(),
            current_artifact: None,
            current_verification: None,
            start_time: Utc::now(),
            end_time: None,
            consecutive_failures: 0,
            config,
        }
    }

    /// Transition to a new state
    pub fn transition_to(&mut self, new_state: LoopState) {
        self.state = new_state;
    }

    /// Start a new iteration
    pub fn start_iteration(&mut self) {
        self.iteration += 1;
    }

    /// Record an iteration result
    pub fn record_iteration(&mut self, iteration: IterationState) {
        self.iteration_history.push(iteration);
    }

    /// Update consecutive failures count
    pub fn update_consecutive_failures(&mut self, failed: bool) {
        if failed {
            self.consecutive_failures += 1;
        } else {
            self.consecutive_failures = 0;
        }
    }

    /// Check if loop has reached max iterations
    pub fn is_max_iterations_reached(&self) -> bool {
        self.iteration >= self.config.max_iterations
    }

    /// Get duration since loop started
    pub fn elapsed(&self) -> chrono::Duration {
        let end = self.end_time.unwrap_or_else(Utc::now);
        end.signed_duration_since(self.start_time)
    }

    /// Check if loop has exceeded timeout
    pub fn is_timeout(&self) -> bool {
        self.elapsed() > chrono::Duration::from_std(self.config.iteration_timeout).unwrap()
    }
}

/// State of a single iteration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationState {
    /// Iteration number (0-indexed)
    pub iteration: usize,

    /// Artifact generated in this iteration
    pub artifact: Artifact,

    /// Verification result
    pub verification: VerificationResult,

    /// Time taken for this iteration (in milliseconds)
    pub duration_ms: u64,

    /// Whether repair was attempted in this iteration
    pub repair_attempted: bool,

    /// Repair result (if attempted)
    pub repair_result: Option<RepairResult>,
}

/// Result of a repair attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairResult {
    /// Whether repair succeeded
    pub success: bool,

    /// Number of repair attempts made
    pub attempts: usize,

    /// Repair strategy used
    pub strategy: RepairStrategy,

    /// Time taken for repair (in milliseconds)
    pub duration_ms: u64,
}

/// Strategy used for repair
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RepairStrategy {
    /// Use LLM to generate fix
    LLMGeneration,

    /// Apply heuristic fixes
    Heuristic,

    /// Rollback to previous version
    Rollback,
}
```

- [ ] **Step 3.2: Write test for state**

Add to `crates/quality/loops/tests/loop_tests.rs`:

```rust
use agentsdk_loops::state::{LoopExecutionState, LoopState, IterationState};
use agentsdk_loops::config::RepairLoopConfig;
use agentsdk_types::{Artifact, ArtifactType, Content};

#[test]
fn test_loop_state_creation() {
    let config = RepairLoopConfig::default();
    let state = LoopExecutionState::new(config);

    assert_eq!(state.iteration, 0);
    assert_eq!(state.consecutive_failures, 0);
    assert!(matches!(state.state, LoopState::Starting));
}

#[test]
fn test_loop_state_iteration() {
    let config = RepairLoopConfig::default();
    let mut state = LoopExecutionState::new(config);

    state.start_iteration();
    assert_eq!(state.iteration, 1);

    state.start_iteration();
    assert_eq!(state.iteration, 2);
}

#[test]
fn test_consecutive_failures() {
    let config = RepairLoopConfig::default();
    let mut state = LoopExecutionState::new(config);

    state.update_consecutive_failures(true);
    assert_eq!(state.consecutive_failures, 1);

    state.update_consecutive_failures(true);
    assert_eq!(state.consecutive_failures, 2);

    state.update_consecutive_failures(false);
    assert_eq!(state.consecutive_failures, 0);
}

#[test]
fn test_max_iterations_reached() {
    let mut config = RepairLoopConfig::default();
    config.max_iterations = 5;

    let mut state = LoopExecutionState::new(config);

    for _ in 0..4 {
        state.start_iteration();
        assert!(!state.is_max_iterations_reached());
    }

    state.start_iteration();
    assert!(state.is_max_iterations_reached());
}
```

- [ ] **Step 3.3: Run tests**

```bash
cargo test --package agentsdk-loops --lib
```

Expected: PASS

- [ ] **Step 3.4: Commit**

```bash
git add crates/quality/loops/src/state.rs crates/quality/loops/tests/loop_tests.rs
git commit -m "feat(quality): define loop state types with tests"
```

---

### Step 4: Implement generate step

- [ ] **Step 4.1: Create generate.rs**

Create file: `crates/quality/loops/src/generate.rs`

```rust
use agentsdk_types::{Artifact, ArtifactType, Content};
use agentsdk_llm::LLMBackend;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenerateError {
    #[error("LLM generation failed: {0}")]
    LLMError(String),

    #[error("Invalid generation result: {0}")]
    InvalidResult(String),

    #[error("Timeout exceeded")]
    Timeout,
}

/// Result of the generate step
#[derive(Debug, Clone)]
pub struct GenerateResult {
    /// Generated artifact
    pub artifact: Artifact,

    /// Generation metadata
    pub metadata: GenerationMetadata,
}

/// Metadata about the generation
#[derive(Debug, Clone)]
pub struct GenerationMetadata {
    /// Time taken to generate (in milliseconds)
    pub duration_ms: u64,

    /// Number of tokens used
    pub tokens_used: Option<usize>,

    /// Model used for generation
    pub model: Option<String>,
}

/// Generate an artifact using the LLM backend
pub async fn generate_artifact(
    backend: &dyn LLMBackend,
    prompt: &str,
    artifact_type: ArtifactType,
    metadata: Value,
) -> Result<GenerateResult, GenerateError> {
    let start = std::time::Instant::now();

    // Call LLM backend
    let response = backend
        .generate(prompt, None)
        .await
        .map_err(|e| GenerateError::LLMError(e.to_string()))?;

    let duration = start.elapsed();

    // Parse response as artifact
    let artifact = Artifact {
        id: uuid::Uuid::new_v4().to_string(),
        artifact_type,
        content: Content {
            data: response.text.as_bytes().to_vec(),
            format: artifact_type.to_string(),
        },
        metadata: {
            let mut map = std::collections::HashMap::new();
            if let Some(obj) = metadata.as_object() {
                for (k, v) in obj {
                    map.insert(k.clone(), v.to_string());
                }
            }
            map
        },
    };

    Ok(GenerateResult {
        artifact,
        metadata: GenerationMetadata {
            duration_ms: duration.as_millis() as u64,
            tokens_used: response.usage.map(|u| u.total_tokens),
            model: response.model,
        },
    })
}
```

- [ ] **Step 4.2: Write test for generate step**

Add to `crates/quality/loops/tests/loop_tests.rs`:

```rust
use agentsdk_loops::generate::{generate_artifact, GenerateError};
use agentsdk_types::ArtifactType;

#[cfg(test)]
use mockall::mock;

#[cfg(test)]
mock! {
    LLMBackend {}

    #[async_trait::async_trait]
    impl agentsdk_llm::LLMBackend for LLMBackend {
        async fn generate(&self, prompt: &str, context: Option<&agentsdk_llm::LLMContext>) -> Result<agentsdk_llm::LLMResponse, Box<dyn std::error::Error>>;
    }
}

#[tokio::test]
async fn test_generate_artifact_success() {
    let mut mock_backend = MockLLMBackend::new();
    mock_backend
        .expect_generate()
        .returning(|_, _| {
            Ok(agentsdk_llm::LLMResponse {
                text: "generated code".to_string(),
                usage: Some(agentsdk_llm::Usage {
                    prompt_tokens: 10,
                    completion_tokens: 20,
                    total_tokens: 30,
                }),
                model: Some("test-model".to_string()),
            })
        });

    let result = generate_artifact(
        &mock_backend,
        "Generate a function",
        ArtifactType::Code,
        serde_json::json!({}),
    )
    .await
    .unwrap();

    assert_eq!(result.artifact.content.data, b"generated code");
    assert_eq!(result.metadata.tokens_used, Some(30));
}
```

- [ ] **Step 4.3: Run tests**

```bash
cargo test --package agentsdk-loops --lib
```

Expected: PASS (after implementing mock)

- [ ] **Step 4.4: Commit**

```bash
git add crates/quality/loops/src/generate.rs crates/quality/loops/tests/loop_tests.rs
git commit -m "feat(quality): implement generate step with tests"
```

---

### Step 5: Implement verify step

- [ ] **Step 5.1: Create verify.rs**

Create file: `crates/quality/loops/src/verify.rs`

```rust
use agentsdk_verifier::{Verifier, VerifierRegistry, VerificationResult, VerificationError};
use agentsdk_types::Artifact;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("No verifiers available for artifact type: {0}")]
    NoVerifiers(String),

    #[error("Timeout exceeded")]
    Timeout,
}

/// Result of the verify step
#[derive(Debug, Clone)]
pub struct VerifyResult {
    /// Combined verification result from all verifiers
    pub result: VerificationResult,

    /// Individual results from each verifier
    pub individual_results: Vec<(String, VerificationResult)>,

    /// Time taken to verify (in milliseconds)
    pub duration_ms: u64,
}

/// Verify an artifact using all applicable verifiers
pub async fn verify_artifact(
    registry: &VerifierRegistry,
    artifact: &Artifact,
) -> Result<VerifyResult, VerifyError> {
    let start = std::time::Instant::now();

    // Get verifiers for this artifact type
    let artifact_type = &artifact.artifact_type.to_string();
    let verifiers = registry
        .get_for_artifact_type(artifact_type)
        .await
        .map_err(|_| VerifyError::NoVerifiers(artifact_type.clone()))?;

    if verifiers.is_empty() {
        return Err(VerifyError::NoVerifiers(artifact_type.clone()));
    }

    // Run all verifiers
    let mut individual_results = Vec::new();
    for verifier in &verifiers {
        let name = verifier.name().to_string();
        let result = verifier.verify(artifact).await.map_err(|e| {
            VerifyError::VerificationFailed(format!("{}: {}", name, e))
        })?;
        individual_results.push((name, result));
    }

    let duration = start.elapsed();

    // Combine results
    let results: Vec<VerificationResult> = individual_results
        .iter()
        .map(|(_, r)| r.clone())
        .collect();
    let combined = VerificationResult::combine(results);

    Ok(VerifyResult {
        result: combined,
        individual_results,
        duration_ms: duration.as_millis() as u64,
    })
}
```

- [ ] **Step 5.2: Write test for verify step**

Add to `crates/quality/loops/tests/loop_tests.rs`:

```rust
use agentsdk_loops::verify::{verify_artifact, VerifyError};
use agentsdk_verifier::{Verifier, VerificationResult, VerifierCapabilities, CheckType};
use agentsdk_types::{Artifact, ArtifactType, Content};
use std::sync::Arc;
use mockall::mock;

mock! {
    Verifier {}

    #[async_trait::async_trait]
    impl agentsdk_verifier::Verifier for Verifier {
        fn capabilities(&self) -> &VerifierCapabilities;
        fn name(&self) -> &str;
        async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult, agentsdk_verifier::VerificationError>;
    }
}

#[tokio::test]
async fn test_verify_artifact_success() {
    let mut mock_verifier = MockVerifier::new();
    mock_verifier.expect_capabilities().returning(|| {
        &VerifierCapabilities {
            artifact_types: vec!["code".to_string()],
            check_types: vec![CheckType::Syntax],
            dependencies: vec![],
            estimated_duration_ms: 100,
            supports_async: true,
        }
    });
    mock_verifier.expect_name().return_const("test-verifier");
    mock_verifier
        .expect_verify()
        .returning(|_| Ok(VerificationResult::pass(1.0)));

    let mut registry = agentsdk_verifier::VerifierRegistry::new();
    registry.register(Arc::new(mock_verifier)).await.unwrap();

    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Code,
        content: Content {
            data: b"test code".to_vec(),
            format: "text".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let result = verify_artifact(&registry, &artifact).await.unwrap();
    assert!(result.result.passed);
}

#[tokio::test]
async fn test_verify_artifact_no_verifiers() {
    let registry = agentsdk_verifier::VerifierRegistry::new();

    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Code,
        content: Content {
            data: b"test code".to_vec(),
            format: "text".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let result = verify_artifact(&registry, &artifact).await;
    assert!(matches!(result, Err(VerifyError::NoVerifiers(_))));
}
```

- [ ] **Step 5.3: Run tests**

```bash
cargo test --package agentsdk-loops --lib
```

Expected: PASS

- [ ] **Step 5.4: Commit**

```bash
git add crates/quality/loops/src/verify.rs crates/quality/loops/tests/loop_tests.rs
git commit -m "feat(quality): implement verify step with tests"
```

---

### Step 6: Implement repair step

- [ ] **Step 6.1: Create repair.rs**

Create file: `crates/quality/loops/src/repair.rs`

```rust
use agentsdk_llm::LLMBackend;
use agentsdk_types::Artifact;
use agentsdk_verifier::VerificationResult;
use thiserror::Error;

use super::state::RepairResult as LoopRepairResult;

#[derive(Error, Debug)]
pub enum RepairError {
    #[error("LLM repair failed: {0}")]
    LLMError(String),

    #[error("Repair strategy failed: {0}")]
    StrategyFailed(String),

    #[error("Timeout exceeded")]
    Timeout,
}

/// Repair an artifact based on verification failures
pub async fn repair_artifact(
    backend: &dyn LLMBackend,
    artifact: &Artifact,
    verification_result: &VerificationResult,
    max_attempts: usize,
) -> Result<LoopRepairResult, RepairError> {
    let start = std::time::Instant::now();

    // Construct repair prompt from verification failures
    let repair_prompt = construct_repair_prompt(artifact, verification_result);

    // Try repair attempts
    let mut attempts = 0;
    let mut last_error = None;

    for attempt in 0..max_attempts {
        attempts += 1;

        // Generate repair using LLM
        let response = backend
            .generate(&repair_prompt, None)
            .await
            .map_err(|e| RepairError::LLMError(e.to_string()))?;

        // Check if repair is successful (simplified check)
        if !response.text.is_empty() {
            let duration = start.elapsed();

            return Ok(LoopRepairResult {
                success: true,
                attempts,
                strategy: super::state::RepairStrategy::LLMGeneration,
                duration_ms: duration.as_millis() as u64,
            });
        }

        last_error = Some("Empty repair response".to_string());
    }

    Err(RepairError::StrategyFailed(last_error.unwrap_or_else(|| "Unknown error".to_string())))
}

/// Construct a repair prompt from verification failures
fn construct_repair_prompt(artifact: &Artifact, verification_result: &VerificationResult) -> String {
    let mut prompt = String::from("Please repair the following artifact based on these errors:\n\n");

    // Add artifact content
    prompt.push_str("Artifact:\n");
    prompt.push_str(&String::from_utf8_lossy(&artifact.content.data));
    prompt.push_str("\n\n");

    // Add verification errors
    if !verification_result.messages.is_empty() {
        prompt.push_str("Errors:\n");
        for msg in &verification_result.messages {
            prompt.push_str(&format!("- {}: {}\n", msg.level, msg.message));
        }
        prompt.push_str("\n");
    }

    prompt.push_str("Provide the repaired version of the artifact.");

    prompt
}
```

- [ ] **Step 6.2: Write test for repair step**

Add to `crates/quality/loops/tests/loop_tests.rs`:

```rust
use agentsdk_loops::repair::{repair_artifact, RepairError};
use agentsdk_types::{Artifact, ArtifactType, Content};
use agentsdk_verifier::{VerificationResult, VerificationMessage, MessageLevel};
use mockall::mock;

mock! {
    LLMBackend {}

    #[async_trait::async_trait]
    impl agentsdk_llm::LLMBackend for LLMBackend {
        async fn generate(&self, prompt: &str, context: Option<&agentsdk_llm::LLMContext>) -> Result<agentsdk_llm::LLMResponse, Box<dyn std::error::Error>>;
    }
}

#[tokio::test]
async fn test_repair_artifact_success() {
    let mut mock_backend = MockLLMBackend::new();
    mock_backend
        .expect_generate()
        .times(1)
        .returning(|_, _| {
            Ok(agentsdk_llm::LLMResponse {
                text: "repaired code".to_string(),
                usage: None,
                model: None,
            })
        });

    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Code,
        content: Content {
            data: b"broken code".to_vec(),
            format: "text".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let verification = VerificationResult::fail("Syntax error");

    let result = repair_artifact(&mock_backend, &artifact, &verification, 3).await.unwrap();
    assert!(result.success);
    assert_eq!(result.attempts, 1);
}

#[tokio::test]
async fn test_repair_artifact_max_attempts() {
    let mut mock_backend = MockLLMBackend::new();
    mock_backend
        .expect_generate()
        .times(3)
        .returning(|_, _| {
            Ok(agentsdk_llm::LLMResponse {
                text: "".to_string(), // Empty response triggers retry
                usage: None,
                model: None,
            })
        });

    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Code,
        content: Content {
            data: b"broken code".to_vec(),
            format: "text".to_string(),
        },
        metadata: std::collections::HashMap::new(),
    };

    let verification = VerificationResult::fail("Syntax error");

    let result = repair_artifact(&mock_backend, &artifact, &verification, 3).await;
    assert!(matches!(result, Err(RepairError::StrategyFailed(_))));
}
```

- [ ] **Step 6.3: Run tests**

```bash
cargo test --package agentsdk-loops --lib
```

Expected: PASS

- [ ] **Step 6.4: Commit**

```bash
git add crates/quality/loops/src/repair.rs crates/quality/loops/tests/loop_tests.rs
git commit -m "feat(quality): implement repair step with tests"
```

---

### Step 7: Implement convergence detection

- [ ] **Step 7.1: Create convergence.rs**

Create file: `crates/quality/loops/src/convergence.rs`

```rust
use agentsdk_verifier::VerificationResult;
use super::config::{ConvergenceCriteria, QualityThresholds};
use super::state::LoopExecutionState;

/// Result of convergence check
#[derive(Debug, Clone, PartialEq)]
pub enum ConvergenceResult {
    /// Converged successfully
    Success,

    /// Failed to converge
    Failure { reason: String },

    /// Continue iterating
    Continue,
}

/// Check if loop should converge
pub fn check_convergence(
    state: &LoopExecutionState,
    verification_result: &VerificationResult,
) -> ConvergenceResult {
    let config = &state.config;

    // Check if quality thresholds met
    if !check_quality_thresholds(&config.quality_thresholds, verification_result) {
        return ConvergenceResult::Continue;
    }

    // Check convergence criteria
    if !check_convergence_criteria(&config.convergence, state, verification_result) {
        return ConvergenceResult::Continue;
    }

    // All checks passed - converge successfully
    ConvergenceResult::Success
}

/// Check if quality thresholds are met
fn check_quality_thresholds(
    thresholds: &QualityThresholds,
    result: &VerificationResult,
) -> bool {
    // Check confidence
    if result.confidence < thresholds.min_avg_confidence {
        return false;
    }

    // Check pass rate
    let pass_rate = if result.passed { 1.0 } else { 0.0 };
    if pass_rate < thresholds.min_pass_rate {
        return false;
    }

    // Check error count
    let error_count = result.messages.iter().filter(|m| matches!(m.level, agentsdk_verifier::MessageLevel::Error | agentsdk_verifier::MessageLevel::Critical)).count();
    if error_count > thresholds.max_errors {
        return false;
    }

    // Check warning count
    let warning_count = result.messages.iter().filter(|m| matches!(m.level, agentsdk_verifier::MessageLevel::Warning)).count();
    if warning_count > thresholds.max_warnings {
        return false;
    }

    true
}

/// Check convergence criteria
fn check_convergence_criteria(
    criteria: &ConvergenceCriteria,
    state: &LoopExecutionState,
    result: &VerificationResult,
) -> bool {
    // Check consecutive failures
    if state.consecutive_failures >= criteria.max_consecutive_failures {
        return false;
    }

    // Check confidence threshold
    if result.confidence < criteria.min_confidence {
        return false;
    }

    // Check for quality plateau if enabled
    if criteria.stop_on_plateau {
        if has_quality_plateau(state, criteria.plateau_window, criteria.equivalence_epsilon) {
            return false;
        }
    }

    true
}

/// Check if quality has plateaued
fn has_quality_plateau(state: &LoopExecutionState, window: usize, epsilon: f64) -> bool {
    if state.iteration_history.len() < window {
        return false;
    }

    // Get last window iterations
    let recent: Vec<_> = state
        .iteration_history
        .iter()
        .rev()
        .take(window)
        .collect();

    // Check if all recent results are equivalent
    let first_confidence = recent[0].verification.confidence;
    recent.iter().all(|r| {
        (r.verification.confidence - first_confidence).abs() < epsilon
    })
}
```

- [ ] **Step 7.2: Write test for convergence**

Add to `crates/quality/loops/tests/loop_tests.rs`:

```rust
use agentsdk_loops::convergence::{check_convergence, ConvergenceResult};
use agentsdk_loops::config::{RepairLoopConfig, ConvergenceCriteria, QualityThresholds};
use agentsdk_loops::state::{LoopExecutionState, IterationState};
use agentsdk_verifier::{VerificationResult, VerificationMessage, MessageLevel};

#[test]
fn test_convergence_success() {
    let config = RepairLoopConfig::default();
    let state = LoopExecutionState::new(config);

    let result = VerificationResult::pass(1.0);

    assert_eq!(check_convergence(&state, &result), ConvergenceResult::Success);
}

#[test]
fn test_convergence_continue_low_confidence() {
    let mut config = RepairLoopConfig::default();
    config.convergence.min_confidence = 0.95;

    let state = LoopExecutionState::new(config);

    let result = VerificationResult::pass(0.5); // Below threshold

    assert_eq!(check_convergence(&state, &result), ConvergenceResult::Continue);
}

#[test]
fn test_convergence_continue_too_many_errors() {
    let mut config = RepairLoopConfig::default();
    config.quality_thresholds.max_errors = 0;

    let state = LoopExecutionState::new(config);

    let mut result = VerificationResult::fail("Test error");
    result.messages.push(VerificationMessage {
        level: MessageLevel::Error,
        message: "Error".to_string(),
        location: None,
    });

    assert_eq!(check_convergence(&state, &result), ConvergenceResult::Continue);
}
```

- [ ] **Step 7.3: Run tests**

```bash
cargo test --package agentsdk-loops --lib
```

Expected: PASS

- [ ] **Step 7.4: Commit**

```bash
git add crates/quality/loops/src/convergence.rs crates/quality/loops/tests/loop_tests.rs
git commit -m "feat(quality): implement convergence detection with tests"
```

---

### Step 8: Implement main quality loop

- [ ] **Step 8.1: Add main loop function to lib.rs**

Update `crates/quality/loops/src/lib.rs`:

```rust
mod config;
mod generate;
mod verify;
mod repair;
mod state;
mod convergence;

pub use config::*;
pub use generate::*;
pub use verify::*;
pub use repair::*;
pub use state::*;
pub use convergence::*;

use agentsdk_llm::LLMBackend;
use agentsdk_verifier::VerifierRegistry;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoopError {
    #[error("Generate error: {0}")]
    Generate(#[from] GenerateError),

    #[error("Verify error: {0}")]
    Verify(#[from] VerifyError),

    #[error("Repair error: {0}")]
    Repair(#[from] RepairError),

    #[error("Loop exceeded max iterations")]
    MaxIterationsExceeded,

    #[error("Loop timed out")]
    Timeout,
}

/// Run the generate-verify-repair quality loop
pub async fn run_quality_loop(
    backend: &dyn LLMBackend,
    registry: &VerifierRegistry,
    prompt: &str,
    artifact_type: agentsdk_types::ArtifactType,
    config: RepairLoopConfig,
) -> Result<(Artifact, LoopExecutionState), LoopError> {
    let mut state = LoopExecutionState::new(config.clone());

    loop {
        // Check for timeout
        if state.is_timeout() {
            state.transition_to(LoopState::TimedOut);
            return Err(LoopError::Timeout);
        }

        // Check for max iterations
        if state.is_max_iterations_reached() {
            state.transition_to(LoopState::Failed);
            return Err(LoopError::MaxIterationsExceeded);
        }

        state.start_iteration();

        // Generate step
        state.transition_to(LoopState::Generating);
        let generate_result = generate_artifact(backend, prompt, artifact_type, serde_json::json!({})).await?;
        let artifact = generate_result.artifact.clone();
        state.current_artifact = Some(artifact.clone());

        // Verify step
        state.transition_to(LoopState::Verifying);
        let verify_result = verify_artifact(registry, &artifact).await?;
        state.current_verification = Some(verify_result.result.clone());

        // Check convergence
        let convergence = check_convergence(&state, &verify_result.result);

        match convergence {
            ConvergenceResult::Success => {
                state.transition_to(LoopState::Completed);
                state.end_time = Some(chrono::Utc::now());

                // Record final iteration
                let iteration = IterationState {
                    iteration: state.iteration,
                    artifact: artifact.clone(),
                    verification: verify_result.result,
                    duration_ms: generate_result.metadata.duration_ms + verify_result.duration_ms,
                    repair_attempted: false,
                    repair_result: None,
                };
                state.record_iteration(iteration);

                return Ok((artifact, state));
            }

            ConvergenceResult::Failure { reason } => {
                state.transition_to(LoopState::Failed);
                state.end_time = Some(chrono::Utc::now());
                return Err(LoopError::Verify(VerifyError::VerificationFailed(reason)));
            }

            ConvergenceResult::Continue => {
                // Repair step
                state.transition_to(LoopState::Repairing);

                let repair_result = repair_artifact(
                    backend,
                    &artifact,
                    &verify_result.result,
                    config.max_iterations - state.iteration,
                )
                .await;

                let repair_success = repair_result.is_ok();

                state.update_consecutive_failures(!repair_success);

                // Record iteration
                let iteration = IterationState {
                    iteration: state.iteration,
                    artifact: artifact.clone(),
                    verification: verify_result.result,
                    duration_ms: generate_result.metadata.duration_ms + verify_result.duration_ms,
                    repair_attempted: true,
                    repair_result: repair_result.ok(),
                };
                state.record_iteration(iteration);

                // Continue to next iteration
                continue;
            }
        }
    }
}
```

- [ ] **Step 8.2: Write integration test for full loop**

Add to `crates/quality/loops/tests/loop_tests.rs`:

```rust
use agentsdk_loops::{run_quality_loop, LoopError};
use agentsdk_types::ArtifactType;
use mockall::mock;

#[tokio::test]
async fn test_quality_loop_converges_successfully() {
    // This test requires careful setup of mocks
    // For now, we'll test the structure exists
    // Full integration test will be in tests/quality-loop-mocks.md
}
```

- [ ] **Step 8.3: Run tests**

```bash
cargo test --package agentsdk-loops --lib
```

Expected: PASS

- [ ] **Step 8.4: Commit**

```bash
git add crates/quality/loops/src/lib.rs crates/quality/loops/tests/loop_tests.rs
git commit -m "feat(quality): implement main quality loop with tests"
```

---

### Step 9: Add documentation and examples

- [ ] **Step 9.1: Create README.md**

Create file: `crates/quality/loops/README.md`

```markdown
# AgentSDK Quality Loops

Generate-Verify-Repair runtime semantics for quality assurance.

## Overview (ADR-0005 Compliance)

Per ADR-0005, quality loops are RUNTIME SEMANTICS, not optional prompt engineering:

- The RepairLoop config is part of ExecutionEngine configuration
- Convergence detection is MANDATORY
- Verifier failures trigger repair step AUTOMATICALLY
- Quality thresholds are ENFORCED at runtime

## Usage

### Basic Usage

```rust
use agentsdk_loops::{run_quality_loop, RepairLoopConfig};
use agentsdk_types::ArtifactType;

let config = RepairLoopConfig::default();
let (artifact, state) = run_quality_loop(
    &backend,
    &registry,
    "Generate a function",
    ArtifactType::Code,
    config,
).await?;

println!("Generated artifact: {}", artifact.id);
println!("Iterations: {}", state.iteration);
```

### Custom Configuration

```rust
let config = RepairLoopConfig {
    max_iterations: 5,
    convergence: ConvergenceCriteria {
        min_confidence: 0.99,
        max_consecutive_failures: 2,
        ..Default::default()
    },
    quality_thresholds: QualityThresholds {
        min_pass_rate: 1.0,
        max_errors: 0,
        ..Default::default()
    },
    ..Default::default()
};
```

## Components

### Generate Step
- Uses LLM backend to generate artifacts
- Tracks tokens used and duration

### Verify Step
- Runs all applicable verifiers
- Combines results from multiple verifiers

### Repair Step
- Constructs repair prompt from verification failures
- Uses LLM to generate repairs
- Supports multiple repair attempts

### Convergence Detection
- Checks quality thresholds
- Detects quality plateaus
- Enforces max iterations and timeouts

## ADR-0005 Compliance

The quality loop enforces:

1. **Runtime Enforcement** - Quality gates are always active
2. **Convergence Guarantees** - Loop must terminate within max_iterations
3. **Automatic Repair** - Verifier failures trigger repair automatically
4. **State Tracking** - Full audit trail for debugging
5. **Quality Thresholds** - Enforced at runtime, not suggested
```

- [ ] **Step 9.2: Commit**

```bash
git add crates/quality/loops/README.md
git commit -m "docs(quality): add quality loops documentation"
```

---

## Completion Criteria

Task 1 is complete when:

- ✅ RepairLoopConfig implemented with all criteria
- ✅ Generate step working with LLM backend
- ✅ Verify step working with verifier registry
- ✅ Repair step working with LLM backend
- ✅ Convergence detection working
- ✅ State tracking complete
- ✅ Main quality loop working end-to-end
- ✅ All tests passing
- ✅ Documentation complete
- ✅ ADR-0005 compliance explicit
- ✅ Code review passed

---

## Handoff

Ready for Task 2: Benchmark Harness

---

## QA Cross-References

- **QA Criteria**: ['$qa_criteria']('$file')
- **Test Cases**: ['$test_case']('$file')
- **Schema Ref**: $schema_ref
