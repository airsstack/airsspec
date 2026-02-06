//! # CLI Argument Definitions
//!
//! Defines the command-line interface for `AirsSpec` using `clap` derive macros.
//!
//! This module contains pure data definitions for CLI parsing -- no business logic.
//! The [`Cli`] struct is the top-level parser, and [`Commands`] enumerates the
//! available subcommands.
//!
//! ## Commands
//!
//! | Command    | Description                              |
//! |------------|------------------------------------------|
//! | `init`     | Initialize a new `AirsSpec` workspace      |
//! | `mcp`      | Start the MCP server (stdio transport)   |
//! | `validate` | Run spec validation with TUI reporter    |
//!
//! ## Architecture
//!
//! Per ADR-002 (4-Crate Structure), this module is part of the thin CLI
//! orchestration layer. All types here are pure data -- command routing and
//! business logic live elsewhere.

use clap::{Parser, Subcommand};

/// Top-level CLI entry point for `AirsSpec`.
///
/// `AirsSpec` is a lightweight, MCP-first spec-driven development framework
/// that provides a 3-phase workflow (Spec -> Plan -> Build) through a
/// plugin system.
///
/// Use `--help` on any subcommand for more details.
#[derive(Debug, Parser)]
#[command(
    name = "airsspec",
    about = "Lightweight, MCP-first spec-driven development framework",
    long_about = "AirsSpec is a lightweight, MCP-first spec-driven development framework \
                  that provides a 3-phase workflow (Spec -> Plan -> Build) scaling from \
                  quick fixes to complex features through a plugin system.",
    version,
    propagate_version = true
)]
pub struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI subcommands.
///
/// Each variant corresponds to a top-level command that delegates to
/// the appropriate library crate for execution.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new `AirsSpec` workspace with an interactive TUI wizard.
    ///
    /// Creates the workspace directory structure, configuration files, and
    /// initial spec templates. Guides the user through project setup with
    /// an interactive terminal UI.
    Init,

    /// Start the MCP server using stdio transport.
    ///
    /// Launches the `AirsSpec` MCP server that exposes spec workflow primitives
    /// (spec, plan, build tools) to AI coding tools. Communication uses the
    /// stdio transport protocol.
    Mcp {
        /// Enable debug mode for verbose logging and diagnostics.
        ///
        /// When set, the MCP server produces detailed debug output including
        /// request/response payloads and internal state transitions.
        #[arg(long, default_value_t = false)]
        debug: bool,
    },

    /// Run spec validation and display results with a TUI reporter.
    ///
    /// Validates all specs in the current workspace against the configured
    /// rules and displays a summary report in the terminal.
    Validate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_init() {
        let cli = Cli::try_parse_from(["airsspec", "init"]).expect("should parse init command");
        assert!(
            matches!(cli.command, Commands::Init),
            "expected Commands::Init"
        );
    }

    #[test]
    fn test_parse_mcp_default() {
        let cli = Cli::try_parse_from(["airsspec", "mcp"]).expect("should parse mcp command");
        match cli.command {
            Commands::Mcp { debug } => {
                assert!(!debug, "debug should default to false");
            }
            _ => panic!("expected Commands::Mcp"),
        }
    }

    #[test]
    fn test_parse_mcp_debug() {
        let cli =
            Cli::try_parse_from(["airsspec", "mcp", "--debug"]).expect("should parse mcp --debug");
        match cli.command {
            Commands::Mcp { debug } => {
                assert!(debug, "debug should be true when --debug flag is passed");
            }
            _ => panic!("expected Commands::Mcp"),
        }
    }

    #[test]
    fn test_parse_validate() {
        let cli =
            Cli::try_parse_from(["airsspec", "validate"]).expect("should parse validate command");
        assert!(
            matches!(cli.command, Commands::Validate),
            "expected Commands::Validate"
        );
    }

    #[test]
    fn test_parse_no_command_fails() {
        let result = Cli::try_parse_from(["airsspec"]);
        assert!(
            result.is_err(),
            "parsing with no subcommand should return an error"
        );
    }

    #[test]
    fn test_parse_unknown_command_fails() {
        let result = Cli::try_parse_from(["airsspec", "unknown"]);
        assert!(
            result.is_err(),
            "parsing with unknown subcommand should return an error"
        );
    }
}
