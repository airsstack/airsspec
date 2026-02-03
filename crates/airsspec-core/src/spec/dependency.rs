//! Dependency relationships between specifications.

use serde::{Deserialize, Serialize};

use super::id::SpecId;

/// A dependency relationship to another specification.
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::{Dependency, DependencyKind, SpecId};
///
/// let parent_id = SpecId::new(1_737_734_400, "user-auth");
/// let dep = Dependency {
///     spec_id: parent_id,
///     kind: DependencyKind::BlockedBy,
/// };
///
/// assert_eq!(dep.kind, DependencyKind::BlockedBy);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dependency {
    /// The spec this dependency points to.
    pub spec_id: SpecId,
    /// The type of dependency relationship.
    pub kind: DependencyKind,
}

impl Dependency {
    /// Creates a new dependency.
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::{Dependency, DependencyKind, SpecId};
    ///
    /// let spec_id = SpecId::new(1_737_734_400, "parent-spec");
    /// let dep = Dependency::new(spec_id, DependencyKind::ChildOf);
    /// ```
    #[must_use]
    pub fn new(spec_id: SpecId, kind: DependencyKind) -> Self {
        Self { spec_id, kind }
    }

    /// Creates a "blocked by" dependency.
    ///
    /// Convenience method for the most common dependency type.
    #[must_use]
    pub fn blocked_by(spec_id: SpecId) -> Self {
        Self::new(spec_id, DependencyKind::BlockedBy)
    }

    /// Creates a "related to" dependency.
    #[must_use]
    pub fn related_to(spec_id: SpecId) -> Self {
        Self::new(spec_id, DependencyKind::RelatedTo)
    }
}

/// Type of dependency relationship between specs.
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::DependencyKind;
///
/// let kind = DependencyKind::BlockedBy;
/// assert_eq!(format!("{kind}"), "blocked_by");
///
/// // Default is RelatedTo (informational)
/// let default_kind = DependencyKind::default();
/// assert_eq!(default_kind, DependencyKind::RelatedTo);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyKind {
    /// This spec is blocked by another spec (hard dependency).
    /// The spec cannot progress until the blocking spec is complete.
    BlockedBy,
    /// This spec is related to another spec (soft dependency).
    /// Informational only, does not block progression.
    #[default]
    RelatedTo,
    /// This spec is a child of another spec (hierarchical).
    ChildOf,
    /// This spec is a parent of another spec (hierarchical).
    ParentOf,
}

impl std::fmt::Display for DependencyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::BlockedBy => "blocked_by",
            Self::RelatedTo => "related_to",
            Self::ChildOf => "child_of",
            Self::ParentOf => "parent_of",
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_new() {
        let spec_id = SpecId::new(1_737_734_400, "parent");
        let dep = Dependency::new(spec_id.clone(), DependencyKind::BlockedBy);

        assert_eq!(dep.spec_id, spec_id);
        assert_eq!(dep.kind, DependencyKind::BlockedBy);
    }

    #[test]
    fn test_dependency_blocked_by() {
        let spec_id = SpecId::new(1_737_734_400, "blocker");
        let dep = Dependency::blocked_by(spec_id.clone());

        assert_eq!(dep.spec_id, spec_id);
        assert_eq!(dep.kind, DependencyKind::BlockedBy);
    }

    #[test]
    fn test_dependency_related_to() {
        let spec_id = SpecId::new(1_737_734_400, "related");
        let dep = Dependency::related_to(spec_id.clone());

        assert_eq!(dep.spec_id, spec_id);
        assert_eq!(dep.kind, DependencyKind::RelatedTo);
    }

    #[test]
    fn test_dependency_clone() {
        let spec_id = SpecId::new(1_737_734_400, "test");
        let dep = Dependency::new(spec_id, DependencyKind::ChildOf);
        let cloned = dep.clone();

        assert_eq!(dep, cloned);
    }

    #[test]
    fn test_dependency_kind_default() {
        let kind = DependencyKind::default();
        assert_eq!(kind, DependencyKind::RelatedTo);
    }

    #[test]
    fn test_dependency_kind_display() {
        assert_eq!(format!("{}", DependencyKind::BlockedBy), "blocked_by");
        assert_eq!(format!("{}", DependencyKind::RelatedTo), "related_to");
        assert_eq!(format!("{}", DependencyKind::ChildOf), "child_of");
        assert_eq!(format!("{}", DependencyKind::ParentOf), "parent_of");
    }

    #[test]
    fn test_dependency_kind_copy() {
        let kind = DependencyKind::BlockedBy;
        let copied = kind; // Copy trait
        assert_eq!(kind, copied);
    }

    #[test]
    fn test_dependency_kind_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(DependencyKind::BlockedBy);
        set.insert(DependencyKind::RelatedTo);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_dependency_serde() {
        let spec_id = SpecId::new(1_737_734_400, "test");
        let dep = Dependency::new(spec_id, DependencyKind::BlockedBy);

        let json = serde_json::to_string(&dep).unwrap();
        let parsed: Dependency = serde_json::from_str(&json).unwrap();

        assert_eq!(dep, parsed);
    }

    #[test]
    fn test_dependency_kind_serde() {
        let kind = DependencyKind::ChildOf;
        let json = serde_json::to_string(&kind).unwrap();
        assert_eq!(json, "\"child_of\"");

        let parsed: DependencyKind = serde_json::from_str(&json).unwrap();
        assert_eq!(kind, parsed);
    }
}
