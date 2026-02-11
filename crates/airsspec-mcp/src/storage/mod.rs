//! # Storage Implementations
//!
//! Provides filesystem-based implementations of traits defined in `airsspec-core`.
//!
//! Per ADR-002 (4-Crate Structure), all I/O operations live in the `airsspec-mcp`
//! crate. This module bridges the gap between the pure domain abstractions in
//! `airsspec-core` and the actual filesystem.
//!
//! ## Implementations
//!
//! - [`FileSystemWorkspaceProvider`] - Discovers and initializes workspaces on the filesystem
//! - [`FileSystemSpecStorage`] - Reads and writes spec YAML files
//! - [`FileSystemPlanStorage`] - Reads and writes plan YAML files

mod plan;
mod spec;
mod workspace;

pub use plan::FileSystemPlanStorage;
pub use spec::FileSystemSpecStorage;
pub use workspace::FileSystemWorkspaceProvider;
