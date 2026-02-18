//! # Validation Orchestration
//!
//! Provides the [`validate_workspace`] function that orchestrates end-to-end
//! workspace validation by loading specs and plans from the filesystem and
//! running all workspace validators.
//!
//! This module lives in `airsspec-mcp` (per ADR-002) because it performs
//! filesystem I/O operations. The validators themselves live in
//! `airsspec-core::validation::validators`.

mod runner;

pub use runner::validate_workspace;
