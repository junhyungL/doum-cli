//! OpenAI LLM integration module.

pub mod client;
pub mod payloads;

pub use client::OpenAIClient;
pub use payloads::OpenAIConfig;
