//! # Workspace Domain Module
//!
//! This module contains all types and logic related to workspace management.
//!
//! ## Types
//!
//! - [`ProjectConfig`] - Project configuration stored in config.toml
//! - [`WorkspaceInfo`] - Information about a discovered workspace
//! - [`WorkspaceError`] - Domain-specific errors
//! - [`WorkspaceProvider`] - Trait for workspace operations
//!
//! ## Architecture
//!
//! Following the modular monolith pattern (ADR-002), all workspace-related
//! code lives in this module. Each type has its own file for clarity.
//!
//! ## Example
//!
//! ```
//! use airsspec_core::workspace::{ProjectConfig, WorkspaceInfo};
//! use std::path::PathBuf;
//!
//! // Create a project configuration
//! let config = ProjectConfig::new("My Project", "A sample project");
//!
//! // Create workspace info (normally done by WorkspaceProvider)
//! let info = WorkspaceInfo::new(PathBuf::from("/project"), config);
//!
//! assert_eq!(info.config().name(), "My Project");
//! ```

mod config;
mod error;
mod info;
mod provider;

pub use config::{ProjectConfig, ProjectInfo, SpecDefaults};
pub use error::WorkspaceError;
pub use info::WorkspaceInfo;
pub use provider::{WorkspaceProvider, WorkspaceProviderExt};
