# Setting Up UOW-002 and Discovering the Foundation Principle

**Date**: 2026-01-12  
**Topic**: AI-DLC Workflow, DAA Generation, Core Abstractions

---

## The Problem I Was Facing

I started setting up UOW-002: Agent System, following the AI-DLC workflow (Research → Inception → Design → Planning → Construction). Everything seemed straightforward until I hit two issues that made me rethink how the framework handles core abstractions.

**Issue 1: DAA Generation Was Wrong**

My first attempt at creating the DAA (Domain Architecture Analysis) for UOW-002 came out implementation-focused — full of Rust crate structures, code snippets, and technology details. The user correctly pointed out that DAA should be **technology-agnostic** and follow **DDD principles** (Bounded Contexts, Entities, Value Objects, Aggregates).

**Issue 2: Pattern-Specific Types in Core**

When designing the `ReasoningPattern` trait for event-driven agent execution, I initially included a `BranchEval` variant in `ReasoningStep` — but that's specific to the Tree-of-Thoughts (ToT) pattern. The user caught this: core types should be **generic and pattern-agnostic**.

---

## What I Did

### 1. Fixed the DAA Generation Workflow

I did a root cause analysis and found the problem: I was referencing UOW-001's DAA as a template instead of reading `templates/uow/DAA.md` as the instructions specify.

To prevent this from happening again, I:
1. Created `researches/ddd-principles.md` with comprehensive DDD guidance
2. Updated `instructions/phases/inception.md` with mandatory prerequisites
3. Added `$TEMPLATES_PATH` and `$RESEARCHES_PATH` to path-variables.md

Now the Inception phase explicitly says:
```markdown
> [!IMPORTANT]
> **MANDATORY READING BEFORE DAA GENERATION**
> 1. READ `$WORKSPACE_ROOT/researches/ddd-principles.md`
> 2. READ `templates/uow/DAA.md`
> 3. READ `uow/{id}/requirements.md`
```

### 2. Created ADRs for UOW-002

With the DAA fixed, I created three Architecture Decision Records:
- **ADR-001**: LLM Integration via Rig + OpenRouter
- **ADR-002**: Tool System with Registry and Sandbox
- **ADR-003**: Event-driven Agent Execution with Pluggable Patterns

The interesting part was ADR-003 — the user wanted event-driven execution with parallel tool support and **abstracted reasoning patterns** (ReAct, CoT, ToT).

### 3. Discovered ReasoningPattern Belongs in Core

While designing ADR-003, I realized `ReasoningPattern` is a foundational trait — same level as `LLMProvider`, `Tool`, and `Agent`. It doesn't belong in UOW-002 (Agent System), it belongs in UOW-001 (Foundation Layer).

I amended UOW-001 with BOLT-007 to add the `reasoning/` module to `airsspec-core`.

### 4. Applied the Foundation Principle

When I drafted the `ReasoningStep` enum, I included:
```rust
BranchEval {
    branch_id: String,
    score: f32,
    should_continue: bool,
}
```

The user correctly called this out — `BranchEval` is ToT-specific. Core shouldn't know about specific patterns. Instead, I added a generic `Extension` variant:
```rust
Extension {
    pattern: String,  // e.g., "tot"
    kind: String,     // e.g., "branch_eval"
    data: String,     // Serialized pattern-specific data
}
```

This way, patterns can extend the step types without modifying core.

### 5. Verified Rust Guidelines Compliance

For BOLT-007, I checked the plans against `.aiassisted/guidelines/rust/project-standard.md`. Found a §4.3 violation — the mod.rs had type re-exports:
```rust
// ❌ FORBIDDEN
pub use traits::{ReasoningPattern, PatternSelector};
```

Fixed to:
```rust
// ✅ COMPLIANT
pub mod traits;
pub mod types;
```

---

## What I Learned

### Templates Over Examples

When generating artifacts, always read the **template** first, not previous examples. UOW-001's DAA was also wrong, so using it as a reference just propagated the error. The template in `templates/uow/DAA.md` is the source of truth.

### Foundation Principle

Core types must be **generic and pattern-agnostic**. If I find myself adding something specific to one implementation (like `BranchEval` for ToT), it doesn't belong in core. Use extension points instead.

### §4.3 Is Easy to Violate

The temptation to add `pub use types::SomeType` in mod.rs is strong — it makes imports shorter. But it violates the module architecture pattern and causes namespace pollution. Callers should use explicit namespaced imports.

### Amending "Complete" UOWs Is Okay

UOW-001 was marked complete, but I needed to add BOLT-007 for reasoning patterns. That's fine — "complete" means the initial scope is done, not that nothing can ever be added. I updated the status back to `in_progress` with a clear reason.

---

## Files Changed

**Research Documents**:
- `researches/ddd-principles.md` — DDD concepts for DAA generation
- `researches/pattern-react.md` — ReAct pattern research
- `researches/pattern-cot.md` — Chain-of-Thought research
- `researches/pattern-tot.md` — Tree-of-Thoughts research
- `researches/pattern-hybrid.md` — Hybrid pattern research

**Instructions**:
- `instructions/phases/inception.md` — Added mandatory DDD prerequisites
- `instructions/core/path-variables.md` — Added $TEMPLATES_PATH, $RESEARCHES_PATH

**UOW-001 Amendment**:
- `.airsspec/uow/UOW-001-foundation/status.yaml` — Reopened with BOLT-007
- `.airsspec/uow/UOW-001-foundation/bolts/BOLT-007-reasoning-patterns/` — New bolt

**UOW-002 Artifacts**:
- `.airsspec/uow/UOW-002-agent-system/requirements.md`
- `.airsspec/uow/UOW-002-agent-system/DAA.md`
- `.airsspec/uow/UOW-002-agent-system/adrs/ADR-001,002,003.md`

**Architecture**:
- `docs/architecture.md` — Added reasoning/ module and ReasoningPattern trait

---

## Next Steps

1. Complete BOLT-007 in UOW-001 (implement the actual Rust code)
2. Continue UOW-002 through Planning phase (create RFC.md and bolts)
3. Consider creating a "Foundation Principle" guideline document to formalize the pattern

---

*Today was a good example of the AI-DLC workflow catching design issues early. The user's feedback on both DAA structure and core type genericity prevented mistakes that would have been harder to fix later. Human-in-the-loop at its finest.*
