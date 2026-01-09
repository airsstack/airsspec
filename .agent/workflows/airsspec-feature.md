---
description: Feature workflow - orchestrates full AI-DLC cycle
---

You are the **Feature** workflow for the AirsSpec AI-DLC.

## Instructions

> [!IMPORTANT]
> **MANDATORY**: Follow the reference priority rule and read your instructions.

1. DETERMINE `$INSTRUCTIONS_SOURCE`:
   - If `.airsspec/agent/` exists → use `.airsspec/agent/`
   - Otherwise → use `instructions/`

2. READ these documents in order:
   - `$INSTRUCTIONS_SOURCE/core/path-variables.md`
   - `$INSTRUCTIONS_SOURCE/core/prompt-guidelines.md`
   - `$INSTRUCTIONS_SOURCE/workflows/feature.md` ← **Your workflow guide**

3. ORCHESTRATE the full AI-DLC cycle as documented.

## Quick Reference

| Item | Value |
|------|-------|
| **Workflow** | Full AI-DLC cycle |
| **Phases** | Research → Inception → Design → Planning → Construction |
| **Output** | Complete implementation with full artifact trail |

## Steps

1. CHECK workspace → invoke `/airsspec-setup` if needed
   // turbo

2. GUIDE user through the Research phase (`/airsspec-research`)
   // turbo

3. GUIDE user through the Inception phase (`/airsspec-inception`)
   // turbo

4. GUIDE user through the Design phase (`/airsspec-design`)
   // turbo

5. GUIDE user through the Planning phase (`/airsspec-planning`)
   // turbo

6. GUIDE user through the Construction phase (`/airsspec-construction`)
   // turbo
