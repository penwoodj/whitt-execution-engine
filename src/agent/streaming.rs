use serde::{Deserialize, Serialize};
use tracing::debug;

#[cfg(feature = "client")]
use futures::stream::Stream;
#[cfg(feature = "client")]
use std::pin::Pin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingResponse {
    model: String,
    content: String,
    is_complete: bool,
}

#[cfg(feature = "client")]
pub type SSEStream = Pin<Box<dyn Stream<Item = Result<StreamEvent, anyhow::Error>> + Send>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamEvent {
    Token(String),
    ToolCallStart { name: String },
    ToolCallEnd { name: String, result: String },
    Complete { total_tokens: u64 },
    Error(String),
}

impl StreamingResponse {
    pub fn new(model: String) -> Self {
        Self {
            model,
            content: String::new(),
            is_complete: false,
        }
    }

    pub fn process_sse_line(&mut self, line: &str) -> Option<StreamEvent> {
        debug!("Processing SSE line: {}", line);

        // Skip empty lines and comments
        let line = line.trim();
        if line.is_empty() || line.starts_with(':') {
            return None;
        }

        // Parse SSE format: "data: {...}"
        if let Some(data_start) = line.strip_prefix("data: ") {
            return self.parse_data_line(data_start);
        }

        None
    }

    fn parse_data_line(&mut self, data: &str) -> Option<StreamEvent> {
        // Try to parse as JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
            // Check for different event types
            if let Some(event_type) = json.get("event").and_then(|e| e.as_str()) {
                match event_type {
                    "token" => {
                        if let Some(token) = json.get("content").and_then(|c| c.as_str()) {
                            self.content.push_str(token);
                            return Some(StreamEvent::Token(token.to_string()));
                        }
                    }
                    "tool_call_start" => {
                        if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                            return Some(StreamEvent::ToolCallStart {
                                name: name.to_string(),
                            });
                        }
                    }
                    "tool_call_end" => {
                        if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                            if let Some(result) = json.get("result").and_then(|r| r.as_str()) {
                                return Some(StreamEvent::ToolCallEnd {
                                    name: name.to_string(),
                                    result: result.to_string(),
                                });
                            }
                        }
                    }
                    "complete" => {
                        self.is_complete = true;
                        if let Some(tokens) = json.get("total_tokens").and_then(|t| t.as_u64()) {
                            return Some(StreamEvent::Complete { total_tokens: tokens });
                        }
                        return Some(StreamEvent::Complete { total_tokens: 0 });
                    }
                    "error" => {
                        if let Some(error) = json.get("message").and_then(|m| m.as_str()) {
                            return Some(StreamEvent::Error(error.to_string()));
                        }
                    }
                    _ => {
                        debug!("Unknown event type: {}", event_type);
                    }
                }
            } else if let Some(content) = json.get("content").and_then(|c| c.as_str()) {
                // Default: treat as token
                self.content.push_str(content);
                return Some(StreamEvent::Token(content.to_string()));
            }
        } else {
            // Not JSON, treat as raw text (some servers send plain text in data)
            self.content.push_str(data);
            return Some(StreamEvent::Token(data.to_string()));
        }

        None
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn model(&self) -> &str {
        &self.model
    }
}

impl Default for StreamingResponse {
    fn default() -> Self {
        Self::new("unknown".to_string())
    }
}

/// Parse raw SSE text into a vector of StreamEvents
///
/// This function is available in all builds, regardless of features.
/// It processes SSE format lines and extracts events.
pub fn parse_sse_stream(raw: &str) -> Vec<StreamEvent> {
    let mut response = StreamingResponse::new("unknown".to_string());
    let mut events = Vec::new();

    for line in raw.lines() {
        if let Some(event) = response.process_sse_line(line) {
            events.push(event);
        }
    }

    events
}

/// Parse a single SSE line into a StreamEvent
///
/// This function is available in all builds, regardless of features.
/// It processes one line of SSE format and extracts an event if present.
pub fn parse_sse_line(line: &str) -> Option<StreamEvent> {
    let mut response = StreamingResponse::new("unknown".to_string());
    response.process_sse_line(line)
}

#[cfg(feature = "client")]
pub mod client_streaming {
    use super::*;
    use futures::stream::unfold;
    use tokio::io::AsyncBufReadExt;

    /// Create an SSE stream from a tokio reader
    pub async fn create_sse_stream<R>(
        reader: R,
        model: String,
    ) -> Result<SSEStream, anyhow::Error>
    where
        R: tokio::io::AsyncBufRead + Unpin + Send + 'static,
    {
        let stream = unfold((reader, String::new(), model.clone()), move |(mut reader, mut line, model)| async move {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => None, // End of stream
                Ok(_) => {
                    let mut response = StreamingResponse::new(model.clone());
                    let event = response.process_sse_line(&line)
                        .unwrap_or_else(|| StreamEvent::Error(format!("Failed to parse SSE line: {}", line)));
                    Some((Ok(event), (reader, line, model)))
                }
                Err(e) => Some((Ok(StreamEvent::Error(format!("Stream error: {}", e))), (reader, line, model))),
            }
        });

        Ok(Box::pin(stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sse_line() {
        // Test token event
        let event = parse_sse_line("data: {\"event\": \"token\", \"content\": \"Hello\"}").unwrap();
        assert!(matches!(event, StreamEvent::Token(ref s) if s == "Hello"));

        // Test tool call start
        let event = parse_sse_line("data: {\"event\": \"tool_call_start\", \"name\": \"test_tool\"}").unwrap();
        assert!(matches!(event, StreamEvent::ToolCallStart { ref name } if name == "test_tool"));

        // Test complete event
        let event = parse_sse_line("data: {\"event\": \"complete\", \"total_tokens\": 100}").unwrap();
        assert!(matches!(event, StreamEvent::Complete { total_tokens: 100 }));

        // Test empty line
        assert!(parse_sse_line("").is_none());
        assert!(parse_sse_line(": comment").is_none());
    }

    #[test]
    fn test_parse_sse_stream() {
        let raw = r#"
data: {"event": "token", "content": "Hello"}
data: {"event": "token", "content": " World"}
data: {"event": "complete", "total_tokens": 10}
"#;

        let events = parse_sse_stream(raw);
        assert_eq!(events.len(), 3);
        assert!(matches!(events[0], StreamEvent::Token(ref s) if s == "Hello"));
        assert!(matches!(events[1], StreamEvent::Token(ref s) if s == " World"));
        assert!(matches!(events[2], StreamEvent::Complete { total_tokens: 10 }));
    }

    #[test]
    fn test_streaming_response() {
        let mut response = StreamingResponse::new("test-model".to_string());

        assert_eq!(response.content(), "");
        assert!(!response.is_complete());

        response.process_sse_line("data: Hello");
        assert_eq!(response.content(), "Hello");

        response.process_sse_line("data: World");
        assert_eq!(response.content(), "HelloWorld");

        response.process_sse_line("data: {\"event\": \"complete\"}");
        assert!(response.is_complete());
    }
}
