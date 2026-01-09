# Syncing OpenCode and AntiGravity Agents

**Date**: 2026-01-09  
**Topic**: Agent Tool Customizations, Instructions Synchronization

---

## The Problem I Was Facing

I've been building out the AirsSpec framework with two different AI coding tools: **OpenCode** and **AntiGravity**. Both are great tools, but the challenge was that each has its own way of defining custom agents/workflows:

- OpenCode uses `@agent_name` with agents defined in `.opencode/agent/`
- AntiGravity uses `/workflow` with workflows defined in `.agent/workflows/`

The problem? I had to maintain two separate sets of instructions, and they were starting to drift apart. When I made improvements to one, I'd forget to update the other. This was becoming a maintenance nightmare.

## What I Did

### 1. Created Foundational Reference Documents

The first thing I realized was that I needed a **single source of truth** for common patterns. So I created two mandatory reference documents in `instructions/core/`:

- **`path-variables.md`** — Defines all the path variables like `$WORKSPACE_ROOT`, `$AIRSSPEC_PATH`, `$INSTRUCTIONS_SOURCE`, etc. This way, agents don't use hardcoded paths anymore.

- **`prompt-guidelines.md`** — Codifies prompt engineering best practices: use explicit action verbs (READ, WRITE, CREATE), structure with XML-style sections, always include validation checklists, etc.

### 2. The Reference Priority Rule

One of the key insights was establishing a **reference priority rule**. Before the workspace is set up, agents use the upstream `instructions/` directory. After setup, instructions are copied to `.airsspec/agent/`, and that takes priority. This allows per-project customization without affecting the base framework.

```
IF $PROJECT_AGENT_PATH exists:
    $INSTRUCTIONS_SOURCE = $PROJECT_AGENT_PATH
ELSE:
    $INSTRUCTIONS_SOURCE = $CORE_INSTRUCTIONS_PATH
```

### 3. Updated All Agents to Reference These Documents

Every agent now starts with a `<references>` section that points to the mandatory documents:

```markdown
<references>
MANDATORY: Read these documents before proceeding.
- `$INSTRUCTIONS_SOURCE/core/path-variables.md`
- `$INSTRUCTIONS_SOURCE/core/prompt-guidelines.md`
</references>
```

This ensures consistency across all agents, regardless of which tool invokes them.

### 4. The Setup Workflow

I also synchronized the `/airsspec-setup` workflow between both platforms. This workflow:

1. Checks if `.airsspec/` exists
2. Explores the workspace to generate `WORKSPACE.md`
3. Creates the directory structure
4. Copies instruction files to `.airsspec/agent/` for customization
5. Creates configuration files

Both OpenCode and AntiGravity now produce identical output when setting up a workspace.

## What I Learned

### Agents Need Clear Boundaries

The "Cognitive Cleanroom" principle is real. When agents have access to everything, they tend to wander. By locking tools per phase and having explicit constraints, the agents stay focused.

### Filesystem as Truth

One thing that's been working well is treating the filesystem as the source of truth. Agents shouldn't rely on conversation memory—they should read from and write to disk. This makes everything:
- Auditable
- Recoverable
- Human-verifiable

### Explicit > Implicit

Claude (and LLMs in general) respond better to explicit instructions. "READ the file at $X" works better than "you should read the philosophy document." Direct imperatives, specific paths, validation checklists.

## Files Changed

- `.agent/workflows/airsspec.md` — Main orchestrator
- `.agent/workflows/airsspec-setup.md` — Setup workflow
- `.opencode/agent/airsspec.md` — Main agent
- `.opencode/agent/airsspec-setup.md` — NEW: Setup agent
- `.opencode/agent/airsspec-*.md` — All phase-specific agents
- `instructions/core/path-variables.md` — NEW: Path definitions
- `instructions/core/prompt-guidelines.md` — NEW: Prompt engineering standards
- `AGENTS.md` — Cross-platform agent instructions

## Next Steps

Now that the agents are synchronized, I can:

1. Test the full AI-DLC cycle with both tools
2. Add more playbooks to `knowledge/playbooks/`
3. Start using the framework to build... the framework itself (dogfooding!)

The goal is that whether I use OpenCode or AntiGravity, the experience should be identical. One framework, two frontends.

---

*This was a good refactoring session. The instructions are cleaner, the agents are synchronized, and I have a solid foundation for the AI-DLC workflow.*
