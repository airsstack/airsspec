//! Server error types for the `AirsSpec` MCP server.
//!
//! Provides a unified error type covering transport, builder, serialization,
//! and provider failure modes for the MCP server module.

use airsprotocols_mcp::McpError;
use airsprotocols_mcp::protocol::TransportError;

/// Errors that can occur in the MCP server.
#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    /// Transport-level I/O error.
    #[error("transport error: {0}")]
    Transport(#[from] TransportError),

    /// Invalid or non-existent workspace path.
    #[error("invalid workspace: {0}")]
    InvalidWorkspace(String),

    /// JSON serialization/deserialization error.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Provider-level error (tool/resource/prompt).
    #[error("provider error: {0}")]
    Provider(#[from] McpError),
}
