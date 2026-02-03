//! # Shared Module
//!
//! Cross-cutting types used across domain modules.
//!
//! ## Types
//!
//! - [`LifecycleState`] - Lifecycle states for specifications and plans
//! - [`Phase`] - Workflow phases (Spec, Plan, Build)
//!
//! ## Example
//!
//! ```
//! use airsspec_core::shared::{LifecycleState, Phase};
//!
//! let state = LifecycleState::Draft;
//! let phase = Phase::Spec;
//!
//! assert!(!state.is_terminal());
//! assert_eq!(phase.next(), Some(Phase::Plan));
//! ```

mod lifecycle;
mod phase;

pub use lifecycle::LifecycleState;
pub use phase::Phase;
