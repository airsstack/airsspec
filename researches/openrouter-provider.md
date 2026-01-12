# OpenRouter Integration Research

**Date**: 2026-01-12
**Context**: LLM Provider Selection for AirsSpec UOW-002
**Status**: Active

---

## 1. Overview

[OpenRouter](https://openrouter.ai/) is a **unified LLM API** that provides access to 200+ models from all major providers through a single endpoint.

### Key Benefits

| Benefit | Description |
|---------|-------------|
| **Unified API** | Single endpoint for OpenAI, Anthropic, Google, Meta, and more |
| **OpenAI Compatible** | Works with OpenAI SDK by changing base URL |
| **Fallback Routing** | Automatic fallback when providers go down |
| **Cost Optimization** | Edge routing for low latency, pay-per-use |
| **Data Policies** | Fine-grained control over which providers receive prompts |

---

## 2. API Integration

### Endpoint

```
https://openrouter.ai/api/v1/chat/completions
```

### Model Format

Models are referenced as `provider/model-name`:
- `openai/gpt-5.2`
- `anthropic/claude-opus-4.5`
- `google/gemini-3-pro-preview`
- `meta-llama/llama-3.1-8b`

### Headers

| Header | Purpose |
|--------|---------|
| `Authorization` | `Bearer <OPENROUTER_API_KEY>` |
| `HTTP-Referer` | Optional, for rankings on openrouter.ai |
| `X-Title` | Optional, site/app name for rankings |

---

## 3. Rig Native Support (Verified from docs.rs)

> **Confirmed**: Rig v0.28.0 includes `rig::providers::openrouter` module.
> Source: [docs.rs/rig-core/latest/rig/providers/openrouter](https://docs.rs/rig-core/latest/rig/providers/openrouter/index.html)

### Module Structure

```
rig::providers::openrouter
├── client     # OpenRouter client implementation
├── completion # Completion model types
└── streaming  # Streaming response handling
```

### Official Example (from docs.rs)

```rust
use rig::providers::openrouter;

// Create OpenRouter client
let client = openrouter::Client::new("YOUR_API_KEY");

// Use predefined model constants
let llama_3_1_8b = client.completion_model(openrouter::LLAMA_3_1_8B);
```

### Available Providers in Rig 0.28.0

From [rig::providers](https://docs.rs/rig-core/latest/rig/providers/index.html):
- `anthropic`, `azure`, `cohere`, `deepseek`
- `galadriel`, `gemini`, `groq`, `huggingface`
- `hyperbolic`, `mira`, `mistral`, `moonshot`
- `ollama`, `openai`, **`openrouter`**, `perplexity`
- `together`, `voyageai`, `xai`

### Provider Registry with OpenRouter

Update to `rig-integration.md` pattern:

```rust
match config.service.as_str() {
    "openrouter" => {
        let client = openrouter::Client::new(&token);
        Ok(Box::new(client.model(&config.model)))
    },
    // Other providers...
}
```

### Configuration

```toml
[providers.primary]
service = "openrouter"
model = "anthropic/claude-opus-4.5"
api_key = "env:OPENROUTER_API_KEY"

[providers.fast]
service = "openrouter"
model = "meta-llama/llama-3.1-8b-instruct"
api_key = "env:OPENROUTER_API_KEY"
```

---

## 4. Advantages for AirsSpec

### Why OpenRouter over Direct Provider APIs

| Aspect | Direct Providers | OpenRouter |
|--------|------------------|------------|
| **API Keys** | Multiple keys per provider | Single key |
| **Switching Models** | Code changes, new deps | Just change model string |
| **Availability** | Single point of failure | Multi-provider fallback |
| **Cost Tracking** | Per-provider dashboards | Unified dashboard |
| **New Models** | Wait for SDK updates | Instant access |

### Strategic Value

1. **Provider Agnostic**: Users choose their preferred models
2. **No Vendor Lock-in**: Switch models without code changes
3. **Future-Proof**: New models available immediately
4. **Simplified Config**: Single API key management

---

## 5. Implementation Notes

### Required Changes

1. Use `rig::providers::openrouter` instead of individual providers
2. Model references use `provider/model` format
3. Single `OPENROUTER_API_KEY` environment variable
4. Provider registry simplifies to OpenRouter-first approach

### Fallback Strategy

OpenRouter handles provider fallbacks internally, but we can also:
1. Configure multiple OpenRouter model aliases
2. Implement application-level fallback via provider registry

---

## 6. References

- [OpenRouter Docs](https://openrouter.ai/docs/quickstart)
- [OpenRouter Models](https://openrouter.ai/models)
- [Rig OpenRouter Module](https://docs.rs/rig/latest/rig/providers/openrouter/index.html)
- [Rig Integration Guide](file:///Users/hiraq/Projects/airsstack/airsspec/researches/rig-integration.md)
