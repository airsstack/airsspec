---
version: "1.0"
status: proposed
author: architect
created_at: 2026-01-12
---

# ADR-002: Tool System Architecture

## Status

Proposed

## Context

From DAA bounded context "Tool Execution":

- Agents need to invoke tools (read_file, write_file, search)
- Tool executions must be validated against security policies
- Must implement `Tool` trait from `airsspec-core`

Key constraints from requirements.md:
- Sandbox filesystem access
- Tool registry for discovery and dispatch

## Decision

Implement a **registry-based tool system** with **sandbox enforcement**.

### Tool Trait Implementation

Each tool is a struct implementing the `Tool` trait:

```rust
// Conceptual structure (technology reference only)
pub struct ReadFileTool {
    sandbox: Sandbox,
}

impl Tool for ReadFileTool {
    fn name(&self) -> &str { "read_file" }
    fn execute(&self, input: ToolInput) -> Result<ToolOutput>;
}
```

### Sandbox Design

Path-based security with canonicalization:

1. All paths resolved to absolute
2. Paths must start with allowed prefix (project root)
3. Deny patterns for sensitive files (.env, .git, etc.)

### Module Structure

```
airsspec-tools/
├── src/
│   ├── lib.rs           # Re-exports
│   ├── registry.rs      # ToolRegistry implementation
│   ├── sandbox.rs       # Security policy enforcement
│   └── tools/
│       ├── mod.rs
│       ├── read_file.rs
│       ├── write_file.rs
│       └── search.rs
```

### Core Tools (MVP)

| Tool | Purpose | Sandbox Level |
|------|---------|---------------|
| `read_file` | Read file contents | Read-only in project |
| `write_file` | Write file contents | Write in designated dirs |
| `search` | Search file contents | Read-only in project |

## Consequences

### Positive

- **Security by default** — sandbox enforced on all operations
- **Extensible** — new tools added via trait impl
- **Testable** — sandbox can be mocked for testing
- **Auditable** — all tool invocations logged via domain events

### Negative

- **Performance overhead** — path canonicalization on every call
- **Limited flexibility** — strict sandbox may block legitimate use cases

### Neutral

- Future tools (shell execution, HTTP requests) will require additional sandbox rules
- Tool schemas for LLM function calling deferred to implementation

## Alternatives Considered

### Option A: No Sandbox

Trust agent to only access appropriate files.

**Pros**: Simpler implementation
**Cons**: Security risk — LLM could hallucinate dangerous commands

**Rejected**: Security is non-negotiable for agent systems.

### Option B: Capability-based (cap-std)

Use capability-based filesystem from `cap-std` crate.

**Pros**: Stronger security model
**Cons**: Additional dependency, learning curve

**Deferred**: May adopt in future version for enhanced security.

## References

- DAA: [DAA.md](../DAA.md) — Tool Execution context
- Requirements: [requirements.md](../requirements.md) — Tool sandboxing
