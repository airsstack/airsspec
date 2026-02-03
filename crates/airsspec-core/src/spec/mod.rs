//! # Spec Domain Module
//!
//! This module contains all types and logic related to specifications.
//!
//! ## Types
//!
//! - [`SpecId`] - Unique identifier for specifications
//! - [`Category`] - Specification categorization
//! - [`Dependency`] - Spec-to-spec relationships
//! - [`DependencyKind`] - Type of dependency relationship
//! - [`SpecError`] - Domain-specific errors
//!
//! ## Architecture
//!
//! Following the modular monolith pattern (ADR-002), all Spec-related
//! code lives in this module. Each type has its own file for clarity.
//!
//! ## Example
//!
//! ```
//! use airsspec_core::spec::{SpecId, Category, Dependency, DependencyKind};
//!
//! // Create a SpecId
//! let id = SpecId::new(1_737_734_400, "user-auth");
//!
//! // Create a dependency
//! let dep = Dependency {
//!     spec_id: id.clone(),
//!     kind: DependencyKind::BlockedBy,
//! };
//! ```

mod category;
mod dependency;
mod error;
mod id;

pub use category::Category;
pub use dependency::{Dependency, DependencyKind};
pub use error::SpecError;
pub use id::SpecId;
