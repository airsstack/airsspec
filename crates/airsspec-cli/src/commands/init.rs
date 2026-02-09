//! # Init Command Handler
//!
//! Handles the `airsspec init` subcommand, which initializes a new `AirsSpec`
//! workspace in the current directory.
//!
//! ## Flow
//!
//! 1. Get current working directory
//! 2. Check if workspace already exists (error if so)
//! 3. Launch TUI wizard for project configuration
//! 4. Create workspace using `FileSystemWorkspaceProvider`
//! 5. Print success message with next steps
//!
//! ## Cancellation
//!
//! If the user cancels the wizard (Esc or Ctrl+C), the command prints
//! "Setup cancelled." and exits normally (no error).

// Layer 1: Standard library
use std::env;

// Layer 2: External crates
use anyhow::Context;

// Layer 3: Internal crates/modules
use airsspec_core::workspace::{ProjectConfig, WorkspaceProvider};
use airsspec_mcp::FileSystemWorkspaceProvider;
use airsspec_tui::run_init_wizard;

/// Run the workspace initialization command.
///
/// Launches the interactive TUI wizard that guides the user through creating
/// a new `AirsSpec` workspace, including directory structure and configuration
/// files.
///
/// # Errors
///
/// Returns an error if:
/// - The current working directory cannot be determined
/// - A workspace already exists in the current directory
/// - The TUI wizard encounters a terminal I/O failure
/// - The workspace directory or config file cannot be created
#[expect(
    clippy::unused_async,
    reason = "async signature established in Phase 3.1 for consistency with other command handlers"
)]
pub async fn run() -> anyhow::Result<()> {
    let cwd = env::current_dir().context("failed to determine current directory")?;
    let provider = FileSystemWorkspaceProvider::new();

    // Check if workspace already exists before launching the wizard
    if provider.exists(&cwd) {
        anyhow::bail!(
            "workspace already exists at {}",
            cwd.join(".airsspec").display()
        );
    }

    // Run the TUI wizard to collect project configuration
    let result = run_init_wizard().context("TUI wizard failed")?;

    // Handle wizard cancellation (not an error)
    let Some(wizard_result) = result else {
        println!("Setup cancelled.");
        return Ok(());
    };

    // Create workspace from wizard result
    let config = ProjectConfig::new(
        &wizard_result.project_name,
        &wizard_result.project_description,
    );

    let info = provider
        .initialize(&cwd, &config)
        .context("failed to create workspace")?;

    // Print success message
    println!();
    println!("Workspace created successfully!");
    println!();
    println!("  Project:  {}", info.config().name());
    println!("  Location: {}", info.airsspec_dir().display());
    println!();
    println!("Next steps:");
    println!("  Start the MCP server:  airsspec mcp");
    println!("  Run validation:        airsspec validate");
    println!();

    Ok(())
}
