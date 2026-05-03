//! Whitt Execution Engine
//!
//! Declarative workflow engine for defining, executing, and optimizing
//! AI-powered workflows with local LLMs.

pub mod agent;
pub mod backend;
pub mod config;
pub mod error;
pub mod model;
pub mod quality;

#[cfg(feature = "client")]
pub mod client;
