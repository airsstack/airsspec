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
//! - [`spec`] - Specification domain (`Spec`, `SpecId`, `SpecBuilder`, `Category`, `Dependency`, errors)
//! - [`plan`] - Plan domain (`Plan`, `PlanStep`, `PlanBuilder`, `StepStatus`, `Complexity`)
//! - [`shared`] - Cross-cutting types (`LifecycleState`, `Phase`)
//! - [`state`] - State machine and workflow tracking (`StateMachine`, `WorkflowState`, `BuildProgress`)
//!
//! ### Future Modules (Phase 2)
//!
//! - `workspace/` - Workspace domain (`ProjectConfig`, `WorkspaceProvider`)
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
//! use airsspec_core::spec::{
//!     Spec, SpecId, SpecBuilder, SpecMetadata,
//!     Category, Dependency, DependencyKind,
//!     validate_spec,
//! };
//! use airsspec_core::plan::{Plan, PlanStep, PlanBuilder, validate_plan};
//! use airsspec_core::shared::{LifecycleState, Phase};
//! use airsspec_core::state::{StateMachine, WorkflowState, BuildProgress};
//!
//! // Create a spec using the builder
//! let spec = SpecBuilder::new()
//!     .title("User Authentication")
//!     .description("Implement OAuth2 login")
//!     .category(Category::Feature)
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(spec.title(), "User Authentication");
//!
//! // Validate the spec
//! let report = validate_spec(&spec);
//! assert!(report.is_valid());
//!
//! // Create a spec ID directly
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
//!
//! // Create a plan for the spec
//! let plan = PlanBuilder::new()
//!     .spec_id(id.clone())
//!     .approach("Incremental implementation")
//!     .step(PlanStep::new(0, "Setup database", "Create schema"))
//!     .step(PlanStep::new(1, "Implement API", "Create endpoints"))
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(plan.step_count(), 2);
//!
//! // Validate the plan
//! let plan_report = validate_plan(&plan);
//! assert!(plan_report.is_valid());
//!
//! // Use lifecycle states
//! let state = LifecycleState::Draft;
//! assert!(!state.is_terminal());
//!
//! // Use workflow phases
//! let phase = Phase::Spec;
//! assert_eq!(phase.next(), Some(Phase::Plan));
//!
//! // Use the state machine
//! let machine = StateMachine::new();
//! assert!(machine.can_transition(LifecycleState::Draft, LifecycleState::Active));
//!
//! // Track workflow state
//! let mut workflow = WorkflowState::new(id);
//! workflow.set_lifecycle(LifecycleState::Active);
//! assert_eq!(workflow.lifecycle(), LifecycleState::Active);
//! ```

pub mod plan;
pub mod shared;
pub mod spec;
pub mod state;

// Convenience re-exports for common types
pub use plan::{
    Complexity, Plan, PlanBuilder, PlanError, PlanStep, PlanStorage, PlanStorageExt, StepBuilder,
    StepStatus, validate_plan,
};
pub use shared::{LifecycleState, Phase};
pub use spec::{
    Category, Dependency, DependencyKind, Spec, SpecBuilder, SpecError, SpecId, SpecMetadata,
    SpecStorage, SpecStorageExt, ValidationIssue, ValidationReport, ValidationSeverity,
    validate_spec,
};
pub use state::{BuildProgress, StateError, StateMachine, WorkflowState};
