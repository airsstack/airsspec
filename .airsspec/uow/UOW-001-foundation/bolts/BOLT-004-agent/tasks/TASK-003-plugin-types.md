# TASK-003: Plugin Types

**Plan Reference**: [../plans/PLAN-003-plugin-types.md](../plans/PLAN-003-plugin-types.md)

## Status

- [x] Completed

## Execution Output

### Actions Taken

1. Created `crates/airsspec-core/src/plugin/` directory
2. Created `crates/airsspec-core/src/plugin/mod.rs` with module exports
3. Created `crates/airsspec-core/src/plugin/types.rs` with the following types:
   - `PluginManifest` struct with fields: `name` (String), `version` (String), `description` (Option<String>), `overlays` (Vec<OverlaySpec>)
   - `OverlaySpec` struct with fields: `target` (AgentId), `path` (PathBuf)
   - `ResolutionOrder` enum with variants: `CoreFirst`, `PluginFirst`, `UserFirst`
4. Derived `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Serialize`, `Deserialize` for all types
5. Added `#[serde(rename_all = "snake_case")]` to `ResolutionOrder` for consistent serialization

### Verification

- [x] `cargo build -p airsspec-core` passes
- [x] All unit tests pass (including serialization tests with snake_case)
- [x] Types implement required derive traits
- [x] Module exports are correctly configured
- [x] `ResolutionOrder::as_str()` returns values matching serde serialization

### Files Created

- `crates/airsspec-core/src/plugin/mod.rs` (new)
- `crates/airsspec-core/src/plugin/types.rs` (new)

### Notes

- `ResolutionOrder` implements `Copy` trait for efficient passing
- `ResolutionOrder::try_from_str()` provides string-to-enum conversion
- `ResolutionOrder::as_str()` returns snake_case values matching serde serialization
- `PluginManifest` structure supports multiple overlays per plugin
- All types support serde serialization for plugin discovery and configuration

### Review Feedback Fixes (2025-01-12)

Fixed the following issues identified by the airsspec-reviewer:

1. **Clippy Warning - Should implement trait**: Renamed `ResolutionOrder::from_str()` to `ResolutionOrder::try_from_str()` to avoid confusion with `std::str::FromStr::from_str()`
2. **Clippy Warning - Doc markdown**: Added backticks around "AirsSpec" in documentation
3. **Clippy Warning - Expect used**: Added `#[allow(clippy::expect_used)]` to serialization tests where expect() is intentional
4. **Clippy Warning - Redundant clone**: Added `#[allow(clippy::redundant_clone)]` to clone tests where clones are intentional (testing Clone trait)

### Standard Compliance Fixes (2025-01-12)

Fixed the following CRITICAL violation of `.aiassisted/guidelines/rust/project-standard.md`:

1. **§4.3 Re-Export Policy Violation (CRITICAL)**: Removed type re-exports from `plugin/mod.rs`
   - Changed from: `pub use traits::{PluginLoader, PromptOverlay}`
   - Changed from: `pub use types::{OverlaySpec, PluginManifest, ResolutionOrder}`
   - Changed to: Module declarations only (`pub mod traits; pub mod types;`)
   - Rationale: Callers must use explicit imports like `use airsspec_core::plugin::traits::PluginLoader;`
