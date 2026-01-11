//! LLM types for request/response handling.
//!
//! This module defines the data structures used for interacting with LLM providers.

// Layer 1: Standard library imports (none needed)

// Layer 2: Third-party crate imports
use serde::{Deserialize, Serialize};

// Layer 3: Internal module imports (none - this is a leaf module)

/// Completion request to an LLM provider.
///
/// This structure represents a standard chat completion request compatible with
/// OpenAI-style APIs and other LLM providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    /// The messages in the conversation.
    pub messages: Vec<Message>,

    /// Maximum number of tokens to generate.
    pub max_tokens: Option<u32>,

    /// Sampling temperature (0.0 to 2.0).
    ///
    /// Higher values (e.g., 0.8) make output more random, lower values (e.g., 0.2)
    /// make it more focused and deterministic.
    pub temperature: Option<f32>,
}

/// A single message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message sender.
    pub role: Role,

    /// The message content.
    pub content: String,
}

/// The role of a message sender in a conversation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    /// System message that sets the behavior of the assistant.
    System,

    /// User message.
    User,

    /// Assistant message (the model's response).
    Assistant,
}

/// Token usage statistics for an LLM completion.
///
/// This tracks how many tokens were consumed for both input and output,
/// useful for cost tracking and budget management.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Number of tokens in the prompt (input).
    pub prompt_tokens: u32,

    /// Number of tokens in the completion (output).
    pub completion_tokens: u32,

    /// Total number of tokens used.
    pub total_tokens: u32,
}

impl TokenUsage {
    /// Creates a new `TokenUsage` from prompt and completion counts.
    #[must_use]
    pub const fn new(prompt_tokens: u32, completion_tokens: u32) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_request_creation() {
        let request = CompletionRequest {
            messages: vec![Message {
                role: Role::User,
                content: "Hello, world!".to_string(),
            }],
            max_tokens: Some(100),
            temperature: Some(0.7),
        };

        assert_eq!(request.messages.len(), 1);
        assert_eq!(request.max_tokens, Some(100));
        assert_eq!(request.temperature, Some(0.7));
    }

    #[test]
    fn test_role_variants() {
        assert_eq!(Role::System, Role::System);
        assert_ne!(Role::User, Role::Assistant);
    }

    #[test]
    fn test_token_usage_new() {
        let usage = TokenUsage::new(10, 5);
        assert_eq!(usage.prompt_tokens, 10);
        assert_eq!(usage.completion_tokens, 5);
        assert_eq!(usage.total_tokens, 15);
    }

    #[test]
    fn test_token_usage_default() {
        let usage = TokenUsage::default();
        assert_eq!(usage.prompt_tokens, 0);
        assert_eq!(usage.completion_tokens, 0);
        assert_eq!(usage.total_tokens, 0);
    }

    #[test]
    fn test_serialization() {
        let request = CompletionRequest {
            messages: vec![Message {
                role: Role::User,
                content: "test".to_string(),
            }],
            max_tokens: None,
            temperature: None,
        };

        // Test that the structure can be serialized to JSON
        let result = serde_json::to_string(&request);
        assert!(result.is_ok());

        match result {
            Ok(json) => {
                assert!(json.contains("messages"));
                assert!(json.contains("role"));
                assert!(json.contains("content"));
            }
            Err(e) => panic!("Serialization should not fail: {e}"),
        }
    }
}
