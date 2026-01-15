# PLAN-001: Agent Executor Setup

## Objective

Create `airsspec-agents` crate with event-driven executor foundation.

## Context

From ADR-003, implement event-driven architecture with reasoning pattern abstraction. This is the orchestration layer that coordinates LLM and tools.

## Steps

1. Create `crates/airsspec-agents/` directory
2. Add `Cargo.toml` with dependencies:
   - `airsspec-core`, `airsspec-llm`, `airsspec-tools`
   - `tokio`, `futures`, `serde`, `async-trait`
3. Create module structure:
   - `src/executor.rs`
   - `src/events.rs`
   - `src/session.rs`
   - `src/error.rs`
   - `src/patterns/` (directory for next plans)
   - `src/agents/` (directory for BOLT-004)
4. Define core types:
   - `AgentExecutor` struct
   - `Session` struct with lifecycle
   - `SessionStatus` enum
   - `SessionEvent` enum
5. Implement basic session management

## Expected Output

### Files Created:
- `crates/airsspec-agents/Cargo.toml`
- `crates/airsspec-agents/src/lib.rs`
- `crates/airsspec-agents/src/executor.rs`
- `crates/airsspec-agents/src/events.rs`
- `crates/airsspec-agents/src/session.rs`
- `crates/airsspec-agents/src/error.rs`

### Files Modified:
- `Cargo.toml` (workspace) — Add airsspec-agents member

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes
- [ ] Core types compile
- [ ] Session lifecycle is defined

## References

- ADR: [../../adrs/ADR-003-agent-architecture.md](../../adrs/ADR-003-agent-architecture.md)
- RFC: [../../RFC.md](../../RFC.md)
