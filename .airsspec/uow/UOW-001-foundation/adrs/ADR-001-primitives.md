---
id: ADR-001
title: Primitives Module Design
status: draft
date: 2026-01-10
uow_ref: UOW-001-foundation
sub_phase: "1.1"
---

# ADR-001: Primitives Module Design

## Context

The Primitives module (`error` + `state`) provides foundational types that all other modules depend upon.

## Decision

### Error Types

```rust
// src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AirsspecError {
    #[error("State error: {0}")]
    State(#[from] StateError),
    
    #[error("Artifact error: {0}")]
    Artifact(#[from] ArtifactError),
    
    #[error("Tool error: {0}")]
    Tool(#[from] ToolError),
    
    #[error("LLM error: {0}")]
    Llm(#[from] LlmError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Invalid transition from {from:?} to {to:?}")]
    InvalidTransition { from: Phase, to: Phase },
    
    #[error("Gate condition not met: {0}")]
    GateNotMet(String),
}
```

### State Types

```rust
// src/state/types.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Idle,
    Research,
    Inception,
    Design,
    Planning,
    Construction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UowState {
    pub id: String,
    pub phase: Phase,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub from: Phase,
    pub to: Phase,
    pub at: DateTime<Utc>,
    pub reason: Option<String>,
}
```

### State Trait

```rust
// src/state/traits.rs
use async_trait::async_trait;

#[async_trait]
pub trait StatePersistence: Send + Sync {
    async fn load(&self, uow_id: &str) -> Result<UowState, StateError>;
    async fn save(&self, state: &UowState) -> Result<(), StateError>;
    async fn record_transition(&self, uow_id: &str, transition: Transition) -> Result<(), StateError>;
}

/// Enforces phase transition rules (gate conditions)
pub trait ComplianceGate: Send + Sync {
    /// Check if transition from current phase is allowed
    fn can_transition(&self, from: Phase, to: Phase, artifacts: &[ArtifactRef]) -> bool;
    
    /// Get required artifacts that must exist and be approved for a phase
    fn required_artifacts(&self, phase: Phase) -> Vec<ArtifactType>;
    
    /// Validate that all gate conditions are met
    fn validate_gate(&self, state: &UowState) -> Result<(), StateError>;
}
```

## Consequences

- All modules can import error types from a single location
- Phase transitions are explicit and auditable
- State persistence is abstracted for different backends
- Gate conditions are enforced via `ComplianceGate` trait
