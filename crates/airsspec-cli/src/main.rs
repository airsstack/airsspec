//! # airsspec-cli
//!
//! Binary entry point for `AirsSpec` -- a lightweight, MCP-first spec-driven
//! development framework.
//!
//! This is a thin orchestration layer that delegates all business logic to
//! library crates. Per ADR-002 (4-Crate Structure), the CLI crate:
//!
//! - Parses CLI arguments using `clap`
//! - Routes commands to appropriate library implementations
//! - Contains NO business logic (all logic lives in library crates)
//!
//! ## Commands
//!
//! | Command              | Description                              |
//! |----------------------|------------------------------------------|
//! | `airsspec init`      | Initialize workspace (TUI wizard)        |
//! | `airsspec mcp`       | Start MCP server (stdio transport)       |
//! | `airsspec validate`  | Run validation with TUI reporter         |
//!
//! ## Error Handling
//!
//! Command handlers return `anyhow::Result<()>`. Errors are caught in `main()`,
//! printed to stderr, and mapped to a non-zero exit code per Unix conventions.

mod cli;
mod commands;

use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => commands::init::run().await,
        Commands::Mcp { debug } => commands::mcp::run(debug).await,
        Commands::Validate => commands::validate::run().await,
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            ExitCode::FAILURE
        }
    }
}
