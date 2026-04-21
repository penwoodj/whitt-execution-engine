//! Whitt Execution Engine
//!
//! Declarative workflow engine for defining, executing, and optimizing
//! AI-powered workflows with local LLMs.

pub mod config;
pub mod error;

#[cfg(feature = "client")]
pub mod client;
