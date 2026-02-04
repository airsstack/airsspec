//! Validation context for workspace-level validation.
//!
//! TODO: Phase 4 - Extend this module when implementing workspace validators.

use std::path::{Path, PathBuf};

/// Context for workspace-level validation operations.
///
/// TODO: Phase 4 - This will be extended to include loaded specs, plans,
/// and configuration when implementing workspace-level validators.
///
/// This struct provides context that validators may need when validating
/// workspace-level concerns (directory structure, cross-spec dependencies, etc.).
///
/// **Note:** Domain validators (spec, plan) typically don't need this context
/// as they validate individual domain objects. This is primarily for
/// Phase 4 (Validation Engine) when implementing workspace validators.
///
/// # Examples
///
/// ```
/// use airsspec_core::validation::ValidationContext;
/// use std::path::PathBuf;
///
/// let context = ValidationContext::new(PathBuf::from("/project"));
/// assert_eq!(context.workspace_path().to_str(), Some("/project"));
/// ```
#[derive(Debug, Clone)]
pub struct ValidationContext {
    workspace_path: PathBuf,
    // TODO: Phase 4 - Add these fields when implementing workspace validators:
    // specs: Vec<Spec>,
    // plans: Vec<Plan>,
    // config: Option<ProjectConfig>,
}

impl ValidationContext {
    /// Creates a new validation context.
    #[must_use]
    pub fn new(workspace_path: PathBuf) -> Self {
        Self { workspace_path }
    }

    /// Returns the workspace root path.
    #[must_use]
    pub fn workspace_path(&self) -> &Path {
        &self.workspace_path
    }
}

/// Builder for creating `ValidationContext` with optional fields.
///
/// Currently simple, but can be extended in Phase 4 to include
/// loaded specs, plans, and configuration.
///
/// TODO: Phase 4 - Add builder methods for specs, plans, and config.
#[derive(Debug, Default)]
pub struct ValidationContextBuilder {
    workspace_path: Option<PathBuf>,
    // TODO: Phase 4 - Add these fields:
    // specs: Vec<Spec>,
    // plans: Vec<Plan>,
    // config: Option<ProjectConfig>,
}

impl ValidationContextBuilder {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the workspace path.
    #[must_use]
    pub fn workspace_path(mut self, path: PathBuf) -> Self {
        self.workspace_path = Some(path);
        self
    }

    /// Builds the `ValidationContext`.
    ///
    /// # Panics
    ///
    /// Panics if `workspace_path` is not set.
    #[must_use]
    pub fn build(self) -> ValidationContext {
        ValidationContext {
            workspace_path: self
                .workspace_path
                .expect("workspace_path is required"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_new() {
        let context = ValidationContext::new(PathBuf::from("/test"));
        assert_eq!(context.workspace_path(), Path::new("/test"));
    }

    #[test]
    fn test_context_builder() {
        let context = ValidationContextBuilder::new()
            .workspace_path(PathBuf::from("/project"))
            .build();
        assert_eq!(context.workspace_path(), Path::new("/project"));
    }

    #[test]
    #[should_panic(expected = "workspace_path is required")]
    fn test_builder_requires_path() {
        let _ = ValidationContextBuilder::new().build();
    }

    #[test]
    fn test_context_clone() {
        let original = ValidationContext::new(PathBuf::from("/test"));
        let cloned = original.clone();
        assert_eq!(original.workspace_path(), cloned.workspace_path());
    }

    #[test]
    fn test_workspace_path_returns_path_not_pathbuf() {
        let context = ValidationContext::new(PathBuf::from("/test"));
        // Verify it returns &Path (type inferred, no annotation needed)
        let path = context.workspace_path();
        assert_eq!(path.to_str(), Some("/test"));
    }

    #[test]
    fn test_builder_default() {
        let builder = ValidationContextBuilder::default();
        // Just verify it can be created
        assert!(builder.workspace_path.is_none());
    }
}
