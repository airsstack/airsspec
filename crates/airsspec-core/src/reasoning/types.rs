//! Types for reasoning pattern execution.
//!
//! These types are intentionally generic and pattern-agnostic.
//! Pattern-specific types (e.g., `ToT`'s branch evaluation) belong
//! in the implementing crates, not in core.

// Layer 1: Standard library imports
use std::collections::HashMap;

// Layer 2: Third-party crate imports
use serde::{Deserialize, Serialize};

/// A single step in the reasoning process.
///
/// This enum defines the **generic** step types that all patterns produce.
/// Pattern-specific step types should use `Extension` with custom data.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::reasoning::types::ReasoningStep;
///
/// // A thought step
/// let thought = ReasoningStep::Thought("Let me think about this problem".to_string());
///
/// // A final answer
/// let answer = ReasoningStep::FinalAnswer("The answer is 42".to_string());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningStep {
    /// Internal reasoning (thought/reflection).
    /// Used by: `CoT`, `ReAct`, `ToT`, `Hybrid`
    Thought(String),

    /// Single tool invocation request.
    /// Used by: `ReAct`, `Hybrid`
    Action(ActionRequest),

    /// Multiple parallel tool invocations.
    /// Used by: `ReAct` (with parallel mode), `Hybrid`
    ParallelActions(Vec<ActionRequest>),

    /// Final answer produced — execution should stop.
    /// Used by: All patterns
    FinalAnswer(String),

    /// Pattern-specific extension point.
    /// Allows patterns to define custom step types without modifying core.
    /// The inner data is pattern-defined (e.g., `ToT`'s branch evaluation).
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
///
/// This structure captures the intent to execute a tool with specific arguments.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::reasoning::types::ActionRequest;
/// use airsspec_core::tool::types::ToolId;
/// use serde_json::json;
///
/// let request = ActionRequest {
///     tool: "bash".to_string(),
///     args: Default::default(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRequest {
    /// Tool name to invoke
    pub tool: String,
    /// Arguments for the tool (intentionally opaque to support any tool)
    pub args: serde_json::Value,
}

impl ActionRequest {
    /// Creates a new action request.
    ///
    /// # Arguments
    ///
    /// * `tool` - The name of the tool to invoke
    /// * `args` - Arguments for the tool as JSON
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::reasoning::types::ActionRequest;
    /// use serde_json::json;
    ///
    /// let action = ActionRequest::new("bash", json!({ "command": "ls -la" }));
    /// ```
    pub fn new(tool: impl Into<String>, args: serde_json::Value) -> Self {
        Self {
            tool: tool.into(),
            args,
        }
    }
}

/// Result of a tool invocation.
///
/// Captures the outcome of executing an action, including success status and output.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::reasoning::types::ActionResult;
/// use serde_json::json;
///
/// let result = ActionResult {
///     tool: "bash".to_string(),
///     output: Default::default(),
///     success: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// Tool that was invoked
    pub tool: String,
    /// Output from the tool
    pub output: serde_json::Value,
    /// Whether the action succeeded
    pub success: bool,
}

impl ActionResult {
    /// Creates a new action result.
    ///
    /// # Arguments
    ///
    /// * `tool` - The name of the tool that was invoked
    /// * `success` - Whether the tool execution succeeded
    /// * `output` - The output from the tool
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::reasoning::types::ActionResult;
    /// use serde_json::json;
    ///
    /// let result = ActionResult::new("bash", true, json!({ "status": "ok" }));
    /// ```
    pub fn new(tool: impl Into<String>, success: bool, output: serde_json::Value) -> Self {
        Self {
            tool: tool.into(),
            success,
            output,
        }
    }
}

/// Context available during reasoning execution.
///
/// This is the shared context passed to all reasoning patterns during the reasoning loop.
/// It accumulates history, tracks iterations, and provides access to available tools.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::reasoning::types::ExecutionContext;
///
/// let context = ExecutionContext {
///     query: "What is 2+2?".to_string(),
///     history: vec![],
///     iteration: 0,
///     tokens_used: 0,
///     available_tools: vec!["calculator".to_string()],
///     metadata: Default::default(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl ExecutionContext {
    /// Creates a new execution context.
    ///
    /// # Arguments
    ///
    /// * `query` - The user's query
    /// * `available_tools` - List of tool names available for use
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::reasoning::types::ExecutionContext;
    ///
    /// let context = ExecutionContext::new(
    ///     "What is 2+2?".to_string(),
    ///     vec!["calculator".to_string()],
    /// );
    /// ```
    #[must_use]
    pub fn new(query: String, available_tools: Vec<String>) -> Self {
        Self {
            query,
            history: Vec::new(),
            iteration: 0,
            tokens_used: 0,
            available_tools,
            metadata: HashMap::new(),
        }
    }

    /// Adds a history entry to this context.
    ///
    /// # Arguments
    ///
    /// * `entry` - The history entry to add
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::reasoning::types::{ExecutionContext, HistoryEntry};
    ///
    /// let mut context = ExecutionContext::new(
    ///     "What is 2+2?".to_string(),
    ///     vec![],
    /// );
    /// context.add_history(HistoryEntry::Thought("Let me calculate".to_string()));
    /// ```
    pub fn add_history(&mut self, entry: HistoryEntry) {
        self.history.push(entry);
    }

    /// Increments the iteration counter.
    #[allow(clippy::missing_const_for_fn)]
    pub fn increment_iteration(&mut self) {
        self.iteration += 1;
    }

    /// Adds to the token count.
    ///
    /// # Arguments
    ///
    /// * `tokens` - Number of tokens to add
    #[allow(clippy::missing_const_for_fn)]
    pub fn add_tokens(&mut self, tokens: usize) {
        self.tokens_used += tokens;
    }

    /// Sets metadata for pattern-specific use.
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
}

/// An entry in the reasoning history.
///
/// Represents a significant event during the reasoning process.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::reasoning::types::{HistoryEntry, ActionRequest};
///
/// let thought = HistoryEntry::Thought("Thinking...".to_string());
/// let error = HistoryEntry::Error("Tool failed".to_string());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
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
///
/// Controls various aspects of pattern execution including iterations,
/// token budgets, and pattern-specific settings.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::reasoning::types::PatternConfig;
///
/// let config = PatternConfig {
///     max_iterations: 20,
///     max_tokens: 100_000,
///     parallel_actions: true,
///     action_timeout_secs: 30,
///     settings: Default::default(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Creates a default configuration suitable for most reasoning patterns.
    ///
    /// Defaults:
    /// - `max_iterations`: 20
    /// - `max_tokens`: 100,000
    /// - `parallel_actions`: true
    /// - `action_timeout_secs`: 30
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
///
/// Covers parsing failures, missing patterns, resource exhaustion, and action failures.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::reasoning::types::PatternError;
///
/// let error = PatternError::ParseError("Invalid response format".to_string());
/// let error = PatternError::BudgetExhausted("Tokens exceeded".to_string());
/// ```
#[derive(Debug, thiserror::Error)]
pub enum PatternError {
    /// Failed to parse LLM response into a structured step
    #[error("Failed to parse LLM response: {0}")]
    ParseError(String),

    /// Requested pattern was not found
    #[error("Pattern not found: {0}")]
    NotFound(String),

    /// Resource budget exhausted (tokens or iterations)
    #[error("Budget exhausted: {0}")]
    BudgetExhausted(String),

    /// Maximum iterations reached
    #[error("Maximum iterations reached: {0}")]
    MaxIterations(usize),

    /// Tool action failed during execution
    #[error("Action failed: {0}")]
    ActionFailed(String),

    /// Internal error in the reasoning pattern
    #[error("Internal error: {0}")]
    Internal(String),
}
