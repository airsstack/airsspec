//! # airsspec-mcp
//!
//! MCP server implementation and I/O layer for `AirsSpec`.
//!
//! This crate provides filesystem storage implementations and (in the future)
//! the MCP protocol server. Per ADR-002, all I/O operations live here.
//!
//! ## Architecture
//!
//! Per ADR-002 (4-Crate Structure), this crate is the I/O adapter layer:
//!
//! - Implements traits from `airsspec-core` with real filesystem operations
//! - All file I/O operations happen here
//! - MCP protocol implementation (Phase 5, deferred)
//! - JSON-RPC 2.0 request/response handling (Phase 5, deferred)
//!
//! ## Modules
//!
//! - [`storage`] - Filesystem implementations of core storage traits
//!
//! ## Future Modules (Phase 5+)
//!
//! - `server/` - MCP server setup and lifecycle
//! - `tools/` - MCP tool handlers (`spec_create`, `plan_create`, etc.)
//! - `resources/` - Resource providers (`airsspec:///` URIs)
//! - `prompts/` - Prompt template providers
//! - `logging/` - JSONL session logging

pub mod storage;

// Convenience re-exports
pub use storage::FileSystemWorkspaceProvider;
