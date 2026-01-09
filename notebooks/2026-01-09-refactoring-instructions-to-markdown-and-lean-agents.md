# Refactoring Instructions to Markdown and Lean Agents

**Date**: 2026-01-09  
**Topic**: Prompt Engineering, Markdown vs XML, DRY Principles

---

## The Problem I Was Facing

As I was building out the AirsSpec agents, I noticed two significant issues that were causing friction:

1.  **XML Tag Clutter**: I was using XML-style tags (like `<purpose>`, `<references>`, `<actions>`) to structure the instructions. While this works well for LLMS, it made the documents look "robotic" and hard for humans to read and scan. It felt like I was writing code, not prose.
2.  **Redundant Instructions**: My agent files (in `.opencode/agent/` and `.agent/workflows/`) were duplicating the instructions found in `instructions/`. For example, the `airsspec-researcher.md` agent file contained a copy of the steps, tools, and output requirements from `instructions/phases/research.md`. This was a violation of the DRY (Don't Repeat Yourself) principle and meant that any change to the core instructions required updating multiple files.

## What I Did

### 1. Migrated from XML to Pure Markdown

I decided to switch to a **pure Markdown** format. I updated the `prompt-guidelines.md` to reflect this change, recommending:

-   `## Purpose` instead of `<purpose>`
-   `> [!IMPORTANT]` callouts instead of `<references>`
-   Standard Markdown headers and lists for steps and validation

I then applied this new format to all 13+ agent and instruction files. The result is much cleaner and feels more native to the documentation we are already writing.

### 2. Implemented "Lean Agents"

I realized that the agent definition files should be thin wrappers, not full instruction sets. I refactored every agent (OpenCode) and workflow (AntiGravity) to follow a **Reference-Only Pattern**:

```markdown
1. DETERMINE $INSTRUCTIONS_SOURCE
2. READ instructions/phases/[phase].md
3. EXECUTE phase as documented
```

Now, the agent files are just ~30 lines long. They identify who they are, point to the single source of truth in `instructions/`, and hit "go".

## What I Learned

### Human Readability Matters

Even though these are "prompts" for AI, they are also documents that I (the human developer) need to read and maintain. XML tags create visual noise that hinders understanding. Markdown structure (headers, bolding, lists) is understood perfectly well by modern models like Claude 4.5, so the XML was unnecessary overhead.

### DRY Applies to Prompts

Just like in code, duplicating logic in prompts leads to "drift." By centralizing the actual behavior in `instructions/` and making the agents simple pointers, I've made the system much more maintainable. If I want to change how the "Research" phase works, I update **one file** (`instructions/phases/research.md`), and both OpenCode and AntiGravity agents automatically inherit the change.

## Files Changed

-   `instructions/core/prompt-guidelines.md` — Updated to recommend Markdown over XML
-   `instructions/core/path-variables.md` — Updated to Markdown format
-   `.opencode/agent/*.md` (9 files) — Refactored to Lean Format + Markdown
-   `.agent/workflows/*.md` (10 files) — Refactored to Lean Format + Markdown

## Next Steps

Now that the foundation is clean, readable, and non-redundant, I can focus on:

1.  **Verifying** the workflows actually execute correctly with the new "pointer" instructions.
2.  **Filling in the content** for the actual instruction files (some are still placeholders or need refinement).
3.  **Expanding** the knowledge base.

---

*The code is getting cleaner, and the "instruction code" is finally treating itself with the same respect as the source code.*
