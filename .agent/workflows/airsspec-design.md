---
description: Design phase - creates ADR-*.md (Architecture Decision Records)
---

# Design Phase

This workflow executes the Design phase to create Architecture Decision Records.

## Prerequisites

- `DAA.md` exists and is approved
- UOW container exists

## Instructions

Follow `instructions/phases/design.md` for detailed guidance.

## Steps

### Step 1: Review Inputs

// turbo
1. Read `.airsspec/uow/{uow-id}/DAA.md`

// turbo
2. List available playbooks in `.airsspec/knowledge/playbooks/`

### Step 2: Select Playbooks

3. Present playbook options to user for selection.

4. User selects which playbooks to apply (e.g., Modulith, Postgres, Redis).

### Step 3: Apply Playbooks

5. For each selected playbook, apply it to the domain model.

6. Formula: `ADR = DAA + (n × Playbooks)`

### Step 4: Create ADRs

7. Create ADR files in `.airsspec/uow/{uow-id}/`:
   - `ADR-001-{topic}.md`
   - `ADR-002-{topic}.md`
   - etc.

8. Follow the MADR format from `instructions/phases/design.md`:
   - Context
   - Decision
   - Consequences
   - Alternatives Considered

### Step 5: Get Approval

9. Present ADRs to user for review.

10. If approved, proceed to `/airsspec-planning`.

## Transition Criteria

- [ ] At least one ADR exists
- [ ] All ADRs are approved
- [ ] Technology decisions are documented
