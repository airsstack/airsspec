//! Lifecycle states for specifications and plans.

use serde::{Deserialize, Serialize};

/// Lifecycle state of a specification or plan.
///
/// State transitions are managed by [`StateMachine`](crate::state::StateMachine).
///
/// # State Transition Diagram
///
/// ```text
/// Draft -> Active -> Done -> Archived
///            |         ^
///            v         |
///         Blocked -----+
///            |
///            v
///       Cancelled -> Archived
/// ```
///
/// # Examples
///
/// ```
/// use airsspec_core::shared::LifecycleState;
///
/// let state = LifecycleState::default();
/// assert_eq!(state, LifecycleState::Draft);
/// assert_eq!(format!("{state}"), "draft");
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LifecycleState {
    /// Work in progress, not ready for implementation.
    #[default]
    Draft,
    /// Being actively worked on.
    Active,
    /// Successfully completed.
    Done,
    /// Waiting on external dependencies.
    Blocked,
    /// Abandoned or no longer needed.
    Cancelled,
    /// Preserved for historical reference.
    Archived,
}

impl std::fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Done => "done",
            Self::Blocked => "blocked",
            Self::Cancelled => "cancelled",
            Self::Archived => "archived",
        };
        write!(f, "{s}")
    }
}

impl LifecycleState {
    /// Returns `true` if this is a terminal state (no further transitions allowed).
    ///
    /// Terminal states: `Done`, `Cancelled`, `Archived`.
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::shared::LifecycleState;
    ///
    /// assert!(!LifecycleState::Draft.is_terminal());
    /// assert!(!LifecycleState::Active.is_terminal());
    /// assert!(LifecycleState::Done.is_terminal());
    /// assert!(LifecycleState::Cancelled.is_terminal());
    /// assert!(LifecycleState::Archived.is_terminal());
    /// ```
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Done | Self::Cancelled | Self::Archived)
    }

    /// Returns `true` if this is the `Active` state.
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::shared::LifecycleState;
    ///
    /// assert!(LifecycleState::Active.is_active());
    /// assert!(!LifecycleState::Draft.is_active());
    /// ```
    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns `true` if this is the `Blocked` state.
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::shared::LifecycleState;
    ///
    /// assert!(LifecycleState::Blocked.is_blocked());
    /// assert!(!LifecycleState::Active.is_blocked());
    /// ```
    #[must_use]
    pub fn is_blocked(&self) -> bool {
        matches!(self, Self::Blocked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let state = LifecycleState::default();
        assert_eq!(state, LifecycleState::Draft);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", LifecycleState::Draft), "draft");
        assert_eq!(format!("{}", LifecycleState::Active), "active");
        assert_eq!(format!("{}", LifecycleState::Done), "done");
        assert_eq!(format!("{}", LifecycleState::Blocked), "blocked");
        assert_eq!(format!("{}", LifecycleState::Cancelled), "cancelled");
        assert_eq!(format!("{}", LifecycleState::Archived), "archived");
    }

    #[test]
    fn test_is_terminal() {
        assert!(!LifecycleState::Draft.is_terminal());
        assert!(!LifecycleState::Active.is_terminal());
        assert!(LifecycleState::Done.is_terminal());
        assert!(!LifecycleState::Blocked.is_terminal());
        assert!(LifecycleState::Cancelled.is_terminal());
        assert!(LifecycleState::Archived.is_terminal());
    }

    #[test]
    fn test_is_active() {
        assert!(!LifecycleState::Draft.is_active());
        assert!(LifecycleState::Active.is_active());
        assert!(!LifecycleState::Done.is_active());
        assert!(!LifecycleState::Blocked.is_active());
        assert!(!LifecycleState::Cancelled.is_active());
        assert!(!LifecycleState::Archived.is_active());
    }

    #[test]
    fn test_is_blocked() {
        assert!(!LifecycleState::Draft.is_blocked());
        assert!(!LifecycleState::Active.is_blocked());
        assert!(!LifecycleState::Done.is_blocked());
        assert!(LifecycleState::Blocked.is_blocked());
        assert!(!LifecycleState::Cancelled.is_blocked());
        assert!(!LifecycleState::Archived.is_blocked());
    }

    #[test]
    fn test_clone_copy() {
        let state = LifecycleState::Active;
        let copied = state; // Copy trait
        let copied2 = copied; // Can copy again (verifies Copy)
        assert_eq!(state, copied);
        assert_eq!(state, copied2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(LifecycleState::Draft);
        set.insert(LifecycleState::Active);
        assert_eq!(set.len(), 2);

        // Same state shouldn't add twice
        set.insert(LifecycleState::Draft);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_serde_serialize() {
        let state = LifecycleState::Active;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "\"active\"");
    }

    #[test]
    fn test_serde_deserialize() {
        let state: LifecycleState = serde_json::from_str("\"blocked\"").unwrap();
        assert_eq!(state, LifecycleState::Blocked);
    }
}
