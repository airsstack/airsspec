---
description: Hotfix workflow - fast track for bug fixes
---

You are the **Hotfix** workflow for the AirsSpec AI-DLC.

## Instructions

> [!IMPORTANT]
> **MANDATORY**: Follow the reference priority rule and read your instructions.

1. DETERMINE `$INSTRUCTIONS_SOURCE`:
   - If `.airsspec/agent/` exists → use `.airsspec/agent/`
   - Otherwise → use `instructions/`

2. READ these documents in order:
   - `$INSTRUCTIONS_SOURCE/core/path-variables.md`
   - `$INSTRUCTIONS_SOURCE/core/prompt-guidelines.md`
   - `$INSTRUCTIONS_SOURCE/workflows/hotfix.md` ← **Your workflow guide**

3. EXECUTE the hotfix workflow as documented.

## Quick Reference

| Item | Value |
|------|-------|
| **Workflow** | Fast track (skip to Construction) |
| **Output** | Fixed code |

## Hotfix Flow

> [!CRITICAL]
> **CONFIRM USER INTENT BEFORE PROCEEDING**

Hotfix skips to Construction phase. You must confirm to fix scope before executing.

1. CHECK workspace → invoke `/airsspec-setup` if needed
2. **HALT** - Confirm user's hotfix intent:
   > "You're using Hotfix workflow (fast track to Construction).
   >
   > **What are you fixing?** Please describe the bug, issue, or refactor."
3. WAIT for user to provide hotfix description
4. **HALT** - Confirm scope:
   > "Understood: {brief summary of hotfix}
   >
   > **Hotfix Scope**:
   > - Bug fix / Refactor / Minor improvement
   > - Files affected: {if known}
   >
   > **Do you want to proceed directly to Construction phase?** (yes/no)"
5. (Only after user confirmation) EXECUTE `/airsspec-construction` with hotfix context
6. WAIT for construction workflow to complete
7. Present completion summary:
   > "✅ **Hotfix Complete**
   >
   > **Fixed**: {description of what was fixed}
   >
   > **Verification**: {test results}"
   >
   > **Hotfix applied successfully! 🎉"

---

## Important Rules

- [ ] ALWAYS confirm hotfix intent before proceeding
- [ ] ALWAYS get user confirmation before executing construction
- [ ] NEVER proceed without user saying "yes"
- [ ] If user says "no" or provides more details, adjust and re-confirm
