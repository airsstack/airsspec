//! Validation report for collecting issues.

use super::issue::ValidationIssue;
use super::severity::ValidationSeverity;

/// Report containing all validation issues found.
///
/// Supports the permissive validation pattern (ADR-005):
/// - Collects all issues (doesn't fail fast)
/// - Distinguishes errors from warnings
/// - Supports merging multiple reports
///
/// # Examples
///
/// ```
/// use airsspec_core::validation::{ValidationReport, ValidationIssue};
///
/// let mut report = ValidationReport::new();
/// report.add_issue(ValidationIssue::warning("Consider adding docs"));
/// report.add_issue(ValidationIssue::error("Missing required field"));
///
/// assert!(!report.is_valid()); // Has errors
/// assert_eq!(report.error_count(), 1);
/// assert_eq!(report.warning_count(), 1);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    /// Creates a new empty validation report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a report from a collection of issues.
    #[must_use]
    pub fn from_issues(issues: impl IntoIterator<Item = ValidationIssue>) -> Self {
        Self {
            issues: issues.into_iter().collect(),
        }
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
            .any(|i| i.severity() == ValidationSeverity::Error)
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

    /// Consumes the report and returns all issues.
    #[must_use]
    pub fn into_issues(self) -> Vec<ValidationIssue> {
        self.issues
    }

    /// Returns only error-level issues.
    #[must_use]
    pub fn errors(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity() == ValidationSeverity::Error)
            .collect()
    }

    /// Returns only warning-level issues.
    #[must_use]
    pub fn warnings(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity() == ValidationSeverity::Warning)
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
            .filter(|i| i.severity() == ValidationSeverity::Error)
            .count()
    }

    /// Returns the number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity() == ValidationSeverity::Warning)
            .count()
    }

    /// Merges another report's issues into this one.
    ///
    /// This is the core operation for composable validation (ADR-005).
    pub fn merge(&mut self, other: ValidationReport) {
        self.issues.extend(other.issues);
    }

    /// Merges multiple reports into this one.
    pub fn merge_all(&mut self, others: impl IntoIterator<Item = ValidationReport>) {
        for other in others {
            self.merge(other);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_report() {
        let report = ValidationReport::new();
        assert!(report.is_valid());
        assert!(report.is_empty());
        assert_eq!(report.issue_count(), 0);
    }

    #[test]
    fn test_with_errors() {
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
    fn test_warnings_only_is_valid() {
        let mut report = ValidationReport::new();
        report.add_warning("Just a warning");

        assert!(report.is_valid()); // Warnings don't make it invalid
        assert!(!report.is_empty());
    }

    #[test]
    fn test_merge() {
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
    fn test_merge_all() {
        let mut main_report = ValidationReport::new();

        let mut r1 = ValidationReport::new();
        r1.add_error("E1");

        let mut r2 = ValidationReport::new();
        r2.add_warning("W1");

        let mut r3 = ValidationReport::new();
        r3.add_info("I1");

        main_report.merge_all([r1, r2, r3]);
        assert_eq!(main_report.issue_count(), 3);
    }

    #[test]
    fn test_from_issues() {
        let issues = vec![
            ValidationIssue::error("E1"),
            ValidationIssue::warning("W1"),
        ];
        let report = ValidationReport::from_issues(issues);
        assert_eq!(report.issue_count(), 2);
    }

    #[test]
    fn test_into_issues() {
        let mut report = ValidationReport::new();
        report.add_error("E1");
        report.add_warning("W1");

        let issues = report.into_issues();
        assert_eq!(issues.len(), 2);
    }

    #[test]
    fn test_errors_and_warnings_accessors() {
        let mut report = ValidationReport::new();
        report.add_error("Error 1");
        report.add_error("Error 2");
        report.add_warning("Warning 1");
        report.add_info("Info 1");

        assert_eq!(report.errors().len(), 2);
        assert_eq!(report.warnings().len(), 1);
    }

    #[test]
    fn test_add_issue() {
        let mut report = ValidationReport::new();
        report.add_issue(ValidationIssue::error("Direct issue").with_field("field"));
        assert_eq!(report.issue_count(), 1);
        assert_eq!(report.issues()[0].field(), Some("field"));
    }

    #[test]
    fn test_default() {
        let report = ValidationReport::default();
        assert!(report.is_empty());
    }

    #[test]
    fn test_clone() {
        let mut original = ValidationReport::new();
        original.add_error("Error 1");
        original.add_warning("Warning 1");

        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(cloned.error_count(), 1);
        assert_eq!(cloned.warning_count(), 1);
    }

    #[test]
    fn test_equality() {
        let mut report1 = ValidationReport::new();
        report1.add_error("Error");

        let mut report2 = ValidationReport::new();
        report2.add_error("Error");

        let mut report3 = ValidationReport::new();
        report3.add_warning("Warning");

        assert_eq!(report1, report2);
        assert_ne!(report1, report3);
    }
}
