//! ID generation utilities.
//!
//! Provides helpers for generating spec IDs following ADR-003 format.

use chrono::Utc;

use crate::spec::SpecId;

use super::slug;

/// Default slug used when the title produces an empty slug.
const DEFAULT_SLUG: &str = "untitled";

/// Ensures a slug is valid, returning a default if empty.
fn ensure_valid_slug(slug: String) -> String {
    if slug.is_empty() {
        DEFAULT_SLUG.to_string()
    } else {
        slug
    }
}

/// Generates a new [`SpecId`] from a title using the current timestamp.
///
/// This is a convenience function that combines timestamp generation
/// with slug generation.
///
/// # Arguments
///
/// * `title` - The spec title to generate an ID from
///
/// # Examples
///
/// ```
/// use airsspec_core::utils::id;
///
/// let spec_id = id::generate_spec_id("User Authentication");
/// assert!(!spec_id.slug().is_empty());
/// assert!(spec_id.timestamp() > 0);
/// ```
#[must_use]
pub fn generate_spec_id(title: &str) -> SpecId {
    let timestamp = Utc::now().timestamp();
    let slug_str = ensure_valid_slug(slug::generate_default(title));
    SpecId::new(timestamp, &slug_str)
}

/// Generates a [`SpecId`] with a specific timestamp (useful for testing).
///
/// # Arguments
///
/// * `timestamp` - Unix timestamp to use
/// * `title` - The spec title to generate slug from
///
/// # Examples
///
/// ```
/// use airsspec_core::utils::id;
///
/// let spec_id = id::generate_spec_id_with_timestamp(1_737_734_400, "Test Feature");
/// assert_eq!(spec_id.timestamp(), 1_737_734_400);
/// assert_eq!(spec_id.slug(), "test-feature");
/// ```
#[must_use]
pub fn generate_spec_id_with_timestamp(timestamp: i64, title: &str) -> SpecId {
    let slug_str = ensure_valid_slug(slug::generate_default(title));
    SpecId::new(timestamp, &slug_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_spec_id() {
        let id = generate_spec_id("User Authentication");
        assert_eq!(id.slug(), "user-authentication");
        assert!(id.timestamp() > 0);
    }

    #[test]
    fn test_generate_spec_id_timestamp() {
        let before = Utc::now().timestamp();
        let id = generate_spec_id("Test");
        let after = Utc::now().timestamp();

        assert!(id.timestamp() >= before);
        assert!(id.timestamp() <= after);
    }

    #[test]
    fn test_generate_spec_id_with_timestamp() {
        let id = generate_spec_id_with_timestamp(1_737_734_400, "Test Feature");
        assert_eq!(id.timestamp(), 1_737_734_400);
        assert_eq!(id.slug(), "test-feature");
        assert_eq!(id.as_str(), "1737734400-test-feature");
    }

    #[test]
    fn test_generate_with_empty_title() {
        let id = generate_spec_id("");
        assert_eq!(id.slug(), "untitled");
    }

    #[test]
    fn test_generate_with_special_chars_only() {
        let id = generate_spec_id("!!!###");
        assert_eq!(id.slug(), "untitled");
    }

    #[test]
    fn test_generated_id_is_valid() -> Result<(), crate::spec::SpecError> {
        let id = generate_spec_id("My Feature");
        // Should be parseable
        let parsed = SpecId::parse(id.as_str())?;
        assert_eq!(id, parsed);
        Ok(())
    }

    #[test]
    fn test_generate_with_spaces() {
        let id = generate_spec_id_with_timestamp(1_000_000, "  Multiple   Spaces  ");
        assert_eq!(id.slug(), "multiple-spaces");
    }

    #[test]
    fn test_generate_with_mixed_case() {
        let id = generate_spec_id_with_timestamp(1_000_000, "CamelCase Title");
        assert_eq!(id.slug(), "camelcase-title");
    }

    #[test]
    fn test_generate_with_numbers() {
        // Periods are stripped (not converted to hyphens)
        let id = generate_spec_id_with_timestamp(1_000_000, "Version 2.0");
        assert_eq!(id.slug(), "version-20");
    }

    #[test]
    fn test_generate_unique_ids() {
        // Generate two IDs with same title but different timestamps
        let id1 = generate_spec_id_with_timestamp(1_000_000, "Test");
        let id2 = generate_spec_id_with_timestamp(2_000_000, "Test");

        assert_ne!(id1, id2);
        assert_eq!(id1.slug(), id2.slug());
        assert_ne!(id1.timestamp(), id2.timestamp());
    }
}
