# PLAN-002: Artifact Traits

## Objective

Define artifact validator and store traits in `src/artifact/traits.rs`.

## Context

- **ADR Reference**: [ADR-002-contract.md](../../../adrs/ADR-002-contract.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/artifact/traits.rs`
2. Define `ArtifactValidator` trait:
   - `fn artifact_type(&self) -> ArtifactType`
   - `async fn validate(&self, content: &str) -> ValidationResult`
   - `async fn validate_file(&self, path: &Path) -> Result<ValidationResult, ArtifactError>`
3. Define `ArtifactStore` trait:
   - `async fn read(&self, path: &Path) -> Result<String, ArtifactError>`
   - `async fn write(&self, path: &Path, content: &str) -> Result<(), ArtifactError>`
   - `async fn exists(&self, path: &Path) -> bool`
4. Add `Send + Sync` bounds
5. Update module exports

## Verification

- [ ] `cargo build -p airsspec-core` passes
- [ ] Traits have `Send + Sync` bounds
