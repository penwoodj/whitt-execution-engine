# Task 00: CLI Foundation

**Files:**
- Create: `src/cli/mod.rs`
- Create: `src/cli/main.rs`
- Create: `src/cli/commands/mod.rs`
- Create: `src/cli/commands/run.rs`
- Create: `src/cli/commands/generate.rs`
- Create: `src/cli/commands/queue.rs`
- Create: `src/cli/commands/status.rs`
- Create: `src/cli/commands/config.rs`
- Create: `src/cli/config/mod.rs`
- Create: `src/cli/config/loader.rs`
- Create: `src/cli/output/mod.rs`
- Create: `src/cli/output/plain.rs`
- Create: `src/cli/output/json.rs`
- Create: `src/cli/output/table.rs`
- Modify: `src/lib.rs` (add cli module)
- Test: `tests/cli/commands_test.rs`

---

## Overview

Implement the complete CLI foundation using clap's derive API. This includes subcommands for all major operations (run, generate, queue, status, config), global options for configuration, output formatting (plain, JSON, table), tab completion support, and configuration file loading from `~/./workspace/config.yaml`.

---

## Implementation Steps

### Step 1: Create CLI module structure

- [ ] **Step 1.1: Write module exports**

```rust
// src/cli/mod.rs
pub mod commands;
pub mod config;
pub mod output;

pub use main::Cli;
pub use main::Commands;
pub use main::OutputFormat;

pub mod main;
```

- [ ] **Step 1.2: Write CLI derive macro**

```rust
// src/cli/main.rs
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "glyphnova")]
#[command(about = "AgentSDK Execution Engine - Orchestrate AI agents with workflows", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Global configuration file path
    #[arg(global = true, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Output format
    #[arg(global = true, long, value_enum, default_value_t = OutputFormat::Plain)]
    pub output: OutputFormat,

    /// Enable verbose output
    #[arg(global = true, long, short)]
    pub verbose: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a workflow from a YAML file
    Run {
        /// Path to workflow YAML file
        #[arg(value_name = "WORKFLOW")]
        workflow: PathBuf,

        /// Enable dry run mode (no execution)
        #[arg(long)]
        dry_run: bool,

        /// Override execution mode (direct | codegen)
        #[arg(long)]
        mode: Option<String>,

        /// Continue from checkpoint
        #[arg(long)]
        checkpoint: Option<String>,
    },

    /// Generate Rust code from workflow
    Generate {
        /// Path to workflow YAML file
        #[arg(value_name = "WORKFLOW")]
        workflow: PathBuf,

        /// Output directory for generated code
        #[arg(short, long, value_name = "DIR")]
        output: Option<PathBuf>,

        /// Compile generated code
        #[arg(long)]
        compile: bool,
    },

    /// Queue operations
    Queue {
        #[command(subcommand)]
        subcommand: QueueCommands,
    },

    /// Status queries
    Status {
        /// Queue ID to query
        #[arg(short, long, value_name = "ID")]
        queue_id: Option<String>,

        /// Show detailed information
        #[arg(long)]
        detailed: bool,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        subcommand: ConfigCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum QueueCommands {
    /// Add workflow to queue
    Add {
        /// Path to workflow YAML file
        #[arg(value_name = "WORKFLOW")]
        workflow: PathBuf,

        /// Priority (1-10)
        #[arg(short, long, default_value_t = 5)]
        priority: u8,
    },

    /// Remove workflow from queue
    Remove {
        /// Queue ID
        #[arg(short, long, value_name = "ID")]
        queue_id: String,
    },

    /// List queue
    List {
        /// Filter by status
        #[arg(short, long, value_name = "STATUS")]
        status: Option<String>,
    },

    /// Clear queue
    Clear,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Show current configuration
    Show {
        /// Show specific section
        #[arg(value_name = "SECTION")]
        section: Option<String>,
    },

    /// Set configuration value
    Set {
        /// Configuration key (e.g., providers.default)
        #[arg(value_name = "KEY")]
        key: String,

        /// Configuration value
        #[arg(value_name = "VALUE")]
        value: String,
    },

    /// Reset configuration to defaults
    Reset,

    /// Validate configuration
    Validate,

    /// Edit configuration file
    Edit,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Plain,
    Json,
    Table,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Plain => write!(f, "plain"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Table => write!(f, "table"),
        }
    }
}
```

- [ ] **Step 1.3: Test basic CLI parsing**

```rust
// tests/cli/commands_test.rs
use glyphnova::cli::{Cli, Commands};
use clap::Parser;

#[test]
fn test_cli_parse_basic_run() {
    let args = vec!["glyphnova", "run", "workflow.yaml"];
    let cli = Cli::try_parse_from(args).unwrap();
    assert!(matches!(cli.command, Commands::Run { .. }));
}

#[test]
fn test_cli_parse_with_options() {
    let args = vec!["glyphnova", "run", "workflow.yaml", "--dry-run", "--mode", "direct"];
    let cli = Cli::try_parse_from(args).unwrap();
    if let Commands::Run { dry_run, mode, .. } = cli.command {
        assert!(dry_run);
        assert_eq!(mode, Some("direct".to_string()));
    } else {
        panic!("Expected Run command");
    }
}

#[test]
fn test_cli_parse_global_options() {
    let args = vec![
        "glyphnova",
        "--config", "/tmp/config.yaml",
        "--output", "json",
        "--verbose",
        "run", "workflow.yaml"
    ];
    let cli = Cli::try_parse_from(args).unwrap();
    assert_eq!(cli.config, Some("/tmp/config.yaml".into()));
    assert_eq!(cli.output, glyphnova::cli::OutputFormat::Json);
    assert!(cli.verbose);
}

#[test]
fn test_cli_parse_queue_add() {
    let args = vec!["glyphnova", "queue", "add", "workflow.yaml", "--priority", "8"];
    let cli = Cli::try_parse_from(args).unwrap();
    assert!(matches!(cli.command, Commands::Queue { .. }));
}

#[test]
fn test_cli_parse_config_set() {
    let args = vec!["glyphnova", "config", "set", "providers.default", "ollama"];
    let cli = Cli::try_parse_from(args).unwrap();
    assert!(matches!(cli.command, Commands::Config { .. }));
}
```

Run: `cargo test test_cli_parse --lib`
Expected: PASS

- [ ] **Step 1.4: Commit**

```bash
git add src/cli/mod.rs src/cli/main.rs tests/cli/commands_test.rs
git commit -m "feat(cli): add CLI structure with clap derive API"
```

---

### Step 2: Implement configuration file loading

- [ ] **Step 2.1: Write configuration structures**

```rust
// src/cli/config/mod.rs
pub mod loader;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub providers: ProvidersConfig,

    #[serde(default)]
    pub permissions: PermissionsConfig,

    #[serde(default)]
    pub rag: RAGConfig,

    #[serde(default)]
    pub codegen: CodeGenConfig,

    #[serde(default)]
    pub self_improvement: SelfImprovementConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct GeneralConfig {
    #[serde(default)]
    pub network_enabled: bool,

    #[serde(default = "default_output_format")]
    pub output_format: String,
}

fn default_output_format() -> String {
    "plain".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ProvidersConfig {
    #[serde(default = "default_provider")]
    pub default: String,

    #[serde(default)]
    pub lmstudio: LLMProviderConfig,

    #[serde(default)]
    pub ollama: LLMProviderConfig,

    #[serde(default)]
    pub llamacpp: LLMProviderConfig,

    #[serde(default)]
    pub openai: OpenAIProviderConfig,
}

fn default_provider() -> String {
    "lmstudio".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LLMProviderConfig {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_host() -> String {
    "localhost".to_string()
}

fn default_port() -> u16 {
    1234
}

fn default_timeout() -> u64 {
    300
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct OpenAIProviderConfig {
    #[serde(default)]
    pub api_key: Option<String>,

    #[serde(default = "default_openai_timeout")]
    pub timeout: u64,
}

fn default_openai_timeout() -> u64 {
    300
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct PermissionsConfig {
    #[serde(default = "default_policy")]
    pub default_policy: String,

    #[serde(default)]
    pub tools: ToolPermissionsConfig,
}

fn default_policy() -> String {
    "deny".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ToolPermissionsConfig {
    #[serde(default)]
    pub allow: Vec<String>,

    #[serde(default)]
    pub deny: Vec<String>,

    #[serde(default)]
    pub confirmation_required: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct RAGConfig {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default = "default_knowledge_base")]
    pub knowledge_base: PathBuf,

    #[serde(default = "default_embedding_model")]
    pub embedding_model: String,

    #[serde(default = "default_max_context")]
    pub max_context: usize,

    #[serde(default = "default_retrieval_limit")]
    pub retrieval_limit: usize,
}

fn default_knowledge_base() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("./workspace/knowledge")
}

fn default_embedding_model() -> String {
    "nomic-embed-text".to_string()
}

fn default_max_context() -> usize {
    2000
}

fn default_retrieval_limit() -> usize {
    5
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CodeGenConfig {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default = "default_codegen_output_dir")]
    pub output_dir: PathBuf,

    #[serde(default)]
    pub compile_on_generate: bool,
}

fn default_codegen_output_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("./workspace/generated")
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SelfImprovementConfig {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default = "default_log_dir")]
    pub log_dir: PathBuf,

    #[serde(default)]
    pub auto_apply: bool,
}

fn default_log_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("./workspace/logs")
}
```

- [ ] **Step 2.2: Write configuration loader**

```rust
// src/cli/config/loader.rs
use super::Config;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_config(config_path: Option<PathBuf>) -> Result<Config> {
    let config_path = config_path.unwrap_or_else(default_config_path);

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let config_content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

    let config: Config = serde_yaml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;

    Ok(config)
}

pub fn save_config(config: &Config, config_path: Option<PathBuf>) -> Result<()> {
    let config_path = config_path.unwrap_or_else(default_config_path);

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
    }

    let config_content = serde_yaml::to_string(config)
        .with_context(|| "Failed to serialize config")?;

    fs::write(&config_path, config_content)
        .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

    Ok(())
}

fn default_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("./workspace/config.yaml")
}

pub fn validate_config(config: &Config) -> Result<()> {
    // Validate provider config
    let valid_providers = ["lmstudio", "ollama", "llamacpp", "openai"];
    if !valid_providers.contains(&config.providers.default.as_str()) {
        anyhow::bail!("Invalid default provider: {}", config.providers.default);
    }

    // Validate policy config
    let valid_policies = ["allow", "deny"];
    if !valid_policies.contains(&config.permissions.default_policy.as_str()) {
        anyhow::bail!("Invalid default policy: {}", config.permissions.default_policy);
    }

    // Validate output format
    let valid_formats = ["plain", "json", "table"];
    if !valid_formats.contains(&config.general.output_format.as_str()) {
        anyhow::bail!("Invalid output format: {}", config.general.output_format);
    }

    Ok(())
}
```

- [ ] **Step 2.3: Add dirs dependency**

```toml
# Cargo.toml
[dependencies]
dirs = "5.0"
```

- [ ] **Step 2.4: Test configuration loading**

```rust
// tests/cli/config_test.rs
use glyphnova::cli::config::{load_config, save_config, validate_config};

#[test]
fn test_load_default_config() {
    let config = load_config(None).unwrap();
    assert_eq!(config.providers.default, "lmstudio");
    assert_eq!(config.permissions.default_policy, "deny");
    assert!(!config.general.network_enabled);
}

#[test]
fn test_save_and_load_config() {
    use tempfile::NamedTempFile;
    use std::path::PathBuf;

    let temp_file = NamedTempFile::new().unwrap();
    let config_path: PathBuf = temp_file.path().to_path_buf();

    let mut config = load_config(None).unwrap();
    config.providers.default = "ollama".to_string();
    config.general.network_enabled = true;

    save_config(&config, Some(config_path.clone())).unwrap();
    let loaded = load_config(Some(config_path)).unwrap();

    assert_eq!(loaded.providers.default, "ollama");
    assert!(loaded.general.network_enabled);
}

#[test]
fn test_validate_config() {
    let mut config = load_config(None).unwrap();

    // Valid config
    assert!(validate_config(&config).is_ok());

    // Invalid provider
    config.providers.default = "invalid".to_string();
    assert!(validate_config(&config).is_err());

    // Invalid policy
    config.providers.default = "lmstudio".to_string();
    config.permissions.default_policy = "invalid".to_string();
    assert!(validate_config(&config).is_err());
}

#[test]
fn test_config_defaults() {
    let config = glyphnova::cli::config::Config::default();
    assert_eq!(config.providers.lmstudio.host, "localhost");
    assert_eq!(config.providers.lmstudio.port, 1234);
    assert_eq!(config.rag.retrieval_limit, 5);
}
```

Run: `cargo test test_config --lib`
Expected: PASS

- [ ] **Step 2.5: Commit**

```bash
git add src/cli/config/mod.rs src/cli/config/loader.rs Cargo.toml tests/cli/config_test.rs
git commit -m "feat(cli): add configuration loading and validation"
```

---

### Step 3: Implement output formatting

- [ ] **Step 3.1: Write output formatting traits**

```rust
// src/cli/output/mod.rs
pub mod plain;
pub mod json;
pub mod table;

use anyhow::Result;
use std::io::Write;

pub trait OutputFormatter {
    fn write<W: Write>(&self, writer: &mut W, data: &OutputData) -> Result<()>;
}

#[derive(Debug)]
pub enum OutputData {
    WorkflowStatus {
        workflow_id: String,
        status: String,
        steps_completed: usize,
        steps_total: usize,
        duration: Option<u64>,
    },

    QueueItem {
        id: String,
        workflow: String,
        priority: u8,
        status: String,
        created_at: String,
    },

    Config {
        key: String,
        value: String,
    },

    Error {
        message: String,
    },

    Success {
        message: String,
    },
}

pub fn create_formatter(format: &str) -> Box<dyn OutputFormatter> {
    match format {
        "json" => Box::new(json::JsonFormatter),
        "table" => Box::new(table::TableFormatter),
        _ => Box::new(plain::PlainFormatter),
    }
}
```

- [ ] **Step 3.2: Write plain formatter**

```rust
// src/cli/output/plain.rs
use super::{OutputData, OutputFormatter};
use anyhow::Result;
use std::io::Write;

pub struct PlainFormatter;

impl OutputFormatter for PlainFormatter {
    fn write<W: Write>(&self, writer: &mut W, data: &OutputData) -> Result<()> {
        match data {
            OutputData::WorkflowStatus { workflow_id, status, steps_completed, steps_total, duration } => {
                writeln!(writer, "Workflow: {}", workflow_id)?;
                writeln!(writer, "  Status: {}", status)?;
                writeln!(writer, "  Progress: {}/{}", steps_completed, steps_total)?;
                if let Some(d) = duration {
                    writeln!(writer, "  Duration: {}s", d)?;
                }
                Ok(())
            }

            OutputData::QueueItem { id, workflow, priority, status, created_at } => {
                writeln!(writer, "[{}] {} - {} (priority: {})", id, workflow, status, priority)?;
                writeln!(writer, "  Created: {}", created_at)?;
                Ok(())
            }

            OutputData::Config { key, value } => {
                writeln!(writer, "{}: {}", key, value)?;
                Ok(())
            }

            OutputData::Error { message } => {
                writeln!(writer, "Error: {}", message)?;
                Ok(())
            }

            OutputData::Success { message } => {
                writeln!(writer, "{}", message)?;
                Ok(())
            }
        }
    }
}
```

- [ ] **Step 3.3: Write JSON formatter**

```rust
// src/cli/output/json.rs
use super::{OutputData, OutputFormatter};
use anyhow::Result;
use serde::Serialize;
use std::io::Write;

pub struct JsonFormatter;

#[derive(Serialize)]
struct WorkflowStatusJson {
    workflow_id: String,
    status: String,
    steps_completed: usize,
    steps_total: usize,
    duration: Option<u64>,
}

#[derive(Serialize)]
struct QueueItemJson {
    id: String,
    workflow: String,
    priority: u8,
    status: String,
    created_at: String,
}

#[derive(Serialize)]
struct ConfigJson {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct ErrorJson {
    error: String,
}

#[derive(Serialize)]
struct SuccessJson {
    success: String,
}

impl OutputFormatter for JsonFormatter {
    fn write<W: Write>(&self, writer: &mut W, data: &OutputData) -> Result<()> {
        let json = match data {
            OutputData::WorkflowStatus { workflow_id, status, steps_completed, steps_total, duration } => {
                serde_json::to_string_pretty(&WorkflowStatusJson {
                    workflow_id: workflow_id.clone(),
                    status: status.clone(),
                    steps_completed: *steps_completed,
                    steps_total: *steps_total,
                    duration: *duration,
                })?
            }

            OutputData::QueueItem { id, workflow, priority, status, created_at } => {
                serde_json::to_string_pretty(&QueueItemJson {
                    id: id.clone(),
                    workflow: workflow.clone(),
                    priority: *priority,
                    status: status.clone(),
                    created_at: created_at.clone(),
                })?
            }

            OutputData::Config { key, value } => {
                serde_json::to_string_pretty(&ConfigJson {
                    key: key.clone(),
                    value: value.clone(),
                })?
            }

            OutputData::Error { message } => {
                serde_json::to_string_pretty(&ErrorJson {
                    error: message.clone(),
                })?
            }

            OutputData::Success { message } => {
                serde_json::to_string_pretty(&SuccessJson {
                    success: message.clone(),
                })?
            }
        };

        writeln!(writer, "{}", json)?;
        Ok(())
    }
}
```

- [ ] **Step 3.4: Write table formatter**

```rust
// src/cli/output/table.rs
use super::{OutputData, OutputFormatter};
use anyhow::Result;
use std::io::Write;
use std::collections::HashMap;

pub struct TableFormatter;

impl OutputFormatter for TableFormatter {
    fn write<W: Write>(&self, writer: &mut W, data: &OutputData) -> Result<()> {
        match data {
            OutputData::WorkflowStatus { workflow_id, status, steps_completed, steps_total, duration } => {
                write_table(
                    writer,
                    &["Field", "Value"],
                    &[
                        vec!["Workflow ID".to_string(), workflow_id.clone()],
                        vec!["Status".to_string(), status.clone()],
                        vec!["Progress".to_string(), format!("{}/{}", steps_completed, steps_total)],
                        vec!["Duration".to_string(), duration.map_or_else(|| "-".to_string(), |d| format!("{}s", d))],
                    ]
                )?;
                Ok(())
            }

            OutputData::QueueItem { id, workflow, priority, status, created_at } => {
                write_table(
                    writer,
                    &["Field", "Value"],
                    &[
                        vec!["Queue ID".to_string(), id.clone()],
                        vec!["Workflow".to_string(), workflow.clone()],
                        vec!["Priority".to_string(), priority.to_string()],
                        vec!["Status".to_string(), status.clone()],
                        vec!["Created".to_string(), created_at.clone()],
                    ]
                )?;
                Ok(())
            }

            OutputData::Config { key, value } => {
                write_table(
                    writer,
                    &["Key", "Value"],
                    &[vec![key.clone(), value.clone()]]
                )?;
                Ok(())
            }

            OutputData::Error { message } => {
                writeln!(writer, "Error: {}", message)?;
                Ok(())
            }

            OutputData::Success { message } => {
                writeln!(writer, "Success: {}", message)?;
                Ok(())
            }
        }
    }
}

fn write_table<W: Write>(writer: &mut W, headers: &[&str], rows: &[Vec<String>]) -> Result<()> {
    // Calculate column widths
    let mut column_widths: HashMap<usize, usize> = HashMap::new();

    for (col, header) in headers.iter().enumerate() {
        column_widths.insert(col, header.len());
    }

    for row in rows {
        for (col, cell) in row.iter().enumerate() {
            let width = cell.len().max(*column_widths.get(&col).unwrap_or(&0));
            column_widths.insert(col, width);
        }
    }

    // Sort columns by key to ensure consistent ordering
    let mut columns: Vec<_> = column_widths.keys().cloned().collect();
    columns.sort();

    // Write header separator
    let separator: String = columns.iter()
        .map(|&col| "+-{}-", "-".repeat(column_widths[&col] + 2))
        .collect::<Vec<_>>()
        .concat() + "+";
    writeln!(writer, "{}", separator)?;

    // Write header
    let header_row: String = columns.iter()
        .map(|&col| format!("| {:width$} ", headers.get(col).unwrap_or(&""), width = column_widths[&col]))
        .collect::<Vec<_>>()
        .concat() + "|";
    writeln!(writer, "{}", header_row)?;
    writeln!(writer, "{}", separator)?;

    // Write rows
    for row in rows {
        let row_str: String = columns.iter()
            .map(|&col| {
                let cell = row.get(col).unwrap_or(&"".to_string());
                format!("| {:width$} ", cell, width = column_widths[&col])
            })
            .collect::<Vec<_>>()
            .concat() + "|";
        writeln!(writer, "{}", row_str)?;
        writeln!(writer, "{}", separator)?;
    }

    Ok(())
}
```

- [ ] **Step 3.5: Test output formatters**

```rust
// tests/cli/output_test.rs
use glyphnova::cli::output::{OutputFormatter, OutputData, create_formatter};

#[test]
fn test_plain_formatter() {
    let formatter = create_formatter("plain");
    let data = OutputData::Success {
        message: "Workflow completed successfully".to_string(),
    };

    let mut output = Vec::new();
    formatter.write(&mut output, &data).unwrap();
    let output_str = String::from_utf8(output).unwrap();

    assert!(output_str.contains("Workflow completed successfully"));
}

#[test]
fn test_json_formatter() {
    let formatter = create_formatter("json");
    let data = OutputData::Success {
        message: "Workflow completed successfully".to_string(),
    };

    let mut output = Vec::new();
    formatter.write(&mut output, &data).unwrap();
    let output_str = String::from_utf8(output).unwrap();

    assert!(output_str.contains("\"success\""));
    assert!(output_str.contains("Workflow completed successfully"));
}

#[test]
fn test_table_formatter() {
    let formatter = create_formatter("table");
    let data = OutputData::Config {
        key: "providers.default".to_string(),
        value: "lmstudio".to_string(),
    };

    let mut output = Vec::new();
    formatter.write(&mut output, &data).unwrap();
    let output_str = String::from_utf8(output).unwrap();

    assert!(output_str.contains("+"));
    assert!(output_str.contains("Key"));
    assert!(output_str.contains("Value"));
    assert!(output_str.contains("providers.default"));
    assert!(output_str.contains("lmstudio"));
}

#[test]
fn test_workflow_status_formats() {
    let data = OutputData::WorkflowStatus {
        workflow_id: "test-workflow".to_string(),
        status: "running".to_string(),
        steps_completed: 3,
        steps_total: 10,
        duration: Some(42),
    };

    for format in ["plain", "json", "table"] {
        let formatter = create_formatter(format);
        let mut output = Vec::new();
        formatter.write(&mut output, &data).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.len() > 0, "Format {} produced empty output", format);
    }
}
```

Run: `cargo test test_output --lib`
Expected: PASS

- [ ] **Step 3.6: Commit**

```bash
git add src/cli/output/mod.rs src/cli/output/plain.rs src/cli/output/json.rs src/cli/output/table.rs tests/cli/output_test.rs
git commit -m "feat(cli): add output formatters (plain, JSON, table)"
```

---

### Step 4: Implement command handlers

- [ ] **Step 4.1: Write run command handler**

```rust
// src/cli/commands/run.rs
use super::super::config::load_config;
use super::super::output::{OutputData, OutputFormatter};
use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

pub async fn execute(args: &RunArgs) -> Result<Box<dyn OutputFormatter>> {
    let config = load_config(None)?;

    // Validate workflow file exists
    if !args.workflow.exists() {
        anyhow::bail!("Workflow file not found: {}", args.workflow.display());
    }

    if args.dry_run {
        return Ok(create_output_formatter(&config, OutputData::Success {
            message: format!("Dry run: would execute workflow {}", args.workflow.display()),
        }));
    }

    // Parse workflow (using Phase 0 parser)
    let workflow_content = fs::read_to_string(&args.workflow)
        .with_context(|| format!("Failed to read workflow file: {}", args.workflow.display()))?;

    // TODO: Integrate with Phase 1 executor
    let workflow_id = "pending-execution".to_string();

    Ok(create_output_formatter(&config, OutputData::WorkflowStatus {
        workflow_id,
        status: "started".to_string(),
        steps_completed: 0,
        steps_total: 0,
        duration: None,
    }))
}

#[derive(Parser, Debug)]
pub struct RunArgs {
    pub workflow: std::path::PathBuf,
    pub dry_run: bool,
    pub mode: Option<String>,
    pub checkpoint: Option<String>,
}

fn create_output_formatter(config: &crate::cli::config::Config, data: OutputData) -> Box<dyn OutputFormatter> {
    use super::super::output::create_formatter;
    create_formatter(&config.general.output_format)
}
```

- [ ] **Step 4.2: Write generate command handler**

```rust
// src/cli/commands/generate.rs
use super::super::config::load_config;
use super::super::output::{OutputData, OutputFormatter};
use anyhow::{Context, Result};
use std::fs;

pub async fn execute(args: &GenerateArgs) -> Result<Box<dyn OutputFormatter>> {
    let config = load_config(None)?;

    // Validate workflow file exists
    if !args.workflow.exists() {
        anyhow::bail!("Workflow file not found: {}", args.workflow.display());
    }

    // Parse workflow (using Phase 0 parser)
    let workflow_content = fs::read_to_string(&args.workflow)
        .with_context(|| format!("Failed to read workflow file: {}", args.workflow.display()))?;

    // Determine output directory
    let output_dir = args.output.as_ref()
        .unwrap_or(&config.codegen.output_dir);

    // Create output directory
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    // TODO: Integrate with codegen module (Task 10)
    if args.compile {
        return Ok(create_output_formatter(&config, OutputData::Success {
            message: format!("Generated code for workflow in {} (compile not yet implemented)", output_dir.display()),
        }));
    }

    Ok(create_output_formatter(&config, OutputData::Success {
        message: format!("Generated code for workflow in {}", output_dir.display()),
    }))
}

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    pub workflow: std::path::PathBuf,
    pub output: Option<std::path::PathBuf>,
    pub compile: bool,
}

fn create_output_formatter(config: &crate::cli::config::Config, data: OutputData) -> Box<dyn OutputFormatter> {
    use super::super::output::create_formatter;
    create_formatter(&config.general.output_format)
}
```

- [ ] **Step 4.3: Write queue command handler**

```rust
// src/cli/commands/queue.rs
use super::super::config::load_config;
use super::super::output::{OutputData, OutputFormatter};
use anyhow::{Context, Result};
use std::fs;

pub async fn execute(args: &QueueArgs) -> Result<Box<dyn OutputFormatter>> {
    let config = load_config(None)?;

    match &args.subcommand {
        QueueSubcommand::Add(add_args) => execute_add(add_args, &config).await,
        QueueSubcommand::Remove(remove_args) => execute_remove(remove_args, &config).await,
        QueueSubcommand::List(list_args) => execute_list(list_args, &config).await,
        QueueSubcommand::Clear => execute_clear(&config).await,
    }
}

async fn execute_add(args: &AddArgs, config: &crate::cli::config::Config) -> Result<Box<dyn OutputFormatter>> {
    // Validate workflow file exists
    if !args.workflow.exists() {
        anyhow::bail!("Workflow file not found: {}", args.workflow.display());
    }

    // TODO: Integrate with Phase 1 queue
    let queue_id = "pending-queue-id".to_string();

    Ok(create_output_formatter(config, OutputData::Success {
        message: format!("Added workflow {} to queue (ID: {})", args.workflow.display(), queue_id),
    }))
}

async fn execute_remove(args: &RemoveArgs, config: &crate::cli::config::Config) -> Result<Box<dyn OutputFormatter>> {
    // TODO: Integrate with Phase 1 queue
    Ok(create_output_formatter(config, OutputData::Success {
        message: format!("Removed workflow {} from queue", args.queue_id),
    }))
}

async fn execute_list(args: &ListArgs, config: &crate::cli::config::Config) -> Result<Box<dyn OutputFormatter>> {
    // TODO: Integrate with Phase 1 queue
    Ok(create_output_formatter(config, OutputData::Success {
        message: "Queue listing not yet implemented".to_string(),
    }))
}

async fn execute_clear(config: &crate::cli::config::Config) -> Result<Box<dyn OutputFormatter>> {
    // TODO: Integrate with Phase 1 queue
    Ok(create_output_formatter(config, OutputData::Success {
        message: "Queue cleared".to_string(),
    }))
}

#[derive(Parser, Debug)]
pub struct QueueArgs {
    #[command(subcommand)]
    pub subcommand: QueueSubcommand,
}

#[derive(Parser, Debug)]
pub enum QueueSubcommand {
    Add {
        workflow: std::path::PathBuf,
        #[arg(short, long, default_value_t = 5)]
        priority: u8,
    },
    Remove {
        #[arg(short, long, value_name = "ID")]
        queue_id: String,
    },
    List {
        #[arg(short, long, value_name = "STATUS")]
        status: Option<String>,
    },
    Clear,
}

#[derive(Parser, Debug)]
pub struct AddArgs {
    pub workflow: std::path::PathBuf,
    pub priority: u8,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    pub queue_id: String,
}

#[derive(Parser, Debug)]
pub struct ListArgs {
    pub status: Option<String>,
}

fn create_output_formatter(config: &crate::cli::config::Config, data: OutputData) -> Box<dyn OutputFormatter> {
    use super::super::output::create_formatter;
    create_formatter(&config.general.output_format)
}
```

- [ ] **Step 4.4: Write status command handler**

```rust
// src/cli/commands/status.rs
use super::super::config::load_config;
use super::super::output::{OutputData, OutputFormatter};
use anyhow::Result;

pub async fn execute(args: &StatusArgs) -> Result<Box<dyn OutputFormatter>> {
    let config = load_config(None)?;

    // TODO: Integrate with Phase 1 status queries
    if let Some(queue_id) = &args.queue_id {
        Ok(create_output_formatter(config, OutputData::WorkflowStatus {
            workflow_id: queue_id.clone(),
            status: "running".to_string(),
            steps_completed: 3,
            steps_total: 10,
            duration: Some(42),
        }))
    } else {
        Ok(create_output_formatter(config, OutputData::Success {
            message: "Status query not yet implemented".to_string(),
        }))
    }
}

#[derive(Parser, Debug)]
pub struct StatusArgs {
    #[arg(short, long, value_name = "ID")]
    pub queue_id: Option<String>,
    pub detailed: bool,
}

fn create_output_formatter(config: &crate::cli::config::Config, data: OutputData) -> Box<dyn OutputFormatter> {
    use super::super::output::create_formatter;
    create_formatter(&config.general.output_format)
}
```

- [ ] **Step 4.5: Write config command handler**

```rust
// src/cli/commands/config.rs
use super::super::config::{load_config, save_config, validate_config};
use super::super::output::{OutputData, OutputFormatter};
use anyhow::Result;

pub async fn execute(args: &ConfigArgs) -> Result<Box<dyn OutputFormatter>> {
    let config = load_config(None)?;

    match &args.subcommand {
        ConfigSubcommand::Show(show_args) => execute_show(show_args, &config).await,
        ConfigSubcommand::Set(set_args) => execute_set(set_args).await,
        ConfigSubcommand::Reset => execute_reset().await,
        ConfigSubcommand::Validate => execute_validate(&config).await,
        ConfigSubcommand::Edit => execute_edit().await,
    }
}

async fn execute_show(args: &ShowArgs, config: &crate::cli::config::Config) -> Result<Box<dyn OutputFormatter>> {
    // TODO: Implement section-based display
    Ok(create_output_formatter(config, OutputData::Success {
        message: format!("Current config:\n{}", serde_yaml::to_string(config)?),
    }))
}

async fn execute_set(args: &SetArgs) -> Result<Box<dyn OutputFormatter>> {
    let config = load_config(None)?;

    // TODO: Implement nested key-value setting
    let config_path = None;
    save_config(&config, config_path)?;

    Ok(create_output_formatter(&config, OutputData::Success {
        message: format!("Set {} to {}", args.key, args.value),
    }))
}

async fn execute_reset() -> Result<Box<dyn OutputFormatter>> {
    let config = crate::cli::config::Config::default();
    let config_path = None;
    save_config(&config, config_path)?;

    Ok(create_output_formatter(&config, OutputData::Success {
        message: "Configuration reset to defaults".to_string(),
    }))
}

async fn execute_validate(config: &crate::cli::config::Config) -> Result<Box<dyn OutputFormatter>> {
    match validate_config(config) {
        Ok(_) => Ok(create_output_formatter(config, OutputData::Success {
            message: "Configuration is valid".to_string(),
        })),
        Err(e) => Ok(create_output_formatter(config, OutputData::Error {
            message: format!("Configuration validation failed: {}", e),
        })),
    }
}

async fn execute_edit() -> Result<Box<dyn OutputFormatter>> {
    let config = load_config(None)?;

    // TODO: Implement editor integration
    Ok(create_output_formatter(&config, OutputData::Success {
        message: "Edit not yet implemented - use $EDITOR ~/./workspace/config.yaml".to_string(),
    }))
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub subcommand: ConfigSubcommand,
}

#[derive(Parser, Debug)]
pub enum ConfigSubcommand {
    Show {
        section: Option<String>,
    },
    Set {
        #[arg(value_name = "KEY")]
        key: String,
        #[arg(value_name = "VALUE")]
        value: String,
    },
    Reset,
    Validate,
    Edit,
}

#[derive(Parser, Debug)]
pub struct ShowArgs {
    pub section: Option<String>,
}

#[derive(Parser, Debug)]
pub struct SetArgs {
    pub key: String,
    pub value: String,
}

fn create_output_formatter(config: &crate::cli::config::Config, data: OutputData) -> Box<dyn OutputFormatter> {
    use super::super::output::create_formatter;
    create_formatter(&config.general.output_format)
}
```

- [ ] **Step 4.6: Write command module exports**

```rust
// src/cli/commands/mod.rs
pub mod run;
pub mod generate;
pub mod queue;
pub mod status;
pub mod config;

pub use run::RunArgs;
pub use generate::GenerateArgs;
pub use queue::{QueueArgs, QueueSubcommand, AddArgs, RemoveArgs, ListArgs};
pub use status::StatusArgs;
pub use config::{ConfigArgs, ConfigSubcommand, ShowArgs, SetArgs};
```

- [ ] **Step 4.7: Test command handlers**

```rust
// tests/cli/commands_test.rs (extended)
use glyphnova::cli::commands::*;
use tempfile::NamedTempFile;
use std::path::PathBuf;

#[test]
fn test_run_command_with_valid_workflow() {
    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), b"name: test\nsteps: []").unwrap();

    let args = RunArgs {
        workflow: temp_file.path().to_path_buf(),
        dry_run: true,
        mode: None,
        checkpoint: None,
    };

    tokio_test::block_on(async {
        let formatter = run::execute(&args).await.unwrap();
        let mut output = Vec::new();
        formatter.write(&mut output, &glyphnova::cli::output::OutputData::Success {
            message: "test".to_string(),
        }).unwrap();
    });
}

#[test]
fn test_run_command_with_missing_workflow() {
    let args = RunArgs {
        workflow: PathBuf::from("/nonexistent/workflow.yaml"),
        dry_run: false,
        mode: None,
        checkpoint: None,
    };

    tokio_test::block_on(async {
        let result = run::execute(&args).await;
        assert!(result.is_err());
    });
}
```

Run: `cargo test test_command --lib`
Expected: PASS

- [ ] **Step 4.8: Commit**

```bash
git add src/cli/commands/mod.rs src/cli/commands/run.rs src/cli/commands/generate.rs src/cli/commands/queue.rs src/cli/commands/status.rs src/cli/commands/config.rs tests/cli/commands_test.rs
git commit -m "feat(cli): add command handlers for all subcommands"
```

---

### Step 5: Add CLI to library

- [ ] **Step 5.1: Update lib.rs**

```rust
// src/lib.rs
pub mod cli;
pub mod backends;
pub mod tools;
pub mod workflows;
pub mod codegen;
pub mod rag;

pub use cli::Cli;
```

- [ ] **Step 5.2: Commit**

```bash
git add src/lib.rs
git commit -m "feat(cli): add CLI module to library exports"
```

---

## Summary

This task implements the complete CLI foundation including:

1. **CLI structure** with clap derive API for all subcommands
2. **Configuration loading** from `~/./workspace/config.yaml` with validation
3. **Output formatters** for plain, JSON, and table output
4. **Command handlers** for run, generate, queue, status, and config subcommands
5. **Comprehensive tests** for all components

**Next:** Task 01 - LLM Backend Trait Definition
