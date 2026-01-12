# ReAct Pattern (Reason + Act)

**Date**: 2026-01-12
**Context**: LLM agent reasoning pattern for tool-using agents
**Status**: Active Reference

---

## 1. Overview

ReAct is a prompting paradigm that synergizes **reasoning** (chain-of-thought) with **acting** (external tool use). Introduced by Yao et al. (2023), it enables LLMs to interact dynamically with external environments while maintaining explicit reasoning traces.

> **Key Insight**: Interleaving thoughts with actions grounds the model in real data, reducing hallucination.

---

## 2. Core Concept

### The Loop

```
Thought → Action → Observation → Thought → Action → ...
```

| Component | Description |
|-----------|-------------|
| **Thought** | Internal reasoning — decompose task, plan next step |
| **Action** | External tool invocation — search, read, write |
| **Observation** | Tool result fed back into context |

### Trace Example

```
Question: What is the capital of the country where the Eiffel Tower is located?

Thought: I need to find which country has the Eiffel Tower.
Action: search("Eiffel Tower location")
Observation: The Eiffel Tower is located in Paris, France.

Thought: France is the country. Now I need its capital.
Action: search("capital of France")
Observation: The capital of France is Paris.

Thought: I have the answer.
Action: respond("The capital is Paris.")
```

---

## 3. Characteristics

| Aspect | Description |
|--------|-------------|
| **Iterative** | Loop continues until final answer or budget exhausted |
| **Tool-integrated** | Actions invoke real external systems |
| **Grounded** | Answers based on retrieved data, not hallucination |
| **Traceable** | Each step explicitly logged and auditable |
| **Flexible** | Can adapt strategy based on observations |

---

## 4. When to Use

✅ **Good for:**
- Research tasks requiring multiple sources
- Fact-checking and verification
- Multi-step information gathering
- Tasks where external data is essential

❌ **Avoid for:**
- Pure reasoning without external data needs
- Simple single-step queries
- Tasks where latency is critical (many tool calls = slow)

---

## 5. Prompt Structure

### System Prompt Template

```markdown
You are an AI assistant that solves problems by reasoning and using tools.

Available tools:
- search(query): Search the web for information
- read_file(path): Read a file's contents
- write_file(path, content): Write content to a file

For each step, respond in this format:
Thought: [your reasoning about what to do next]
Action: [tool_name](arguments)

After receiving an observation, continue reasoning until you can provide a final answer.

When you have the final answer:
Thought: I now have enough information.
Action: respond("[your final answer]")
```

---

## 6. Implementation Considerations

### Parsing Actions

```rust
pub struct ActionParser;

impl ActionParser {
    pub fn parse(response: &str) -> Result<ReasoningStep, ParseError> {
        if let Some(action_line) = extract_line(response, "Action:") {
            let (tool, args) = parse_tool_call(action_line)?;
            if tool == "respond" {
                Ok(ReasoningStep::FinalAnswer(args))
            } else {
                Ok(ReasoningStep::Action { tool, args })
            }
        } else if let Some(thought_line) = extract_line(response, "Thought:") {
            Ok(ReasoningStep::Thought(thought_line))
        } else {
            Err(ParseError::InvalidFormat)
        }
    }
}
```

### Observation Injection

After each tool execution, inject result into context:

```
Observation: [tool output]

Thought: [model continues reasoning...]
```

---

## 7. Advantages over Pure CoT

| Aspect | CoT | ReAct |
|--------|-----|-------|
| External data | ❌ No | ✅ Yes |
| Hallucination | Higher risk | Lower risk |
| Verifiable | Hard | Easy (check tool outputs) |
| Flexibility | Static | Dynamic adaptation |

---

## 8. References

- Yao, S., et al. (2023). "ReAct: Synergizing Reasoning and Acting in Language Models"
- https://react-lm.github.io/
