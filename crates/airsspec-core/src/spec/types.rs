//! Core `Spec` and `SpecMetadata` types.
//!
//! This module defines the main specification types used throughout the system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::category::Category;
use super::dependency::Dependency;
use super::id::SpecId;

/// Metadata associated with a specification.
///
/// Contains descriptive and organizational information about a spec,
/// including timestamps for tracking creation and modification.
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::{SpecMetadata, Category};
/// use chrono::Utc;
///
/// let metadata = SpecMetadata::new("User Authentication", "Implement user auth flow");
/// assert_eq!(metadata.title(), "User Authentication");
/// assert_eq!(metadata.category(), Category::Feature);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecMetadata {
    /// The title of the specification.
    title: String,

    /// Detailed description of what the spec covers.
    description: String,

    /// Category classifying the type of work.
    category: Category,

    /// Dependencies on other specifications.
    dependencies: Vec<Dependency>,

    /// Timestamp when the spec was created.
    created_at: DateTime<Utc>,

    /// Timestamp when the spec was last updated.
    updated_at: DateTime<Utc>,
}

impl SpecMetadata {
    /// Creates new metadata with the given title and description.
    ///
    /// Sets `created_at` and `updated_at` to the current time,
    /// category to the default (`Feature`), and an empty dependencies list.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the specification
    /// * `description` - Detailed description of the spec
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecMetadata;
    ///
    /// let metadata = SpecMetadata::new("Payment Integration", "Add Stripe payment flow");
    /// assert_eq!(metadata.title(), "Payment Integration");
    /// ```
    #[must_use]
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            title: title.into(),
            description: description.into(),
            category: Category::default(),
            dependencies: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Returns the title of the specification.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the description of the specification.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the category of the specification.
    #[must_use]
    pub fn category(&self) -> Category {
        self.category
    }

    /// Returns the dependencies of this specification.
    #[must_use]
    pub fn dependencies(&self) -> &[Dependency] {
        &self.dependencies
    }

    /// Returns when the specification was created.
    #[must_use]
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns when the specification was last updated.
    #[must_use]
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Sets the title of the specification.
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
        self.updated_at = Utc::now();
    }

    /// Sets the description of the specification.
    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
        self.updated_at = Utc::now();
    }

    /// Sets the category of the specification.
    pub fn set_category(&mut self, category: Category) {
        self.category = category;
        self.updated_at = Utc::now();
    }

    /// Adds a dependency to this specification.
    pub fn add_dependency(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
        self.updated_at = Utc::now();
    }

    /// Sets the dependencies of this specification, replacing any existing ones.
    pub fn set_dependencies(&mut self, dependencies: Vec<Dependency>) {
        self.dependencies = dependencies;
        self.updated_at = Utc::now();
    }

    /// Updates the `updated_at` timestamp to the current time.
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

/// A specification in the `AirsSpec` workflow.
///
/// A `Spec` represents a single unit of work in the spec-driven development process.
/// It contains a unique identifier, metadata describing the work, and the actual
/// content (typically Markdown documentation).
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::{Spec, SpecId, SpecMetadata};
///
/// let id = SpecId::new(1_737_734_400, "user-auth");
/// let metadata = SpecMetadata::new("User Authentication", "Implement user auth flow");
/// let content = "# User Authentication\n\nImplement OAuth2 login...";
///
/// let spec = Spec::new(id.clone(), metadata, content);
/// assert_eq!(spec.id(), &id);
/// assert_eq!(spec.metadata().title(), "User Authentication");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spec {
    /// Unique identifier for this specification.
    id: SpecId,

    /// Metadata describing the specification.
    metadata: SpecMetadata,

    /// The specification content (typically Markdown).
    content: String,
}

impl Spec {
    /// Creates a new specification with the given ID, metadata, and content.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the spec
    /// * `metadata` - Descriptive metadata
    /// * `content` - The spec content (typically Markdown)
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::{Spec, SpecId, SpecMetadata};
    ///
    /// let id = SpecId::new(1_737_734_400, "payment-flow");
    /// let metadata = SpecMetadata::new("Payment Flow", "Stripe integration");
    /// let spec = Spec::new(id, metadata, "# Payment\n\nDetails...");
    /// ```
    #[must_use]
    pub fn new(id: SpecId, metadata: SpecMetadata, content: impl Into<String>) -> Self {
        Self {
            id,
            metadata,
            content: content.into(),
        }
    }

    /// Returns a reference to the specification ID.
    #[must_use]
    pub fn id(&self) -> &SpecId {
        &self.id
    }

    /// Returns a reference to the specification metadata.
    #[must_use]
    pub fn metadata(&self) -> &SpecMetadata {
        &self.metadata
    }

    /// Returns a mutable reference to the specification metadata.
    pub fn metadata_mut(&mut self) -> &mut SpecMetadata {
        &mut self.metadata
    }

    /// Returns the specification content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Sets the specification content.
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
        self.metadata.touch();
    }

    /// Returns the title from metadata (convenience accessor).
    #[must_use]
    pub fn title(&self) -> &str {
        self.metadata.title()
    }

    /// Returns the description from metadata (convenience accessor).
    #[must_use]
    pub fn description(&self) -> &str {
        self.metadata.description()
    }

    /// Returns the category from metadata (convenience accessor).
    #[must_use]
    pub fn category(&self) -> Category {
        self.metadata.category()
    }

    /// Returns the dependencies from metadata (convenience accessor).
    #[must_use]
    pub fn dependencies(&self) -> &[Dependency] {
        self.metadata.dependencies()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::{DependencyKind, SpecError};

    #[test]
    fn test_metadata_new() {
        let metadata = SpecMetadata::new("Test Title", "Test Description");

        assert_eq!(metadata.title(), "Test Title");
        assert_eq!(metadata.description(), "Test Description");
        assert_eq!(metadata.category(), Category::Feature);
        assert!(metadata.dependencies().is_empty());
        assert!(metadata.created_at() <= Utc::now());
        assert_eq!(metadata.created_at(), metadata.updated_at());
    }

    #[test]
    fn test_metadata_setters() {
        let mut metadata = SpecMetadata::new("Initial", "Initial desc");
        let initial_updated = metadata.updated_at();

        // Small delay to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(10));

        metadata.set_title("Updated Title");
        assert_eq!(metadata.title(), "Updated Title");
        assert!(metadata.updated_at() > initial_updated);

        metadata.set_description("Updated desc");
        assert_eq!(metadata.description(), "Updated desc");

        metadata.set_category(Category::BugFix);
        assert_eq!(metadata.category(), Category::BugFix);
    }

    #[test]
    fn test_metadata_dependencies() {
        let mut metadata = SpecMetadata::new("Test", "Desc");
        let dep_id = SpecId::new(1_737_734_400, "dep-spec");
        let dep = Dependency::blocked_by(dep_id.clone());

        metadata.add_dependency(dep.clone());
        assert_eq!(metadata.dependencies().len(), 1);
        assert_eq!(metadata.dependencies()[0].spec_id, dep_id);
        assert_eq!(metadata.dependencies()[0].kind, DependencyKind::BlockedBy);

        // Test set_dependencies
        let new_deps = vec![
            Dependency::blocked_by(SpecId::new(1_737_734_401, "another")),
            Dependency::related_to(SpecId::new(1_737_734_402, "related")),
        ];
        metadata.set_dependencies(new_deps);
        assert_eq!(metadata.dependencies().len(), 2);
    }

    #[test]
    fn test_metadata_touch() {
        let mut metadata = SpecMetadata::new("Test", "Desc");
        let initial = metadata.updated_at();

        std::thread::sleep(std::time::Duration::from_millis(10));
        metadata.touch();

        assert!(metadata.updated_at() > initial);
    }

    #[test]
    fn test_spec_new() {
        let id = SpecId::new(1_737_734_400, "test-spec");
        let metadata = SpecMetadata::new("Test Spec", "A test specification");
        let content = "# Test\n\nThis is test content.";

        let spec = Spec::new(id.clone(), metadata, content);

        assert_eq!(spec.id(), &id);
        assert_eq!(spec.title(), "Test Spec");
        assert_eq!(spec.description(), "A test specification");
        assert_eq!(spec.category(), Category::Feature);
        assert_eq!(spec.content(), content);
        assert!(spec.dependencies().is_empty());
    }

    #[test]
    fn test_spec_set_content() {
        let id = SpecId::new(1_737_734_400, "test-spec");
        let metadata = SpecMetadata::new("Test", "Desc");
        let mut spec = Spec::new(id, metadata, "Initial content");

        let initial_updated = spec.metadata().updated_at();
        std::thread::sleep(std::time::Duration::from_millis(10));

        spec.set_content("Updated content");
        assert_eq!(spec.content(), "Updated content");
        assert!(spec.metadata().updated_at() > initial_updated);
    }

    #[test]
    fn test_spec_metadata_mut() {
        let id = SpecId::new(1_737_734_400, "test-spec");
        let metadata = SpecMetadata::new("Test", "Desc");
        let mut spec = Spec::new(id, metadata, "Content");

        spec.metadata_mut().set_title("New Title");
        assert_eq!(spec.title(), "New Title");

        spec.metadata_mut().set_category(Category::Enhancement);
        assert_eq!(spec.category(), Category::Enhancement);
    }

    #[test]
    fn test_spec_clone() {
        let id = SpecId::new(1_737_734_400, "test-spec");
        let metadata = SpecMetadata::new("Test", "Desc");
        let spec = Spec::new(id, metadata, "Content");

        let cloned = spec.clone();
        assert_eq!(spec, cloned);
    }

    #[test]
    fn test_metadata_serde_roundtrip() {
        let mut metadata = SpecMetadata::new("Serde Test", "Testing serialization");
        metadata.set_category(Category::BugFix);
        metadata.add_dependency(Dependency::blocked_by(SpecId::new(1_737_734_400, "dep")));

        let json = serde_json::to_string(&metadata).unwrap();
        let parsed: SpecMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(metadata.title(), parsed.title());
        assert_eq!(metadata.description(), parsed.description());
        assert_eq!(metadata.category(), parsed.category());
        assert_eq!(metadata.dependencies().len(), parsed.dependencies().len());
    }

    #[test]
    fn test_spec_serde_roundtrip() {
        let id = SpecId::new(1_737_734_400, "serde-test");
        let metadata = SpecMetadata::new("Serde Spec", "Testing spec serialization");
        let spec = Spec::new(id, metadata, "# Content\n\nMarkdown content here.");

        let json = serde_json::to_string(&spec).unwrap();
        let parsed: Spec = serde_json::from_str(&json).unwrap();

        assert_eq!(spec.id(), parsed.id());
        assert_eq!(spec.title(), parsed.title());
        assert_eq!(spec.content(), parsed.content());
    }

    #[test]
    fn test_spec_with_string_content() {
        let id = SpecId::new(1_737_734_400, "string-test");
        let metadata = SpecMetadata::new("String Test", "Testing String content");
        let content = String::from("Dynamic content");

        let spec = Spec::new(id, metadata, content);
        assert_eq!(spec.content(), "Dynamic content");
    }

    // This test verifies the SpecError import is working (used in builder)
    #[test]
    fn test_spec_error_available() {
        let err = SpecError::MissingField("title".to_string());
        assert!(err.to_string().contains("title"));
    }
}
