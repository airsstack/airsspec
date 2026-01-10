---
id: ADR-003
title: Cognition Module Design
status: draft
date: 2026-01-10
uow_ref: UOW-001-foundation
sub_phase: "1.3"
---

# ADR-003: Cognition Module Design

## Context

The Cognition module (`llm` + `memory` + `knowledge`) defines abstractions for LLM interaction and memory management.

## Decision

### LLM Types

```rust
// src/llm/types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
```

### LLM Traits

```rust
// src/llm/traits.rs
use async_trait::async_trait;

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn complete(&self, request: CompletionRequest) -> Result<String, LlmError>;
    async fn complete_with_usage(&self, request: CompletionRequest) -> Result<(String, TokenUsage), LlmError>;
}

#[async_trait]
pub trait StreamHandler: Send + Sync {
    async fn on_token(&mut self, token: &str);
    async fn on_complete(&mut self);
    async fn on_error(&mut self, error: &LlmError);
}
```

### Memory Types

```rust
// src/memory/types.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFragment {
    pub id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub token_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub threshold_tokens: u32,
    pub target_ratio: f32,
}
```

### Memory Traits

```rust
// src/memory/traits.rs
use async_trait::async_trait;

#[async_trait]
pub trait HotMemory: Send + Sync {
    async fn push(&mut self, fragment: MemoryFragment);
    async fn get_window(&self, limit: usize) -> Vec<MemoryFragment>;
    async fn token_count(&self) -> u32;
    async fn clear(&mut self);
}

#[async_trait]
pub trait WarmMemory: Send + Sync {
    async fn store(&mut self, summary: String) -> Result<String, MemoryError>;
    async fn retrieve(&self, id: &str) -> Result<String, MemoryError>;
}

#[async_trait]
pub trait ColdMemory: Send + Sync {
    async fn index(&mut self, content: &str) -> Result<(), MemoryError>;
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, MemoryError>;
}
```

### Knowledge Types & Traits

```rust
// src/knowledge/types.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub content: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    pub vector: Vec<f32>,
    pub dimensions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub document_id: String,
    pub score: f32,
    pub snippet: String,
}

// src/knowledge/traits.rs
#[async_trait]
pub trait KnowledgeStore: Send + Sync {
    async fn ingest(&mut self, doc: Document) -> Result<(), KnowledgeError>;
    async fn query(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, KnowledgeError>;
}

#[async_trait]
pub trait Compressor: Send + Sync {
    async fn compress(&self, fragments: Vec<MemoryFragment>) -> Result<String, MemoryError>;
}

#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn upsert(&mut self, id: &str, embedding: Embedding) -> Result<(), KnowledgeError>;
    async fn search(&self, query_embedding: Embedding, limit: usize) -> Result<Vec<(String, f32)>, KnowledgeError>;
}
```

## Consequences

- LLM providers are swappable via trait
- 3-tier memory (Hot/Warm/Cold) is explicit
- Knowledge retrieval follows RAG patterns
