# Memory Management

This instruction explains the memory architecture and context management for AI agents.

---

## Memory Tiers Overview

AI agents operate within context limits. AirsDLC uses a multi-tiered memory system to maximize effectiveness while staying within bounds.

| Tier | Type | Storage | Capacity | Strategy |
|------|------|---------|----------|----------|
| **Hot** | Rolling Window | RAM + JSONL | ~30 messages | FIFO Buffer |
| **Warm** | Synthesized | `knowledge/library/` | Unlimited | Tree-Reduce |
| **Cold** | Vector Index | `knowledge/vectors/` | Unlimited | Map-Reduce + Embed |
| **Frozen** | Audit Logs | `contexts/agent/*.jsonl` | Unlimited | Append-Only |

---

## Hot Memory (Immediate Context)

### What It Is
The most recent ~30 interactions (messages, actions, observations) kept in active context.

### How It Works
1. Every interaction is appended to `contexts/agent/{session}.jsonl`
2. On agent startup, load the last 30 lines
3. Oldest messages are pushed out as new ones arrive (FIFO)

### When to Use
- Current task execution
- Recent decisions and observations
- Immediate error context

### Persistence Path
```
.airsspec/contexts/agent/{session-id}.jsonl
```

Each line is a JSON object:
```json
{"type": "user", "content": "...", "ts": "..."}
{"type": "thought", "content": "...", "ts": "..."}
{"type": "action", "tool": "write_file", "params": {...}, "ts": "..."}
{"type": "observation", "result": "...", "ts": "..."}
```

---

## Warm Memory (Synthesized Summaries)

### What It Is
Compressed summaries of older context, preserved for mid-term recall.

### How It Works (Tree-Reduce Algorithm)
1. **Trigger**: When Hot Memory exceeds 80% of token limit
2. **Freeze**: Pause the agent
3. **Segment**: Take oldest 50% of messages, split into chunks of ~10
4. **Compress**: Summarize each chunk, then merge summaries
5. **Persist**: Write to `knowledge/library/history-{seq}.md`
6. **Resume**: Agent continues with: `[Summary Link] + [Recent 50%]`

### When to Use
- Recall decisions from earlier in the session
- Understand context evolution
- Trace back reasoning

### Persistence Path
```
.airsspec/knowledge/library/
├── history-001.md
├── history-002.md
└── ...
```

---

## Cold Memory (Semantic Search)

### What It Is
Vector-indexed knowledge for semantic retrieval across all sessions and sources.

### How It Works
1. **Ingestion**: Documents added to `sources/` are processed
2. **Chunking**: Split into meaningful segments
3. **Embedding**: Convert to vectors
4. **Index**: Store in Lance database

### When to Use
- Recall decisions from weeks/months ago
- Find relevant architectural patterns
- Search across all project knowledge

### Tool: `query_memory`
```json
{
  "thought": "I need to recall the authentication decision.",
  "action": {
    "tool_name": "query_memory",
    "params": { "query": "Authentication ADR decision" }
  }
}
```

### Persistence Path
```
.airsspec/knowledge/vectors/
```

---

## Frozen Memory (Audit Trail)

### What It Is
Complete, append-only logs of every agent session.

### How It Works
- Every interaction is immediately persisted
- Never modified or deleted
- Used for replay, debugging, and audit

### When to Use
- Post-mortem analysis
- Compliance auditing
- Training data extraction

### Persistence Path
Same as Hot Memory JSONL files:
```
.airsspec/contexts/agent/{session-id}.jsonl
```

---

## Context Compression Flow

```
┌─────────────────────────────────────────────────────────────┐
│                     HOT MEMORY (RAM)                        │
│  [msg1] [msg2] [msg3] ... [msg28] [msg29] [msg30]          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ (Token count > 80%)
┌─────────────────────────────────────────────────────────────┐
│                    COMPRESSION TRIGGER                      │
│  1. Pause agent                                             │
│  2. Take oldest 50% (msg1-msg15)                            │
│  3. Chunk into groups of 10                                 │
│  4. Summarize each chunk                                    │
│  5. Merge into master summary                               │
│  6. Write to library/history-{seq}.md                       │
│  7. Resume with [Summary Link] + [msg16-msg30]              │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   WARM MEMORY (Library)                     │
│  history-001.md  history-002.md  history-003.md   ...      │
└─────────────────────────────────────────────────────────────┘
```

---

## Configuration

In `airsspec.toml`:

```toml
[memory]
hot_window_size = 30          # Messages in rolling window
compression_threshold = 0.8    # Trigger at 80% capacity
chunk_size = 10               # Messages per summary chunk
```

---

## Best Practices for Agents

1. **Minimize context bloat**: Don't output unnecessarily verbose responses
2. **Reference artifacts**: Instead of repeating content, link to files
3. **Use `query_memory`**: When you need historical context
4. **Trust the summary**: Don't try to "remember" — the system handles it

---

**Next**: [constraints.md](./constraints.md) — Output format and tool constraints
