//! Core Plan type.
//!
//! This module defines the main Plan type used throughout the system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::spec::SpecId;

use super::error::PlanError;
use super::step::{PlanStep, StepStatus};

/// An implementation plan for a specification.
///
/// A Plan breaks down the work required to implement a spec into discrete steps.
/// It tracks progress, complexity, and notes for each step.
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::SpecId;
/// use airsspec_core::plan::{Plan, PlanStep};
///
/// let spec_id = SpecId::new(1_737_734_400, "user-auth");
/// let steps = vec![
///     PlanStep::new(0, "Setup database", "Create schema"),
///     PlanStep::new(1, "Implement API", "Create endpoints"),
/// ];
///
/// let plan = Plan::new(spec_id.clone(), "Incremental implementation", steps);
/// assert_eq!(plan.spec_id(), &spec_id);
/// assert_eq!(plan.step_count(), 2);
/// assert_eq!(plan.completion_percentage(), 0);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    /// ID of the spec this plan belongs to.
    spec_id: SpecId,

    /// High-level approach/strategy for implementation.
    approach: String,

    /// Ordered list of implementation steps.
    steps: Vec<PlanStep>,

    /// When the plan was created.
    created_at: DateTime<Utc>,

    /// When the plan was last updated.
    updated_at: DateTime<Utc>,
}

impl Plan {
    /// Creates a new plan for the given spec.
    ///
    /// # Arguments
    ///
    /// * `spec_id` - ID of the specification this plan implements
    /// * `approach` - High-level implementation strategy
    /// * `steps` - Ordered list of implementation steps
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecId;
    /// use airsspec_core::plan::{Plan, PlanStep};
    ///
    /// let spec_id = SpecId::new(1_737_734_400, "feature");
    /// let steps = vec![PlanStep::new(0, "Step 1", "Description")];
    /// let plan = Plan::new(spec_id, "Strategy", steps);
    /// ```
    #[must_use]
    pub fn new(spec_id: SpecId, approach: impl Into<String>, steps: Vec<PlanStep>) -> Self {
        let now = Utc::now();
        Self {
            spec_id,
            approach: approach.into(),
            steps,
            created_at: now,
            updated_at: now,
        }
    }

    /// Returns a reference to the spec ID.
    #[must_use]
    pub fn spec_id(&self) -> &SpecId {
        &self.spec_id
    }

    /// Returns the implementation approach.
    #[must_use]
    pub fn approach(&self) -> &str {
        &self.approach
    }

    /// Returns a reference to all steps.
    #[must_use]
    pub fn steps(&self) -> &[PlanStep] {
        &self.steps
    }

    /// Returns a mutable reference to all steps.
    pub fn steps_mut(&mut self) -> &mut Vec<PlanStep> {
        &mut self.steps
    }

    /// Returns when the plan was created.
    #[must_use]
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns when the plan was last updated.
    #[must_use]
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Returns the number of steps in the plan.
    #[must_use]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Returns a reference to a step by index.
    #[must_use]
    pub fn step(&self, index: usize) -> Option<&PlanStep> {
        self.steps.get(index)
    }

    /// Returns a mutable reference to a step by index.
    pub fn step_mut(&mut self, index: usize) -> Option<&mut PlanStep> {
        self.steps.get_mut(index)
    }

    /// Sets the implementation approach.
    pub fn set_approach(&mut self, approach: impl Into<String>) {
        self.approach = approach.into();
        self.touch();
    }

    /// Adds a step to the plan.
    pub fn add_step(&mut self, step: PlanStep) {
        self.steps.push(step);
        self.touch();
    }

    /// Updates the `updated_at` timestamp to the current time.
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Marks a step as completed with optional notes.
    ///
    /// # Arguments
    ///
    /// * `index` - The step index to complete
    /// * `notes` - Optional implementation notes
    ///
    /// # Errors
    ///
    /// Returns `PlanError::StepIndexOutOfBounds` if index is invalid.
    pub fn complete_step(&mut self, index: usize, notes: Option<String>) -> Result<(), PlanError> {
        let total = self.steps.len();
        let step = self
            .steps
            .get_mut(index)
            .ok_or(PlanError::StepIndexOutOfBounds { index, total })?;
        step.complete(notes);
        self.touch();
        Ok(())
    }

    /// Returns the completion percentage (0-100).
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecId;
    /// use airsspec_core::plan::{Plan, PlanStep};
    ///
    /// let spec_id = SpecId::new(1_737_734_400, "test");
    /// let mut plan = Plan::new(spec_id, "Strategy", vec![
    ///     PlanStep::new(0, "Step 1", ""),
    ///     PlanStep::new(1, "Step 2", ""),
    /// ]);
    ///
    /// assert_eq!(plan.completion_percentage(), 0);
    /// plan.complete_step(0, None).unwrap();
    /// assert_eq!(plan.completion_percentage(), 50);
    /// ```
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "percentage is always 0-100, fits in u8"
    )]
    pub fn completion_percentage(&self) -> u8 {
        if self.steps.is_empty() {
            return 100;
        }
        let completed = self
            .steps
            .iter()
            .filter(|s| s.status() == StepStatus::Completed)
            .count();
        ((completed * 100) / self.steps.len()) as u8
    }

    /// Returns the number of completed steps.
    #[must_use]
    pub fn completed_steps(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| s.status() == StepStatus::Completed)
            .count()
    }

    /// Returns the number of pending steps.
    #[must_use]
    pub fn pending_steps(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| s.status() == StepStatus::Pending)
            .count()
    }

    /// Returns the number of blocked steps.
    #[must_use]
    pub fn blocked_steps(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| s.status() == StepStatus::Blocked)
            .count()
    }

    /// Returns true if all steps are completed.
    #[must_use]
    pub fn is_completed(&self) -> bool {
        !self.steps.is_empty()
            && self
                .steps
                .iter()
                .all(|s| s.status() == StepStatus::Completed)
    }

    /// Returns true if any step is blocked.
    #[must_use]
    pub fn is_blocked(&self) -> bool {
        self.steps.iter().any(|s| s.status() == StepStatus::Blocked)
    }

    /// Returns the current (first non-completed) step, if any.
    #[must_use]
    pub fn current_step(&self) -> Option<&PlanStep> {
        self.steps
            .iter()
            .find(|s| s.status() != StepStatus::Completed && s.status() != StepStatus::Skipped)
    }

    /// Returns the index of the current (first non-completed) step, if any.
    #[must_use]
    pub fn current_step_index(&self) -> Option<usize> {
        self.steps
            .iter()
            .position(|s| s.status() != StepStatus::Completed && s.status() != StepStatus::Skipped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_spec_id() -> SpecId {
        SpecId::new(1_737_734_400, "test-spec")
    }

    fn test_steps() -> Vec<PlanStep> {
        vec![
            PlanStep::new(0, "Step 1", "First step"),
            PlanStep::new(1, "Step 2", "Second step"),
            PlanStep::new(2, "Step 3", "Third step"),
        ]
    }

    #[test]
    fn test_plan_new() {
        let spec_id = test_spec_id();
        let steps = test_steps();
        let plan = Plan::new(spec_id.clone(), "Test approach", steps);

        assert_eq!(plan.spec_id(), &spec_id);
        assert_eq!(plan.approach(), "Test approach");
        assert_eq!(plan.step_count(), 3);
        assert!(plan.created_at() <= Utc::now());
        assert_eq!(plan.created_at(), plan.updated_at());
    }

    #[test]
    fn test_plan_step_access() {
        let plan = Plan::new(test_spec_id(), "Approach", test_steps());

        assert!(plan.step(0).is_some());
        assert!(plan.step(1).is_some());
        assert!(plan.step(2).is_some());
        assert!(plan.step(3).is_none());

        assert_eq!(plan.step(0).unwrap().title(), "Step 1");
    }

    #[test]
    fn test_plan_step_mut() {
        let mut plan = Plan::new(test_spec_id(), "Approach", test_steps());

        if let Some(step) = plan.step_mut(0) {
            step.set_status(StepStatus::InProgress);
        }

        assert_eq!(plan.step(0).unwrap().status(), StepStatus::InProgress);
    }

    #[test]
    fn test_plan_set_approach() {
        let mut plan = Plan::new(test_spec_id(), "Initial", test_steps());
        let initial_updated = plan.updated_at();

        std::thread::sleep(std::time::Duration::from_millis(10));
        plan.set_approach("Updated approach");

        assert_eq!(plan.approach(), "Updated approach");
        assert!(plan.updated_at() > initial_updated);
    }

    #[test]
    fn test_plan_add_step() {
        let mut plan = Plan::new(test_spec_id(), "Approach", test_steps());
        assert_eq!(plan.step_count(), 3);

        plan.add_step(PlanStep::new(3, "Step 4", "Fourth step"));
        assert_eq!(plan.step_count(), 4);
    }

    #[test]
    fn test_plan_complete_step() {
        let mut plan = Plan::new(test_spec_id(), "Approach", test_steps());

        plan.complete_step(0, Some("Done!".to_string())).unwrap();

        assert!(plan.step(0).unwrap().is_completed());
        assert_eq!(plan.step(0).unwrap().notes(), Some("Done!"));
    }

    #[test]
    fn test_plan_complete_step_out_of_bounds() {
        let mut plan = Plan::new(test_spec_id(), "Approach", test_steps());

        let result = plan.complete_step(10, None);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PlanError::StepIndexOutOfBounds {
                index: 10,
                total: 3
            }
        ));
    }

    #[test]
    fn test_plan_completion_percentage() {
        let mut plan = Plan::new(test_spec_id(), "Approach", test_steps());

        assert_eq!(plan.completion_percentage(), 0);
        plan.complete_step(0, None).unwrap();
        assert_eq!(plan.completion_percentage(), 33); // 1/3
        plan.complete_step(1, None).unwrap();
        assert_eq!(plan.completion_percentage(), 66); // 2/3
        plan.complete_step(2, None).unwrap();
        assert_eq!(plan.completion_percentage(), 100);
    }

    #[test]
    fn test_plan_completion_percentage_empty() {
        let plan = Plan::new(test_spec_id(), "Approach", vec![]);
        assert_eq!(plan.completion_percentage(), 100);
    }

    #[test]
    fn test_plan_step_counts() {
        let mut plan = Plan::new(test_spec_id(), "Approach", test_steps());

        assert_eq!(plan.completed_steps(), 0);
        assert_eq!(plan.pending_steps(), 3);
        assert_eq!(plan.blocked_steps(), 0);

        plan.complete_step(0, None).unwrap();
        plan.step_mut(1).unwrap().set_status(StepStatus::Blocked);

        assert_eq!(plan.completed_steps(), 1);
        assert_eq!(plan.pending_steps(), 1);
        assert_eq!(plan.blocked_steps(), 1);
    }

    #[test]
    fn test_plan_is_completed() {
        let mut plan = Plan::new(test_spec_id(), "Approach", test_steps());
        assert!(!plan.is_completed());

        for i in 0..3 {
            plan.complete_step(i, None).unwrap();
        }
        assert!(plan.is_completed());
    }

    #[test]
    fn test_plan_is_completed_empty() {
        let plan = Plan::new(test_spec_id(), "Approach", vec![]);
        assert!(!plan.is_completed()); // Empty plan is not completed
    }

    #[test]
    fn test_plan_is_blocked() {
        let mut plan = Plan::new(test_spec_id(), "Approach", test_steps());
        assert!(!plan.is_blocked());

        plan.step_mut(1).unwrap().set_status(StepStatus::Blocked);
        assert!(plan.is_blocked());
    }

    #[test]
    fn test_plan_current_step() {
        let mut plan = Plan::new(test_spec_id(), "Approach", test_steps());

        assert_eq!(plan.current_step_index(), Some(0));
        assert_eq!(plan.current_step().unwrap().title(), "Step 1");

        plan.complete_step(0, None).unwrap();
        assert_eq!(plan.current_step_index(), Some(1));
        assert_eq!(plan.current_step().unwrap().title(), "Step 2");
    }

    #[test]
    fn test_plan_current_step_skipped() {
        let mut plan = Plan::new(test_spec_id(), "Approach", test_steps());

        plan.step_mut(0).unwrap().set_status(StepStatus::Skipped);
        assert_eq!(plan.current_step_index(), Some(1));
    }

    #[test]
    fn test_plan_serde_roundtrip() {
        let mut plan = Plan::new(test_spec_id(), "Test approach", test_steps());
        plan.complete_step(0, Some("Notes".to_string())).unwrap();

        let json = serde_json::to_string(&plan).unwrap();
        let parsed: Plan = serde_json::from_str(&json).unwrap();

        assert_eq!(plan.spec_id(), parsed.spec_id());
        assert_eq!(plan.approach(), parsed.approach());
        assert_eq!(plan.step_count(), parsed.step_count());
    }

    #[test]
    fn test_plan_clone() {
        let plan = Plan::new(test_spec_id(), "Approach", test_steps());
        let cloned = plan.clone();
        assert_eq!(plan, cloned);
    }
}
