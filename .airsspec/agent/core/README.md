# AirsSpec AI Instructions

This directory contains AI instructions for developing the AirsSpec project using the **AI-DLC (AI Development Lifecycle)** methodology.

## Purpose

These instructions guide AI agents (and developers working with AI) to:
- Set up and maintain the `.airsspec/` workspace structure
- Follow phase-gated development workflows
- Produce consistent, auditable artifacts
- Operate within the "Cognitive Cleanroom" constraints

## Directory Structure

```
instructions/
├── README.md               ← You are here
├── core/                   ← Foundation & setup
│   ├── README.md           ← Main entrypoint (START HERE)
│   ├── philosophy.md       ← AirSDLC principles
│   ├── workspace-explore.md
│   ├── workspace-setup.md
│   ├── memory.md
│   └── constraints.md
├── phases/                 ← Phase-specific instructions
│   ├── ingestion.md
│   ├── research.md
│   ├── inception.md
│   ├── design.md
│   ├── planning.md
│   └── construction.md
└── workflows/              ← End-to-end workflows
    ├── feature.md
    └── hotfix.md
```

## Quick Start

**→ Start with [core/README.md](./core/README.md)**

This is the main entrypoint that guides you through:
1. Understanding the philosophy
2. Exploring your project
3. Setting up the workspace
4. Choosing a workflow

## Reference Documents

For detailed specifications, refer to:
- [docs/ai-dlc-phases.md](../docs/ai-dlc-phases.md) — Phase & artifact definitions
- [docs/user-journey-and-workflow.md](../docs/user-journey-and-workflow.md) — User journey details
- [docs/multi-agent-architecture.md](../docs/multi-agent-architecture.md) — Agent system design
- [docs/architecture.md](../docs/architecture.md) — Technical architecture
