//! Workflow phase types.

use serde::{Deserialize, Serialize};

/// Current phase in the spec workflow.
///
/// The workflow progresses linearly: Spec -> Plan -> Build.
///
/// # Examples
///
/// ```
/// use airsspec_core::shared::Phase;
///
/// let phase = Phase::default();
/// assert_eq!(phase, Phase::Spec);
/// assert_eq!(format!("{phase}"), "spec");
///
/// // Navigation
/// assert_eq!(phase.next(), Some(Phase::Plan));
/// assert_eq!(Phase::Build.next(), None);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    /// Specification phase - defining what to build.
    #[default]
    Spec,
    /// Planning phase - deciding how to build it.
    Plan,
    /// Build phase - implementing the plan.
    Build,
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Spec => "spec",
            Self::Plan => "plan",
            Self::Build => "build",
        };
        write!(f, "{s}")
    }
}

impl Phase {
    /// Returns the next phase in the workflow, or `None` if at the final phase.
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::shared::Phase;
    ///
    /// assert_eq!(Phase::Spec.next(), Some(Phase::Plan));
    /// assert_eq!(Phase::Plan.next(), Some(Phase::Build));
    /// assert_eq!(Phase::Build.next(), None);
    /// ```
    #[must_use]
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::Spec => Some(Self::Plan),
            Self::Plan => Some(Self::Build),
            Self::Build => None,
        }
    }

    /// Returns the previous phase in the workflow, or `None` if at the initial phase.
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::shared::Phase;
    ///
    /// assert_eq!(Phase::Spec.previous(), None);
    /// assert_eq!(Phase::Plan.previous(), Some(Phase::Spec));
    /// assert_eq!(Phase::Build.previous(), Some(Phase::Plan));
    /// ```
    #[must_use]
    pub fn previous(&self) -> Option<Self> {
        match self {
            Self::Spec => None,
            Self::Plan => Some(Self::Spec),
            Self::Build => Some(Self::Plan),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let phase = Phase::default();
        assert_eq!(phase, Phase::Spec);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Phase::Spec), "spec");
        assert_eq!(format!("{}", Phase::Plan), "plan");
        assert_eq!(format!("{}", Phase::Build), "build");
    }

    #[test]
    fn test_next() {
        assert_eq!(Phase::Spec.next(), Some(Phase::Plan));
        assert_eq!(Phase::Plan.next(), Some(Phase::Build));
        assert_eq!(Phase::Build.next(), None);
    }

    #[test]
    fn test_previous() {
        assert_eq!(Phase::Spec.previous(), None);
        assert_eq!(Phase::Plan.previous(), Some(Phase::Spec));
        assert_eq!(Phase::Build.previous(), Some(Phase::Plan));
    }

    #[test]
    fn test_clone_copy() {
        let phase = Phase::Plan;
        let copied = phase; // Copy trait
        let copied2 = copied; // Can copy again (verifies Copy)
        assert_eq!(phase, copied);
        assert_eq!(phase, copied2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Phase::Spec);
        set.insert(Phase::Plan);
        assert_eq!(set.len(), 2);

        // Same phase shouldn't add twice
        set.insert(Phase::Spec);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_serde_roundtrip() {
        let phase = Phase::Build;
        let json = serde_json::to_string(&phase).unwrap();
        assert_eq!(json, "\"build\"");

        let parsed: Phase = serde_json::from_str(&json).unwrap();
        assert_eq!(phase, parsed);
    }
}
