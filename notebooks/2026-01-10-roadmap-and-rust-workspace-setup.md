# Setting Up the AirsSpec Roadmap and Rust Workspace

**Date**: 2026-01-10  
**Topic**: Project Planning, Rust Workspace Setup

---

## The Problem I Was Facing

The AirsSpec project had all its specifications and documentation in place—the architecture was designed, the AI-DLC phases were defined, and the knowledge base was cataloged. But it was all just... markdown. There was no actual Rust code, no implementation path, and no clear roadmap for turning these specs into a working tool.

I needed to:
1. Create a structured roadmap that converts the architecture into actionable Units of Work (UOWs)
2. Set up a Rust workspace that matches the crate architecture I'd already designed

## What I Did

### 1. Discussed Roadmap Granularity

Before jumping into creating files, I had a discussion session to align on key decisions:

- **Roadmap granularity**: 1 Phase = 1 UOW (5 UOWs total following the architecture)
- **Format**: A single `.airsspec/ROADMAP.md` as the high-level overview
- **Source of truth**: The existing `architecture.md` in the knowledge library
- **MVP definition**: Phase 4 complete (working CLI/TUI)

### 2. Created the Implementation Roadmap

I created `.airsspec/ROADMAP.md` which maps out the 5 implementation phases:

| UOW | Phase | Key Crates |
|-----|-------|------------|
| UOW-001 | Foundation | `airsspec-core`, `airsspec-artifacts`, `airsspec-runtime` |
| UOW-002 | Agent System | `airsspec-llm`, `airsspec-tools`, `airsspec-agents` |
| UOW-003 | Knowledge | `airsspec-knowledge` |
| UOW-004 | Interface ★ MVP | `airsspec-cli`, `airsspec-tui`, `airsspec-plugins` |
| UOW-005 | Integration | `airsspec-mcp`, additional providers |

I also broke down Phase 1's `airsspec-core` into 4 sub-phases:
- 1.1 Primitives (error + state)
- 1.2 Contract Layer (artifact + tool)
- 1.3 Cognition Layer (llm + memory + knowledge)
- 1.4 Agent Layer (agent + plugin)

### 3. Set Up the Rust Workspace

With the roadmap in place, I initialized the Rust workspace with only Phase 1 crates:

```
airsspec/
├── Cargo.toml              # Workspace root
└── crates/
    ├── airsspec-core/      # Traits, types, errors
    ├── airsspec-artifacts/ # Validators, persistence
    └── airsspec-runtime/   # Orchestrator, state machine
```

Key workspace decisions:
- **Rust 2024 edition** with MSRV 1.85 (latest stable)
- **Workspace-level dependencies** for consistency across crates
- **Strict clippy lints** (pedantic, nursery, forbid unsafe)
- **No CI/CD yet** - keeping it simple for now

### 4. Verified Everything Compiles

Ran `cargo check --workspace` and got a clean build. Each crate has placeholder code and documentation explaining what it will contain.

## What I Learned

### Start with Discussion, Not Implementation

I was tempted to just start creating files immediately. But taking 10 minutes to discuss options (roadmap granularity, format, sub-phases) saved me from having to refactor later. The async back-and-forth clarified things like MVP definition and whether to detail all phases equally.

### Option B is Usually Right

For most decisions, I offered 2-3 options. "Option B" (the middle ground) was usually the winner:
- Phase 1 crates only vs. all 11 crates → Phase 1 only won
- Detailed sub-phases for all phases vs. just Phase 1 → Just Phase 1 won

Starting minimal and expanding is better than starting big and trimming.

### Workspace Dependencies Save Headaches

Setting up `[workspace.dependencies]` from day one means all crates share the same `tokio`, `serde`, `thiserror` versions. This avoids the "dependency hell" that happens when different crates drift to different versions.

### Cargo.lock Decision

For applications/binaries: commit `Cargo.lock`. For libraries: ignore it. Since AirsSpec will have a CLI binary, we commit the lock file for reproducible builds.

## Files Changed

- `.airsspec/ROADMAP.md` — NEW: Implementation roadmap with 5 phases
- `Cargo.toml` — NEW: Workspace root configuration
- `crates/airsspec-core/Cargo.toml` — NEW: Core crate manifest
- `crates/airsspec-core/src/lib.rs` — NEW: Core crate placeholder with docs
- `crates/airsspec-artifacts/Cargo.toml` — NEW: Artifacts crate manifest
- `crates/airsspec-artifacts/src/lib.rs` — NEW: Artifacts crate placeholder
- `crates/airsspec-runtime/Cargo.toml` — NEW: Runtime crate manifest
- `crates/airsspec-runtime/src/lib.rs` — NEW: Runtime crate placeholder
- `Cargo.lock` — NEW: Generated dependency lock file

## Next Steps

The foundation is laid. Next up:

1. **Implement Sub-Phase 1.1 (Primitives)**: Create `error.rs` and `state/` modules in `airsspec-core`
2. **Define the core error types**: `AirsspecError` enum with variants for each domain
3. **Define state types**: `Phase`, `UowState`, `Transition` structs

The goal is to have `airsspec-core` feature-complete before moving to `airsspec-artifacts` and `airsspec-runtime`.

---

*Having a roadmap feels good. Instead of "build AirsSpec" (overwhelming), I now have "implement `error.rs`" (doable). One module at a time.*
