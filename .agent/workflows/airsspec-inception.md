---
description: Inception phase - creates DAA.md (Domain Architecture Analysis)
---

# Inception Phase

This workflow executes the Inception phase to create `DAA.md`.

## Prerequisites

- `requirements.md` exists and is approved
- UOW container exists

## Instructions

Follow `instructions/phases/inception.md` for detailed guidance.

## Steps

### Step 1: Analyze Requirements

// turbo
1. Read `.airsspec/uow/{uow-id}/requirements.md`

2. Extract core concepts:
   - Identify nouns (entities)
   - Identify verbs (behaviors)
   - Identify rules (constraints)

### Step 2: Define Domain Model

3. Define the domain model:
   - Entities and their attributes
   - Value Objects
   - Aggregates and boundaries

### Step 3: Map Bounded Contexts

4. Identify bounded contexts and their relationships.

5. Establish ubiquitous language for the domain.

### Step 4: Draft DAA

6. Create `.airsspec/uow/{uow-id}/DAA.md` following the structure in `instructions/phases/inception.md`.

7. Include:
   - Domain Model Diagram
   - Bounded Contexts
   - Entities & Value Objects
   - Aggregates
   - Domain Rules
   - Ubiquitous Language

### Step 5: Get Approval

8. Present `DAA.md` to user for review.

9. If approved, proceed to `/airsspec-design`.

## Transition Criteria

- [ ] `DAA.md` exists
- [ ] Status is `approved`
- [ ] Domain model is complete
