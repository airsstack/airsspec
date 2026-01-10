# PLAN-002: State Types

## Objective

Define state types for UOW lifecycle in `src/state/types.rs`.

## Context

- **ADR Reference**: [ADR-001-primitives.md](../../../adrs/ADR-001-primitives.md)
- **Crate**: `airsspec-core`
- **Directory**: `src/state/`

## Steps

1. Create `src/state/` directory in `crates/airsspec-core/`
2. Create `src/state/mod.rs` with module exports
3. Create `src/state/types.rs` with:
   - `Phase` enum: `Idle`, `Research`, `Inception`, `Design`, `Planning`, `Construction`
   - `UowState` struct: `id`, `phase`, `created_at`, `updated_at`
   - `Transition` struct: `from`, `to`, `at`, `reason`
4. Derive `Serialize`, `Deserialize`, `Clone`, `Debug` for all types
5. Add `PartialEq`, `Eq`, `Copy` for `Phase` enum
6. Update `lib.rs` to export state module

## Verification

- [ ] `cargo build -p airsspec-core` passes
- [ ] All types derive required traits
- [ ] Doc comments on all public types
