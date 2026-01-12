---
id: UOW-002
title: Agent System
version: "1.0"
status: draft
author: airsspec-orchestrator
created_at: 2026-01-12
priority: high
phase: Research
---

# UOW-002: Agent System

**Goal**: Create the first working agent that can reason and execute tools.

---

## Problem Statement

With the Foundation Layer (UOW-001) complete, AirsSpec has the core abstractions (`LLMProvider`, `Tool`, `Agent` traits) but no concrete implementations. To validate the architecture and enable actual AI-DLC workflows, we need:

1. A working LLM provider integration via **OpenRouter** (unified API for 200+ models)
2. Core tools that agents use to interact with the filesystem and external world
3. A first agent implementation (Researcher) that demonstrates the complete flow
4. Security boundaries for tool execution

Without working agents, AirsSpec remains a specification framework with no execution capability.

---

## Success Criteria

1. `airsspec-llm` provides OpenRouter integration via Rig library implementing `LLMProvider` trait
2. `airsspec-tools` provides `read_file`, `write_file`, `search` tools implementing `Tool` trait
3. `airsspec-agents` provides Researcher agent implementing `Agent` trait
4. Researcher agent can analyze sources and produce `requirements.md` artifact
5. Tool execution has sandboxing for filesystem access
6. `cargo build` succeeds with zero errors
7. `cargo clippy` passes with zero warnings
8. Integration tests demonstrate agent → tool → LLM flow

---

## Scope

### In Scope

| Crate | Modules | Responsibility |
|-------|---------|-----------------|
| `airsspec-llm` | `openrouter`, `client`, `streaming` | OpenRouter API via Rig, streaming response handling |
| `airsspec-tools` | `filesystem`, `search`, `registry` | Core tool implementations, tool dispatch |
| `airsspec-agents` | `researcher`, `executor` | First agent implementation, agent execution loop |

### Out of Scope

- Additional LLM providers (Anthropic, Ollama) — deferred to UOW-005
- All other agents (Architect, Designer, Planner, Builder) — future UOWs
- Vector store integration — Phase 3 (UOW-003)
- CLI/TUI interfaces — Phase 4 (UOW-004)
- MCP server — Phase 5 (UOW-005)

---

## Key Deliverables

Per [ROADMAP.md](file:///Users/hiraq/Projects/airsstack/airsspec/.airsspec/ROADMAP.md):

| Deliverable | Description |
|-------------|-------------|
| LLM Provider Integration | OpenRouter via Rig library (access to 200+ models) |
| Core Tool Implementations | `read_file`, `write_file`, `search` |
| First Agent: Researcher | Agent that produces `requirements.md` |
| Tool Sandboxing | Security boundaries for filesystem access |

---

## Dependencies

| Dependency | Status | Required For |
|------------|--------|--------------|
| UOW-001 Foundation | ✅ Complete | Core traits (`LLMProvider`, `Tool`, `Agent`) |
| `airsspec-core` | ✅ Available | Trait definitions |
| `airsspec-artifacts` | ✅ Available | Artifact persistence |
| `airsspec-runtime` | ✅ Available | State machine integration |

---

## Technology Constraints

| Component | Choice | Rationale |
|-----------|--------|-----------|
| LLM Framework | `rig` | Multi-provider support, type-safe extractors, native OpenRouter support |
| LLM Provider | OpenRouter | Unified API for 200+ models, single API key, fallback routing |
| HTTP Client | `reqwest` | Already a transitive dep via Rig |
| Streaming | `tokio-stream` | Consistent with async runtime |
| Sandbox | `cap-std` (optional) | Capability-based filesystem |

---

## Source References

| Source | Relevance |
|--------|-----------|
| [ROADMAP.md](file:///Users/hiraq/Projects/airsstack/airsspec/.airsspec/ROADMAP.md) | UOW-002 definition, key deliverables |
| [architecture.md](file:///Users/hiraq/Projects/airsstack/airsspec/.airsspec/knowledge/library/architecture.md) | Crate responsibilities, trait designs |
| [UOW-001 DAA.md](file:///Users/hiraq/Projects/airsstack/airsspec/.airsspec/uow/UOW-001-foundation/DAA.md) | Core trait definitions to implement |
| [rig-integration.md](file:///Users/hiraq/Projects/airsstack/airsspec/researches/rig-integration.md) | Rig library usage patterns |
| [openrouter-provider.md](file:///Users/hiraq/Projects/airsstack/airsspec/researches/openrouter-provider.md) | OpenRouter integration research |

---

## Open Questions

*None at this time.*

---

**Previous Phase**: UOW-001 Foundation Layer (✅ Complete)
**Next Phase**: UOW-003 Knowledge (requires working agents)
