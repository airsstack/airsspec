# PLAN-003: Streaming Support

## Objective

Implement streaming response handling for real-time LLM output.

## Context

For responsive UX, agents need to stream LLM responses as they arrive rather than waiting for complete responses. Rig provides native streaming support via `tokio-stream`.

## Steps

1. Create `src/streaming.rs` module
2. Define `StreamChunk` type for delta content
3. Implement `stream()` method in `OpenRouterProvider`:
   - Call Rig's streaming API
   - Return `CompletionStream` (wraps `tokio_stream::Stream`)
   - Handle partial chunks
4. Add streaming error handling:
   - Connection interruptions
   - Malformed chunks
   - Timeout during stream
5. Write integration test:
   - Test with real OpenRouter API (marked `#[ignore]` by default)
   - Verify streaming works end-to-end
   - Collect all chunks and validate complete response

## Expected Output

### Files Created:
- `crates/airsspec-llm/src/streaming.rs`

### Files Modified:
- `crates/airsspec-llm/src/provider.rs` — Add `stream()` method
- `crates/airsspec-llm/src/lib.rs` — Export streaming types

### Files Created:
- `crates/airsspec-llm/tests/integration.rs` — Integration test

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes with zero warnings
- [ ] `cargo test` passes all tests
- [ ] Integration test works with real API (when run with `cargo test -- --ignored`)
- [ ] Streaming produces correct output
- [ ] Stream error handling is robust
- [ ] Documentation is complete

## References

- ADR: [../../adrs/ADR-001-llm-integration.md](../../adrs/ADR-001-llm-integration.md)
- RFC: [../../RFC.md](../../RFC.md)
- Tokio Stream Docs: https://docs.rs/tokio-stream/latest/tokio_stream/
