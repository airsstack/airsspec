//! # Plan Domain Module
//!
//! This module contains all types and logic related to implementation plans.
//!
//! ## Types
//!
//! - [`Plan`] - The main plan type
//! - [`PlanStep`] - Individual implementation steps
//! - [`StepStatus`] - Status of a plan step
//! - [`Complexity`] - Complexity estimate for steps
//! - [`PlanBuilder`] - Builder pattern for creating plans
//! - [`StepBuilder`] - Builder pattern for creating steps
//! - [`PlanError`] - Domain-specific errors
//! - [`PlanStorage`] - Trait for plan persistence
//!
//! ## Validation
//!
//! - [`validate_plan`] - Validate a plan
//! - [`ValidationReport`] - Report of validation issues (reused from spec)
//! - [`ValidationIssue`] - A single validation issue (reused from spec)
//! - [`ValidationSeverity`] - Severity level (reused from spec)
//!
//! ## Architecture
//!
//! Following the modular monolith pattern (ADR-002), all Plan-related
//! code lives in this module. Each type has its own file for clarity.
//!
//! ## Example
//!
//! ```
//! use airsspec_core::spec::SpecId;
//! use airsspec_core::plan::{
//!     Plan, PlanStep, PlanBuilder, StepBuilder,
//!     StepStatus, Complexity,
//!     validate_plan,
//! };
//!
//! // Create a plan using the builder
//! let spec_id = SpecId::new(1_737_734_400, "user-auth");
//!
//! let plan = PlanBuilder::new()
//!     .spec_id(spec_id)
//!     .approach("Incremental implementation")
//!     .step(PlanStep::new(0, "Setup database", "Create schema"))
//!     .step(PlanStep::new(1, "Implement API", "Create endpoints"))
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(plan.step_count(), 2);
//!
//! // Validate the plan
//! let report = validate_plan(&plan);
//! assert!(report.is_valid());
//!
//! // Create a step using the builder
//! let step = StepBuilder::new()
//!     .index(0)
//!     .title("Setup")
//!     .description("Initial setup")
//!     .complexity(Complexity::Simple)
//!     .build()
//!     .unwrap();
//! ```

mod builder;
mod error;
mod step;
mod storage;
mod types;
mod validator;

pub use builder::PlanBuilder;
pub use error::PlanError;
pub use step::{Complexity, PlanStep, StepBuilder, StepStatus};
pub use storage::{PlanStorage, PlanStorageExt};
pub use types::Plan;
pub use validator::{validate_plan, ValidationIssue, ValidationReport, ValidationSeverity};
