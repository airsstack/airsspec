//! # Spec Domain Module
//!
//! This module contains all types and logic related to specifications.
//!
//! ## Types
//!
//! - [`Spec`] - The main specification type
//! - [`SpecMetadata`] - Metadata for a specification
//! - [`SpecId`] - Unique identifier for specifications
//! - [`SpecBuilder`] - Builder pattern for creating specs
//! - [`Category`] - Specification categorization
//! - [`Dependency`] - Spec-to-spec relationships
//! - [`DependencyKind`] - Type of dependency relationship
//! - [`SpecError`] - Domain-specific errors
//! - [`SpecStorage`] - Trait for spec persistence
//!
//! ## Validation
//!
//! - [`validate_spec`] - Validate a specification
//! - [`ValidationReport`] - Report of validation issues
//! - [`ValidationIssue`] - A single validation issue
//! - [`ValidationSeverity`] - Severity level (info, warning, error)
//!
//! ## Architecture
//!
//! Following the modular monolith pattern (ADR-002), all Spec-related
//! code lives in this module. Each type has its own file for clarity.
//!
//! ## Example
//!
//! ```
//! use airsspec_core::spec::{
//!     Spec, SpecId, SpecBuilder, SpecMetadata,
//!     Category, Dependency, DependencyKind,
//!     validate_spec,
//! };
//!
//! // Create a spec using the builder
//! let spec = SpecBuilder::new()
//!     .title("User Authentication")
//!     .description("Implement OAuth2 login flow")
//!     .category(Category::Feature)
//!     .content("# User Auth\n\nImplementation details...")
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(spec.title(), "User Authentication");
//!
//! // Validate the spec
//! let report = validate_spec(&spec);
//! assert!(report.is_valid());
//!
//! // Create a SpecId directly
//! let id = SpecId::new(1_737_734_400, "user-auth");
//!
//! // Create a dependency
//! let dep = Dependency {
//!     spec_id: id.clone(),
//!     kind: DependencyKind::BlockedBy,
//! };
//! ```

mod builder;
mod category;
mod dependency;
mod error;
mod id;
mod storage;
mod types;
mod validator;

pub use builder::SpecBuilder;
pub use category::Category;
pub use dependency::{Dependency, DependencyKind};
pub use error::SpecError;
pub use id::SpecId;
pub use storage::{SpecStorage, SpecStorageExt};
pub use types::{Spec, SpecMetadata};
pub use validator::{ValidationIssue, ValidationReport, ValidationSeverity, validate_spec};
