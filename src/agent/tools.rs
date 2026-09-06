use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, warn};
use crate::backend::llm_backend::{LlmBackend, ChatMessage};
use crate::model::registry::{ModelLifecycle, ThreadSafeModelRegistry};
use crate::agent::sandbox::{ToolSandbox, SandboxConfig, FileOperation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_name: String,
    pub output: String,
    pub success: bool,
    pub metadata: HashMap<String, String>,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> serde_json::Value;
    async fn execute(&self, arguments: HashMap<String, serde_json::Value>) -> Result<ToolResult, anyhow::Error>;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
    allowed_tools: Option<Vec<String>>,
    forbidden_tools: Vec<String>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            allowed_tools: None,
            forbidden_tools: Vec::new(),
        }
    }

    pub fn with_allowed_tools(mut self, tools: Vec<String>) -> Self {
        self.allowed_tools = if tools.is_empty() { None } else { Some(tools) };
        self
    }

    pub fn with_forbidden_tools(mut self, tools: Vec<String>) -> Self {
        self.forbidden_tools = tools;
        self
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        let name = tool.name().to_string();
        debug!("Registering tool: {}", name);
        self.tools.insert(name, tool);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    pub fn list(&self) -> Vec<(&str, &str)> {
        self.tools
            .iter()
            .filter(|(name, _)| self.is_tool_allowed(name))
            .map(|(name, tool)| (name.as_str(), tool.description()))
            .collect()
    }

    pub async fn execute(&self, call: ToolCall) -> Result<ToolResult, anyhow::Error> {
        debug!("Executing tool: {} with arguments: {:?}", call.name, call.arguments);

        if !self.is_tool_allowed(&call.name) {
            anyhow::bail!("Tool '{}' is not allowed", call.name);
        }

        match self.get(&call.name) {
            Some(tool) => tool.execute(call.arguments).await,
            None => anyhow::bail!("Tool not found: {}", call.name),
        }
    }

    fn is_tool_allowed(&self, name: &str) -> bool {
        if self.forbidden_tools.iter().any(|f| f == name) {
            return false;
        }
        if let Some(allowed) = &self.allowed_tools {
            return allowed.iter().any(|a| a == name);
        }
        true
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// 1. ModelListTool
pub struct ModelListTool {
    registry: Arc<Mutex<crate::model::registry::ThreadSafeModelRegistry>>,
}

impl ModelListTool {
    pub fn new(registry: Arc<Mutex<crate::model::registry::ThreadSafeModelRegistry>>) -> Self {
        Self { registry }
    }
}

#[async_trait::async_trait]
impl Tool for ModelListTool {
    fn name(&self) -> &str {
        "model_list"
    }

    fn description(&self) -> &str {
        "Returns a list of available models and their lifecycle state"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {},
            "required": []
        })
    }

    async fn execute(&self, _arguments: HashMap<String, serde_json::Value>) -> Result<ToolResult, anyhow::Error> {
        debug!("Executing ModelListTool");
        let registry = self.registry.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;

        let models = registry.list_models();
        let output = serde_json::to_string_pretty(&models)?;
        Ok(ToolResult {
            tool_name: "model_list".to_string(),
            output,
            success: true,
            metadata: HashMap::new(),
        })
    }
}

// 2. ModelLoadTool
pub struct ModelLoadTool {
    registry: Arc<Mutex<ThreadSafeModelRegistry>>,
    backend: Arc<dyn LlmBackend>,
}

impl ModelLoadTool {
    pub fn new(registry: Arc<Mutex<ThreadSafeModelRegistry>>, backend: Arc<dyn LlmBackend>) -> Self {
        Self { registry, backend }
    }
}

#[async_trait::async_trait]
impl Tool for ModelLoadTool {
    fn name(&self) -> &str {
        "model_load"
    }

    fn description(&self) -> &str {
        "Loads a model by name into memory"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "model_name": {
                    "type": "string",
                    "description": "Name of the model to load"
                }
            },
            "required": ["model_name"]
        })
    }

    async fn execute(&self, arguments: HashMap<String, serde_json::Value>) -> Result<ToolResult, anyhow::Error> {
        let model_name = arguments
            .get("model_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid model_name argument"))?;

        debug!("Executing ModelLoadTool for model: {}", model_name);

        match self.backend.load_model(model_name).await {
            Ok(_) => {
                if let Err(e) = self.registry
                    .lock()
                    .map_err(|e| anyhow::anyhow!("Lock error: {}", e))
                    .and_then(|r| r.set_state(model_name, ModelLifecycle::Loaded).map_err(|e| anyhow::anyhow!("{}", e)))
                {
                    warn!("[tools] registry state update failed for {}: {}", model_name, e);
                }

                Ok(ToolResult {
                    tool_name: "model_load".to_string(),
                    output: format!("Model '{}' loaded successfully", model_name),
                    success: true,
                    metadata: HashMap::new(),
                })
            }
            Err(e) => {
                if let Err(e2) = self.registry
                    .lock()
                    .map_err(|e| anyhow::anyhow!("Lock error: {}", e))
                    .and_then(|r| r.set_state(
                        model_name,
                        ModelLifecycle::Error(e.to_string()),
                    ).map_err(|e| anyhow::anyhow!("{}", e)))
                {
                    warn!("[tools] registry state update failed for {}: {}", model_name, e2);
                }

                Ok(ToolResult {
                    tool_name: "model_load".to_string(),
                    output: format!("Failed to load model '{}': {}", model_name, e),
                    success: false,
                    metadata: {
                        let mut meta = HashMap::new();
                        meta.insert("error".to_string(), e.to_string());
                        meta
                    },
                })
            }
        }
    }
}

// 3. ModelUnloadTool
pub struct ModelUnloadTool {
    registry: Arc<Mutex<ThreadSafeModelRegistry>>,
    backend: Arc<dyn LlmBackend>,
}

impl ModelUnloadTool {
    pub fn new(registry: Arc<Mutex<ThreadSafeModelRegistry>>, backend: Arc<dyn LlmBackend>) -> Self {
        Self { registry, backend }
    }
}

#[async_trait::async_trait]
impl Tool for ModelUnloadTool {
    fn name(&self) -> &str {
        "model_unload"
    }

    fn description(&self) -> &str {
        "Unloads a model by name from memory"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "model_name": {
                    "type": "string",
                    "description": "Name of the model to unload"
                }
            },
            "required": ["model_name"]
        })
    }

    async fn execute(&self, arguments: HashMap<String, serde_json::Value>) -> Result<ToolResult, anyhow::Error> {
        let model_name = arguments
            .get("model_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid model_name argument"))?;

        debug!("Executing ModelUnloadTool for model: {}", model_name);

        match self.backend.unload_model(model_name).await {
            Ok(_) => {
                if let Err(e) = self.registry
                    .lock()
                    .map_err(|e| anyhow::anyhow!("Lock error: {}", e))
                    .and_then(|r| r.set_state(model_name, ModelLifecycle::Unloaded).map_err(|e| anyhow::anyhow!("{}", e)))
                {
                    warn!("[tools] registry state update failed for {}: {}", model_name, e);
                }

                Ok(ToolResult {
                    tool_name: "model_unload".to_string(),
                    output: format!("Model '{}' unloaded successfully", model_name),
                    success: true,
                    metadata: HashMap::new(),
                })
            }
            Err(e) => Ok(ToolResult {
                tool_name: "model_unload".to_string(),
                output: format!("Failed to unload model '{}': {}", model_name, e),
                success: false,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("error".to_string(), e.to_string());
                    meta
                },
            }),
        }
    }
}

// 4. ChatTool
pub struct ChatTool {
    backend: Arc<dyn LlmBackend>,
}

impl ChatTool {
    pub fn new(backend: Arc<dyn LlmBackend>) -> Self {
        Self { backend }
    }
}

#[async_trait::async_trait]
impl Tool for ChatTool {
    fn name(&self) -> &str {
        "chat"
    }

    fn description(&self) -> &str {
        "Sends a chat message to a loaded model and returns the response"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "model_name": {
                    "type": "string",
                    "description": "Name of the model to chat with"
                },
                "message": {
                    "type": "string",
                    "description": "The message to send"
                },
                "temperature": {
                    "type": "number",
                    "description": "Optional temperature for sampling"
                }
            },
            "required": ["model_name", "message"]
        })
    }

    async fn execute(&self, arguments: HashMap<String, serde_json::Value>) -> Result<ToolResult, anyhow::Error> {
        let model_name = arguments
            .get("model_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid model_name argument"))?;

        let message = arguments
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid message argument"))?;

        debug!("Executing ChatTool for model: {}", model_name);

        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: message.to_string(),
        }];

        match self.backend.chat(messages, model_name).await {
            Ok(response) => Ok(ToolResult {
                tool_name: "chat".to_string(),
                output: response.content,
                success: true,
                metadata: HashMap::new(),
            }),
            Err(e) => Ok(ToolResult {
                tool_name: "chat".to_string(),
                output: format!("Chat failed: {}", e),
                success: false,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("error".to_string(), e.to_string());
                    meta
                },
            }),
        }
    }
}

// 5. FileReadTool
pub struct FileReadTool {
    sandbox: ToolSandbox,
}

impl FileReadTool {
    pub fn new(config: SandboxConfig) -> Self {
        let sandbox = ToolSandbox::new(config);
        Self { sandbox }
    }
}

#[async_trait::async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str {
        "file_read"
    }

    fn description(&self) -> &str {
        "Reads a file from the workspace, enforcing allowed paths restrictions"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
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

    async fn execute(&self, arguments: HashMap<String, serde_json::Value>) -> Result<ToolResult, anyhow::Error> {
        let path_str = arguments
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid path argument"))?;

        let path = std::path::PathBuf::from(path_str);
        debug!("Executing FileReadTool for path: {}", path_str);

        if let Err(e) = self.sandbox.validate_file_access(&path, FileOperation::Read) {
            return Ok(ToolResult {
                tool_name: "file_read".to_string(),
                output: format!("Access denied: {}", e),
                success: false,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("error".to_string(), e.to_string());
                    meta
                },
            });
        }

        match tokio::fs::read_to_string(&path).await {
            Ok(content) => Ok(ToolResult {
                tool_name: "file_read".to_string(),
                output: content,
                success: true,
                metadata: HashMap::new(),
            }),
            Err(e) => Ok(ToolResult {
                tool_name: "file_read".to_string(),
                output: format!("Failed to read file '{}': {}", path_str, e),
                success: false,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("error".to_string(), e.to_string());
                    meta
                },
            }),
        }
    }
}

// 6. FinalAnswerTool
pub struct FinalAnswerTool;

impl FinalAnswerTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FinalAnswerTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Tool for FinalAnswerTool {
    fn name(&self) -> &str {
        "final_answer"
    }

    fn description(&self) -> &str {
        "Signals completion of the ReAct loop with a final answer"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "answer": {
                    "type": "string",
                    "description": "The final answer to return"
                }
            },
            "required": ["answer"]
        })
    }

    async fn execute(&self, arguments: HashMap<String, serde_json::Value>) -> Result<ToolResult, anyhow::Error> {
        let answer = arguments
            .get("answer")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid answer argument"))?;

        debug!("Executing FinalAnswerTool with answer length: {}", answer.len());

        Ok(ToolResult {
            tool_name: "final_answer".to_string(),
            output: answer.to_string(),
            success: true,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("type".to_string(), "final_answer".to_string());
                meta
            },
        })
    }
}

pub struct ToolExecutor {
    registry: ToolRegistry,
    sandbox_config: SandboxConfig,
}

impl ToolExecutor {
    pub fn new(registry: ToolRegistry, sandbox_config: SandboxConfig) -> Self {
        Self {
            registry,
            sandbox_config,
        }
    }

    pub async fn execute_tool(&self, tool_name: &str, args: HashMap<String, serde_json::Value>) -> Result<ToolResult, anyhow::Error> {
        let tool = self.registry.get(tool_name)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", tool_name))?;

        if tool_name == "file_read" {
            if let Some(path_value) = args.get("path") {
                if let Some(path_str) = path_value.as_str() {
                    let path = std::path::PathBuf::from(path_str);
                    debug!("Checking sandbox permissions for path: {}", path_str);

                    match ToolSandbox::new(self.sandbox_config.clone()).is_path_allowed(&path) {
                        Ok(allowed) => {
                            if !allowed {
                                debug!("Sandbox denied access to path: {}", path_str);
                                return Ok(ToolResult {
                                    tool_name: tool_name.to_string(),
                                    output: format!("Access denied by sandbox: path '{}' is not allowed", path_str),
                                    success: false,
                                    metadata: {
                                        let mut meta = HashMap::new();
                                        meta.insert("error".to_string(), "Sandbox access denied".to_string());
                                        meta
                                    },
                                });
                            }
                            debug!("Sandbox allowed access to path: {}", path_str);
                        }
                        Err(e) => {
                            debug!("Sandbox validation error for path {}: {}", path_str, e);
                            return Ok(ToolResult {
                                tool_name: tool_name.to_string(),
                                output: format!("Sandbox validation error: {}", e),
                                success: false,
                                metadata: {
                                    let mut meta = HashMap::new();
                                    meta.insert("error".to_string(), e.to_string());
                                    meta
                                },
                            });
                        }
                    }
                }
            }
        }

        tool.execute(args).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTool {
        name: String,
        description: String,
        fixed_output: String,
    }

    impl MockTool {
        fn new(name: &str, description: &str, fixed_output: &str) -> Self {
            Self {
                name: name.to_string(),
                description: description.to_string(),
                fixed_output: fixed_output.to_string(),
            }
        }
    }

    #[async_trait::async_trait]
    impl Tool for MockTool {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            &self.description
        }

        fn parameters_schema(&self) -> serde_json::Value {
            serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            })
        }

        async fn execute(&self, _arguments: HashMap<String, serde_json::Value>) -> Result<ToolResult, anyhow::Error> {
            Ok(ToolResult {
                tool_name: self.name.clone(),
                output: self.fixed_output.clone(),
                success: true,
                metadata: HashMap::new(),
            })
        }
    }

    #[tokio::test]
    async fn test_registry_no_filtering_allows_all() -> anyhow::Result<()> {
        let mut registry = ToolRegistry::new();
        registry.register(Box::new(MockTool::new("tool_a", "Tool A", "A output")));
        registry.register(Box::new(MockTool::new("tool_b", "Tool B", "B output")));

        // Both tools should execute
        let result_a = registry
            .execute(ToolCall {
                name: "tool_a".to_string(),
                arguments: HashMap::new(),
            })
            .await?;
        assert_eq!(result_a.tool_name, "tool_a");

        let result_b = registry
            .execute(ToolCall {
                name: "tool_b".to_string(),
                arguments: HashMap::new(),
            })
            .await?;
        assert_eq!(result_b.tool_name, "tool_b");

        Ok(())
    }

    #[tokio::test]
    async fn test_registry_allowed_tools_filters_execution() -> anyhow::Result<()> {
        let mut registry = ToolRegistry::new()
            .with_allowed_tools(vec!["tool_a".to_string()]);
        registry.register(Box::new(MockTool::new("tool_a", "Tool A", "A output")));
        registry.register(Box::new(MockTool::new("tool_b", "Tool B", "B output")));

        // tool_a should execute
        let result_a = registry
            .execute(ToolCall {
                name: "tool_a".to_string(),
                arguments: HashMap::new(),
            })
            .await?;
        assert_eq!(result_a.tool_name, "tool_a");

        // tool_b should be denied
        let result_b = registry
            .execute(ToolCall {
                name: "tool_b".to_string(),
                arguments: HashMap::new(),
            })
            .await;
        assert!(result_b.is_err());
        assert!(result_b.unwrap_err().to_string().contains("not allowed"));

        Ok(())
    }

    #[tokio::test]
    async fn test_registry_forbidden_tools_takes_precedence() -> anyhow::Result<()> {
        let mut registry = ToolRegistry::new()
            .with_allowed_tools(vec!["tool_a".to_string(), "tool_b".to_string()])
            .with_forbidden_tools(vec!["tool_a".to_string()]);
        registry.register(Box::new(MockTool::new("tool_a", "Tool A", "A output")));
        registry.register(Box::new(MockTool::new("tool_b", "Tool B", "B output")));

        // tool_a should be denied (forbidden wins over allowed)
        let result_a = registry
            .execute(ToolCall {
                name: "tool_a".to_string(),
                arguments: HashMap::new(),
            })
            .await;
        assert!(result_a.is_err());
        assert!(result_a.unwrap_err().to_string().contains("not allowed"));

        // tool_b should execute
        let result_b = registry
            .execute(ToolCall {
                name: "tool_b".to_string(),
                arguments: HashMap::new(),
            })
            .await?;
        assert_eq!(result_b.tool_name, "tool_b");

        Ok(())
    }

    #[tokio::test]
    async fn test_registry_list_filters_by_allowed() {
        let registry = ToolRegistry::new()
            .with_allowed_tools(vec!["tool_a".to_string()]);
        let mut registry = registry;
        registry.register(Box::new(MockTool::new("tool_a", "Tool A", "A output")));
        registry.register(Box::new(MockTool::new("tool_b", "Tool B", "B output")));

        let list = registry.list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].0, "tool_a");
    }

    #[tokio::test]
    async fn test_registry_list_excludes_forbidden() {
        let registry = ToolRegistry::new()
            .with_forbidden_tools(vec!["tool_b".to_string()]);
        let mut registry = registry;
        registry.register(Box::new(MockTool::new("tool_a", "Tool A", "A output")));
        registry.register(Box::new(MockTool::new("tool_b", "Tool B", "B output")));

        let list = registry.list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].0, "tool_a");
    }

    #[tokio::test]
    async fn test_registry_allowed_empty_means_all() -> anyhow::Result<()> {
        // Empty vec should map to None (allow all)
        let mut registry = ToolRegistry::new()
            .with_allowed_tools(vec![]);
        registry.register(Box::new(MockTool::new("tool_a", "Tool A", "A output")));
        registry.register(Box::new(MockTool::new("tool_b", "Tool B", "B output")));

        // Both tools should execute
        let result_a = registry
            .execute(ToolCall {
                name: "tool_a".to_string(),
                arguments: HashMap::new(),
            })
            .await?;
        assert_eq!(result_a.tool_name, "tool_a");

        let result_b = registry
            .execute(ToolCall {
                name: "tool_b".to_string(),
                arguments: HashMap::new(),
            })
            .await?;
        assert_eq!(result_b.tool_name, "tool_b");

        Ok(())
    }

    #[tokio::test]
    async fn test_registry_execute_blocked_returns_error() -> anyhow::Result<()> {
        let mut registry = ToolRegistry::new()
            .with_forbidden_tools(vec!["tool_a".to_string()]);
        registry.register(Box::new(MockTool::new("tool_a", "Tool A", "A output")));

        let result = registry
            .execute(ToolCall {
                name: "tool_a".to_string(),
                arguments: HashMap::new(),
            })
            .await;

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("not allowed"));
        assert!(error_msg.contains("tool_a"));

        Ok(())
    }
}
