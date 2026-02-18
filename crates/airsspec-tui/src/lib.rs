//! # airsspec-tui
//!
//! Terminal UI components for `AirsSpec`.
//!
//! This crate provides interactive terminal interfaces using `ratatui` and `crossterm`,
//! following an immediate-mode rendering approach where the UI is rebuilt from state
//! each frame.
//!
//! ## Architecture
//!
//! Per [ADR-002: 4-Crate Structure](../../.memory-bank/sub-projects/airsspec/docs/adr/adr-002-4-crate-structure.md),
//! this crate handles all terminal UI concerns:
//!
//! - Immediate-mode rendering (state passed in, UI rebuilt each frame)
//! - Elm Architecture (TEA) pattern for wizard state management
//! - Reusable widget components
//! - Centralized theme definitions
//! - No MCP knowledge (separation of concerns)
//!
//! ## Modules
//!
//! - [`theme`] - Color palette and style definitions
//! - [`widgets`] - Reusable input components ([`widgets::TextInput`])
//! - [`wizard`] - Multi-step wizard framework and init wizard
//! - [`reporter`] - Styled terminal validation report output
//!
//! ## Quick Start
//!
//! ```ignore
//! use airsspec_tui::run_init_wizard;
//!
//! let result = run_init_wizard()?;
//! match result {
//!     Some(config) => println!("Project: {}", config.project_name),
//!     None => println!("Cancelled"),
//! }
//! ```
//!
//! ## Dependencies
//!
//! - `airsspec-core` - Domain models and types
//! - `ratatui` - Terminal UI framework (v0.29)
//! - `crossterm` - Terminal manipulation (v0.28)

pub mod reporter;
pub mod theme;
pub mod widgets;
pub mod wizard;

pub use reporter::render_validation_report;
pub use wizard::{InitWizardResult, run_init_wizard};
