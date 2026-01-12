---
version: "1.0"
status: proposed
author: architect
created_at: 2026-01-12
---

# ADR-001: LLM Integration via Rig + OpenRouter

## Status

Proposed

## Context

The Agent System requires LLM capabilities to enable reasoning and text generation. From DAA bounded context "Language Model Communication":

- Need to support multiple providers and models
- Streaming responses required for responsive UX
- Must implement `LLMProvider` trait from `airsspec-core`

Key constraints from requirements.md:
- OpenRouter as primary provider (unified API for 200+ models)
- Rig library for LLM abstraction (native OpenRouter support)

## Decision

Use **Rig library (v0.28.0+)** with **OpenRouter provider** for all LLM interactions.

### Implementation Approach

1. **Provider Abstraction**: Wrap Rig's `openrouter::Client` in a struct implementing `LLMProvider` trait
2. **Model Configuration**: Use environment-based model selection with defaults
3. **Streaming**: Leverage Rig's native streaming support via `tokio-stream`

### Module Structure

```
airsspec-llm/
├── src/
│   ├── lib.rs           # Re-exports
│   ├── provider.rs      # OpenRouterProvider impl
│   ├── config.rs        # Provider configuration
│   └── streaming.rs     # Stream handling
```

### Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `OPENROUTER_API_KEY` | Authentication | Required |
| `AIRSSPEC_DEFAULT_MODEL` | Default model | `anthropic/claude-sonnet-4` |

## Consequences

### Positive

- **200+ models accessible** via single integration
- **Rig handles complexity** (retries, rate limiting, parsing)
- **Type-safe extractors** for structured output
- **Native Rust async** — no FFI overhead

### Negative

- **External dependency** on Rig library updates
- **OpenRouter lock-in** for initial release
- **API key required** — no local-only mode in v1

### Neutral

- Other providers (Ollama, direct Anthropic) deferred to UOW-005
- Provider registry pattern established for future multi-provider support

## Alternatives Considered

### Option A: Direct OpenRouter HTTP

Build raw HTTP client using `reqwest`.

**Pros**: No library dependency
**Cons**: Must handle streaming, retries, error parsing manually

**Rejected**: Too much low-value implementation work.

### Option B: async-openai crate

Use OpenAI SDK with OpenRouter base URL.

**Pros**: Well-maintained, popular
**Cons**: OpenAI-specific types may not match all OpenRouter models

**Rejected**: Rig provides better multi-provider abstraction.

## References

- DAA: [DAA.md](../DAA.md) — Language Model Communication context
- Research: [rig-integration.md](../../../../researches/rig-integration.md)
- Research: [openrouter-provider.md](../../../../researches/openrouter-provider.md)
