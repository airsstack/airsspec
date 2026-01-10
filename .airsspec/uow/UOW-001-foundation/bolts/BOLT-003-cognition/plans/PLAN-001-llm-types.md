# PLAN-001: LLM Types

## Objective

Define LLM types in `src/llm/types.rs`.

## Context

- **ADR Reference**: [ADR-003-cognition.md](../../../adrs/ADR-003-cognition.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/llm/` directory
2. Create `src/llm/mod.rs` with module exports
3. Create `src/llm/types.rs` with:
   - `CompletionRequest` struct: `messages`, `max_tokens`, `temperature`
   - `Message` struct: `role`, `content`
   - `Role` enum: `System`, `User`, `Assistant`
   - `TokenUsage` struct: `prompt_tokens`, `completion_tokens`, `total_tokens`
4. Derive serialization traits

## Verification

- [ ] `cargo build -p airsspec-core` passes
