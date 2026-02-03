//! Plan step types and related enums.
//!
//! This module defines the building blocks for implementation plans.

use serde::{Deserialize, Serialize};

/// Status of a plan step.
///
/// # Examples
///
/// ```
/// use airsspec_core::plan::StepStatus;
///
/// let status = StepStatus::default();
/// assert_eq!(status, StepStatus::Pending);
/// assert_eq!(format!("{status}"), "pending");
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    /// Step is waiting to be started.
    #[default]
    Pending,
    /// Step is currently being worked on.
    InProgress,
    /// Step is completed successfully.
    Completed,
    /// Step is blocked by an external dependency.
    Blocked,
    /// Step was intentionally skipped.
    Skipped,
}

impl std::fmt::Display for StepStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Blocked => "blocked",
            Self::Skipped => "skipped",
        };
        write!(f, "{s}")
    }
}

/// Complexity estimate for a plan step.
///
/// Used to estimate effort and plan work distribution.
///
/// # Examples
///
/// ```
/// use airsspec_core::plan::Complexity;
///
/// let complexity = Complexity::default();
/// assert_eq!(complexity, Complexity::Medium);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Complexity {
    /// Trivial change, minimal effort.
    Trivial,
    /// Simple change, low effort.
    Simple,
    /// Moderate complexity, typical effort.
    #[default]
    Medium,
    /// Complex change, significant effort.
    Complex,
}

impl std::fmt::Display for Complexity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Trivial => "trivial",
            Self::Simple => "simple",
            Self::Medium => "medium",
            Self::Complex => "complex",
        };
        write!(f, "{s}")
    }
}

/// A single step in an implementation plan.
///
/// Each step represents a discrete unit of work within the plan.
/// Steps track status, complexity, and optional notes added during implementation.
///
/// # Examples
///
/// ```
/// use airsspec_core::plan::{PlanStep, StepStatus, Complexity};
///
/// let step = PlanStep::new(0, "Setup database schema", "Create tables for user data");
/// assert_eq!(step.index(), 0);
/// assert_eq!(step.status(), StepStatus::Pending);
/// assert_eq!(step.complexity(), Complexity::Medium);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanStep {
    /// Step index (0-based).
    index: usize,

    /// Brief title of the step.
    title: String,

    /// Detailed description of what needs to be done.
    description: String,

    /// Estimated complexity.
    complexity: Complexity,

    /// Current status of the step.
    status: StepStatus,

    /// Implementation notes (filled in during build phase).
    notes: Option<String>,
}

impl PlanStep {
    /// Creates a new plan step with default status and complexity.
    ///
    /// # Arguments
    ///
    /// * `index` - The step index (0-based)
    /// * `title` - Brief title describing the step
    /// * `description` - Detailed description of the work
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::plan::PlanStep;
    ///
    /// let step = PlanStep::new(0, "Setup database", "Create schema and tables");
    /// ```
    #[must_use]
    pub fn new(index: usize, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            index,
            title: title.into(),
            description: description.into(),
            complexity: Complexity::default(),
            status: StepStatus::default(),
            notes: None,
        }
    }

    /// Returns the step index.
    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the step title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the step description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the step complexity.
    #[must_use]
    pub fn complexity(&self) -> Complexity {
        self.complexity
    }

    /// Returns the step status.
    #[must_use]
    pub fn status(&self) -> StepStatus {
        self.status
    }

    /// Returns the implementation notes, if any.
    #[must_use]
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    /// Sets the step complexity.
    pub fn set_complexity(&mut self, complexity: Complexity) {
        self.complexity = complexity;
    }

    /// Sets the step status.
    pub fn set_status(&mut self, status: StepStatus) {
        self.status = status;
    }

    /// Sets implementation notes.
    pub fn set_notes(&mut self, notes: impl Into<String>) {
        self.notes = Some(notes.into());
    }

    /// Clears implementation notes.
    pub fn clear_notes(&mut self) {
        self.notes = None;
    }

    /// Marks the step as completed with optional notes.
    pub fn complete(&mut self, notes: Option<String>) {
        self.status = StepStatus::Completed;
        self.notes = notes;
    }

    /// Returns true if the step is completed.
    #[must_use]
    pub fn is_completed(&self) -> bool {
        self.status == StepStatus::Completed
    }

    /// Returns true if the step is blocked.
    #[must_use]
    pub fn is_blocked(&self) -> bool {
        self.status == StepStatus::Blocked
    }
}

/// Builder for constructing [`PlanStep`] instances.
///
/// # Examples
///
/// ```
/// use airsspec_core::plan::{StepBuilder, Complexity};
///
/// let step = StepBuilder::new()
///     .index(0)
///     .title("Setup database")
///     .description("Create schema and tables")
///     .complexity(Complexity::Simple)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default)]
pub struct StepBuilder {
    index: Option<usize>,
    title: Option<String>,
    description: Option<String>,
    complexity: Option<Complexity>,
}

impl StepBuilder {
    /// Creates a new `StepBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the step index.
    #[must_use]
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Sets the step title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the step description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the step complexity.
    #[must_use]
    pub fn complexity(mut self, complexity: Complexity) -> Self {
        self.complexity = Some(complexity);
        self
    }

    /// Builds the `PlanStep`.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields (index, title) are missing.
    pub fn build(self) -> Result<PlanStep, super::error::PlanError> {
        let index = self
            .index
            .ok_or_else(|| super::error::PlanError::MissingField("index".to_string()))?;
        let title = self
            .title
            .ok_or_else(|| super::error::PlanError::MissingField("title".to_string()))?;

        if title.is_empty() {
            return Err(super::error::PlanError::MissingField(
                "title cannot be empty".to_string(),
            ));
        }

        Ok(PlanStep {
            index,
            title,
            description: self.description.unwrap_or_default(),
            complexity: self.complexity.unwrap_or_default(),
            status: StepStatus::default(),
            notes: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // StepStatus tests
    #[test]
    fn test_step_status_default() {
        assert_eq!(StepStatus::default(), StepStatus::Pending);
    }

    #[test]
    fn test_step_status_display() {
        assert_eq!(format!("{}", StepStatus::Pending), "pending");
        assert_eq!(format!("{}", StepStatus::InProgress), "in_progress");
        assert_eq!(format!("{}", StepStatus::Completed), "completed");
        assert_eq!(format!("{}", StepStatus::Blocked), "blocked");
        assert_eq!(format!("{}", StepStatus::Skipped), "skipped");
    }

    #[test]
    fn test_step_status_serde() {
        let status = StepStatus::Completed;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"completed\"");
        let parsed: StepStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, parsed);
    }

    // Complexity tests
    #[test]
    fn test_complexity_default() {
        assert_eq!(Complexity::default(), Complexity::Medium);
    }

    #[test]
    fn test_complexity_display() {
        assert_eq!(format!("{}", Complexity::Trivial), "trivial");
        assert_eq!(format!("{}", Complexity::Simple), "simple");
        assert_eq!(format!("{}", Complexity::Medium), "medium");
        assert_eq!(format!("{}", Complexity::Complex), "complex");
    }

    #[test]
    fn test_complexity_serde() {
        let complexity = Complexity::Simple;
        let json = serde_json::to_string(&complexity).unwrap();
        assert_eq!(json, "\"simple\"");
        let parsed: Complexity = serde_json::from_str(&json).unwrap();
        assert_eq!(complexity, parsed);
    }

    // PlanStep tests
    #[test]
    fn test_plan_step_new() {
        let step = PlanStep::new(0, "Test Step", "Test description");
        assert_eq!(step.index(), 0);
        assert_eq!(step.title(), "Test Step");
        assert_eq!(step.description(), "Test description");
        assert_eq!(step.complexity(), Complexity::Medium);
        assert_eq!(step.status(), StepStatus::Pending);
        assert!(step.notes().is_none());
    }

    #[test]
    fn test_plan_step_setters() {
        let mut step = PlanStep::new(0, "Test", "Desc");

        step.set_complexity(Complexity::Complex);
        assert_eq!(step.complexity(), Complexity::Complex);

        step.set_status(StepStatus::InProgress);
        assert_eq!(step.status(), StepStatus::InProgress);

        step.set_notes("Some notes");
        assert_eq!(step.notes(), Some("Some notes"));

        step.clear_notes();
        assert!(step.notes().is_none());
    }

    #[test]
    fn test_plan_step_complete() {
        let mut step = PlanStep::new(0, "Test", "Desc");
        step.complete(Some("Done!".to_string()));

        assert!(step.is_completed());
        assert_eq!(step.notes(), Some("Done!"));
    }

    #[test]
    fn test_plan_step_is_blocked() {
        let mut step = PlanStep::new(0, "Test", "Desc");
        assert!(!step.is_blocked());

        step.set_status(StepStatus::Blocked);
        assert!(step.is_blocked());
    }

    #[test]
    fn test_plan_step_serde_roundtrip() {
        let mut step = PlanStep::new(0, "Serde Test", "Testing serialization");
        step.set_complexity(Complexity::Simple);
        step.set_notes("Test notes");

        let json = serde_json::to_string(&step).unwrap();
        let parsed: PlanStep = serde_json::from_str(&json).unwrap();

        assert_eq!(step, parsed);
    }

    // StepBuilder tests
    #[test]
    fn test_step_builder_minimal() {
        let step = StepBuilder::new()
            .index(0)
            .title("Test")
            .build()
            .unwrap();

        assert_eq!(step.index(), 0);
        assert_eq!(step.title(), "Test");
        assert!(step.description().is_empty());
        assert_eq!(step.complexity(), Complexity::Medium);
    }

    #[test]
    fn test_step_builder_full() {
        let step = StepBuilder::new()
            .index(1)
            .title("Full Step")
            .description("Full description")
            .complexity(Complexity::Complex)
            .build()
            .unwrap();

        assert_eq!(step.index(), 1);
        assert_eq!(step.title(), "Full Step");
        assert_eq!(step.description(), "Full description");
        assert_eq!(step.complexity(), Complexity::Complex);
    }

    #[test]
    fn test_step_builder_missing_index() {
        let result = StepBuilder::new().title("Test").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_step_builder_missing_title() {
        let result = StepBuilder::new().index(0).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_step_builder_empty_title() {
        let result = StepBuilder::new().index(0).title("").build();
        assert!(result.is_err());
    }
}
