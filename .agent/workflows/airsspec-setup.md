---
description: Setup workflow - bootstraps .airsspec workspace structure
---

You are the **Setup** workflow for the AirsSpec AI-DLC.

## Instructions

> [!IMPORTANT]
> **MANDATORY**: Follow the reference priority rule and read your instructions.

1. DETERMINE `$INSTRUCTIONS_SOURCE`:
   - If `.airsspec/agent/` exists → use `.airsspec/agent/`
   - Otherwise → use `instructions/`

2. READ these documents in order:
   - `$INSTRUCTIONS_SOURCE/core/path-variables.md`
   - `$INSTRUCTIONS_SOURCE/core/prompt-guidelines.md`
   - `$INSTRUCTIONS_SOURCE/core/workspace-explore.md`
   - `$INSTRUCTIONS_SOURCE/core/workspace-setup.md`

3. EXECUTE the workspace setup as documented.

4. COPY instruction files to enable project-local customization:
   // turbo
   ```bash
   mkdir -p .airsspec/agent
   cp -r instructions/core/ .airsspec/agent/core/
   cp -r instructions/phases/ .airsspec/agent/phases/
   cp -r instructions/workflows/ .airsspec/agent/workflows/
   ```

## Quick Reference

| Item | Value |
|------|-------|
| **Phase** | Setup (Pre-workflow) |
| **Output** | Complete `.airsspec/` directory structure |
| **Next** | `/airsspec-feature` or `/airsspec-hotfix` |
