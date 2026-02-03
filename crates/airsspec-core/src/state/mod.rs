//! # State Module
//!
//! State machine and workflow tracking for specifications.
//!
//! This module contains:
//! - [`StateMachine`] - Enforces valid lifecycle transitions
//! - [`WorkflowState`] - Complete workflow state for a spec
//! - [`BuildProgress`] - Build phase progress tracking
//! - [`StateError`] - State-related errors
//!
//! ## State Transition Rules
//!
//! - **Draft -> Active**: Spec is ready to start work
//! - **Active -> Done**: Work completed successfully
//! - **Active -> Blocked**: Waiting on dependencies
//! - **Active -> Cancelled**: Work abandoned
//! - **Blocked -> Active**: Dependencies resolved
//! - **Done -> Archived**: Historical reference
//! - **Cancelled -> Archived**: Historical reference
//!
//! ## Example
//!
//! ```
//! use airsspec_core::state::{StateMachine, WorkflowState, BuildProgress};
//! use airsspec_core::shared::LifecycleState;
//! use airsspec_core::spec::SpecId;
//!
//! let spec_id = SpecId::new(1737734400, "user-auth");
//! let machine = StateMachine::new();
//!
//! // Check if transition is valid
//! assert!(machine.can_transition(LifecycleState::Draft, LifecycleState::Active));
//!
//! // Attempt a transition
//! let new_state = machine.transition(LifecycleState::Draft, LifecycleState::Active);
//! assert_eq!(new_state.unwrap(), LifecycleState::Active);
//!
//! // Track workflow state
//! let mut workflow = WorkflowState::new(spec_id);
//! workflow.set_lifecycle(LifecycleState::Active);
//! ```

mod error;
mod machine;
mod progress;
mod workflow;

pub use error::StateError;
pub use machine::StateMachine;
pub use progress::BuildProgress;
pub use workflow::WorkflowState;
