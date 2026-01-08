---
description: Full AI-DLC workflow for implementing new features
---

# Feature Workflow

This workflow guides you through the complete AI-DLC cycle for implementing a new feature.

## When to Use

- Building a new feature from scratch
- Implementing a significant enhancement
- Work that needs architectural decisions

**Not for**: Quick fixes, bug patches → Use `/airsspec-hotfix`

## Prerequisites

- `.airsspec/` directory exists (run `/airsspec-setup` if not)

## Steps

### Step 1: Create Unit of Work

1. Create a UOW container following `instructions/workflows/feature.md` Step 2.

2. Initialize `status.yaml`:
   ```yaml
   id: feature-{name}
   status: DRAFT
   created_at: <now>
   progress:
     total_bolts: 0
     completed_bolts: 0
   ```

### Step 2: Ingestion

// turbo
3. Review sources in `.airsspec/sources/`

4. Review playbooks in `.airsspec/knowledge/playbooks/`

5. Add any new relevant documents if needed.

### Step 3: Research Phase

6. Run `/airsspec-research` to create `requirements.md`.

7. Wait for user approval of requirements before proceeding.

### Step 4: Inception Phase

8. Run `/airsspec-inception` to create `DAA.md`.

9. Wait for user approval of DAA before proceeding.

### Step 5: Design Phase

10. Run `/airsspec-design` to create `ADR-*.md` files.

11. Wait for user approval of ADRs before proceeding.

### Step 6: Planning Phase

12. Run `/airsspec-planning` to create `RFC.md` and Bolt structure.

13. Wait for user approval of RFC before proceeding.

### Step 7: Construction Phase

14. Run `/airsspec-construction` to implement code.

15. Verify all tasks are complete.

### Step 8: Completion

16. Update UOW `status.yaml` to `status: COMPLETED`.

17. Archive the UOW if needed.

## Gate Summary

| Phase | Artifact | Gate |
|-------|----------|------|
| Research | `requirements.md` | Approved |
| Inception | `DAA.md` | Approved |
| Design | `ADR-*.md` | Approved |
| Planning | `RFC.md`, `bolts/` | Approved |
| Construction | Source code | All tasks done |
