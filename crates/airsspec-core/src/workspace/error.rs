//! Workspace errors.

use std::path::PathBuf;

use thiserror::Error;

/// Errors that can occur during workspace operations.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum WorkspaceError {
    /// Workspace not found at or above the given path.
    #[error("workspace not found at or above: {0}")]
    NotFound(PathBuf),

    /// Workspace already exists at the given path.
    #[error("workspace already exists at: {0}")]
    AlreadyExists(PathBuf),

    /// Invalid workspace configuration.
    #[error("invalid workspace configuration: {0}")]
    InvalidConfig(String),

    /// I/O error (stored as string since `io::Error` doesn't impl Clone/Eq).
    #[error("I/O error: {0}")]
    Io(String),
}

impl From<std::io::Error> for WorkspaceError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_not_found() {
        let err = WorkspaceError::NotFound(PathBuf::from("/path/to/project"));
        assert_eq!(
            err.to_string(),
            "workspace not found at or above: /path/to/project"
        );
    }

    #[test]
    fn test_error_display_already_exists() {
        let err = WorkspaceError::AlreadyExists(PathBuf::from("/project"));
        assert_eq!(err.to_string(), "workspace already exists at: /project");
    }

    #[test]
    fn test_error_display_invalid_config() {
        let err = WorkspaceError::InvalidConfig("missing project name".to_string());
        assert_eq!(
            err.to_string(),
            "invalid workspace configuration: missing project name"
        );
    }

    #[test]
    fn test_error_display_io() {
        let err = WorkspaceError::Io("file not found".to_string());
        assert_eq!(err.to_string(), "I/O error: file not found");
    }

    #[test]
    fn test_error_clone() {
        let err = WorkspaceError::NotFound(PathBuf::from("/project"));
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let workspace_err: WorkspaceError = io_err.into();
        assert!(matches!(workspace_err, WorkspaceError::Io(_)));
        assert!(workspace_err.to_string().contains("file not found"));
    }

    #[test]
    fn test_error_equality() {
        let err1 = WorkspaceError::NotFound(PathBuf::from("/project"));
        let err2 = WorkspaceError::NotFound(PathBuf::from("/project"));
        let err3 = WorkspaceError::NotFound(PathBuf::from("/other"));
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }
}
