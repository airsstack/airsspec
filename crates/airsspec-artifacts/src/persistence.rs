//! Artifact persistence implementation using JSONL format.
//!
//! This module provides the `JsonlPersistence` implementation of the `ArtifactStore` trait,
//! which handles reading and writing artifacts with YAML frontmatter.

// Layer 1: Standard library imports
use std::path::Path;

// Layer 2: Third-party crate imports
use async_trait::async_trait;
use serde::de::DeserializeOwned;

// Layer 3: Internal module imports
use airsspec_core::artifact::traits::ArtifactStore;
use airsspec_core::error::ArtifactError;

/// Frontmatter delimiter marker.
const FRONTMATTER_DELIMITER: &str = "---";

/// JSONL-based artifact persistence implementation.
///
/// This implementation handles artifacts with YAML frontmatter between `---` markers
/// followed by a markdown body.
///
/// # File Format
///
/// Artifacts are stored in the following format:
///
/// ```text
/// ---
/// id: RFC-001
/// title: Example RFC
/// status: draft
/// ---
///
/// # Content here
///
/// The markdown body content.
/// ```
///
/// # Examples
///
/// ```rust,ignore
/// use airsspec_artifacts::persistence::JsonlPersistence;
/// use std::path::Path;
///
/// let store = JsonlPersistence::new();
/// let path = Path::new("test.md");
///
/// // Write an artifact
/// store.write(path, "---\ntitle: Test\n---\n# Content").await?;
///
/// // Read it back
/// let content = store.read(path).await?;
/// ```
#[derive(Debug)]
pub struct JsonlPersistence;

impl JsonlPersistence {
    /// Creates a new `JsonlPersistence` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_artifacts::persistence::JsonlPersistence;
    ///
    /// let store = JsonlPersistence::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Extracts the frontmatter (YAML) from the content.
    ///
    /// # Arguments
    ///
    /// * `content` - The full content including frontmatter
    ///
    /// # Returns
    ///
    /// The frontmatter YAML string if present, or `None` if no frontmatter is found.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let content = "---\ntitle: Test\n---\n# Body";
    /// let frontmatter = JsonlPersistence::extract_frontmatter(content)?;
    /// ```
    #[must_use]
    pub fn extract_frontmatter(content: &str) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();

        // Find the first --- delimiter
        let start_idx = lines
            .iter()
            .position(|line| *line == FRONTMATTER_DELIMITER)?;

        // Find the second --- delimiter after the first
        let end_idx = lines[start_idx + 1..]
            .iter()
            .position(|line| *line == FRONTMATTER_DELIMITER)?
            + start_idx
            + 1;

        if end_idx <= start_idx + 1 {
            // No content between delimiters
            return Some(String::new());
        }

        // Extract lines between delimiters
        let frontmatter_lines = &lines[start_idx + 1..end_idx];
        Some(frontmatter_lines.join("\n"))
    }

    /// Extracts the body (markdown) from the content.
    ///
    /// # Arguments
    ///
    /// * `content` - The full content including frontmatter
    ///
    /// # Returns
    ///
    /// The body content (everything after the frontmatter).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let content = "---\ntitle: Test\n---\n# Body";
    /// let body = JsonlPersistence::extract_body(content)?;
    /// ```
    #[must_use]
    pub fn extract_body(content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();

        // Find the first --- delimiter
        let Some(start_idx) = lines.iter().position(|line| *line == FRONTMATTER_DELIMITER) else {
            // No frontmatter found, return entire content
            return content.to_string();
        };

        // Find the second --- delimiter after the first
        let Some(rel_end_idx) = lines[start_idx + 1..]
            .iter()
            .position(|line| *line == FRONTMATTER_DELIMITER)
        else {
            // No closing delimiter found, return empty body
            return String::new();
        };

        let end_idx = rel_end_idx + start_idx + 1;

        if end_idx >= lines.len() - 1 {
            // No content after frontmatter
            return String::new();
        }

        // Extract lines after the second delimiter
        let body_lines = &lines[end_idx + 1..];
        body_lines.join("\n")
    }

    /// Parses YAML frontmatter into a typed structure.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type to deserialize into (must implement `DeserializeOwned`)
    ///
    /// # Arguments
    ///
    /// * `content` - The full content including frontmatter
    ///
    /// # Returns
    ///
    /// The deserialized frontmatter structure.
    ///
    /// # Errors
    ///
    /// Returns an `ArtifactError` if the frontmatter cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct Metadata {
    ///     title: String,
    ///     status: String,
    /// }
    ///
    /// let content = "---\ntitle: Test\nstatus: draft\n---\n# Body";
    /// let metadata: Metadata = JsonlPersistence::parse_frontmatter(content)?;
    /// ```
    pub fn parse_frontmatter<T: DeserializeOwned>(content: &str) -> Result<T, ArtifactError> {
        let Some(frontmatter) = Self::extract_frontmatter(content) else {
            return Err(ArtifactError::Storage(
                "No frontmatter found in content".to_string(),
            ));
        };

        serde_yaml::from_str(&frontmatter)
            .map_err(|e| ArtifactError::Storage(format!("Failed to parse frontmatter YAML: {e}")))
    }
}

impl Default for JsonlPersistence {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ArtifactStore for JsonlPersistence {
    async fn read(&self, path: &Path) -> Result<String, ArtifactError> {
        tokio::fs::read_to_string(path)
            .await
            .map_err(|e| ArtifactError::Storage(format!("Failed to read artifact: {e}")))
    }

    async fn write(&self, path: &Path, content: &str) -> Result<(), ArtifactError> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| ArtifactError::Storage(format!("Failed to create directory: {e}")))?;
        }

        tokio::fs::write(path, content)
            .await
            .map_err(|e| ArtifactError::Storage(format!("Failed to write artifact: {e}")))
    }

    async fn exists(&self, path: &Path) -> bool {
        tokio::fs::try_exists(path).await.unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonl_persistence_new() {
        let store = JsonlPersistence::new();
        // Test that the store is created successfully
        let _ = store;
    }

    #[test]
    #[allow(clippy::default_constructed_unit_structs)]
    fn test_jsonl_persistence_default() {
        let _store = JsonlPersistence::default();
        // Test that the store is created via Default trait
    }

    #[test]
    fn test_extract_frontmatter_valid() {
        let content = "---\ntitle: Test\nstatus: draft\n---\n# Body content";
        let frontmatter = JsonlPersistence::extract_frontmatter(content);

        assert!(frontmatter.is_some());
        assert_eq!(frontmatter, Some("title: Test\nstatus: draft".to_string()));
    }

    #[test]
    fn test_extract_frontmatter_empty() {
        let content = "---\n---\n# Body content";
        let frontmatter = JsonlPersistence::extract_frontmatter(content);

        assert!(frontmatter.is_some());
        assert_eq!(frontmatter, Some(String::new()));
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_parse_frontmatter_no_frontmatter() {
        let content = "# No frontmatter\nJust content";
        let result: Result<String, _> = JsonlPersistence::parse_frontmatter(content);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No frontmatter found")
        );
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_parse_frontmatter_invalid_yaml() {
        let content = "---\ninvalid: yaml: content:\n---\n# Body";
        let result: Result<String, _> = JsonlPersistence::parse_frontmatter(content);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to parse frontmatter YAML")
        );
    }

    #[test]
    fn test_extract_frontmatter_incomplete() {
        let content = "---\ntitle: Test\n# No closing delimiter";
        let frontmatter = JsonlPersistence::extract_frontmatter(content);

        assert!(frontmatter.is_none());
    }

    #[test]
    fn test_extract_body_with_frontmatter() {
        let content = "---\ntitle: Test\n---\n# Body content\nSome text";
        let body = JsonlPersistence::extract_body(content);

        assert_eq!(body, "# Body content\nSome text");
    }

    #[test]
    fn test_extract_body_no_frontmatter() {
        let content = "# Just body content\nNo frontmatter";
        let body = JsonlPersistence::extract_body(content);

        assert_eq!(body, "# Just body content\nNo frontmatter");
    }

    #[test]
    fn test_extract_body_empty_after_frontmatter() {
        let content = "---\ntitle: Test\n---";
        let body = JsonlPersistence::extract_body(content);

        assert_eq!(body, "");
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_parse_frontmatter_valid() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Metadata {
            title: String,
            status: String,
        }

        let content = "---\ntitle: Test\nstatus: draft\n---\n# Body";
        let result = JsonlPersistence::parse_frontmatter::<Metadata>(content);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Metadata {
                title: "Test".to_string(),
                status: "draft".to_string()
            }
        );
    }

    #[test]
    fn test_parse_frontmatter_empty() {
        let content = "---\n---\n# Body";
        let result: Result<String, _> = JsonlPersistence::parse_frontmatter(content);

        // Empty YAML should fail to parse as String
        assert!(result.is_err());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_artifact_store_write_read() {
        let store = JsonlPersistence::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test.md");

        let content = "---\ntitle: Test\n---\n# Body content";
        let write_result = store.write(&path, content).await;

        assert!(write_result.is_ok());
        assert!(store.exists(&path).await);

        let read_result = store.read(&path).await;
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_artifact_store_write_creates_directory() {
        let store = JsonlPersistence::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("nested/dir/test.md");

        let content = "---\ntitle: Test\n---\n# Body content";
        let write_result = store.write(&path, content).await;

        assert!(write_result.is_ok());
        assert!(store.exists(&path).await);
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_artifact_store_read_nonexistent() {
        let store = JsonlPersistence::new();
        let path = Path::new("/nonexistent/path.md");

        let result = store.read(path).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to read artifact")
        );
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_artifact_store_exists_true() {
        let store = JsonlPersistence::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test.md");

        tokio::fs::write(&path, "content").await.unwrap();

        assert!(store.exists(&path).await);
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_airsspec_error_from_artifact_error() {
        use airsspec_core::error::AirsspecError;

        let artifact_error = ArtifactError::Storage("test".to_string());
        let airsspec_error: AirsspecError = artifact_error.into();

        assert!(matches!(airsspec_error, AirsspecError::Artifact(_)));
    }
}
