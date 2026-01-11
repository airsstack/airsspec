# Agent Orchestration, Code Quality, and Language Guidelines

**Date**: 2026-01-11
**Topic**: Agent Orchestration, Code Quality, Language Guidelines

---

## The Problem I Was Facing

I was running into two related issues that were preventing the AI-DLC workflow from working as intended:

**Issue 1: Project Standard Consistency**
My Rust project guidelines (in `.aiassisted/guidelines/rust/project-standard.md` and related files) weren't being consistently loaded by AI agents during the Construction phase. These guidelines define critical patterns that I want the codebase to follow:
- 3-layer import organization
- No fully qualified names in type annotations
- chrono `DateTime<Utc>` as the standard
- Module architecture patterns (no type re-exports in mod.rs)
- Dependency management conventions
- ADT patterns

The agent was implementing code without knowing about these standards, which meant I'd have to manually fix things afterward.

**Issue 2: Missing Review Points in Bolt Development**
When `airsspec-builder` was called, it would explore the active UOW and find the next sequential ordered bolt. The problem was that it would start working on tasks directly without giving me a chance to review what was implemented. There was no HALT point after each completed task — the agent would just keep going.

This violated the "Trust But Verify" principle at the bolt level. Even though I had implemented HALT points at the phase level (Research, Inception, Design, Planning), the actual code implementation phase was missing these checkpoints.

## What I Did

### 1. Fixed Project Standard Loading

I adjusted the Construction phase instructions to explicitly reference the Rust guidelines as mandatory reading:

**In `.airsspec/agent/phases/construction.md` (project-local):**
```markdown
Also read relevant:
- **Rust implementation guidelines** in `$WORKSPACE_ROOT/.aiassisted/guidelines/rust/`:
  - `project-standard.md` — Import organization, module patterns
  - `adt-patterns.md` — Sum types, newtypes, derive macros
  - `dependency-management.md` — When adding dependencies
```

The key insight was to keep upstream instructions generic (`instructions/`) and put project-specific guidelines in `.airsspec/agent/`. This pattern allows AirsSpec to work for any tech stack while letting each project define its own standards.

### 2. Integrated Reviewer as Blocking Gate

I updated the core instructions and guidelines to ensure the bolt development workflow HALTs after each completed task. The Reviewer sub-agent was integrated as a blocking gate that:

- Checks only uncommitted changes per Bolt (via `git diff --name-only`)
- Runs lint checks, test suites, and pattern checks against project guidelines
- BLOCKS task completion if issues are found
- For Rust: runs `cargo clippy --all-targets --all-features -- -D warnings` and `cargo test`

**In the construction instructions:**
```markdown
**Reviewer Sub-Agent**: Blocking Gate
- Scope: Check only uncommitted changes per Bolt
- Process: Run lint/test → check patterns → block if issues found
- Blocking: Cannot mark task complete if Reviewer finds issues
```

### 3. Applied the Pattern Consistently

This built on the HALT strategy I implemented across all phases (Research, Inception, Design, Planning) earlier. The pattern was now complete:
- Phase-level HALTs: After requirements.md, DAA.md, ADRs, RFC.md
- Task-level HALTs: After each bolt task is completed

### 4. Set Up UOW-001 Foundation

Alongside these fixes, I also set up the first complete UOW (UOW-001 Foundation) with all the proper AI-DLC artifacts:
- `requirements.md` — Research phase
- `DAA.md` — Inception phase
- 4 ADRs — Design phase
- `RFC.md` — Planning phase
- 6 Bolts with 21 plans/tasks — Planning phase

This gave me a real project to test the improved workflow on.

## What I Learned

### Upstream vs Project-Local is the Right Pattern

The separation between `instructions/` (upstream, generic) and `.airsspec/agent/` (project-local, specific) continues to prove valuable. It allows AirsSpec to be a framework that works for any tech stack while giving each project the freedom to define its own standards.

When I first thought about adding language-specific guidelines, I considered putting them directly in the upstream Construction instructions. But that would have polluted the framework with Rust-specific details. Keeping them project-local makes the framework more reusable.

### Human-in-the-Loop Works at Multiple Levels

I now have HALT points at two levels:
1. **Phase level**: Between major artifacts (requirements → DAA → ADRs → RFC → Bolts)
2. **Task level**: After each completed bolt task

This gives me control at different granularities. I can let a phase complete entirely and then review the whole artifact, or I can review each individual task as it's completed. Both are useful depending on the context.

### Reviewer as Gate, Not Suggestion

Making the Reviewer a blocking gate rather than optional feedback changes the dynamic. Tasks literally can't be marked complete without passing review. This enforces quality at the workflow level, not just as a suggestion that might be ignored.

### Scope Awareness Prevents Distraction

Having the Reviewer check only uncommitted changes per Bolt (not the entire codebase) keeps the scope focused. `git diff --name-only` filtered by Bolt context prevents the agent from getting distracted by unrelated files. This is important — you don't want the agent failing a review because of an issue in a completely different part of the codebase.

### Filesystem as Truth

All these improvements reinforced the philosophy that the filesystem is the single source of truth. Instructions, guidelines, status files — everything is on disk. The agents read from the filesystem, make decisions, and write back. This makes the system transparent and debuggable.

## Files Changed

### Phase Instructions
- `instructions/phases/construction.md` — Added generic Reviewer workflow (upstream)
- `.airsspec/agent/phases/construction.md` — Added Rust-specific Reviewer with cargo commands and guideline references

### Workflow Agents
- `.agent/workflows/airsspec-construction.md` — Updated Quick Reference
- `.opencode/agent/airsspec-builder.md` — Updated description and Quick Reference

### UOW-001 Artifacts
- `.airsspec/uows/uow-001-foundation/requirements.md` — Research artifact
- `.airsspec/uows/uow-001-foundation/DAA.md` — Inception artifact
- `.airsspec/uows/uow-001-foundation/adrs/ADR-*.md` — 4 Design artifacts
- `.airsspec/uows/uow-001-foundation/RFC.md` — Planning artifact
- `.airsspec/uows/uow-001-foundation/bolts/bolt-*/plan.md` — 6 Bolt plans
- `.airsspec/uows/uow-001-foundation/bolts/bolt-*/status.yaml` — 6 Bolt status files
- `.airsspec/uows/uow-001-foundation/bolts/bolt-*/tasks/task-*.md` — 21 Task files

## Next Steps

1. **Test the full Construction flow** with BOLT-001-primitives to verify the agent loads Rust guidelines and runs `cargo clippy` before marking tasks complete
2. **Intentionally introduce a lint error** to confirm the blocking Reviewer gate works as intended
3. **Complete remaining BOLT-001 tasks** to validate the workflow through multiple iterations
4. **Consider adding more language-specific guideline templates** for other stacks (Python, TypeScript, etc.) as reusable starting points

---

*These improvements give me confidence that the AI-DLC workflow can scale. The combination of phase-level and task-level HALTs, along with blocking quality gates, creates a development process that's both efficient and controlled. The framework stays generic while each project can define its own standards.*
