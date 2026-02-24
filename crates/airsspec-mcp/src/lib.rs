//! # airsspec-mcp
//!
//! MCP server implementation and I/O layer for `AirsSpec`.
//!
//! This crate provides filesystem storage implementations and the MCP
//! protocol server. Per ADR-002, all I/O operations live here.
//!
//! ## Architecture
//!
//! Per ADR-002 (4-Crate Structure), this crate is the I/O adapter layer:
//!
//! - Implements traits from `airsspec-core` with real filesystem operations
//! - All file I/O operations happen here
//! - MCP protocol server with JSON-RPC 2.0 request/response handling
//!
//! ## Modules
//!
//! - [`storage`] - Filesystem implementations of core storage traits
//! - [`validation`] - Workspace validation orchestration
//! - [`server`] - MCP server setup, handler, and lifecycle
//!
//! ## Future Modules (Phase 5+)
//!
//! - `tools/` - MCP tool handlers (`spec_create`, `plan_create`, etc.)
//! - `resources/` - Resource providers (`airsspec:///` URIs)
//! - `prompts/` - Prompt template providers
//! - `logging/` - JSONL session logging

pub mod server;
pub mod storage;
pub mod validation;

// Convenience re-exports
pub use server::{AirsSpecHandler, McpServerBuilder, ServerError};
pub use storage::FileSystemPlanStorage;
pub use storage::FileSystemSpecStorage;
pub use storage::FileSystemWorkspaceProvider;
pub use validation::validate_workspace;
