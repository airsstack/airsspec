//! Agent execution traits and related types.
//!
//! This module defines the core traits for agent execution, including the `Agent` trait,
//! `AgentExecutor` trait, and supporting types for context, output, and errors.

// Layer 1: Standard library imports
use std::path::PathBuf;

// Layer 2: Third-party crate imports
use async_trait::async_trait;
use thiserror::Error;

// Layer 3: Internal module imports
use crate::error::AgentError;
use crate::memory::types::MemoryFragment;
use crate::state::Phase;

use super::types::{AgentConfig, AgentId, DelegationSignal};

/// Execution context for an agent.
///
/// Provides the agent with information about the current UOW, phase, memory fragments,
/// and available artifacts.
///
/// # Fields
///
/// * `uow_id` - Identifier for the current Unit of Work
/// * `phase` - Current phase in the AI-DLC
/// * `memory` - Memory fragments containing context from previous interactions
/// * `artifacts` - List of artifact paths relevant to the current task
///
/// # Examples
///
/// ```rust
/// use airsspec_core::agent::traits::AgentContext;
/// use airsspec_core::state::Phase;
///
/// let context = AgentContext {
///     uow_id: "UOW-001".to_string(),
///     phase: Phase::Construction,
///     memory: vec![],
///     artifacts: vec![],
/// };
/// ```
#[derive(Debug, Clone)]
pub struct AgentContext {
    /// Identifier for the current Unit of Work.
    pub uow_id: String,

    /// Current phase in the AI-DLC.
    pub phase: Phase,

    /// Memory fragments containing context from previous interactions.
    pub memory: Vec<MemoryFragment>,

    /// List of artifact paths relevant to the current task.
    pub artifacts: Vec<PathBuf>,
}

/// Token usage tracking for agent execution.
///
/// Tracks the number of tokens used during agent execution.
///
/// # Fields
///
/// * `prompt_tokens` - Number of tokens in the prompt/input
/// * `completion_tokens` - Number of tokens in the completion/output
/// * `total_tokens` - Total number of tokens used
///
/// # Examples
///
/// ```rust
/// use airsspec_core::agent::traits::TokenUsage;
///
/// let usage = TokenUsage {
///     prompt_tokens: 1000,
///     completion_tokens: 500,
///     total_tokens: 1500,
/// };
/// assert_eq!(usage.total_tokens, 1500);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TokenUsage {
    /// Number of tokens in the prompt/input.
    pub prompt_tokens: u32,

    /// Number of tokens in the completion/output.
    pub completion_tokens: u32,

    /// Total number of tokens used.
    pub total_tokens: u32,
}

impl TokenUsage {
    /// Creates a new token usage record.
    ///
    /// # Arguments
    ///
    /// * `prompt_tokens` - Number of tokens in the prompt/input
    /// * `completion_tokens` - Number of tokens in the completion/output
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::agent::traits::TokenUsage;
    ///
    /// let usage = TokenUsage::new(1000, 500);
    /// assert_eq!(usage.total_tokens, 1500);
    /// ```
    #[must_use]
    pub const fn new(prompt_tokens: u32, completion_tokens: u32) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
        }
    }

    /// Adds another token usage record to this one.
    ///
    /// # Arguments
    ///
    /// * `other` - Another token usage record to add
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::agent::traits::TokenUsage;
    ///
    /// let usage1 = TokenUsage::new(1000, 500);
    /// let usage2 = TokenUsage::new(500, 250);
    /// let total = usage1 + usage2;
    /// assert_eq!(total.total_tokens, 2250);
    /// ```
    #[must_use]
    pub const fn add(&self, other: &Self) -> Self {
        Self {
            prompt_tokens: self.prompt_tokens + other.prompt_tokens,
            completion_tokens: self.completion_tokens + other.completion_tokens,
            total_tokens: self.total_tokens + other.total_tokens,
        }
    }
}

impl std::ops::Add for TokenUsage {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.prompt_tokens + other.prompt_tokens,
            self.completion_tokens + other.completion_tokens,
        )
    }
}

/// Output from an agent execution.
///
/// Contains the result of agent execution, including any created artifacts,
/// delegation signals, and token usage statistics.
///
/// # Fields
///
/// * `result` - The primary result text from the agent
/// * `artifacts_created` - List of artifact paths created during execution
/// * `delegation` - Optional delegation signal to another agent
/// * `token_usage` - Token usage statistics for this execution
///
/// # Examples
///
/// ```rust
/// use airsspec_core::agent::traits::AgentOutput;
/// use airsspec_core::agent::traits::TokenUsage;
///
/// let output = AgentOutput {
///     result: "Task completed successfully".to_string(),
///     artifacts_created: vec![],
///     delegation: None,
///     token_usage: TokenUsage::new(1000, 500),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct AgentOutput {
    /// The primary result text from the agent.
    pub result: String,

    /// List of artifact paths created during execution.
    pub artifacts_created: Vec<PathBuf>,

    /// Optional delegation signal to another agent.
    pub delegation: Option<DelegationSignal>,

    /// Token usage statistics for this execution.
    pub token_usage: TokenUsage,
}

/// Agent execution trait.
///
/// Defines the core interface for agent execution in the `AirsSpec` framework.
/// Agents implement this trait to provide their execution logic.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::agent::traits::{Agent, AgentContext, AgentOutput};
/// use airsspec_core::agent::types::{AgentId, AgentConfig};
/// use airsspec_core::error::AgentError;
/// use async_trait::async_trait;
///
/// struct MyAgent {
///     config: AgentConfig,
/// }
///
/// #[async_trait]
/// impl Agent for MyAgent {
///     fn id(&self) -> &AgentId {
///         &self.config.id
///     }
///
///     fn config(&self) -> &AgentConfig {
///         &self.config
///     }
///
///     async fn execute(&self, context: AgentContext) -> Result<AgentOutput, AgentError> {
///         // Agent execution logic here
///         Ok(AgentOutput {
///             result: "Done".to_string(),
///             artifacts_created: vec![],
///             delegation: None,
///             token_usage: Default::default(),
///         })
///     }
/// }
/// ```
#[async_trait]
pub trait Agent: Send + Sync {
    /// Returns the agent's unique identifier.
    fn id(&self) -> &AgentId;

    /// Returns the agent's configuration.
    fn config(&self) -> &AgentConfig;

    /// Executes the agent with the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - The execution context containing UOW ID, phase, memory, and artifacts
    ///
    /// # Returns
    ///
    /// The agent's output, including results, artifacts, and delegation signals
    async fn execute(&self, context: AgentContext) -> Result<AgentOutput, AgentError>;
}

/// Result from agent execution.
///
/// Contains the final output along with execution statistics.
///
/// # Fields
///
/// * `output` - The final agent output
/// * `iterations` - Number of iterations executed
/// * `total_tokens` - Total tokens consumed across all iterations
///
/// # Examples
///
/// ```rust
/// use airsspec_core::agent::traits::{AgentOutput, ExecutionResult, TokenUsage};
/// use std::path::PathBuf;
///
/// let result = ExecutionResult {
///     output: AgentOutput {
///         result: "Complete".to_string(),
///         artifacts_created: vec![],
///         delegation: None,
///         token_usage: TokenUsage::new(1000, 500),
///     },
///     iterations: 3,
///     total_tokens: 1500,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// The final agent output.
    pub output: AgentOutput,

    /// Number of iterations executed.
    pub iterations: u32,

    /// Total tokens consumed across all iterations.
    pub total_tokens: u32,
}

/// Agent executor trait.
///
/// Defines the interface for executing agents with budget constraints and iteration limits.
/// Executors are responsible for managing the execution loop, enforcing budget limits,
/// and handling delegation between agents.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::agent::traits::{Agent, AgentExecutor, ExecutionResult, ExecutionError};
/// use airsspec_core::agent::types::Budget;
/// use async_trait::async_trait;
///
/// struct MyExecutor;
///
/// #[async_trait]
/// impl AgentExecutor for MyExecutor {
///     async fn run(
///         &self,
///         agent: &dyn Agent,
///         budget: Budget,
///     ) -> Result<ExecutionResult, ExecutionError> {
///         // Execution logic with budget enforcement
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait AgentExecutor: Send + Sync {
    /// Runs an agent with the given budget constraints.
    ///
    /// # Arguments
    ///
    /// * `agent` - The agent to execute
    /// * `budget` - The budget constraints for this execution
    ///
    /// # Returns
    ///
    /// The execution result including output and statistics
    async fn run(
        &self,
        agent: &dyn Agent,
        budget: super::types::Budget,
    ) -> Result<ExecutionResult, ExecutionError>;
}

/// Execution errors.
///
/// Errors that can occur during agent execution.
#[derive(Debug, Error)]
pub enum ExecutionError {
    /// Budget exceeded during execution.
    #[error("Budget exceeded: {0}")]
    BudgetExceeded(String),

    /// Execution timeout.
    #[error("Execution timeout after {0} seconds")]
    Timeout(u64),

    /// Maximum iterations exceeded.
    #[error("Maximum iterations exceeded: {0}")]
    MaxIterationsExceeded(u32),

    /// Agent execution failed.
    #[error("Agent execution failed: {0}")]
    AgentExecution(String),

    /// Invalid agent state.
    #[error("Invalid agent state: {0}")]
    InvalidState(String),

    /// Delegation failed.
    #[error("Delegation failed: {0}")]
    DelegationFailed(String),
}

impl From<AgentError> for ExecutionError {
    fn from(error: AgentError) -> Self {
        Self::AgentExecution(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_usage_new() {
        let usage = TokenUsage::new(1000, 500);
        assert_eq!(usage.prompt_tokens, 1000);
        assert_eq!(usage.completion_tokens, 500);
        assert_eq!(usage.total_tokens, 1500);
    }

    #[test]
    fn test_token_usage_add() {
        let usage1 = TokenUsage::new(1000, 500);
        let usage2 = TokenUsage::new(500, 250);
        let total = usage1.add(&usage2);

        assert_eq!(total.prompt_tokens, 1500);
        assert_eq!(total.completion_tokens, 750);
        assert_eq!(total.total_tokens, 2250);
    }

    #[test]
    fn test_token_usage_add_operator() {
        let usage1 = TokenUsage::new(1000, 500);
        let usage2 = TokenUsage::new(500, 250);
        let total = usage1 + usage2;

        assert_eq!(total.prompt_tokens, 1500);
        assert_eq!(total.completion_tokens, 750);
        assert_eq!(total.total_tokens, 2250);
    }

    #[test]
    fn test_token_usage_default() {
        let usage = TokenUsage::default();
        assert_eq!(usage.prompt_tokens, 0);
        assert_eq!(usage.completion_tokens, 0);
        assert_eq!(usage.total_tokens, 0);
    }

    #[test]
    fn test_token_usage_partial_eq() {
        let usage1 = TokenUsage::new(1000, 500);
        let usage2 = TokenUsage::new(1000, 500);
        let usage3 = TokenUsage::new(1000, 600);

        assert_eq!(usage1, usage2);
        assert_ne!(usage1, usage3);
    }

    #[test]
    fn test_execution_error_budget_exceeded() {
        let error = ExecutionError::BudgetExceeded("Token limit reached".to_string());
        assert!(error.to_string().contains("Budget exceeded"));
        assert!(error.to_string().contains("Token limit reached"));
    }

    #[test]
    fn test_execution_error_timeout() {
        let error = ExecutionError::Timeout(300);
        assert!(error.to_string().contains("timeout"));
        assert!(error.to_string().contains("300"));
    }

    #[test]
    fn test_execution_error_max_iterations_exceeded() {
        let error = ExecutionError::MaxIterationsExceeded(100);
        assert!(error.to_string().contains("Maximum iterations exceeded"));
        assert!(error.to_string().contains("100"));
    }

    #[test]
    fn test_execution_error_from_agent_error() {
        let agent_error = AgentError::Execution("Test error".to_string());
        let exec_error: ExecutionError = agent_error.into();

        match exec_error {
            ExecutionError::AgentExecution(msg) => {
                assert!(msg.contains("Test error"));
            }
            _ => panic!("Expected AgentExecution variant"),
        }
    }

    #[test]
    fn test_clone_token_usage() {
        let usage = TokenUsage::new(1000, 500);
        let cloned = usage;

        assert_eq!(cloned.prompt_tokens, 1000);
        assert_eq!(cloned.completion_tokens, 500);
    }

    #[test]
    fn test_copy_token_usage() {
        let usage = TokenUsage::new(1000, 500);
        let copied = usage; // TokenUsage is Copy

        assert_eq!(copied.prompt_tokens, 1000);
        assert_eq!(copied.completion_tokens, 500);
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_agent_context_clone() {
        let context = AgentContext {
            uow_id: "UOW-001".to_string(),
            phase: Phase::Construction,
            memory: vec![],
            artifacts: vec![],
        };

        let cloned = context.clone();
        assert_eq!(cloned.uow_id, "UOW-001");
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_agent_output_clone() {
        let output = AgentOutput {
            result: "Test".to_string(),
            artifacts_created: vec![],
            delegation: None,
            token_usage: TokenUsage::new(1000, 500),
        };

        let cloned = output.clone();
        assert_eq!(cloned.result, "Test");
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_execution_result_clone() {
        let result = ExecutionResult {
            output: AgentOutput {
                result: "Test".to_string(),
                artifacts_created: vec![],
                delegation: None,
                token_usage: TokenUsage::new(1000, 500),
            },
            iterations: 5,
            total_tokens: 1500,
        };

        let cloned = result.clone();
        assert_eq!(cloned.iterations, 5);
        assert_eq!(cloned.total_tokens, 1500);
    }
}
