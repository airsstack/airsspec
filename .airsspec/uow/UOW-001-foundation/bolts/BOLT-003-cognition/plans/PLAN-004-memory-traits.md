# PLAN-004: Memory Traits

## Objective

Define 3-tier memory traits in `src/memory/traits.rs`.

## Context

- **ADR Reference**: [ADR-003-cognition.md](../../../adrs/ADR-003-cognition.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/memory/traits.rs`
2. Define `HotMemory` trait:
   - `async fn push(&mut self, fragment: MemoryFragment)`
   - `async fn get_window(&self, limit: usize) -> Vec<MemoryFragment>`
   - `async fn token_count(&self) -> u32`
   - `async fn clear(&mut self)`
3. Define `WarmMemory` trait:
   - `async fn store(&mut self, summary: String) -> Result<String, MemoryError>`
   - `async fn retrieve(&self, id: &str) -> Result<String, MemoryError>`
4. Define `ColdMemory` trait:
   - `async fn index(&mut self, content: &str) -> Result<(), MemoryError>`
   - `async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, MemoryError>`
5. Add `Send + Sync` bounds

## Verification

- [ ] `cargo build -p airsspec-core` passes
