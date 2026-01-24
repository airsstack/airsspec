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
//! Per [ADR-002: 4-Crate Structure](../../.memory-bank/sub-projects/airsspec/docs/adr/adr-002-4-crate-structure.md),
//! this crate defines all domain abstractions (traits) that other crates implement:
//!
//! - Domain models: `Spec`, `Plan`, `State`, `Config`
//! - Trait definitions: `WorkspaceProvider`, `SpecStorage`, `Validator`
//! - Business logic: State machine transitions, validation rules
//! - Pure utilities: Slug generation, ID generation
//!
//! ## Modules (Future)
//!
//! - `models/` - Domain types and structures
//! - `traits/` - Trait definitions for external implementations
//! - `state/` - State machine and transition logic
//! - `validation/` - Validation rules and error reporting
//! - `utils/` - Pure utility functions
//!
//! ## Dependencies
//!
//! Only minimal dependencies allowed:
//! - `serde` - Serialization/deserialization
//! - `thiserror` - Error type definitions
//! - `chrono` - Time and date handling
//!
//! **NO** tokio, **NO** file I/O, **NO** network operations.

// Domain models, traits, and business logic will be implemented in Phase 2
