//! Specification categorization.

use serde::{Deserialize, Serialize};

/// Category of a specification.
///
/// Used to classify the type of work a spec represents.
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::Category;
///
/// let cat = Category::Feature;
/// assert_eq!(format!("{cat}"), "feature");
///
/// // Default is Feature
/// let default_cat = Category::default();
/// assert_eq!(default_cat, Category::Feature);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// New feature implementation
    #[default]
    Feature,
    /// Enhancement to existing feature
    Enhancement,
    /// Bug fix
    BugFix,
    /// Code refactoring (no behavior change)
    Refactor,
    /// Documentation changes
    Documentation,
    /// Infrastructure or tooling changes
    Infrastructure,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Feature => "feature",
            Self::Enhancement => "enhancement",
            Self::BugFix => "bugfix",
            Self::Refactor => "refactor",
            Self::Documentation => "documentation",
            Self::Infrastructure => "infrastructure",
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let cat = Category::default();
        assert_eq!(cat, Category::Feature);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Category::Feature), "feature");
        assert_eq!(format!("{}", Category::Enhancement), "enhancement");
        assert_eq!(format!("{}", Category::BugFix), "bugfix");
        assert_eq!(format!("{}", Category::Refactor), "refactor");
        assert_eq!(format!("{}", Category::Documentation), "documentation");
        assert_eq!(format!("{}", Category::Infrastructure), "infrastructure");
    }

    #[test]
    fn test_clone_copy() {
        let cat = Category::BugFix;
        let copied = cat; // Copy trait
        let copied2 = copied; // Can copy again (verifies Copy)
        assert_eq!(cat, copied);
        assert_eq!(cat, copied2);
    }

    #[test]
    fn test_equality() {
        assert_eq!(Category::Feature, Category::Feature);
        assert_ne!(Category::Feature, Category::BugFix);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Category::Feature);
        set.insert(Category::BugFix);
        assert_eq!(set.len(), 2);

        // Same category shouldn't add twice
        set.insert(Category::Feature);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_serde_serialize() {
        let cat = Category::BugFix;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, "\"bugfix\"");
    }

    #[test]
    fn test_serde_deserialize() {
        let cat: Category = serde_json::from_str("\"enhancement\"").unwrap();
        assert_eq!(cat, Category::Enhancement);
    }
}
