//! Plan domain errors.
//!
//! Per ADR-002 (modular monolith), each domain has its own error type.
//! Per ADR-005, errors use thiserror for derivation.

use thiserror::Error;

/// Errors specific to the Plan domain.
///
/// # Examples
///
/// ```
/// use airsspec_core::plan::PlanError;
///
/// let err = PlanError::NotFound("1737734400-test".to_string());
/// assert!(err.to_string().contains("plan not found"));
/// ```
#[non_exhaustive]
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum PlanError {
    /// Plan for the given spec ID was not found.
    #[error("plan not found for spec: {0}")]
    NotFound(String),

    /// The plan content or structure is invalid.
    #[error("invalid plan format: {0}")]
    InvalidFormat(String),

    /// A required field is missing.
    #[error("missing required field: {0}")]
    MissingField(String),

    /// Step not found at the given index.
    #[error("step not found at index {0}")]
    StepNotFound(usize),

    /// Step index out of bounds.
    #[error("step index {index} out of bounds (plan has {total} steps)")]
    StepIndexOutOfBounds {
        /// The requested index.
        index: usize,
        /// The total number of steps.
        total: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_error() {
        let err = PlanError::NotFound("1737734400-missing".to_string());
        let msg = err.to_string();
        assert!(msg.contains("plan not found"));
        assert!(msg.contains("1737734400-missing"));
    }

    #[test]
    fn test_invalid_format_error() {
        let err = PlanError::InvalidFormat("missing steps".to_string());
        let msg = err.to_string();
        assert!(msg.contains("invalid plan format"));
    }

    #[test]
    fn test_missing_field_error() {
        let err = PlanError::MissingField("approach".to_string());
        let msg = err.to_string();
        assert!(msg.contains("missing required field"));
        assert!(msg.contains("approach"));
    }

    #[test]
    fn test_step_not_found_error() {
        let err = PlanError::StepNotFound(5);
        let msg = err.to_string();
        assert!(msg.contains("step not found"));
        assert!(msg.contains('5'));
    }

    #[test]
    fn test_step_index_out_of_bounds_error() {
        let err = PlanError::StepIndexOutOfBounds { index: 10, total: 5 };
        let msg = err.to_string();
        assert!(msg.contains("step index 10"));
        assert!(msg.contains("5 steps"));
    }

    #[test]
    fn test_error_clone() {
        let err = PlanError::InvalidFormat("test".to_string());
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn test_error_debug() {
        let err = PlanError::InvalidFormat("test".to_string());
        let debug = format!("{err:?}");
        assert!(debug.contains("InvalidFormat"));
    }

    #[test]
    fn test_error_is_std_error() {
        fn assert_std_error<T: std::error::Error>() {}
        assert_std_error::<PlanError>();
    }
}
