//! # `AirsSpec` Core
//!
//! Core traits, types, and errors for `AirsSpec` multi-agent orchestration framework.
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
//! - [`reasoning`] - Reasoning pattern abstractions
//! - [`agent`] - Agent execution traits
//! - [`plugin`] - Plugin loading traits
//!
//! ## Design Philosophy
//!
//! All crates in the `AirsSpec` ecosystem depend on `airsspec-core` for their abstractions.
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

// Sub-phase 1.1: Primitives
pub mod error;
pub mod state;

// Sub-phase 1.2: Contract Layer
pub mod artifact;
pub mod tool;

// Sub-phase 1.3: Cognition Layer
pub mod knowledge;
pub mod llm;
pub mod memory;
pub mod reasoning;

// Sub-phase 1.4: Agent Layer
pub mod agent;
pub mod plugin;
