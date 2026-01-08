# AirSDLC Philosophy

This document defines the core principles that govern all AI-DLC workflows.

---

## 1. The Cognitive Cleanroom

**Principle**: Prevent context pollution by isolating agents to specific phases with constrained capabilities.

### How It Works
- Each phase has **allowed tools** and **blocked tools**
- Agents cannot access code-writing tools during specification phases
- Agents must reference artifacts, not assumptions

### Why It Matters
- Reduces hallucination by forcing agents to work with explicit context
- Creates audit trail: every decision traces back to an artifact
- Prevents "drift" from requirements to implementation

---

## 2. Filesystem as Truth

**Principle**: All state is persisted to disk. The UI (TUI/CLI) is just a view into the filesystem.

### Implications
- No hidden RAM buffers between agents
- Agents communicate through artifacts (files), not direct messages
- User can inspect, edit, or veto any artifact at any time
- Crash recovery is automatic — just read the last state from disk

### Key Directories
```
.airsspec/
├── WORKSPACE.md          # Project metadata
├── airsspec.toml         # Configuration
├── sources/              # Raw knowledge sources
├── knowledge/            # Synthesized context
│   ├── library/          # Warm memory (summaries)
│   └── vectors/          # Cold memory (embeddings)
├── contexts/             # Session logs (frozen)
└── uow/                  # Units of Work
    └── {uow-id}/
        ├── status.yaml   # Current status
        └── ...artifacts
```

---

## 3. Convention over Conversation

**Principle**: Reduce cognitive load by assuming reasonable defaults. Don't make the user memorize commands or answer redundant questions.

### Examples
- **Lazy Ingestion**: User drops files into `sources/`; system indexes when needed
- **Strict Compliance**: If an artifact is missing, halt and ask (don't guess)
- **Wizard Pattern**: One interactive flow gathers all context upfront

---

## 4. Artifact-Driven Communication

**Principle**: Agents do not speak directly. They write artifacts which serve as the "packet" of communication.

### The Blackboard Pattern
```
Agent A → writes → [Artifact] → reads ← Agent B
```

### Benefits
- **Transparency**: User sees everything agents "say"
- **Auditability**: Every decision is recorded
- **Interruptibility**: User can pause, edit, resume

---

## 5. Phase-Locked Progression

**Principle**: Work progresses through discrete phases. Each phase has prerequisites (gates) that must be met before proceeding.

### The Phases
1. **INGESTION** — Load knowledge sources
2. **RESEARCH** — Create PRD (Product Requirements)
3. **INCEPTION** — Create DAA (Domain Architecture Analysis)
4. **DESIGN** — Create ADRs (Architecture Decision Records)
5. **PLANNING** — Create RFC + Bolt Plans
6. **CONSTRUCTION** — Write code within Bolt tasks

### Gate Conditions
- requirements.md must exist and be approved → proceed to Inception
- DAA.md must exist and be approved → proceed to Design
- ADRs must exist and be approved → proceed to Planning
- RFC.md + plan.json must exist → proceed to Construction

---

## 6. Trust But Verify

**Principle**: AI agents do the work, but humans remain the gatekeepers.

### Human-in-the-Loop
```
Agent → produces artifact → User reviews → Approve/Reject → Next phase
```

Every phase transition requires explicit or implicit human approval.

---

## Summary Table

| Principle | Key Rule |
|-----------|----------|
| Cognitive Cleanroom | Constrain tools per phase |
| Filesystem as Truth | Persist all state to disk |
| Convention over Conversation | Assume defaults, minimize prompts |
| Artifact-Driven Communication | Agents communicate via files |
| Phase-Locked Progression | Gate transitions on artifacts |
| Trust But Verify | Human approves every phase |

---

**Next**: [workspace-explore.md](./workspace-explore.md) — Learn how to explore a project
