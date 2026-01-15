//! Reasoning pattern abstractions.
//!
//! This module defines traits for pluggable reasoning strategies:
//! - `ReasoningPattern` - Core trait for reasoning/acting loops
//! - `PatternSelector` - Dynamic pattern selection
//!
//! # Patterns
//!
//! Implementations (in `airsspec-agents`) include:
//! - **`ReAct`**: Thought → Action → Observation loop
//! - **`CoT`**: Chain-of-Thought step-by-step reasoning
//! - **`ToT`**: Tree-of-Thoughts branching exploration
//! - **`Hybrid`**: Combines multiple patterns
//!
//! # Usage
//!
//! ```rust
//! use airsspec_core::reasoning::traits::ReasoningPattern;
//! use airsspec_core::reasoning::types::{ExecutionContext, ReasoningStep};
//!
//! // Implementations typically come from airsspec-agents
//! // async fn use_pattern(pattern: &dyn ReasoningPattern) -> Result<ReasoningStep, _> {
//! //     let context = ExecutionContext::new("query".into(), vec![]);
//! //     pattern.next_step(&context).await
//! // }
//! ```

// NOTE: Per §4.3, no type re-exports. Callers use namespaced access:
// use airsspec_core::reasoning::traits::ReasoningPattern;
// use airsspec_core::reasoning::types::ExecutionContext;

pub mod traits;
pub mod types;
