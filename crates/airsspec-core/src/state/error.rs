//! State module errors.
//!
//! Per ADR-005 (Error Handling Strategy), each domain has its own error type.

use thiserror::Error;

use crate::shared::LifecycleState;

/// Errors related to state transitions.
///
/// # Examples
///
/// ```
/// use airsspec_core::state::StateError;
/// use airsspec_core::shared::LifecycleState;
///
/// let err = StateError::InvalidTransition {
///     from: LifecycleState::Draft,
///     to: LifecycleState::Done,
/// };
/// assert!(err.to_string().contains("invalid transition"));
/// ```
#[non_exhaustive]
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum StateError {
    /// Invalid state transition attempted.
    #[error("invalid transition from {from} to {to}")]
    InvalidTransition {
        /// The current state.
        from: LifecycleState,
        /// The target state.
        to: LifecycleState,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = StateError::InvalidTransition {
            from: LifecycleState::Draft,
            to: LifecycleState::Done,
        };
        let msg = err.to_string();
        assert!(msg.contains("invalid transition"));
        assert!(msg.contains("draft"));
        assert!(msg.contains("done"));
    }

    #[test]
    fn test_error_clone() {
        let err = StateError::InvalidTransition {
            from: LifecycleState::Active,
            to: LifecycleState::Draft,
        };
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn test_error_is_std_error() {
        let err = StateError::InvalidTransition {
            from: LifecycleState::Draft,
            to: LifecycleState::Archived,
        };
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_error_debug() {
        let err = StateError::InvalidTransition {
            from: LifecycleState::Draft,
            to: LifecycleState::Done,
        };
        let debug = format!("{err:?}");
        assert!(debug.contains("InvalidTransition"));
    }
}
