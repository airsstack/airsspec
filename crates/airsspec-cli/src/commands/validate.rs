//! # Validate Command Handler
//!
//! Handles the `airsspec validate` subcommand by running workspace validation
//! and displaying results with a styled TUI reporter.
//!
//! ## Flow
//!
//! 1. Determine the current working directory
//! 2. Run all workspace validators via [`airsspec_mcp::validate_workspace`]
//! 3. Render the validation report to stdout via [`airsspec_tui::render_validation_report`]
//! 4. Return `Ok(())` if validation passed (exit code 0) or `Err` if failed (exit code 1)
//!
//! ## Exit Codes
//!
//! | Code | Meaning |
//! |------|---------|
//! | 0    | Validation passed (no errors; warnings are OK) |
//! | 1    | Validation failed (errors present) or internal error |

// Layer 1: Standard library
use std::io;

// Layer 2: External crates
use anyhow::Context;

// Layer 3: Internal crates
use airsspec_mcp::validate_workspace;
use airsspec_tui::render_validation_report;

/// Run the workspace validation command.
///
/// Validates all specs in the current workspace against the configured
/// validation rules and displays a summary report using the TUI reporter.
///
/// # Flow
///
/// 1. Gets the current working directory
/// 2. Calls [`validate_workspace`] to run all validators
/// 3. Renders the report to stdout via [`render_validation_report`]
/// 4. Returns `Err` if validation found errors (maps to exit code 1 in `main()`)
///
/// # Errors
///
/// Returns an error if:
/// - The current working directory cannot be determined
/// - Writing the validation report to stdout fails
/// - Validation found errors (to trigger non-zero exit code)
pub async fn run() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().context("failed to determine current directory")?;

    let report = validate_workspace(&cwd).await;

    let mut stdout = io::stdout();
    render_validation_report(&report, &mut stdout).context("failed to write validation report")?;

    if !report.is_valid() {
        anyhow::bail!("validation failed with {} error(s)", report.error_count());
    }

    Ok(())
}
