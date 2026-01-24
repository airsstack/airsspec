//! # airsspec-tui
//!
//! Terminal UI components for `AirsSpec`.
//!
//! This crate provides interactive terminal interfaces using `ratatui` and `crossterm`.
//! It is a stateless presentation layer that renders domain models from `airsspec-core`.
//!
//! ## Architecture
//!
//! Per [ADR-002: 4-Crate Structure](../../.memory-bank/sub-projects/airsspec/docs/adr/adr-002-4-crate-structure.md),
//! this crate handles all terminal UI concerns:
//!
//! - Stateless rendering (state passed in, not stored)
//! - Event-driven architecture
//! - Implements core traits where needed
//! - No MCP knowledge (separation of concerns)
//!
//! ## Modules (Future)
//!
//! - `wizard/` - Init wizard for project setup
//! - `reporter/` - Validation reporter for error display
//! - `widgets/` - Reusable UI components (input, select, progress)
//! - `theme/` - Color and style definitions
//!
//! ## Dependencies
//!
//! - `airsspec-core` - Domain models and types
//! - `ratatui` - Terminal UI framework
//! - `crossterm` - Terminal manipulation

// Terminal UI components: wizard, reporter, widgets will be implemented in Phase 3
