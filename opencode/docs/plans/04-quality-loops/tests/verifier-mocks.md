# Verifier Mock Implementations

## Mock Verifier 1: Always Pass Verifier

```rust
use agentsdk_verifier::{Verifier, VerificationResult, VerifierCapabilities, CheckType};
use agentsdk_types::{Artifact, ArtifactType};
use async_trait::async_trait;

pub struct AlwaysPassVerifier {
    name: String,
    capabilities: VerifierCapabilities,
}

impl AlwaysPassVerifier {
    pub fn new(name: String) -> Self {
        Self {
            capabilities: VerifierCapabilities {
                artifact_types: vec!["code".to_string()],
                check_types: vec![CheckType::Syntax],
                dependencies: vec![],
                estimated_duration_ms: 10,
                supports_async: true,
            },
            name,
        }
    }
}

#[async_trait]
impl Verifier for AlwaysPassVerifier {
    fn capabilities(&self) -> &VerifierCapabilities {
        &self.capabilities
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn verify(&self, _artifact: &Artifact) -> Result<VerificationResult, agentsdk_verifier::VerificationError> {
        Ok(VerificationResult::pass(1.0))
    }
}
```

## Mock Verifier 2: Always Fail Verifier

```rust
pub struct AlwaysFailVerifier {
    name: String,
    capabilities: VerifierCapabilities,
    error_message: String,
}

impl AlwaysFailVerifier {
    pub fn new(name: String, error_message: String) -> Self {
        Self {
            capabilities: VerifierCapabilities {
                artifact_types: vec!["code".to_string()],
                check_types: vec![CheckType::Syntax],
                dependencies: vec![],
                estimated_duration_ms: 10,
                supports_async: true,
            },
            name,
            error_message,
        }
    }
}

#[async_trait]
impl Verifier for AlwaysFailVerifier {
    fn capabilities(&self) -> &VerifierCapabilities {
        &self.capabilities
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn verify(&self, _artifact: &Artifact) -> Result<VerificationResult, agentsdk_verifier::VerificationError> {
        Ok(VerificationResult::fail(&self.error_message))
    }
}
```

## Mock Verifier 3: Threshold Verifier

```rust
pub struct ThresholdVerifier {
    name: String,
    capabilities: VerifierCapabilities,
    min_length: usize,
}

impl ThresholdVerifier {
    pub fn new(name: String, min_length: usize) -> Self {
        Self {
            capabilities: VerifierCapabilities {
                artifact_types: vec!["code".to_string()],
                check_types: vec![CheckType::Syntax],
                dependencies: vec![],
                estimated_duration_ms: 10,
                supports_async: true,
            },
            name,
            min_length,
        }
    }
}

#[async_trait]
impl Verifier for ThresholdVerifier {
    fn capabilities(&self) -> &VerifierCapabilities {
        &self.capabilities
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult, agentsdk_verifier::VerificationError> {
        if artifact.content.data.len() >= self.min_length {
            Ok(VerificationResult::pass(1.0))
        } else {
            Ok(VerificationResult::fail(format!(
                "Content too short: {} < {}",
                artifact.content.data.len(),
                self.min_length
            )))
        }
    }
}
```

## Mock Verifier 4: Random Verifier

```rust
pub struct RandomVerifier {
    name: String,
    capabilities: VerifierCapabilities,
    pass_probability: f64,
}

impl RandomVerifier {
    pub fn new(name: String, pass_probability: f64) -> Self {
        Self {
            capabilities: VerifierCapabilities {
                artifact_types: vec!["code".to_string()],
                check_types: vec![CheckType::Syntax],
                dependencies: vec![],
                estimated_duration_ms: 10,
                supports_async: true,
            },
            name,
            pass_probability,
        }
    }
}

#[async_trait]
impl Verifier for RandomVerifier {
    fn capabilities(&self) -> &VerifierCapabilities {
        &self.capabilities
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn verify(&self, _artifact: &Artifact) -> Result<VerificationResult, agentsdk_verifier::VerificationError> {
        let passed = rand::random::<f64>() < self.pass_probability;

        if passed {
            Ok(VerificationResult::pass(self.pass_probability))
        } else {
            Ok(VerificationResult::fail("Random failure".to_string()))
        }
    }
}
```

## Usage in Tests

```rust
#[tokio::test]
async fn test_multiple_verifiers() {
    let registry = VerifierRegistry::new();

    registry.register(Arc::new(AlwaysPassVerifier::new("pass1".to_string()))).await.unwrap();
    registry.register(Arc::new(AlwaysFailVerifier::new("fail1".to_string(), "Test error".to_string()))).await.unwrap();
    registry.register(Arc::new(ThresholdVerifier::new("threshold".to_string(), 100))).await.unwrap();

    let artifact = Artifact {
        id: "test".to_string(),
        artifact_type: ArtifactType::Code,
        content: Content {
            data: vec![0; 200], // Long enough for threshold
            format: "text".to_string(),
        },
        metadata: HashMap::new(),
    };

    let verifiers = registry.get_for_artifact_type("code").await.unwrap();
    assert_eq!(verifiers.len(), 3);

    let mut pass_count = 0;
    for verifier in &verifiers {
        let result = verifier.verify(&artifact).await.unwrap();
        if result.passed {
            pass_count += 1;
        }
    }

    assert_eq!(pass_count, 2); // pass1 and threshold pass, fail1 fails
}
```
