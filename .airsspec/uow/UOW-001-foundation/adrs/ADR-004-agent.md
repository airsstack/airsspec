---
id: ADR-004
title: Agent Module Design
status: draft
date: 2026-01-10
uow_ref: UOW-001-foundation
sub_phase: "1.4"
---

# ADR-004: Agent Module Design

## Context

The Agent module (`agent` + `plugin`) defines abstractions for agent execution and plugin loading.

## Decision

### Agent Types

```rust
// src/agent/types.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: AgentId,
    pub name: String,
    pub description: String,
    pub allowed_tools: Vec<ToolId>,
    pub allowed_paths: Vec<PathBuf>,
    pub budget: Budget,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Budget {
    pub max_tokens: u32,
    pub max_iterations: u32,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelegationSignal {
    Delegate { target: AgentId, query: String },
    Complete { result: String },
    Error { message: String },
}
```

### Agent Traits

```rust
// src/agent/traits.rs
use async_trait::async_trait;

#[async_trait]
pub trait Agent: Send + Sync {
    fn id(&self) -> &AgentId;
    fn config(&self) -> &AgentConfig;
    
    async fn execute(&self, context: AgentContext) -> Result<AgentOutput, AgentError>;
}

#[derive(Debug, Clone)]
pub struct AgentContext {
    pub uow_id: String,
    pub phase: Phase,
    pub memory: Vec<MemoryFragment>,
    pub artifacts: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct AgentOutput {
    pub result: String,
    pub artifacts_created: Vec<PathBuf>,
    pub delegation: Option<DelegationSignal>,
    pub token_usage: TokenUsage,
}

#[async_trait]
pub trait AgentExecutor: Send + Sync {
    async fn run(
        &self,
        agent: &dyn Agent,
        budget: Budget,
    ) -> Result<ExecutionResult, ExecutionError>;
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub output: AgentOutput,
    pub iterations: u32,
    pub total_tokens: u32,
}
```

### Plugin Types

```rust
// src/plugin/types.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub overlays: Vec<OverlaySpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlaySpec {
    pub target: AgentId,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionOrder {
    CoreFirst,
    PluginFirst,
    UserFirst,
}
```

### Plugin Traits

```rust
// src/plugin/traits.rs
use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait PluginLoader: Send + Sync {
    async fn load(&self, plugin_dir: &Path) -> Result<PluginManifest, PluginError>;
    async fn list_plugins(&self, workspace: &Path) -> Result<Vec<PluginManifest>, PluginError>;
}

pub trait PromptOverlay: Send + Sync {
    fn stack_prompts(&self, agent_id: &AgentId, order: ResolutionOrder) -> String;
}
```

## Consequences

- Agents are first-class with explicit budgets
- Delegation between agents is signal-based
- Plugins extend agent prompts via overlays
