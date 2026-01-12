---
version: "2.0"
status: proposed
author: architect
created_at: 2026-01-12
supersedes: ADR-003-v1 (single-loop model)
---

# ADR-003: Agent Execution Architecture

## Status

Proposed (v2 — supersedes original)

## Context

From DAA bounded context "Agent Orchestration":

- Agents are autonomous actors with identity and configuration
- Sessions represent single execution runs
- Need to coordinate LLM calls and tool invocations in parallel
- Must implement `Agent` and `AgentExecutor` traits from `airsspec-core`

**User feedback on v1**:
1. Event-driven execution preferred over simple loop
2. Parallel tool execution is important
3. Reasoning patterns (ReAct, CoT) should be abstracted

## Decision

Implement an **event-driven execution model** with **pluggable reasoning patterns** and **parallel tool execution**.

### 1. Event-Driven Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Event Bus                           │
├─────────────────────────────────────────────────────────┤
│  ┌───────────┐   ┌─────────────┐   ┌───────────────┐   │
│  │ Executor  │──▷│ Reasoning   │──▷│ Tool          │   │
│  │           │◁──│ Pattern     │◁──│ Executor      │   │
│  └───────────┘   └─────────────┘   └───────────────┘   │
│       │                │                   │            │
│       ▽                ▽                   ▽            │
│  [SessionEvent]  [ReasoningEvent]    [ToolEvent]        │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 2. Reasoning Pattern Trait

Abstraction allowing multiple reasoning strategies:

```rust
pub trait ReasoningPattern: Send + Sync {
    fn name(&self) -> &str;
    
    async fn next_step(&self, context: &ExecutionContext) 
        -> Result<ReasoningStep, PatternError>;
    
    fn should_continue(&self, context: &ExecutionContext) -> bool;
    
    fn format_prompt(&self, query: &str) -> String;
}
```

### 3. Available Patterns

| Pattern | Implementation | Behavior |
|---------|---------------|----------|
| **ReAct** | `ReactPattern` | Thought → Action → Observation loop |
| **CoT** | `ChainOfThoughtPattern` | Step-by-step reasoning, no actions |
| **ToT** | `TreeOfThoughtPattern` | Branching exploration (future) |
| **Hybrid** | `HybridPattern` | Combines patterns (future) |

### 4. Parallel Tool Execution

When reasoning pattern requests multiple independent actions:

```rust
pub enum ReasoningStep {
    Thought(String),
    Action { tool: String, args: ToolInput },
    ParallelActions { actions: Vec<Action> },  // NEW
    FinalAnswer(String),
}

// Executor handles parallel actions via join
let results = futures::future::join_all(
    actions.iter().map(|a| tool_registry.execute(a))
).await;
```

### 5. Event Types

| Event | Direction | Purpose |
|-------|-----------|---------|
| `SessionStarted` | Executor → Bus | Session begins |
| `StepRequested` | Executor → Pattern | Request next reasoning step |
| `ThoughtGenerated` | Pattern → Executor | Internal reasoning recorded |
| `ActionRequested` | Pattern → ToolExecutor | Tool(s) needed |
| `ActionCompleted` | ToolExecutor → Pattern | Tool result(s) returned |
| `AnswerReady` | Pattern → Executor | Final answer produced |
| `SessionCompleted` | Executor → Bus | Session ends |

### 6. Module Structure

```
airsspec-agents/
├── src/
│   ├── lib.rs
│   ├── executor.rs       # Event-driven executor
│   ├── events.rs         # Event types
│   ├── session.rs        # Session management
│   ├── patterns/         # Reasoning patterns
│   │   ├── mod.rs
│   │   ├── traits.rs     # ReasoningPattern trait
│   │   ├── react.rs      # ReAct implementation
│   │   └── cot.rs        # CoT implementation
│   └── agents/
│       ├── mod.rs
│       └── researcher.rs
```

## Consequences

### Positive

- **Decoupled architecture** — Pattern, executor, tools communicate via events
- **Parallel efficiency** — Multiple tools run concurrently
- **Extensible patterns** — New reasoning strategies without changing executor
- **Observable** — All events available for logging/debugging
- **Testable** — Mock event bus for unit tests

### Negative

- **Increased complexity** — Event system more complex than simple loop
- **Debugging overhead** — Event traces harder to follow than linear execution
- **Initial patterns limited** — Only ReAct in v1, others follow

### Neutral

- Event persistence (for replay/audit) deferred to future UOW
- Multi-agent coordination via shared event bus deferred

## Alternatives Considered

### Option A: Simple Loop (v1)

Single synchronous loop: prompt → parse → execute → repeat.

**Pros**: Simple, easy to understand
**Cons**: No parallelism, tight coupling, no pattern abstraction

**Superseded**: Too limited for advanced use cases.

### Option B: Actor Model

Full actor system with Actix/Bastion.

**Pros**: Maximum concurrency control
**Cons**: Heavy dependency, learning curve, overkill for v1

**Deferred**: May adopt for multi-agent orchestration later.

## References

- DAA: [DAA.md](../DAA.md) — Agent Orchestration context
- BOLT-007: [BOLT-007-reasoning-patterns](../../../UOW-001-foundation/bolts/BOLT-007-reasoning-patterns/) — Core trait definition
- Research: [pattern-react.md](../../../../researches/pattern-react.md)
- Research: [pattern-cot.md](../../../../researches/pattern-cot.md)
- Research: [pattern-tot.md](../../../../researches/pattern-tot.md)
- Research: [pattern-hybrid.md](../../../../researches/pattern-hybrid.md)
- Requirements: [requirements.md](../requirements.md)
