//! Spec-specific validation logic.
//!
//! This module provides validation for specifications. The `ValidationReport`
//! struct is a placeholder that will be replaced by the full validation framework
//! in Task 2.5.

use super::types::Spec;

/// Severity level for validation issues.
///
/// Indicates how serious a validation issue is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationSeverity {
    /// Informational message, not a problem.
    Info,
    /// Warning that should be addressed but doesn't block.
    Warning,
    /// Error that must be fixed.
    Error,
}

impl std::fmt::Display for ValidationSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        };
        write!(f, "{s}")
    }
}

/// A single validation issue found during spec validation.
///
/// Contains information about what was wrong and where.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    /// The severity of this issue.
    pub severity: ValidationSeverity,

    /// A human-readable message describing the issue.
    pub message: String,

    /// Optional field name or path where the issue was found.
    pub field: Option<String>,
}

impl ValidationIssue {
    /// Creates a new validation issue.
    #[must_use]
    pub fn new(severity: ValidationSeverity, message: impl Into<String>) -> Self {
        Self {
            severity,
            message: message.into(),
            field: None,
        }
    }

    /// Creates a new info-level issue.
    #[must_use]
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(ValidationSeverity::Info, message)
    }

    /// Creates a new warning-level issue.
    #[must_use]
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(ValidationSeverity::Warning, message)
    }

    /// Creates a new error-level issue.
    #[must_use]
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(ValidationSeverity::Error, message)
    }

    /// Sets the field path for this issue.
    #[must_use]
    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
}

impl std::fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(field) = &self.field {
            write!(f, "[{}] {}: {}", self.severity, field, self.message)
        } else {
            write!(f, "[{}] {}", self.severity, self.message)
        }
    }
}

/// Report containing all validation issues found.
///
/// **Note:** This is a placeholder implementation that will be replaced
/// by the full validation framework in Task 2.5.
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::{validate_spec, SpecBuilder};
///
/// let spec = SpecBuilder::new()
///     .title("Valid Spec")
///     .description("A valid specification")
///     .content("# Spec\n\nContent here.")
///     .build()
///     .unwrap();
///
/// let report = validate_spec(&spec);
/// assert!(report.is_valid());
/// ```
#[derive(Debug, Default)]
pub struct ValidationReport {
    /// All issues found during validation.
    issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    /// Creates a new empty validation report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an issue to the report.
    pub fn add_issue(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    /// Adds an error to the report.
    pub fn add_error(&mut self, message: impl Into<String>) {
        self.issues.push(ValidationIssue::error(message));
    }

    /// Adds a warning to the report.
    pub fn add_warning(&mut self, message: impl Into<String>) {
        self.issues.push(ValidationIssue::warning(message));
    }

    /// Adds an info message to the report.
    pub fn add_info(&mut self, message: impl Into<String>) {
        self.issues.push(ValidationIssue::info(message));
    }

    /// Returns `true` if there are no errors.
    ///
    /// Warnings and info messages do not affect validity.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|i| i.severity == ValidationSeverity::Error)
    }

    /// Returns `true` if there are no issues at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    /// Returns all issues in the report.
    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    /// Returns only error-level issues.
    #[must_use]
    pub fn errors(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Error)
            .collect()
    }

    /// Returns only warning-level issues.
    #[must_use]
    pub fn warnings(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Warning)
            .collect()
    }

    /// Returns the total number of issues.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Returns the number of errors.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Error)
            .count()
    }

    /// Returns the number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Warning)
            .count()
    }

    /// Merges another report's issues into this one.
    pub fn merge(&mut self, other: ValidationReport) {
        self.issues.extend(other.issues);
    }
}

/// Validates a specification and returns a report of any issues.
///
/// Currently validates:
/// - Title is not empty
/// - Title length is reasonable (< 200 chars)
/// - Description is recommended (warning if empty)
/// - Content is recommended (warning if empty)
///
/// **Note:** This is a basic implementation. The full validation framework
/// (Task 2.5) will provide more comprehensive validation rules.
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
    let mut seen_ids = std::collections::HashSet::new();
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
        assert_eq!(info.severity, ValidationSeverity::Info);

        let warning = ValidationIssue::warning("Warning message");
        assert_eq!(warning.severity, ValidationSeverity::Warning);

        let error = ValidationIssue::error("Error message");
        assert_eq!(error.severity, ValidationSeverity::Error);
    }

    #[test]
    fn test_validation_issue_with_field() {
        let issue = ValidationIssue::error("Test").with_field("test.field");
        assert_eq!(issue.field, Some("test.field".to_string()));
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
                .message
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
        assert!(report.warnings()[0].message.contains("Content is empty"));
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
        assert!(report.warnings().iter().any(|w| w.message.contains("long")));
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
                .any(|w| w.message.contains("whitespace"))
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
        assert!(report.errors().iter().any(|e| e.message.contains("itself")));
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
                .any(|w| w.message.contains("Duplicate"))
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
