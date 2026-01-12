# Hybrid Reasoning Pattern

**Date**: 2026-01-12
**Context**: LLM agent reasoning pattern combining multiple strategies
**Status**: Active Reference

---

## 1. Overview

The Hybrid pattern combines multiple reasoning strategies (CoT, ReAct, ToT) within a single agent execution. Different phases of a task may benefit from different approaches, and a hybrid system selects the optimal strategy dynamically.

> **Key Insight**: No single pattern is optimal for all tasks. Adaptive strategy selection improves overall performance.

---

## 2. Core Concept

### Pattern Composition

```
┌─────────────────────────────────────────────────────────┐
│                    Hybrid Executor                      │
├─────────────────────────────────────────────────────────┤
│  ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐ │
│  │   CoT   │   │  ReAct  │   │   ToT   │   │ Custom  │ │
│  └────┬────┘   └────┬────┘   └────┬────┘   └────┬────┘ │
│       └─────────────┴─────────────┴─────────────┘      │
│                         ↓                               │
│                  Pattern Selector                       │
│                         ↓                               │
│              Selected Pattern Executes                  │
└─────────────────────────────────────────────────────────┘
```

### Execution Example

```
Task: Research competitors and write analysis report

Phase 1 - Planning (CoT):
  Thought: Let me plan my approach...
  Step 1: Identify key competitors
  Step 2: Gather data on each
  Step 3: Compare and analyze
  Step 4: Write report

Phase 2 - Research (ReAct):
  Thought: I'll start gathering data...
  Action: search("competitor A revenue 2025")
  Observation: $50M annual revenue...
  Action: search("competitor B market share")
  Observation: 15% market share...

Phase 3 - Analysis (CoT):
  Thought: Now I'll analyze the data...
  Step 1: Competitor A leads in revenue...
  Step 2: Competitor B growing faster...
  Conclusion: ...

Phase 4 - Writing (CoT):
  Thought: Structuring the report...
  [produces final report]
```

---

## 3. Strategy Selection

### Rule-Based Selection

```rust
fn select_pattern(task_type: &TaskType) -> Box<dyn ReasoningPattern> {
    match task_type {
        TaskType::Research => Box::new(ReactPattern::new()),
        TaskType::Analysis => Box::new(CotPattern::new()),
        TaskType::Creative => Box::new(TotPattern::new()),
        TaskType::Simple => Box::new(DirectPattern::new()),
    }
}
```

### LLM-Based Selection

Let the model itself choose the pattern:

```markdown
Given this task: [task description]

Which reasoning approach would be most effective?
1. Chain-of-Thought: Think step by step (for logic/math)
2. ReAct: Use tools to gather information (for research)
3. Tree-of-Thoughts: Explore alternatives (for creative/complex)

Select approach:
```

### Heuristic Selection

```rust
fn select_by_heuristics(query: &str, context: &Context) -> PatternType {
    if query.contains_any(&["search", "find", "look up", "what is"]) {
        PatternType::ReAct
    } else if query.contains_any(&["calculate", "solve", "prove"]) {
        PatternType::CoT
    } else if query.contains_any(&["design", "create", "write story"]) {
        PatternType::ToT
    } else if context.has_tools() {
        PatternType::ReAct  // Default to ReAct if tools available
    } else {
        PatternType::CoT    // Fallback to CoT
    }
}
```

---

## 4. Composition Strategies

### Sequential Composition

Use different patterns for different phases:

```
Phase 1 (Planning)    → CoT
Phase 2 (Execution)   → ReAct  
Phase 3 (Reflection)  → CoT
```

### Nested Composition

Use one pattern inside another:

```
ReAct Loop:
  Thought: I need to plan this carefully...
  [Switch to CoT for planning]
  
  CoT:
    Step 1: First I'll...
    Step 2: Then I'll...
  [End CoT]
  
  Action: execute_plan(step_1)
```

### Fallback Composition

Switch patterns on failure:

```
Try: ToT (explore alternatives)
  ↓ (failed/timeout)
Fallback: ReAct (simpler approach)
  ↓ (failed)
Fallback: CoT (simplest)
```

---

## 5. Implementation

### Hybrid Pattern Trait

```rust
pub struct HybridPattern {
    patterns: HashMap<String, Box<dyn ReasoningPattern>>,
    selector: Box<dyn PatternSelector>,
}

impl ReasoningPattern for HybridPattern {
    fn name(&self) -> &str { "hybrid" }
    
    async fn next_step(&self, context: &ExecutionContext) 
        -> Result<ReasoningStep> 
    {
        // Select appropriate pattern for current context
        let pattern_name = self.selector.select(context)?;
        let pattern = self.patterns.get(&pattern_name)
            .ok_or(PatternError::NotFound)?;
        
        // Delegate to selected pattern
        pattern.next_step(context).await
    }
}
```

### Pattern Selector Trait

```rust
pub trait PatternSelector: Send + Sync {
    fn select(&self, context: &ExecutionContext) -> Result<String, SelectError>;
}

pub struct RuleBasedSelector {
    rules: Vec<(Condition, String)>,
}

impl PatternSelector for RuleBasedSelector {
    fn select(&self, context: &ExecutionContext) -> Result<String> {
        for (condition, pattern_name) in &self.rules {
            if condition.matches(context) {
                return Ok(pattern_name.clone());
            }
        }
        Ok("cot".into()) // Default fallback
    }
}
```

---

## 6. Configuration

```toml
[patterns.hybrid]
default = "react"

[patterns.hybrid.rules]
# Pattern selection rules (first match wins)
[[patterns.hybrid.rules.rule]]
condition = "query_contains('calculate', 'solve', 'prove')"
pattern = "cot"

[[patterns.hybrid.rules.rule]]
condition = "query_contains('design', 'create', 'alternatives')"
pattern = "tot"

[[patterns.hybrid.rules.rule]]
condition = "tools_available"
pattern = "react"
```

---

## 7. When to Use

✅ **Good for:**
- Complex multi-phase tasks
- Tasks with unknown optimal strategy
- General-purpose agents
- Research + analysis + writing workflows

❌ **Avoid for:**
- Simple, well-defined tasks
- Latency-critical scenarios
- When a single pattern is clearly optimal

---

## 8. Advantages

| Benefit | Description |
|---------|-------------|
| **Flexibility** | Adapts to task requirements |
| **Robustness** | Fallback options on failure |
| **Efficiency** | Uses simple patterns for simple subtasks |
| **Quality** | Uses powerful patterns when needed |

---

## 9. References

- Internal AirsSpec design discussions
- Pattern composition strategies from multi-agent literature
