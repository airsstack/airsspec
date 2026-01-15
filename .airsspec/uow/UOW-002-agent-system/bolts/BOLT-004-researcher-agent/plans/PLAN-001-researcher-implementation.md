# PLAN-001: Researcher Agent Implementation

## Objective

Implement the Researcher agent that analyzes sources and produces `requirements.md` artifacts.

## Context

First concrete agent demonstrating the complete system: LLM + Tools + Executor + Pattern. Validates entire architecture works end-to-end.

## Steps

1. Create `src/agents/researcher.rs`
2. Implement `ResearcherAgent` struct:
   - Holds executor, config
   - Uses ReAct pattern by default
3. Define `ResearcherConfig`:
   - Preamble (prompt engineering)
   - Max sources
   - Output path
4. Implement `research()` method:
   - Create session
   - Build research prompt from sources
   - Execute via executor
   - Parse result into `RequirementsDocument`
   - Write artifact to filesystem
5. Define `RequirementsDocument` struct with fields:
   - id, title, version, status
   - goal, problem_statement
   - success_criteria, scope
   - dependencies, source_references
6. Implement `RequirementsDocument::from_markdown()` parser
7. Implement `RequirementsDocument::to_markdown()` generator
8. Create comprehensive preamble for Researcher
9. Implement `Agent` trait

## Expected Output

### Files Created:
- `crates/airsspec-agents/src/agents/researcher.rs`

### Files Modified:
- `crates/airsspec-agents/src/agents/mod.rs` — Export Researcher
- `crates/airsspec-agents/src/lib.rs` — Re-export Researcher

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes
- [ ] Researcher compiles
- [ ] RequirementsDocument types are correct
- [ ] Agent trait is implemented

## References

- Requirements: [../../requirements.md](../../requirements.md)
- RFC: [../../RFC.md](../../RFC.md)
- ADR-003: [../../adrs/ADR-003-agent-architecture.md](../../adrs/ADR-003-agent-architecture.md)
