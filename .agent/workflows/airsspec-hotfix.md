---
description: Fast track workflow for bug fixes and refactors
---

# Hotfix Workflow

This workflow provides a fast track for bug fixes, refactors, and optimizations.

## When to Use

- Fixing a bug
- Performing a refactor
- Making an optimization
- Any change that doesn't need new architecture decisions

**Not for**: New features, significant enhancements → Use `/airsspec-feature`

## Prerequisites

- `.airsspec/` directory exists (run `/airsspec-setup` if not)

## Choose Your Pattern

### Pattern A: Transient UOW
For independent fixes not related to existing work.

### Pattern B: Bolt Injection
For fixes related to an existing feature UOW.

## Steps

### Step 1: Identify the Fix

1. Describe the issue or refactoring goal.

2. Identify the relevant files.

### Step 2: Create UOW (Pattern A) or Inject Bolt (Pattern B)

**Pattern A - New UOW:**
3. Create a transient UOW:
   ```
   .airsspec/uow/fix-{name}/
   ├── status.yaml
   └── bolts/
       └── fix/
   ```

4. Initialize `status.yaml`:
   ```yaml
   id: fix-{name}
   type: hotfix
   status: IN_PROGRESS
   created_at: <now>
   ```

**Pattern B - Inject into existing UOW:**
3. Find the existing UOW in `.airsspec/uow/`

4. Add a new bolt to the existing structure.

### Step 3: Define Fix Context

// turbo
5. Read the relevant source files to understand the issue.

6. Document the fix context in the bolt.

### Step 4: Create Plan and Task

7. Create a plan in `bolts/fix/plans/PLAN-001.md`:
   - Describe what needs to change
   - List affected files
   - Define verification steps

8. Create corresponding task in `bolts/fix/tasks/TASK-001.md`.

### Step 5: Execute Fix

9. Run `/airsspec-construction` to implement the fix.

### Step 6: Complete

10. Update Bolt status to `COMPLETED`.

11. Update UOW status to `COMPLETED`.

## When to Escalate

If during the hotfix you discover:
- The fix requires architectural changes
- New domain concepts are needed

**Stop** and escalate to `/airsspec-feature` workflow.
