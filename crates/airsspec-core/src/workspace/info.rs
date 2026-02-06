//! Workspace information types.

use std::path::{Path, PathBuf};

use super::config::ProjectConfig;

/// Information about a discovered workspace.
///
/// Contains paths to important workspace locations and the loaded configuration.
///
/// # Examples
///
/// ```
/// use std::path::{Path, PathBuf};
/// use airsspec_core::workspace::{WorkspaceInfo, ProjectConfig};
///
/// let config = ProjectConfig::new("Test", "A test project");
/// let info = WorkspaceInfo::new(PathBuf::from("/project"), config);
///
/// assert_eq!(info.root(), Path::new("/project"));
/// assert_eq!(info.airsspec_dir(), Path::new("/project/.airsspec"));
/// assert_eq!(info.specs_dir(), Path::new("/project/.airsspec/specs"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceInfo {
    /// Root path of the workspace (parent of .airsspec/)
    root: PathBuf,
    /// Loaded project configuration
    config: ProjectConfig,
}

impl WorkspaceInfo {
    /// Creates a new workspace info.
    #[must_use]
    pub fn new(root: PathBuf, config: ProjectConfig) -> Self {
        Self { root, config }
    }

    /// Returns the workspace root path.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the path to the `.airsspec` directory.
    #[must_use]
    pub fn airsspec_dir(&self) -> PathBuf {
        self.root.join(".airsspec")
    }

    /// Returns the path to the specs directory.
    #[must_use]
    pub fn specs_dir(&self) -> PathBuf {
        self.airsspec_dir().join("specs")
    }

    /// Returns the path to the logs directory.
    #[must_use]
    pub fn logs_dir(&self) -> PathBuf {
        self.airsspec_dir().join("logs")
    }

    /// Returns the path to the config file.
    #[must_use]
    pub fn config_path(&self) -> PathBuf {
        self.airsspec_dir().join("config.toml")
    }

    /// Returns the loaded project configuration.
    #[must_use]
    pub fn config(&self) -> &ProjectConfig {
        &self.config
    }

    /// Consumes self and returns the configuration.
    #[must_use]
    pub fn into_config(self) -> ProjectConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> ProjectConfig {
        ProjectConfig::new("Test Project", "A test project")
    }

    #[test]
    fn test_workspace_info_new() {
        let config = sample_config();
        let info = WorkspaceInfo::new(PathBuf::from("/project"), config.clone());

        assert_eq!(info.root(), Path::new("/project"));
        assert_eq!(info.config().name(), "Test Project");
    }

    #[test]
    fn test_workspace_info_root() {
        let info = WorkspaceInfo::new(PathBuf::from("/my/project"), sample_config());
        assert_eq!(info.root(), Path::new("/my/project"));
    }

    #[test]
    fn test_workspace_info_airsspec_dir() {
        let info = WorkspaceInfo::new(PathBuf::from("/project"), sample_config());
        assert_eq!(info.airsspec_dir(), Path::new("/project/.airsspec"));
    }

    #[test]
    fn test_workspace_info_specs_dir() {
        let info = WorkspaceInfo::new(PathBuf::from("/project"), sample_config());
        assert_eq!(info.specs_dir(), Path::new("/project/.airsspec/specs"));
    }

    #[test]
    fn test_workspace_info_logs_dir() {
        let info = WorkspaceInfo::new(PathBuf::from("/project"), sample_config());
        assert_eq!(info.logs_dir(), Path::new("/project/.airsspec/logs"));
    }

    #[test]
    fn test_workspace_info_config_path() {
        let info = WorkspaceInfo::new(PathBuf::from("/project"), sample_config());
        assert_eq!(
            info.config_path(),
            Path::new("/project/.airsspec/config.toml")
        );
    }

    #[test]
    fn test_workspace_info_config() {
        let config = sample_config();
        let info = WorkspaceInfo::new(PathBuf::from("/project"), config.clone());
        assert_eq!(info.config(), &config);
    }

    #[test]
    fn test_workspace_info_into_config() {
        let config = sample_config();
        let info = WorkspaceInfo::new(PathBuf::from("/project"), config.clone());
        let extracted = info.into_config();
        assert_eq!(extracted, config);
    }

    #[test]
    fn test_workspace_info_clone() {
        let info = WorkspaceInfo::new(PathBuf::from("/project"), sample_config());
        let cloned = info.clone();
        assert_eq!(info, cloned);
    }

    #[test]
    fn test_workspace_info_eq() {
        let info1 = WorkspaceInfo::new(PathBuf::from("/project"), sample_config());
        let info2 = WorkspaceInfo::new(PathBuf::from("/project"), sample_config());
        let info3 = WorkspaceInfo::new(PathBuf::from("/other"), sample_config());

        assert_eq!(info1, info2);
        assert_ne!(info1, info3);
    }

    #[test]
    fn test_workspace_info_paths_with_relative_root() {
        let info = WorkspaceInfo::new(PathBuf::from("./relative/path"), sample_config());
        assert_eq!(info.root(), Path::new("./relative/path"));
        assert_eq!(info.airsspec_dir(), Path::new("./relative/path/.airsspec"));
    }
}
