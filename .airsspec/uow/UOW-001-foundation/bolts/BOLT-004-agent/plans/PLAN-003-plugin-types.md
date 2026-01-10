# PLAN-003: Plugin Types

## Objective

Define plugin types in `src/plugin/types.rs`.

## Context

- **ADR Reference**: [ADR-004-agent.md](../../../adrs/ADR-004-agent.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/plugin/` directory
2. Create `src/plugin/mod.rs` with module exports
3. Create `src/plugin/types.rs` with:
   - `PluginManifest` struct: `name`, `version`, `overlays`
   - `OverlaySpec` struct: `target_agent`, `path`, `order`
   - `ResolutionOrder` enum: `Before`, `After`, `Replace`
4. Derive serialization traits

## Verification

- [ ] `cargo build -p airsspec-core` passes
