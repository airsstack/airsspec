# PLAN-006: Knowledge Traits

## Objective

Define knowledge store, compressor, and vector store traits in `src/knowledge/traits.rs`.

## Context

- **ADR Reference**: [ADR-003-cognition.md](../../../adrs/ADR-003-cognition.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/knowledge/traits.rs`
2. Define `KnowledgeStore` trait:
   - `async fn ingest(&mut self, doc: Document) -> Result<(), KnowledgeError>`
   - `async fn query(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, KnowledgeError>`
3. Define `Compressor` trait:
   - `async fn compress(&self, fragments: Vec<MemoryFragment>) -> Result<String, MemoryError>`
4. Define `VectorStore` trait:
   - `async fn upsert(&mut self, id: &str, embedding: Embedding) -> Result<(), KnowledgeError>`
   - `async fn search(&self, query_embedding: Embedding, limit: usize) -> Result<Vec<(String, f32)>, KnowledgeError>`
5. Add `Send + Sync` bounds

## Verification

- [ ] `cargo build -p airsspec-core` passes
