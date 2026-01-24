//! # airsspec-mcp
//!
//! MCP server implementation for `AirsSpec`.
//!
//! ## ⚠️ Phase 5 Implementation Note
//!
//! **MCP Protocol Dependency Deferred:**
//! The `airsprotocols-mcp` library dependency is currently commented out due to compilation
//! errors with the current Rust toolchain. This is intentional and follows Option E from the
//! Phase 1 implementation plan.
//!
//! **Why This Is Acceptable:**
//! - MCP server implementation doesn't start until Phase 5 (6-8 hours from Phase 1)
//! - Phases 1-4 (workspace setup, domain models, CLI skeleton, validation) don't need MCP
//! - This allows us to proceed with foundational work without being blocked
//! - When Phase 5 begins, we'll uncomment the dependency and address any compilation issues
//!
//! **TODO(Phase 5):**
//! 1. Uncomment `airsprotocols-mcp` dependency in workspace and this crate's Cargo.toml
//! 2. Fix any compilation issues (or upgrade to a fixed version)
//! 3. Implement MCP server using the library
//!
//! ## Architecture
//!
//! Per [ADR-002: 4-Crate Structure](../../.memory-bank/sub-projects/airsspec/docs/adr/adr-002-4-crate-structure.md)
//! and [ADR-007: MCP Library Selection](../../.memory-bank/sub-projects/airsspec/docs/adr/adr-007-mcp-library-selection.md),
//! this crate is the I/O adapter layer:
//!
//! - Implements traits from `airsspec-core`
//! - All file I/O operations happen here
//! - MCP protocol implementation via `airsprotocols-mcp` (when dependency is restored)
//! - JSON-RPC 2.0 request/response handling
//!
//! ## Modules (Future - Phase 5+)
//!
//! - `server/` - MCP server setup and lifecycle
//! - `tools/` - MCP tool handlers (`spec_create`, `plan_create`, etc.)
//! - `resources/` - Resource providers (airsspec:/// URIs)
//! - `prompts/` - Prompt template providers
//! - `storage/` - File system implementations of core traits
//! - `logging/` - JSONL session logging
//!
//! ## Dependencies
//!
//! - `airsspec-core` - Domain models and trait definitions
//! - `airsprotocols-mcp` - **DEFERRED to Phase 5** (see note above)
//! - `tokio` - Async runtime
//! - `serde`, `serde_json` - Serialization

// Phase 1: Skeleton only - MCP server implementation will happen in Phase 5
