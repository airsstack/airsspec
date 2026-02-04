//! Generic validator trait.

use super::report::ValidationReport;

/// Trait for types that can validate a target.
///
/// This is a generic trait that allows for type-safe validation.
/// Domain-specific validators implement this for their domain types.
///
/// # Design Notes
///
/// - Uses generics (not `dyn`) for static dispatch per project guidelines
/// - Returns `ValidationReport` for permissive error collection (ADR-005)
/// - `Send + Sync` bounds for async compatibility
///
/// # Examples
///
/// ```
/// use airsspec_core::validation::{Validator, ValidationReport, ValidationIssue};
///
/// struct LengthValidator {
///     max_length: usize,
/// }
///
/// impl Validator<String> for LengthValidator {
///     fn name(&self) -> &str {
///         "length-validator"
///     }
///
///     fn validate(&self, target: &String) -> ValidationReport {
///         let mut report = ValidationReport::new();
///         if target.len() > self.max_length {
///             report.add_error(format!(
///                 "String exceeds max length of {}",
///                 self.max_length
///             ));
///         }
///         report
///     }
/// }
///
/// let validator = LengthValidator { max_length: 10 };
/// let report = validator.validate(&"hello".to_string());
/// assert!(report.is_valid());
///
/// let report = validator.validate(&"this is way too long".to_string());
/// assert!(!report.is_valid());
/// ```
pub trait Validator<T>: Send + Sync {
    /// Returns the name of this validator.
    ///
    /// Used for logging and error attribution.
    fn name(&self) -> &str;

    /// Validates the target and returns a report.
    ///
    /// This is permissive - it collects all issues rather than failing fast.
    fn validate(&self, target: &T) -> ValidationReport;
}

/// Extension trait for running multiple validators.
///
/// Provides utility methods for composing validators.
pub trait ValidatorExt<T> {
    /// Runs all validators and merges their reports.
    fn validate_all(&self, target: &T) -> ValidationReport;
}

impl<T, V> ValidatorExt<T> for [V]
where
    V: Validator<T>,
{
    fn validate_all(&self, target: &T) -> ValidationReport {
        let mut report = ValidationReport::new();
        for validator in self {
            report.merge(validator.validate(target));
        }
        report
    }
}

impl<T, V> ValidatorExt<T> for Vec<V>
where
    V: Validator<T>,
{
    fn validate_all(&self, target: &T) -> ValidationReport {
        self.as_slice().validate_all(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::ValidationIssue;

    struct AlwaysValid;

    impl Validator<String> for AlwaysValid {
        fn name(&self) -> &'static str {
            "always-valid"
        }

        fn validate(&self, _target: &String) -> ValidationReport {
            ValidationReport::new()
        }
    }

    struct AlwaysError {
        message: String,
    }

    impl Validator<String> for AlwaysError {
        fn name(&self) -> &'static str {
            "always-error"
        }

        fn validate(&self, _target: &String) -> ValidationReport {
            let mut report = ValidationReport::new();
            report.add_issue(ValidationIssue::error(&self.message));
            report
        }
    }

    #[test]
    fn test_validator_trait() {
        let validator = AlwaysValid;
        let report = validator.validate(&"test".to_string());
        assert!(report.is_valid());
    }

    #[test]
    fn test_validator_name() {
        let validator = AlwaysValid;
        assert_eq!(validator.name(), "always-valid");

        let error_validator = AlwaysError {
            message: "error".to_string(),
        };
        assert_eq!(error_validator.name(), "always-error");
    }

    #[test]
    fn test_validate_all_with_slice() {
        // Using homogeneous validators (same type) - no dyn needed
        let validators = [
            AlwaysError {
                message: "error1".to_string(),
            },
            AlwaysError {
                message: "error2".to_string(),
            },
        ];

        let report = validators.validate_all(&"test".to_string());
        assert!(!report.is_valid());
        assert_eq!(report.error_count(), 2);
    }

    #[test]
    fn test_validate_all_extension_vec() {
        let validators = vec![
            AlwaysError {
                message: "e1".to_string(),
            },
            AlwaysError {
                message: "e2".to_string(),
            },
        ];

        let report = validators.validate_all(&"test".to_string());
        assert!(!report.is_valid());
        assert_eq!(report.error_count(), 2);
    }

    #[test]
    fn test_multiple_validator_types_manual_composition() {
        // When you need different validator types, manually compose their results
        // This avoids dyn and demonstrates the intended pattern
        let valid = AlwaysValid;
        let error = AlwaysError {
            message: "error1".to_string(),
        };

        let mut report = ValidationReport::new();
        report.merge(valid.validate(&"test".to_string()));
        report.merge(error.validate(&"test".to_string()));

        assert!(!report.is_valid());
        assert_eq!(report.error_count(), 1);
    }

    #[test]
    fn test_empty_validators_slice() {
        let validators: [AlwaysError; 0] = [];
        let report = validators.validate_all(&"test".to_string());
        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_empty_validators_vec() {
        let validators: Vec<AlwaysError> = vec![];
        let report = validators.validate_all(&"test".to_string());
        assert!(report.is_valid());
        assert!(report.is_empty());
    }
}
