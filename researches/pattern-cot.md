# Chain-of-Thought (CoT) Pattern

**Date**: 2026-01-12
**Context**: LLM agent reasoning pattern for complex logic
**Status**: Active Reference

---

## 1. Overview

Chain-of-Thought (CoT) is a prompting technique that encourages LLMs to generate intermediate reasoning steps before arriving at a final answer. Introduced by Wei et al. (2022), it significantly improves performance on complex reasoning tasks.

> **Key Insight**: Explicit step-by-step reasoning helps models avoid errors on multi-step problems.

---

## 2. Core Concept

### The Pattern

```
Question → Step 1 → Step 2 → ... → Step N → Answer
```

Unlike ReAct, CoT is **purely internal** — no external tool calls. The model reasons through the problem using only its internal knowledge.

### Trace Example

```
Question: If John has 5 apples and gives 2 to Mary, then buys 3 more, 
how many apples does John have?

Let me think step by step:

Step 1: John starts with 5 apples.
Step 2: John gives 2 to Mary, so 5 - 2 = 3 apples remaining.
Step 3: John buys 3 more, so 3 + 3 = 6 apples.

Answer: John has 6 apples.
```

---

## 3. Variants

### Zero-Shot CoT

Simply add "Let's think step by step" to the prompt:

```
Q: [complex question]

Let's think step by step.
```

### Few-Shot CoT

Provide examples of step-by-step reasoning:

```
Q: [example question 1]
A: Let me think step by step...
   Step 1: ...
   Answer: ...

Q: [example question 2]
A: Let me think step by step...
   Step 1: ...
   Answer: ...

Q: [actual question]
A:
```

### Self-Consistency CoT

Generate multiple CoT traces and vote on the answer:

```
Trace 1 → Answer A
Trace 2 → Answer A
Trace 3 → Answer B

Final Answer: A (majority vote)
```

---

## 4. Characteristics

| Aspect | Description |
|--------|-------------|
| **Linear** | Steps flow sequentially |
| **Internal** | No external tool calls |
| **Explicit** | Forces model to show work |
| **Deterministic** | Same input → similar reasoning |
| **Fast** | Single LLM call (no tool latency) |

---

## 5. When to Use

✅ **Good for:**
- Mathematical reasoning
- Logical deduction
- Multi-step planning (without external data)
- Code generation with complex logic
- Explaining decisions

❌ **Avoid for:**
- Tasks requiring external/current data
- Simple factual questions
- Tasks where reasoning path doesn't help

---

## 6. Prompt Structure

### System Prompt Template

```markdown
You are an AI assistant that solves problems by thinking step by step.

When given a problem:
1. Break it down into smaller steps
2. Solve each step explicitly
3. Show your reasoning clearly
4. Arrive at the final answer

Always structure your response as:
Let me think step by step:
Step 1: [first reasoning step]
Step 2: [second reasoning step]
...
Answer: [final answer]
```

---

## 7. Implementation Considerations

### Parsing Steps

```rust
pub struct CotParser;

impl CotParser {
    pub fn parse(response: &str) -> Result<Vec<ReasoningStep>, ParseError> {
        let mut steps = Vec::new();
        
        for line in response.lines() {
            if line.starts_with("Step") || line.starts_with("Let me think") {
                steps.push(ReasoningStep::Thought(line.to_string()));
            } else if line.starts_with("Answer:") {
                steps.push(ReasoningStep::FinalAnswer(
                    line.strip_prefix("Answer:").unwrap().trim().to_string()
                ));
            }
        }
        
        Ok(steps)
    }
}
```

### Single-Turn Execution

Unlike ReAct, CoT typically completes in a single LLM call:

```rust
async fn execute_cot(&self, query: &str) -> Result<String> {
    let prompt = format!("{}\n\nLet me think step by step:", query);
    let response = self.llm.complete(&prompt).await?;
    
    // Extract answer from response
    CotParser::extract_answer(&response)
}
```

---

## 8. Comparison with ReAct

| Aspect | CoT | ReAct |
|--------|-----|-------|
| External tools | ❌ No | ✅ Yes |
| LLM calls | 1 | Many |
| Latency | Low | Higher |
| Data freshness | Training cutoff | Real-time |
| Best for | Reasoning | Research |

---

## 9. References

- Wei, J., et al. (2022). "Chain-of-Thought Prompting Elicits Reasoning in Large Language Models"
- Kojima, T., et al. (2022). "Large Language Models are Zero-Shot Reasoners"
