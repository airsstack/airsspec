//! Spec-specific validation logic.
//!
//! This module provides validation for specifications using the
//! validation framework from [`crate::validation`].

use std::collections::HashSet;

use super::types::Spec;

// Re-export validation types for backward compatibility
// Also makes them available for use within this module
pub use crate::validation::{ValidationIssue, ValidationReport, ValidationSeverity};

/// Validates a specification and returns a report of any issues.
///
/// Currently validates:
/// - Title is not empty
/// - Title length is reasonable (< 200 chars)
/// - Description is recommended (warning if empty)
/// - Content is recommended (warning if empty)
///
/// # Arguments
///
/// * `spec` - The specification to validate
///
/// # Returns
///
/// A `ValidationReport` containing any issues found.
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::{validate_spec, SpecBuilder};
///
/// let spec = SpecBuilder::new()
///     .title("My Feature")
///     .build()
///     .unwrap();
///
/// let report = validate_spec(&spec);
/// // Warnings for missing description and content, but still valid
/// assert!(report.is_valid());
/// ```
#[must_use]
pub fn validate_spec(spec: &Spec) -> ValidationReport {
    let mut report = ValidationReport::new();

    // Validate title
    validate_title(spec, &mut report);

    // Validate description
    validate_description(spec, &mut report);

    // Validate content
    validate_content(spec, &mut report);

    // Validate dependencies
    validate_dependencies(spec, &mut report);

    report
}

/// Maximum recommended title length.
const MAX_TITLE_LENGTH: usize = 200;

/// Validates the spec title.
fn validate_title(spec: &Spec, report: &mut ValidationReport) {
    let title = spec.title();

    if title.is_empty() {
        report.add_issue(
            ValidationIssue::error("Title cannot be empty").with_field("metadata.title"),
        );
        return;
    }

    if title.len() > MAX_TITLE_LENGTH {
        report.add_issue(
            ValidationIssue::warning(format!(
                "Title is very long ({} characters), consider shortening",
                title.len()
            ))
            .with_field("metadata.title"),
        );
    }

    // Check for title starting/ending with whitespace
    if title != title.trim() {
        report.add_issue(
            ValidationIssue::warning("Title has leading or trailing whitespace")
                .with_field("metadata.title"),
        );
    }
}

/// Validates the spec description.
fn validate_description(spec: &Spec, report: &mut ValidationReport) {
    let description = spec.description();

    if description.is_empty() {
        report.add_issue(
            ValidationIssue::warning("Description is empty, consider adding details")
                .with_field("metadata.description"),
        );
    }
}

/// Validates the spec content.
fn validate_content(spec: &Spec, report: &mut ValidationReport) {
    let content = spec.content();

    if content.is_empty() {
        report.add_issue(
            ValidationIssue::warning("Content is empty, consider adding documentation")
                .with_field("content"),
        );
    }
}

/// Validates spec dependencies.
fn validate_dependencies(spec: &Spec, report: &mut ValidationReport) {
    let dependencies = spec.dependencies();

    // Check for duplicate dependencies
    let mut seen_ids = HashSet::new();
    for (idx, dep) in dependencies.iter().enumerate() {
        let id_str = dep.spec_id.as_str();
        if !seen_ids.insert(id_str) {
            report.add_issue(
                ValidationIssue::warning(format!("Duplicate dependency: {}", dep.spec_id.as_str()))
                    .with_field(format!("metadata.dependencies[{idx}]")),
            );
        }
    }

    // Check for self-reference
    let self_id = spec.id().as_str();
    if dependencies.iter().any(|d| d.spec_id.as_str() == self_id) {
        report.add_issue(
            ValidationIssue::error("Spec cannot depend on itself")
                .with_field("metadata.dependencies"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::{Dependency, SpecBuilder, SpecId, SpecMetadata};

    #[test]
    fn test_validation_severity_display() {
        assert_eq!(format!("{}", ValidationSeverity::Info), "info");
        assert_eq!(format!("{}", ValidationSeverity::Warning), "warning");
        assert_eq!(format!("{}", ValidationSeverity::Error), "error");
    }

    #[test]
    fn test_validation_issue_constructors() {
        let info = ValidationIssue::info("Info message");
        assert_eq!(info.severity(), ValidationSeverity::Info);

        let warning = ValidationIssue::warning("Warning message");
        assert_eq!(warning.severity(), ValidationSeverity::Warning);

        let error = ValidationIssue::error("Error message");
        assert_eq!(error.severity(), ValidationSeverity::Error);
    }

    #[test]
    fn test_validation_issue_with_field() {
        let issue = ValidationIssue::error("Test").with_field("test.field");
        assert_eq!(issue.field(), Some("test.field"));
    }

    #[test]
    fn test_validation_issue_display() {
        let issue = ValidationIssue::error("Something wrong").with_field("field.name");
        let display = format!("{issue}");
        assert!(display.contains("error"));
        assert!(display.contains("field.name"));
        assert!(display.contains("Something wrong"));
    }

    #[test]
    fn test_validation_report_empty() {
        let report = ValidationReport::new();
        assert!(report.is_valid());
        assert!(report.is_empty());
        assert_eq!(report.issue_count(), 0);
    }

    #[test]
    fn test_validation_report_with_errors() {
        let mut report = ValidationReport::new();
        report.add_error("An error");
        report.add_warning("A warning");

        assert!(!report.is_valid());
        assert!(!report.is_empty());
        assert_eq!(report.error_count(), 1);
        assert_eq!(report.warning_count(), 1);
        assert_eq!(report.issue_count(), 2);
    }

    #[test]
    fn test_validation_report_warnings_only() {
        let mut report = ValidationReport::new();
        report.add_warning("Just a warning");

        assert!(report.is_valid()); // Warnings don't make it invalid
        assert!(!report.is_empty());
    }

    #[test]
    fn test_validation_report_merge() {
        let mut report1 = ValidationReport::new();
        report1.add_error("Error 1");

        let mut report2 = ValidationReport::new();
        report2.add_warning("Warning 1");
        report2.add_warning("Warning 2");

        report1.merge(report2);
        assert_eq!(report1.issue_count(), 3);
        assert_eq!(report1.error_count(), 1);
        assert_eq!(report1.warning_count(), 2);
    }

    #[test]
    fn test_validate_spec_valid() {
        let spec = SpecBuilder::new()
            .title("Valid Spec")
            .description("A valid description")
            .content("# Content\n\nSome content here.")
            .build()
            .unwrap();

        let report = validate_spec(&spec);
        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_validate_spec_missing_description() {
        let spec = SpecBuilder::new()
            .title("Missing Desc")
            .content("Content")
            .build()
            .unwrap();

        let report = validate_spec(&spec);
        assert!(report.is_valid()); // Warning only
        assert_eq!(report.warning_count(), 1);
        assert!(
            report.warnings()[0]
                .message()
                .contains("Description is empty")
        );
    }

    #[test]
    fn test_validate_spec_missing_content() {
        let spec = SpecBuilder::new()
            .title("Missing Content")
            .description("Has description")
            .build()
            .unwrap();

        let report = validate_spec(&spec);
        assert!(report.is_valid()); // Warning only
        assert_eq!(report.warning_count(), 1);
        assert!(report.warnings()[0].message().contains("Content is empty"));
    }

    #[test]
    fn test_validate_spec_long_title() {
        let long_title = "A".repeat(MAX_TITLE_LENGTH + 50);
        let spec = SpecBuilder::new()
            .title(long_title)
            .description("Description")
            .content("Content")
            .build()
            .unwrap();

        let report = validate_spec(&spec);
        assert!(report.is_valid()); // Warning only
        assert!(report.warnings().iter().any(|w| w.message().contains("long")));
    }

    #[test]
    fn test_validate_spec_title_whitespace() {
        // Create spec with whitespace in title via direct construction
        let id = SpecId::new(1_737_734_400, "whitespace-test");
        let metadata = SpecMetadata::new("  Leading Space", "Description");
        let spec = Spec::new(id, metadata, "Content");

        let report = validate_spec(&spec);
        assert!(report.is_valid()); // Warning only
        assert!(
            report
                .warnings()
                .iter()
                .any(|w| w.message().contains("whitespace"))
        );
    }

    #[test]
    fn test_validate_spec_self_reference() {
        // Create a spec that depends on itself
        let id = SpecId::new(1_737_734_400, "self-ref");
        let mut metadata = SpecMetadata::new("Self Reference", "Description");
        metadata.add_dependency(Dependency::blocked_by(id.clone()));
        let spec = Spec::new(id, metadata, "Content");

        let report = validate_spec(&spec);
        assert!(!report.is_valid()); // Error
        assert!(report.errors().iter().any(|e| e.message().contains("itself")));
    }

    #[test]
    fn test_validate_spec_duplicate_dependencies() {
        let dep_id = SpecId::new(1_737_734_400, "dep-spec");
        let id = SpecId::new(1_737_734_401, "main-spec");
        let mut metadata = SpecMetadata::new("Duplicate Deps", "Description");
        metadata.add_dependency(Dependency::blocked_by(dep_id.clone()));
        metadata.add_dependency(Dependency::related_to(dep_id)); // Same ID, different kind
        let spec = Spec::new(id, metadata, "Content");

        let report = validate_spec(&spec);
        assert!(report.is_valid()); // Warning only
        assert!(
            report
                .warnings()
                .iter()
                .any(|w| w.message().contains("Duplicate"))
        );
    }

    #[test]
    fn test_validation_report_errors_and_warnings_accessors() {
        let mut report = ValidationReport::new();
        report.add_error("Error 1");
        report.add_error("Error 2");
        report.add_warning("Warning 1");
        report.add_info("Info 1");

        assert_eq!(report.errors().len(), 2);
        assert_eq!(report.warnings().len(), 1);
    }
}
