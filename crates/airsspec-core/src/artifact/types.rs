//! Core artifact type definitions.
//!
//! This module defines artifact management types.

// Layer 1: Standard library imports
use std::fmt;

// Layer 2: Third-party crate imports
use serde::{Deserialize, Serialize};

// Layer 3: Internal module imports (none - this is a types module)

/// Type of artifact in the `AirsSpec` framework.
///
/// Each artifact type corresponds to a specific document in the AI-DLC workflow.
/// Artifacts are validated according to their type's schema and rules.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::artifact::types::ArtifactType;
///
/// let artifact_type = ArtifactType::Requirements;
/// assert_eq!(format!("{:?}", artifact_type), "Requirements");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtifactType {
    /// Requirements document from the Research phase.
    ///
    /// Contains feature requirements, constraints, and success criteria.
    Requirements,

    /// Domain Architecture Analysis document.
    ///
    /// Defines the domain model, bounded contexts, and entity relationships.
    Daa,

    /// Architecture Decision Record.
    ///
    /// Records significant technical decisions with context and alternatives.
    Adr,

    /// Request for Comments document.
    ///
    /// Proposes implementation plans for review and approval.
    Rfc,

    /// Bolt plan document.
    ///
    /// Detailed plan for implementing a specific Bolt (work unit).
    BoltPlan,
}

impl fmt::Display for ArtifactType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requirements => write!(f, "Requirements"),
            Self::Daa => write!(f, "DAA"),
            Self::Adr => write!(f, "ADR"),
            Self::Rfc => write!(f, "RFC"),
            Self::BoltPlan => write!(f, "BoltPlan"),
        }
    }
}

/// Artifact validation result.
///
/// Contains validation outcome including errors and warnings.
///
/// # Fields
///
/// * `valid` - Whether the artifact passed validation (no errors)
/// * `errors` - List of validation errors that must be fixed
/// * `warnings` - List of warnings that should be addressed but don't block approval
///
/// # Examples
///
/// ```rust
/// use airsspec_core::artifact::types::{ValidationResult, ValidationError};
///
/// let result = ValidationResult {
///     valid: false,
///     errors: vec![
///         ValidationError {
///             field: "title".to_string(),
///             message: "Title is required".to_string(),
///         }
///     ],
///     warnings: vec![],
/// };
///
/// assert!(!result.valid);
/// assert_eq!(result.errors.len(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the artifact passed validation.
    pub valid: bool,

    /// List of validation errors.
    pub errors: Vec<ValidationError>,

    /// List of validation warnings.
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Creates a successful validation result.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::artifact::types::ValidationResult;
    ///
    /// let result = ValidationResult::success();
    /// assert!(result.valid);
    /// assert!(result.errors.is_empty());
    /// ```
    #[must_use]
    pub const fn success() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Creates a failed validation result with errors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::artifact::types::{ValidationResult, ValidationError};
    ///
    /// let result = ValidationResult::failure(vec![
    ///     ValidationError {
    ///         field: "title".to_string(),
    ///         message: "Title is required".to_string(),
    ///     }
    /// ]);
    /// assert!(!result.valid);
    /// ```
    #[must_use]
    pub const fn failure(errors: Vec<ValidationError>) -> Self {
        Self {
            valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Adds a warning to the validation result.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::artifact::types::ValidationResult;
    ///
    /// let mut result = ValidationResult::success();
    /// result.add_warning("Description is too short".to_string());
    /// assert_eq!(result.warnings.len(), 1);
    /// ```
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::success()
    }
}

/// Artifact validation error.
///
/// Represents a validation error found in an artifact.
///
/// # Fields
///
/// * `field` - The field or path where the error was found
/// * `message` - Human-readable error message describing the issue
///
/// # Examples
///
/// ```rust
/// use airsspec_core::artifact::types::ValidationError;
///
/// let error = ValidationError {
///     field: "metadata.version".to_string(),
///     message: "Version must be in semver format".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// The field or path where the error was found.
    pub field: String,

    /// Human-readable error message.
    pub message: String,
}

impl ValidationError {
    /// Creates a new validation error.
    ///
    /// # Arguments
    ///
    /// * `field` - The field or path where the error was found
    /// * `message` - Human-readable error message
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::artifact::types::ValidationError;
    ///
    /// let error = ValidationError::new("title", "Title is required");
    /// assert_eq!(error.field, "title");
    /// assert_eq!(error.message, "Title is required");
    /// ```
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// Reference to an artifact with metadata.
///
/// Represents an artifact reference including type and status.
///
/// # Fields
///
/// * `path` - Filesystem path to the artifact
/// * `artifact_type` - Type of the artifact
/// * `status` - Status of the artifact (e.g., "draft", "approved")
///
/// # Examples
///
/// ```rust
/// use airsspec_core::artifact::types::{ArtifactRef, ArtifactType};
///
/// let artifact_ref = ArtifactRef {
///     path: "requirements.md".to_string(),
///     artifact_type: ArtifactType::Requirements,
///     status: "draft".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRef {
    /// Filesystem path to the artifact.
    pub path: String,

    /// Type of the artifact.
    pub artifact_type: ArtifactType,

    /// Status of the artifact.
    pub status: String,
}

impl ArtifactRef {
    /// Creates a new artifact reference.
    ///
    /// # Arguments
    ///
    /// * `path` - Filesystem path to the artifact
    /// * `artifact_type` - Type of the artifact
    /// * `status` - Status of the artifact
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::artifact::types::{ArtifactRef, ArtifactType};
    ///
    /// let artifact_ref = ArtifactRef::new(
    ///     "requirements.md",
    ///     ArtifactType::Requirements,
    ///     "draft"
    /// );
    /// ```
    pub fn new(
        path: impl Into<String>,
        artifact_type: ArtifactType,
        status: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            artifact_type,
            status: status.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_type_display() {
        assert_eq!(ArtifactType::Requirements.to_string(), "Requirements");
        assert_eq!(ArtifactType::Daa.to_string(), "DAA");
        assert_eq!(ArtifactType::Adr.to_string(), "ADR");
        assert_eq!(ArtifactType::Rfc.to_string(), "RFC");
        assert_eq!(ArtifactType::BoltPlan.to_string(), "BoltPlan");
    }

    #[test]
    fn test_validation_result_success() {
        let result = ValidationResult::success();
        assert!(result.valid);
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_validation_result_failure() {
        let errors = vec![
            ValidationError::new("title", "Title is required"),
            ValidationError::new("status", "Status is required"),
        ];
        let result = ValidationResult::failure(errors);
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 2);
        assert_eq!(result.errors[0].field, "title");
        assert_eq!(result.errors[1].field, "status");
    }

    #[test]
    fn test_validation_result_add_warning() {
        let mut result = ValidationResult::success();
        result.add_warning("Description is too short".to_string());
        assert!(result.valid);
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.warnings[0], "Description is too short");
    }

    #[test]
    fn test_validation_result_default() {
        let result = ValidationResult::default();
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validation_error_new() {
        let error = ValidationError::new("field.name", "Error message");
        assert_eq!(error.field, "field.name");
        assert_eq!(error.message, "Error message");
    }

    #[test]
    fn test_artifact_ref_new() {
        let artifact_ref = ArtifactRef::new("requirements.md", ArtifactType::Requirements, "draft");
        assert_eq!(artifact_ref.path, "requirements.md");
        assert_eq!(artifact_ref.artifact_type, ArtifactType::Requirements);
        assert_eq!(artifact_ref.status, "draft");
    }

    #[test]
    fn test_artifact_type_serialization() {
        let artifact_type = ArtifactType::Requirements;

        // Test serialization
        if let Ok(serialized) = serde_json::to_string(&artifact_type) {
            assert!(serialized.contains("Requirements"));

            // Test deserialization
            if let Ok(deserialized) = serde_json::from_str::<ArtifactType>(&serialized) {
                assert_eq!(deserialized, ArtifactType::Requirements);
            } else {
                panic!("Failed to deserialize ArtifactType");
            }
        } else {
            panic!("Failed to serialize ArtifactType");
        }
    }

    #[test]
    fn test_validation_result_serialization() {
        let result = ValidationResult {
            valid: false,
            errors: vec![ValidationError::new("title", "Required")],
            warnings: vec!["Warning".to_string()],
        };

        // Test serialization
        if let Ok(serialized) = serde_json::to_string(&result) {
            // Test deserialization
            if let Ok(deserialized) = serde_json::from_str::<ValidationResult>(&serialized) {
                assert!(!deserialized.valid);
                assert_eq!(deserialized.errors.len(), 1);
                assert_eq!(deserialized.warnings.len(), 1);
            } else {
                panic!("Failed to deserialize ValidationResult");
            }
        } else {
            panic!("Failed to serialize ValidationResult");
        }
    }
}
