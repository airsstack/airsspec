# PLAN-004: Plugin Traits

## Objective

Define plugin loader and overlay traits in `src/plugin/traits.rs`.

## Context

- **ADR Reference**: [ADR-004-agent.md](../../../adrs/ADR-004-agent.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/plugin/traits.rs`
2. Define `PluginLoader` trait:
   - `async fn load(&self, path: &Path) -> Result<PluginManifest, PluginError>`
   - `fn list_plugins(&self) -> Vec<&PluginManifest>`
3. Define `PromptOverlay` trait:
   - `fn agent_id(&self) -> &AgentId`
   - `fn get_overlay(&self) -> &str`
   - `fn resolution_order(&self) -> ResolutionOrder`
4. Add `Send + Sync` bounds

## Verification

- [ ] `cargo build -p airsspec-core` passes
