# TASK-001: Remove Type Re-exports from lib.rs

**Plan Reference**: [../plans/PLAN-001.md](../plans/PLAN-001.md)

## Execution Output

### Actions Taken
1. Removed all `#[doc(inline)] pub use` type re-exports from `crates/airsspec-core/src/lib.rs` (lines 59-82)
2. Removed the justification comment section (lines 59-66) for the removed re-exports
3. Updated doctest imports in `crates/airsspec-core/src/state/types.rs` to use full module paths:
   - `use airsspec_core::Phase;` → `use airsspec_core::state::Phase;`
   - `use airsspec_core::{Phase, UowState};` → `use airsspec_core::state::{Phase, UowState};`
   - `use airsspec_core::{Phase, Transition};` → `use airsspec_core::state::{Phase, Transition};`

### Files Modified
- `crates/airsspec-core/src/lib.rs` (removed re-exports)
- `crates/airsspec-core/src/state/types.rs` (updated doctest imports)

### Verification
- [x] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [x] `cargo test` passes (all 53 unit tests + 28 doctests)
- [x] No `#[doc(inline)] pub use` remains in lib.rs
