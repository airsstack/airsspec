# TASK-002: Agent Traits

**Plan Reference**: [../plans/PLAN-002-agent-traits.md](../plans/PLAN-002-agent-traits.md)

## Status

- [x] Completed

## Execution Output

### Actions Taken

1. Created `crates/airsspec-core/src/agent/traits.rs` with the following definitions:
   - `Agent` trait with `async_trait` and `Send + Sync` bounds:
     - `fn id(&self) -> &AgentId`
     - `fn config(&self) -> &AgentConfig`
     - `async fn execute(&self, context: AgentContext) -> Result<AgentOutput, AgentError>`
   - `AgentContext` struct with fields: `uow_id` (String), `phase` (Phase), `memory` (Vec<MemoryFragment>), `artifacts` (Vec<PathBuf>)
   - `AgentOutput` struct with fields: `result` (String), `artifacts_created` (Vec<PathBuf>), `delegation` (Option<DelegationSignal>), `token_usage` (TokenUsage)
   - `TokenUsage` struct with fields: `prompt_tokens` (u32), `completion_tokens` (u32), `total_tokens` (u32)
   - `AgentExecutor` trait with `async_trait` and `Send + Sync` bounds:
     - `async fn run(&self, agent: &dyn Agent, budget: Budget) -> Result<ExecutionResult, ExecutionError>`
   - `ExecutionResult` struct with fields: `output` (AgentOutput), `iterations` (u32), `total_tokens` (u32)
   - `ExecutionError` enum (using `thiserror::Error`)
2. Updated `crates/airsspec-core/src/agent/mod.rs` to re-export all public types and traits

### Verification

- [x] `cargo build -p airsspec-core` passes
- [x] All unit tests pass (including error conversion tests)
- [x] `async_trait` is used for all async traits
- [x] `Send + Sync` bounds are applied to all traits
- [x] `TokenUsage` implements `Add` trait for accumulation
- [x] Module re-exports are correctly configured

### Files Created

- `crates/airsspec-core/src/agent/traits.rs` (new)
- Updated `crates/airsspec-core/src/agent/mod.rs` (added re-exports)

### Notes

- `TokenUsage` implements `std::ops::Add` for convenient token accumulation
- `ExecutionError` implements `From<AgentError>` for automatic conversion
- All traits use `async_trait::async_trait` for async method support
- `AgentContext` imports `Phase` from the `state` module
- `AgentOutput` imports `MemoryFragment` from the `memory` module

### Review Feedback Fixes (2025-01-12)

Fixed the following issues identified by the airsspec-reviewer:

1. **Doc Test Failure - Missing import**: Fixed Agent trait doc test by adding `use airsspec_core::error::AgentError;` import
2. **Doc Test Failure - Wrong imports**: Fixed AgentExecutor trait doc test by correcting imports:
   - Changed `Budget` import to `use airsspec_core::agent::types::Budget;`
   - Added `ExecutionResult` and `ExecutionError` imports
3. **Clippy Warning - Missing const fn**: Changed `TokenUsage::new()` and `TokenUsage::add()` to `pub const fn`
4. **Clippy Warning - Use Self**: Changed `TokenUsage` to `Self` in impl blocks (line 136 and 149)
5. **Clippy Warning - Manual Default impl**: Replaced manual `impl Default for TokenUsage` with `#[derive(Default)]` macro
6. **Clippy Warning - Doc markdown**: Added backticks around "AirsSpec" in documentation
7. **Clippy Warning - Redundant clone**: Added `#[allow(clippy::redundant_clone)]` to clone tests where clones are intentional

### Standard Compliance Fixes (2025-01-12)

Fixed the following CRITICAL and MANDATORY violations of `.aiassisted/guidelines/rust/project-standard.md`:

1. **§4.3 Re-Export Policy Violation (CRITICAL)**: Removed type re-exports from `agent/mod.rs`
   - Changed from: `pub use traits::{Agent, AgentContext, AgentExecutor, ...}`
   - Changed from: `pub use types::{AgentConfig, AgentId, Budget, DelegationSignal}`
   - Changed to: Module declarations only (`pub mod traits; pub mod types;`)
   - Rationale: Callers must use explicit imports like `use airsspec_core::agent::traits::Agent;`

2. **§2.2 No FQN in Type Annotations (MANDATORY)**: Replaced fully qualified name in `AgentExecutor::run()` signature
   - Changed from: `budget: super::types::Budget`
   - Changed to: `budget: Budget` with import added to Layer 3
   - Added `Budget` to import: `use super::types::{AgentConfig, AgentId, Budget, DelegationSignal};`

3. **§2.1 3-Layer Import Organization (MANDATORY)**: Fixed import grouping in `agent/traits.rs`
   - Removed empty line between Layer 3 imports
   - All internal module imports now grouped together without separation
