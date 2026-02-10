# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

AirsSpec is a lightweight, MCP-first spec-driven development framework built in Rust. It provides a 3-phase workflow (Spec → Plan → Build) that scales from quick fixes to complex features through a plugin system.

**Philosophy:** "Simple by default, powerful when needed."

**Architecture:** MCP server that exposes spec workflow primitives to AI coding tools via stdio transport.

## Build Commands

```bash
# Build
cargo build --workspace

# Lint (strict - all warnings treated as errors in CI)
cargo clippy --workspace --all-targets

# Test
cargo test --workspace

# Format
cargo fmt --all

# Single crate operations
cargo test -p airsspec-core
cargo clippy -p airsspec-core
cargo doc -p airsspec-core --no-deps
```

## Project Structure

4-crate modular monolith workspace:

- **airsspec-core** - Pure domain logic (no I/O, no tokio). Models, traits, state machine, validation rules.
- **airsspec-mcp** - MCP server implementation, I/O layer, storage implementations.
- **airsspec-tui** - Terminal UI components (ratatui). Stateless presentation layer.
- **airsspec-cli** - Thin CLI entry point (clap). No business logic.

## Key Technical Decisions

These are final decisions - do not re-debate:

1. **MCP-First**: AirsSpec is an MCP server, not a standalone CLI with LLM integrations
2. **Spec ID Format**: `{unix-timestamp}-{title-slug}` (e.g., `1737734400-user-auth`)
3. **Slug Max Length**: 50 characters
4. **Error Handling**: Permissive - collect all errors, report together
5. **No `dyn` trait objects**: Use generics for static dispatch
6. **No `unsafe` code**: Unless absolutely necessary with documentation
7. **MCP Library**: `airsprotocols-mcp`

## Rust Patterns

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

### Error Types

```rust
#[derive(Error, Debug)]
pub enum SpecError {
    #[error("spec not found: {0}")]
    NotFound(SpecId),
}
```

### Builder Pattern

```rust
let spec = SpecBuilder::new()
    .title("User Authentication")
    .description("...")
    .build()?;
```

## Quality Gates

Before marking any implementation complete:

1. `cargo build --workspace` succeeds
2. `cargo clippy --workspace --all-targets` passes (zero warnings)
3. `cargo test --workspace` passes
4. No `unsafe` code (unless documented)
5. No `dyn` trait objects (use generics)

## Lazy-Load Architecture Documentation

Detailed architecture docs are in `plans/`. Load only when needed:

| Document | When to Load |
|----------|--------------|
| `plans/01-architecture-overview.md` | High-level design, technology stack, MCP integration |
| `plans/02-crate-breakdown.md` | Crate structure, Cargo.toml configs, module layouts |
| `plans/03-domain-models.md` | Rust types, traits, state machine, validation |
| `plans/04-mcp-server.md` | MCP tools, resources, prompts, logging |
| `plans/05-implementation-roadmap.md` | Task planning, phase dependencies |

## Implementation Status

**Current Phase:** Phase 1 Complete (Project Setup)

**Next Phase:** Phase 2 - Core Domain Models

Phases overview:
1. ✅ Project Setup (workspace structure)
2. ⏳ Core Domain Models (Spec, Plan, State types)
3. ⏳ CLI Skeleton & TUI Init Wizard
4. ⏳ Validation Engine
5. ⏳ MCP Server
6. ⏳ Integration & Documentation

## Commit Convention

Use conventional commits:

```
<type>(<scope>): <description>

Types: feat, fix, docs, chore, refactor, perf, test, build, ci, style, revert
```

Examples:
- `feat(core): add SpecId newtype with validation`
- `fix(mcp): handle empty spec list correctly`
- `docs: update architecture overview`

## CLI Commands (Planned)

```bash
airsspec init          # Initialize workspace (TUI wizard)
airsspec mcp [--debug] # Start MCP server (stdio)
airsspec validate      # Run validation with TUI reporter
```

## MCP Tools (12 total)

| Category | Tools |
|----------|-------|
| Spec | `spec_create`, `spec_update`, `spec_transition`, `spec_list`, `spec_status`, `spec_check_dependencies` |
| Plan | `plan_create`, `plan_update`, `plan_step_complete` |
| Build | `build_start`, `build_update`, `build_complete` |

## Workspace Dependencies

All dependencies managed at workspace level in root `Cargo.toml`. Key dependencies:
- `tokio` (async runtime)
- `serde`, `serde_json`, `toml`, `serde_yaml` (serialization)
- `clap` (CLI parsing)
- `ratatui`, `crossterm` (TUI)
- `thiserror`, `anyhow` (error handling)
- `tracing` (logging)

Minimum Rust version: 1.88, Edition 2024.

## Guidelines Reference

Rust coding standards in `.aiassisted/guidelines/rust/`. Key files:
- `microsoft-rust-guidelines.md` - Core coding standards
- `rust-dependency-injection-dip-guide.md` - Trait-based abstractions
- `rust-policy-guide.md` - Zero warnings, static dispatch, testing requirements

## Memory Bank (Session Continuity)

A multi-project memory bank at `.memory-bank/` (gitignored) tracks context across sessions. AI agents rely on this to maintain continuity between sessions.

**Structure:**
- `current-context.md` - Active sub-project tracker
- `workspace/` - Shared workspace context (project-brief, shared-patterns, architecture)
- `sub-projects/airsspec/` - Project-specific context (active-context, progress, tasks)
- `templates/docs/` - Templates for technical debt, knowledge docs, ADRs

**Key Commands:**
```bash
show-memory-bank airsspec      # View current state
update-memory-bank airsspec    # Refresh context files
switch-context airsspec        # Set active sub-project
show-tasks airsspec            # List all tasks
add-task airsspec [name]       # Create new task
save-context [description]     # Save context snapshot
```

**Task Management:**
- Each task = one directory with two files: `<task-id>.md` (objectives) and `<task-id>.plans.md` (implementation)
- Single action per task rule - do one thing, do it right
- Tasks must reference relevant ADRs and Knowledge documents

**Full documentation:** `.aiassisted/instructions/multi-project-memory-bank.instructions.md`
