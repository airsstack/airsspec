//! Error types for `AirsSpec`.
//!
//! This module defines all error types used throughout the `AirsSpec` framework.
//! All errors derive from `thiserror::Error` and provide helpful context for debugging.

// Layer 1: Standard library imports
use std::io;

// Layer 2: Third-party crate imports
use thiserror::Error;

// Layer 3: Internal module imports (none - this is a root module)

/// Top-level error type for `AirsSpec`.
///
/// This enum wraps all domain-specific errors and provides a single error type
/// for the entire framework. It automatically converts from `std::io::Error` and
/// all domain error variants via `From` implementations.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::error::{AirsspecError, Phase, StateError};
///
/// fn example() -> Result<(), AirsspecError> {
///     // Error automatically converts to AirsspecError
///     Err(StateError::InvalidTransition { from: Phase::Idle, to: Phase::Construction })?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Error)]
pub enum AirsspecError {
    /// State management errors.
    #[error("State error: {0}")]
    State(#[from] StateError),

    /// Artifact validation and storage errors.
    #[error("Artifact error: {0}")]
    Artifact(#[from] ArtifactError),

    /// Tool execution and registration errors.
    #[error("Tool error: {0}")]
    Tool(#[from] ToolError),

    /// LLM provider and interaction errors.
    #[error("LLM error: {0}")]
    Llm(#[from] LlmError),

    /// Memory tier errors.
    #[error("Memory error: {0}")]
    Memory(#[from] MemoryError),

    /// Knowledge store and vector search errors.
    #[error("Knowledge error: {0}")]
    Knowledge(#[from] KnowledgeError),

    /// Agent execution and delegation errors.
    #[error("Agent error: {0}")]
    Agent(#[from] AgentError),

    /// Plugin loading and execution errors.
    #[error("Plugin error: {0}")]
    Plugin(#[from] PluginError),

    /// I/O errors.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

/// State management errors.
///
/// These errors occur during state transitions, gate validation, and state persistence.
#[derive(Debug, Error)]
pub enum StateError {
    /// Invalid phase transition.
    ///
    /// This error occurs when attempting to transition from one phase to another
    /// that is not allowed by the compliance rules.
    #[error("Invalid transition from {from:?} to {to:?}")]
    InvalidTransition { from: Phase, to: Phase },

    /// Gate condition not met.
    ///
    /// This error occurs when a phase transition is attempted but the required
    /// gate conditions (e.g., missing or unapproved artifacts) are not satisfied.
    #[error("Gate condition not met: {0}")]
    GateNotMet(String),

    /// State persistence error.
    ///
    /// This error occurs when loading or saving state fails.
    #[error("State persistence error: {0}")]
    Persistence(String),
}

/// Artifact validation and storage errors.
///
/// These errors occur during artifact creation, validation, and persistence.
#[derive(Debug, Error)]
pub enum ArtifactError {
    /// Artifact validation failed.
    #[error("Artifact validation failed: {0}")]
    Validation(String),

    /// Artifact not found.
    #[error("Artifact not found: {0}")]
    NotFound(String),

    /// Artifact storage error.
    #[error("Artifact storage error: {0}")]
    Storage(String),
}

/// Tool execution and registration errors.
///
/// These errors occur during tool registration, execution, and output handling.
#[derive(Debug, Error)]
pub enum ToolError {
    /// Tool not found.
    #[error("Tool not found: {0}")]
    NotFound(String),

    /// Tool execution failed.
    #[error("Tool execution failed: {0}")]
    Execution(String),

    /// Invalid tool input.
    #[error("Invalid tool input: {0}")]
    InvalidInput(String),

    /// Tool output parsing failed.
    #[error("Tool output parsing failed: {0}")]
    OutputParse(String),
}

/// LLM provider and interaction errors.
///
/// These errors occur during LLM requests, streaming, and response handling.
#[derive(Debug, Error)]
pub enum LlmError {
    /// LLM provider request failed.
    #[error("LLM request failed: {0}")]
    Request(String),

    /// Invalid LLM response.
    #[error("Invalid LLM response: {0}")]
    InvalidResponse(String),

    /// Token usage exceeded budget.
    #[error("Token usage exceeded budget: used={used}, limit={limit}")]
    ExceededBudget { used: u32, limit: u32 },

    /// Streaming error.
    #[error("LLM streaming error: {0}")]
    Streaming(String),
}

/// Memory tier errors.
///
/// These errors occur during memory operations across the three tiers (hot, warm, cold).
#[derive(Debug, Error)]
pub enum MemoryError {
    /// Hot memory error.
    #[error("Hot memory error: {0}")]
    Hot(String),

    /// Warm memory error.
    #[error("Warm memory error: {0}")]
    Warm(String),

    /// Cold memory error.
    #[error("Cold memory error: {0}")]
    Cold(String),

    /// Memory compression error.
    #[error("Memory compression error: {0}")]
    Compression(String),
}

/// Knowledge store and vector search errors.
///
/// These errors occur during document indexing, embedding generation, and similarity search.
#[derive(Debug, Error)]
pub enum KnowledgeError {
    /// Document indexing failed.
    #[error("Document indexing failed: {0}")]
    Indexing(String),

    /// Embedding generation failed.
    #[error("Embedding generation failed: {0}")]
    Embedding(String),

    /// Vector search failed.
    #[error("Vector search failed: {0}")]
    Search(String),

    /// Knowledge store not found.
    #[error("Knowledge store not found: {0}")]
    StoreNotFound(String),
}

/// Agent execution and delegation errors.
///
/// These errors occur during agent execution, delegation, and output handling.
#[derive(Debug, Error)]
pub enum AgentError {
    /// Agent not found.
    #[error("Agent not found: {0}")]
    NotFound(String),

    /// Agent execution failed.
    #[error("Agent execution failed: {0}")]
    Execution(String),

    /// Delegation failed.
    #[error("Delegation failed: {0}")]
    Delegation(String),

    /// Budget exceeded.
    #[error("Agent budget exceeded: {0}")]
    BudgetExceeded(String),

    /// Invalid agent configuration.
    #[error("Invalid agent configuration: {0}")]
    InvalidConfig(String),
}

/// Plugin loading and execution errors.
///
/// These errors occur during plugin discovery, loading, and overlay application.
#[derive(Debug, Error)]
pub enum PluginError {
    /// Plugin not found.
    #[error("Plugin not found: {0}")]
    NotFound(String),

    /// Plugin loading failed.
    #[error("Plugin loading failed: {0}")]
    Load(String),

    /// Invalid plugin manifest.
    #[error("Invalid plugin manifest: {0}")]
    InvalidManifest(String),

    /// Prompt overlay application failed.
    #[error("Prompt overlay application failed: {0}")]
    Overlay(String),

    /// Resolution order conflict.
    #[error("Resolution order conflict: {0}")]
    ResolutionConflict(String),
}

/// Phase enum used in error types.
///
/// This is a forward declaration - the full `Phase` type will be defined in `state/types.rs`.
/// This placeholder allows error types to compile before the state module is implemented.
///
/// TODO: Remove this placeholder and import `Phase` from `state::types` after state types are implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Idle,
    Research,
    Inception,
    Design,
    Planning,
    Construction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_airsspec_error_display() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "test");
        let error: AirsspecError = io_error.into();
        assert!(error.to_string().contains("IO error"));
    }

    #[test]
    fn test_state_error_invalid_transition() {
        let error = StateError::InvalidTransition {
            from: Phase::Idle,
            to: Phase::Construction,
        };
        assert!(error.to_string().contains("Invalid transition"));
        assert!(error.to_string().contains("Idle"));
        assert!(error.to_string().contains("Construction"));
    }

    #[test]
    fn test_state_error_gate_not_met() {
        let error = StateError::GateNotMet("Artifact not approved".to_string());
        assert!(error.to_string().contains("Gate condition not met"));
        assert!(error.to_string().contains("Artifact not approved"));
    }

    #[test]
    fn test_airsspec_error_from_state_error() {
        let state_error = StateError::GateNotMet("test".to_string());
        let airsspec_error: AirsspecError = state_error.into();
        assert!(matches!(airsspec_error, AirsspecError::State(_)));
    }
}
