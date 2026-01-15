//! Reasoning pattern abstractions for pluggable agent execution strategies.

use async_trait::async_trait;

use super::types::{ExecutionContext, PatternConfig, PatternError, ReasoningStep};

/// A reasoning pattern that guides agent execution.
///
/// Patterns like `ReAct`, `CoT`, and `ToT` implement this trait to provide
/// different strategies for how agents reason and act.
///
/// # Examples
///
/// ```rust
/// # use airsspec_core::reasoning::traits::ReasoningPattern;
/// # use airsspec_core::reasoning::types::{ExecutionContext, PatternConfig};
/// // Callers typically get implementations from other crates (e.g., airsspec-agents)
/// // and invoke methods on them:
/// // let pattern: Box<dyn ReasoningPattern> = get_pattern_from_somewhere();
/// // let step = pattern.next_step(&context).await;
/// ```
#[async_trait]
pub trait ReasoningPattern: Send + Sync {
    /// Pattern identifier (e.g., "react", "cot", "tot", "hybrid")
    fn name(&self) -> &str;

    /// Pattern configuration
    fn config(&self) -> &PatternConfig;

    /// Generate the next reasoning step given current context.
    ///
    /// This is the core method that drives the reasoning loop.
    ///
    /// # Arguments
    ///
    /// * `context` - The current execution context including history and available tools
    ///
    /// # Returns
    ///
    /// A `ReasoningStep` representing the next action to take, or an error if generation fails.
    async fn next_step(&self, context: &ExecutionContext) -> Result<ReasoningStep, PatternError>;

    /// Determine if execution should continue.
    ///
    /// Returns false when:
    /// - Final answer has been produced
    /// - Budget is exhausted
    /// - Maximum iterations reached
    ///
    /// # Arguments
    ///
    /// * `context` - The current execution context
    ///
    /// # Returns
    ///
    /// `true` if execution should continue, `false` otherwise.
    fn should_continue(&self, context: &ExecutionContext) -> bool;

    /// Format the initial prompt for this pattern.
    ///
    /// Each pattern has its own prompt structure (e.g., `ReAct`'s
    /// "Thought/Action/Observation" format).
    ///
    /// # Arguments
    ///
    /// * `query` - The user's original query
    ///
    /// # Returns
    ///
    /// A formatted prompt string ready to be sent to an LLM.
    fn format_prompt(&self, query: &str) -> String;

    /// Parse LLM response into a structured step.
    ///
    /// Extracts thoughts, actions, or final answers from raw LLM output
    /// according to the pattern's expected format.
    ///
    /// # Arguments
    ///
    /// * `response` - The raw response from an LLM
    ///
    /// # Returns
    ///
    /// A `ReasoningStep` parsed from the response, or a `PatternError` if parsing fails.
    ///
    /// # Errors
    ///
    /// Returns `PatternError::ParseError` if the response cannot be parsed.
    fn parse_response(&self, response: &str) -> Result<ReasoningStep, PatternError>;
}

/// Selector for choosing reasoning patterns dynamically.
///
/// Implementations can use rules, heuristics, or LLM-based selection
/// to determine which pattern is most suitable for a given context.
///
/// # Examples
///
/// ```rust
/// # use airsspec_core::reasoning::traits::PatternSelector;
/// # use airsspec_core::reasoning::types::ExecutionContext;
/// // Callers typically get selectors from other crates and use them:
/// // let selector: Box<dyn PatternSelector> = get_selector_from_somewhere();
/// // let pattern_name = selector.select(&context)?;
/// ```
pub trait PatternSelector: Send + Sync {
    /// Select the appropriate pattern for the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - The execution context to make a selection based on
    ///
    /// # Returns
    ///
    /// The name of the selected pattern (e.g., "react", "cot"), or an error if selection fails.
    ///
    /// # Errors
    ///
    /// Returns `PatternError::Internal` if the selection process fails.
    fn select(&self, context: &ExecutionContext) -> Result<String, PatternError>;
}
