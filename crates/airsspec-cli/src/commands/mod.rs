//! # Command Handlers
//!
//! Contains the implementation modules for each CLI subcommand.
//!
//! Each command handler is an async function that returns `anyhow::Result<()>`.
//! These are placeholder implementations that will be replaced with real logic
//! in subsequent phases:
//!
//! - [`init`] -- Phase 3.3 (Init Command Integration with TUI Wizard)
//! - [`mcp`] -- Phase 5 (MCP Server)
//! - [`validate`] -- Phase 4 (Validation Engine)
//!
//! ## Architecture
//!
//! Per ADR-002, command handlers are thin orchestration functions that delegate
//! to library crates (`airsspec-core`, `airsspec-mcp`, `airsspec-tui`).
//! No business logic belongs in these handlers.

pub mod init;
pub mod mcp;
pub mod validate;
