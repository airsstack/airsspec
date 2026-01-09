# Workspace: AirsSpec

## Overview

AirsSpec is an AI-native development framework implementing the AI Development Lifecycle (AI-DLC). It provides structured workflows for AI agents to build software through 6 phases: Ingestion, Research, Inception, Design, Planning, and Construction. It serves as the technical engine for the AirSDLC Methodology, orchestrating the entire lifecycle through the Model Context Protocol (MCP).

## Project Type

- **Language**: Specification-only (future Rust implementation planned)
- **Framework**: AI-DLC / AirSDLC
- **Build Tool**: None yet (specification phase)

## Structure

```
airsspec/
├── docs/                    # Architecture documentation
│   ├── ai-dlc-phases.md     # Phase definitions
│   ├── architecture.md      # System architecture
│   ├── uow-bolt-spec.md     # UOW & Bolt specification
│   ├── user-journey-and-workflow.md
│   └── multi-agent-architecture.md
├── instructions/            # AI agent instructions (upstream source)
│   ├── core/               # Foundation & setup
│   ├── phases/             # Phase-specific guides
│   └── workflows/          # End-to-end workflows
├── templates/               # Artifact templates
│   ├── uow/                # UOW artifacts (requirements, DAA, ADR, RFC)
│   └── bolt/               # Bolt artifacts (PLAN, TASK)
├── researches/             # Research documents
├── notebooks/              # Developer journals
├── .opencode/agent/        # OpenCode custom agents
├── .agent/workflows/       # AntiGravity workflows
└── .airsspec/              # AirsSpec workspace (this directory)
```

## Entry Points

- **Main Entry**: `instructions/core/README.md` (for AI agents)
- **Documentation**: `docs/user-journey-and-workflow.md` (for humans)
- **Agent Config**: `.opencode/agent/` (OpenCode agents)
- **Workflow Config**: `.agent/workflows/` (AntiGravity workflows)

## Key Resources

- **Core Philosophy**: Spec-Driven Engineering — precise markdown specifications are executable contracts
- **Dogfooding**: AirsSpec uses its own workflows to develop itself
- **Integration**: Supports both OpenCode (@agent) and AntiGravity (/workflow) interfaces

## Ingested Knowledge

### Sources (`.airsspec/sources/`)
Raw research materials gathered during initial research phase:
- `spec-framework-analysis.md` - SDD framework analysis (SpecKit, OpenSpec)
- `agentic-workflows-and-tui.md` - Workflow and TUI research
- `ai-dlc-analysis.md` - AI-DLC concepts and phase definitions
- `rig-integration.md` - LLM provider integration (Rig library)

**Catalog**: See `.airsspec/sources/SOURCES.md` for detailed catalog

### Knowledge Library (`.airsspec/knowledge/library/`)
Synthesized, authoritative documents:
- `architecture.md` - Complete Rust crate architecture (ADR-002)
- `uow-bolt-spec.md` - UOW & Bolt specification (ADR-001)
- `multi-agent-architecture.md` - Multi-agent system design
- `ai-dlc-phases.md` - Official phase definitions
- `user-journey-and-workflow.md` - UX and workflow design
- `cli-tui-design-specification.md` - UI specifications
- `knowledge-base-strategy.md` - Knowledge base architecture
- `context-compression-strategy.md` - Memory management strategy
- `ai-workflows.md` - Workflow orchestration patterns

**Catalog**: See `.airsspec/knowledge/library/LIBRARY.md` for detailed catalog

**Status**: All materials ingested and cataloged as of 2026-01-09

## Conventions

- **Code Style**: Markdown for all documentation and artifacts
- **Configuration**: YAML for configuration files (`status.yaml`, `airsspec.toml`)
- **Naming**: 
  - Files: lowercase with hyphens (`requirements.md`, `adr-001.md`)
  - Directories: lowercase (`uow/`, `bolts/`)
- **Commits**: Conventional Commits format (`feat:`, `fix:`, `docs:`, etc.)

## Notes

- This is currently a specification-only project — no implementation code exists yet
- The project follows the reference priority rule: instructions are copied from `instructions/` to `.airsspec/agent/` during setup
- Self-referential architecture: AirsSpec is built using the same AI-DLC phases and agent integrations it provides
- All AI agent state is persisted to the filesystem in `.airsspec/` (Filesystem as Truth principle)
