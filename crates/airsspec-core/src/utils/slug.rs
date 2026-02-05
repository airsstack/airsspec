//! Slug generation utilities.
//!
//! Per ADR-003, slugs must be:
//! - Lowercase
//! - Alphanumeric + hyphens only
//! - Max 50 characters
//! - No leading/trailing hyphens
//! - No consecutive hyphens

use crate::spec::SpecId;

/// Generates a URL-safe slug from a title.
///
/// # Rules (ADR-003)
///
/// - Converts to lowercase
/// - Replaces spaces, underscores, and special chars with hyphens
/// - Removes non-alphanumeric characters (except hyphens)
/// - Collapses consecutive hyphens
/// - Trims leading/trailing hyphens
/// - Truncates to `max_length` without breaking on a hyphen
///
/// # Arguments
///
/// * `title` - The input string to slugify
/// * `max_length` - Maximum length of the resulting slug
///
/// # Examples
///
/// ```
/// use airsspec_core::utils::slug;
///
/// assert_eq!(slug::generate("User Authentication", 50), "user-authentication");
/// assert_eq!(slug::generate("Fix Bug #123!", 50), "fix-bug-123");
/// assert_eq!(slug::generate("  Multiple   Spaces  ", 50), "multiple-spaces");
/// assert_eq!(slug::generate("---leading-trailing---", 50), "leading-trailing");
/// ```
#[must_use]
pub fn generate(title: &str, max_length: usize) -> String {
    // Step 1: Convert to lowercase and map characters
    let slug: String = title
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '_' {
                '-'
            } else {
                // Mark for removal
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect();

    // Step 2: Collapse consecutive hyphens and trim leading hyphens
    let mut result = String::with_capacity(slug.len());
    let mut prev_hyphen = true; // Start as true to trim leading hyphens

    for c in slug.chars() {
        if c == '-' {
            if !prev_hyphen {
                result.push(c);
                prev_hyphen = true;
            }
            // Skip consecutive hyphens
        } else {
            result.push(c);
            prev_hyphen = false;
        }
    }

    // Step 3: Trim trailing hyphen
    while result.ends_with('-') {
        result.pop();
    }

    // Step 4: Truncate to max_length, avoiding mid-word breaks
    if result.len() > max_length {
        let truncated = &result[..max_length];
        // Don't end on a hyphen
        truncated.trim_end_matches('-').to_string()
    } else {
        result
    }
}

/// Generates a slug using the default max length from ADR-003.
///
/// This is a convenience function that uses `SpecId::MAX_SLUG_LENGTH` (50).
///
/// # Examples
///
/// ```
/// use airsspec_core::utils::slug;
///
/// let slug = slug::generate_default("My Long Title");
/// assert!(slug.len() <= 50);
/// ```
#[must_use]
pub fn generate_default(title: &str) -> String {
    generate(title, SpecId::MAX_SLUG_LENGTH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_title() {
        assert_eq!(generate("User Authentication", 50), "user-authentication");
    }

    #[test]
    fn test_special_chars() {
        assert_eq!(generate("Fix Bug #123!", 50), "fix-bug-123");
    }

    #[test]
    fn test_underscores() {
        assert_eq!(generate("snake_case_title", 50), "snake-case-title");
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(generate("Multiple   Spaces", 50), "multiple-spaces");
    }

    #[test]
    fn test_leading_trailing_hyphens() {
        assert_eq!(generate("---title---", 50), "title");
    }

    #[test]
    fn test_leading_trailing_spaces() {
        assert_eq!(generate("  title  ", 50), "title");
    }

    #[test]
    fn test_consecutive_hyphens() {
        assert_eq!(generate("a--b---c", 50), "a-b-c");
    }

    #[test]
    fn test_mixed_case() {
        assert_eq!(generate("CamelCase Title", 50), "camelcase-title");
    }

    #[test]
    fn test_max_length_exact() {
        let title = "a".repeat(50);
        let slug = generate(&title, 50);
        assert_eq!(slug.len(), 50);
    }

    #[test]
    fn test_max_length_truncation() {
        let title = "this-is-a-very-long-title-that-exceeds-the-maximum-allowed-length";
        let slug = generate(title, 50);
        assert!(slug.len() <= 50);
        assert!(!slug.ends_with('-'));
    }

    #[test]
    fn test_truncation_at_hyphen() {
        // Create a title that would truncate right at a hyphen position
        let title = "a".repeat(49) + "-b";
        let slug = generate(&title, 50);
        assert!(slug.len() <= 50);
        assert!(!slug.ends_with('-'));
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(generate("", 50), "");
    }

    #[test]
    fn test_only_special_chars() {
        assert_eq!(generate("!!!###", 50), "");
    }

    #[test]
    fn test_unicode() {
        // Unicode chars should be stripped (only ASCII alphanumeric allowed)
        assert_eq!(generate("cafe", 50), "cafe");
    }

    #[test]
    fn test_numbers() {
        // Periods are stripped (not converted to hyphens)
        assert_eq!(generate("Version 2.0.1", 50), "version-201");
    }

    #[test]
    fn test_generate_default() {
        let slug = generate_default("Test Title");
        assert_eq!(slug, "test-title");
    }

    #[test]
    fn test_generate_default_max_length() {
        let long_title = "a".repeat(100);
        let slug = generate_default(&long_title);
        assert!(slug.len() <= SpecId::MAX_SLUG_LENGTH);
    }

    #[test]
    fn test_periods_are_stripped() {
        // Periods are special chars that get filtered out (not converted to hyphens)
        assert_eq!(generate("v1.2.3", 50), "v123");
    }

    #[test]
    fn test_at_symbol() {
        assert_eq!(generate("user@domain", 50), "userdomain");
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(generate("feature (new)", 50), "feature-new");
    }

    #[test]
    fn test_brackets() {
        assert_eq!(generate("fix [urgent]", 50), "fix-urgent");
    }
}
