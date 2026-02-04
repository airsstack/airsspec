//! Validation issue types.

use std::fmt;

use super::severity::ValidationSeverity;

/// A single validation issue found during validation.
///
/// Contains information about what was wrong and where.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidationIssue {
    severity: ValidationSeverity,
    message: String,
    field: Option<String>,
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

    /// Returns the severity of this issue.
    #[must_use]
    pub fn severity(&self) -> ValidationSeverity {
        self.severity
    }

    /// Returns the message describing this issue.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the field path where this issue was found.
    #[must_use]
    pub fn field(&self) -> Option<&str> {
        self.field.as_deref()
    }
}

impl fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(field) = &self.field {
            write!(f, "[{}] {}: {}", self.severity, field, self.message)
        } else {
            write!(f, "[{}] {}", self.severity, self.message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        let info = ValidationIssue::info("Info message");
        assert_eq!(info.severity(), ValidationSeverity::Info);

        let warning = ValidationIssue::warning("Warning message");
        assert_eq!(warning.severity(), ValidationSeverity::Warning);

        let error = ValidationIssue::error("Error message");
        assert_eq!(error.severity(), ValidationSeverity::Error);
    }

    #[test]
    fn test_with_field() {
        let issue = ValidationIssue::error("Test").with_field("test.field");
        assert_eq!(issue.field(), Some("test.field"));
    }

    #[test]
    fn test_display_with_field() {
        let issue = ValidationIssue::error("Something wrong").with_field("field.name");
        let display = format!("{issue}");
        assert!(display.contains("error"));
        assert!(display.contains("field.name"));
        assert!(display.contains("Something wrong"));
    }

    #[test]
    fn test_display_without_field() {
        let issue = ValidationIssue::warning("Just a warning");
        let display = format!("{issue}");
        assert!(display.contains("warning"));
        assert!(display.contains("Just a warning"));
    }

    #[test]
    fn test_getters() {
        let issue = ValidationIssue::error("Test message").with_field("some.field");
        assert_eq!(issue.severity(), ValidationSeverity::Error);
        assert_eq!(issue.message(), "Test message");
        assert_eq!(issue.field(), Some("some.field"));
    }

    #[test]
    fn test_no_field() {
        let issue = ValidationIssue::info("No field");
        assert_eq!(issue.field(), None);
    }

    #[test]
    fn test_clone() {
        let original = ValidationIssue::error("Test").with_field("field");
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_equality() {
        let issue1 = ValidationIssue::error("Test").with_field("field");
        let issue2 = ValidationIssue::error("Test").with_field("field");
        let issue3 = ValidationIssue::error("Different");

        assert_eq!(issue1, issue2);
        assert_ne!(issue1, issue3);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ValidationIssue::error("Error 1"));
        set.insert(ValidationIssue::warning("Warning 1"));
        set.insert(ValidationIssue::error("Error 1")); // Duplicate
        assert_eq!(set.len(), 2);
    }
}
