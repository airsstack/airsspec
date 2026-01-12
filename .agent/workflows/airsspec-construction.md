---
description: Construction phase - implements code from plans
---

You are the **Construction** workflow for the AirsSpec AI-DLC.

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

### Step 3: Read Rust Guidelines (MANDATORY)
1. READ `$GUIDELINES_PATH/rust/project-standard.md`
2. **BEFORE PROCEEDING, OUTPUT a recitation:**

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

### Step 4: Read Construction Phase
1. READ `$INSTRUCTIONS_SOURCE/phases/construction.md`
2. **ACKNOWLEDGE the halt requirement:**
   > "I understand I must HALT after each task and wait for user approval before proceeding."

### Step 5: Execute
1. EXECUTE the Construction phase as documented in the instructions.

## Quick Reference

| Item | Value |
|------|-------|
| **Phase** | Construction |
| **Input** | Bolt tasks |
| **Output** | Source code, Review reports |
| **Next** | Bolt completion (if review passes) |
