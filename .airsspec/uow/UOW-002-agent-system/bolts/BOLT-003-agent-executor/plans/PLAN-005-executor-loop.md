# PLAN-005: Executor Event Loop

## Objective

Implement the event-driven execution loop that orchestrates reasoning patterns, LLM calls, and tool execution.

## Context

The executor ties everything together: session management, pattern execution, tool dispatch, and parallel execution support.

## Steps

1. Implement `AgentExecutor::execute()` method:
   - Initialize execution context
   - Emit `SessionStarted` event
   - Loop: call pattern.next_step()
   - Match on `ReasoningStep`:
     - `Thought` → log and continue
     - `Action` → execute tool, add observation
     - `ParallelActions` → `futures::join_all()`
     - `FinalAnswer` → return result
   - Check pattern.should_continue()
   - Emit `SessionCompleted` event
2. Implement `AgentExecutor` trait from `airsspec-core`
3. Add parallel tool execution with `futures::join_all()`
4. Add error handling and recovery
5. Implement event emission (can be simple logging for now)
6. Write integration tests:
   - Full execution with mocked pattern and tools
   - Parallel tool execution
   - Error scenarios

## Expected Output

### Files Modified:
- `crates/airsspec-agents/src/executor.rs` — Complete implementation

### Files Created:
- `crates/airsspec-agents/tests/executor_tests.rs`

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes
- [ ] `cargo test` passes all tests
- [ ] Event loop executes correctly
- [ ] Parallel actions work
- [ ] Error handling is robust
- [ ] Integration test demonstrates full flow

## References

- ADR: [../../adrs/ADR-003-agent-architecture.md](../../adrs/ADR-003-agent-architecture.md)
- RFC: [../../RFC.md](../../RFC.md)
