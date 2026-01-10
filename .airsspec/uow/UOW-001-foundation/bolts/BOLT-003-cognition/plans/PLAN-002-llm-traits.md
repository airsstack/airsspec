# PLAN-002: LLM Traits

## Objective

Define LLM provider and stream handler traits in `src/llm/traits.rs`.

## Context

- **ADR Reference**: [ADR-003-cognition.md](../../../adrs/ADR-003-cognition.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/llm/traits.rs`
2. Define `LLMProvider` trait:
   - `async fn complete(&self, request: CompletionRequest) -> Result<String, LlmError>`
   - `async fn complete_with_usage(&self, request: CompletionRequest) -> Result<(String, TokenUsage), LlmError>`
3. Define `StreamHandler` trait:
   - `async fn on_token(&mut self, token: &str)`
   - `async fn on_complete(&mut self)`
   - `async fn on_error(&mut self, error: &LlmError)`
4. Add `Send + Sync` bounds
5. Update module exports

## Verification

- [ ] `cargo build -p airsspec-core` passes
