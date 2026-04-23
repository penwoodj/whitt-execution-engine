//! Client module for llama-server communication.
//!
//! Provides:
//! - `types` — API request/response types (OpenAI-compatible)
//! - `http_client` — HTTP client with SSE streaming
//! - `docker_manager` — Docker container lifecycle management
//! - `model_download` — HuggingFace model download utilities

pub mod docker_manager;
pub mod http_client;
pub mod model_download;
pub mod prompt_chain;
pub mod types;
