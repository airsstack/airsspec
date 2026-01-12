//! Plugin loading and prompt overlay types.
//!
//! This module defines the core types for plugin management, including plugin manifests,
//! overlay specifications, and resolution orders in the `AirsSpec` framework.

pub mod traits;
pub mod types;

// Public re-exports for convenience
pub use traits::{PluginLoader, PromptOverlay};
pub use types::{OverlaySpec, PluginManifest, ResolutionOrder};
