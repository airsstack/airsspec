# Knowledge Library Catalog

**Last Updated**: 2026-01-09

## Overview

This directory contains synthesized, architectural, and strategic documents. These are "warm memory" - distilled insights from research that serve as authoritative references for AI agents throughout the development lifecycle.

---

## Architecture Documents

| File | Description | Status | Phase |
|------|-------------|--------|-------|
| `architecture.md` | Complete Rust crate architecture and system design | **Authoritative** | Design |
| `uow-bolt-spec.md` | UOW & Bolt specification (essentially ADR-001) | **Authoritative** | Design |
| `multi-agent-architecture.md` | Detailed multi-agent system design | **Authoritative** | Design |

---

## Phase Definitions

| File | Description | Usage |
|------|-------------|-------|
| `ai-dlc-phases.md` | Official AI-DLC phase definitions and gate conditions | Reference for all phases |
| `user-journey-and-workflow.md` | User experience and workflow design | Reference for TUI/CLI design |

---

## Strategy Documents

| File | Description | Domain |
|------|-------------|--------|
| `knowledge-base-strategy.md` | Knowledge base architecture and management strategy | Infrastructure |
| `context-compression-strategy.md` | Context compression and memory management | Memory System |
| `cli-tui-design-specification.md` | CLI and TUI interface specifications | UI/UX |
| `ai-workflows.md` | AI workflow orchestration patterns | Agent System |

---

## Document Status

- **Authoritative**: Final, approved documents that define the system
- **Draft**: Work-in-progress documents under review
- **Deprecated**: Historical documents, kept for reference only

---

## Cross-Reference Index

### Architecture Decision Records (ADRs)

The following documents function as Architecture Decision Records:

| ADR | Title | Document | Key Decisions |
|-----|-------|----------|----------------|
| ADR-001 | UOW & Bolt Architecture | `uow-bolt-spec.md` | Cognitive Cleanroom, Fractional Cognition |
| ADR-002 | Crate Architecture | `architecture.md` | Dependency Inversion, Filesystem as Truth |
| ADR-003 | Multi-Agent System | `multi-agent-architecture.md` | Sequential execution, Agent budgets |

### Implementation Priorities

From `architecture.md`, Phase 1 priorities:
1. `airsspec-core` - All traits and types
2. `airsspec-artifacts` - JSONL persistence + validators
3. `airsspec-runtime` - State machine + orchestrator skeleton

---

## Usage Guidelines

### For Researchers
- Read `architecture.md` to understand the technical context
- Reference `uow-bolt-spec.md` when defining UOW structures in `requirements.md`

### For Spec-Writers (Inception Phase)
- Use `uow-bolt-spec.md` as reference for creating `DAA.md`
- Domain Architecture Analysis should align with `architecture.md`

### For Architects (Design Phase)
- `architecture.md` is the primary reference for ADRs
- Ensure all ADRs are consistent with documented decisions
- Update ADRs if architectural decisions evolve

### For Managers (Planning Phase)
- `ai-dlc-phases.md` defines phase transitions and gates
- Use `user-journey-and-workflow.md` for task breakdown
- Implementation priorities are in `architecture.md` Section 13

### For Builders (Construction Phase)
- Follow `uow-bolt-spec.md` for Bolt structure
- Reference `architecture.md` for crate boundaries and dependencies
- Use `cli-tui-design-specification.md` for UI implementation

---

## Maintenance

### When to Update This Catalog
- New documents added to `library/`
- Status changes (draft → authoritative)
- New cross-references needed
- Implementation priorities updated

### Version Control
- All documents should be committed with meaningful messages
- Use Conventional Commits format: `docs(architecture): update crate diagram`

---

**Maintained by**: Librarian Agent
**Catalog Version**: 1.0
**Last Audit**: 2026-01-09
