# PLAN-004: Chain-of-Thought Pattern

## Objective

Implement the Chain-of-Thought (CoT) pattern for step-by-step reasoning without tool execution.

## Context

CoT is useful for pure reasoning tasks that don't require tool use. Agent generates reasoning steps and produces final answer.

## Steps

1. Create `src/patterns/cot.rs`
2. Implement `ChainOfThoughtPattern` struct
3. Implement `ReasoningPattern` trait:
   - `name()` returns "cot"
   - `next_step()` requests reasoning step from LLM
   - `should_continue()` checks if final answer reached
   - `format_prompt()` includes "Let's think step by step"
4. Add logic to detect final answer pattern
5. Write unit tests with mocked LLM
6. Document when to use CoT vs ReAct

## Expected Output

### Files Created:
- `crates/airsspec-agents/src/patterns/cot.rs`

### Files Modified:
- `crates/airsspec-agents/src/patterns/mod.rs` — Export ChainOfThoughtPattern

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] CoT produces reasoning chain
- [ ] Final answer detection works
- [ ] No tool execution occurs

## References

- ADR: [../../adrs/ADR-003-agent-architecture.md](../../adrs/ADR-003-agent-architecture.md)
- Research: [../../../../researches/pattern-cot.md](../../../../researches/pattern-cot.md)
- RFC: [../../RFC.md](../../RFC.md)
