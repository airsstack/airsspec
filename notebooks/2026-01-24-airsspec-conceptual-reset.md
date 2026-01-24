# AirsSpec Conceptual Reset

**Date**: 2026-01-24

---

## Context

After several weeks of developing AirsSpec based on the AI-DLC (AI-Driven Development Lifecycle) methodology, I stepped back to evaluate the framework's direction. The implementation had grown increasingly complex, and I wasn't satisfied with the developer experience.

---

## Problems with the Previous Approach

### 1. Too Many Phases

The 6-phase lifecycle (Ingestion → Research → Inception → Design → Planning → Construction) was comprehensive but overwhelming. Most development tasks don't need this level of ceremony.

### 2. Rigid Structure

Every workflow had to pass through all phases, even when unnecessary. A simple bug fix shouldn't require a DAA (Domain Architecture Analysis) or ADR (Architecture Decision Record).

### 3. Configuration Complexity

The system had 15+ path variables for artifact locations, deep instruction chaining between agents, and complex state management. This made the framework hard to understand and extend.

### 4. Opinionated Agent Integration

The framework was tightly coupled to specific AI tools (OpenCode, AntiGravity) rather than being tool-agnostic.

---

## Research Phase

I studied three successful open-source projects for inspiration:

### GitHub Spec-Kit (64.7k stars)

- 5 slash commands: constitution → specify → plan → tasks → implement
- Intent-driven, focuses on specification quality
- Strong emphasis on structured specifications

**Takeaway**: The phased approach works, but phases should be optional.

### OpenSpec (19.3k stars)

- Only 3 commands: proposal → apply → archive
- Lightweight, brownfield-first design
- Change-centric rather than document-centric

**Takeaway**: Simplicity wins. Most tasks need minimal ceremony.

### TOON Format (22.1k stars)

- Token-Oriented Object Notation for LLM contexts
- ~40% fewer tokens than JSON
- Human-readable, machine-efficient

**Takeaway**: Format choice matters for AI workflows. Token efficiency improves context utilization.

---

## Key Decisions

### 1. Three-Phase Core Workflow

```
Spec → Plan → Build
```

This covers 90% of development tasks. Additional phases become plugins, not requirements.

### 2. Plugin Hook Architecture

Six hook points allow plugins to inject behavior:

```
before:spec  → [SPEC]  → after:spec
before:plan  → [PLAN]  → after:plan
before:build → [BUILD] → after:build
```

This makes ceremony opt-in. A simple fix uses the bare workflow. A complex feature can load research, design, and review plugins.

### 3. Format Strategy

| Purpose | Format | Rationale |
|---------|--------|-----------|
| Project config | TOML | Human-readable, Rust-native |
| Spec metadata | YAML frontmatter | Developer-familiar |
| Workflow state | TOON | Token-efficient for AI contexts |

### 4. Dependencies Model

- **Hard dependencies**: Block progression (must be resolved)
- **Soft dependencies**: Informational only (noted but don't block)

### 5. Universal Compatibility

Generate AGENTS.md for cross-tool compatibility. AirsSpec should work with any AI coding assistant, not just specific tools.

### 6. Implementation in Rust

Clean slate. The CLI will be implemented in Rust for performance and reliability.

---

## What Was Removed

All previous implementation artifacts were deleted:

- `.agent/` - AntiGravity workflow definitions
- `.opencode/` - OpenCode agent definitions
- `.airsspec/` - Old workspace structure
- `docs/` - Previous documentation
- `instructions/` - Agent instruction files
- `templates/` - Artifact templates
- `crates/` - Rust workspace (incomplete)
- `researches/` - Research artifacts
- `Cargo.toml`, `Cargo.lock` - Rust configuration
- `AGENTS.md`, `AIRSDLC.md` - Old methodology docs

---

## What Remains

- `notebooks/` - Developer journals (including this entry)
- `README.md` - Rewritten with new concept
- `LICENSE-APACHE`, `LICENSE-MIT` - Dual license
- `.vscode/` - Editor configuration
- `.gitignore` - Git ignore rules

---

## Next Steps

1. **Formalize the specification** - Document the plugin interface, state machine, and CLI commands
2. **Design the TOON schema** - Define the workflow state structure
3. **Implement the Rust CLI** - Start with core workflow, add plugin system later
4. **Create example plugins** - Demonstrate the hook system with research, design, review plugins

---

## Reflection

This reset was necessary. The previous approach tried to encode every possible workflow into the core framework. The new approach trusts developers to add ceremony when they need it.

**Simplicity is a feature, not a limitation.**
