# PLAN-003: Memory Types

## Objective

Define memory types in `src/memory/types.rs`.

## Context

- **ADR Reference**: [ADR-003-cognition.md](../../../adrs/ADR-003-cognition.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/memory/` directory
2. Create `src/memory/mod.rs` with module exports
3. Create `src/memory/types.rs` with:
   - `MemoryFragment` struct: `id`, `content`, `created_at`, `token_count`
   - `CompressionConfig` struct: `threshold_tokens`, `target_ratio`
4. Derive serialization traits

## Verification

- [ ] `cargo build -p airsspec-core` passes
