use serde::{Deserialize, Serialize};

/// Tool permissions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ToolPermissions {
    #[serde(default)]
    pub file_operations: Option<FileOperations>,
    #[serde(default)]
    pub web_operations: Option<WebOperations>,
    #[serde(default)]
    pub shell_operations: Option<ShellOperations>,
    #[serde(default)]
    pub content_operations: Option<ContentOperations>,
    #[serde(default)]
    pub system_operations: Option<SystemOperations>,
}

/// File operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct FileOperations {
    #[serde(default)]
    pub read: Option<FileReadConfig>,
    #[serde(default)]
    pub write: Option<FileWriteConfig>,
    #[serde(default)]
    pub delete: Option<FileDeleteConfig>,
}

/// File read configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct FileReadConfig {
    #[serde(default)]
    pub require_confirmation: Option<bool>,
    #[serde(default)]
    pub allowed_paths: Option<Vec<String>>,
    #[serde(default)]
    pub allowed_patterns: Option<Vec<String>>,
    #[serde(default)]
    pub forbidden_paths: Option<Vec<String>>,
    #[serde(default)]
    pub max_file_size_mb: Option<u32>,
}

/// File write configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct FileWriteConfig {
    #[serde(default)]
    pub require_confirmation: Option<bool>,
    #[serde(default)]
    pub allowed_paths: Option<Vec<String>>,
    #[serde(default)]
    pub backup_existing: Option<bool>,
    #[serde(default)]
    pub create_parent_directories: Option<bool>,
}

/// File delete configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct FileDeleteConfig {
    #[serde(default)]
    pub require_confirmation: Option<bool>,
    #[serde(default)]
    pub allowed_paths: Option<Vec<String>>,
}

/// Web operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct WebOperations {
    #[serde(default)]
    pub fetch: Option<WebFetchConfig>,
    #[serde(default)]
    pub scrape: Option<WebScrapeConfig>,
}

/// Web fetch configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct WebFetchConfig {
    #[serde(default)]
    pub max_concurrent_requests: Option<u32>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    #[serde(default)]
    pub allowed_domains: Option<Vec<String>>,
    #[serde(default)]
    pub forbidden_domains: Option<Vec<String>>,
    #[serde(default)]
    pub rate_limits: Option<RateLimits>,
    #[serde(default)]
    pub rate_limits_file: Option<String>,
}

/// Web scrape configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct WebScrapeConfig {
    #[serde(default)]
    pub respect_robots_txt: Option<bool>,
    #[serde(default)]
    pub max_pages_per_domain: Option<u32>,
}

/// Rate limits.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct RateLimits {
    #[serde(default)]
    pub max_per_hour: Option<u32>,
    #[serde(default)]
    pub max_per_minute: Option<u32>,
}

/// Shell operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ShellOperations {
    #[serde(default)]
    pub exec: Option<ShellExecConfig>,
}

/// Shell exec configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ShellExecConfig {
    #[serde(default)]
    pub require_confirmation: Option<bool>,
    #[serde(default)]
    pub timeout_seconds: Option<u64>,
    #[serde(default)]
    pub allowed_commands: Option<Vec<String>>,
    #[serde(default)]
    pub forbidden_commands: Option<Vec<String>>,
    #[serde(default)]
    pub working_directories: Option<Vec<String>>,
}

/// Content operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ContentOperations {
    #[serde(default)]
    pub generate: Option<ContentGenerateConfig>,
    #[serde(default)]
    pub web_search: Option<WebSearchConfig>,
}

/// Content generate configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ContentGenerateConfig {
    #[serde(default)]
    pub rate_limits: Option<RateLimits>,
}

/// Web search configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct WebSearchConfig {
    #[serde(default)]
    pub rate_limits: Option<RateLimits>,
    #[serde(default)]
    pub rate_limits_file: Option<String>,
}

/// System operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct SystemOperations {
    #[serde(default)]
    pub process_management: Option<ProcessManagementConfig>,
    #[serde(default)]
    pub network_operations: Option<NetworkOperationsConfig>,
    #[serde(default)]
    pub file_system_mount: Option<FileSystemMountConfig>,
    #[serde(default)]
    pub environment_variables: Option<EnvironmentVariablesConfig>,
    #[serde(default)]
    pub service_management: Option<ServiceManagementConfig>,
}

/// Process management configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ProcessManagementConfig {
    #[serde(default)]
    pub disabled: Option<bool>,
    #[serde(default)]
    pub allowed_signals: Option<Vec<String>>,
    #[serde(default)]
    pub max_cpu_percent: Option<u32>,
    #[serde(default)]
    pub max_memory_mb: Option<u32>,
    #[serde(default)]
    pub monitored_processes: Option<Vec<String>>,
}

/// Network operations configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct NetworkOperationsConfig {
    #[serde(default)]
    pub disabled: Option<bool>,
    #[serde(default)]
    pub allowed_ports: Option<Vec<u16>>,
    #[serde(default)]
    pub allowed_protocols: Option<Vec<String>>,
    #[serde(default)]
    pub max_bandwidth_mbps: Option<u32>,
    #[serde(default)]
    pub blocked_hosts: Option<Vec<String>>,
}

/// File system mount configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct FileSystemMountConfig {
    #[serde(default)]
    pub disabled: Option<bool>,
    #[serde(default)]
    pub allowed_mount_points: Option<Vec<String>>,
    #[serde(default)]
    pub mount_options: Option<Vec<String>>,
}

/// Environment variables configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentVariablesConfig {
    #[serde(default)]
    pub read: Option<EnvVarReadConfig>,
    #[serde(default)]
    pub write: Option<EnvVarWriteConfig>,
}

/// Environment variable read configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EnvVarReadConfig {
    #[serde(default)]
    pub allowed_patterns: Option<Vec<String>>,
}

/// Environment variable write configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EnvVarWriteConfig {
    #[serde(default)]
    pub disabled: Option<bool>,
}

/// Service management configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ServiceManagementConfig {
    #[serde(default)]
    pub disabled: Option<bool>,
    #[serde(default)]
    pub allowed_services: Option<Vec<String>>,
    #[serde(default)]
    pub allowed_actions: Option<Vec<String>>,
}
