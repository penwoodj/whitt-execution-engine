use anyhow::Result;
use futures::Stream;
use std::pin::Pin;

use super::http_client::LlamaHttpClient;
use super::types::{ChatCompletionChunk, ChatCompletionRequest, ChatMessage};

#[derive(Debug, Clone)]
pub struct ChainStepResult {
    pub content: String,
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub step_number: usize,
}

pub struct PromptChain {
    client: LlamaHttpClient,
    messages: Vec<ChatMessage>,
    model: String,
    max_tokens: usize,
    temperature: f32,
    top_p: f32,
    step_count: usize,
}

impl std::fmt::Debug for PromptChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PromptChain")
            .field("model", &self.model)
            .field("max_tokens", &self.max_tokens)
            .field("temperature", &self.temperature)
            .field("top_p", &self.top_p)
            .field("step_count", &self.step_count)
            .field("messages_count", &self.messages.len())
            .field(
                "messages_preview",
                &self
                    .messages
                    .iter()
                    .map(|m| {
                        format!(
                            "{}: {}...",
                            m.role,
                            m.content.chars().take(50).collect::<String>()
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl PromptChain {
    pub fn new(client: LlamaHttpClient, model: impl Into<String>) -> Self {
        Self {
            client,
            messages: Vec::new(),
            model: model.into(),
            max_tokens: 512,
            temperature: 0.7,
            top_p: 0.95,
            step_count: 0,
        }
    }

    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.messages.insert(0, ChatMessage::system(prompt));
        self
    }

    pub async fn step(&mut self, user_prompt: impl Into<String>) -> Result<ChainStepResult> {
        let user_prompt = user_prompt.into();
        self.messages.push(ChatMessage::user(&user_prompt));

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: self.messages.clone(),
            max_tokens: Some(self.max_tokens),
            temperature: Some(self.temperature),
            top_p: Some(self.top_p),
            stream: false,
            ..Default::default()
        };

        let response = self.client.chat_completion(request).await?;

        let assistant_message = response.choices[0].message.clone();
        let content = assistant_message.content.clone();

        self.messages.push(assistant_message);
        self.step_count += 1;

        Ok(ChainStepResult {
            content,
            prompt_tokens: response.usage.prompt_tokens,
            completion_tokens: response.usage.completion_tokens,
            total_tokens: response.usage.total_tokens,
            step_number: self.step_count,
        })
    }

    pub async fn step_stream(
        &mut self,
        user_prompt: impl Into<String>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk>>>>> {
        let user_prompt = user_prompt.into();
        self.messages.push(ChatMessage::user(&user_prompt));

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: self.messages.clone(),
            max_tokens: Some(self.max_tokens),
            temperature: Some(self.temperature),
            top_p: Some(self.top_p),
            stream: true,
            ..Default::default()
        };

        let stream = self.client.chat_completion_stream(request).await?;

        Ok(stream)
    }

    pub fn history(&self) -> &[ChatMessage] {
        &self.messages
    }

    pub fn last_response(&self) -> Option<&str> {
        self.messages
            .last()
            .filter(|m| m.role == "assistant")
            .map(|m| m.content.as_str())
    }

    pub fn step_count(&self) -> usize {
        self.step_count
    }
}
