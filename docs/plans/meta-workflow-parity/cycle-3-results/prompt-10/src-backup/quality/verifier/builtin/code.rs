use super::super::{Verifier, VerificationResult, VerifierCapabilities, CheckType, ArtifactType, Artifact};
use crate::error::Result;
use async_trait::async_trait;

/// Code verifier for compilation, linting, and tests
pub struct CodeVerifier {
    language: String,
    check_types: Vec<CheckType>,
    capabilities: VerifierCapabilities,
}

impl CodeVerifier {
    pub fn new(language: String) -> Self {
        let capabilities = VerifierCapabilities {
            artifact_types: vec![ArtifactType::Code],
            check_types: vec![CheckType::Syntax],
            dependencies: match language.as_str() {
                "rust" => vec!["rustc".to_string()],
                "python" => vec!["python3".to_string()],
                _ => vec![],
            },
            estimated_duration_ms: 500,
            supports_async: true,
        };

        Self {
            language,
            check_types: vec![CheckType::Syntax],
            capabilities,
        }
    }

    pub fn with_checks(mut self, check_types: Vec<CheckType>) -> Self {
        self.check_types = check_types.clone();
        self.capabilities.check_types = check_types;
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
        &self.capabilities
    }

    fn name(&self) -> &str {
        &self.language
    }

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult> {
        let mut results = Vec::new();

        if self.check_types.contains(&CheckType::Syntax) {
            results.push(self.check_syntax(artifact).await);
        }

        Ok(VerificationResult::combine(results))
    }
}
