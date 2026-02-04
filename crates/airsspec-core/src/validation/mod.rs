//! Validation framework for `AirsSpec`.
//!
//! This module provides the core validation infrastructure used across
//! all domain modules. It implements the permissive validation pattern
//! described in ADR-005.
//!
//! ## Architecture
//!
//! The validation framework consists of:
//!
//! - [`ValidationSeverity`] - Issue severity levels (Info, Warning, Error)
//! - [`ValidationIssue`] - A single validation issue with severity and message
//! - [`ValidationReport`] - Collection of issues with merge support
//! - [`Validator`] - Generic trait for implementing validators
//! - [`ValidationContext`] - Context for workspace-level validation
//!
//! ## Permissive Validation (ADR-005)
//!
//! Validators collect ALL issues rather than failing on the first error.
//! This provides better user experience by showing all problems at once.
//!
//! ```rust
//! use airsspec_core::validation::{ValidationReport, ValidationIssue};
//!
//! // Validators collect all issues
//! let mut report = ValidationReport::new();
//! report.add_error("Missing title");
//! report.add_warning("Description is short");
//!
//! // Reports can be merged
//! let mut other = ValidationReport::new();
//! other.add_error("Invalid dependency");
//! report.merge(other);
//!
//! // Check validity (only errors count)
//! assert!(!report.is_valid());
//! assert_eq!(report.error_count(), 2);
//! assert_eq!(report.warning_count(), 1);
//! ```
//!
//! ## Domain Validators
//!
//! Domain-specific validators are in their respective modules:
//!
//! - [`crate::spec::validate_spec`] - Spec validation
//! - [`crate::plan::validate_plan`] - Plan validation
//!
//! These functions use the validation framework from this module.

mod context;
mod issue;
mod report;
mod severity;
mod validator;

pub use context::{ValidationContext, ValidationContextBuilder};
pub use issue::ValidationIssue;
pub use report::ValidationReport;
pub use severity::ValidationSeverity;
pub use validator::{Validator, ValidatorExt};
