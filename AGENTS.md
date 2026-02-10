# AGENTS.md

> AI Agent Instructions for AirsSpec Development

---

## Project Overview

**AirsSpec** is a lightweight, MCP-first spec-driven development framework built in Rust.

**Philosophy:** *"Simple by default, powerful when needed."*

**Core Workflow:**
```
SPEC (What & Why) → PLAN (How & Steps) → BUILD (Execute)
```

**Architecture:** MCP server that exposes spec workflow primitives to AI coding tools (OpenCode, Claude Code, Cursor) via stdio transport.

---

## Project Structure

```
airsspec/
├── Cargo.toml                  # Workspace root (pending)
├── README.md                   # Project overview
├── AGENTS.md                   # This file
├── plans/                      # Architecture documentation
├── .aiassisted/                # AI development guidelines
│   ├── guidelines/             # Coding standards
│   ├── instructions/           # Workflow instructions
│   └── prompts/                # Prompt templates
└── crates/                     # Rust workspace (pending)
    ├── airsspec-core/          # Domain logic, traits, no I/O
    ├── airsspec-mcp/           # MCP server implementation
    ├── airsspec-tui/           # TUI components (ratatui)
    └── airsspec-cli/           # Binary entry point
```

---

## Architecture Plans (Lazy-Load References)

Detailed architecture documentation is in `plans/`. **Load only when needed:**

| Document | When to Load |
|----------|--------------|
| `plans/01-architecture-overview.md` | When you need high-level design, technology stack, or MCP integration model |
| `plans/02-crate-breakdown.md` | When working on crate structure, Cargo.toml configs, or module layouts |
| `plans/03-domain-models.md` | When implementing Rust types, traits, state machine, or validation |
| `plans/04-mcp-server.md` | When implementing MCP tools, resources, prompts, or logging |
| `plans/05-implementation-roadmap.md` | When planning tasks, checking phase dependencies, or estimating effort |

**Usage Pattern:**
- Do NOT read all plans at session start
- Load specific plan when entering that implementation phase
- Reference plan document in commit messages when implementing from it

---

## Development Guidelines (Lazy-Load References)

Guidelines are in `.aiassisted/`. **Load based on context:**

### Rust Coding Standards

| Document | When to Load |
|----------|--------------|
| `.aiassisted/guidelines/rust/microsoft-rust-guidelines.md` | Before writing any Rust code - core coding standards |
| `.aiassisted/guidelines/rust/rust-dependency-injection-dip-guide.md` | When designing traits, abstractions, or module boundaries |
| `.aiassisted/guidelines/rust/dependency-management.md` | When adding dependencies to Cargo.toml |

### Documentation Standards

| Document | When to Load |
|----------|--------------|
| `.aiassisted/guidelines/documentation/diataxis-guidelines.md` | When writing user documentation |
| `.aiassisted/guidelines/documentation/documentation-quality-standards.md` | When reviewing or creating docs |
| `.aiassisted/guidelines/documentation/task-documentation-standards.md` | When documenting task completion |

### Workflow Instructions

| Document | When to Load |
|----------|--------------|
| `.aiassisted/instructions/rust.instructions.md` | Before any Rust implementation session |
| `.aiassisted/instructions/conventional-commits.instructions.md` | Before making git commits |
| `.aiassisted/instructions/multi-project-memory-bank.instructions.md` | When using memory bank commands |
| `.aiassisted/instructions/ai-prompt-engineering-safety-best-practices.instructions.md` | When creating prompts or AI interactions |
| `.aiassisted/instructions/setup-agents-context.instructions.md` | When setting up new agent contexts |

### Prompts

| Document | When to Load |
|----------|--------------|
| `.aiassisted/prompts/git.commit.prompt.md` | When committing changes |

---

## Key Technical Decisions

These decisions are final. Do not re-debate unless explicitly requested:

1. **MCP-First Architecture**: No standalone CLI with LLM integrations. AirsSpec is an MCP server.
2. **4-Crate Structure**: `airsspec-core` (pure domain), `airsspec-mcp` (I/O), `airsspec-tui` (UI), `airsspec-cli` (binary)
3. **Spec ID Format**: `{unix-timestamp}-{title-slug}` (e.g., `1737734400-user-auth`)
4. **Slug Max Length**: 50 characters
5. **State Module Name**: `state/` (not `state_machine/`)
6. **Error Handling**: Permissive - collect all errors, report together
7. **Logging**: Daily rotation + per-session JSONL files in `.airsspec/logs/`
8. **MCP Library**: `airsprotocols-mcp` v1.0.0-rc.2

---

## Rust Patterns (Quick Reference)

### Import Organization (Mandatory)

```rust
// Layer 1: Standard library
use std::collections::HashMap;

// Layer 2: External crates
use serde::{Deserialize, Serialize};

// Layer 3: Internal crates/modules
use crate::domain::Spec;
```

### Module Structure

- `mod.rs` contains ONLY declarations and re-exports
- No implementation code in `mod.rs`
- One type per file when possible

### Error Handling

```rust
#[derive(Error, Debug)]
pub enum SpecError {
    #[error("spec not found: {0}")]
    NotFound(SpecId),
    // ...
}
```

### Builder Pattern

```rust
let spec = SpecBuilder::new()
    .title("User Authentication")
    .description("...")
    .build()?;
```

---

## CLI Commands

```bash
# Initialize workspace (TUI wizard)
airsspec init

# Start MCP server (stdio transport)
airsspec mcp [--debug]

# Run validation with TUI reporter
airsspec validate
```

---

## MCP Tools (12 total)

| Category | Tools |
|----------|-------|
| Spec | `spec_create`, `spec_update`, `spec_transition`, `spec_list`, `spec_status`, `spec_check_dependencies` |
| Plan | `plan_create`, `plan_update`, `plan_step_complete` |
| Build | `build_start`, `build_update`, `build_complete` |

---

## Implementation Phases

| Phase | Description | Estimated Time |
|-------|-------------|----------------|
| 1 | Project Setup (Cargo workspace) | 1-2 hours |
| 2 | Core Domain Models | 3-4 hours |
| 3 | CLI Skeleton & Init Wizard | 2-3 hours |
| 4 | Validation Engine | 2-3 hours |
| 5 | MCP Server | 6-8 hours |
| 6 | Integration & Documentation | 2-3 hours |

**Current Status:** Phase 1 pending. Architecture complete.

---

## Commit Convention

Use conventional commits. Load `.aiassisted/instructions/conventional-commits.instructions.md` for details.

Quick reference:
- `feat(scope):` - New feature
- `fix(scope):` - Bug fix
- `docs:` - Documentation
- `chore:` - Maintenance
- `refactor(scope):` - Code restructuring

---

## Quality Gates

Before marking any implementation complete:

1. `cargo build --workspace` succeeds
2. `cargo clippy --workspace --all-targets -- -D warnings` passes
3. `cargo test --workspace` passes
4. No `unsafe` code (unless absolutely necessary)
5. No `dyn` trait objects (use generics)

---

## Memory Bank

Local memory bank at `.memory-bank/` tracks session context. It is gitignored.

Commands:
- `show-memory-bank airsspec` - View current state
- `update-memory-bank airsspec` - Refresh context files
- `show-tasks airsspec` - List tasks
- `add-task airsspec [name]` - Create task

Load `.aiassisted/instructions/multi-project-memory-bank.instructions.md` for full documentation.
