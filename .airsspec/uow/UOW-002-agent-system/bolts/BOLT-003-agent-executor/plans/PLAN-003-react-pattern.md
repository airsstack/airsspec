# PLAN-003: ReAct Pattern Implementation

## Objective

Implement the ReAct (Reasoning + Acting) pattern that enables thought → action → observation loops.

## Context

ReAct is the primary reasoning pattern. Agent thinks, acts with tools, observes results, and repeats until reaching final answer.

## Steps

1. Create `src/patterns/react.rs`
2. Implement `ReactPattern` struct with:
   - `max_iterations` limit
   - Prompt template for ReAct format
3. Implement `ReasoningPattern` trait:
   - `name()` returns "react"
   - `next_step()` calls LLM and parses response
   - `should_continue()` checks iteration limit
   - `format_prompt()` builds ReAct-style prompt
4. Add response parsing logic to extract:
   - "Thought: ..." → `ReasoningStep::Thought`
   - "Action: tool\nAction Input: {json}" → `ReasoningStep::Action`
   - "Final Answer: ..." → `ReasoningStep::FinalAnswer`
5. Write unit tests with mocked LLM
6. Add example ReAct prompt template

## Expected Output

### Files Created:
- `crates/airsspec-agents/src/patterns/react.rs`

### Files Modified:
- `crates/airsspec-agents/src/patterns/mod.rs` — Export ReactPattern

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] ReAct format is correctly parsed
- [ ] Iteration limit works
- [ ] Prompt includes available tools

## References

- ADR: [../../adrs/ADR-003-agent-architecture.md](../../adrs/ADR-003-agent-architecture.md)
- Research: [../../../../researches/pattern-react.md](../../../../researches/pattern-react.md)
- RFC: [../../RFC.md](../../RFC.md)
