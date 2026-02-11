//! State transition validator.
//!
//! Validates that specifications have the required artifacts for their
//! current workflow state. For example, a spec in the Plan or Build phase
//! should have an associated plan.
//!
//! Issues are reported as warnings rather than errors because specs may
//! legitimately be in a transitional state.
//!
//! Uses [`ValidatableSpec`] and [`ValidatablePlan`] trait abstractions
//! for DIP compliance.

use std::collections::HashSet;

use crate::validation::context::ValidationContext;
use crate::validation::issue::ValidationIssue;
use crate::validation::report::ValidationReport;
use crate::validation::traits::{ValidatablePlan, ValidatableSpec};
use crate::validation::validator::Validator;

/// Validates workspace consistency between spec states and their artifacts.
///
/// Checks:
/// - Specs that have plans but the plan has no steps (warning)
///
/// Issues are reported as **warnings** (not errors) because the workspace
/// may be in a transitional state. The state machine already enforces
/// valid transitions at transition time.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use airsspec_core::validation::{
///     Validator, ValidationContextBuilder, StateTransitionValidator,
///     ValidatableSpec, ValidatablePlan, ValidationReport,
/// };
///
/// // StateTransitionValidator requires S: ValidatableSpec + P: ValidatablePlan
/// struct MockSpec { id: String }
/// impl ValidatableSpec for MockSpec {
///     fn id_str(&self) -> &str { &self.id }
///     fn dependency_ids(&self) -> Vec<&str> { vec![] }
///     fn validate_content(&self) -> ValidationReport { ValidationReport::new() }
/// }
///
/// struct MockPlan { spec_id: String, steps: usize, done: bool }
/// impl ValidatablePlan for MockPlan {
///     fn spec_id_str(&self) -> &str { &self.spec_id }
///     fn step_count(&self) -> usize { self.steps }
///     fn is_completed(&self) -> bool { self.done }
/// }
///
/// let context = ValidationContextBuilder::new()
///     .workspace_path(PathBuf::from("/project"))
///     .specs(vec![MockSpec { id: "1000000-my-spec".into() }])
///     .plans(vec![MockPlan {
///         spec_id: "1000000-my-spec".into(),
///         steps: 3,
///         done: false,
///     }])
///     .build();
///
/// let validator = StateTransitionValidator;
/// let report = validator.validate(&context);
/// assert!(report.is_valid());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct StateTransitionValidator;

impl<S, P> Validator<ValidationContext<S, P>> for StateTransitionValidator
where
    S: ValidatableSpec,
    P: ValidatablePlan,
{
    fn name(&self) -> &'static str {
        "state-transition"
    }

    fn validate(&self, context: &ValidationContext<S, P>) -> ValidationReport {
        let mut report = ValidationReport::new();
        let specs = context.specs();
        let plans = context.plans();

        if specs.is_empty() {
            return report;
        }

        // Build set of spec IDs that have plans
        let plan_spec_ids: HashSet<&str> = plans.iter().map(ValidatablePlan::spec_id_str).collect();

        for spec in specs {
            let spec_id = spec.id_str();

            // Check: specs that have plans but the plan has no steps
            if plan_spec_ids.contains(spec_id) {
                let plan = plans.iter().find(|p| p.spec_id_str() == spec_id);
                if let Some(plan) = plan
                    && plan.step_count() == 0
                {
                    report.add_issue(
                        ValidationIssue::warning(format!(
                            "Spec '{spec_id}' has a plan with no steps"
                        ))
                        .with_field(format!("[{spec_id}] plan.steps")),
                    );
                }
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::plan::{Plan, PlanStep};
    use crate::spec::{Spec, SpecId, SpecMetadata};
    use crate::validation::ValidationContextBuilder;

    fn make_spec(timestamp: i64, slug: &str) -> Spec {
        Spec::new(
            SpecId::new(timestamp, slug),
            SpecMetadata::new(slug, "Description"),
            "Content",
        )
    }

    fn make_plan(timestamp: i64, slug: &str, steps: Vec<PlanStep>) -> Plan {
        Plan::new(SpecId::new(timestamp, slug), "Test approach", steps)
    }

    fn make_context(specs: Vec<Spec>, plans: Vec<Plan>) -> ValidationContext<Spec, Plan> {
        ValidationContextBuilder::new()
            .workspace_path(PathBuf::from("/project"))
            .specs(specs)
            .plans(plans)
            .build()
    }

    #[test]
    fn test_empty_context_is_valid() {
        let context = make_context(vec![], vec![]);
        let validator = StateTransitionValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_spec_without_plan_is_valid() {
        let spec = make_spec(1_000_000, "draft-spec");
        let context = make_context(vec![spec], vec![]);
        let validator = StateTransitionValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_spec_with_valid_plan_passes() {
        let spec = make_spec(1_000_000, "planned-spec");
        let plan = make_plan(
            1_000_000,
            "planned-spec",
            vec![PlanStep::new(0, "Step 1", "Do something")],
        );

        let context = make_context(vec![spec], vec![plan]);
        let validator = StateTransitionValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_spec_with_empty_plan_warns() {
        let spec = make_spec(1_000_000, "empty-plan-spec");
        let plan = make_plan(1_000_000, "empty-plan-spec", vec![]);

        let context = make_context(vec![spec], vec![plan]);
        let validator = StateTransitionValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid()); // Warnings only
        assert_eq!(report.warning_count(), 1);
        assert!(report.warnings()[0].message().contains("no steps"));
    }

    #[test]
    fn test_spec_with_completed_plan_passes() {
        let spec = make_spec(1_000_000, "done-spec");
        let mut plan = make_plan(
            1_000_000,
            "done-spec",
            vec![PlanStep::new(0, "Step 1", "Do something")],
        );
        plan.complete_step(0, None).unwrap();

        let context = make_context(vec![spec], vec![plan]);
        let validator = StateTransitionValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_multiple_specs_mixed() {
        let spec_a = make_spec(1_000_000, "spec-a");
        let spec_b = make_spec(1_000_001, "spec-b");

        let plan_a = make_plan(1_000_000, "spec-a", vec![PlanStep::new(0, "Step", "desc")]);
        let plan_b = make_plan(1_000_001, "spec-b", vec![]);

        let context = make_context(vec![spec_a, spec_b], vec![plan_a, plan_b]);
        let validator = StateTransitionValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid()); // Warnings only
        assert_eq!(report.warning_count(), 1); // Only spec-b's empty plan
    }

    #[test]
    fn test_validator_name() {
        let validator = StateTransitionValidator;
        assert_eq!(
            Validator::<ValidationContext<Spec, Plan>>::name(&validator),
            "state-transition"
        );
    }
}
