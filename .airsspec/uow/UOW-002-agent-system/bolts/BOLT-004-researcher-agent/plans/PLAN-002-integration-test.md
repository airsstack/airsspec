# PLAN-002: End-to-End Integration

## Objective

Create integration test and example demonstrating Researcher agent working end-to-end.

## Context

Validate the entire system: OpenRouter LLM + Tools + Executor + ReAct Pattern + Researcher Agent producing real artifacts.

## Steps

1. Create test fixtures in `tests/fixtures/`:
   - `user_story.md` — Sample user story
   - `tech_notes.md` — Sample technical constraints
   - `expected_requirements.md` — Expected output format
2. Create `tests/researcher_integration.rs`:
   - Setup LLM provider (real or mocked)
   - Setup tool registry with sandbox
   - Create Researcher agent
   - Execute research on fixtures
   - Verify output structure
   - Check artifact file written
3. Create `examples/research_example.rs`:
   - Complete working example
   - Uses environment variables for API key
   - Demonstrates typical usage
   - Shows output
4. Add README section explaining how to run example
5. Test with real OpenRouter API (marked `#[ignore]`)

## Expected Output

### Files Created:
- `crates/airsspec-agents/tests/fixtures/user_story.md`
- `crates/airsspec-agents/tests/fixtures/tech_notes.md`
- `crates/airsspec-agents/tests/fixtures/expected_requirements.md`
- `crates/airsspec-agents/tests/researcher_integration.rs`
- `crates/airsspec-agents/examples/research_example.rs`

### Files Modified:
- `crates/airsspec-agents/README.md` — Usage documentation

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes (with mocked LLM)
- [ ] Integration test demonstrates full flow
- [ ] Example runs with real API
- [ ] Produces valid requirements.md
- [ ] Documentation is clear

## References

- Requirements: [../../requirements.md](../../requirements.md) — Success criteria #4
- RFC: [../../RFC.md](../../RFC.md)
- ROADMAP: [../../../../ROADMAP.md](../../../../ROADMAP.md) — UOW-002 completion
