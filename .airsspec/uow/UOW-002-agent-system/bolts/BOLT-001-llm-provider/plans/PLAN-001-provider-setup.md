# PLAN-001: LLM Provider Setup

## Objective

Set up `airsspec-llm` crate with OpenRouter provider implementation via Rig library.

## Context

From ADR-001, we need to integrate with OpenRouter using the Rig library (v0.28.0+) to provide LLM capabilities for agent reasoning. This provides access to 200+ models via a single unified API.

## Steps

1. Create `crates/airsspec-llm/` directory structure
2. Add `Cargo.toml` with dependencies:
   - `airsspec-core` (workspace dependency)
   - `rig = "0.28.0"`
   - `tokio`, `tokio-stream`, `serde`, `anyhow`, `tracing`
3. Create module structure:
   - `src/lib.rs` — Public API exports
   - `src/provider.rs` — OpenRouterProvider struct
   - `src/config.rs` — Configuration loading
   - `src/error.rs` — Error types
4. Implement `OpenRouterProvider` struct wrapping Rig's `openrouter::Client`
5. Implement `LLMProvider` trait from `airsspec-core`
6. Add configuration loading from environment variables:
   - `OPENROUTER_API_KEY` (required)
   - `AIRSSPEC_DEFAULT_MODEL` (optional, default: `anthropic/claude-sonnet-4`)
   - `AIRSSPEC_LLM_TIMEOUT` (optional, default: 120 seconds)

## Expected Output

### Files Created:
- `crates/airsspec-llm/Cargo.toml`
- `crates/airsspec-llm/src/lib.rs`
- `crates/airsspec-llm/src/provider.rs`
- `crates/airsspec-llm/src/config.rs`
- `crates/airsspec-llm/src/error.rs`

### Files Modified:
- `Cargo.toml` (workspace root) — Add `airsspec-llm` member

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes with zero warnings
- [ ] Crate structure matches expected layout
- [ ] `OpenRouterProvider` struct compiles
- [ ] `LLMProvider` trait is implemented (may not compile fully without completion method)
- [ ] Configuration types are defined

## References

- ADR: [../../adrs/ADR-001-llm-integration.md](../../adrs/ADR-001-llm-integration.md)
- RFC: [../../RFC.md](../../RFC.md)
- DAA: [../../DAA.md](../../DAA.md) — Language Model Communication context
