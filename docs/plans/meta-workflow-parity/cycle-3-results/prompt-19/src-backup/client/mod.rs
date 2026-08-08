//! Client module for llama-server communication.
//!
//! Provides:
//! - `types` — API request/response types (OpenAI-compatible)
//! - `http_client` — HTTP client with SSE streaming
//! - `docker_manager` — Docker container lifecycle management
//! - `model_download` — HuggingFace model download utilities
//! - `model_discovery` — GGUF model discovery from directories
//! - `disk_monitor` — Disk space monitoring
//! - `memory_monitor` — System memory monitoring

pub mod disk_monitor;
pub mod docker_manager;
pub mod http_client;
pub mod memory_monitor;
pub mod model_discovery;
pub mod model_download;
pub mod prompt_chain;
pub mod types;
