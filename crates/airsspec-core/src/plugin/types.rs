//! Core plugin type definitions.
//!
//! This module defines plugin management types including plugin manifests,
//! overlay specifications, and resolution orders.

// Layer 1: Standard library imports
use std::path::PathBuf;

// Layer 2: Third-party crate imports
use serde::{Deserialize, Serialize};

// Layer 3: Internal module imports
use crate::agent::types::AgentId;

/// Plugin manifest.
///
/// Defines metadata for a plugin, including its name, version, description,
/// and the list of overlays it provides.
///
/// # Fields
///
/// * `name` - Unique name for the plugin
/// * `version` - Semantic version of the plugin
/// * `description` - Optional description of the plugin's purpose
/// * `overlays` - List of overlay specifications provided by this plugin
///
/// # Examples
///
/// ```rust
/// use airsspec_core::plugin::types::{PluginManifest, OverlaySpec};
/// use airsspec_core::agent::types::AgentId;
/// use std::path::PathBuf;
///
/// let manifest = PluginManifest {
///     name: "custom-tools".to_string(),
///     version: "1.0.0".to_string(),
///     description: Some("Custom tool extensions".to_string()),
///     overlays: vec![
///         OverlaySpec {
///             target: AgentId::new("builder"),
///             path: PathBuf::from("overlays/builder.md"),
///         },
///     ],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Unique name for the plugin.
    pub name: String,

    /// Semantic version of the plugin.
    pub version: String,

    /// Optional description of the plugin's purpose.
    pub description: Option<String>,

    /// List of overlay specifications provided by this plugin.
    pub overlays: Vec<OverlaySpec>,
}

/// Overlay specification.
///
/// Defines a prompt overlay that extends or modifies an agent's behavior.
///
/// # Fields
///
/// * `target` - The agent ID this overlay applies to
/// * `path` - Path to the overlay content file
///
/// # Examples
///
/// ```rust
/// use airsspec_core::plugin::types::OverlaySpec;
/// use airsspec_core::agent::types::AgentId;
/// use std::path::PathBuf;
///
/// let overlay = OverlaySpec {
///     target: AgentId::new("builder"),
///     path: PathBuf::from("overlays/builder.md"),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlaySpec {
    /// The agent ID this overlay applies to.
    pub target: AgentId,

    /// Path to the overlay content file.
    pub path: PathBuf,
}

/// Resolution order for prompt stacking.
///
/// Defines how prompts are stacked when applying overlays from multiple sources
/// (core, plugins, user).
///
/// # Variants
///
/// * `CoreFirst` - Core prompts first, then plugins, then user overrides
/// * `PluginFirst` - Plugin prompts first, then core, then user overrides
/// * `UserFirst` - User prompts first, then plugins, then core fallback
///
/// # Examples
///
/// ```rust
/// use airsspec_core::plugin::types::ResolutionOrder;
///
/// let order = ResolutionOrder::CoreFirst;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionOrder {
    /// Core prompts first, then plugins, then user overrides.
    ///
    /// This is the default resolution order, ensuring that core behavior
    /// is respected, plugins can extend it, and users have final say.
    CoreFirst,

    /// Plugin prompts first, then core, then user overrides.
    ///
    /// Useful when plugin functionality should take precedence over
    /// core behavior.
    PluginFirst,

    /// User prompts first, then plugins, then core fallback.
    ///
    /// Useful when user customizations should take precedence.
    UserFirst,
}

impl ResolutionOrder {
    /// Returns a string representation of the resolution order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::plugin::types::ResolutionOrder;
    ///
    /// assert_eq!(ResolutionOrder::CoreFirst.as_str(), "core_first");
    /// ```
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::CoreFirst => "core_first",
            Self::PluginFirst => "plugin_first",
            Self::UserFirst => "user_first",
        }
    }

    /// Creates a resolution order from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string representation
    ///
    /// # Returns
    ///
    /// The corresponding `ResolutionOrder` variant, or `None` if not recognized
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::plugin::types::ResolutionOrder;
    ///
    /// assert_eq!(ResolutionOrder::try_from_str("core_first"), Some(ResolutionOrder::CoreFirst));
    /// assert_eq!(ResolutionOrder::try_from_str("invalid"), None);
    /// ```
    #[must_use]
    pub fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "core_first" => Some(Self::CoreFirst),
            "plugin_first" => Some(Self::PluginFirst),
            "user_first" => Some(Self::UserFirst),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manifest() {
        let manifest = PluginManifest {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test plugin".to_string()),
            overlays: vec![],
        };

        assert_eq!(manifest.name, "test-plugin");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.description, Some("Test plugin".to_string()));
    }

    #[test]
    fn test_overlay_spec() {
        let overlay = OverlaySpec {
            target: AgentId::new("builder"),
            path: PathBuf::from("overlays/builder.md"),
        };

        assert_eq!(overlay.target.as_str(), "builder");
        assert_eq!(overlay.path, PathBuf::from("overlays/builder.md"));
    }

    #[test]
    fn test_resolution_order_as_str() {
        assert_eq!(ResolutionOrder::CoreFirst.as_str(), "core_first");
        assert_eq!(ResolutionOrder::PluginFirst.as_str(), "plugin_first");
        assert_eq!(ResolutionOrder::UserFirst.as_str(), "user_first");
    }

    #[test]
    fn test_resolution_order_try_from_str() {
        assert_eq!(
            ResolutionOrder::try_from_str("core_first"),
            Some(ResolutionOrder::CoreFirst)
        );
        assert_eq!(
            ResolutionOrder::try_from_str("plugin_first"),
            Some(ResolutionOrder::PluginFirst)
        );
        assert_eq!(
            ResolutionOrder::try_from_str("user_first"),
            Some(ResolutionOrder::UserFirst)
        );
        assert_eq!(ResolutionOrder::try_from_str("invalid"), None);
    }

    #[test]
    fn test_resolution_order_partial_eq() {
        assert_eq!(ResolutionOrder::CoreFirst, ResolutionOrder::CoreFirst);
        assert_ne!(ResolutionOrder::CoreFirst, ResolutionOrder::PluginFirst);
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_clone_plugin_manifest() {
        let manifest = PluginManifest {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test plugin".to_string()),
            overlays: vec![],
        };

        let cloned = manifest.clone();
        assert_eq!(cloned.name, "test-plugin");
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_clone_overlay_spec() {
        let overlay = OverlaySpec {
            target: AgentId::new("builder"),
            path: PathBuf::from("overlays/builder.md"),
        };

        let cloned = overlay.clone();
        assert_eq!(cloned.target.as_str(), "builder");
    }

    #[test]
    fn test_copy_resolution_order() {
        let order = ResolutionOrder::CoreFirst;
        let copied = order; // ResolutionOrder is Copy

        assert_eq!(copied, ResolutionOrder::CoreFirst);
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_serialization_plugin_manifest() {
        let manifest = PluginManifest {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test plugin".to_string()),
            overlays: vec![],
        };

        let json = serde_json::to_string(&manifest).expect("Failed to serialize");
        assert!(json.contains("test-plugin"));

        let deserialized: PluginManifest =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.name, manifest.name);
        assert_eq!(deserialized.version, manifest.version);
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_serialization_overlay_spec() {
        let overlay = OverlaySpec {
            target: AgentId::new("builder"),
            path: PathBuf::from("overlays/builder.md"),
        };

        let json = serde_json::to_string(&overlay).expect("Failed to serialize");
        assert!(json.contains("builder"));

        let deserialized: OverlaySpec = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.target.as_str(), "builder");
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_serialization_resolution_order() {
        let order = ResolutionOrder::CoreFirst;

        let json = serde_json::to_string(&order).expect("Failed to serialize");
        assert!(json.contains("core_first"));

        let deserialized: ResolutionOrder =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, ResolutionOrder::CoreFirst);
    }
}
