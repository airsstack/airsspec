//! Validation severity levels.

use std::fmt;

/// Severity level for validation issues.
///
/// Indicates how serious a validation issue is.
/// Ordered from least to most severe (Info < Warning < Error).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ValidationSeverity {
    /// Informational message, not a problem.
    Info,
    /// Warning that should be addressed but doesn't block.
    Warning,
    /// Error that must be fixed.
    Error,
}

impl fmt::Display for ValidationSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", ValidationSeverity::Info), "info");
        assert_eq!(format!("{}", ValidationSeverity::Warning), "warning");
        assert_eq!(format!("{}", ValidationSeverity::Error), "error");
    }

    #[test]
    fn test_equality() {
        assert_eq!(ValidationSeverity::Error, ValidationSeverity::Error);
        assert_ne!(ValidationSeverity::Error, ValidationSeverity::Warning);
    }

    #[test]
    fn test_ordering() {
        // Info < Warning < Error (ordered by severity)
        assert!(ValidationSeverity::Info < ValidationSeverity::Warning);
        assert!(ValidationSeverity::Warning < ValidationSeverity::Error);
        assert!(ValidationSeverity::Info < ValidationSeverity::Error);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ValidationSeverity::Info);
        set.insert(ValidationSeverity::Warning);
        set.insert(ValidationSeverity::Error);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_clone_and_copy() {
        let severity = ValidationSeverity::Warning;
        let copied = severity; // Copy trait
        let copied2 = severity; // Still works because of Copy
        assert_eq!(severity, copied);
        assert_eq!(severity, copied2);
    }
}
