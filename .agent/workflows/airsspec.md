---
description: AirsSpec AI-DLC Guide - helps you navigate the AI Development Lifecycle
---

# AirsSpec Guide

Welcome to AirsSpec! This workflow helps you understand and use the AI Development Lifecycle.

## What is AirsSpec?

AirsSpec is an AI-native development framework with 6 phases:
1. **Ingestion** — Load knowledge sources
2. **Research** — Create requirements
3. **Inception** — Define domain model
4. **Design** — Make architecture decisions
5. **Planning** — Create execution strategy
6. **Construction** — Implement code

## Getting Started

1. First, check if `.airsspec/` directory exists in this project.

2. If not, run `/airsspec-setup` to initialize the workspace.

3. Choose your workflow:
   - **New feature** → Run `/airsspec-feature`
   - **Bug fix / Refactor** → Run `/airsspec-hotfix`

## Available Workflows

| Command | Purpose |
|---------|---------|
| `/airsspec-setup` | Initialize workspace |
| `/airsspec-feature` | Full AI-DLC cycle |
| `/airsspec-hotfix` | Fast track to Construction |
| `/airsspec-research` | Research phase only |
| `/airsspec-inception` | Inception phase only |
| `/airsspec-design` | Design phase only |
| `/airsspec-planning` | Planning phase only |
| `/airsspec-construction` | Construction phase only |

## Core Principles

Read `instructions/core/philosophy.md` for:
- **Cognitive Cleanroom** — Phase-locked constraints
- **Filesystem as Truth** — State on disk
- **Convention over Conversation** — Minimal prompting

## Need Help?

- Main instructions: `instructions/README.md`
- Core setup: `instructions/core/README.md`
- Phase details: `instructions/phases/`
- Workflow guides: `instructions/workflows/`
