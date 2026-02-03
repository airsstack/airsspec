//! Builder pattern for constructing Spec instances.
//!
//! Per ADR-002 (modular monolith), the builder is in its own file.
//! This provides a fluent API for creating specs with validation on build.

use chrono::Utc;

use super::category::Category;
use super::dependency::Dependency;
use super::error::SpecError;
use super::id::SpecId;
use super::types::{Spec, SpecMetadata};

/// Builder for constructing [`Spec`] instances with validation.
///
/// Provides a fluent API for creating specifications. The `build()` method
/// validates all required fields and generates the spec ID from the title.
///
/// # Required Fields
///
/// - `title` - Must be set via `title()` method
///
/// # Optional Fields
///
/// - `description` - Defaults to empty string
/// - `category` - Defaults to `Category::Feature`
/// - `content` - Defaults to empty string
/// - `dependencies` - Defaults to empty vec
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::{SpecBuilder, Category};
///
/// let spec = SpecBuilder::new()
///     .title("User Authentication")
///     .description("Implement OAuth2 login flow")
///     .category(Category::Feature)
///     .content("# User Auth\n\nImplementation details...")
///     .build()
///     .unwrap();
///
/// assert_eq!(spec.title(), "User Authentication");
/// assert!(spec.id().slug().contains("user-authentication"));
/// ```
#[derive(Debug, Default)]
pub struct SpecBuilder {
    /// The title of the specification (required).
    title: Option<String>,

    /// The description of the specification.
    description: Option<String>,

    /// The category of the specification.
    category: Option<Category>,

    /// Dependencies on other specifications.
    dependencies: Vec<Dependency>,

    /// The content of the specification.
    content: Option<String>,
}

impl SpecBuilder {
    /// Creates a new `SpecBuilder` with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecBuilder;
    ///
    /// let builder = SpecBuilder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the title of the specification (required).
    ///
    /// The title is used to generate the spec ID slug.
    ///
    /// # Arguments
    ///
    /// * `title` - The title for the specification
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecBuilder;
    ///
    /// let builder = SpecBuilder::new().title("My Feature");
    /// ```
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the description of the specification.
    ///
    /// # Arguments
    ///
    /// * `description` - Detailed description of the spec
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecBuilder;
    ///
    /// let builder = SpecBuilder::new()
    ///     .title("Feature")
    ///     .description("Detailed description here");
    /// ```
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the category of the specification.
    ///
    /// # Arguments
    ///
    /// * `category` - The category classifying the type of work
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::{SpecBuilder, Category};
    ///
    /// let builder = SpecBuilder::new()
    ///     .title("Bug Fix")
    ///     .category(Category::BugFix);
    /// ```
    #[must_use]
    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    /// Adds a dependency to the specification.
    ///
    /// Can be called multiple times to add multiple dependencies.
    ///
    /// # Arguments
    ///
    /// * `dependency` - A dependency on another specification
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::{SpecBuilder, Dependency, SpecId};
    ///
    /// let dep_id = SpecId::new(1_737_734_400, "auth-system");
    /// let builder = SpecBuilder::new()
    ///     .title("User Profile")
    ///     .dependency(Dependency::blocked_by(dep_id));
    /// ```
    #[must_use]
    pub fn dependency(mut self, dependency: Dependency) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Adds multiple dependencies to the specification.
    ///
    /// # Arguments
    ///
    /// * `dependencies` - An iterator of dependencies
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::{SpecBuilder, Dependency, SpecId};
    ///
    /// let deps = vec![
    ///     Dependency::blocked_by(SpecId::new(1_737_734_400, "auth")),
    ///     Dependency::related_to(SpecId::new(1_737_734_401, "profile")),
    /// ];
    ///
    /// let builder = SpecBuilder::new()
    ///     .title("Dashboard")
    ///     .dependencies(deps);
    /// ```
    #[must_use]
    pub fn dependencies(mut self, dependencies: impl IntoIterator<Item = Dependency>) -> Self {
        self.dependencies.extend(dependencies);
        self
    }

    /// Sets the content of the specification.
    ///
    /// Typically Markdown documentation describing the spec in detail.
    ///
    /// # Arguments
    ///
    /// * `content` - The specification content
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecBuilder;
    ///
    /// let builder = SpecBuilder::new()
    ///     .title("Feature")
    ///     .content("# Feature\n\n## Overview\n\nThis feature...");
    /// ```
    #[must_use]
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Builds the specification, validating all required fields.
    ///
    /// Generates a `SpecId` from the current timestamp and a slug derived
    /// from the title.
    ///
    /// # Errors
    ///
    /// Returns `SpecError::MissingField` if the title is not set.
    /// Returns `SpecError::InvalidId` if the generated slug is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecBuilder;
    ///
    /// // Successful build
    /// let spec = SpecBuilder::new()
    ///     .title("User Auth")
    ///     .build()
    ///     .unwrap();
    ///
    /// // Missing title returns error
    /// let result = SpecBuilder::new().build();
    /// assert!(result.is_err());
    /// ```
    pub fn build(self) -> Result<Spec, SpecError> {
        // Validate required field: title
        let title = self
            .title
            .ok_or_else(|| SpecError::MissingField("title".to_string()))?;

        if title.is_empty() {
            return Err(SpecError::MissingField("title cannot be empty".to_string()));
        }

        // Generate slug from title
        let slug = generate_slug(&title);

        // Create SpecId with current timestamp
        let timestamp = Utc::now().timestamp();
        let id = SpecId::try_new(timestamp, &slug)?;

        // Build metadata with defaults for optional fields
        let mut metadata = SpecMetadata::new(title, self.description.unwrap_or_default());

        if let Some(category) = self.category {
            metadata.set_category(category);
        }

        if !self.dependencies.is_empty() {
            metadata.set_dependencies(self.dependencies);
        }

        // Build the spec
        let content = self.content.unwrap_or_default();
        Ok(Spec::new(id, metadata, content))
    }
}

/// Generates a URL-safe slug from a title.
///
/// Converts the title to lowercase, replaces spaces and special characters
/// with hyphens, and truncates to the maximum slug length.
///
/// # Arguments
///
/// * `title` - The title to convert to a slug
///
/// # Examples
///
/// ```ignore
/// let slug = generate_slug("User Authentication System");
/// assert_eq!(slug, "user-authentication-system");
/// ```
fn generate_slug(title: &str) -> String {
    let slug: String = title
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();

    // Collapse multiple hyphens and trim leading/trailing hyphens
    let mut result = String::with_capacity(slug.len());
    let mut prev_was_hyphen = true; // Start true to skip leading hyphens

    for c in slug.chars() {
        if c == '-' {
            if !prev_was_hyphen {
                result.push(c);
                prev_was_hyphen = true;
            }
        } else {
            result.push(c);
            prev_was_hyphen = false;
        }
    }

    // Remove trailing hyphen if present
    if result.ends_with('-') {
        result.pop();
    }

    // Truncate to max slug length
    if result.len() > SpecId::MAX_SLUG_LENGTH {
        // Find a good break point (don't cut mid-word if possible)
        let truncate_at = result[..SpecId::MAX_SLUG_LENGTH]
            .rfind('-')
            .unwrap_or(SpecId::MAX_SLUG_LENGTH);
        result.truncate(truncate_at);
    }

    // Handle edge case: empty result
    if result.is_empty() {
        result = "spec".to_string();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_minimal() {
        let spec = SpecBuilder::new().title("Test Spec").build().unwrap();

        assert_eq!(spec.title(), "Test Spec");
        assert!(spec.description().is_empty());
        assert_eq!(spec.category(), Category::Feature);
        assert!(spec.dependencies().is_empty());
        assert!(spec.content().is_empty());
    }

    #[test]
    fn test_builder_full() {
        let dep_id = SpecId::new(1_737_734_400, "dependency");
        let dep = Dependency::blocked_by(dep_id);

        let spec = SpecBuilder::new()
            .title("Full Spec")
            .description("A complete specification")
            .category(Category::BugFix)
            .dependency(dep)
            .content("# Full Spec\n\nContent here.")
            .build()
            .unwrap();

        assert_eq!(spec.title(), "Full Spec");
        assert_eq!(spec.description(), "A complete specification");
        assert_eq!(spec.category(), Category::BugFix);
        assert_eq!(spec.dependencies().len(), 1);
        assert_eq!(spec.content(), "# Full Spec\n\nContent here.");
    }

    #[test]
    fn test_builder_multiple_dependencies() {
        let deps = vec![
            Dependency::blocked_by(SpecId::new(1_737_734_400, "dep1")),
            Dependency::related_to(SpecId::new(1_737_734_401, "dep2")),
        ];

        let spec = SpecBuilder::new()
            .title("Multi Dep")
            .dependencies(deps)
            .build()
            .unwrap();

        assert_eq!(spec.dependencies().len(), 2);
    }

    #[test]
    fn test_builder_missing_title() {
        let result = SpecBuilder::new().build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, SpecError::MissingField(_)));
        assert!(err.to_string().contains("title"));
    }

    #[test]
    fn test_builder_empty_title() {
        let result = SpecBuilder::new().title("").build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, SpecError::MissingField(_)));
    }

    #[test]
    fn test_builder_generates_slug() {
        let spec = SpecBuilder::new()
            .title("User Authentication System")
            .build()
            .unwrap();

        let slug = spec.id().slug();
        assert_eq!(slug, "user-authentication-system");
    }

    #[test]
    fn test_generate_slug_simple() {
        assert_eq!(generate_slug("Hello World"), "hello-world");
    }

    #[test]
    fn test_generate_slug_special_chars() {
        assert_eq!(generate_slug("User Auth (OAuth2)"), "user-auth-oauth2");
    }

    #[test]
    fn test_generate_slug_multiple_spaces() {
        assert_eq!(generate_slug("A   B   C"), "a-b-c");
    }

    #[test]
    fn test_generate_slug_leading_trailing_special() {
        assert_eq!(generate_slug("--Hello--"), "hello");
    }

    #[test]
    fn test_generate_slug_numbers() {
        assert_eq!(generate_slug("Phase 2 Auth"), "phase-2-auth");
    }

    #[test]
    fn test_generate_slug_long_title() {
        let long_title = "This is a very long title that exceeds the maximum slug length limit";
        let slug = generate_slug(long_title);
        assert!(slug.len() <= SpecId::MAX_SLUG_LENGTH);
        // Should break at a hyphen if possible
        assert!(!slug.ends_with('-'));
    }

    #[test]
    fn test_generate_slug_empty() {
        assert_eq!(generate_slug(""), "spec");
    }

    #[test]
    fn test_generate_slug_only_special_chars() {
        assert_eq!(generate_slug("!@#$%"), "spec");
    }

    #[test]
    fn test_builder_default() {
        let builder = SpecBuilder::default();
        // Should be equivalent to new()
        let result = builder.build();
        assert!(result.is_err()); // No title set
    }

    #[test]
    fn test_builder_chain_order_independent() {
        // Order of method calls shouldn't matter
        let spec1 = SpecBuilder::new()
            .title("Test")
            .category(Category::Refactor)
            .description("Desc")
            .build()
            .unwrap();

        let spec2 = SpecBuilder::new()
            .description("Desc")
            .category(Category::Refactor)
            .title("Test")
            .build()
            .unwrap();

        assert_eq!(spec1.title(), spec2.title());
        assert_eq!(spec1.description(), spec2.description());
        assert_eq!(spec1.category(), spec2.category());
    }

    #[test]
    fn test_builder_debug() {
        let builder = SpecBuilder::new().title("Debug Test");
        let debug = format!("{builder:?}");
        assert!(debug.contains("SpecBuilder"));
        assert!(debug.contains("Debug Test"));
    }
}
