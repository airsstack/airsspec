//! Memory types for fragment storage and compression configuration.
//!
//! This module defines the data structures used across all three memory tiers.

// Layer 1: Standard library imports
use std::fmt;

// Layer 2: Third-party crate imports
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Layer 3: Internal module imports (none - this is a leaf module)

/// A memory fragment representing a unit of context.
///
/// Fragments are the basic building blocks of memory, containing content,
/// metadata, and tracking information. They are stored in hot memory for
/// immediate access and can be compressed into warm memory for long-term storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFragment {
    /// Unique identifier for this fragment.
    pub id: String,

    /// The content of the fragment (text, code, etc.).
    pub content: String,

    /// When this fragment was created.
    pub created_at: DateTime<Utc>,

    /// Number of tokens in this fragment.
    ///
    /// This is used for token budget management and determining when to compress.
    pub token_count: u32,
}

impl MemoryFragment {
    /// Creates a new memory fragment.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the fragment
    /// * `content` - The fragment content
    /// * `token_count` - Number of tokens in the content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::memory::types::MemoryFragment;
    /// use chrono::Utc;
    ///
    /// let fragment = MemoryFragment::new(
    ///     "frag-123".to_string(),
    ///     "Some content".to_string(),
    ///     10
    /// );
    /// ```
    #[must_use]
    pub fn new(id: String, content: String, token_count: u32) -> Self {
        Self {
            id,
            content,
            created_at: Utc::now(),
            token_count,
        }
    }
}

impl fmt::Display for MemoryFragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MemoryFragment({}, {} tokens, {})",
            self.id,
            self.token_count,
            self.created_at.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

/// Configuration for memory compression.
///
/// This controls when and how fragments are compressed from hot memory
/// into warm memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// Token threshold at which compression is triggered.
    ///
    /// When the total token count in hot memory exceeds this threshold,
    /// the system compresses older fragments into warm memory.
    pub threshold_tokens: u32,

    /// Target compression ratio (0.0 to 1.0).
    ///
    /// This represents the desired size of the compressed output relative
    /// to the original input. For example, a ratio of 0.2 means compress
    /// to 20% of the original size.
    pub target_ratio: f32,
}

impl CompressionConfig {
    /// Creates a new compression configuration with default values.
    ///
    /// # Returns
    ///
    /// A new `CompressionConfig` with sensible defaults:
    /// - `threshold_tokens`: 4096
    /// - `target_ratio`: 0.2
    #[must_use]
    pub const fn new() -> Self {
        Self {
            threshold_tokens: 4096,
            target_ratio: 0.2,
        }
    }

    /// Creates a custom compression configuration.
    ///
    /// # Arguments
    ///
    /// * `threshold_tokens` - Token threshold for triggering compression
    /// * `target_ratio` - Target compression ratio (0.0 to 1.0)
    ///
    /// # Panics
    ///
    /// Panics if `target_ratio` is not between 0.0 and 1.0.
    #[must_use]
    pub fn with_values(threshold_tokens: u32, target_ratio: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&target_ratio),
            "target_ratio must be between 0.0 and 1.0"
        );
        Self {
            threshold_tokens,
            target_ratio,
        }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::float_cmp,
    clippy::uninlined_format_args,
    unused_must_use
)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_fragment_new() {
        let fragment = MemoryFragment::new("test-id".to_string(), "test content".to_string(), 15);

        assert_eq!(fragment.id, "test-id");
        assert_eq!(fragment.content, "test content");
        assert_eq!(fragment.token_count, 15);
    }

    #[test]
    fn test_memory_fragment_created_at() {
        let before = Utc::now();
        let fragment = MemoryFragment::new("test-id".to_string(), "test content".to_string(), 10);
        let after = Utc::now();

        assert!(fragment.created_at >= before);
        assert!(fragment.created_at <= after);
    }

    #[test]
    fn test_memory_fragment_display() {
        let fragment = MemoryFragment::new("test-id".to_string(), "test content".to_string(), 10);

        let display = fragment.to_string();
        assert!(display.contains("test-id"));
        assert!(display.contains("10 tokens"));
    }

    #[test]
    fn test_compression_config_new() {
        let config = CompressionConfig::new();

        assert_eq!(config.threshold_tokens, 4096);
        assert_eq!(config.target_ratio, 0.2);
    }

    #[test]
    fn test_compression_config_default() {
        let config = CompressionConfig::default();

        assert_eq!(config.threshold_tokens, 4096);
        assert_eq!(config.target_ratio, 0.2);
    }

    #[test]
    fn test_compression_config_with_values() {
        let config = CompressionConfig::with_values(8192, 0.3);

        assert_eq!(config.threshold_tokens, 8192);
        assert_eq!(config.target_ratio, 0.3);
    }

    #[test]
    fn test_compression_config_with_values_invalid_ratio() {
        // Test that invalid ratios panic
        let result = std::panic::catch_unwind(|| {
            let _ = CompressionConfig::with_values(4096, 1.5);
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_serialization_memory_fragment() {
        let fragment = MemoryFragment::new("test-id".to_string(), "test content".to_string(), 10);

        let json = serde_json::to_string(&fragment).unwrap();
        assert!(json.contains("test-id"));
        assert!(json.contains("test content"));

        let deserialized: MemoryFragment = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, fragment.id);
        assert_eq!(deserialized.content, fragment.content);
    }

    #[test]
    fn test_serialization_compression_config() {
        let config = CompressionConfig::new();

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("4096"));
        assert!(json.contains("0.2"));

        let deserialized: CompressionConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.threshold_tokens, config.threshold_tokens);
        assert_eq!(deserialized.target_ratio, config.target_ratio);
    }

    #[test]
    fn test_clone_memory_fragment() {
        let fragment = MemoryFragment::new("test-id".to_string(), "test content".to_string(), 10);

        let cloned = fragment.clone();
        assert_eq!(cloned.id, fragment.id);
        assert_eq!(cloned.content, fragment.content);
        assert_eq!(cloned.token_count, fragment.token_count);
    }

    #[test]
    fn test_clone_compression_config() {
        let config = CompressionConfig::new();

        let cloned = config.clone();
        assert_eq!(cloned.threshold_tokens, config.threshold_tokens);
        assert_eq!(cloned.target_ratio, config.target_ratio);
    }
}
