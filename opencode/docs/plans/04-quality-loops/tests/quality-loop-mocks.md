# Quality Loop Mock LLM

## Mock LLM Backend 1: Always Converge

```rust
use agentsdk_llm::{LLMBackend, LLMResponse, Usage};
use async_trait::async_trait;

pub struct AlwaysConvergeLLM {
    generation_count: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl AlwaysConvergeLLM {
    pub fn new() -> Self {
        Self {
            generation_count: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    pub fn generation_count(&self) -> usize {
        self.generation_count.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[async_trait]
impl LLMBackend for AlwaysConvergeLLM {
    async fn generate(&self, prompt: &str, context: Option<&agentsdk_llm::LLMContext>) -> Result<LLMResponse, Box<dyn std::error::Error>> {
        self.generation_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        // Generate valid code that passes verifiers
        let code = r#"
fn main() {
    println!("Hello, world!");
}
"#;

        Ok(LLMResponse {
            text: code.to_string(),
            usage: Some(Usage {
                prompt_tokens: prompt.len(),
                completion_tokens: code.len(),
                total_tokens: prompt.len() + code.len(),
            }),
            model: Some("mock-model".to_string()),
        })
    }
}
```

## Mock LLM Backend 2: Never Converge

```rust
pub struct NeverConvergeLLM {
    generation_count: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl NeverConvergeLLM {
    pub fn new() -> Self {
        Self {
            generation_count: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    pub fn generation_count(&self) -> usize {
        self.generation_count.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[async_trait]
impl LLMBackend for NeverConvergeLLM {
    async fn generate(&self, prompt: &str, context: Option<&agentsdk_llm::LLMContext>) -> Result<LLMResponse, Box<dyn std::error::Error>> {
        self.generation_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        // Always generate invalid code
        let code = "fn main( {"; // Syntax error

        Ok(LLMResponse {
            text: code.to_string(),
            usage: Some(Usage {
                prompt_tokens: prompt.len(),
                completion_tokens: code.len(),
                total_tokens: prompt.len() + code.len(),
            }),
            model: Some("mock-model".to_string()),
        })
    }
}
```

## Mock LLM Backend 3: Fixed Iterations

```rust
pub struct FixedIterationsLLM {
    generation_count: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    converge_after: usize,
}

impl FixedIterationsLLM {
    pub fn new(converge_after: usize) -> Self {
        Self {
            generation_count: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            converge_after,
        }
    }

    pub fn generation_count(&self) -> usize {
        self.generation_count.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[async_trait]
impl LLMBackend for FixedIterationsLLM {
    async fn generate(&self, prompt: &str, context: Option<&agentsdk_llm::LLMContext>) -> Result<LLMResponse, Box<dyn std::error::Error>> {
        let count = self.generation_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let code = if count >= self.converge_after {
            // Generate valid code after threshold
            r#"
fn main() {
    println!("Hello, world!");
}
"#
        } else {
            // Generate invalid code before threshold
            "fn main( {"
        };

        Ok(LLMResponse {
            text: code.to_string(),
            usage: Some(Usage {
                prompt_tokens: prompt.len(),
                completion_tokens: code.len(),
                total_tokens: prompt.len() + code.len(),
            }),
            model: Some("mock-model".to_string()),
        })
    }
}
```

## Mock LLM Backend 4: Repair Success

```rust
pub struct RepairSuccessLLM {
    generation_count: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl RepairSuccessLLM {
    pub fn new() -> Self {
        Self {
            generation_count: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    pub fn generation_count(&self) -> usize {
        self.generation_count.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[async_trait]
impl LLMBackend for RepairSuccessLLM {
    async fn generate(&self, prompt: &str, context: Option<&agentsdk_llm::LLMContext>) -> Result<LLMResponse, Box<dyn std::error::Error>> {
        self.generation_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        // If prompt contains "repair", generate fixed code
        // Otherwise generate broken code
        let code = if prompt.contains("repair") {
            r#"
fn main() {
    println!("Hello, world!");
}
"#
        } else {
            "fn main( {"
        };

        Ok(LLMResponse {
            text: code.to_string(),
            usage: Some(Usage {
                prompt_tokens: prompt.len(),
                completion_tokens: code.len(),
                total_tokens: prompt.len() + code.len(),
            }),
            model: Some("mock-model".to_string()),
        })
    }
}
```

## Usage in Tests

```rust
#[tokio::test]
async fn test_quality_loop_converges() {
    let mock_llm = Arc::new(AlwaysConvergeLLM::new());
    let registry = VerifierRegistry::new();

    // Register code verifier
    registry.register(Arc::new(CodeVerifier::new("rust".to_string()))).await.unwrap();

    let config = RepairLoopConfig {
        max_iterations: 10,
        ..Default::default()
    };

    let (artifact, state) = run_quality_loop(
        mock_llm.as_ref(),
        &registry,
        "Generate a function",
        ArtifactType::Code,
        config,
    )
    .await
    .unwrap();

    assert_eq!(mock_llm.generation_count(), 1); // Only one generation needed
    assert_eq!(state.iteration, 1);
    assert!(matches!(state.state, LoopState::Completed));
}
```
