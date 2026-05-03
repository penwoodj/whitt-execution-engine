use super::super::{Verifier, VerificationResult, VerifierCapabilities, CheckType, ArtifactType, Artifact, MessageLevel, VerificationMessage};
use crate::error::Result;
use async_trait::async_trait;

/// Documentation verifier for completeness and formatting
pub struct DocsVerifier {
    required_sections: Vec<String>,
    check_formatting: bool,
    capabilities: VerifierCapabilities,
}

impl DocsVerifier {
    pub fn new() -> Self {
        let capabilities = VerifierCapabilities {
            artifact_types: vec![ArtifactType::Documentation],
            check_types: vec![CheckType::Documentation, CheckType::Formatting],
            dependencies: vec![],
            estimated_duration_ms: 100,
            supports_async: true,
        };

        Self {
            required_sections: vec![
                "Introduction".to_string(),
                "Usage".to_string(),
                "API".to_string(),
            ],
            check_formatting: true,
            capabilities,
        }
    }

    pub fn with_sections(mut self, sections: Vec<String>) -> Self {
        self.required_sections = sections;
        self
    }

    pub fn without_formatting(mut self) -> Self {
        self.check_formatting = false;
        self.capabilities.check_types = vec![CheckType::Documentation];
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
            messages.push(VerificationMessage {
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
            messages.push(VerificationMessage {
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
        &self.capabilities
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
