//! # airsspec-cli
//!
//! Binary entry point for `AirsSpec` - a lightweight, MCP-first spec-driven development framework.
//!
//! This is a thin orchestration layer that delegates all business logic to library crates.
//!
//! ## Architecture
//!
//! Per [ADR-002: 4-Crate Structure](../../.memory-bank/sub-projects/airsspec/docs/adr/adr-002-4-crate-structure.md),
//! this crate:
//!
//! - Parses CLI arguments using `clap`
//! - Routes commands to appropriate library implementations
//! - Contains NO business logic (all logic lives in libraries)
//! - Uses `mimalloc` allocator for better performance
//!
//! ## Commands (Future)
//!
//! - `airsspec init` - Initialize workspace (TUI wizard)
//! - `airsspec mcp` - Start MCP server (stdio transport)
//! - `airsspec validate` - Run validation with TUI reporter
//!
//! ## Global Allocator
//!
//! Uses `mimalloc` for improved memory allocation performance.

use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

fn main() {
    // CLI entry point - thin orchestration layer
    // Command implementations will be added in Phase 3
    println!("AirsSpec v0.1.0");
    println!("Phase 1: Project Setup Complete");
}
