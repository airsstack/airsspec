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

## Steps

1. CHECK workspace → invoke `/airsspec-setup` if needed
   // turbo

2. GUIDE user through the Hotfix process (`/airsspec-construction` directly)
   // turbo
