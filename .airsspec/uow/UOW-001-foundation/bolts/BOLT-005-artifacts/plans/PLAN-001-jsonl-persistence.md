# PLAN-001: JSONL Persistence

## Objective

Implement JSONL persistence for artifacts in `airsspec-artifacts` crate.

## Context

- **RFC Reference**: [RFC.md](../../../../RFC.md)
- **Crate**: `airsspec-artifacts`

## Steps

1. Implement `ArtifactStore` trait in `src/persistence.rs`
2. Create `JsonlPersistence` struct implementing:
   - `read()` — Parse JSONL frontmatter and body
   - `write()` — Serialize with YAML frontmatter
   - `exists()` — Check file existence
3. Handle frontmatter parsing (YAML between `---` markers)
4. Add error handling for malformed files

## Verification

- [ ] `cargo build -p airsspec-artifacts` passes
- [ ] Unit tests for read/write round-trip
