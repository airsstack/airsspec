---
description: Construction phase - implements code from plans
---

# Construction Phase

## Core Instructions

Follow `instructions/phases/construction.md` for detailed guidance.

## Quick Reference

| Item | Value |
|------|-------|
| **Input** | `RFC.md` + `bolts/` with plans |
| **Output** | Source code |
| **Template** | `templates/bolt/TASK.md` |

## Steps

// turbo
1. Read `RFC.md` and list Bolts

2. Identify Bolt execution order (dependencies)

3. For each Bolt:
   - Execute plans
   - Update task files
   // turbo
   - Run tests
   // turbo
   - Run linting

4. Mark tasks and Bolts as completed

5. Update UOW status to `COMPLETED`

## Verification

- [ ] All tasks executed
- [ ] Tests pass
- [ ] Code matches ADR decisions
