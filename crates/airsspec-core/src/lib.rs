//! # AirsSpec Core
//!
//! Core traits, types, and errors for the AirsSpec multi-agent orchestration framework.
//!
//! This crate contains **zero implementations** - only abstractions that other crates
//! depend on. This enforces the Dependency Inversion Principle (DIP) throughout the system.
//!
//! ## Module Structure
//!
//! The crate is organized into domain-specific modules:
//!
//! - [`error`] - Shared error types
//! - [`state`] - Phase, UOW state, and transitions
//! - [`artifact`] - Artifact validation and storage traits
//! - [`tool`] - Tool definitions and registry traits
//! - [`llm`] - LLM provider traits
//! - [`memory`] - Memory tier traits (Hot/Warm/Cold)
//! - [`knowledge`] - Knowledge store and vector search traits
//! - [`agent`] - Agent execution traits
//! - [`plugin`] - Plugin loading traits
//!
//! ## Design Philosophy
//!
//! All crates in the AirsSpec ecosystem depend on `airsspec-core` for their abstractions.
//! Implementations live in separate crates (e.g., `airsspec-llm`, `airsspec-agents`).
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │           airsspec-runtime              │
//! │           airsspec-agents               │
//! │           airsspec-llm                  │
//! │              ...                        │
//! └─────────────────┬───────────────────────┘
//!                   │ depends on
//!                   ▼
//! ┌─────────────────────────────────────────┐
//! │           airsspec-core                 │
//! │     (traits, types, errors only)        │
//! └─────────────────────────────────────────┘
//! ```

// Modules will be added as we implement sub-phases:
// Sub-phase 1.1: Primitives
// pub mod error;
// pub mod state;

// Sub-phase 1.2: Contract Layer
// pub mod artifact;
// pub mod tool;

// Sub-phase 1.3: Cognition Layer
// pub mod llm;
// pub mod memory;
// pub mod knowledge;

// Sub-phase 1.4: Agent Layer
// pub mod agent;
// pub mod plugin;

/// Placeholder to make the crate compile.
/// This will be removed as modules are implemented.
pub fn placeholder() {
    // TODO: Remove when first module is added
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        // Basic smoke test
        super::placeholder();
    }
}
