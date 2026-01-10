# PLAN-003: State Traits

## Objective

Define state persistence and compliance gate traits in `src/state/traits.rs`.

## Context

- **ADR Reference**: [ADR-001-primitives.md](../../../adrs/ADR-001-primitives.md)
- **Crate**: `airsspec-core`
- **File**: `src/state/traits.rs`

## Steps

1. Create `src/state/traits.rs`
2. Define `StatePersistence` trait with async methods:
   - `load(&self, uow_id: &str) -> Result<UowState, StateError>`
   - `save(&self, state: &UowState) -> Result<(), StateError>`
   - `record_transition(&self, uow_id: &str, transition: Transition) -> Result<(), StateError>`
3. Define `ComplianceGate` trait with methods:
   - `can_transition(&self, from: Phase, to: Phase, artifacts: &[ArtifactRef]) -> bool`
   - `required_artifacts(&self, phase: Phase) -> Vec<ArtifactType>`
   - `validate_gate(&self, state: &UowState) -> Result<(), StateError>`
4. Add `Send + Sync` bounds to all traits
5. Use `async_trait` macro for async traits
6. Update `src/state/mod.rs` to export traits

## Verification

- [ ] `cargo build -p airsspec-core` passes
- [ ] Traits have `Send + Sync` bounds
- [ ] Doc comments on all public traits and methods
