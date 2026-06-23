//! Configuration layering with priority: CLI args > Environment > Config file > Defaults.
//!
//! This module implements a layered configuration system that allows configuration
//! to be overridden at different levels:
//!
//! 1. **Defaults** - Base configuration from schema defaults
//! 2. **Config file** - YAML workflow file settings
//! 3. **Environment variables** - `BENCH_*` environment variables
//! 4. **CLI arguments** - Command-line flags (highest priority)
//!
//! Once a configuration is frozen, it cannot be modified.

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use tracing::debug;

/// Source of a configuration value.
///
/// Used to track where each configuration value came from and
/// to implement priority ordering during resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigSource {
    /// Default value from schema
    Default,
    /// Value loaded from YAML config file
    File,
    /// Value from environment variable
    Environment,
    /// Value from CLI argument (highest priority)
    CliArgument,
}

impl ConfigSource {
    /// Get priority value for ordering (higher = more important).
    ///
    /// Used to determine which source takes precedence when merging configs.
    pub fn priority(&self) -> u8 {
        match self {
            ConfigSource::Default => 0,
            ConfigSource::File => 1,
            ConfigSource::Environment => 2,
            ConfigSource::CliArgument => 3,
        }
    }
}

/// Layered configuration with source tracking and freeze protection.
///
/// This struct wraps a `WorkflowFile` and tracks which configuration
/// values came from which source. It can be frozen to prevent further
/// modifications once the benchmark starts.
#[derive(Debug, Clone)]
pub struct LayeredConfig {
    /// The underlying workflow configuration
    pub workflow: crate::workflow::WorkflowFile,

    /// Map of config key paths to their sources
    pub source_map: HashMap<String, ConfigSource>,

    /// Whether the configuration is frozen (immutable)
    frozen: bool,
}

impl LayeredConfig {
    /// Create a new LayeredConfig from a WorkflowFile (File source).
    pub fn from_file_config(workflow: crate::workflow::WorkflowFile) -> Self {
        let mut source_map = HashMap::new();

        // Mark all file-loaded keys as File source
        if workflow.providers.is_some() {
            source_map.insert("providers".to_string(), ConfigSource::File);
        }
        if workflow.models.is_some() {
            source_map.insert("models".to_string(), ConfigSource::File);
        }
        if workflow.workspace.is_some() {
            source_map.insert("workspace".to_string(), ConfigSource::File);
        }

        Self {
            workflow,
            source_map,
            frozen: false,
        }
    }

    /// Apply environment variable overrides.
    ///
    /// Reads `BENCH_*` environment variables and applies them to the configuration.
    /// Environment variables override config file values but CLI args override both.
    ///
    /// Supported environment variables:
    /// - `BENCH_SERVER_HOST` → providers.llama_cpp_with_vulkan.config.host
    /// - `BENCH_SERVER_PORT` → providers.llama_cpp_with_vulkan.config.port
    /// - `BENCH_GPU_LAYERS` → providers.*.hosting.max_concurrent_models
    /// - `BENCH_OUTPUT_DIR` → workspace.output_path
    pub fn apply_env_overrides(&mut self) -> Result<()> {
        self.check_frozen()?;

        debug!("applying environment variable overrides");

        // BENCH_SERVER_HOST → providers.llama_cpp_with_vulkan.config.host
        if let Ok(host) = std::env::var("BENCH_SERVER_HOST") {
            debug!("BENCH_SERVER_HOST={}", host);
            if let Some(ref mut providers) = self.workflow.providers {
                if let Some(ref mut provider) = providers.providers.get_mut("llama_cpp_with_vulkan") {
                    if provider.config.is_none() {
                        provider.config = Some(crate::workflow::ProviderConnectionConfig::default());
                    }
                    if let Some(ref mut config) = provider.config {
                        config.host = host.clone();
                        self.source_map.insert(
                            "providers.llama_cpp_with_vulkan.config.host".to_string(),
                            ConfigSource::Environment,
                        );
                    }
                }
            }
        }

        // BENCH_SERVER_PORT → providers.llama_cpp_with_vulkan.config.port
        if let Ok(port_str) = std::env::var("BENCH_SERVER_PORT") {
            debug!("BENCH_SERVER_PORT={}", port_str);
            let port: u16 = port_str.parse().map_err(|e| {
                anyhow!("Invalid BENCH_SERVER_PORT: '{}'. Must be a u16: {}", port_str, e)
            })?;
            if let Some(ref mut providers) = self.workflow.providers {
                if let Some(ref mut provider) = providers.providers.get_mut("llama_cpp_with_vulkan") {
                    if provider.config.is_none() {
                        provider.config = Some(crate::workflow::ProviderConnectionConfig::default());
                    }
                    if let Some(ref mut config) = provider.config {
                        config.port = port;
                        self.source_map.insert(
                            "providers.llama_cpp_with_vulkan.config.port".to_string(),
                            ConfigSource::Environment,
                        );
                    }
                }
            }
        }

        // BENCH_OUTPUT_DIR → workspace.output_path
        if let Ok(output_dir) = std::env::var("BENCH_OUTPUT_DIR") {
            debug!("BENCH_OUTPUT_DIR={}", output_dir);
            if self.workflow.workspace.is_none() {
                self.workflow.workspace = Some(crate::workflow::WorkspaceConfig::default());
            }
            if let Some(ref mut workspace) = self.workflow.workspace {
                workspace.output_path = output_dir.clone();
                self.source_map.insert(
                    "workspace.output_path".to_string(),
                    ConfigSource::Environment,
                );
            }
        }

        debug!("environment variable overrides applied");
        Ok(())
    }

    /// Apply CLI argument overrides.
    ///
    /// CLI arguments override both config file and environment variable values.
    ///
    /// # Arguments
    ///
    /// * `max_tokens` - CLI-provided max_tokens (overrides config file and env var)
    /// * `temperature` - CLI-provided temperature (overrides config file and env var)
    /// * `output_dir` - CLI-provided output_dir (overrides config file and env var)
    /// * `server_url` - CLI-provided server URL (overrides config file and env var)
    pub fn apply_cli_overrides(
        &mut self,
        max_tokens: Option<usize>,
        temperature: Option<f64>,
        output_dir: Option<String>,
        server_url: Option<String>,
    ) -> Result<()> {
        self.check_frozen()?;

        debug!("applying CLI argument overrides");

        // Apply max_tokens override to all models
        if let Some(tokens) = max_tokens {
            debug!("CLI override: max_tokens={}", tokens);
            if let Some(ref mut models) = self.workflow.models {
                for (model_name, ref mut model) in models.models.iter_mut() {
                    model.execution.max_tokens = tokens;
                    self.source_map.insert(
                        format!("models.{}.execution.max_tokens", model_name),
                        ConfigSource::CliArgument,
                    );
                }
            }
        }

        // Apply temperature override to all models
        if let Some(temp) = temperature {
            debug!("CLI override: temperature={}", temp);
            if let Some(ref mut models) = self.workflow.models {
                for (model_name, ref mut model) in models.models.iter_mut() {
                    model.execution.temperature = temp as f32;
                    self.source_map.insert(
                        format!("models.{}.execution.temperature", model_name),
                        ConfigSource::CliArgument,
                    );
                }
            }
        }

        // Apply output_dir override
        if let Some(dir) = output_dir {
            debug!("CLI override: output_dir={}", dir);
            if self.workflow.workspace.is_none() {
                self.workflow.workspace = Some(crate::workflow::WorkspaceConfig::default());
            }
            if let Some(ref mut workspace) = self.workflow.workspace {
                workspace.output_path = dir.clone();
                self.source_map.insert(
                    "workspace.output_path".to_string(),
                    ConfigSource::CliArgument,
                );
            }
        }

        // Apply server_url override
        if let Some(url) = server_url {
            debug!("CLI override: server_url={}", url);
            if let Some(ref mut providers) = self.workflow.providers {
                if let Some(ref mut provider) = providers.providers.get_mut("llama_cpp_with_vulkan") {
                    if provider.config.is_none() {
                        provider.config = Some(crate::workflow::ProviderConnectionConfig::default());
                    }
                    if let Some(ref mut config) = provider.config {
                        // Parse host and port from URL
                        let url_str = url.trim();
                        let (host, port) = if url_str.starts_with("http://") || url_str.starts_with("https://") {
                            let after_proto = url_str
                                .trim_start_matches("http://")
                                .trim_start_matches("https://");

                            let (host_part, port_part) = after_proto.split_once(':').unwrap_or((after_proto, "1234"));
                            let host = host_part.split('/').next().unwrap_or("localhost");
                            let port = port_part.split('/').next().unwrap_or("1234");

                            (host.to_string(), port.parse::<u16>().unwrap_or(1234))
                        } else {
                            (url_str.to_string(), 1234)
                        };

                        config.host = host;
                        config.port = port;
                        self.source_map.insert(
                            "providers.llama_cpp_with_vulkan.config.host".to_string(),
                            ConfigSource::CliArgument,
                        );
                        self.source_map.insert(
                            "providers.llama_cpp_with_vulkan.config.port".to_string(),
                            ConfigSource::CliArgument,
                        );
                    }
                }
            }
        }

        debug!("CLI argument overrides applied");
        Ok(())
    }

    /// Freeze the configuration.
    ///
    /// Once frozen, the configuration cannot be modified. This prevents
    /// accidental changes after the benchmark has started.
    pub fn freeze(mut self) -> Result<Self> {
        self.frozen = true;
        debug!("configuration frozen");
        Ok(self)
    }

    /// Check if the configuration is frozen.
    ///
    /// Returns an error if the configuration is frozen and a modification is attempted.
    fn check_frozen(&self) -> Result<()> {
        if self.frozen {
            Err(anyhow!("Config is frozen — cannot modify after benchmark starts"))
        } else {
            Ok(())
        }
    }

    /// Get the source of a configuration key.
    ///
    /// Returns `None` if the key is not in the source map.
    pub fn get_source(&self, key: &str) -> Option<ConfigSource> {
        self.source_map.get(key).copied()
    }

    /// Check if a configuration key was set by CLI arguments.
    ///
    /// Returns true if the key exists and has source `CliArgument`.
    pub fn is_cli_set(&self, key: &str) -> bool {
        matches!(
            self.source_map.get(key),
            Some(ConfigSource::CliArgument)
        )
    }

    /// Check if a configuration key was set by environment variables.
    ///
    /// Returns true if the key exists and has source `Environment`.
    pub fn is_env_set(&self, key: &str) -> bool {
        matches!(
            self.source_map.get(key),
            Some(ConfigSource::Environment)
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::WorkflowFile;

    #[test]
    fn test_config_source_priority() {
        assert!(ConfigSource::Default.priority() < ConfigSource::File.priority());
        assert!(ConfigSource::File.priority() < ConfigSource::Environment.priority());
        assert!(ConfigSource::Environment.priority() < ConfigSource::CliArgument.priority());
    }

    #[test]
    fn test_from_file_config_marks_source() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
workspace:
  output_path: "./outputs"
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let layered = LayeredConfig::from_file_config(workflow);

        assert_eq!(
            layered.get_source("providers"),
            Some(ConfigSource::File)
        );
        assert_eq!(
            layered.get_source("models"),
            Some(ConfigSource::File)
        );
        assert_eq!(
            layered.get_source("workspace"),
            Some(ConfigSource::File)
        );
    }

    #[test]
    fn test_apply_env_overrides_host() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let mut layered = LayeredConfig::from_file_config(workflow);

        std::env::set_var("BENCH_SERVER_HOST", "remote-host");
        layered.apply_env_overrides().unwrap();
        std::env::remove_var("BENCH_SERVER_HOST");

        assert_eq!(
            layered
                .workflow
                .providers
                .and_then(|p| p.providers.get("llama_cpp_with_vulkan"))
                .and_then(|p| p.config.as_ref())
                .map(|c| c.host.as_str()),
            Some("remote-host")
        );
        assert_eq!(
            layered.get_source("providers.llama_cpp_with_vulkan.config.host"),
            Some(ConfigSource::Environment)
        );
        assert!(layered.is_env_set("providers.llama_cpp_with_vulkan.config.host"));
    }

    #[test]
    fn test_apply_env_overrides_port() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let mut layered = LayeredConfig::from_file_config(workflow);

        std::env::set_var("BENCH_SERVER_PORT", "9999");
        layered.apply_env_overrides().unwrap();
        std::env::remove_var("BENCH_SERVER_PORT");

        assert_eq!(
            layered
                .workflow
                .providers
                .and_then(|p| p.providers.get("llama_cpp_with_vulkan"))
                .and_then(|p| p.config.as_ref())
                .map(|c| c.port),
            Some(9999)
        );
        assert_eq!(
            layered.get_source("providers.llama_cpp_with_vulkan.config.port"),
            Some(ConfigSource::Environment)
        );
    }

    #[test]
    fn test_apply_env_overrides_output_dir() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let mut layered = LayeredConfig::from_file_config(workflow);

        std::env::set_var("BENCH_OUTPUT_DIR", "/custom/outputs");
        layered.apply_env_overrides().unwrap();
        std::env::remove_var("BENCH_OUTPUT_DIR");

        assert_eq!(
            layered
                .workflow
                .workspace
                .as_ref()
                .map(|w| w.output_path.as_str()),
            Some("/custom/outputs")
        );
        assert_eq!(
            layered.get_source("workspace.output_path"),
            Some(ConfigSource::Environment)
        );
    }

    #[test]
    fn test_apply_cli_overrides_max_tokens() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
    execution:
      max_tokens: 100
      temperature: 0.7
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let mut layered = LayeredConfig::from_file_config(workflow);

        layered
            .apply_cli_overrides(Some(500), None, None, None)
            .unwrap();

        assert_eq!(
            layered
                .workflow
                .models
                .and_then(|m| m.models.get("test-model"))
                .map(|m| m.execution.max_tokens),
            Some(500)
        );
        assert_eq!(
            layered.get_source("models.test-model.execution.max_tokens"),
            Some(ConfigSource::CliArgument)
        );
        assert!(layered.is_cli_set("models.test-model.execution.max_tokens"));
    }

    #[test]
    fn test_apply_cli_overrides_temperature() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
    execution:
      max_tokens: 100
      temperature: 0.7
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let mut layered = LayeredConfig::from_file_config(workflow);

        layered
            .apply_cli_overrides(None, Some(0.9), None, None)
            .unwrap();

        assert_eq!(
            layered
                .workflow
                .models
                .and_then(|m| m.models.get("test-model"))
                .map(|m| m.execution.temperature),
            0.9f32
        );
        assert_eq!(
            layered.get_source("models.test-model.execution.temperature"),
            Some(ConfigSource::CliArgument)
        );
    }

    #[test]
    fn test_apply_cli_overrides_output_dir() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
workspace:
  output_path: "./outputs"
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let mut layered = LayeredConfig::from_file_config(workflow);

        layered
            .apply_cli_overrides(None, None, Some("/cli/outputs".to_string()), None)
            .unwrap();

        assert_eq!(
            layered
                .workflow
                .workspace
                .as_ref()
                .map(|w| w.output_path.as_str()),
            Some("/cli/outputs")
        );
        assert_eq!(
            layered.get_source("workspace.output_path"),
            Some(ConfigSource::CliArgument)
        );
    }

    #[test]
    fn test_apply_cli_overrides_server_url() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let mut layered = LayeredConfig::from_file_config(workflow);

        layered
            .apply_cli_overrides(None, None, None, Some("http://remote-server:9999".to_string()))
            .unwrap();

        assert_eq!(
            layered
                .workflow
                .providers
                .and_then(|p| p.providers.get("llama_cpp_with_vulkan"))
                .and_then(|p| p.config.as_ref())
                .map(|c| c.host.as_str()),
            Some("remote-server")
        );
        assert_eq!(
            layered
                .workflow
                .providers
                .and_then(|p| p.providers.get("llama_cpp_with_vulkan"))
                .and_then(|p| p.config.as_ref())
                .map(|c| c.port),
            Some(9999)
        );
    }

    #[test]
    fn test_freeze_prevents_modification() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let mut layered = LayeredConfig::from_file_config(workflow);

        layered.freeze().unwrap();

        let result = layered.apply_env_overrides();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("frozen"));

        let result = layered.apply_cli_overrides(None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_overrides_env() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
    execution:
      max_tokens: 100
      temperature: 0.7
workspace:
  output_path: "./outputs"
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let mut layered = LayeredConfig::from_file_config(workflow);

        // Set env var
        std::env::set_var("BENCH_OUTPUT_DIR", "/env/outputs");
        layered.apply_env_overrides().unwrap();

        // Apply CLI override (should override env var)
        layered
            .apply_cli_overrides(None, None, Some("/cli/outputs".to_string()), None)
            .unwrap();
        std::env::remove_var("BENCH_OUTPUT_DIR");

        // CLI should win
        assert_eq!(
            layered
                .workflow
                .workspace
                .as_ref()
                .map(|w| w.output_path.as_str()),
            Some("/cli/outputs")
        );
        assert_eq!(
            layered.get_source("workspace.output_path"),
            Some(ConfigSource::CliArgument)
        );
    }

    #[test]
    fn test_env_overrides_file() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
workspace:
  output_path: "./outputs"
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let mut layered = LayeredConfig::from_file_config(workflow);

        // Apply env override
        std::env::set_var("BENCH_OUTPUT_DIR", "/env/outputs");
        layered.apply_env_overrides().unwrap();
        std::env::remove_var("BENCH_OUTPUT_DIR");

        // Env should override file config
        assert_eq!(
            layered
                .workflow
                .workspace
                .as_ref()
                .map(|w| w.output_path.as_str()),
            Some("/env/outputs")
        );
        assert_eq!(
            layered.get_source("workspace.output_path"),
            Some(ConfigSource::Environment)
        );
    }

    #[test]
    fn test_invalid_env_port_fails() {
        let yaml = r#"
workflow_id: test
name: Test Workflow
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
"#;
        let workflow = WorkflowFile::from_yaml(yaml).unwrap();
        let mut layered = LayeredConfig::from_file_config(workflow);

        std::env::set_var("BENCH_SERVER_PORT", "not-a-number");
        let result = layered.apply_env_overrides();
        std::env::remove_var("BENCH_SERVER_PORT");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid BENCH_SERVER_PORT"));
    }
}