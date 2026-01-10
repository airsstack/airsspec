# PLAN-001: Error Types

## Objective

Define all error types for `airsspec-core` in `src/error.rs`.

## Context

- **ADR Reference**: [ADR-001-primitives.md](../../../adrs/ADR-001-primitives.md)
- **Crate**: `airsspec-core`
- **File**: `src/error.rs`

## Steps

1. Create `src/error.rs` in `crates/airsspec-core/`
2. Define `AirsspecError` enum with variants:
   - `State(StateError)`
   - `Artifact(ArtifactError)`
   - `Tool(ToolError)`
   - `Llm(LlmError)`
   - `Io(std::io::Error)`
3. Define `StateError` enum with variants:
   - `InvalidTransition { from: Phase, to: Phase }`
   - `GateNotMet(String)`
4. Define placeholder error types for other modules:
   - `ArtifactError`
   - `ToolError`
   - `LlmError`
   - `MemoryError`
   - `KnowledgeError`
   - `AgentError`
   - `PluginError`
5. Derive `thiserror::Error` for all error types
6. Add to `lib.rs` as public module

## Verification

- [ ] `cargo build -p airsspec-core` passes
- [ ] All error types derive `Debug`, `Error`
- [ ] Doc comments on all public types
