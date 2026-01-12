# TASK-004: Plugin Traits

**Plan Reference**: [../plans/PLAN-004-plugin-traits.md](../plans/PLAN-004-plugin-traits.md)

## Status

- [x] Completed

## Execution Output

### Actions Taken

1. Created `crates/airsspec-core/src/plugin/traits.rs` with the following definitions:
   - `PluginLoader` trait with `async_trait` and `Send + Sync` bounds:
     - `async fn load(&self, plugin_dir: &Path) -> Result<PluginManifest, PluginError>`
     - `async fn list_plugins(&self, workspace: &Path) -> Result<Vec<PluginManifest>, PluginError>`
   - `PromptOverlay` trait with `Send + Sync` bounds:
     - `fn stack_prompts(&self, agent_id: &AgentId, order: ResolutionOrder) -> String`
2. Updated `crates/airsspec-core/src/plugin/mod.rs` to re-export all public types and traits

### Verification

- [x] `cargo build -p airsspec-core` passes
- [x] All unit tests pass (including async trait tests)
- [x] `async_trait` is used for all async traits
- [x] `Send + Sync` bounds are applied to all traits
- [x] Mock implementations demonstrate trait usage
- [x] Module re-exports are correctly configured

### Files Created

- `crates/airsspec-core/src/plugin/traits.rs` (new)
- Updated `crates/airsspec-core/src/plugin/mod.rs` (added re-exports)

### Notes

- `PluginLoader` trait enables plugin discovery from filesystem
- `PromptOverlay` trait supports flexible prompt stacking with resolution order
- Mock implementations provided for testing trait semantics
- All traits use `Send + Sync` bounds for thread-safe usage
- Async methods use `async_trait::async_trait` for trait object support

### Review Feedback Fixes (2025-01-12)

Fixed the following issues identified by the airsspec-reviewer:

1. **Doc Test Failure - Missing import**: Fixed PluginLoader trait doc test by adding `use airsspec_core::error::PluginError;` import
2. **Clippy Warning - Doc markdown**: Added backticks around "AirsSpec" in documentation
3. **Clippy Warning - Expect used**: Added `#[allow(clippy::expect_used)]` to async test functions where expect() is intentional

### Standard Compliance Fixes (2025-01-12)

Fixed the following CRITICAL violation of `.aiassisted/guidelines/rust/project-standard.md`:

1. **§4.3 Re-Export Policy Violation (CRITICAL)**: Removed type re-exports from `plugin/mod.rs`
   - Changed from: `pub use traits::{PluginLoader, PromptOverlay}`
   - Changed from: `pub use types::{OverlaySpec, PluginManifest, ResolutionOrder}`
   - Changed to: Module declarations only (`pub mod traits; pub mod types;`)
   - Rationale: Callers must use explicit imports like `use airsspec_core::plugin::traits::PluginLoader;`
