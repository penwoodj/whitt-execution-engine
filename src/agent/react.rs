use crate::agent::tools::{ToolCall, ToolRegistry, ToolResult};
use crate::backend::llm_backend::{LlmBackend, ChatMessage, ChatResponse};
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct AgentResponse {
    pub final_answer: String,
    pub tool_calls: Vec<ToolCall>,
    pub tool_results: Vec<ToolResult>,
    pub iterations: usize,
    pub total_tokens: u64,
}

pub struct ReactAgent {
    backend: Arc<dyn LlmBackend>,
    tools: Arc<ToolRegistry>,
    model: String,
    max_iterations: usize,
    system_prompt: String,
}

impl ReactAgent {
    pub fn new(backend: Arc<dyn LlmBackend>, tools: Arc<ToolRegistry>, model: String) -> Self {
        Self {
            backend,
            tools,
            model,
            max_iterations: 10,
            system_prompt: Self::default_system_prompt(),
        }
    }

    pub fn with_system_prompt(mut self, prompt: String) -> Self {
        self.system_prompt = prompt;
        self
    }

    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    fn default_system_prompt() -> String {
        r#"You are a helpful AI assistant with access to tools. Use the tools to gather information and complete tasks.

Available tools:
- model_list: List available models and their state
- model_load: Load a model into memory (args: model_name)
- model_unload: Unload a model from memory (args: model_name)
- chat: Send a message to a model (args: model_name, message, temperature)
- file_read: Read a file from the workspace (args: path)
- final_answer: Provide your final answer (args: answer)

When you need to use a tool, respond with a JSON object in this format:
{
  "tool": "tool_name",
  "arguments": {
    "arg1": "value1",
    "arg2": "value2"
  }
}

When you have the final answer, use the final_answer tool."#
            .to_string()
    }

    async fn build_messages(&self, user_message: &str, conversation: &[ChatMessage]) -> Vec<ChatMessage> {
        let mut messages = Vec::new();

        // Add system prompt with tool definitions
        let mut system_content = self.system_prompt.clone();
        
        // Add tool schemas
        system_content.push_str("\n\nTool Definitions:\n");
        for (name, description) in self.tools.list() {
            system_content.push_str(&format!("- {}: {}\n", name, description));
        }

        messages.push(ChatMessage {
            role: "system".to_string(),
            content: system_content,
        });

        // Add conversation history
        messages.extend_from_slice(conversation);

        // Add user message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_message.to_string(),
        });

        messages
    }

    fn parse_response(&self, response: &str) -> Result<Option<ToolCall>, anyhow::Error> {
        debug!("Parsing LLM response: {}", response);

        // Try to parse as JSON tool call
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(response) {
            if let Some(tool_name) = json.get("tool").and_then(|t| t.as_str()) {
                let arguments = json
                    .get("arguments")
                    .and_then(|a| a.as_object())
                    .map(|obj| {
                        obj.iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect()
                    })
                    .unwrap_or_default();

                return Ok(Some(ToolCall {
                    name: tool_name.to_string(),
                    arguments,
                }));
            }
        }

        // Check if it looks like a final answer
        let lower = response.to_lowercase();
        if lower.contains("final_answer") || lower.starts_with("answer:") {
            return Ok(Some(ToolCall {
                name: "final_answer".to_string(),
                arguments: {
                    let mut args = std::collections::HashMap::new();
                    args.insert(
                        "answer".to_string(),
                        serde_json::Value::String(response.to_string()),
                    );
                    args
                },
            }));
        }

        Ok(None)
    }

    pub async fn run(&self, user_message: String) -> Result<AgentResponse, anyhow::Error> {
        debug!("Starting ReAct loop with model: {}", self.model);

        let mut conversation: Vec<ChatMessage> = Vec::new();
        let mut tool_calls = Vec::new();
        let mut tool_results = Vec::new();
        let mut total_tokens = 0;

        for iteration in 0..self.max_iterations {
            debug!("ReAct iteration: {}/{}", iteration + 1, self.max_iterations);

            // Build messages
            let messages = self.build_messages(&user_message, &conversation).await;

            // Call LLM
            let llm_response = self.call_llm(&messages).await?;

            // Parse response for tool calls
            match self.parse_response(&llm_response.content)? {
                Some(tool_call) => {
                    tool_calls.push(tool_call.clone());

                    // Execute tool
                    let tool_result = self.tools.execute(tool_call.clone()).await?;
                    debug!("Tool result: {:?}", tool_result);

                    tool_results.push(tool_result.clone());

                    if tool_call.name == "final_answer" || tool_result.metadata.get("type") == Some(&"final_answer".to_string()) {
                        debug!("Final answer received, ending ReAct loop");
                        let tokens_used = *llm_response.usage.get("total_tokens").unwrap_or(&0);
                        return Ok(AgentResponse {
                            final_answer: tool_result.output,
                            tool_calls,
                            tool_results,
                            iterations: iteration + 1,
                            total_tokens: tokens_used + total_tokens,
                        });
                    }

                    // Add tool result to conversation
                    conversation.push(ChatMessage {
                        role: "assistant".to_string(),
                        content: llm_response.content,
                    });
                    conversation.push(ChatMessage {
                        role: "tool".to_string(),
                        content: tool_result.output,
                    });

                    let tokens_used = *llm_response.usage.get("total_tokens").unwrap_or(&0);
                    total_tokens += tokens_used;
                }
                None => {
                    debug!("No tool call detected, treating as direct response");

                    let tokens_used = *llm_response.usage.get("total_tokens").unwrap_or(&0);
                    return Ok(AgentResponse {
                        final_answer: llm_response.content,
                        tool_calls,
                        tool_results,
                        iterations: iteration + 1,
                        total_tokens: tokens_used + total_tokens,
                    });
                }
            }
        }

        debug!("Max iterations reached, returning best effort");

        Ok(AgentResponse {
            final_answer: conversation
                .last()
                .map(|m| m.content.clone())
                .unwrap_or_else(|| "No answer generated".to_string()),
            tool_calls,
            tool_results,
            iterations: self.max_iterations,
            total_tokens,
        })
    }

    async fn call_llm(&self, messages: &[ChatMessage]) -> Result<ChatResponse, anyhow::Error> {
        debug!("Calling LLM with {} messages", messages.len());

        let response = self
            .backend
            .chat(messages.to_vec(), &self.model)
            .await
            .map_err(|e| anyhow::anyhow!("LLM call failed: {}", e))?;

        let tokens_used = *response.usage.get("total_tokens").unwrap_or(&0);

        debug!("LLM response received, tokens used: {}", tokens_used);

        Ok(response)
    }
}
