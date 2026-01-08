---
description: Planning phase - creates RFC.md and Bolt structure
---

# Planning Phase

This workflow executes the Planning phase to create the RFC and Bolt structure.

## Prerequisites

- `DAA.md` exists and is approved
- `ADR-*.md` files exist and are approved
- UOW container exists

## Instructions

Follow `instructions/phases/planning.md` for detailed guidance.

## Steps

### Step 1: Review Inputs

// turbo
1. Read `.airsspec/uow/{uow-id}/DAA.md`

// turbo
2. Read all `.airsspec/uow/{uow-id}/ADR-*.md` files

### Step 2: Create RFC

3. Synthesize DAA + ADRs into implementation strategy.

4. Formula: `RFC = DAA + Σ(ADRs)`

5. Create `.airsspec/uow/{uow-id}/RFC.md` with:
   - Summary
   - Motivation
   - Design Overview
   - Implementation Plan
   - Risks & Mitigations

### Step 3: Decompose into Bolts

6. Break work into logical Bolts (modules).

7. For each Bolt, create directory structure:
   ```
   bolts/{bolt-id}/
   ├── status.yaml
   ├── plans/
   └── tasks/
   ```

### Step 4: Create Plans and Tasks

8. For each Bolt, create plans in `plans/`:
   - `PLAN-001.md` — What to do, how to do it

9. For each plan, create corresponding task in `tasks/`:
   - `TASK-001.md` — Links to plan, tracks execution

### Step 5: Get Approval

10. Present RFC and Bolt structure to user for review.

11. If approved, proceed to `/airsspec-construction`.

## Transition Criteria

- [ ] `RFC.md` exists and is approved
- [ ] All Bolts have plans and tasks
- [ ] Dependencies between Bolts are clear
