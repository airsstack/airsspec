//! # airsspec-core
//!
//! Pure domain logic for `AirsSpec` - a lightweight, MCP-first spec-driven development framework.
//!
//! This crate contains domain models, business rules, and trait abstractions following the
//! Dependency Inversion Principle (DIP). It has **zero I/O dependencies** - no tokio, no file I/O,
//! no network operations.
//!
//! ## Architecture
//!
//! Per [ADR-002](../../.memory-bank/sub-projects/airsspec/docs/adr/adr-002-4-crate-structure.md),
//! this crate follows a **modular monolith** pattern, organizing code by domain concepts
//! rather than technical layers.
//!
//! ## Modules
//!
//! ### Domain Modules
//!
//! - [`spec`] - Specification domain (`SpecId`, `Category`, `Dependency`, errors)
//!
//! ### Future Modules (Phase 2)
//!
//! - `plan/` - Plan domain (`Plan`, `PlanStep`, `PlanBuilder`)
//! - `workspace/` - Workspace domain (`ProjectConfig`, `WorkspaceProvider`)
//! - `shared/` - Shared types (`LifecycleState`, `Phase`)
//! - `state/` - State machine and transitions
//! - `validation/` - Validation framework
//! - `utils/` - Pure utilities
//!
//! ## Dependencies
//!
//! Only minimal dependencies allowed:
//! - `serde` - Serialization/deserialization
//! - `thiserror` - Error type definitions
//! - `chrono` - Time and date handling
//!
//! **NO** tokio, **NO** file I/O, **NO** network operations.
//!
//! ## Examples
//!
//! ```
//! use airsspec_core::spec::{SpecId, Category, Dependency, DependencyKind};
//!
//! // Create a spec ID
//! let id = SpecId::new(1_737_734_400, "user-auth");
//! assert_eq!(id.timestamp(), 1_737_734_400);
//! assert_eq!(id.slug(), "user-auth");
//!
//! // Parse a spec ID from string
//! let parsed = SpecId::parse("1737734400-payment-system").unwrap();
//!
//! // Create a dependency
//! let dep = Dependency::blocked_by(id.clone());
//! assert_eq!(dep.kind, DependencyKind::BlockedBy);
//! ```

pub mod spec;

// Convenience re-exports for common types
pub use spec::{Category, Dependency, DependencyKind, SpecError, SpecId};
