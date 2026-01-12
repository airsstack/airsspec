# PLAN-002: Reasoning Types

## Objective

Define supporting types for reasoning patterns in `airsspec-core/src/reasoning/types.rs`.

## Deliverables

- `airsspec-core/src/reasoning/types.rs`

## Implementation

### File: `airsspec-core/src/reasoning/types.rs`

```rust
//! Types for reasoning pattern execution.
//!
//! These types are intentionally generic and pattern-agnostic.
//! Pattern-specific types (e.g., ToT's branch evaluation) belong
//! in the implementing crates, not in core.

use std::collections::HashMap;

use crate::tool::types::{ToolInput, ToolOutput};

/// A single step in the reasoning process.
///
/// This enum defines the **generic** step types that all patterns produce.
/// Pattern-specific step types should use `Extension` with custom data.
#[derive(Debug, Clone)]
pub enum ReasoningStep {
    /// Internal reasoning (thought/reflection).
    /// Used by: CoT, ReAct, ToT, Hybrid
    Thought(String),
    
    /// Single tool invocation request.
    /// Used by: ReAct, Hybrid
    Action(ActionRequest),
    
    /// Multiple parallel tool invocations.
    /// Used by: ReAct (with parallel mode), Hybrid
    ParallelActions(Vec<ActionRequest>),
    
    /// Final answer produced — execution should stop.
    /// Used by: All patterns
    FinalAnswer(String),
    
    /// Pattern-specific extension point.
    /// Allows patterns to define custom step types without modifying core.
    /// The inner data is pattern-defined (e.g., ToT's branch evaluation).
    Extension {
        /// Pattern that defines this extension
        pattern: String,
        /// Extension type identifier
        kind: String,
        /// Serialized extension data (JSON or other format)
        data: String,
    },
}

/// A request to invoke a tool.
#[derive(Debug, Clone)]
pub struct ActionRequest {
    /// Tool name to invoke
    pub tool: String,
    /// Arguments for the tool
    pub args: ToolInput,
}

/// Result of a tool invocation.
#[derive(Debug, Clone)]
pub struct ActionResult {
    /// Tool that was invoked
    pub tool: String,
    /// Output from the tool
    pub output: ToolOutput,
    /// Whether the action succeeded
    pub success: bool,
}

/// Context available during reasoning execution.
///
/// This is the shared context passed to all reasoning patterns.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Original user query
    pub query: String,
    
    /// Conversation history (thoughts, actions, observations)
    pub history: Vec<HistoryEntry>,
    
    /// Current iteration count
    pub iteration: usize,
    
    /// Tokens consumed so far
    pub tokens_used: usize,
    
    /// Available tool names
    pub available_tools: Vec<String>,
    
    /// Arbitrary metadata for pattern-specific data
    pub metadata: HashMap<String, String>,
}

/// An entry in the reasoning history.
#[derive(Debug, Clone)]
pub enum HistoryEntry {
    /// A thought was generated
    Thought(String),
    
    /// An action was requested
    ActionRequested(ActionRequest),
    
    /// An observation was received (tool output)
    Observation(ActionResult),
    
    /// An error occurred
    Error(String),
}

/// Configuration for a reasoning pattern.
#[derive(Debug, Clone)]
pub struct PatternConfig {
    /// Maximum iterations before stopping
    pub max_iterations: usize,
    
    /// Maximum tokens before budget exhaustion
    pub max_tokens: usize,
    
    /// Enable parallel action execution
    pub parallel_actions: bool,
    
    /// Timeout per action in seconds
    pub action_timeout_secs: u64,
    
    /// Pattern-specific settings
    pub settings: HashMap<String, String>,
}

impl Default for PatternConfig {
    fn default() -> Self {
        Self {
            max_iterations: 20,
            max_tokens: 100_000,
            parallel_actions: true,
            action_timeout_secs: 30,
            settings: HashMap::new(),
        }
    }
}

/// Errors that can occur during pattern execution.
#[derive(Debug, thiserror::Error)]
pub enum PatternError {
    #[error("Failed to parse LLM response: {0}")]
    ParseError(String),
    
    #[error("Pattern not found: {0}")]
    NotFound(String),
    
    #[error("Budget exhausted: {0}")]
    BudgetExhausted(String),
    
    #[error("Maximum iterations reached: {0}")]
    MaxIterations(usize),
    
    #[error("Action failed: {0}")]
    ActionFailed(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}
```

## Guidelines Compliance

This plan complies with `.aiassisted/guidelines/rust/project-standard.md`:

| Rule | Status | Notes |
|------|--------|-------|
| §2.1 3-Layer Imports | ✅ | Std (HashMap) → internal (crate::tool) |
| §2.2 No FQN | ✅ | All types imported at top |
| §6.1 YAGNI | ✅ | Generic types only, no pattern-specific variants |
| §6.2 Avoid dyn | ✅ | No `dyn` trait objects |
| §6.4 Quality Gates | ✅ | thiserror for errors, Default impl |

### Foundation Principle

Core types are **generic and pattern-agnostic**:
- ❌ Removed: `BranchEval` (ToT-specific)
- ✅ Added: `Extension` variant for pattern-specific data
- Pattern implementations (in `airsspec-agents`) can use `Extension` for custom step types

## Acceptance Criteria

- [ ] `ReasoningStep` enum with generic variants (no pattern-specific types)
- [ ] `ActionRequest` and `ActionResult` for tool invocation
- [ ] `ExecutionContext` with full history tracking
- [ ] `PatternConfig` with sensible defaults
- [ ] `PatternError` with thiserror
- [ ] `Extension` variant for pattern-specific extensibility
- [ ] `cargo build` passes
- [ ] `cargo clippy` with zero warnings
