//! Build progress tracking.

use serde::{Deserialize, Serialize};

/// Build progress tracking for the build phase.
///
/// Tracks the number of completed steps out of total steps.
///
/// # Examples
///
/// ```
/// use airsspec_core::state::BuildProgress;
///
/// let progress = BuildProgress::new(5);
/// assert_eq!(progress.percentage(), 0);
///
/// let progress = progress.with_completed(3);
/// assert_eq!(progress.percentage(), 60);
/// assert!(!progress.is_complete());
///
/// let progress = progress.with_completed(5);
/// assert!(progress.is_complete());
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildProgress {
    /// Total number of steps in the plan.
    total_steps: usize,
    /// Number of completed steps.
    completed_steps: usize,
    /// Current step being worked on.
    current_step: Option<String>,
    /// Build notes.
    notes: Option<String>,
}

impl BuildProgress {
    /// Creates new build progress with the given total steps.
    #[must_use]
    pub fn new(total_steps: usize) -> Self {
        Self {
            total_steps,
            completed_steps: 0,
            current_step: None,
            notes: None,
        }
    }

    /// Returns the total number of steps.
    #[must_use]
    pub fn total_steps(&self) -> usize {
        self.total_steps
    }

    /// Returns the number of completed steps.
    #[must_use]
    pub fn completed_steps(&self) -> usize {
        self.completed_steps
    }

    /// Returns the completion percentage (0-100).
    ///
    /// Returns 0 if `total_steps` is 0.
    #[must_use]
    pub fn percentage(&self) -> u8 {
        if self.total_steps == 0 {
            return 0;
        }
        let pct = (self.completed_steps * 100) / self.total_steps;
        // Clamp to 100, then convert to u8
        // Safe: pct.min(100) guarantees result fits in u8
        u8::try_from(pct.min(100)).unwrap_or(100)
    }

    /// Returns the current step being worked on, if any.
    #[must_use]
    pub fn current_step(&self) -> Option<&str> {
        self.current_step.as_deref()
    }

    /// Returns the build notes, if any.
    #[must_use]
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    /// Returns `true` if all steps are complete.
    ///
    /// Returns `false` if `total_steps` is 0.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.total_steps > 0 && self.completed_steps >= self.total_steps
    }

    /// Sets the number of completed steps, clamped to `total_steps`.
    #[must_use]
    pub fn with_completed(mut self, completed: usize) -> Self {
        self.completed_steps = completed.min(self.total_steps);
        self
    }

    /// Sets the current step name.
    #[must_use]
    pub fn with_current_step(mut self, step: impl Into<String>) -> Self {
        self.current_step = Some(step.into());
        self
    }

    /// Sets the build notes.
    #[must_use]
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Increments the completed steps by 1, respecting the maximum.
    pub fn complete_step(&mut self) {
        if self.completed_steps < self.total_steps {
            self.completed_steps += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let progress = BuildProgress::new(10);
        assert_eq!(progress.total_steps(), 10);
        assert_eq!(progress.completed_steps(), 0);
        assert!(progress.current_step().is_none());
        assert!(progress.notes().is_none());
    }

    #[test]
    fn test_default() {
        let progress = BuildProgress::default();
        assert_eq!(progress.total_steps(), 0);
        assert_eq!(progress.completed_steps(), 0);
        assert!(progress.current_step().is_none());
        assert!(progress.notes().is_none());
    }

    #[test]
    fn test_percentage() {
        let progress = BuildProgress::new(10).with_completed(0);
        assert_eq!(progress.percentage(), 0);

        let progress = BuildProgress::new(3).with_completed(1);
        assert_eq!(progress.percentage(), 33);

        let progress = BuildProgress::new(10).with_completed(5);
        assert_eq!(progress.percentage(), 50);

        let progress = BuildProgress::new(10).with_completed(10);
        assert_eq!(progress.percentage(), 100);
    }

    #[test]
    fn test_percentage_zero_total() {
        let progress = BuildProgress::new(0);
        assert_eq!(progress.percentage(), 0);
    }

    #[test]
    fn test_is_complete() {
        let progress = BuildProgress::new(5).with_completed(4);
        assert!(!progress.is_complete());

        let progress = BuildProgress::new(5).with_completed(5);
        assert!(progress.is_complete());

        // Zero total is never complete
        let progress = BuildProgress::new(0);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_with_completed_clamped() {
        let progress = BuildProgress::new(5).with_completed(10);
        assert_eq!(progress.completed_steps(), 5); // Clamped to total
        assert_eq!(progress.percentage(), 100);
    }

    #[test]
    fn test_with_current_step() {
        let progress = BuildProgress::new(5).with_current_step("Step 1: Setup");
        assert_eq!(progress.current_step(), Some("Step 1: Setup"));
    }

    #[test]
    fn test_with_notes() {
        let progress = BuildProgress::new(5).with_notes("Making good progress");
        assert_eq!(progress.notes(), Some("Making good progress"));
    }

    #[test]
    fn test_complete_step() {
        let mut progress = BuildProgress::new(3);
        assert_eq!(progress.completed_steps(), 0);

        progress.complete_step();
        assert_eq!(progress.completed_steps(), 1);

        progress.complete_step();
        assert_eq!(progress.completed_steps(), 2);

        progress.complete_step();
        assert_eq!(progress.completed_steps(), 3);

        // Should not go beyond total
        progress.complete_step();
        assert_eq!(progress.completed_steps(), 3);
    }

    #[test]
    fn test_serde_roundtrip() {
        let progress = BuildProgress::new(5)
            .with_completed(2)
            .with_current_step("Step 3")
            .with_notes("In progress");

        let json = serde_json::to_string(&progress).unwrap();
        let parsed: BuildProgress = serde_json::from_str(&json).unwrap();

        assert_eq!(progress, parsed);
    }

    #[test]
    fn test_builder_chaining() {
        let progress = BuildProgress::new(10)
            .with_completed(3)
            .with_current_step("Database setup")
            .with_notes("Working on schema");

        assert_eq!(progress.total_steps(), 10);
        assert_eq!(progress.completed_steps(), 3);
        assert_eq!(progress.current_step(), Some("Database setup"));
        assert_eq!(progress.notes(), Some("Working on schema"));
        assert_eq!(progress.percentage(), 30);
    }
}
