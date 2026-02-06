//! # Wizard State
//!
//! Navigation state tracker for multi-step wizards.
//!
//! [`WizardState`] manages the current step index and provides
//! boundary-safe forward/backward navigation, serving as part of
//! the "Model" in the Elm Architecture (TEA) pattern.

/// Tracks the current position within a multi-step wizard.
///
/// Provides boundary-safe navigation methods that prevent stepping
/// past the first or last step. Step indices are 0-based internally,
/// with [`WizardState::display_step`] returning a 1-based number
/// suitable for user-facing display.
#[derive(Debug, Clone)]
pub struct WizardState {
    current: usize,
    total: usize,
}

impl WizardState {
    /// Creates a new wizard state for the given number of steps.
    ///
    /// Starts at step 0 (the first step).
    #[must_use]
    pub fn new(total_steps: usize) -> Self {
        Self {
            current: 0,
            total: total_steps,
        }
    }

    /// Returns the current step index (0-based).
    #[must_use]
    pub fn current(&self) -> usize {
        self.current
    }

    /// Returns the total number of steps.
    #[must_use]
    pub fn total(&self) -> usize {
        self.total
    }

    /// Returns the current step number for display (1-based).
    #[must_use]
    pub fn display_step(&self) -> usize {
        self.current + 1
    }

    /// Returns `true` if the current step is the first step.
    #[must_use]
    pub fn is_first(&self) -> bool {
        self.current == 0
    }

    /// Returns `true` if the current step is the last step.
    #[must_use]
    pub fn is_last(&self) -> bool {
        self.current == self.total.saturating_sub(1)
    }

    /// Advances to the next step if not already at the last step.
    ///
    /// Returns `true` if navigation occurred, `false` if already at the end.
    #[allow(
        clippy::should_implement_trait,
        reason = "not an Iterator; 'next' is the natural name for wizard navigation"
    )]
    pub fn next(&mut self) -> bool {
        if self.current < self.total.saturating_sub(1) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// Moves to the previous step if not already at the first step.
    ///
    /// Returns `true` if navigation occurred, `false` if already at the start.
    pub fn previous(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state() {
        let state = WizardState::new(3);
        assert_eq!(state.current(), 0);
        assert_eq!(state.display_step(), 1);
        assert_eq!(state.total(), 3);
        assert!(state.is_first());
        assert!(!state.is_last());
    }

    #[test]
    fn test_navigation() {
        let mut state = WizardState::new(3);

        // Forward navigation
        assert!(state.next());
        assert_eq!(state.current(), 1);
        assert!(!state.is_first());
        assert!(!state.is_last());

        assert!(state.next());
        assert_eq!(state.current(), 2);
        assert!(state.is_last());

        // Cannot go past last step
        assert!(!state.next());
        assert_eq!(state.current(), 2);

        // Backward navigation
        assert!(state.previous());
        assert_eq!(state.current(), 1);

        assert!(state.previous());
        assert_eq!(state.current(), 0);
        assert!(state.is_first());
    }

    #[test]
    fn test_first_step_cannot_go_back() {
        let mut state = WizardState::new(3);
        assert!(state.is_first());

        let moved = state.previous();
        assert!(!moved);
        assert_eq!(state.current(), 0);
    }
}
