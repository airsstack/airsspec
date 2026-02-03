//! Builder pattern for constructing Plan instances.
//!
//! Per ADR-002 (modular monolith), the builder is in its own file.

use crate::spec::SpecId;

use super::error::PlanError;
use super::step::PlanStep;
use super::types::Plan;

/// Builder for constructing [`Plan`] instances with validation.
///
/// Provides a fluent API for creating plans. The `build()` method
/// validates all required fields.
///
/// # Required Fields
///
/// - `spec_id` - Must be set via `spec_id()` method
///
/// # Optional Fields
///
/// - `approach` - Defaults to empty string
/// - `steps` - At least one step is required
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::SpecId;
/// use airsspec_core::plan::{PlanBuilder, PlanStep, Complexity};
///
/// let spec_id = SpecId::new(1_737_734_400, "user-auth");
///
/// let plan = PlanBuilder::new()
///     .spec_id(spec_id)
///     .approach("Incremental implementation")
///     .step(PlanStep::new(0, "Setup database", "Create schema"))
///     .step(PlanStep::new(1, "Implement API", "Create endpoints"))
///     .build()
///     .unwrap();
///
/// assert_eq!(plan.step_count(), 2);
/// ```
#[derive(Debug, Default)]
pub struct PlanBuilder {
    /// The spec ID (required).
    spec_id: Option<SpecId>,

    /// High-level implementation approach.
    approach: Option<String>,

    /// Implementation steps.
    steps: Vec<PlanStep>,
}

impl PlanBuilder {
    /// Creates a new `PlanBuilder` with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::plan::PlanBuilder;
    ///
    /// let builder = PlanBuilder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the spec ID for the plan (required).
    ///
    /// # Arguments
    ///
    /// * `spec_id` - The ID of the spec this plan implements
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecId;
    /// use airsspec_core::plan::PlanBuilder;
    ///
    /// let builder = PlanBuilder::new()
    ///     .spec_id(SpecId::new(1_737_734_400, "feature"));
    /// ```
    #[must_use]
    pub fn spec_id(mut self, spec_id: SpecId) -> Self {
        self.spec_id = Some(spec_id);
        self
    }

    /// Sets the implementation approach.
    ///
    /// # Arguments
    ///
    /// * `approach` - High-level strategy description
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::plan::PlanBuilder;
    ///
    /// let builder = PlanBuilder::new()
    ///     .approach("Bottom-up implementation");
    /// ```
    #[must_use]
    pub fn approach(mut self, approach: impl Into<String>) -> Self {
        self.approach = Some(approach.into());
        self
    }

    /// Adds a step to the plan.
    ///
    /// Can be called multiple times to add multiple steps.
    ///
    /// # Arguments
    ///
    /// * `step` - A plan step to add
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::plan::{PlanBuilder, PlanStep};
    ///
    /// let builder = PlanBuilder::new()
    ///     .step(PlanStep::new(0, "First step", "Description"));
    /// ```
    #[must_use]
    pub fn step(mut self, step: PlanStep) -> Self {
        self.steps.push(step);
        self
    }

    /// Adds multiple steps to the plan.
    ///
    /// # Arguments
    ///
    /// * `steps` - An iterator of plan steps
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::plan::{PlanBuilder, PlanStep};
    ///
    /// let steps = vec![
    ///     PlanStep::new(0, "Step 1", "First"),
    ///     PlanStep::new(1, "Step 2", "Second"),
    /// ];
    ///
    /// let builder = PlanBuilder::new()
    ///     .steps(steps);
    /// ```
    #[must_use]
    pub fn steps(mut self, steps: impl IntoIterator<Item = PlanStep>) -> Self {
        self.steps.extend(steps);
        self
    }

    /// Builds the plan, validating all required fields.
    ///
    /// # Errors
    ///
    /// Returns `PlanError::MissingField` if:
    /// - `spec_id` is not set
    ///
    /// Returns `PlanError::InvalidFormat` if:
    /// - No steps have been added
    ///
    /// # Examples
    ///
    /// ```
    /// use airsspec_core::spec::SpecId;
    /// use airsspec_core::plan::{PlanBuilder, PlanStep};
    ///
    /// // Successful build
    /// let plan = PlanBuilder::new()
    ///     .spec_id(SpecId::new(1_737_734_400, "feature"))
    ///     .step(PlanStep::new(0, "Step", "Desc"))
    ///     .build()
    ///     .unwrap();
    ///
    /// // Missing spec_id returns error
    /// let result = PlanBuilder::new()
    ///     .step(PlanStep::new(0, "Step", "Desc"))
    ///     .build();
    /// assert!(result.is_err());
    ///
    /// // No steps returns error
    /// let result = PlanBuilder::new()
    ///     .spec_id(SpecId::new(1_737_734_400, "feature"))
    ///     .build();
    /// assert!(result.is_err());
    /// ```
    pub fn build(self) -> Result<Plan, PlanError> {
        // Validate required field: spec_id
        let spec_id = self
            .spec_id
            .ok_or_else(|| PlanError::MissingField("spec_id".to_string()))?;

        // Validate at least one step
        if self.steps.is_empty() {
            return Err(PlanError::InvalidFormat(
                "plan must have at least one step".to_string(),
            ));
        }

        // Build the plan
        Ok(Plan::new(
            spec_id,
            self.approach.unwrap_or_default(),
            self.steps,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_spec_id() -> SpecId {
        SpecId::new(1_737_734_400, "test-spec")
    }

    #[test]
    fn test_builder_minimal() {
        let plan = PlanBuilder::new()
            .spec_id(test_spec_id())
            .step(PlanStep::new(0, "Step", "Description"))
            .build()
            .unwrap();

        assert_eq!(plan.spec_id(), &test_spec_id());
        assert!(plan.approach().is_empty());
        assert_eq!(plan.step_count(), 1);
    }

    #[test]
    fn test_builder_full() {
        let plan = PlanBuilder::new()
            .spec_id(test_spec_id())
            .approach("Incremental implementation")
            .step(PlanStep::new(0, "Step 1", "First"))
            .step(PlanStep::new(1, "Step 2", "Second"))
            .build()
            .unwrap();

        assert_eq!(plan.spec_id(), &test_spec_id());
        assert_eq!(plan.approach(), "Incremental implementation");
        assert_eq!(plan.step_count(), 2);
    }

    #[test]
    fn test_builder_with_steps_batch() {
        let steps = vec![
            PlanStep::new(0, "Step 1", "First"),
            PlanStep::new(1, "Step 2", "Second"),
            PlanStep::new(2, "Step 3", "Third"),
        ];

        let plan = PlanBuilder::new()
            .spec_id(test_spec_id())
            .steps(steps)
            .build()
            .unwrap();

        assert_eq!(plan.step_count(), 3);
    }

    #[test]
    fn test_builder_missing_spec_id() {
        let result = PlanBuilder::new()
            .step(PlanStep::new(0, "Step", "Desc"))
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PlanError::MissingField(_)));
        assert!(err.to_string().contains("spec_id"));
    }

    #[test]
    fn test_builder_no_steps() {
        let result = PlanBuilder::new().spec_id(test_spec_id()).build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PlanError::InvalidFormat(_)));
        assert!(err.to_string().contains("at least one step"));
    }

    #[test]
    fn test_builder_chain_order_independent() {
        let plan1 = PlanBuilder::new()
            .spec_id(test_spec_id())
            .approach("Approach")
            .step(PlanStep::new(0, "Step", "Desc"))
            .build()
            .unwrap();

        let plan2 = PlanBuilder::new()
            .step(PlanStep::new(0, "Step", "Desc"))
            .approach("Approach")
            .spec_id(test_spec_id())
            .build()
            .unwrap();

        assert_eq!(plan1.spec_id(), plan2.spec_id());
        assert_eq!(plan1.approach(), plan2.approach());
        assert_eq!(plan1.step_count(), plan2.step_count());
    }

    #[test]
    fn test_builder_default() {
        let builder = PlanBuilder::default();
        let result = builder.build();
        assert!(result.is_err()); // No spec_id set
    }

    #[test]
    fn test_builder_debug() {
        let builder = PlanBuilder::new()
            .spec_id(test_spec_id())
            .approach("Test approach");
        let debug = format!("{builder:?}");
        assert!(debug.contains("PlanBuilder"));
    }
}
