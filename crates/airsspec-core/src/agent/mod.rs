//! Agent execution and management types.
//!
//! This module defines the core types for agent configuration, execution context,
//! and delegation signals in the `AirsSpec` framework.

pub mod traits;
pub mod types;

// Public re-exports for convenience
pub use traits::{
    Agent, AgentContext, AgentExecutor, AgentOutput, ExecutionError, ExecutionResult, TokenUsage,
};
pub use types::{AgentConfig, AgentId, Budget, DelegationSignal};
