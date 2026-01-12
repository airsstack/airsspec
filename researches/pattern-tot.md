# Tree-of-Thoughts (ToT) Pattern

**Date**: 2026-01-12
**Context**: LLM agent reasoning pattern for multi-path exploration
**Status**: Active Reference

---

## 1. Overview

Tree-of-Thoughts (ToT) extends Chain-of-Thought by exploring multiple reasoning paths simultaneously. Introduced by Yao et al. (2023), it treats problem-solving as a search through a tree of possible thoughts, with evaluation and pruning at each step.

> **Key Insight**: Some problems benefit from exploring alternatives before committing to a solution.

---

## 2. Core Concept

### The Tree Structure

```
                    [Problem]
                   /    |    \
              [Path A] [Path B] [Path C]
               /   \      |        \
          [A.1] [A.2]  [B.1]      [C.1]
            ↓
       [Solution]
```

### Trace Example

```
Problem: Write a coherent 4-paragraph story about a detective.

Branch A - Start with crime scene:
  Thought: "The rain fell heavily as Detective Mills..."
  Evaluation: 7/10 - Good atmosphere, but cliché opening

Branch B - Start with character:
  Thought: "Twenty years on the force had taught Mills..."
  Evaluation: 8/10 - Character-focused, engaging

Branch C - Start with dialogue:
  Thought: "'Another murder,' Mills muttered..."
  Evaluation: 6/10 - Abrupt, lacks context

Selected: Branch B (highest score)
Continue expanding Branch B...
```

---

## 3. Key Components

| Component | Description |
|-----------|-------------|
| **Thought Generator** | Produces multiple candidate thoughts |
| **State Evaluator** | Scores each thought/branch |
| **Search Algorithm** | BFS, DFS, or beam search |
| **Pruning** | Abandons low-scoring branches |

---

## 4. Search Strategies

### Breadth-First Search (BFS)

Explore all branches at each level before going deeper:

```
Level 1: [A, B, C] → Evaluate all → Prune to [A, B]
Level 2: [A.1, A.2, B.1, B.2] → Evaluate all → Prune to [A.2, B.1]
Level 3: Continue...
```

### Depth-First Search (DFS)

Explore one branch fully before backtracking:

```
Path: A → A.1 → A.1.1 → Dead end
Backtrack: A → A.2 → A.2.1 → Solution!
```

### Beam Search

Keep top-k branches at each level:

```
Beam width = 2
Level 1: [A=8, B=7, C=5] → Keep [A, B]
Level 2: [A.1=6, A.2=9, B.1=7, B.2=8] → Keep [A.2, B.2]
```

---

## 5. Characteristics

| Aspect | Description |
|--------|-------------|
| **Parallel exploration** | Multiple paths considered |
| **Evaluative** | Branches scored and compared |
| **Expensive** | Many LLM calls (one per branch per level) |
| **Powerful** | Finds better solutions for hard problems |
| **Complex** | More implementation overhead |

---

## 6. When to Use

✅ **Good for:**
- Creative writing with multiple approaches
- Complex planning with alternatives
- Game playing (chess, puzzles)
- Mathematical proofs with backtracking
- Design problems with trade-offs

❌ **Avoid for:**
- Simple factual queries
- Tasks with clear single solution
- Latency-critical applications
- Budget-constrained scenarios

---

## 7. Prompt Structure

### Thought Generation Prompt

```markdown
Given the problem: [problem]
Current state: [current progress]

Generate 3 different possible next steps.
For each, explain your reasoning.

Option 1: [thought]
Option 2: [thought]
Option 3: [thought]
```

### Evaluation Prompt

```markdown
Evaluate this partial solution on a scale of 1-10:

Problem: [problem]
Current approach: [thought]

Score: [1-10]
Reasoning: [why this score]
Should continue: [yes/no]
```

---

## 8. Implementation Considerations

### Tree Node Structure

```rust
pub struct ThoughtNode {
    pub thought: String,
    pub score: f32,
    pub children: Vec<ThoughtNode>,
    pub is_terminal: bool,
}

impl ThoughtNode {
    pub fn expand(&self, llm: &dyn LLMProvider) -> Vec<ThoughtNode> {
        // Generate multiple candidate thoughts
        let candidates = llm.generate_candidates(&self.thought, 3);
        
        candidates.into_iter().map(|thought| {
            let score = llm.evaluate(&thought);
            ThoughtNode { thought, score, children: vec![], is_terminal: false }
        }).collect()
    }
}
```

### Beam Search Implementation

```rust
pub async fn beam_search(
    problem: &str,
    beam_width: usize,
    max_depth: usize,
) -> Result<String> {
    let mut beam = vec![ThoughtNode::root(problem)];
    
    for depth in 0..max_depth {
        let mut candidates = Vec::new();
        
        for node in &beam {
            candidates.extend(node.expand(&llm).await?);
        }
        
        // Keep top-k by score
        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        beam = candidates.into_iter().take(beam_width).collect();
        
        if beam.iter().any(|n| n.is_terminal) {
            break;
        }
    }
    
    Ok(beam[0].thought.clone())
}
```

---

## 9. Cost Analysis

| Metric | CoT | ReAct | ToT |
|--------|-----|-------|-----|
| LLM calls | 1 | ~5-10 | ~30-100+ |
| Latency | Low | Medium | High |
| Token usage | Low | Medium | High |
| Solution quality | Good | Good | Best |

---

## 10. References

- Yao, S., et al. (2023). "Tree of Thoughts: Deliberate Problem Solving with Large Language Models"
- https://github.com/princeton-nlp/tree-of-thought-llm
