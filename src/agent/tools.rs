use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::debug;

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
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
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
            .map(|(name, tool)| (name.as_str(), tool.description()))
            .collect()
    }

    pub async fn execute(&self, call: ToolCall) -> Result<ToolResult, anyhow::Error> {
        debug!("Executing tool: {} with arguments: {:?}", call.name, call.arguments);
        match self.get(&call.name) {
            Some(tool) => tool.execute(call.arguments).await,
            None => anyhow::bail!("Tool not found: {}", call.name),
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Replace with actual ModelRegistry type when implemented
// Placeholder types for ModelRegistry and LlmBackend
pub struct ModelRegistry {
    // Placeholder - will be replaced with actual implementation
}

#[allow(dead_code)]
impl ModelRegistry {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_model_state(&self, _name: &str) -> ModelLifecycleState {
        ModelLifecycleState::Unloaded
    }

    pub fn set_model_state(&self, _name: &str, _state: ModelLifecycleState) {
        // Placeholder - will be replaced with actual implementation
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelLifecycleState {
    Unloaded,
    Loading,
    Active,
    Error { reason: String },
}

// TODO: Replace with actual LlmBackend trait when implemented
#[async_trait::async_trait]
pub trait LlmBackend: Send + Sync {
    async fn load_model(&self, name: &str) -> Result<(), anyhow::Error>;
    async fn unload_model(&self, name: &str) -> Result<(), anyhow::Error>;
    async fn chat(&self, model: &str, message: &str, temperature: Option<f32>) -> Result<String, anyhow::Error>;
}

// 1. ModelListTool
pub struct ModelListTool {
    registry: Arc<Mutex<ModelRegistry>>,
}

impl ModelListTool {
    pub fn new(registry: Arc<Mutex<ModelRegistry>>) -> Self {
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

        // Placeholder - return mock data
        let models = vec![
            serde_json::json!({
                "name": "llama-3.2-3b-instruct",
                "state": registry.get_model_state("llama-3.2-3b-instruct")
            }),
            serde_json::json!({
                "name": "llama3.2",
                "state": registry.get_model_state("llama3.2")
            }),
        ];

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
    registry: Arc<Mutex<ModelRegistry>>,
    backend: Arc<dyn LlmBackend>,
}

impl ModelLoadTool {
    pub fn new(registry: Arc<Mutex<ModelRegistry>>, backend: Arc<dyn LlmBackend>) -> Self {
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

        // Set state to Loading
        self.registry
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?
            .set_model_state(model_name, ModelLifecycleState::Loading);

        // Load model
        match self.backend.load_model(model_name).await {
            Ok(_) => {
                // Set state to Active
                self.registry
                    .lock()
                    .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?
                    .set_model_state(model_name, ModelLifecycleState::Active);

                Ok(ToolResult {
                    tool_name: "model_load".to_string(),
                    output: format!("Model '{}' loaded successfully", model_name),
                    success: true,
                    metadata: HashMap::new(),
                })
            }
            Err(e) => {
                // Set state to Error
                self.registry
                    .lock()
                    .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?
                    .set_model_state(
                        model_name,
                        ModelLifecycleState::Error {
                            reason: e.to_string(),
                        },
                    );

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
    registry: Arc<Mutex<ModelRegistry>>,
    backend: Arc<dyn LlmBackend>,
}

impl ModelUnloadTool {
    pub fn new(registry: Arc<Mutex<ModelRegistry>>, backend: Arc<dyn LlmBackend>) -> Self {
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
                self.registry
                    .lock()
                    .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?
                    .set_model_state(model_name, ModelLifecycleState::Unloaded);

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

        let temperature = arguments.get("temperature").and_then(|v| v.as_f64()).map(|t| t as f32);

        debug!("Executing ChatTool for model: {}, temperature: {:?}", model_name, temperature);

        match self.backend.chat(model_name, message, temperature).await {
            Ok(response) => Ok(ToolResult {
                tool_name: "chat".to_string(),
                output: response,
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
    allowed_paths: Vec<std::path::PathBuf>,
}

impl FileReadTool {
    pub fn new(allowed_paths: Vec<std::path::PathBuf>) -> Self {
        Self { allowed_paths }
    }

    fn is_path_allowed(&self, path: &std::path::Path) -> bool {
        self.allowed_paths.iter().any(|allowed| path.starts_with(allowed))
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

        if !self.is_path_allowed(&path) {
            return Ok(ToolResult {
                tool_name: "file_read".to_string(),
                output: format!("Access denied: path '{}' is not in allowed paths", path_str),
                success: false,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("error".to_string(), "Path not allowed".to_string());
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
