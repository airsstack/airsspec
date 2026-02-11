//! Spec domain errors.
//!
//! Per ADR-002 (modular monolith), each domain has its own error type.
//! Per ADR-005, errors use thiserror for derivation.

use thiserror::Error;

/// Errors specific to the Spec domain.
///
/// # Examples
///
/// ```
/// use airsspec_core::spec::SpecError;
///
/// let err = SpecError::InvalidId("bad format".to_string());
/// assert!(err.to_string().contains("invalid spec ID"));
/// ```
#[non_exhaustive]
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SpecError {
    /// Spec with the given ID was not found.
    #[error("spec not found: {0}")]
    NotFound(String),

    /// The spec ID format is invalid.
    #[error("invalid spec ID: {0}")]
    InvalidId(String),

    /// The spec content or structure is invalid.
    #[error("invalid spec format: {0}")]
    InvalidFormat(String),

    /// A required field is missing.
    #[error("missing required field: {0}")]
    MissingField(String),

    /// A duplicate spec already exists.
    #[error("spec already exists: {0}")]
    AlreadyExists(String),

    /// I/O error (stored as string since `io::Error` doesn't impl Clone/Eq).
    #[error("I/O error: {0}")]
    Io(String),
}

impl From<std::io::Error> for SpecError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_error() {
        let err = SpecError::NotFound("1737734400-missing".to_string());

        let msg = err.to_string();
        assert!(msg.contains("spec not found"));
        assert!(msg.contains("1737734400-missing"));
    }

    #[test]
    fn test_invalid_id_error() {
        let err = SpecError::InvalidId("bad format".to_string());
        let msg = err.to_string();
        assert!(msg.contains("invalid spec ID"));
        assert!(msg.contains("bad format"));
    }

    #[test]
    fn test_invalid_format_error() {
        let err = SpecError::InvalidFormat("missing title".to_string());
        let msg = err.to_string();
        assert!(msg.contains("invalid spec format"));
    }

    #[test]
    fn test_missing_field_error() {
        let err = SpecError::MissingField("description".to_string());
        let msg = err.to_string();
        assert!(msg.contains("missing required field"));
        assert!(msg.contains("description"));
    }

    #[test]
    fn test_already_exists_error() {
        let err = SpecError::AlreadyExists("1737734400-existing".to_string());

        let msg = err.to_string();
        assert!(msg.contains("spec already exists"));
    }

    #[test]
    fn test_error_clone() {
        let err = SpecError::InvalidId("test".to_string());
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn test_error_debug() {
        let err = SpecError::InvalidId("test".to_string());
        let debug = format!("{err:?}");
        assert!(debug.contains("InvalidId"));
    }

    #[test]
    fn test_io_error() {
        let err = SpecError::Io("permission denied".to_string());
        let msg = err.to_string();
        assert!(msg.contains("I/O error"));
        assert!(msg.contains("permission denied"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
        let spec_err: SpecError = io_err.into();
        assert!(matches!(spec_err, SpecError::Io(_)));
        assert!(spec_err.to_string().contains("permission denied"));
    }

    #[test]
    fn test_error_is_std_error() {
        fn assert_std_error<T: std::error::Error>() {}
        assert_std_error::<SpecError>();
    }
}
