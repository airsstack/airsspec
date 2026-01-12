# PLAN-001: ReasoningPattern Trait

## Objective

Define the core `ReasoningPattern` trait and related traits in `airsspec-core/src/reasoning/traits.rs`.

## Deliverables

- `airsspec-core/src/reasoning/traits.rs`
- `airsspec-core/src/reasoning/mod.rs` (re-exports)

## Implementation

### File: `airsspec-core/src/reasoning/traits.rs`

```rust
//! Reasoning pattern abstractions for pluggable agent execution strategies.

use async_trait::async_trait;
use crate::tool::types::ToolInput;
use super::types::{ReasoningStep, ExecutionContext, PatternConfig, PatternError};

/// A reasoning pattern that guides agent execution.
///
/// Patterns like ReAct, CoT, and ToT implement this trait to provide
/// different strategies for how agents reason and act.
#[async_trait]
pub trait ReasoningPattern: Send + Sync {
    /// Pattern identifier (e.g., "react", "cot", "tot", "hybrid")
    fn name(&self) -> &str;
    
    /// Pattern configuration
    fn config(&self) -> &PatternConfig;
    
    /// Generate the next reasoning step given current context.
    ///
    /// This is the core method that drives the reasoning loop.
    async fn next_step(
        &self,
        context: &ExecutionContext,
    ) -> Result<ReasoningStep, PatternError>;
    
    /// Determine if execution should continue.
    ///
    /// Returns false when:
    /// - Final answer has been produced
    /// - Budget is exhausted
    /// - Maximum iterations reached
    fn should_continue(&self, context: &ExecutionContext) -> bool;
    
    /// Format the initial prompt for this pattern.
    ///
    /// Each pattern has its own prompt structure (e.g., ReAct's
    /// "Thought/Action/Observation" format).
    fn format_prompt(&self, query: &str) -> String;
    
    /// Parse LLM response into a structured step.
    ///
    /// Extracts thoughts, actions, or final answers from raw LLM output.
    fn parse_response(&self, response: &str) -> Result<ReasoningStep, PatternError>;
}

/// Selector for choosing reasoning patterns dynamically.
///
/// Implementations can use rules, heuristics, or LLM-based selection.
pub trait PatternSelector: Send + Sync {
    /// Select the appropriate pattern for the given context.
    fn select(&self, context: &ExecutionContext) -> Result<String, PatternError>;
}
```

### File: `airsspec-core/src/reasoning/mod.rs`

```rust
//! Reasoning pattern abstractions.
//!
//! This module defines traits for pluggable reasoning strategies:
//! - `ReasoningPattern` - Core trait for reasoning/acting loops
//! - `PatternSelector` - Dynamic pattern selection
//!
//! # Patterns
//!
//! Implementations (in `airsspec-agents`) include:
//! - **ReAct**: Thought → Action → Observation loop
//! - **CoT**: Chain-of-Thought step-by-step reasoning
//! - **ToT**: Tree-of-Thoughts branching exploration
//! - **Hybrid**: Combines multiple patterns
//!
//! # Usage
//!
//! ```rust
//! use airsspec_core::reasoning::traits::ReasoningPattern;
//! use airsspec_core::reasoning::types::{ExecutionContext, ReasoningStep};
//! ```

// NOTE: Per §4.3, no type re-exports. Callers use namespaced access:
// use airsspec_core::reasoning::traits::ReasoningPattern;
// use airsspec_core::reasoning::types::ExecutionContext;

pub mod traits;
pub mod types;
```

## Guidelines Compliance

This plan complies with `.aiassisted/guidelines/rust/project-standard.md`:

| Rule | Status | Notes |
|------|--------|-------|
| §2.1 3-Layer Imports | ✅ | Std → third-party → internal |
| §2.2 No FQN | ✅ | All types imported |
| §4.3 mod.rs Policy | ✅ | Only module declarations, no type re-exports |
| §6.2 Avoid dyn | ✅ | No `dyn` trait objects used |
| §6.4 Quality Gates | ✅ | thiserror, zero warnings |

## Acceptance Criteria

- [ ] `ReasoningPattern` trait defined with all methods
- [ ] `PatternSelector` trait defined
- [ ] Module declarations only in mod.rs (no type re-exports)
- [ ] Documentation with usage examples showing namespaced imports
- [ ] `cargo build` passes
- [ ] `cargo clippy` with zero warnings
