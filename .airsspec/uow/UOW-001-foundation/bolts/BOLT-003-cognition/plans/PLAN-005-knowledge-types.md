# PLAN-005: Knowledge Types

## Objective

Define knowledge types in `src/knowledge/types.rs`.

## Context

- **ADR Reference**: [ADR-003-cognition.md](../../../adrs/ADR-003-cognition.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/knowledge/` directory
2. Create `src/knowledge/mod.rs` with module exports
3. Create `src/knowledge/types.rs` with:
   - `Document` struct: `id`, `content`, `metadata`
   - `Embedding` struct: `vector`, `dimensions`
   - `SearchResult` struct: `document_id`, `score`, `snippet`
4. Derive serialization traits

## Verification

- [ ] `cargo build -p airsspec-core` passes
