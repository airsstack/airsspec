//! State management types for UOW lifecycle.
//!
//! This module defines the core types for managing UOW state, phases, and transitions.

pub mod traits;
pub mod types;

pub use traits::{ArtifactRef, ArtifactType, ComplianceGate, StatePersistence};
pub use types::{Phase, Transition, UowState};
