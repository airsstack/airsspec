# AGENTS.md

This file provides context and instructions for AI coding agents working on the AirsSpec project.

## Project Overview

AirsSpec is an AI-native development framework implementing the AI Development Lifecycle (AI-DLC). It provides structured workflows for AI agents to build software through 6 phases: Ingestion, Research, Inception, Design, Planning, and Construction.

## Core Principles

- **Cognitive Cleanroom**: Phase-locked tool constraints prevent context pollution
- **Filesystem as Truth**: All state persisted to disk in `.airsspec/`
- **Convention over Conversation**: Minimize prompting through artifact conventions

## Directory Structure

```
airsspec/
├── docs/                    # Architecture documentation
├── instructions/            # AI agent instructions
│   ├── core/               # Foundation & setup
│   ├── phases/             # Phase-specific guides
│   └── workflows/          # End-to-end workflows
├── .opencode/agent/        # OpenCode custom agents
├── .agent/workflows/       # AntiGravity workflows
└── .airsspec/              # Workspace (when initialized)
```

## Mandatory Reference Documents

> **IMPORTANT**: All agents MUST read these documents before proceeding with any instruction.

| Document | Purpose |
|----------|---------|
| [`instructions/core/path-variables.md`](instructions/core/path-variables.md) | Path variable definitions and reference priority rule |
| [`instructions/core/prompt-guidelines.md`](instructions/core/prompt-guidelines.md) | Prompt engineering guidelines and instruction template |

These foundational documents define:
- All path variables (`$WORKSPACE_ROOT`, `$AIRSSPEC_PATH`, `$INSTRUCTIONS_SOURCE`, etc.)
- Reference priority rule (project-local vs upstream instructions)
- 8 core prompt engineering principles
- Required instruction structure with XML sections

## Key Documentation

| File | Purpose |
|------|---------|
| `instructions/README.md` | Main instruction overview |
| `instructions/core/README.md` | Agent entrypoint |
| `instructions/core/path-variables.md` | **MANDATORY** — Path variables |
| `instructions/core/prompt-guidelines.md` | **MANDATORY** — Prompt guidelines |
| `docs/ai-dlc-phases.md` | 6-phase lifecycle |
| `docs/uow-bolt-spec.md` | UOW & Bolt architecture |
| `docs/user-journey-and-workflow.md` | User experience flow |

## Available Workflows

### AntiGravity (via `/command`)
- `/airsspec` — Main guide
- `/airsspec-feature` — Full AI-DLC cycle
- `/airsspec-hotfix` — Fast track for fixes
- `/airsspec-setup` — Initialize workspace
- `/git-commit` — Conventional Commits

### OpenCode (via `@agent`)
- `@airsspec` — Main orchestrator
- `@airsspec-feature` — Feature workflow
- `@airsspec-hotfix` — Hotfix workflow
- `@git-commit` — Commit helper

## Code Style

- Markdown for all documentation and artifacts
- YAML for configuration (`status.yaml`, `airsspec.toml`)
- Follow existing naming conventions:
  - Files: lowercase with hyphens (`requirements.md`)
  - Directories: lowercase (`uow/`, `bolts/`)

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`

## AI-DLC Phases

| Phase | Artifact | Agent |
|-------|----------|-------|
| Research | `requirements.md` | Researcher |
| Inception | `DAA.md` | Spec-Writer |
| Design | `ADR-*.md` | Architect |
| Planning | `RFC.md`, `bolts/` | Manager |
| Construction | Source code | Builder |

## Working on This Project

1. **READ** `instructions/core/path-variables.md` — Understand path variables
2. **READ** `instructions/core/prompt-guidelines.md` — Understand instruction format
3. Start with `instructions/core/README.md` for orientation
4. Follow `instructions/core/philosophy.md` for principles
5. Use workflows from `instructions/workflows/` for tasks
6. Reference phase guides in `instructions/phases/` for details

## Testing

No automated tests yet. Verify changes by:
- Reviewing markdown renders correctly
- Checking cross-references between files
- Validating YAML syntax in examples

## Important Constraints

- Do NOT modify files outside `.airsspec/` during Construction phase
- Each phase must produce its required artifact before proceeding
- Gate conditions (user approval) must be satisfied between phases
- Bolt scope must be respected — only modify relevant files
