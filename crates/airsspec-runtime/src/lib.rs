//! # `AirsSpec` Runtime
//!
//! Orchestrator and state machine for `AirsSpec` agent execution.
//!
//! This crate is the "brain" of the `AirsSpec` system, responsible for:
//!
//! - **Orchestrator**: Coordinates agent execution and delegation
//! - **State Machine**: Manages UOW phase transitions
//! - **Memory Manager**: Handles Hot/Warm/Cold memory tiers
//! - **Budget Enforcement**: Tracks and limits agent execution
//!
//! ## State Machine
//!
//! The runtime implements the AI-DLC phase transitions:
//!
//! ```text
//! IDLE ──▶ RESEARCH ──▶ INCEPTION ──▶ DESIGN ──▶ PLANNING ──▶ CONSTRUCTION ──▶ COMPLETE
//!   │                                                                              │
//!   └──────────────────────── (Fast Track) ────────────────────────────────────────┘
//! ```
//!
//! ## Orchestrator Pattern
//!
//! ```text
//! ┌────────────────┐
//! │  Orchestrator  │
//! └───────┬────────┘
//!         │
//!    ┌────┴────┐
//!    ▼         ▼
//! ┌─────┐  ┌─────┐
//! │Agent│  │Agent│  ... (Sequential execution)
//! └─────┘  └─────┘
//! ```
//!
//! ## Memory Management
//!
//! ```text
//! ┌─────────────────┐
//! │   Hot Memory    │  ← Rolling window (~30 messages)
//! └────────┬────────┘
//!          │ compress when > 80%
//!          ▼
//! ┌─────────────────┐
//! │   Warm Memory   │  ← Tree-reduced summaries
//! └────────┬────────┘
//!          │ index
//!          ▼
//! ┌─────────────────┐
//! │   Cold Memory   │  ← Vector store (Lance)
//! └─────────────────┘
//! ```

// Modules will be added as we implement:
// pub mod orchestrator;
// pub mod state_machine;
// pub mod memory_manager;
// pub mod executor;
