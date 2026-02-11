//! Spec content validator.
//!
//! Validates all specifications in the workspace by calling
//! [`ValidatableSpec::validate_content()`] on each spec and attributing
//! issues to their source spec.
//!
//! This validator uses the [`ValidatableSpec`] trait abstraction rather
//! than importing concrete domain types, following the Dependency
//! Inversion Principle.

use crate::validation::context::ValidationContext;
use crate::validation::issue::ValidationIssue;
use crate::validation::report::ValidationReport;
use crate::validation::traits::ValidatableSpec;
use crate::validation::validator::Validator;

/// Validates the content of all specifications in the workspace.
///
/// Iterates over all specs in the [`ValidationContext`] and calls
/// [`ValidatableSpec::validate_content()`] on each one. Issues are
/// attributed to the source spec by prefixing the field with the spec ID.
///
/// An empty specs list is valid (empty workspace).
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use airsspec_core::validation::{
///     Validator, ValidationContextBuilder, SpecContentValidator,
/// };
/// use airsspec_core::spec::{Spec, SpecId, SpecMetadata};
///
/// let spec = Spec::new(
///     SpecId::new(1_737_734_400, "test"),
///     SpecMetadata::new("Test", "Description"),
///     "content",
/// );
///
/// let context = ValidationContextBuilder::new()
///     .workspace_path(PathBuf::from("/project"))
///     .specs(vec![spec])
///     .build();
///
/// let validator = SpecContentValidator;
/// let report = validator.validate(&context);
/// assert!(report.is_valid());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SpecContentValidator;

impl<S, P> Validator<ValidationContext<S, P>> for SpecContentValidator
where
    S: ValidatableSpec,
{
    fn name(&self) -> &'static str {
        "spec-content"
    }

    fn validate(&self, context: &ValidationContext<S, P>) -> ValidationReport {
        let mut report = ValidationReport::new();

        for spec in context.specs() {
            let spec_report = spec.validate_content();
            let spec_id = spec.id_str();

            // Re-attribute each issue with the spec ID prefix
            for issue in spec_report.into_issues() {
                let prefixed_field = match issue.field() {
                    Some(field) => format!("[{spec_id}] {field}"),
                    None => format!("[{spec_id}]"),
                };

                let attributed = ValidationIssue::new(issue.severity(), issue.message())
                    .with_field(prefixed_field);
                report.add_issue(attributed);
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::spec::{Dependency, Spec, SpecId, SpecMetadata};
    use crate::validation::ValidationContextBuilder;

    fn make_context(specs: Vec<Spec>) -> ValidationContext<Spec> {
        ValidationContextBuilder::new()
            .workspace_path(PathBuf::from("/project"))
            .specs(specs)
            .build()
    }

    #[test]
    fn test_empty_specs_is_valid() {
        let context = make_context(vec![]);
        let validator = SpecContentValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_valid_spec_passes() {
        let spec = Spec::new(
            SpecId::new(1_737_734_400, "valid-spec"),
            SpecMetadata::new("Valid Spec", "A valid description"),
            "# Content\n\nSome content here.",
        );

        let context = make_context(vec![spec]);
        let validator = SpecContentValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_invalid_spec_reports_errors_with_spec_id_prefix() {
        let id = SpecId::new(1_737_734_400, "self-ref");
        let mut metadata = SpecMetadata::new("Self Ref", "Description");
        metadata.add_dependency(Dependency::blocked_by(id.clone()));
        let spec = Spec::new(id, metadata, "Content");

        let context = make_context(vec![spec]);
        let validator = SpecContentValidator;
        let report = validator.validate(&context);

        assert!(!report.is_valid());
        let errors = report.errors();
        assert!(!errors.is_empty());
        assert!(errors[0].field().unwrap().contains("1737734400-self-ref"));
    }

    #[test]
    fn test_multiple_specs_mixed_validity() {
        let valid_spec = Spec::new(
            SpecId::new(1_737_734_400, "valid"),
            SpecMetadata::new("Valid", "Description"),
            "Content",
        );

        let bad_id = SpecId::new(1_737_734_401, "bad-spec");
        let mut bad_metadata = SpecMetadata::new("Bad", "Description");
        bad_metadata.add_dependency(Dependency::blocked_by(bad_id.clone()));
        let bad_spec = Spec::new(bad_id, bad_metadata, "Content");

        let context = make_context(vec![valid_spec, bad_spec]);
        let validator = SpecContentValidator;
        let report = validator.validate(&context);

        assert!(!report.is_valid());
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.field().unwrap().contains("1737734401-bad-spec"))
        );
    }

    #[test]
    fn test_warning_issues_preserved() {
        let spec = Spec::new(
            SpecId::new(1_737_734_400, "warn-spec"),
            SpecMetadata::new("Valid Title", ""),
            "",
        );

        let context = make_context(vec![spec]);
        let validator = SpecContentValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid()); // Warnings only
        assert!(report.warning_count() >= 1);
    }

    #[test]
    fn test_validator_name() {
        let validator = SpecContentValidator;
        assert_eq!(
            Validator::<ValidationContext<Spec>>::name(&validator),
            "spec-content"
        );
    }
}
