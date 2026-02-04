//! Plan-specific validation logic.
//!
//! This module provides validation for implementation plans
//! using the validation framework from [`crate::validation`].

use std::collections::HashSet;

use super::step::StepStatus;
use super::types::Plan;

// Re-export validation types from validation module for convenience
pub use crate::validation::{ValidationIssue, ValidationReport, ValidationSeverity};

/// Validates a plan and returns a report of any issues.
///
/// Currently validates:
/// - Plan has at least one step
/// - Step indices are sequential (0, 1, 2, ...)
/// - No duplicate step indices
/// - Approach is recommended (warning if empty)
/// - Steps have titles (error if empty)
/// - Blocked steps should have notes (warning)
///
/// # Arguments
///
/// * `plan` - The plan to validate
///
/// # Returns
///
/// A `ValidationReport` containing any issues found.
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::SpecId;
/// use airsspec_core::plan::{Plan, PlanStep, validate_plan};
///
/// let spec_id = SpecId::new(1_737_734_400, "test");
/// let plan = Plan::new(spec_id, "Strategy", vec![
///     PlanStep::new(0, "Step 1", "Description"),
/// ]);
///
/// let report = validate_plan(&plan);
/// assert!(report.is_valid());
/// ```
#[must_use]
pub fn validate_plan(plan: &Plan) -> ValidationReport {
    let mut report = ValidationReport::new();

    // Validate has steps
    validate_has_steps(plan, &mut report);

    // Validate approach
    validate_approach(plan, &mut report);

    // Validate step indices
    validate_step_indices(plan, &mut report);

    // Validate step content
    validate_step_content(plan, &mut report);

    // Validate blocked steps
    validate_blocked_steps(plan, &mut report);

    report
}

/// Validates that the plan has at least one step.
fn validate_has_steps(plan: &Plan, report: &mut ValidationReport) {
    if plan.steps().is_empty() {
        report.add_issue(
            ValidationIssue::error("Plan must have at least one step").with_field("steps"),
        );
    }
}

/// Validates the approach field.
fn validate_approach(plan: &Plan, report: &mut ValidationReport) {
    if plan.approach().is_empty() {
        report.add_issue(
            ValidationIssue::warning("Approach is empty, consider describing your strategy")
                .with_field("approach"),
        );
    }
}

/// Validates step indices are sequential and unique.
fn validate_step_indices(plan: &Plan, report: &mut ValidationReport) {
    let steps = plan.steps();
    if steps.is_empty() {
        return;
    }

    // Check for sequential indices starting from 0
    for (expected_idx, step) in steps.iter().enumerate() {
        if step.index() != expected_idx {
            report.add_issue(
                ValidationIssue::warning(format!(
                    "Step has index {} but expected {}",
                    step.index(),
                    expected_idx
                ))
                .with_field(format!("steps[{expected_idx}].index")),
            );
        }
    }

    // Check for duplicate indices
    let mut seen_indices = HashSet::new();
    for (pos, step) in steps.iter().enumerate() {
        if !seen_indices.insert(step.index()) {
            report.add_issue(
                ValidationIssue::warning(format!("Duplicate step index: {}", step.index()))
                    .with_field(format!("steps[{pos}].index")),
            );
        }
    }
}

/// Validates step content.
fn validate_step_content(plan: &Plan, report: &mut ValidationReport) {
    for (idx, step) in plan.steps().iter().enumerate() {
        // Title must not be empty
        if step.title().is_empty() {
            report.add_issue(
                ValidationIssue::error("Step title cannot be empty")
                    .with_field(format!("steps[{idx}].title")),
            );
        }

        // Long titles are discouraged
        if step.title().len() > 100 {
            report.add_issue(
                ValidationIssue::warning(format!(
                    "Step title is very long ({} characters)",
                    step.title().len()
                ))
                .with_field(format!("steps[{idx}].title")),
            );
        }

        // Description recommended
        if step.description().is_empty() {
            report.add_issue(
                ValidationIssue::info("Step has no description")
                    .with_field(format!("steps[{idx}].description")),
            );
        }
    }
}

/// Validates blocked steps have notes explaining why.
fn validate_blocked_steps(plan: &Plan, report: &mut ValidationReport) {
    for (idx, step) in plan.steps().iter().enumerate() {
        if step.status() == StepStatus::Blocked && step.notes().is_none() {
            report.add_issue(
                ValidationIssue::warning("Blocked step has no notes explaining the blocker")
                    .with_field(format!("steps[{idx}]")),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::step::PlanStep;
    use crate::spec::SpecId;

    fn test_spec_id() -> SpecId {
        SpecId::new(1_737_734_400, "test-spec")
    }

    #[test]
    fn test_validate_valid_plan() {
        let plan = Plan::new(
            test_spec_id(),
            "Test approach",
            vec![
                PlanStep::new(0, "Step 1", "First step description"),
                PlanStep::new(1, "Step 2", "Second step description"),
            ],
        );

        let report = validate_plan(&plan);
        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_validate_empty_steps() {
        let plan = Plan::new(test_spec_id(), "Approach", vec![]);

        let report = validate_plan(&plan);
        assert!(!report.is_valid());
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.message().contains("at least one step"))
        );
    }

    #[test]
    fn test_validate_empty_approach() {
        let plan = Plan::new(
            test_spec_id(),
            "",
            vec![PlanStep::new(0, "Step", "Description")],
        );

        let report = validate_plan(&plan);
        assert!(report.is_valid()); // Warning only
        assert!(
            report
                .warnings()
                .iter()
                .any(|w| w.message().contains("Approach"))
        );
    }

    #[test]
    fn test_validate_step_indices_non_sequential() {
        // Create steps with non-sequential indices
        let plan = Plan::new(
            test_spec_id(),
            "Approach",
            vec![
                PlanStep::new(0, "Step 0", ""),
                PlanStep::new(5, "Step 5", ""), // Should be 1
            ],
        );

        let report = validate_plan(&plan);
        assert!(report.is_valid()); // Warning only
        assert!(
            report
                .warnings()
                .iter()
                .any(|w| w.message().contains("expected"))
        );
    }

    #[test]
    fn test_validate_empty_step_title() {
        // For testing, we deserialize a plan with empty title
        let json = r#"{
            "spec_id": "1737734400-test",
            "approach": "Test",
            "steps": [{"index": 0, "title": "", "description": "", "complexity": "medium", "status": "pending", "notes": null}],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-01T00:00:00Z"
        }"#;
        let plan: Plan = serde_json::from_str(json).unwrap();

        let report = validate_plan(&plan);
        assert!(!report.is_valid());
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.message().contains("title cannot be empty"))
        );
    }

    #[test]
    fn test_validate_long_step_title() {
        let long_title = "A".repeat(150);
        let plan = Plan::new(
            test_spec_id(),
            "Approach",
            vec![PlanStep::new(0, long_title, "Description")],
        );

        let report = validate_plan(&plan);
        assert!(report.is_valid()); // Warning only
        assert!(report.warnings().iter().any(|w| w.message().contains("long")));
    }

    #[test]
    fn test_validate_missing_step_description() {
        let plan = Plan::new(
            test_spec_id(),
            "Approach",
            vec![PlanStep::new(0, "Step", "")],
        );

        let report = validate_plan(&plan);
        assert!(report.is_valid()); // Info only
        assert!(report.issues().iter().any(|i| {
            i.severity() == ValidationSeverity::Info && i.message().contains("no description")
        }));
    }

    #[test]
    fn test_validate_blocked_step_without_notes() {
        let mut plan = Plan::new(
            test_spec_id(),
            "Approach",
            vec![PlanStep::new(0, "Step", "Description")],
        );
        plan.step_mut(0).unwrap().set_status(StepStatus::Blocked);

        let report = validate_plan(&plan);
        assert!(report.is_valid()); // Warning only
        assert!(
            report
                .warnings()
                .iter()
                .any(|w| w.message().contains("Blocked step"))
        );
    }

    #[test]
    fn test_validate_blocked_step_with_notes() {
        let mut plan = Plan::new(
            test_spec_id(),
            "Approach",
            vec![PlanStep::new(0, "Step", "Description")],
        );
        plan.step_mut(0).unwrap().set_status(StepStatus::Blocked);
        plan.step_mut(0)
            .unwrap()
            .set_notes("Waiting for API approval");

        let report = validate_plan(&plan);
        assert!(report.is_valid());
        // Should not have the blocked warning
        assert!(
            !report
                .warnings()
                .iter()
                .any(|w| w.message().contains("Blocked step"))
        );
    }
}
