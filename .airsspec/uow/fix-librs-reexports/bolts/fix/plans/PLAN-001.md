# PLAN-001: Remove Type Re-exports from lib.rs

## Objective

Remove all `#[doc(inline)] pub use` type re-exports from `crates/airsspec-core/src/lib.rs` to comply with `.aiassisted/guidelines/rust/project-standard.md` §4.3.

## Root Cause

The file `crates/airsspec-core/src/lib.rs` contains `#[doc(inline)] pub use` type re-exports on lines 67-82. The project standard §4.3 explicitly forbids:

1. Type re-exports in lib.rs
2. `#[doc(inline)]` re-exports (forbidden in BOTH mod.rs and lib.rs)

The guideline states:
```rust
// ❌ ALSO FORBIDDEN - doc(inline) re-exports
#[doc(inline)]
pub use config::OSLConfig;  // ❌ Don't use doc(inline) re-exports
```

## Solution

1. Remove all `#[doc(inline)] pub use` type re-exports (lines 59-82)
2. Keep only the module declarations (lines 42-57)
3. Remove the comment section (lines 59-66) that justified the removed re-exports

## Expected Result

The lib.rs file should contain only module declarations:

```rust
// Sub-phase 1.1: Primitives
pub mod error;
pub mod state;

// Sub-phase 1.2: Contract Layer
pub mod artifact;
pub mod tool;

// Sub-phase 1.3: Cognition Layer
pub mod llm;
// pub mod memory;
// pub mod knowledge;

// Sub-phase 1.4: Agent Layer
// pub mod agent;
// pub mod plugin;
```

## Steps

1. Read the current lib.rs file to understand the context
2. Remove lines 59-82 (the re-export section and its justification comment)
3. Ensure module declarations remain intact
4. Verify no `#[doc(inline)] pub use` remains in the file

## Verification

- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] No `#[doc(inline)] pub use` remains in lib.rs

## Files to Modify

- `crates/airsspec-core/src/lib.rs`
