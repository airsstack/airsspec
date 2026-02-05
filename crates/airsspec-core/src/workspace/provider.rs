//! Workspace provider trait.
//!
//! Following project guidelines, we use generics (NOT `dyn` trait objects)
//! for static dispatch. The trait is synchronous as `airsspec-core` has no
//! I/O dependencies - actual I/O implementations live in `airsspec-mcp`.

use std::path::Path;

use super::config::ProjectConfig;
use super::error::WorkspaceError;
use super::info::WorkspaceInfo;

/// Trait for workspace discovery and initialization.
///
/// Implementations of this trait handle the actual filesystem operations
/// for finding and creating workspaces. This trait is synchronous; async
/// implementations can wrap it as needed in the `airsspec-mcp` crate.
///
/// # Thread Safety
///
/// All implementations must be `Send + Sync` for thread safety.
///
/// # Examples
///
/// ```ignore
/// // In airsspec-mcp crate:
/// use airsspec_core::workspace::{WorkspaceProvider, WorkspaceInfo, WorkspaceError};
/// use std::path::Path;
///
/// struct FileSystemWorkspace { /* ... */ }
///
/// impl WorkspaceProvider for FileSystemWorkspace {
///     fn discover(&self, start: &Path) -> Result<WorkspaceInfo, WorkspaceError> {
///         // Walk up from start looking for .airsspec/
///         todo!()
///     }
///     // ... other methods
/// }
/// ```
pub trait WorkspaceProvider: Send + Sync {
    /// Discovers a workspace by traversing up from the given path.
    ///
    /// Looks for a `.airsspec` directory starting from `start` and walking
    /// up to parent directories.
    ///
    /// # Arguments
    ///
    /// * `start` - The path to start searching from
    ///
    /// # Errors
    ///
    /// Returns `WorkspaceError::NotFound` if no workspace is found.
    fn discover(&self, start: &Path) -> Result<WorkspaceInfo, WorkspaceError>;

    /// Initializes a new workspace at the given path.
    ///
    /// Creates the `.airsspec` directory structure and config file.
    ///
    /// # Arguments
    ///
    /// * `path` - The root path for the new workspace
    /// * `config` - The project configuration to use
    ///
    /// # Errors
    ///
    /// Returns `WorkspaceError::AlreadyExists` if a workspace already exists.
    fn initialize(
        &self,
        path: &Path,
        config: &ProjectConfig,
    ) -> Result<WorkspaceInfo, WorkspaceError>;

    /// Checks if a workspace exists at the given path.
    ///
    /// Default implementation calls `discover` and checks for success.
    fn exists(&self, path: &Path) -> bool {
        self.discover(path).is_ok()
    }
}

/// Extension trait for [`WorkspaceProvider`] providing additional convenience methods.
pub trait WorkspaceProviderExt: WorkspaceProvider {
    /// Loads only the configuration from a workspace.
    ///
    /// Convenience method that discovers the workspace and extracts the config.
    ///
    /// # Errors
    ///
    /// Returns `WorkspaceError` if the workspace is not found.
    fn load_config(&self, start: &Path) -> Result<ProjectConfig, WorkspaceError> {
        let info = self.discover(start)?;
        Ok(info.into_config())
    }
}

// Blanket implementation
impl<T: WorkspaceProvider> WorkspaceProviderExt for T {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Mutex;

    use super::*;

    /// In-memory workspace provider for testing.
    struct MockWorkspaceProvider {
        workspaces: Mutex<Vec<WorkspaceInfo>>,
    }

    impl MockWorkspaceProvider {
        fn new() -> Self {
            Self {
                workspaces: Mutex::new(Vec::new()),
            }
        }

        fn add_workspace(&self, root: PathBuf, config: ProjectConfig) {
            let mut workspaces = self.workspaces.lock().unwrap();
            workspaces.push(WorkspaceInfo::new(root, config));
        }
    }

    impl WorkspaceProvider for MockWorkspaceProvider {
        fn discover(&self, start: &Path) -> Result<WorkspaceInfo, WorkspaceError> {
            let workspaces = self.workspaces.lock().unwrap();
            workspaces
                .iter()
                .find(|ws| start.starts_with(ws.root()))
                .cloned()
                .ok_or_else(|| WorkspaceError::NotFound(start.to_path_buf()))
        }

        fn initialize(
            &self,
            path: &Path,
            config: &ProjectConfig,
        ) -> Result<WorkspaceInfo, WorkspaceError> {
            let mut workspaces = self.workspaces.lock().unwrap();

            // Check if workspace already exists
            let exists = workspaces.iter().any(|ws| ws.root() == path);
            if exists {
                return Err(WorkspaceError::AlreadyExists(path.to_path_buf()));
            }

            let info = WorkspaceInfo::new(path.to_path_buf(), config.clone());
            workspaces.push(info.clone());

            Ok(info)
        }
    }

    #[test]
    fn test_provider_discover() -> Result<(), WorkspaceError> {
        let provider = MockWorkspaceProvider::new();
        let config = ProjectConfig::new("Test", "Test project");
        provider.add_workspace(PathBuf::from("/project"), config);

        let info = provider.discover(Path::new("/project/src"))?;
        assert_eq!(info.config().name(), "Test");
        Ok(())
    }

    #[test]
    fn test_provider_discover_not_found() {
        let provider = MockWorkspaceProvider::new();

        let result = provider.discover(Path::new("/unknown"));
        assert!(matches!(result, Err(WorkspaceError::NotFound(_))));
    }

    #[test]
    fn test_provider_initialize() -> Result<(), WorkspaceError> {
        let provider = MockWorkspaceProvider::new();
        let config = ProjectConfig::new("New Project", "A new project");

        let info = provider.initialize(Path::new("/new-project"), &config)?;
        assert_eq!(info.config().name(), "New Project");
        Ok(())
    }

    #[test]
    fn test_provider_initialize_already_exists() {
        let provider = MockWorkspaceProvider::new();
        let config = ProjectConfig::new("Test", "Test");
        provider.add_workspace(PathBuf::from("/project"), config.clone());

        let result = provider.initialize(Path::new("/project"), &config);
        assert!(matches!(result, Err(WorkspaceError::AlreadyExists(_))));
    }

    #[test]
    fn test_provider_exists_true() {
        let provider = MockWorkspaceProvider::new();
        let config = ProjectConfig::new("Test", "Test");
        provider.add_workspace(PathBuf::from("/project"), config);

        let result = provider.exists(Path::new("/project"));
        assert!(result);
    }

    #[test]
    fn test_provider_exists_false() {
        let provider = MockWorkspaceProvider::new();

        let result = provider.exists(Path::new("/unknown"));
        assert!(!result);
    }

    #[test]
    fn test_provider_ext_load_config() -> Result<(), WorkspaceError> {
        let provider = MockWorkspaceProvider::new();
        let config = ProjectConfig::new("Test Project", "Description");
        provider.add_workspace(PathBuf::from("/project"), config);

        let loaded_config = provider.load_config(Path::new("/project"))?;
        assert_eq!(loaded_config.name(), "Test Project");
        Ok(())
    }

    #[test]
    fn test_provider_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MockWorkspaceProvider>();
    }
}
