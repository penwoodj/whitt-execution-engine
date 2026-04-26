//! Backend abstraction layer for LLM providers.
//!
//! Provides a unified interface for interacting with different LLM backends
//! (LM Studio, Ollama, llama.cpp with Vulkan).

pub mod llm_backend;
pub mod llama_vulkan;

pub use llm_backend::{LlmBackend, LlmError, BackendCapabilities, HealthStatus};
pub use llama_vulkan::LlamaCppVulkanBackend;
