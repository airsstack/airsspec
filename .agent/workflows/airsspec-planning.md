---
description: Planning phase - creates RFC.md and Bolt structure
---

# Planning Phase

## Core Instructions

Follow `instructions/phases/planning.md` for detailed guidance.

## Quick Reference

| Item | Value |
|------|-------|
| **Input** | `DAA.md` + `ADR-*.md` (approved) |
| **Output** | `RFC.md`, `bolts/` |
| **Templates** | `templates/uow/RFC.md`, `templates/bolt/*` |
| **Formula** | `RFC = DAA + Σ(ADRs)` |
| **Next** | `/airsspec-construction` |

## Steps

// turbo
1. Read `DAA.md` and `ADR-*.md`

2. Create `RFC.md` using the template

3. Decompose into Bolts (modules)

4. Create plans and tasks for each Bolt

5. Get user approval before proceeding

## Transition

- [ ] `RFC.md` exists and approved
- [ ] Bolts have plans and tasks
