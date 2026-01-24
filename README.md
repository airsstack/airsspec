# AirsSpec

**Lightweight Spec-Driven Development Framework**

AirsSpec is a minimal, extensible framework for specification-driven development. It provides a simple 3-phase workflow that scales from quick fixes to complex features through a plugin system.

---

## Core Philosophy

> **"Simple by default, powerful when needed."**

AirsSpec avoids the complexity of heavyweight methodologies. Instead of rigid phases and mandatory artifacts, it provides:

- A **3-phase core workflow** that covers 90% of development tasks
- A **plugin hook system** that adds ceremony only when needed
- **Universal compatibility** with any AI coding tool via AGENTS.md generation

---

## Core Workflow

```
┌──────────┐      ┌──────────┐      ┌──────────┐
│   SPEC   │ ───► │   PLAN   │ ───► │  BUILD   │
└──────────┘      └──────────┘      └──────────┘
  What & Why       How & Steps       Execute
```

| Phase | Purpose | Output |
|-------|---------|--------|
| **Spec** | Define what to build and why | `spec.md` |
| **Plan** | Break down into actionable steps | `plan.md` |
| **Build** | Execute the plan | Code changes |

---

## Plugin Architecture

Plugins extend the core workflow through 6 hook points:

```
before:spec  → [SPEC]  → after:spec
before:plan  → [PLAN]  → after:plan
before:build → [BUILD] → after:build
```

### Example Plugins

| Plugin | Hook | Purpose |
|--------|------|---------|
| `@airsspec/research` | `before:spec` | Gather requirements before spec |
| `@airsspec/design` | `before:plan` | Create architecture decisions |
| `@airsspec/gate` | `after:spec`, `after:plan` | Add human approval gates |
| `@airsspec/review` | `after:build` | Code review and quality checks |

Plugins define ceremony implicitly. A simple bug fix uses no plugins (just Spec → Plan → Build). A major feature might use research, design, and review plugins.

---

## Format Strategy

| Purpose | Format | File |
|---------|--------|------|
| Project config | TOML | `.airsspec/config.toml` |
| Spec metadata | YAML frontmatter | `spec.md`, `plan.md` |
| Workflow state | TOON | `state.toon` |

**Why these formats?**

- **TOML**: Human-readable config, well-supported in Rust
- **YAML frontmatter**: Familiar to developers, clean separation of metadata and content
- **TOON**: Token-efficient (~40% smaller than JSON), machine-managed state

---

## Directory Structure

```
.airsspec/
├── config.toml              # Project configuration
├── specs/
│   └── {spec-id}/
│       ├── state.toon       # Workflow state (machine-managed)
│       ├── spec.md          # What & Why
│       └── plan.md          # How & Steps
├── schemas/                 # Artifact JSON schemas (optional)
└── plugins/                 # Local plugin definitions (optional)
```

---

## Spec Lifecycle

```
draft ──► active ──► done ──► archived
             │
             └──► blocked
             └──► cancelled
```

| State | Meaning |
|-------|---------|
| `draft` | Work in progress, not ready |
| `active` | Being worked on |
| `done` | Completed successfully |
| `archived` | Preserved for reference |
| `blocked` | Waiting on dependencies |
| `cancelled` | Abandoned |

---

## Dependencies

Specs can declare dependencies on other specs:

- **Hard dependencies**: Block progression until resolved
- **Soft dependencies**: Informational only, don't block

---

## Status

AirsSpec is in early development. The Rust CLI is planned but not yet implemented.

### Inspiration

- [GitHub Spec-Kit](https://github.com/github/spec-kit) - Structured specification phases
- [OpenSpec](https://github.com/Fission-AI/OpenSpec) - Lightweight, change-centric approach
- [TOON Format](https://github.com/toon-format/toon) - Token-efficient serialization

---

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
