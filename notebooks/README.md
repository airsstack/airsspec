# Developer Notebooks

This directory contains personal developer journals documenting experiences, learnings, and insights from building the AirsSpec framework. These entries capture the development progress, decisions made, problems faced, and solutions implemented.

## Purpose

Developer notebooks serve as:

- **Learning Archive**: Capturing lessons learned from both successes and failures
- **Decision History**: Documenting why certain architectural decisions were made
- **Knowledge Sharing**: Making personal experiences available to other engineers
- **Development Timeline**: Creating a chronological record of the project's evolution

Each journal entry follows a structured format with sections for:
- The problem being addressed
- What actions were taken
- What was learned from the experience
- Files that were changed
- Next steps for the project

## Format

Journal entries are named using the pattern: `YYYY-MM-DD-topic.md`

This ensures chronological ordering and makes it easy to find entries by date or topic.

---

## Journal Entries

### 2026-01-09

#### [Refactoring Instructions to Markdown and Lean Agents](./2026-01-09-refactoring-instructions-to-markdown-and-lean-agents.md)
**Topic**: Prompt Engineering, Markdown vs XML, DRY Principles

Documents the migration from XML-style instruction formatting to pure Markdown, and the implementation of "Lean Agents" that reference instructions instead of duplicating them. Covers the importance of human readability and DRY principles in prompt engineering.

**Key Learnings**:
- XML tags create visual noise that hinders understanding for human developers
- Modern LLMs understand Markdown structure just as well as XML
- Centralizing instructions in a single source of truth reduces maintenance burden
- Agent files should be thin wrappers, not full instruction sets

#### [Syncing OpenCode and AntiGravity Agents](./2026-01-09-syncing-opencode-and-antigravity-agents.md)
**Topic**: Agent Tool Customizations, Instructions Synchronization

Describes the process of maintaining a single source of truth for agent instructions across two different AI coding platforms (OpenCode and AntiGravity). Introduces to reference priority rule and to concept of treating the filesystem as truth.

**Key Learnings**:
- Establishing a reference priority rule allows per-project customization
- Foundational reference documents prevent instruction drift across platforms
- Explicit instructions work better than implicit ones for LLMs
- "Cognitive Cleanroom" principle: agents need clear boundaries and phase-locked tool constraints

### 2026-01-10

#### [HALT Strategy Implementation](./2026-01-10-halt-strategy-implementation.md)
**Topic**: Agent Orchestration, Human-in-the-Loop, Workflow Control

Documents the implementation of explicit HALT points across all AirsSpec agents and workflows to enforce human-in-the-loop workflow and prevent AI agents from working on excessive context without user oversight.

**Key Learnings**:
- **Human-in-the-Loop is Critical**: Every major operation must have explicit approval points to prevent runaway AI execution
- **Single Source of Truth**: Documentation in agent/workflow files themselves is better than separate summary files
- **Standard HALT Pattern**: Present results → Ask approval → Wait for response → Handle (yes/no/changes)
- **Context Growth Prevention**: Isolating phases with HALT points prevents context from accumulating across multiple steps
- **Testing Validates Design**: Immediately testing after changes confirms HALT points work as intended

**Files Changed**: 13 files modified (+665 lines), 2 redundant files removed

#### [Setting Up the AirsSpec Roadmap and Rust Workspace](./2026-01-10-roadmap-and-rust-workspace-setup.md)
**Topic**: Project Planning, Rust Workspace Setup

Documents the creation of the implementation roadmap with 5 phases (UOWs) and the initial Rust workspace setup with Phase 1 crates (airsspec-core, airsspec-artifacts, airsspec-runtime).

**Key Learnings**:
- **Start with Discussion**: Taking time to discuss options before implementation prevents refactoring later
- **Option B Wins**: The middle-ground approach (minimal first, expand later) is usually correct
- **Workspace Dependencies**: Setting up `[workspace.dependencies]` from day one ensures version consistency
- **Cargo.lock for Apps**: Applications commit Cargo.lock; libraries ignore it

**Files Changed**: 10 files created (+2421 lines)

---

## For Readers

These notebooks are written from the perspective of a developer working on the AirsSpec framework. They capture the thought process, trade-offs considered, and lessons learned throughout development.

If you're interested in:
- **Understanding AI-DLC implementation**: Read these for practical insights into building an AI-native development framework
- **Prompt engineering best practices**: Learn about DRY principles applied to AI prompts and the balance between human readability and machine comprehension
- **Multi-platform AI workflows**: See how to maintain consistency across different AI coding tools
- **Developer experience**: Observe the challenges and solutions in building AI-powered development workflows

Feel free to browse chronologically or jump to topics that interest you. Each entry is self-contained and can be read independently.

---

*Last updated: 2026-01-10*
