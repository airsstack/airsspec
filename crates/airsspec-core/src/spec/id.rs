//! Unique identifier for specifications.
//!
//! Per ADR-003, spec IDs follow the format `{unix-timestamp}-{title-slug}`.

use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::error::SpecError;

/// Unique identifier for a specification.
///
/// Format: `{unix-timestamp}-{title-slug}`
///
/// # Format Specification (ADR-003)
///
/// - **Timestamp**: Unix timestamp (seconds since epoch)
/// - **Separator**: Single hyphen (`-`)
/// - **Slug**: Lowercase, alphanumeric + hyphens, max 50 chars
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::SpecId;
///
/// // Create from components
/// let id = SpecId::new(1_737_734_400, "user-auth");
/// assert_eq!(id.timestamp(), 1_737_734_400);
/// assert_eq!(id.slug(), "user-auth");
/// assert_eq!(id.as_str(), "1737734400-user-auth");
///
/// // Parse from string
/// let parsed = SpecId::parse("1737734400-user-auth").unwrap();
/// assert_eq!(id, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpecId(String);

impl SpecId {
    /// Maximum length for the slug portion (ADR-003).
    pub const MAX_SLUG_LENGTH: usize = 50;

    /// Creates a new `SpecId` from timestamp and slug, validating the slug.
    ///
    /// This is the recommended constructor for external input.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Unix timestamp (seconds since epoch)
    /// * `slug` - URL-safe identifier (lowercase, alphanumeric + hyphens)
    ///
    /// # Errors
    ///
    /// Returns `SpecError::InvalidId` if:
    /// - `slug` is empty
    /// - `slug` exceeds `MAX_SLUG_LENGTH` (50 characters)
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecId;
    ///
    /// let id = SpecId::try_new(1_737_734_400, "user-authentication")?;
    /// assert_eq!(id.as_str(), "1737734400-user-authentication");
    ///
    /// // Empty slug returns error
    /// assert!(SpecId::try_new(1_737_734_400, "").is_err());
    /// # Ok::<(), airsspec_core::spec::SpecError>(())
    /// ```
    pub fn try_new(timestamp: i64, slug: &str) -> Result<Self, SpecError> {
        if slug.is_empty() {
            return Err(SpecError::InvalidId("slug cannot be empty".to_string()));
        }

        if slug.len() > Self::MAX_SLUG_LENGTH {
            return Err(SpecError::InvalidId(format!(
                "slug exceeds maximum length of {} characters",
                Self::MAX_SLUG_LENGTH
            )));
        }

        Ok(Self(format!("{timestamp}-{slug}")))
    }

    /// Creates a new `SpecId` from timestamp and slug.
    ///
    /// For external/untrusted input, prefer [`try_new`](Self::try_new) which
    /// returns a `Result` instead of panicking.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Unix timestamp (seconds since epoch)
    /// * `slug` - URL-safe identifier (lowercase, alphanumeric + hyphens)
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `slug` is empty
    /// - `slug` exceeds `MAX_SLUG_LENGTH` (50 characters)
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecId;
    ///
    /// let id = SpecId::new(1_737_734_400, "user-authentication");
    /// assert_eq!(id.as_str(), "1737734400-user-authentication");
    /// ```
    #[must_use]
    pub fn new(timestamp: i64, slug: &str) -> Self {
        self::SpecId::try_new(timestamp, slug).expect("invalid slug for SpecId::new()")
    }

    /// Parses a `SpecId` from a string.
    ///
    /// Validates that the string follows the `{timestamp}-{slug}` format.
    ///
    /// # Errors
    ///
    /// Returns `SpecError::InvalidId` if:
    /// - Format is invalid (missing hyphen separator)
    /// - Timestamp portion is not a valid integer
    /// - Slug is empty or exceeds `MAX_SLUG_LENGTH`
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecId;
    ///
    /// // Valid ID
    /// let id = SpecId::parse("1737734400-user-auth").unwrap();
    /// assert_eq!(id.timestamp(), 1_737_734_400);
    ///
    /// // Invalid format (missing slug)
    /// assert!(SpecId::parse("1737734400").is_err());
    ///
    /// // Invalid timestamp
    /// assert!(SpecId::parse("not-a-timestamp-slug").is_err());
    /// ```
    pub fn parse(s: &str) -> Result<Self, SpecError> {
        let parts: Vec<&str> = s.splitn(2, '-').collect();

        if parts.len() != 2 {
            return Err(SpecError::InvalidId(
                "format must be {timestamp}-{slug}".to_string(),
            ));
        }

        let _timestamp: i64 = parts[0].parse().map_err(|_err| {
            SpecError::InvalidId(format!(
                "invalid timestamp '{}': must be an integer",
                parts[0]
            ))
        })?;

        let slug = parts[1];

        if slug.is_empty() {
            return Err(SpecError::InvalidId("slug cannot be empty".to_string()));
        }

        if slug.len() > Self::MAX_SLUG_LENGTH {
            return Err(SpecError::InvalidId(format!(
                "slug exceeds maximum length of {} characters",
                Self::MAX_SLUG_LENGTH
            )));
        }

        Ok(Self(s.to_string()))
    }

    /// Returns the Unix timestamp portion.
    ///
    /// # Panics
    ///
    /// Panics if the internal string representation is malformed (should never
    /// happen as `SpecId` is only constructed via validated constructors).
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecId;
    ///
    /// let id = SpecId::new(1_737_734_400, "user-auth");
    /// assert_eq!(id.timestamp(), 1_737_734_400);
    /// ```
    #[must_use]
    pub fn timestamp(&self) -> i64 {
        // Find the position where slug starts (after timestamp and first hyphen after digits)
        // Handle negative timestamps: "-86400-slug" -> timestamp is "-86400"
        let bytes = self.0.as_bytes();
        let start = usize::from(bytes.first() == Some(&b'-'));

        // Find first hyphen after the initial digit portion
        let hyphen_pos = bytes[start..]
            .iter()
            .position(|&b| b == b'-')
            .map(|p| p + start)
            .expect("SpecId guaranteed to have hyphen separator");

        self.0[..hyphen_pos]
            .parse()
            .expect("SpecId guaranteed to have valid timestamp")
    }

    /// Returns the slug portion.
    ///
    /// # Panics
    ///
    /// Panics if the internal string representation is malformed (should never
    /// happen as `SpecId` is only constructed via validated constructors).
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecId;
    ///
    /// let id = SpecId::new(1_737_734_400, "user-auth");
    /// assert_eq!(id.slug(), "user-auth");
    /// ```
    #[must_use]
    pub fn slug(&self) -> &str {
        // Handle negative timestamps: "-86400-slug" -> slug is "slug"
        let bytes = self.0.as_bytes();
        let start = usize::from(bytes.first() == Some(&b'-'));

        // Find first hyphen after the initial digit portion
        let hyphen_pos = bytes[start..]
            .iter()
            .position(|&b| b == b'-')
            .map(|p| p + start)
            .expect("SpecId guaranteed to have hyphen separator");

        &self.0[hyphen_pos + 1..]
    }

    /// Returns the full ID as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecId;
    ///
    /// let id = SpecId::new(1_737_734_400, "user-auth");
    /// assert_eq!(id.as_str(), "1737734400-user-auth");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SpecId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for SpecId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let id = SpecId::new(1_737_734_400, "user-auth");
        assert_eq!(id.timestamp(), 1_737_734_400);
        assert_eq!(id.slug(), "user-auth");
        assert_eq!(id.as_str(), "1737734400-user-auth");
    }

    #[test]
    fn test_new_max_slug_length() {
        let slug = "a".repeat(SpecId::MAX_SLUG_LENGTH);
        let id = SpecId::new(1_737_734_400, &slug);
        assert_eq!(id.slug(), slug);
    }

    #[test]
    #[should_panic(expected = "invalid slug for SpecId::new()")]
    fn test_new_empty_slug_panics() {
        let _ = SpecId::new(1_737_734_400, "");
    }

    #[test]
    #[should_panic(expected = "invalid slug for SpecId::new()")]
    fn test_new_slug_too_long_panics() {
        let slug = "a".repeat(SpecId::MAX_SLUG_LENGTH + 1);
        let _ = SpecId::new(1_737_734_400, &slug);
    }

    #[test]
    fn test_try_new_valid() {
        let id = SpecId::try_new(1_737_734_400, "user-auth").unwrap();
        assert_eq!(id.timestamp(), 1_737_734_400);
        assert_eq!(id.slug(), "user-auth");
        assert_eq!(id.as_str(), "1737734400-user-auth");
    }

    #[test]
    fn test_try_new_max_slug_length() {
        let slug = "a".repeat(SpecId::MAX_SLUG_LENGTH);
        let id = SpecId::try_new(1_737_734_400, &slug).unwrap();
        assert_eq!(id.slug(), slug);
    }

    #[test]
    fn test_try_new_empty_slug_error() {
        let result = SpecId::try_new(1_737_734_400, "");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, SpecError::InvalidId(_)));
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn test_try_new_slug_too_long_error() {
        let slug = "a".repeat(SpecId::MAX_SLUG_LENGTH + 1);
        let result = SpecId::try_new(1_737_734_400, &slug);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, SpecError::InvalidId(_)));
        assert!(err.to_string().contains("exceeds maximum length"));
    }

    #[test]
    fn test_try_new_negative_timestamp() {
        let id = SpecId::try_new(-86400, "pre-epoch").unwrap();
        assert_eq!(id.timestamp(), -86400);
        assert_eq!(id.slug(), "pre-epoch");
    }

    #[test]
    fn test_parse_valid() {
        let id = SpecId::parse("1737734400-user-auth").unwrap();
        assert_eq!(id.timestamp(), 1_737_734_400);
        assert_eq!(id.slug(), "user-auth");
    }

    #[test]
    fn test_parse_slug_with_hyphens() {
        let id = SpecId::parse("1737734400-user-auth-system").unwrap();
        assert_eq!(id.slug(), "user-auth-system");
    }

    #[test]
    fn test_parse_invalid_no_hyphen() {
        let result = SpecId::parse("1737734400");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, SpecError::InvalidId(_)));
    }

    #[test]
    fn test_parse_invalid_timestamp() {
        let result = SpecId::parse("not-a-number-slug");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_slug() {
        let result = SpecId::parse("1737734400-");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_slug_too_long() {
        let long_slug = "a".repeat(SpecId::MAX_SLUG_LENGTH + 1);
        let input = format!("1737734400-{long_slug}");
        let result = SpecId::parse(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        let id = SpecId::new(1_737_734_400, "user-auth");
        assert_eq!(format!("{id}"), "1737734400-user-auth");
    }

    #[test]
    fn test_equality() {
        let id1 = SpecId::new(1_737_734_400, "user-auth");
        let id2 = SpecId::parse("1737734400-user-auth").unwrap();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;

        let id1 = SpecId::new(1_737_734_400, "user-auth");
        let id2 = SpecId::parse("1737734400-user-auth").unwrap();

        let mut set = HashSet::new();
        set.insert(id1);
        assert!(set.contains(&id2));
    }

    #[test]
    fn test_clone() {
        let id = SpecId::new(1_737_734_400, "user-auth");
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_as_ref() {
        let id = SpecId::new(1_737_734_400, "user-auth");
        let s: &str = id.as_ref();
        assert_eq!(s, "1737734400-user-auth");
    }

    #[test]
    fn test_negative_timestamp() {
        // Negative timestamps are valid (dates before 1970)
        let id = SpecId::new(-86400, "pre-epoch");
        assert_eq!(id.timestamp(), -86400);
    }

    #[test]
    fn test_serde_roundtrip() {
        let id = SpecId::new(1_737_734_400, "user-auth");
        let json = serde_json::to_string(&id).unwrap();
        let parsed: SpecId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }
}
