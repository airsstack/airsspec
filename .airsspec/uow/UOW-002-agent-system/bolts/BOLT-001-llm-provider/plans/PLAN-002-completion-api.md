# PLAN-002: Completion API Implementation

## Objective

Implement the completion methods for sending prompts to OpenRouter and receiving responses.

## Context

With the provider structure in place (PLAN-001), now implement the actual LLM communication logic. This includes request building, response handling, and error management.

## Steps

1. Implement `complete()` method in `OpenRouterProvider`:
   - Build `CompletionRequest` from parameters
   - Call Rig's OpenRouter client
   - Parse response into `CompletionResponse`
   - Extract token usage metadata
2. Add retry logic with exponential backoff for rate limits
3. Implement timeout enforcement (from config)
4. Add proper error handling:
   - API key invalid
   - Rate limit exceeded
   - Timeout errors
   - Network errors
5. Add tracing/logging for observability
6. Write unit tests for:
   - Request building
   - Error handling
   - Configuration validation

## Expected Output

### Files Modified:
- `crates/airsspec-llm/src/provider.rs` — Complete implementation
- `crates/airsspec-llm/src/error.rs` — Error variants

### Files Created:
- `crates/airsspec-llm/src/lib.rs` (tests module)

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes with zero warnings
- [ ] `cargo test` passes all unit tests
- [ ] Error handling covers all known failure modes
- [ ] Retry logic implements exponential backoff
- [ ] Timeout is enforced correctly
- [ ] Logging provides useful debugging information

## References

- ADR: [../../adrs/ADR-001-llm-integration.md](../../adrs/ADR-001-llm-integration.md)
- RFC: [../../RFC.md](../../RFC.md)
- Rig Docs: https://docs.rs/rig/latest/rig/
