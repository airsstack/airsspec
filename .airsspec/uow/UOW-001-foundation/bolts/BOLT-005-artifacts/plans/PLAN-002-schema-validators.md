# PLAN-002: Schema Validators

## Objective

Implement JSON Schema validators for artifacts in `airsspec-artifacts` crate.

## Context

- **RFC Reference**: [RFC.md](../../../../RFC.md)
- **Crate**: `airsspec-artifacts`

## Steps

1. Define JSON Schema files for each artifact type in `schemas/`
2. Implement `ArtifactValidator` trait in `src/validators.rs`
3. Create validators for:
   - `RequirementsValidator`
   - `DaaValidator`
   - `AdrValidator`
   - `RfcValidator`
   - `BoltPlanValidator`
4. Use `jsonschema` crate for validation
5. Return detailed validation errors

## Verification

- [ ] `cargo build -p airsspec-artifacts` passes
- [ ] Unit tests for valid/invalid artifacts
