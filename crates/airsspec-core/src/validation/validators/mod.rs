//! Workspace-level validators for the `AirsSpec` validation engine.
//!
//! This module contains validators that operate on the workspace as a whole,
//! validating cross-cutting concerns like directory structure, spec content,
//! cross-spec dependencies, and state consistency.
//!
//! Each validator implements [`Validator<ValidationContext<S, P>>`](crate::validation::Validator)
//! with appropriate trait bounds on `S` and `P`, and returns a
//! [`ValidationReport`](crate::validation::ValidationReport).
//!
//! ## Dependency Inversion Principle
//!
//! Validators use trait bounds ([`ValidatableSpec`](crate::validation::ValidatableSpec),
//! [`ValidatablePlan`](crate::validation::ValidatablePlan)) rather than concrete domain types.
//! This means the validation framework never imports from domain modules.
//!
//! ## Validators
//!
//! - [`DirectoryStructureValidator`] -- Checks workspace directory structure
//! - [`SpecContentValidator`] -- Validates all specs using the `ValidatableSpec` trait
//! - [`DependencyValidator`] -- Cross-spec dependency validation (broken refs, cycles)
//! - [`StateTransitionValidator`] -- Validates specs have required artifacts for their state

mod content;
mod dependencies;
mod state;
mod structure;

pub use content::SpecContentValidator;
pub use dependencies::DependencyValidator;
pub use state::StateTransitionValidator;
pub use structure::DirectoryStructureValidator;
