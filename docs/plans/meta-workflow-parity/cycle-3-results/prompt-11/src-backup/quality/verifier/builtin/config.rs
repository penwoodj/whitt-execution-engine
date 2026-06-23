use super::super::{Verifier, VerificationResult, VerifierCapabilities, CheckType, ArtifactType, Artifact, MessageLevel, VerificationMessage};
use crate::error::{Result, Error};
use async_trait::async_trait;
use serde_json::Value;

/// Configuration verifier for schema validation
pub struct ConfigVerifier {
    schema: Option<Value>,
    capabilities: VerifierCapabilities,
}

impl ConfigVerifier {
    pub fn new() -> Self {
        let capabilities = VerifierCapabilities {
            artifact_types: vec![ArtifactType::Config],
            check_types: vec![CheckType::SchemaValidation],
            dependencies: vec![],
            estimated_duration_ms: 50,
            supports_async: true,
        };

        Self { schema: None, capabilities }
    }

    pub fn with_schema(mut self, schema: Value) -> Self {
        self.schema = Some(schema);
        self
    }

    fn validate_schema(&self, value: &Value) -> VerificationResult {
        if let Some(schema) = &self.schema {
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

    fn validate_against_schema(&self, value: &Value, schema: &Value) -> Vec<VerificationMessage> {
        let mut messages = Vec::new();

        if let Some(required) = schema.get("required").and_then(|v| v.as_array()) {
            if let Some(obj) = value.as_object() {
                for field in required {
                    if let Some(field_name) = field.as_str() {
                        if !obj.contains_key(field_name) {
                            messages.push(VerificationMessage {
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
        &self.capabilities
    }

    fn name(&self) -> &str {
        "config-verifier"
    }

    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult> {
        let content = String::from_utf8_lossy(&artifact.content.data).to_string();
        let format = artifact.content.format.as_str();

        let value = match format {
            "json" => serde_json::from_str::<Value>(&content)
                .map_err(|e| Error::verification(format!("Invalid JSON: {}", e)))?,
            "yaml" => {
                serde_json::from_str::<Value>(&content)
                    .map_err(|e| Error::verification(format!("Invalid YAML/JSON: {}", e)))?
            }
            _ => return Ok(VerificationResult::fail(format!("Unsupported format: {}", format))),
        };

        Ok(self.validate_schema(&value))
    }
}
