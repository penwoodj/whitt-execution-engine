//! Unified schema config resolution.
//!
//! Provides unified loading and resolution of provider and model configurations
//! from unified YAML schema (v2.0).

use crate::config::provider::ProvidersConfig;
use crate::model::schema::ModelsConfig;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::debug;

// ---------------------------------------------------------------------------
// Top-level unified config
// ---------------------------------------------------------------------------

/// Complete unified configuration from unified YAML schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnifiedConfig {
    /// Workflow ID (unique identifier).
    pub workflow_id: Option<String>,

    /// Workflow name (human-readable).
    pub name: Option<String>,

    /// Workflow description.
    pub description: Option<String>,

    /// Workflow version (semver).
    #[serde(default = "default_version")]
    pub version: String,

    /// Workflow author.
    pub author: Option<String>,

    /// Workflow tags for categorization.
    #[serde(default)]
    pub tags: Vec<String>,

    /// Schema version (e.g., "2.0.0").
    #[serde(default = "default_schema_version")]
    pub schema_version: String,

    /// Minimum schema version supported.
    #[serde(default = "default_schema_version")]
    pub min_schema_version: String,

    /// Provider configurations.
    pub providers: ProvidersConfig,

    /// Model specifications.
    pub models: ModelsConfig,
}

fn default_schema_version() -> String {
    "2.0.0".into()
}

fn default_version() -> String {
    "1.0.0".into()
}

impl UnifiedConfig {
    /// Load unified config from a YAML string.
    pub fn from_yaml(yaml: &str) -> anyhow::Result<Self> {
        let config: UnifiedConfig = serde_saphyr::from_str(yaml)?;

        // Validate schema version
        config.validate_schema_version()?;

        // Validate min_schema_version
        config.validate_min_schema_version()?;

        Ok(config)
    }

    /// Load unified config from a file.
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        Self::from_yaml(&contents)
    }

    /// Validate that schema version is supported.
    pub fn validate_schema_version(&self) -> anyhow::Result<()> {
        let version = &self.schema_version;

        // Parse version string (e.g., "2.0.0")
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() < 2 {
            anyhow::bail!("Invalid schema version format: '{}'. Expected 'X.Y.Z'", version);
        }

        let major: u32 = parts[0]
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid major version: {}", e))?;

        // Support schema version 2.0.0 and higher
        if major < 2 {
            anyhow::bail!(
                "Schema version '{}' is not supported. Minimum required: 2.0.0",
                version
            );
        }

        debug!(version = %version, "Schema version validated");

        Ok(())
    }

    /// Validate that min_schema_version is supported (>= 2.0.0).
    pub fn validate_min_schema_version(&self) -> anyhow::Result<()> {
        let version = &self.min_schema_version;

        // Parse version string (e.g., "2.0.0")
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() < 2 {
            anyhow::bail!("Invalid min_schema_version format: '{}'. Expected 'X.Y.Z'", version);
        }

        let major: u32 = parts[0]
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid major version: {}", e))?;

        // min_schema_version must be 2.0.0 or higher
        if major < 2 {
            anyhow::bail!(
                "min_schema_version '{}' is not supported. Minimum required: 2.0.0",
                version
            );
        }

        debug!(min_schema_version = %version, "min_schema_version validated");

        Ok(())
    }

    /// Resolve a model's full configuration by merging:
    /// 1. Provider defaults
    /// 2. Model-specific overrides
    /// 3. Step-level overrides (if provided)
    pub fn resolve_model_config(
        &self,
        model_name: &str,
        step_overrides: Option<&HashMap<String, Value>>,
    ) -> Result<ResolvedModelConfig, anyhow::Error> {
        // Get model specification
        let model_spec = self
            .models
            .models
            .get(model_name)
            .ok_or_else(|| anyhow::anyhow!("Model '{}' not found in configuration", model_name))?;

        // Get provider configuration for this model
        let provider_name = &model_spec.host.r#type;
        let provider_config = self
            .providers
            .providers
            .get(provider_name)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Provider '{}' not found in configuration",
                    provider_name
                )
            })?;

        // Resolve host (provider config -> model overrides -> step overrides)
        let host = if let Some(step) = step_overrides.and_then(|s| s.get("host")) {
            step.as_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| provider_name.clone())
        } else if let Some(host) = model_spec.host.connection_settings.get("host") {
            host.clone()
        } else if let Some(config) = &provider_config.config {
            config.host.clone()
        } else {
            provider_name.clone()
        };

        // Resolve port
        let port = if let Some(step) = step_overrides.and_then(|s| s.get("port")) {
            step.as_u64()
                .map(|p| p as u32)
                .unwrap_or(1234)
        } else if let Some(port) = model_spec.host.connection_settings.get("port") {
            port.parse().unwrap_or(1234)
        } else if let Some(config) = &provider_config.config {
            config.port
        } else {
            1234
        };

        // Resolve temperature
        let temperature = step_overrides
            .and_then(|s| s.get("temperature"))
            .and_then(|t| t.as_f64())
            .map(|t| t as f32)
            .or(None);

        // Resolve max_tokens
        let max_tokens = step_overrides
            .and_then(|s| s.get("max_tokens"))
            .and_then(|t| t.as_u64())
            .map(|t| t as usize)
            .or(None);

        // Resolve timeout (from provider request config)
        let timeout_secs = if let Some(step) = step_overrides.and_then(|s| s.get("timeout_secs")) {
            step.as_u64().unwrap_or(120)
        } else if let Some(requests) = &provider_config.requests {
            requests.request_timeout_secs
        } else {
            120
        };

        // Resolve retry config
        let max_retries = if let Some(step) = step_overrides.and_then(|s| s.get("max_retries")) {
            step.as_u64().map(|r| r as u32).unwrap_or(3)
        } else if let Some(requests) = &provider_config.requests {
            requests.retry.as_ref().map(|r| r.max_retries).unwrap_or(3)
        } else {
            3
        };

        debug!(
            model = %model_name,
            host = %host,
            port = port,
            timeout_secs = timeout_secs,
            max_retries = max_retries,
            "Resolved model configuration"
        );

        Ok(ResolvedModelConfig {
            host,
            port,
            temperature,
            max_tokens,
            timeout_secs,
            max_retries,
        })
    }
}

// ---------------------------------------------------------------------------
// Resolved model configuration
// ---------------------------------------------------------------------------

/// Fully resolved model configuration after merging all sources.
#[derive(Debug, Clone)]
pub struct ResolvedModelConfig {
    /// Server host address.
    pub host: String,

    /// Server port number.
    pub port: u32,

    /// Sampling temperature (optional, uses backend default if None).
    pub temperature: Option<f32>,

    /// Maximum tokens to generate (optional, uses backend default if None).
    pub max_tokens: Option<usize>,

    /// Request timeout in seconds.
    pub timeout_secs: u64,

    /// Maximum number of retry attempts.
    pub max_retries: u32,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_unified_config() {
        let yaml = r#"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 1234
models:
  test-model:
    name: "Test Model"
    host:
      type: "llama_cpp_with_vulkan"
      connection_settings:
        host: "localhost"
        port: "1234"
"#;
        let config = UnifiedConfig::from_yaml(yaml).expect("parse");
        assert_eq!(config.schema_version, "2.0.0");
        assert!(config.providers.providers.contains_key("llama_cpp_with_vulkan"));
        assert!(config.models.models.contains_key("test-model"));
    }

    #[test]
    fn validate_schema_version_rejects_1_x() {
        let yaml = r#"
schema_version: "1.5.0"
providers:
  test_provider: {}
models:
  test-model:
    name: "Test"
    host:
      type: "test_provider"
        "#;
        let result = UnifiedConfig::from_yaml(yaml);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not supported"));
    }

    #[test]
    fn resolve_model_config_merges_sources() {
        let yaml = r#"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: provider-host
      port: 9000
    requests:
      request_timeout_secs: 60
      retry:
        max_retries: 5
models:
  test-model:
    name: "Test Model"
    host:
      type: "llama_cpp_with_vulkan"
      connection_settings:
        host: model-host
        port: "1234"
"#;
        let config = UnifiedConfig::from_yaml(yaml).expect("parse");
        let resolved = config
            .resolve_model_config("test-model", None)
            .expect("resolve");

        // Model connection settings override provider defaults
        assert_eq!(resolved.host, "model-host");
        assert_eq!(resolved.port, 1234);

        // Provider request settings apply when not overridden
        assert_eq!(resolved.timeout_secs, 60);
        assert_eq!(resolved.max_retries, 5);
    }

    #[test]
    fn resolve_model_config_step_overrides() {
        let yaml = r#"
schema_version: "2.0.0"
providers:
  llama_cpp_with_vulkan:
    config:
      host: provider-host
      port: 9000
models:
  test-model:
    name: "Test Model"
    host:
      type: "llama_cpp_with_vulkan"
      connection_settings:
        host: model-host
        port: "1234"
"#;
        let config = UnifiedConfig::from_yaml(yaml).expect("parse");

        let mut step_overrides = HashMap::new();
        step_overrides.insert("host".to_string(), serde_json::json!("step-host"));
        step_overrides.insert("port".to_string(), serde_json::json!(7000));
        step_overrides.insert("temperature".to_string(), serde_json::json!(0.5));
        step_overrides.insert("max_tokens".to_string(), serde_json::json!(256));

        let resolved = config
            .resolve_model_config("test-model", Some(&step_overrides))
            .expect("resolve");

        // Step overrides take highest priority
        assert_eq!(resolved.host, "step-host");
        assert_eq!(resolved.port, 7000);
        assert_eq!(resolved.temperature, Some(0.5));
        assert_eq!(resolved.max_tokens, Some(256));
    }

    #[test]
    fn resolve_model_config_returns_defaults() {
        let yaml = r#"
schema_version: "2.0.0"
providers:
  test_provider: {}
models:
  test-model:
    name: "Test Model"
    host:
      type: "test_provider"
"#;
        let config = UnifiedConfig::from_yaml(yaml).expect("parse");
        let resolved = config
            .resolve_model_config("test-model", None)
            .expect("resolve");

        // Defaults when not specified
        assert_eq!(resolved.host, "test_provider");
        assert_eq!(resolved.port, 1234);
        assert_eq!(resolved.temperature, None);
        assert_eq!(resolved.max_tokens, None);
        assert_eq!(resolved.timeout_secs, 120);
        assert_eq!(resolved.max_retries, 3);
    }

    #[test]
    fn resolve_model_config_unknown_model_fails() {
        let yaml = r#"
schema_version: "2.0.0"
providers:
  test_provider: {}
models:
"#;
        let config = UnifiedConfig::from_yaml(yaml).expect("parse");
        let result = config.resolve_model_config("unknown", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn resolve_model_config_unknown_provider_fails() {
        let yaml = r#"
schema_version: "2.0.0"
providers:
  test_provider: {}
models:
  test-model:
    name: "Test Model"
    host:
      type: "unknown_provider"
"#;
        let config = UnifiedConfig::from_yaml(yaml).expect("parse");
        let result = config.resolve_model_config("test-model", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn parse_all_top_level_fields() {
        let yaml = r#"
workflow_id: "test-workflow"
name: "Test Workflow"
description: "A test workflow for verifying all fields"
version: "1.5.2"
author: "test-author"
tags:
  - test
  - example
schema_version: "2.0.0"
min_schema_version: "2.0.0"
providers:
  test_provider: {}
models:
"#;
        let config = UnifiedConfig::from_yaml(yaml).expect("parse");
        assert_eq!(config.workflow_id, Some("test-workflow".to_string()));
        assert_eq!(config.name, Some("Test Workflow".to_string()));
        assert_eq!(config.description, Some("A test workflow for verifying all fields".to_string()));
        assert_eq!(config.version, "1.5.2");
        assert_eq!(config.author, Some("test-author".to_string()));
        assert_eq!(config.tags, vec!["test".to_string(), "example".to_string()]);
        assert_eq!(config.schema_version, "2.0.0");
        assert_eq!(config.min_schema_version, "2.0.0");
    }

    #[test]
    fn parse_minimal_top_level_fields_with_defaults() {
        let yaml = r#"
providers:
  test_provider: {}
models:
"#;
        let config = UnifiedConfig::from_yaml(yaml).expect("parse");
        assert_eq!(config.workflow_id, None);
        assert_eq!(config.name, None);
        assert_eq!(config.description, None);
        assert_eq!(config.version, "1.0.0"); // default
        assert_eq!(config.author, None);
        assert_eq!(config.tags, Vec::<String>::new()); // empty vector by default
        assert_eq!(config.schema_version, "2.0.0"); // default
        assert_eq!(config.min_schema_version, "2.0.0"); // default
    }
}
