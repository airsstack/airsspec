//! LLM provider traits and types.
//!
//! This module defines the abstraction layer for Large Language Model providers.
//! All LLM implementations (`OpenAI`, `Anthropic`, local models, etc.) implement
//! these traits, making them swappable without changing application code.

pub mod traits;
pub mod types;

// No type re-exports per project-standard.md §4.3
// Callers import types directly from the types module:
// use airsspec_core::llm::types::{CompletionRequest, Message, Role};
