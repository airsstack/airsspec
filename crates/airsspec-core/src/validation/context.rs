//! Validation context for workspace-level validation.
//!
//! Provides the [`ValidationContext`] struct that carries all data needed
//! by workspace-level validators, and [`ValidationContextBuilder`] for
//! constructing contexts with optional fields.
//!
//! ## Dependency Inversion Principle
//!
//! `ValidationContext` is generic over its spec (`S`) and plan (`P`) types.
//! Validators constrain these via trait bounds ([`ValidatableSpec`],
//! [`ValidatablePlan`]) rather than depending on concrete domain types.
//! This ensures the validation framework never imports from domain modules.

use std::fmt;
use std::path::{Path, PathBuf};

/// Context for workspace-level validation operations.
///
/// Carries the workspace path and loaded domain data that validators need.
/// Generic over spec type `S` and plan type `P` to follow the Dependency
/// Inversion Principle -- validators constrain these via trait bounds.
///
/// Default type parameters `S = (), P = ()` allow creating a context with
/// only a workspace path (no specs or plans), which is sufficient for
/// validators like [`DirectoryStructureValidator`](super::validators::DirectoryStructureValidator).
///
/// # Examples
///
/// ```
/// use airsspec_core::validation::ValidationContext;
/// use std::path::PathBuf;
///
/// // Simple context with no specs/plans (uses default type parameters)
/// let context = ValidationContext::new(PathBuf::from("/project"));
/// assert_eq!(context.workspace_path().to_str(), Some("/project"));
/// assert!(context.specs().is_empty());
/// assert!(context.plans().is_empty());
/// ```
pub struct ValidationContext<S = (), P = ()> {
    workspace_path: PathBuf,
    specs: Vec<S>,
    plans: Vec<P>,
}

impl<S, P> ValidationContext<S, P> {
    /// Returns the workspace root path.
    #[must_use]
    pub fn workspace_path(&self) -> &Path {
        &self.workspace_path
    }

    /// Returns all loaded specifications.
    #[must_use]
    pub fn specs(&self) -> &[S] {
        &self.specs
    }

    /// Returns all loaded plans.
    #[must_use]
    pub fn plans(&self) -> &[P] {
        &self.plans
    }
}

// Convenience constructor for the default (no specs, no plans) case.
impl ValidationContext<(), ()> {
    /// Creates a new validation context with only a workspace path.
    ///
    /// Specs and plans default to empty. Use [`ValidationContextBuilder`]
    /// to construct a context with loaded domain data.
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
    #[must_use]
    pub fn new(workspace_path: PathBuf) -> Self {
        Self {
            workspace_path,
            specs: Vec::new(),
            plans: Vec::new(),
        }
    }
}

// Manual Clone: only require Clone on S, P when cloning the context.
impl<S: Clone, P: Clone> Clone for ValidationContext<S, P> {
    fn clone(&self) -> Self {
        Self {
            workspace_path: self.workspace_path.clone(),
            specs: self.specs.clone(),
            plans: self.plans.clone(),
        }
    }
}

// Manual Debug: only require Debug on S, P when debugging.
impl<S: fmt::Debug, P: fmt::Debug> fmt::Debug for ValidationContext<S, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidationContext")
            .field("workspace_path", &self.workspace_path)
            .field("specs", &self.specs)
            .field("plans", &self.plans)
            .finish()
    }
}

/// Builder for creating [`ValidationContext`] with typed specs and plans.
///
/// Allows constructing a `ValidationContext<S, P>` step by step. The type
/// parameters `S` and `P` are determined by which builder methods you call.
///
/// # Examples
///
/// ```
/// use airsspec_core::validation::ValidationContextBuilder;
/// use std::path::PathBuf;
///
/// // Build a context with no specs/plans (uses () defaults)
/// let context = ValidationContextBuilder::new()
///     .workspace_path(PathBuf::from("/project"))
///     .build();
/// assert_eq!(context.workspace_path().to_str(), Some("/project"));
/// ```
#[derive(Debug)]
pub struct ValidationContextBuilder<S = (), P = ()> {
    workspace_path: Option<PathBuf>,
    specs: Vec<S>,
    plans: Vec<P>,
}

impl Default for ValidationContextBuilder<(), ()> {
    fn default() -> Self {
        Self {
            workspace_path: None,
            specs: Vec::new(),
            plans: Vec::new(),
        }
    }
}

impl ValidationContextBuilder<(), ()> {
    /// Creates a new builder with default type parameters.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S, P> ValidationContextBuilder<S, P> {
    /// Sets the workspace path.
    #[must_use]
    pub fn workspace_path(mut self, path: PathBuf) -> Self {
        self.workspace_path = Some(path);
        self
    }

    /// Sets the loaded specifications, changing the spec type parameter.
    ///
    /// This method transforms `ValidationContextBuilder<S, P>` into
    /// `ValidationContextBuilder<S2, P>` where `S2` is the type of specs
    /// in the provided vector.
    #[must_use]
    pub fn specs<S2>(self, specs: Vec<S2>) -> ValidationContextBuilder<S2, P> {
        ValidationContextBuilder {
            workspace_path: self.workspace_path,
            specs,
            plans: self.plans,
        }
    }

    /// Sets the loaded plans, changing the plan type parameter.
    ///
    /// This method transforms `ValidationContextBuilder<S, P>` into
    /// `ValidationContextBuilder<S, P2>` where `P2` is the type of plans
    /// in the provided vector.
    #[must_use]
    pub fn plans<P2>(self, plans: Vec<P2>) -> ValidationContextBuilder<S, P2> {
        ValidationContextBuilder {
            workspace_path: self.workspace_path,
            specs: self.specs,
            plans,
        }
    }

    /// Builds the [`ValidationContext`].
    ///
    /// # Panics
    ///
    /// Panics if `workspace_path` is not set.
    #[must_use]
    pub fn build(self) -> ValidationContext<S, P> {
        ValidationContext {
            workspace_path: self.workspace_path.expect("workspace_path is required"),
            specs: self.specs,
            plans: self.plans,
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
    fn test_context_new_has_empty_defaults() {
        let context = ValidationContext::new(PathBuf::from("/test"));
        assert!(context.specs().is_empty());
        assert!(context.plans().is_empty());
    }

    #[test]
    fn test_context_builder_minimal() {
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
        let path = context.workspace_path();
        assert_eq!(path.to_str(), Some("/test"));
    }

    #[test]
    fn test_builder_default() {
        let builder = ValidationContextBuilder::default();
        assert!(builder.workspace_path.is_none());
    }

    #[test]
    fn test_builder_with_specs() {
        let context = ValidationContextBuilder::new()
            .workspace_path(PathBuf::from("/project"))
            .specs(vec!["spec-a", "spec-b"])
            .build();
        assert_eq!(context.specs().len(), 2);
        assert_eq!(context.specs()[0], "spec-a");
    }

    #[test]
    fn test_builder_with_plans() {
        let context = ValidationContextBuilder::new()
            .workspace_path(PathBuf::from("/project"))
            .plans(vec![1u32, 2u32, 3u32])
            .build();
        assert_eq!(context.plans().len(), 3);
    }

    #[test]
    fn test_builder_with_specs_and_plans() {
        let context = ValidationContextBuilder::new()
            .workspace_path(PathBuf::from("/project"))
            .specs(vec!["spec-a"])
            .plans(vec![42u32])
            .build();

        assert_eq!(context.workspace_path(), Path::new("/project"));
        assert_eq!(context.specs().len(), 1);
        assert_eq!(context.plans().len(), 1);
    }

    #[test]
    fn test_builder_type_transformation() {
        // Demonstrates that specs() and plans() change the builder's type parameters
        let builder = ValidationContextBuilder::new().workspace_path(PathBuf::from("/project"));
        // builder is ValidationContextBuilder<(), ()> at this point

        let builder = builder.specs(vec!["a", "b"]);
        // builder is now ValidationContextBuilder<&str, ()>

        let builder = builder.plans(vec![1u64, 2u64]);
        // builder is now ValidationContextBuilder<&str, u64>

        let context = builder.build();
        assert_eq!(context.specs().len(), 2);
        assert_eq!(context.plans().len(), 2);
    }

    #[test]
    fn test_context_debug() {
        let context = ValidationContext::new(PathBuf::from("/test"));
        let debug_str = format!("{context:?}");
        assert!(debug_str.contains("ValidationContext"));
        assert!(debug_str.contains("/test"));
    }

    #[test]
    fn test_context_with_typed_data_clone() {
        let context = ValidationContextBuilder::new()
            .workspace_path(PathBuf::from("/project"))
            .specs(vec!["a".to_string(), "b".to_string()])
            .plans(vec![1u32, 2u32])
            .build();
        let cloned = context.clone();
        assert_eq!(context.specs(), cloned.specs());
        assert_eq!(context.plans(), cloned.plans());
    }
}
