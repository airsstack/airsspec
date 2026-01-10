# PLAN-001: Agent Types

## Objective

Define agent types in `src/agent/types.rs`.

## Context

- **ADR Reference**: [ADR-004-agent.md](../../../adrs/ADR-004-agent.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/agent/` directory
2. Create `src/agent/mod.rs` with module exports
3. Create `src/agent/types.rs` with:
   - `AgentId` newtype struct
   - `AgentConfig` struct: `max_loops`, `allowed_tools`, `allowed_paths`
   - `Budget` struct: `max_tokens`, `remaining_tokens`, `max_iterations`
   - `DelegationSignal` struct: `target_agent`, `query`, `context`
4. Derive serialization traits

## Verification

- [ ] `cargo build -p airsspec-core` passes
