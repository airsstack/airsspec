//! MCP server implementation for `AirsSpec`.
//!
//! This module provides the core server infrastructure:
//!
//! - [`AirsSpecHandler`] -- Central message handler implementing
//!   [`MessageHandler`](airsprotocols_mcp::MessageHandler). Routes JSON-RPC
//!   requests to providers and writes responses directly to stdout.
//! - [`McpServerBuilder`] -- Builder for constructing a fully configured
//!   MCP server with stdio transport.
//! - [`ServerError`] -- Error types for server operations.

mod builder;
mod error;
mod handler;

pub use builder::McpServerBuilder;
pub use error::ServerError;
pub use handler::AirsSpecHandler;
