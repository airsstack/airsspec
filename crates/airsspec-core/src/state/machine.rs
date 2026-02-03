//! State machine for lifecycle transitions.

use crate::shared::LifecycleState;

use super::error::StateError;

/// State machine for spec lifecycle transitions.
///
/// Enforces valid state transitions according to lifecycle rules.
///
/// # Valid Transitions
///
/// - Draft -> Active (start work)
/// - Active -> Done (complete successfully)
/// - Active -> Blocked (waiting on dependencies)
/// - Active -> Cancelled (abandon work)
/// - Blocked -> Active (unblocked)
/// - Done -> Archived (preserve for reference)
/// - Cancelled -> Archived (preserve for reference)
///
/// # Examples
///
/// ```
/// use airsspec_core::state::StateMachine;
/// use airsspec_core::shared::LifecycleState;
///
/// let machine = StateMachine::new();
///
/// // Valid transitions
/// assert!(machine.can_transition(LifecycleState::Draft, LifecycleState::Active));
/// assert!(machine.can_transition(LifecycleState::Active, LifecycleState::Done));
///
/// // Invalid transitions
/// assert!(!machine.can_transition(LifecycleState::Draft, LifecycleState::Done));
/// assert!(!machine.can_transition(LifecycleState::Archived, LifecycleState::Active));
/// ```
#[derive(Debug, Clone, Default)]
pub struct StateMachine;

impl StateMachine {
    /// Creates a new state machine instance.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Checks if a transition from one state to another is valid.
    #[must_use]
    pub fn can_transition(&self, from: LifecycleState, to: LifecycleState) -> bool {
        use LifecycleState::{Active, Archived, Blocked, Cancelled, Done, Draft};
        matches!(
            (from, to),
            (Draft | Blocked, Active)
                | (Active, Done | Blocked | Cancelled)
                | (Done | Cancelled, Archived)
        )
    }

    /// Returns all valid transitions from the given state.
    #[must_use]
    pub fn valid_transitions(&self, from: LifecycleState) -> Vec<LifecycleState> {
        use LifecycleState::{Active, Archived, Blocked, Cancelled, Done};
        match from {
            LifecycleState::Draft | LifecycleState::Blocked => vec![Active],
            LifecycleState::Active => vec![Done, Blocked, Cancelled],
            LifecycleState::Done | LifecycleState::Cancelled => vec![Archived],
            LifecycleState::Archived => vec![],
        }
    }

    /// Attempts a state transition, returning the new state or an error.
    ///
    /// # Errors
    ///
    /// Returns [`StateError::InvalidTransition`] if the transition is not allowed.
    pub fn transition(
        &self,
        current: LifecycleState,
        target: LifecycleState,
    ) -> Result<LifecycleState, StateError> {
        if self.can_transition(current, target) {
            Ok(target)
        } else {
            Err(StateError::InvalidTransition {
                from: current,
                to: target,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let machine = StateMachine::new();
        // Just verify it constructs without panic
        assert!(machine.can_transition(LifecycleState::Draft, LifecycleState::Active));
    }

    #[test]
    fn test_default_trait() {
        // StateMachine is a unit struct that implements Default
        // Verify it can be created via the Default trait
        fn assert_default<T: Default>() {}
        assert_default::<StateMachine>();
    }

    #[test]
    fn test_valid_transitions_from_draft() {
        let machine = StateMachine::new();
        assert!(machine.can_transition(LifecycleState::Draft, LifecycleState::Active));
        assert!(!machine.can_transition(LifecycleState::Draft, LifecycleState::Done));
        assert!(!machine.can_transition(LifecycleState::Draft, LifecycleState::Blocked));
        assert!(!machine.can_transition(LifecycleState::Draft, LifecycleState::Cancelled));
        assert!(!machine.can_transition(LifecycleState::Draft, LifecycleState::Archived));
        assert!(!machine.can_transition(LifecycleState::Draft, LifecycleState::Draft));
    }

    #[test]
    fn test_valid_transitions_from_active() {
        let machine = StateMachine::new();
        assert!(!machine.can_transition(LifecycleState::Active, LifecycleState::Draft));
        assert!(machine.can_transition(LifecycleState::Active, LifecycleState::Done));
        assert!(machine.can_transition(LifecycleState::Active, LifecycleState::Blocked));
        assert!(machine.can_transition(LifecycleState::Active, LifecycleState::Cancelled));
        assert!(!machine.can_transition(LifecycleState::Active, LifecycleState::Archived));
        assert!(!machine.can_transition(LifecycleState::Active, LifecycleState::Active));
    }

    #[test]
    fn test_valid_transitions_from_blocked() {
        let machine = StateMachine::new();
        assert!(!machine.can_transition(LifecycleState::Blocked, LifecycleState::Draft));
        assert!(machine.can_transition(LifecycleState::Blocked, LifecycleState::Active));
        assert!(!machine.can_transition(LifecycleState::Blocked, LifecycleState::Done));
        assert!(!machine.can_transition(LifecycleState::Blocked, LifecycleState::Cancelled));
        assert!(!machine.can_transition(LifecycleState::Blocked, LifecycleState::Archived));
        assert!(!machine.can_transition(LifecycleState::Blocked, LifecycleState::Blocked));
    }

    #[test]
    fn test_valid_transitions_from_done() {
        let machine = StateMachine::new();
        assert!(!machine.can_transition(LifecycleState::Done, LifecycleState::Draft));
        assert!(!machine.can_transition(LifecycleState::Done, LifecycleState::Active));
        assert!(!machine.can_transition(LifecycleState::Done, LifecycleState::Blocked));
        assert!(!machine.can_transition(LifecycleState::Done, LifecycleState::Cancelled));
        assert!(machine.can_transition(LifecycleState::Done, LifecycleState::Archived));
        assert!(!machine.can_transition(LifecycleState::Done, LifecycleState::Done));
    }

    #[test]
    fn test_valid_transitions_from_cancelled() {
        let machine = StateMachine::new();
        assert!(!machine.can_transition(LifecycleState::Cancelled, LifecycleState::Draft));
        assert!(!machine.can_transition(LifecycleState::Cancelled, LifecycleState::Active));
        assert!(!machine.can_transition(LifecycleState::Cancelled, LifecycleState::Blocked));
        assert!(!machine.can_transition(LifecycleState::Cancelled, LifecycleState::Done));
        assert!(machine.can_transition(LifecycleState::Cancelled, LifecycleState::Archived));
        assert!(!machine.can_transition(LifecycleState::Cancelled, LifecycleState::Cancelled));
    }

    #[test]
    fn test_valid_transitions_from_archived() {
        let machine = StateMachine::new();
        assert!(!machine.can_transition(LifecycleState::Archived, LifecycleState::Draft));
        assert!(!machine.can_transition(LifecycleState::Archived, LifecycleState::Active));
        assert!(!machine.can_transition(LifecycleState::Archived, LifecycleState::Blocked));
        assert!(!machine.can_transition(LifecycleState::Archived, LifecycleState::Done));
        assert!(!machine.can_transition(LifecycleState::Archived, LifecycleState::Cancelled));
        assert!(!machine.can_transition(LifecycleState::Archived, LifecycleState::Archived));
    }

    #[test]
    fn test_valid_transitions_list() {
        let machine = StateMachine::new();

        assert_eq!(
            machine.valid_transitions(LifecycleState::Draft),
            vec![LifecycleState::Active]
        );
        assert_eq!(
            machine.valid_transitions(LifecycleState::Active),
            vec![
                LifecycleState::Done,
                LifecycleState::Blocked,
                LifecycleState::Cancelled
            ]
        );
        assert_eq!(
            machine.valid_transitions(LifecycleState::Blocked),
            vec![LifecycleState::Active]
        );
        assert_eq!(
            machine.valid_transitions(LifecycleState::Done),
            vec![LifecycleState::Archived]
        );
        assert_eq!(
            machine.valid_transitions(LifecycleState::Cancelled),
            vec![LifecycleState::Archived]
        );
        assert_eq!(
            machine.valid_transitions(LifecycleState::Archived),
            Vec::<LifecycleState>::new()
        );
    }

    #[test]
    fn test_transition_success() {
        let machine = StateMachine::new();
        let result = machine.transition(LifecycleState::Draft, LifecycleState::Active);
        assert_eq!(result, Ok(LifecycleState::Active));
    }

    #[test]
    fn test_transition_failure() {
        let machine = StateMachine::new();
        let result = machine.transition(LifecycleState::Draft, LifecycleState::Done);
        assert_eq!(
            result,
            Err(StateError::InvalidTransition {
                from: LifecycleState::Draft,
                to: LifecycleState::Done,
            })
        );
    }

    #[test]
    fn test_clone() {
        let machine = StateMachine::new();
        let cloned = machine.clone();
        assert!(cloned.can_transition(LifecycleState::Draft, LifecycleState::Active));
    }
}
