# Integrating Rust Guidelines and Reviewer into Construction Phase

**Date**: 2026-01-11  
**Topic**: Construction Phase, Code Review, Language Guidelines

---

## The Problem I Was Facing

When I started working on the first bolt task (BOLT-001-primitives), I realized that the Construction phase instructions were missing something important: **language-specific coding guidelines**.

I have a set of Rust guidelines in `.aiassisted/guidelines/rust/` that cover import organization, module patterns, ADT patterns, and more. But there was no mechanism for the AI agent to load these guidelines during the Construction phase. The agent was implementing code without knowing about the project's coding standards.

Additionally, the Reviewer sub-agent was only mentioned conceptually in the docs but had no detailed workflow. I needed:
1. A way to inject language-specific guidelines into Construction
2. A proper Reviewer workflow that blocks task completion if issues are found
3. Both to work across AntiGravity and OpenCode

## What I Did

### 1. Analyzed Where to Inject Guidelines

I explored three options:
- **Option A**: Add guidelines reference to each Plan file
- **Option B**: Add guidelines loading to Construction phase instructions
- **Option C**: Add a new path variable

I chose **Option B** because it's process-level and applies to all bolts automatically.

### 2. Updated Construction Phase with Rust Guidelines

I modified `.airsspec/agent/phases/construction.md` (project-local) to include:

```markdown
Also read relevant:
- **Rust implementation guidelines** in `$WORKSPACE_ROOT/.aiassisted/guidelines/rust/`:
  - `project-standard.md` — Import organization, module patterns
  - `adt-patterns.md` — Sum types, newtypes, derive macros
  - `dependency-management.md` — When adding dependencies
```

The key insight was keeping the **upstream generic** (in `instructions/`) and putting **project-specific** stuff in `.airsspec/agent/`. Other projects using AirsSpec can define their own guidelines (Python, Node.js, etc.) without affecting the base framework.

### 3. Integrated Reviewer into Construction

Based on the multi-agent-architecture.md spec, the Builder has two sub-agents: Coder and Reviewer. I expanded the Reviewer section to include:

**Scope**: Check only uncommitted changes per Bolt (via `git diff`)
**Process**: Run lint/test → check patterns → block if issues found
**Blocking**: Cannot mark task complete if Reviewer finds issues

For Rust specifically in project-local:
```
- cargo clippy --all-targets --all-features -- -D warnings
- cargo test
- Check patterns against Rust guidelines
```

### 4. Kept Upstream Generic

An important correction during the session: the upstream `instructions/phases/construction.md` should NOT reference `.aiassisted/` — that's project-specific. I updated it to say:

```markdown
**Language-Specific Guidelines**:
- Project-local instructions define the guidelines location
- Apply project-specific patterns and standards
- Use commands appropriate for the project's language/stack
```

## What I Learned

### Separation of Concerns: Upstream vs Project-Local

The `$INSTRUCTIONS_SOURCE` reference priority rule continues to prove its value:
- **Upstream** (`instructions/`): Generic, language-agnostic instructions
- **Project-local** (`.airsspec/agent/`): Project-specific customizations

This allows AirsSpec to work for any tech stack while letting each project define its own standards.

### Reviewer as Blocking Gate

Making the Reviewer a **blocking gate** rather than optional feedback changes the dynamic. Tasks can't be marked complete without passing review. This enforces quality at the workflow level, not just as a suggestion.

### Scope Awareness Matters

Having the Reviewer check only uncommitted changes per Bolt (not the entire codebase) keeps the scope focused. `git diff --name-only` filtered by Bolt context prevents the agent from getting distracted by unrelated files.

### Merged vs Separate Workflows

I considered creating a separate `/airsspec-review` workflow but decided to **merge it into Construction**. This ensures review always happens and can't be skipped by accident.

## Files Changed

- `instructions/phases/construction.md` — Added generic Reviewer workflow (upstream)
- `.airsspec/agent/phases/construction.md` — Added Rust-specific Reviewer with cargo commands and guideline references
- `.agent/workflows/airsspec-construction.md` — Updated Quick Reference
- `.opencode/agent/airsspec-builder.md` — Updated description and Quick Reference

## Next Steps

1. Test the full flow with BOLT-001-primitives
2. Verify the agent loads Rust guidelines and runs `cargo clippy` before marking tasks complete
3. Intentionally introduce a lint error to confirm blocking works
4. Consider adding more language-specific guideline templates for other stacks

---

*This was a good session for establishing the pattern of "generic upstream + project-specific local." It's a clean separation that should scale well as we add support for different languages and frameworks.*
