//! # Filesystem Workspace Provider
//!
//! Implements the [`WorkspaceProvider`] trait for local filesystem operations.
//!
//! This provider creates and discovers `.airsspec/` workspace directories,
//! handling directory structure creation and `config.toml` serialization.
//!
//! ## Workspace Structure
//!
//! ```text
//! <project-root>/
//! └── .airsspec/
//!     ├── config.toml    # Project configuration (TOML format)
//!     ├── specs/         # Specification files
//!     └── logs/          # Session logs
//! ```
//!
//! ## Examples
//!
//! ```no_run
//! use std::path::Path;
//!
//! use airsspec_core::workspace::{ProjectConfig, WorkspaceProvider};
//! use airsspec_mcp::FileSystemWorkspaceProvider;
//!
//! let provider = FileSystemWorkspaceProvider::new();
//! let config = ProjectConfig::new("my-project", "A sample project");
//!
//! // Initialize a new workspace
//! let info = provider.initialize(Path::new("."), &config).unwrap();
//! println!("Workspace created at: {}", info.airsspec_dir().display());
//!
//! // Discover an existing workspace
//! let info = provider.discover(Path::new(".")).unwrap();
//! println!("Found project: {}", info.config().name());
//! ```

// Layer 1: Standard library
use std::fs;
use std::path::Path;

// Layer 3: Internal crates/modules
use airsspec_core::workspace::{ProjectConfig, WorkspaceError, WorkspaceInfo, WorkspaceProvider};

/// Filesystem-based workspace provider.
///
/// Implements [`WorkspaceProvider`] by creating and reading `.airsspec/`
/// directories on the local filesystem. All operations are synchronous
/// since the core trait is synchronous by design (no async runtime in
/// `airsspec-core`).
///
/// # Thread Safety
///
/// This type is `Send + Sync` (it holds no state). Multiple instances
/// can safely operate on different paths concurrently.
#[derive(Debug, Clone, Default)]
pub struct FileSystemWorkspaceProvider;

impl FileSystemWorkspaceProvider {
    /// Creates a new filesystem workspace provider.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

/// Name of the workspace directory.
const WORKSPACE_DIR: &str = ".airsspec";

/// Name of the configuration file within the workspace directory.
const CONFIG_FILE: &str = "config.toml";

/// Subdirectories created inside a new workspace.
const SUBDIRS: &[&str] = &["specs", "logs"];

impl WorkspaceProvider for FileSystemWorkspaceProvider {
    fn discover(&self, start: &Path) -> Result<WorkspaceInfo, WorkspaceError> {
        let mut current = start.to_path_buf();

        // Canonicalize the starting path to resolve symlinks and relative paths.
        // If canonicalization fails (e.g., path doesn't exist), fall back to the
        // original path and let the loop handle the error naturally.
        if let Ok(canonical) = current.canonicalize() {
            current = canonical;
        }

        loop {
            let config_path = current.join(WORKSPACE_DIR).join(CONFIG_FILE);

            if config_path.is_file() {
                let content = fs::read_to_string(&config_path)
                    .map_err(|e| WorkspaceError::Io(e.to_string()))?;
                let config: ProjectConfig = toml::from_str(&content)
                    .map_err(|e| WorkspaceError::InvalidConfig(e.to_string()))?;
                return Ok(WorkspaceInfo::new(current, config));
            }

            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => return Err(WorkspaceError::NotFound(start.to_path_buf())),
            }
        }
    }

    fn initialize(
        &self,
        path: &Path,
        config: &ProjectConfig,
    ) -> Result<WorkspaceInfo, WorkspaceError> {
        let workspace_dir = path.join(WORKSPACE_DIR);

        // Check if workspace already exists
        if workspace_dir.exists() {
            return Err(WorkspaceError::AlreadyExists(workspace_dir));
        }

        // Create workspace directory and subdirectories
        fs::create_dir_all(&workspace_dir)?;

        for subdir in SUBDIRS {
            fs::create_dir_all(workspace_dir.join(subdir))?;
        }

        // Write config file
        let config_content = toml::to_string_pretty(config)
            .map_err(|e| WorkspaceError::InvalidConfig(e.to_string()))?;
        let config_path = workspace_dir.join(CONFIG_FILE);
        fs::write(&config_path, config_content)?;

        Ok(WorkspaceInfo::new(path.to_path_buf(), config.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::TempDir;

    #[test]
    fn test_initialize_creates_workspace() {
        let temp = TempDir::new().unwrap();
        let provider = FileSystemWorkspaceProvider::new();
        let config = ProjectConfig::new("test-project", "A test project");

        let result = provider.initialize(temp.path(), &config);

        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.config().name(), "test-project");
        assert_eq!(info.config().description(), "A test project");

        // Verify directory structure
        assert!(temp.path().join(".airsspec").is_dir());
        assert!(temp.path().join(".airsspec/config.toml").is_file());
        assert!(temp.path().join(".airsspec/specs").is_dir());
        assert!(temp.path().join(".airsspec/logs").is_dir());
    }

    #[test]
    fn test_initialize_fails_if_exists() {
        let temp = TempDir::new().unwrap();
        let provider = FileSystemWorkspaceProvider::new();
        let config = ProjectConfig::new("test", "test");

        // First init succeeds
        provider.initialize(temp.path(), &config).unwrap();

        // Second init fails with AlreadyExists
        let result = provider.initialize(temp.path(), &config);
        assert!(matches!(result, Err(WorkspaceError::AlreadyExists(_))));
    }

    #[test]
    fn test_discover_finds_workspace() {
        let temp = TempDir::new().unwrap();
        let provider = FileSystemWorkspaceProvider::new();
        let config = ProjectConfig::new("discovered", "A discovered project");

        // Create workspace
        provider.initialize(temp.path(), &config).unwrap();

        // Discover from workspace root
        let result = provider.discover(temp.path());
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.config().name(), "discovered");
        assert_eq!(info.config().description(), "A discovered project");
    }

    #[test]
    fn test_discover_walks_up_tree() {
        let temp = TempDir::new().unwrap();
        let provider = FileSystemWorkspaceProvider::new();
        let config = ProjectConfig::new("parent-project", "A parent project");

        // Create workspace at root
        provider.initialize(temp.path(), &config).unwrap();

        // Create nested directory
        let nested = temp.path().join("src").join("deep").join("nested");
        fs::create_dir_all(&nested).unwrap();

        // Discover from nested directory
        let result = provider.discover(&nested);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().config().name(), "parent-project");
    }

    #[test]
    fn test_discover_not_found() {
        let temp = TempDir::new().unwrap();
        let provider = FileSystemWorkspaceProvider::new();

        let result = provider.discover(temp.path());
        assert!(matches!(result, Err(WorkspaceError::NotFound(_))));
    }

    #[test]
    fn test_config_toml_roundtrip() {
        let temp = TempDir::new().unwrap();
        let provider = FileSystemWorkspaceProvider::new();
        let config = ProjectConfig::new("roundtrip-test", "Testing serialization");

        // Initialize workspace (writes config.toml)
        provider.initialize(temp.path(), &config).unwrap();

        // Read back config.toml and verify
        let config_path = temp.path().join(".airsspec/config.toml");
        let content = fs::read_to_string(&config_path).unwrap();
        let parsed: ProjectConfig = toml::from_str(&content).unwrap();
        assert_eq!(parsed.name(), "roundtrip-test");
        assert_eq!(parsed.description(), "Testing serialization");
    }

    #[test]
    fn test_exists_true_when_workspace_present() {
        let temp = TempDir::new().unwrap();
        let provider = FileSystemWorkspaceProvider::new();
        let config = ProjectConfig::new("exists-test", "Testing exists");

        provider.initialize(temp.path(), &config).unwrap();

        assert!(provider.exists(temp.path()));
    }

    #[test]
    fn test_exists_false_when_no_workspace() {
        let temp = TempDir::new().unwrap();
        let provider = FileSystemWorkspaceProvider::new();

        assert!(!provider.exists(temp.path()));
    }

    #[test]
    fn test_discover_invalid_config_toml() {
        let temp = TempDir::new().unwrap();

        // Manually create .airsspec/ with invalid TOML content
        let airsspec_dir = temp.path().join(".airsspec");
        fs::create_dir_all(&airsspec_dir).unwrap();
        fs::write(airsspec_dir.join("config.toml"), "not valid { toml !!!").unwrap();

        let provider = FileSystemWorkspaceProvider::new();
        let result = provider.discover(temp.path());
        assert!(
            matches!(result, Err(WorkspaceError::InvalidConfig(_))),
            "expected InvalidConfig error, got: {result:?}"
        );
    }

    #[test]
    fn test_initialize_io_error_on_invalid_path() {
        // Use a path where directory creation is guaranteed to fail.
        // On Unix, creating a directory under /dev/null (a file, not a directory) fails.
        let invalid_path = Path::new("/dev/null/impossible");

        let provider = FileSystemWorkspaceProvider::new();
        let config = ProjectConfig::new("test", "test");
        let result = provider.initialize(invalid_path, &config);
        assert!(
            matches!(result, Err(WorkspaceError::Io(_))),
            "expected Io error for invalid path, got: {result:?}"
        );
    }

    #[test]
    fn test_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<FileSystemWorkspaceProvider>();
    }
}
