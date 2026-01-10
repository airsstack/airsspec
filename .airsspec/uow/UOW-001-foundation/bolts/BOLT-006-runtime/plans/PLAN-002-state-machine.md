# PLAN-002: State Machine

## Objective

Implement phase transition state machine in `airsspec-runtime` crate.

## Context

- **RFC Reference**: [RFC.md](../../../../RFC.md)
- **Crate**: `airsspec-runtime`

## Steps

1. Create `src/state_machine.rs`
2. Implement `ComplianceGate` trait as `DefaultComplianceGate`:
   - `can_transition()` — Validate phase transitions
   - `required_artifacts()` — Return artifacts needed for each phase
   - `validate_gate()` — Check all conditions
3. Define valid phase transitions:
   - `Idle` → `Research`
   - `Research` → `Inception` (requires: requirements.md)
   - `Inception` → `Design` (requires: DAA.md)
   - `Design` → `Planning` (requires: ADR-*.md)
   - `Planning` → `Construction` (requires: RFC.md, bolts/)
4. Implement `StatePersistence` as `FileStatePersistence`

## Verification

- [ ] `cargo build -p airsspec-runtime` passes
- [ ] Unit tests for valid/invalid transitions
