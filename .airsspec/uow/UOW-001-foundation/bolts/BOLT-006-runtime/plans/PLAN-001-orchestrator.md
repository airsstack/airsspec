# PLAN-001: Orchestrator Skeleton

## Objective

Implement orchestrator skeleton in `airsspec-runtime` crate.

## Context

- **RFC Reference**: [RFC.md](../../../../RFC.md)
- **Crate**: `airsspec-runtime`

## Steps

1. Create `src/orchestrator.rs`
2. Define `Orchestrator` struct with:
   - `workspace_path: PathBuf`
   - `state_persistence: Box<dyn StatePersistence>`
   - `compliance_gate: Box<dyn ComplianceGate>`
3. Implement skeleton methods:
   - `new() -> Self`
   - `load_uow(&self, id: &str) -> Result<UowState, StateError>`
   - `transition(&self, uow_id: &str, to: Phase) -> Result<(), StateError>`
4. Wire dependencies from `airsspec-core`

## Verification

- [ ] `cargo build -p airsspec-runtime` passes
