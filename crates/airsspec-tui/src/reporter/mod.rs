//! # Validation Reporter
//!
//! Provides styled terminal output for validation results.
//!
//! The reporter takes a [`ValidationReport`](airsspec_core::validation::ValidationReport)
//! and writes a colored, structured summary to any [`std::io::Write`] destination.
//! This is a one-shot display (not interactive) -- it formats and prints, then returns.
//!
//! ## Output Format
//!
//! Issues are grouped by severity (errors first, then warnings, then info)
//! with a summary status line at the end. Colors use the
//! [theme](crate::theme) palette for consistency with other TUI components.

mod validation;

pub use validation::render_validation_report;
