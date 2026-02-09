//! # MCP Command Handler
//!
//! Handles the `airsspec mcp` subcommand, which starts the MCP server
//! using stdio transport.
//!
//! ## Current Status
//!
//! This is a placeholder implementation. The full MCP server will be
//! implemented in Phase 5 (MCP Server).

/// Run the MCP server command.
///
/// Starts the `AirsSpec` MCP server that exposes spec workflow primitives
/// to AI coding tools via stdio transport.
///
/// # Arguments
///
/// * `debug` - When `true`, enables verbose debug logging and diagnostics
///   including request/response payloads and internal state transitions.
///
/// # Errors
///
/// Returns an error if:
/// - The MCP server fails to bind to stdio transport
/// - The server encounters a fatal runtime error
// Intentionally async: MCP server will use async I/O in Phase 5.
#[expect(
    clippy::unused_async,
    reason = "intentionally async for Phase 5 MCP server integration"
)]
pub async fn run(debug: bool) -> anyhow::Result<()> {
    println!("AirsSpec MCP Server");
    println!("Debug mode: {debug}");
    println!("(MCP server will be implemented in Phase 5)");
    Ok(())
}
