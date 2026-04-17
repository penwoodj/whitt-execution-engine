# Task 08: Tool Execution Framework

**Files:**
- Create: `src/tools/trait.rs`
- Create: `src/tools/builtin/mod.rs`
- Create: `src/tools/builtin/file.rs`
- Create: `src/tools/builtin/web.rs`
- Create: `src/tools/builtin/shell.rs`
- Create: `src/tools/registry.rs`
- Create: `src/tools/execution.rs`
- Modify: `src/tools/mod.rs` (add new modules)
- Test: `tests/tools/execution_test.rs`

---

## Overview

Implement tool execution framework with trait-based tools, built-in tools (file, web, shell), custom tool registration, and execution engine with permission checks.

---

## Implementation Steps

### Step 1: Define tool trait

- [ ] **Step 1.1: Write tool trait**

```rust
// src/tools/trait.rs
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn parameters_schema(&self) -> serde_json::Value;

    async fn execute(&self, input: ToolInput) -> Result<ToolOutput>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInput {
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub result: serde_json::Value,
    pub status: ToolStatus,
    pub metadata: Option<ToolMetadata>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ToolStatus {
    Success,
    Error,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub execution_time_ms: Option<u64>,
    pub error_message: Option<String>,
    pub additional_info: Option<serde_json::Value>,
}

impl ToolOutput {
    pub fn success(result: serde_json::Value) -> Self {
        Self {
            result,
            status: ToolStatus::Success,
            metadata: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            result: serde_json::Value::Null,
            status: ToolStatus::Error,
            metadata: Some(ToolMetadata {
                execution_time_ms: None,
                error_message: Some(message),
                additional_info: None,
            }),
        }
    }
}
```

- [ ] **Step 1.2: Commit**

```bash
git add src/tools/trait.rs
git commit -m "feat(tools): add Tool trait and types"
```

---

### Step 2: Implement built-in tools

- [ ] **Step 2.1: Write file tools**

```rust
// src/tools/builtin/file.rs
use super::super::trait::{Tool, ToolInput, ToolOutput};
use async_trait::async_trait;
use serde_json::json;
use std::fs;
use std::path::Path;

pub struct FileReadTool;
pub struct FileWriteTool;
pub struct FileDeleteTool;

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &'static str {
        "file.read"
    }

    fn description(&self) -> &'static str {
        "Read the contents of a file"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to read"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: ToolInput) -> anyhow::Result<ToolOutput> {
        let path = input.params["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' parameter"))?;

        let content = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))?;

        Ok(ToolOutput::success(json!({
            "path": path,
            "content": content
        })))
    }
}

#[async_trait]
impl Tool for FileWriteTool {
    fn name(&self) -> &'static str {
        "file.write"
    }

    fn description(&self) -> &'static str {
        "Write content to a file"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write"
                }
            },
            "required": ["path", "content"]
        })
    }

    async fn execute(&self, input: ToolInput) -> anyhow::Result<ToolOutput> {
        let path = input.params["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' parameter"))?;

        let content = input.params["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'content' parameter"))?;

        // Create parent directories if they don't exist
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| anyhow::anyhow!("Failed to create directory: {}", e))?;
        }

        fs::write(path, content)
            .map_err(|e| anyhow::anyhow!("Failed to write file: {}", e))?;

        Ok(ToolOutput::success(json!({
            "path": path,
            "bytes_written": content.len()
        })))
    }
}

#[async_trait]
impl Tool for FileDeleteTool {
    fn name(&self) -> &'static str {
        "file.delete"
    }

    fn description(&self) -> &'static str {
        "Delete a file"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to delete"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: ToolInput) -> anyhow::Result<ToolOutput> {
        let path = input.params["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' parameter"))?;

        fs::remove_file(path)
            .map_err(|e| anyhow::anyhow!("Failed to delete file: {}", e))?;

        Ok(ToolOutput::success(json!({
            "path": path,
            "deleted": true
        })))
    }
}
```

- [ ] **Step 2.2: Write web tools**

```rust
// src/tools/builtin/web.rs
use super::super::trait::{Tool, ToolInput, ToolOutput};
use async_trait::async_trait;
use serde_json::json;
use reqwest::Client;

pub struct WebFetchTool;
pub struct WebScrapeTool;

#[async_trait]
impl Tool for WebFetchTool {
    fn name(&self) -> &'static str {
        "web.fetch"
    }

    fn description(&self) -> &'static str {
        "Fetch content from a URL"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "URL to fetch"
                }
            },
            "required": ["url"]
        })
    }

    async fn execute(&self, input: ToolInput) -> anyhow::Result<ToolOutput> {
        let url = input.params["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'url' parameter"))?;

        let client = Client::new();
        let response = client.get(url).send().await
            .map_err(|e| anyhow::anyhow!("Failed to fetch URL: {}", e))?;

        let content = response.text().await
            .map_err(|e| anyhow::anyhow!("Failed to read response: {}", e))?;

        Ok(ToolOutput::success(json!({
            "url": url,
            "content": content
        })))
    }
}

#[async_trait]
impl Tool for WebScrapeTool {
    fn name(&self) -> &'static str {
        "web.scrape"
    }

    fn description(&self) -> &'static str {
        "Scrape structured content from a URL"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "URL to scrape"
                }
            },
            "required": ["url"]
        })
    }

    async fn execute(&self, input: ToolInput) -> anyhow::Result<ToolOutput> {
        let url = input.params["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'url' parameter"))?;

        let client = Client::new();
        let response = client.get(url).send().await
            .map_err(|e| anyhow::anyhow!("Failed to fetch URL: {}", e))?;

        let html = response.text().await
            .map_err(|e| anyhow::anyhow!("Failed to read response: {}", e))?;

        // Basic scraping: extract text content
        let text = html
            .split(|c| c == '<')
            .filter_map(|s| s.split('>').next())
            .collect::<Vec<_>>()
            .join(" ");

        Ok(ToolOutput::success(json!({
            "url": url,
            "text": text
        })))
    }
}
```

- [ ] **Step 2.3: Write shell tools**

```rust
// src/tools/builtin/shell.rs
use super::super::trait::{Tool, ToolInput, ToolOutput};
use async_trait::async_trait;
use serde_json::json;
use std::process::Command;

pub struct ShellExecTool;

#[async_trait]
impl Tool for ShellExecTool {
    fn name(&self) -> &'static str {
        "shell.exec"
    }

    fn description(&self) -> &'static str {
        "Execute a shell command"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "Command to execute"
                },
                "args": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Arguments for the command"
                }
            },
            "required": ["command"]
        })
    }

    async fn execute(&self, input: ToolInput) -> anyhow::Result<ToolOutput> {
        let command = input.params["command"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'command' parameter"))?;

        let args: Vec<String> = input.params["args"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let output = Command::new(command)
            .args(&args)
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok(ToolOutput::success(json!({
            "command": command,
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr
        })))
    }
}
```

- [ ] **Step 2.4: Write builtin module**

```rust
// src/tools/builtin/mod.rs
pub mod file;
pub mod web;
pub mod shell;

pub use file::{FileReadTool, FileWriteTool, FileDeleteTool};
pub use web::{WebFetchTool, WebScrapeTool};
pub use shell::ShellExecTool;
```

- [ ] **Step 2.5: Commit**

```bash
git add src/tools/builtin/mod.rs src/tools/builtin/file.rs src/tools/builtin/web.rs src/tools/builtin/shell.rs
git commit -m "feat(tools): add built-in tools (file, web, shell)"
```

---

### Step 3: Create tool registry

- [ ] **Step 3.1: Write tool registry**

```rust
// src/tools/registry.rs
use super::trait::Tool;
use super::builtin::*;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };
        registry.register_builtins();
        registry
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    pub fn get_tool_info(&self, name: &str) -> Option<ToolInfo> {
        self.tools.get(name).map(|tool| ToolInfo {
            name: tool.name().to_string(),
            description: tool.description().to_string(),
            parameters_schema: tool.parameters_schema(),
        })
    }

    pub fn list_tools_info(&self) -> Vec<ToolInfo> {
        self.tools.values()
            .map(|tool| ToolInfo {
                name: tool.name().to_string(),
                description: tool.description().to_string(),
                parameters_schema: tool.parameters_schema(),
            })
            .collect()
    }

    fn register_builtins(&mut self) {
        self.register(Arc::new(FileReadTool));
        self.register(Arc::new(FileWriteTool));
        self.register(Arc::new(FileDeleteTool));
        self.register(Arc::new(WebFetchTool));
        self.register(Arc::new(WebScrapeTool));
        self.register(Arc::new(ShellExecTool));
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub parameters_schema: serde_json::Value,
}
```

- [ ] **Step 3.2: Commit**

```bash
git add src/tools/registry.rs
git commit -m "feat(tools): add tool registry with built-in tools"
```

---

### Step 4: Create execution engine

- [ ] **Step 4.1: Write execution engine**

```rust
// src/tools/execution.rs
use super::trait::{Tool, ToolInput, ToolOutput};
use super::registry::ToolRegistry;
use super::permissions::{PermissionManager, ToolEvaluation};
use std::sync::Arc;

pub struct ToolExecutor {
    registry: Arc<ToolRegistry>,
    permission_manager: PermissionManager,
}

impl ToolExecutor {
    pub fn new(
        registry: Arc<ToolRegistry>,
        permission_manager: PermissionManager,
    ) -> Self {
        Self {
            registry,
            permission_manager,
        }
    }

    pub async fn execute(&self, tool_name: &str, input: ToolInput) -> anyhow::Result<ToolOutput> {
        // Get tool from registry
        let tool = self.registry.get(tool_name)
            .ok_or_else(|| anyhow::anyhow!("Tool '{}' not found", tool_name))?;

        // Evaluate permissions
        let input_str = serde_json::to_string(&input.params)
            .unwrap_or_default();

        let evaluation = self.permission_manager.evaluate_tool_request(
            tool_name,
            &input_str
        )?;

        if !evaluation.allowed {
            return Ok(ToolOutput::error(
                evaluation.reason.unwrap_or_else(|| "Permission denied".to_string())
            ));
        }

        // Request confirmation if required
        if evaluation.requires_confirmation {
            let details = format!("Tool: {}, Input: {}", tool_name, input_str);
            let approved = self.permission_manager.request_confirmation(tool_name, &details)?;

            if !approved {
                return Ok(ToolOutput::error("Execution cancelled by user".to_string()));
            }
        }

        // Execute tool
        let start = std::time::Instant::now();
        let result = tool.execute(input).await;
        let execution_time = start.elapsed().as_millis() as u64;

        match result {
            Ok(mut output) => {
                if let Some(metadata) = &mut output.metadata {
                    metadata.execution_time_ms = Some(execution_time);
                }
                Ok(output)
            }
            Err(e) => Ok(ToolOutput::error(e.to_string())),
        }
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.registry.list_tools()
    }

    pub fn get_tool_info(&self, name: &str) -> Option<super::registry::ToolInfo> {
        self.registry.get_tool_info(name)
    }
}
```

- [ ] **Step 4.2: Update tools module**

```rust
// src/tools/mod.rs
pub mod trait;
pub mod builtin;
pub mod permissions;
pub mod registry;
pub mod execution;

pub use trait::{Tool, ToolInput, ToolOutput, ToolStatus};
pub use permissions::*;
pub use registry::{ToolRegistry, ToolInfo};
pub use execution::ToolExecutor;
```

- [ ] **Step 4.3: Commit**

```bash
git add src/tools/execution.rs src/tools/mod.rs
git commit -m "feat(tools): add tool execution engine with permission checks"
```

---

### Step 5: Write tests

- [ ] **Step 5.1: Write integration tests**

```rust
// tests/tools/execution_test.rs
use whitt_execution_engine::tools::{
    ToolRegistry, ToolExecutor, PermissionManager, PermissionConfig, Policy,
    ToolInput, builtin::FileReadTool,
};
use std::sync::Arc;
use tempfile::NamedTempFile;
use std::fs;

#[tokio::test]
async fn test_tool_execution() {
    let mut config = PermissionConfig::default();
    config.default_policy = Policy::Allow;

    let registry = Arc::new(ToolRegistry::new());
    let permission_manager = PermissionManager::new(config, false);
    let executor = ToolExecutor::new(registry.clone(), permission_manager);

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_str().unwrap();

    let input = ToolInput {
        params: serde_json::json!({ "path": path }),
    };

    let output = executor.execute("file.read", input).await.unwrap();

    assert_eq!(output.status, whitt_execution_engine::tools::ToolStatus::Success);
}

#[tokio::test]
async fn test_permission_denied() {
    let mut config = PermissionConfig::default();
    config.default_policy = Policy::Deny;

    let registry = Arc::new(ToolRegistry::new());
    let permission_manager = PermissionManager::new(config, false);
    let executor = ToolExecutor::new(registry.clone(), permission_manager);

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_str().unwrap();

    let input = ToolInput {
        params: serde_json::json!({ "path": path }),
    };

    let output = executor.execute("file.read", input).await.unwrap();

    assert_eq!(output.status, whitt_execution_engine::tools::ToolStatus::Error);
    assert!(output.metadata.as_ref().unwrap().error_message.is_some());
}
```

- [ ] **Step 5.2: Commit**

```bash
git add tests/tools/execution_test.rs
git commit -m "test(tools): add tool execution integration tests"
```

---

## Summary

This task implements the complete tool execution framework including:

1. **Tool trait** defining the interface for all tools
2. **Built-in tools**: file (read/write/delete), web (fetch/scrape), shell (exec)
3. **Tool registry** for discovering and managing tools
4. **Execution engine** with permission checks and confirmation flow
5. **Comprehensive tests** for tool execution and permissions

**Built-in Tools:**
- `file.read`: Read file contents
- `file.write`: Write content to file
- `file.delete`: Delete file
- `web.fetch`: Fetch URL content
- `web.scrape`: Scrape structured web content
- `shell.exec`: Execute shell command

**Security:**
- All tool executions go through permission manager
- Guardrails enforced before execution
- Confirmation flow for dangerous tools
- Execution time tracking in metadata

**Next:** Task 09 - Sub-Workflow Execution
