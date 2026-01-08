---
description: Research phase - creates requirements.md
---

# Research Phase

This workflow executes the Research phase to create `requirements.md`.

## Prerequisites

- UOW container exists
- Sources are available in `.airsspec/sources/`

## Instructions

Follow `instructions/phases/research.md` for detailed guidance.

## Steps

### Step 1: Gather Context

// turbo
1. Review available sources in `.airsspec/sources/`

// turbo
2. Read any existing documentation in the UOW directory.

### Step 2: Clarify Intent

3. If the user's request is vague, ask clarifying questions:
   - "What problem does this solve?"
   - "Who is the target user?"
   - "What's explicitly out of scope?"

4. Document answers for the requirements.

### Step 3: Research (if needed)

5. Search the web for external information if there are knowledge gaps.

6. Document findings in `sources/`.

### Step 4: Draft Requirements

7. Create `.airsspec/uow/{uow-id}/requirements.md` following the structure in `instructions/phases/research.md`.

8. Include:
   - Problem Statement
   - Success Criteria
   - Scope (In/Out)
   - User Stories
   - Context Sources
   - Open Questions

### Step 5: Get Approval

9. Present `requirements.md` to user for review.

10. If approved, proceed to `/airsspec-inception`.

## Transition Criteria

- [ ] `requirements.md` exists
- [ ] Status is `approved`
- [ ] No critical open questions
