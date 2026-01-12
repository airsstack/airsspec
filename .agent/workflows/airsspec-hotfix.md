---
description: Hotfix workflow - fast track for bug fixes
---

You are the **Hotfix** workflow for the AirsSpec AI-DLC.

## Instructions

> [!CAUTION]
> **READING GATE — MANDATORY BEFORE ANY IMPLEMENTATION**

### Step 1: Determine Instruction Source
1. CHECK if `.airsspec/agent/` exists
2. SET `$INSTRUCTIONS_SOURCE`:
   - If exists → `.airsspec/agent/`
   - Otherwise → `instructions/`

### Step 2: Read Path Variables
1. READ `$INSTRUCTIONS_SOURCE/core/path-variables.md`
2. **OUTPUT**: `"Path variables loaded. $GUIDELINES_PATH = {resolved value}"`

### Step 3: Read Rust Guidelines (MANDATORY FOR RUST PROJECTS)
1. READ `$GUIDELINES_PATH/rust/project-standard.md`
2. **BEFORE IMPLEMENTING, OUTPUT a recitation:**

> **Pre-Implementation Recitation**
>
> I have read project-standard.md. Key constraints I will follow:
>
> **§4.3 Module Architecture**:
> > [QUOTE the exact rule about mod.rs/lib.rs — what they MUST contain and what is FORBIDDEN]
>
> **§2.1 Import Organization**:
> > [QUOTE the exact 3-layer import pattern]

⚠️ **If you cannot quote these sections, STOP and read the file now.**

### Step 4: Read Hotfix Workflow
1. READ `$INSTRUCTIONS_SOURCE/workflows/hotfix.md`
2. READ `$INSTRUCTIONS_SOURCE/core/validators.md` → Validation commands

### Step 5: Execute
1. EXECUTE the hotfix workflow as documented.

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
