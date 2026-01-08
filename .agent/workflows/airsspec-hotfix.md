---
description: Fast track workflow for bug fixes and refactors
---

# Hotfix Workflow

## Core Instructions

Follow `instructions/workflows/hotfix.md` for detailed guidance.

## Quick Reference

| Item | Value |
|------|-------|
| **Workflow** | Fast track |
| **Use Case** | Bug fixes, refactors, optimizations |
| **Path** | Direct to Construction |

## Patterns

| Pattern | Use When |
|---------|----------|
| **Transient UOW** | Independent fix |
| **Bolt Injection** | Related to existing feature |

## Steps

1. Check `.airsspec/` exists

2. Create UOW or inject into existing:
   - Transient: Create `fix-{name}/` UOW
   - Inject: Add bolt to existing UOW

3. Define fix context and create plan

4. Run `/airsspec-construction`

5. Update status to `COMPLETED`

## When to Escalate

If fix requires architectural changes → use `/airsspec-feature` instead
