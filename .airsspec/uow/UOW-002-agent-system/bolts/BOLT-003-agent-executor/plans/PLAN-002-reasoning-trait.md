# PLAN-002: Reasoning Pattern Trait

## Objective

Define the `ReasoningPattern` trait abstraction that allows pluggable reasoning strategies.

## Context

From ADR-003, reasoning patterns (ReAct, CoT, etc.) should be abstracted behind a trait. This enables different agents to use different reasoning strategies.

## Steps

1. Create `src/patterns/mod.rs` and `src/patterns/traits.rs`
2. Define `ReasoningPattern` trait with methods:
   - `name() -> &str`
   - `next_step(context) -> Result<ReasoningStep>`
   - `should_continue(context) -> bool`
   - `format_prompt(query, history) -> String`
3. Define `ReasoningStep` enum:
   - `Thought(String)`
   - `Action { tool, args }`
   - `ParallelActions { actions }`
   - `FinalAnswer(String)`
4. Define `ExecutionContext` struct with:
   - LLM provider reference
   - Tool registry reference
   - Query string
   - Execution history
5. Define `StepRecord` for tracking execution

## Expected Output

### Files Created:
- `crates/airsspec-agents/src/patterns/mod.rs`
- `crates/airsspec-agents/src/patterns/traits.rs`

### Files Modified:
- `crates/airsspec-agents/src/lib.rs` — Export pattern types

## Verification

- [ ] `cargo build` succeeds
- [ ] Trait compiles with async methods
- [ ] Types are properly exported
- [ ] Documentation explains pattern contract

## References

- ADR: [../../adrs/ADR-003-agent-architecture.md](../../adrs/ADR-003-agent-architecture.md)
- RFC: [../../RFC.md](../../RFC.md)
