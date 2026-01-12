# Workflow: Hotfix (Fast Track)

A step-by-step guide for quick fixes, bug patches, and refactors that skip the full AI-DLC cycle.

---

## When to Use

Use this workflow when:
- Fixing a bug
- Performing a refactor
- Making an optimization
- Any change that doesn't need new architecture decisions

**Not for**: New features, significant enhancements → Use [feature.md](./feature.md)

---

## Prerequisites

Before starting, ensure:
- [ ] `.airsspec/` directory exists
- [ ] You understand the codebase context
- [ ] You have a clear description of the fix

---

## Overview

The Fast Track skips Research → Inception → Design → Planning and goes directly to Construction.

### Choose Your Pattern

| Pattern | Use Case | Creates |
|---------|----------|---------|
| **Transient UOW** | Independent fix, not related to existing work | New UOW container |
| **Bolt Injection** | Fix related to an existing feature UOW | Bolt inside existing UOW |

```
Pattern A: Transient UOW
  Ingestion → → → → → → → → → [New UOW] → Construction

Pattern B: Bolt Injection
  [Existing UOW] → → → → → → → [Inject Bolt] → Construction
```

---

## Step 1: Initialize Workspace (if needed)

If `.airsspec/` doesn't exist:

1. Follow [workspace-explore.md](../core/workspace-explore.md)
2. Follow [workspace-setup.md](../core/workspace-setup.md)

**Verification**:
- [ ] `.airsspec/WORKSPACE.md` exists

---

## Step 2: Choose Pattern

Based on your use case, choose one:

---

# Pattern A: Transient UOW

Use when the fix is **independent** and not related to existing work.

## A.1: Create Transient UOW

### Choose UOW ID
Format: `{type}-{short-name}`
Examples: `fix-race-condition`, `refactor-auth-middleware`, `optimize-query`

### Create UOW Directory

```
.airsspec/uow/{uow-id}/
├── status.yaml
└── bolts/
    └── fix/
        ├── status.yaml
        ├── plans/
        │   └── PLAN-001.md
        └── tasks/
            └── TASK-001.md
```

### Initialize Status

```yaml
id: {uow-id}
type: hotfix
status: IN_PROGRESS
created_at: <ISO-8601>
description: "<fix description>"
progress:
  total_bolts: 1
  completed_bolts: 0
```

**Continue to**: [Step 3: Define Fix Context](#step-3-define-fix-context)

---

# Pattern B: Bolt Injection

Use when the fix is **related to an existing feature** UOW.

## B.1: Select Target UOW

List existing UOWs:
```
.airsspec/uow/
├── feature-payment/      ← Inject here if fix relates to payment
├── feature-auth/
└── ...
```

## B.2: Inject Bolt into Existing UOW

Create a new Bolt inside the existing UOW:

```
.airsspec/uow/{existing-uow-id}/bolts/
├── database/             ← Existing bolt
├── api/                  ← Existing bolt
└── fix-{issue}/          ← NEW: Injected bolt
    ├── status.yaml
    ├── plans/
    │   └── PLAN-001.md
    └── tasks/
        └── TASK-001.md
```

### Initialize Bolt Status

```yaml
# .airsspec/uow/{existing-uow-id}/bolts/fix-{issue}/status.yaml
id: fix-{issue}
parent_uow: {existing-uow-id}
type: hotfix-injection
status: PENDING
created_at: <ISO-8601>
description: "<fix description>"
```

**Note**: The parent UOW state does NOT need to change. The bolt operates within the existing UOW context.

**Continue to**: [Step 3: Define Fix Context](#step-3-define-fix-context)

---

## Step 3: Define Fix Context

*(Applies to both patterns)*

### 3.1 Identify Relevant Files

List the files that will be modified:
```
context:
  files:
    - src/auth/middleware.rs
    - src/handlers/login.rs
```

### 3.2 Mount External Context (if needed)

If you need reference documentation:
```bash
cp docs/auth-spec.md .airsspec/sources/
```

---

## Step 4: Create Plan and Task

### 4.1 Create Plan

```markdown
# PLAN-001: [Fix Description]

## Objective
[What are we fixing?]

## Root Cause
[Why is this happening?]

## Solution
[How will we fix it?]

## Steps
1. [Step 1]
2. [Step 2]
3. [Step 3]

## Verification
- [ ] Unit test for the fix
- [ ] Existing tests pass
- [ ] No regression

## Files to Modify
- `path/to/file1.rs`
- `path/to/file2.rs`
```

### 4.2 Create Task

```markdown
# TASK-001: [Fix Description]

**Plan Reference**: [../plans/PLAN-001.md](../plans/PLAN-001.md)

## Execution Output
(Will be filled during execution)
```

---

## Step 5: Execute Fix

**Reference**: [phases/construction.md](../phases/construction.md)

### 5.1 Read Context
1. Read the Plan
2. Read the relevant source files
3. Understand the current behavior

### 5.2 Implement Fix
1. Make the code changes
2. Follow existing patterns in the codebase
3. Keep changes minimal and focused

### 5.3 Verify
```bash
cargo test --package {package} -- {test_name}
cargo clippy
```

### 5.4 Document in Task

```markdown
# TASK-001: Fix Race Condition in Auth Middleware

**Plan Reference**: [../plans/PLAN-001.md](../plans/PLAN-001.md)

## Execution Output

### Actions Taken
- Added mutex guard around session state access
- Added regression test

### Files Modified
- `src/auth/middleware.rs` (modified)
- `tests/auth_test.rs` (added test)

### Verification
- [x] New test passes
- [x] All existing tests pass
```

---

## Step 6: Complete

### 6.1 Update Bolt Status

```yaml
status: COMPLETED
completed_at: <ISO-8601>
```

### 6.2 Update State (Pattern A only)

For **Transient UOW** pattern, update the UOW state:
```json
{
  "phase": "COMPLETED",
  "completed_at": "<ISO-8601>"
}
```

For **Bolt Injection** pattern, the parent UOW remains in its current phase — only the injected bolt is marked complete.

### 6.3 (Optional) Update Knowledge

If this fix reveals a lesson learned:
```markdown
# .airsspec/knowledge/library/lesson-{date}-{topic}.md

## Lesson: [Topic]
...
```

---

## Quick Reference (Future CLI)

> **Note**: The `airsspec` CLI does not exist yet. These instructions are for AI agents working manually. The commands below represent the **future CLI design**.

```bash
# Future: Pattern A - Create transient UOW
airsspec fix "Fix race condition in auth_middleware.rs"

# Future: Pattern B - Inject into existing UOW
airsspec fix "Fix race condition" --uow feature-payment
```

**For now**: Follow the manual steps above to create directories and files.

---

## Summary

| Pattern | Creates | Best For |
|---------|---------|----------|
| Transient UOW | New independent UOW | Standalone fixes |
| Bolt Injection | Bolt in existing UOW | Related fixes, maintaining context |

---

## When to Escalate

If during the hotfix you discover:
- The fix requires architectural changes
- New domain concepts are needed

**Stop** and escalate to [feature.md](./feature.md) workflow.

---

**Alternative Workflow**: [feature.md](./feature.md) — Full AI-DLC Cycle
