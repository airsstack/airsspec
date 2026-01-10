---
id: ADR-002
title: Contract Module Design
status: draft
date: 2026-01-10
uow_ref: UOW-001-foundation
sub_phase: "1.2"
---

# ADR-002: Contract Module Design

## Context

The Contract module (`artifact` + `tool`) defines abstractions for artifact validation and tool execution.

## Decision

### Artifact Types

```rust
// src/artifact/types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactType {
    Requirements,
    Daa,
    Adr,
    Rfc,
    BoltPlan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}
```

### Artifact Traits

```rust
// src/artifact/traits.rs
use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait ArtifactValidator: Send + Sync {
    fn artifact_type(&self) -> ArtifactType;
    async fn validate(&self, content: &str) -> ValidationResult;
    async fn validate_file(&self, path: &Path) -> Result<ValidationResult, ArtifactError>;
}

#[async_trait]
pub trait ArtifactStore: Send + Sync {
    async fn read(&self, path: &Path) -> Result<String, ArtifactError>;
    async fn write(&self, path: &Path, content: &str) -> Result<(), ArtifactError>;
    async fn exists(&self, path: &Path) -> bool;
}
```

### Tool Types

```rust
// src/tool/types.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ToolId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInput {
    pub tool_id: ToolId,
    pub params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub success: bool,
    pub result: Value,
    pub error: Option<String>,
}
```

### Tool Traits

```rust
// src/tool/traits.rs
use async_trait::async_trait;

#[async_trait]
pub trait Tool: Send + Sync {
    fn id(&self) -> &ToolId;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, input: ToolInput) -> Result<ToolOutput, ToolError>;
}

pub trait ToolRegistry: Send + Sync {
    fn register(&mut self, tool: Box<dyn Tool>);
    fn get(&self, id: &ToolId) -> Option<&dyn Tool>;
    fn list(&self) -> Vec<&ToolId>;
}
```

## Consequences

- Artifacts have explicit types with validation
- Tools are registered and retrieved by ID
- All I/O is async for consistency
