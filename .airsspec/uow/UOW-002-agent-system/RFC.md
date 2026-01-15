---
version: "1.0"
status: draft
author: airsspec-manager
created_at: 2026-01-15
---

# RFC: Agent System Implementation

## Overview

This RFC defines the implementation plan for UOW-002 (Agent System), synthesizing architectural decisions from ADR-001 (LLM Integration), ADR-002 (Tool System), and ADR-003 (Agent Architecture) into executable work units (Bolts).

**Goal**: Deliver the first working agent (Researcher) that can analyze sources and produce `requirements.md` artifacts using LLM reasoning and tool execution.

**Derived from**: 
- [requirements.md](./requirements.md)
- [DAA.md](./DAA.md)
- [ADR-001](./adrs/ADR-001-llm-integration.md)
- [ADR-002](./adrs/ADR-002-tool-system.md)
- [ADR-003](./adrs/ADR-003-agent-architecture.md)

---

## Architecture Summary

### Three Bounded Contexts

| Context | Crate | Responsibility |
|---------|-------|----------------|
| **Language Model Communication** | `airsspec-llm` | OpenRouter provider via Rig, streaming |
| **Tool Execution** | `airsspec-tools` | Registry, sandbox, core tools |
| **Agent Orchestration** | `airsspec-agents` | Event-driven executor, reasoning patterns, Researcher agent |

### Key Architectural Decisions

1. **LLM Integration** (ADR-001):
   - Rig library v0.28.0+ for OpenRouter access
   - 200+ models via single API
   - Environment-based configuration

2. **Tool System** (ADR-002):
   - Registry-based tool discovery
   - Path-based sandbox enforcement
   - Core tools: `read_file`, `write_file`, `search`

3. **Agent Architecture** (ADR-003):
   - Event-driven execution model
   - Pluggable reasoning patterns (ReAct, CoT)
   - Parallel tool execution support

---

## Implementation Plan

### Phase 1: Foundation (Bolts 1-3)

Build the core infrastructure layers bottom-up:

```
BOLT-001: LLM Provider
    ↓
BOLT-002: Tool System
    ↓
BOLT-003: Agent Executor
```

### Phase 2: Integration (Bolt 4)

Assemble components into working Researcher agent:

```
BOLT-004: Researcher Agent
```

---

## Bolt Breakdown

### BOLT-001: LLM Provider Integration

**Objective**: Implement OpenRouter provider via Rig library

**Scope**:
- `airsspec-llm` crate setup
- `OpenRouterProvider` implementing `LLMProvider` trait
- Configuration via environment variables
- Streaming response support

**Deliverables**:
- `src/provider.rs` — Provider implementation
- `src/config.rs` — Configuration loading
- `src/streaming.rs` — Stream handling
- Unit tests for provider
- Integration test calling OpenRouter

**Dependencies**: 
- `airsspec-core` (provides `LLMProvider` trait)
- Rig library v0.28.0+

**Success Criteria**:
- ✅ OpenRouter completion request succeeds
- ✅ Streaming responses work
- ✅ Environment config loads correctly
- ✅ `cargo test` passes

---

### BOLT-002: Tool System

**Objective**: Implement tool registry and core filesystem tools

**Scope**:
- `airsspec-tools` crate setup
- `ToolRegistry` for tool discovery
- `Sandbox` for security enforcement
- Core tools: `ReadFileTool`, `WriteFileTool`, `SearchTool`

**Deliverables**:
- `src/registry.rs` — Tool registration and dispatch
- `src/sandbox.rs` — Path validation and security
- `src/tools/read_file.rs` — Read file tool
- `src/tools/write_file.rs` — Write file tool
- `src/tools/search.rs` — Content search tool
- Unit tests for each tool
- Sandbox security tests

**Dependencies**:
- `airsspec-core` (provides `Tool` trait)

**Success Criteria**:
- ✅ Tools can be registered and retrieved
- ✅ Sandbox blocks unauthorized access
- ✅ All three core tools work correctly
- ✅ `cargo test` passes

---

### BOLT-003: Agent Executor Framework

**Objective**: Implement event-driven execution engine with reasoning patterns

**Scope**:
- `airsspec-agents` crate setup
- `AgentExecutor` implementing event loop
- `ReasoningPattern` trait abstraction
- ReAct pattern implementation
- Chain-of-Thought pattern implementation
- Session management

**Deliverables**:
- `src/executor.rs` — Event-driven executor
- `src/events.rs` — Event type definitions
- `src/session.rs` — Session lifecycle
- `src/patterns/traits.rs` — ReasoningPattern trait
- `src/patterns/react.rs` — ReAct implementation
- `src/patterns/cot.rs` — CoT implementation
- Unit tests for executor
- Pattern tests with mocked LLM

**Dependencies**:
- `airsspec-core` (provides `Agent`, `AgentExecutor` traits)
- BOLT-001 (`airsspec-llm`)
- BOLT-002 (`airsspec-tools`)

**Success Criteria**:
- ✅ Executor can run event loop
- ✅ ReAct pattern completes thought → action cycle
- ✅ CoT pattern generates reasoning chains
- ✅ Parallel tool execution works
- ✅ `cargo test` passes

---

### BOLT-004: Researcher Agent

**Objective**: Implement first concrete agent using executor framework

**Scope**:
- `ResearcherAgent` implementing `Agent` trait
- Preamble/prompt engineering for research tasks
- Source ingestion logic
- `requirements.md` artifact generation
- End-to-end integration test

**Deliverables**:
- `src/agents/researcher.rs` — Researcher implementation
- Integration test: full research flow
- Example: generate requirements from sample sources
- Documentation for using Researcher

**Dependencies**:
- BOLT-003 (`AgentExecutor`)
- BOLT-001 (`LLMProvider`)
- BOLT-002 (`ToolRegistry`)

**Success Criteria**:
- ✅ Researcher can analyze source files
- ✅ Produces valid `requirements.md` artifact
- ✅ End-to-end test passes
- ✅ Example run succeeds

---

## Dependency Graph

```
BOLT-001 (LLM)     BOLT-002 (Tools)
    \                  /
     \                /
      \              /
       BOLT-003 (Executor)
            |
            |
       BOLT-004 (Researcher)
```

**Critical Path**: Sequential execution required (BOLT-001/002 → BOLT-003 → BOLT-004)

---

## Quality Gates

Each Bolt must satisfy:

1. **Build**: `cargo build` succeeds with zero errors
2. **Lint**: `cargo clippy` passes with zero warnings
3. **Test**: `cargo test` passes with 100% of tests
4. **Documentation**: Public APIs documented with rustdoc
5. **Integration**: Integration test demonstrates working flow

---

## Risk Management

| Risk | Mitigation |
|------|------------|
| **OpenRouter API downtime** | Implement retry logic with exponential backoff in BOLT-001 |
| **Sandbox too restrictive** | Document allowed paths clearly; make configurable in BOLT-002 |
| **Reasoning patterns incomplete** | Start with ReAct (proven), defer advanced patterns |
| **Integration complexity** | Mock dependencies in unit tests; integration tests per Bolt |

---

## Timeline Estimate

| Bolt | Complexity | Estimate |
|------|------------|----------|
| BOLT-001 | Low-Medium | 1-2 days |
| BOLT-002 | Medium | 2-3 days |
| BOLT-003 | High | 3-4 days |
| BOLT-004 | Medium | 2-3 days |

**Total**: 8-12 days for full implementation

---

## Success Metrics

**UOW-002 complete when**:

1. ✅ All 4 Bolts delivered and passing quality gates
2. ✅ Researcher agent can analyze sources and produce `requirements.md`
3. ✅ End-to-end integration test passes
4. ✅ Documentation complete
5. ✅ User approval obtained

---

## References

- Requirements: [requirements.md](./requirements.md)
- Domain Analysis: [DAA.md](./DAA.md)
- Architecture Decisions: [adrs/](./adrs/)
- Foundation: [UOW-001](../UOW-001-foundation/)
- ROADMAP: [.airsspec/ROADMAP.md](../../ROADMAP.md)
