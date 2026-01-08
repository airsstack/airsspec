---
description: Construction phase - implements code from plans
---

# Construction Phase

This workflow executes the Construction phase to implement code.

## Prerequisites

- `RFC.md` exists and is approved
- Bolts with plans and tasks exist
- UOW container exists

## Instructions

Follow `instructions/phases/construction.md` for detailed guidance.

## Steps

### Step 1: Review Work

// turbo
1. Read `.airsspec/uow/{uow-id}/RFC.md`

// turbo
2. List all Bolts in `.airsspec/uow/{uow-id}/bolts/`

3. Identify Bolt execution order (based on dependencies).

### Step 2: Execute Bolts

For each Bolt (in dependency order):

4. Update Bolt `status.yaml` to `status: IN_PROGRESS`

5. For each Task in the Bolt:
   
   a. Read the corresponding Plan
   
   b. Implement the code according to the plan
   
   // turbo
   c. Run tests to verify implementation
   
   // turbo
   d. Run linting to check code quality
   
   e. Update Task file with execution output

6. Update Bolt `status.yaml` to `status: COMPLETED`

### Step 3: Verify All Bolts

7. Ensure all Bolts are marked `COMPLETED`.

8. Run full test suite to verify integration.

9. Check that implementation matches ADR decisions.

### Step 4: Complete

10. Update UOW `status.yaml` to `status: COMPLETED`.

11. Document any deviations or learnings in the UOW.

## Constraints

Even with full tool access:
- Every change must reference the active Task
- Only modify files relevant to current Bolt
- Verify before marking complete

## Verification Checklist

- [ ] All Tasks executed
- [ ] Tests pass
- [ ] Linting passes
- [ ] Code matches ADR decisions
- [ ] Task files updated with execution output
