# PLAN-001: Artifact Types

## Objective

Define artifact types and validation result structures in `src/artifact/types.rs`.

## Context

- **ADR Reference**: [ADR-002-contract.md](../../../adrs/ADR-002-contract.md)
- **Crate**: `airsspec-core`
- **Directory**: `src/artifact/`

## Steps

1. Create `src/artifact/` directory
2. Create `src/artifact/mod.rs` with module exports
3. Create `src/artifact/types.rs` with:
   - `ArtifactType` enum: `Requirements`, `Daa`, `Adr`, `Rfc`, `BoltPlan`
   - `ValidationResult` struct: `valid`, `errors`, `warnings`
   - `ValidationError` struct: `field`, `message`
   - `ArtifactRef` struct: `path`, `artifact_type`, `status`
4. Derive appropriate traits for serialization
5. Update `lib.rs` to export artifact module

## Verification

- [ ] `cargo build -p airsspec-core` passes
- [ ] All types derive required traits
