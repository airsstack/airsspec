# Setting Up UOW-001: From Research to Bolt Plans

**Date**: 2026-01-10
**Topic**: AI-DLC, UOW Setup, Documentation Alignment

---

## The Problem I Was Facing

The roadmap and Rust workspace were set up from earlier today, but I still had no actual "Unit of Work" to track. The architecture talked about UOWs, the workflow instructions talked about UOWs, but there was no concrete UOW sitting in `.airsspec/uow/`.

I needed to:
1. Create the first real UOW (UOW-001 Foundation Layer)
2. Walk through all the AI-DLC phases: Research → Inception → Design → Planning
3. Make sure everything aligned with the existing docs and instructions

## What I Did

### 1. Created UOW-001 with Full AI-DLC Artifacts

I created the complete UOW-001 Foundation following all phases:

| Phase | Artifact | Content |
|-------|----------|---------|
| Research | `requirements.md` | Problem statement, success criteria, scope |
| Inception | `DAA.md` | Domain model, bounded contexts, module breakdown |
| Design | 4 ADRs | Primitives, Contract, Cognition, Agent sub-phases |
| Planning | `RFC.md` + 6 Bolts | Implementation plan with 21 plans/21 tasks |

### 2. Caught Documentation Inconsistencies

While creating UOW-001, I compared against `docs/` and `instructions/` and found several inconsistencies:

- `architecture.md` referenced `state.json` and `PRD.md` but newer specs used `status.yaml` and `requirements.md`
- ADR files were flat in UOW root, but grouping them in `adrs/` made more sense
- Bolt `status.yaml` in instructions was minimal, but I added useful fields (title, description, dependencies)

### 3. Synced Everything

Instead of just moving on, I stopped to fix the inconsistencies:

1. Updated `docs/architecture.md` to use `status.yaml` and `requirements.md`
2. Restructured UOW to put ADRs in an `adrs/` subdirectory
3. Formalized the enhanced bolt `status.yaml` schema in instructions

### 4. Created All Bolt Plans

Each of the 6 bolts got detailed plans with:
- Clear objectives
- ADR references
- Step-by-step implementation guide
- Verification checklists

Total: 21 plans, 21 tasks with strict 1:1 mapping.

## What I Learned

### Documentation Drift is Real

Even though all the docs were written recently, they had already drifted:
- `architecture.md` said one thing
- `uow-bolt-spec.md` said another
- `instructions/phases/planning.md` had a third variation

This session was as much about alignment as it was about creating UOW-001.

### "Enhancements" Should Be Documented

I added useful fields to bolt `status.yaml` (title, description, plans, dependencies) because they seemed helpful. But the user correctly pointed out this was a "surprise" — it wasn't in the instructions.

Lesson: If you improve a format, update the source-of-truth documentation immediately. Don't let improvements live only in examples.

### ADR Directory Makes Sense

Originally, ADRs sat flat alongside `DAA.md`, `RFC.md`, etc. Grouping them in `adrs/` keeps the UOW root cleaner:

```
UOW-001/
├── status.yaml
├── requirements.md
├── DAA.md
├── adrs/           ← cleaner
│   ├── ADR-001.md
│   └── ADR-002.md
├── RFC.md
└── bolts/
```

### The AI-DLC Flow Works

Walking through Research → Inception → Design → Planning felt methodical. Each phase built on the previous one:
- `requirements.md` informed `DAA.md`
- `DAA.md` informed the ADRs
- ADRs informed `RFC.md` and bolts

The human-in-the-loop approvals at each gate kept things on track.

## Files Changed

### UOW-001 Artifacts (First Commit)
- `.airsspec/uow/UOW-001-foundation/requirements.md` — Research phase output
- `.airsspec/uow/UOW-001-foundation/DAA.md` — Inception phase output
- `.airsspec/uow/UOW-001-foundation/adrs/ADR-001-primitives.md` — Design phase
- `.airsspec/uow/UOW-001-foundation/adrs/ADR-002-contract.md` — Design phase
- `.airsspec/uow/UOW-001-foundation/adrs/ADR-003-cognition.md` — Design phase
- `.airsspec/uow/UOW-001-foundation/adrs/ADR-004-agent.md` — Design phase
- `.airsspec/uow/UOW-001-foundation/RFC.md` — Planning phase output
- `.airsspec/uow/UOW-001-foundation/status.yaml` — UOW state tracking

### Bolt Plans (Second Commit)
- 6 bolt directories with `status.yaml`, `plans/`, `tasks/`
- 21 plan files (PLAN-001 through PLAN-006 per bolt)
- 21 task files (1:1 mapping to plans)

### Documentation Sync
- `docs/architecture.md` — Updated to use `status.yaml`, `requirements.md`, `adrs/`
- `docs/uow-bolt-spec.md` — Added enhanced bolt status schema
- `instructions/phases/planning.md` — Formalized bolt status.yaml fields
- `instructions/phases/design.md` — Updated ADR output paths

## Next Steps

UOW-001 is fully planned. Next up:

1. **Enter Construction phase** — Start with BOLT-001-primitives
2. **Implement `error.rs`** — Define all error types per PLAN-001
3. **Implement `state/` module** — Types and traits per PLAN-002, PLAN-003
4. **Verify with cargo build** — Each plan has a verification checklist

---

*First UOW complete. 21 plans might seem like a lot, but each one is small and focused. That's the whole point of the Cognitive Cleanroom — keep the context narrow so the agent (or human) can actually execute without getting lost.*
