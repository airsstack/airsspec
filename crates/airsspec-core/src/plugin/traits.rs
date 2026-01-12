//! Plugin loading and prompt overlay traits.
//!
//! This module defines the core traits for plugin management, including the `PluginLoader` trait
/// and `PromptOverlay` trait in the `AirsSpec` framework.
// Layer 1: Standard library imports
use std::path::Path;

// Layer 2: Third-party crate imports
use async_trait::async_trait;

// Layer 3: Internal module imports
use crate::agent::types::AgentId;
use crate::error::PluginError;

use super::types::{PluginManifest, ResolutionOrder};

/// Plugin loader trait.
///
/// Defines the interface for loading and discovering plugins from the filesystem.
/// Implementations of this trait are responsible for reading plugin manifests
/// from plugin directories and enumerating available plugins.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::plugin::traits::PluginLoader;
/// use airsspec_core::plugin::types::PluginManifest;
/// use airsspec_core::error::PluginError;
/// use async_trait::async_trait;
/// use std::path::Path;
///
/// struct MyLoader;
///
/// #[async_trait]
/// impl PluginLoader for MyLoader {
///     async fn load(&self, plugin_dir: &Path) -> Result<PluginManifest, PluginError> {
///         // Load plugin manifest from directory
///         todo!()
///     }
///
///     async fn list_plugins(&self, workspace: &Path) -> Result<Vec<PluginManifest>, PluginError> {
///         // Scan workspace for plugins and return manifests
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait PluginLoader: Send + Sync {
    /// Loads a plugin manifest from a plugin directory.
    ///
    /// # Arguments
    ///
    /// * `plugin_dir` - Path to the plugin directory containing a manifest
    ///
    /// # Returns
    ///
    /// The plugin manifest if successfully loaded
    async fn load(&self, plugin_dir: &Path) -> Result<PluginManifest, PluginError>;

    /// Lists all available plugins in a workspace.
    ///
    /// # Arguments
    ///
    /// * `workspace` - Path to the workspace to scan for plugins
    ///
    /// # Returns
    ///
    /// A vector of plugin manifests for all discovered plugins
    async fn list_plugins(&self, workspace: &Path) -> Result<Vec<PluginManifest>, PluginError>;
}

/// Prompt overlay trait.
///
/// Defines the interface for stacking and applying prompt overlays to agents.
/// Implementations of this trait are responsible for resolving and stacking
/// prompts from multiple sources (core, plugins, user) according to a resolution order.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::plugin::traits::PromptOverlay;
/// use airsspec_core::plugin::types::ResolutionOrder;
/// use airsspec_core::agent::types::AgentId;
///
/// struct MyOverlay;
///
/// impl PromptOverlay for MyOverlay {
///     fn stack_prompts(&self, agent_id: &AgentId, order: ResolutionOrder) -> String {
///         // Stack prompts according to resolution order
///         todo!()
///     }
/// }
/// ```
pub trait PromptOverlay: Send + Sync {
    /// Stacks prompts for an agent according to the specified resolution order.
    ///
    /// This method collects prompts from multiple sources (core, plugins, user)
    /// and combines them into a single prompt string based on the resolution order.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The agent ID to stack prompts for
    /// * `order` - The resolution order to use when stacking prompts
    ///
    /// # Returns
    ///
    /// A single prompt string with all applicable prompts stacked according to the order
    fn stack_prompts(&self, agent_id: &AgentId, order: ResolutionOrder) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::types::OverlaySpec;
    use std::path::PathBuf;

    // Mock plugin loader for testing
    struct MockPluginLoader;

    #[async_trait]
    impl PluginLoader for MockPluginLoader {
        async fn load(&self, _plugin_dir: &Path) -> Result<PluginManifest, PluginError> {
            Ok(PluginManifest {
                name: "test-plugin".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Test plugin".to_string()),
                overlays: vec![],
            })
        }

        async fn list_plugins(
            &self,
            _workspace: &Path,
        ) -> Result<Vec<PluginManifest>, PluginError> {
            Ok(vec![PluginManifest {
                name: "test-plugin".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Test plugin".to_string()),
                overlays: vec![],
            }])
        }
    }

    // Mock prompt overlay for testing
    struct MockPromptOverlay;

    impl PromptOverlay for MockPromptOverlay {
        fn stack_prompts(&self, _agent_id: &AgentId, _order: ResolutionOrder) -> String {
            "Stacked prompt".to_string()
        }
    }

    #[test]
    fn test_resolution_order_core_first() {
        let order = ResolutionOrder::CoreFirst;
        assert_eq!(order.as_str(), "core_first");
    }

    #[test]
    fn test_resolution_order_plugin_first() {
        let order = ResolutionOrder::PluginFirst;
        assert_eq!(order.as_str(), "plugin_first");
    }

    #[test]
    fn test_resolution_order_user_first() {
        let order = ResolutionOrder::UserFirst;
        assert_eq!(order.as_str(), "user_first");
    }

    #[test]
    fn test_resolution_order_copy() {
        let order = ResolutionOrder::CoreFirst;
        let copied = order; // ResolutionOrder is Copy
        assert_eq!(copied, ResolutionOrder::CoreFirst);
    }

    #[test]
    fn test_mock_prompt_overlay() {
        let overlay = MockPromptOverlay;
        let agent_id = AgentId::new("builder");

        let result = overlay.stack_prompts(&agent_id, ResolutionOrder::CoreFirst);
        assert_eq!(result, "Stacked prompt");
    }

    #[test]
    fn test_overlay_spec_target() {
        let overlay = OverlaySpec {
            target: AgentId::new("builder"),
            path: PathBuf::from("overlays/builder.md"),
        };

        assert_eq!(overlay.target.as_str(), "builder");
    }

    #[test]
    fn test_overlay_spec_path() {
        let overlay = OverlaySpec {
            target: AgentId::new("builder"),
            path: PathBuf::from("overlays/builder.md"),
        };

        assert_eq!(overlay.path, PathBuf::from("overlays/builder.md"));
    }

    #[tokio::test]
    #[allow(clippy::expect_used)]
    async fn test_mock_plugin_loader_load() {
        let loader = MockPluginLoader;
        let plugin_dir = PathBuf::from("test-plugin");

        let manifest = loader
            .load(&plugin_dir)
            .await
            .expect("Failed to load plugin");
        assert_eq!(manifest.name, "test-plugin");
        assert_eq!(manifest.version, "1.0.0");
    }

    #[tokio::test]
    #[allow(clippy::expect_used)]
    async fn test_mock_plugin_loader_list() {
        let loader = MockPluginLoader;
        let workspace = PathBuf::from("workspace");

        let manifests = loader
            .list_plugins(&workspace)
            .await
            .expect("Failed to list plugins");
        assert_eq!(manifests.len(), 1);
        assert_eq!(manifests[0].name, "test-plugin");
    }
}
