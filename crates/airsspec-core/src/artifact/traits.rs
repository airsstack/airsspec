//! Artifact validator and store traits.
//!
//! This module defines the core traits for artifact validation and storage.
//! Implementations of these traits are provided in separate crates to enable
//! different validation strategies and storage backends.

// Layer 1: Standard library imports
use std::path::Path;

// Layer 2: Third-party crate imports
use async_trait::async_trait;

// Layer 3: Internal module imports
use crate::artifact::types::{ArtifactType, ValidationResult};
use crate::error::ArtifactError;

/// Trait for validating artifacts.
///
/// This trait defines the interface for validating artifact content according
/// to its type's schema and rules. Implementations can provide different
/// validation strategies (e.g., JSON Schema, custom validators).
///
/// # Examples
///
/// ```rust,ignore
/// use airsspec_core::artifact::{ArtifactValidator, ArtifactType, ValidationResult};
/// use async_trait::async_trait;
///
/// struct MyValidator;
///
/// #[async_trait]
/// impl ArtifactValidator for MyValidator {
///     fn artifact_type(&self) -> ArtifactType {
///         ArtifactType::Requirements
///     }
///
///     async fn validate(&self, content: &str) -> ValidationResult {
///         // Implementation here
///         ValidationResult::success()
///     }
///
///     async fn validate_file(&self, path: &Path) -> Result<ValidationResult, ArtifactError> {
///         // Implementation here
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait ArtifactValidator: Send + Sync {
    /// Returns the artifact type this validator handles.
    ///
    /// This method allows the validator to be registered for a specific
    /// artifact type in a validator registry.
    ///
    /// # Returns
    ///
    /// The artifact type this validator validates.
    fn artifact_type(&self) -> ArtifactType;

    /// Validates artifact content.
    ///
    /// This method validates the provided content string and returns a
    /// validation result with any errors or warnings found.
    ///
    /// # Arguments
    ///
    /// * `content` - The artifact content to validate
    ///
    /// # Returns
    ///
    /// A validation result containing any errors or warnings.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let result = validator.validate(content).await;
    /// if result.valid {
    ///     println!("Artifact is valid!");
    /// } else {
    ///     for error in result.errors {
    ///         eprintln!("Error in field '{}': {}", error.field, error.message);
    ///     }
    /// }
    /// ```
    async fn validate(&self, content: &str) -> ValidationResult;

    /// Validates an artifact file.
    ///
    /// This method reads the artifact from the specified path and validates it.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the artifact file
    ///
    /// # Returns
    ///
    /// A validation result if the file was read successfully, or an error
    /// if the file could not be read.
    ///
    /// # Errors
    ///
    /// Returns an `ArtifactError` if the file could not be read.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let path = Path::new("requirements.md");
    /// let result = validator.validate_file(path).await?;
    /// ```
    async fn validate_file(&self, path: &Path) -> Result<ValidationResult, ArtifactError>;
}

/// Trait for artifact storage operations.
///
/// This trait defines the interface for reading and writing artifacts
/// to persistent storage. Implementations can provide different storage
/// backends (e.g., filesystem, S3, database).
///
/// # Examples
///
/// ```rust,ignore
/// use airsspec_core::artifact::{ArtifactStore, ArtifactError};
/// use async_trait::async_trait;
/// use std::path::Path;
///
/// struct MyStore;
///
/// #[async_trait]
/// impl ArtifactStore for MyStore {
///     async fn read(&self, path: &Path) -> Result<String, ArtifactError> {
///         // Implementation here
///         todo!()
///     }
///
///     async fn write(&self, path: &Path, content: &str) -> Result<(), ArtifactError> {
///         // Implementation here
///         todo!()
///     }
///
///     async fn exists(&self, path: &Path) -> bool {
///         // Implementation here
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait ArtifactStore: Send + Sync {
    /// Reads artifact content from storage.
    ///
    /// This method reads the artifact content from the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the artifact
    ///
    /// # Returns
    ///
    /// The artifact content as a string if successful.
    ///
    /// # Errors
    ///
    /// Returns an `ArtifactError` if the artifact could not be read.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let path = Path::new("requirements.md");
    /// let content = store.read(path).await?;
    /// ```
    async fn read(&self, path: &Path) -> Result<String, ArtifactError>;

    /// Writes artifact content to storage.
    ///
    /// This method writes the provided content to the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to write the artifact to
    /// * `content` - The artifact content to write
    ///
    /// # Returns
    ///
    /// `Ok(())` if the write was successful.
    ///
    /// # Errors
    ///
    /// Returns an `ArtifactError` if the artifact could not be written.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let path = Path::new("requirements.md");
    /// let content = "# Requirements\n\nThis is a requirements document.";
    /// store.write(path, content).await?;
    /// ```
    async fn write(&self, path: &Path, content: &str) -> Result<(), ArtifactError>;

    /// Checks if an artifact exists in storage.
    ///
    /// This method checks if an artifact exists at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check
    ///
    /// # Returns
    ///
    /// `true` if the artifact exists, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let path = Path::new("requirements.md");
    /// if store.exists(path).await {
    ///     println!("Artifact exists");
    /// } else {
    ///     println!("Artifact does not exist");
    /// }
    /// ```
    async fn exists(&self, path: &Path) -> bool;
}
