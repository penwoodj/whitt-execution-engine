use whitt_execution_engine::quality::verifier::{
    VerificationResult, CheckType, ArtifactType, Artifact, Content, MessageLevel,
    Verifier, VerifierCapabilities, VerifierRegistry,
};
use whitt_execution_engine::quality::verifier::builtin::{CodeVerifier, DocsVerifier, ConfigVerifier};
use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;

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
    assert_eq!(result.messages[0].level, MessageLevel::Error);
}

#[test]
fn test_verification_result_combine() {
    let result1 = VerificationResult::pass(0.8);
    let result2 = VerificationResult::pass(0.9);
    let combined = VerificationResult::combine(vec![result1, result2]);

    assert!(combined.passed);
    assert!((combined.confidence - 0.85).abs() < 0.0001); // (0.8 + 0.9) / 2
}

struct MockVerifier {
    name: String,
    capabilities: VerifierCapabilities,
}

#[async_trait]
impl Verifier for MockVerifier {
    fn capabilities(&self) -> &VerifierCapabilities {
        &self.capabilities
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn verify(&self, artifact: &Artifact) -> whitt_execution_engine::error::Result<VerificationResult> {
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
        metadata: HashMap::new(),
    };

    let result = verifier.verify(&artifact).await.unwrap();
    assert!(!result.passed);
}

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
        metadata: HashMap::new(),
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
        metadata: HashMap::new(),
    };

    let result = verifier.verify(&artifact).await.unwrap();
    assert!(!result.passed);
}

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
        metadata: HashMap::new(),
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
        metadata: HashMap::new(),
    };

    let result = verifier.verify(&artifact).await.unwrap();
    assert!(!result.messages.is_empty());
}

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
        metadata: HashMap::new(),
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
        metadata: HashMap::new(),
    };

    let result = verifier.verify(&artifact).await;
    assert!(result.is_err() || !result.unwrap().passed);
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
        metadata: HashMap::new(),
    };

    let result = verifier.verify(&artifact_missing).await.unwrap();
    assert!(!result.passed);
    assert!(result.messages.iter().any(|m| m.message.contains("version")));
}

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
