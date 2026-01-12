//! Core agent type definitions.
//!
//! This module defines agent management types including agent identifiers,
//! configuration, budget constraints, and delegation signals.

// Layer 1: Standard library imports
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

// Layer 2: Third-party crate imports
use serde::{Deserialize, Serialize};

// Layer 3: Internal module imports
use crate::tool::types::ToolId;

/// Agent unique identifier.
///
/// Newtype pattern ensures type safety and prevents mixing with other IDs.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::agent::types::AgentId;
///
/// let agent_id = AgentId::new("researcher");
/// assert_eq!(agent_id.as_str(), "researcher");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentId(pub String);

impl AgentId {
    /// Creates a new agent ID from a string.
    ///
    /// # Arguments
    ///
    /// * `id` - The agent identifier string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::agent::types::AgentId;
    ///
    /// let agent_id = AgentId::new("builder");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns a reference to the inner string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::agent::types::AgentId;
    ///
    /// let agent_id = AgentId::new("reviewer");
    /// assert_eq!(agent_id.as_str(), "reviewer");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the wrapper and returns the inner string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::agent::types::AgentId;
    ///
    /// let agent_id = AgentId::new("builder");
    /// let inner: String = agent_id.into_inner();
    /// assert_eq!(inner, "builder");
    /// ```
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for AgentId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for AgentId {}

impl PartialOrd for AgentId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AgentId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Hash for AgentId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<S> From<S> for AgentId
where
    S: Into<String>,
{
    fn from(id: S) -> Self {
        Self::new(id)
    }
}

impl AsRef<str> for AgentId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Agent configuration.
///
/// Defines the behavior, capabilities, and constraints for an agent.
///
/// # Fields
///
/// * `id` - Unique identifier for this agent
/// * `name` - Human-readable name for this agent
/// * `description` - Detailed description of the agent's purpose and capabilities
/// * `allowed_tools` - List of tool IDs this agent is permitted to use
/// * `allowed_paths` - List of filesystem paths this agent may access
/// * `budget` - Resource constraints for agent execution
///
/// # Examples
///
/// ```rust
/// use airsspec_core::agent::types::{AgentId, AgentConfig, Budget};
/// use std::path::PathBuf;
///
/// let config = AgentConfig {
///     id: AgentId::new("builder"),
///     name: "Builder Agent".to_string(),
///     description: "Implements code changes during Construction phase".to_string(),
///     allowed_tools: vec!["write_code".into(), "read_file".into()],
///     allowed_paths: vec![PathBuf::from("crates/")],
///     budget: Budget::new(10000, 100, 300),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Unique identifier for this agent.
    pub id: AgentId,

    /// Human-readable name for this agent.
    pub name: String,

    /// Detailed description of the agent's purpose and capabilities.
    pub description: String,

    /// List of tool IDs this agent is permitted to use.
    pub allowed_tools: Vec<ToolId>,

    /// List of filesystem paths this agent may access.
    pub allowed_paths: Vec<PathBuf>,

    /// Resource constraints for agent execution.
    pub budget: Budget,
}

/// Execution budget for an agent.
///
/// Defines resource limits to prevent runaway execution.
///
/// # Fields
///
/// * `max_tokens` - Maximum total tokens allowed (input + output)
/// * `max_iterations` - Maximum number of execution iterations
/// * `timeout_secs` - Maximum execution time in seconds
///
/// # Examples
///
/// ```rust
/// use airsspec_core::agent::types::Budget;
///
/// let budget = Budget::new(10000, 100, 300);
/// assert_eq!(budget.max_tokens, 10000);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Budget {
    /// Maximum total tokens allowed (input + output).
    pub max_tokens: u32,

    /// Maximum number of execution iterations.
    pub max_iterations: u32,

    /// Maximum execution time in seconds.
    pub timeout_secs: u64,
}

impl Budget {
    /// Creates a new budget with specified limits.
    ///
    /// # Arguments
    ///
    /// * `max_tokens` - Maximum total tokens allowed
    /// * `max_iterations` - Maximum number of execution iterations
    /// * `timeout_secs` - Maximum execution time in seconds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::agent::types::Budget;
    ///
    /// let budget = Budget::new(10000, 100, 300);
    /// ```
    #[must_use]
    pub const fn new(max_tokens: u32, max_iterations: u32, timeout_secs: u64) -> Self {
        Self {
            max_tokens,
            max_iterations,
            timeout_secs,
        }
    }

    /// Checks if the budget has been exceeded.
    ///
    /// # Arguments
    ///
    /// * `used_tokens` - Number of tokens already used
    /// * `iterations` - Number of iterations already performed
    /// * `elapsed_secs` - Number of seconds elapsed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::agent::types::Budget;
    ///
    /// let budget = Budget::new(10000, 100, 300);
    /// assert!(!budget.exceeded(5000, 50, 150));
    /// assert!(budget.exceeded(15000, 150, 450));
    /// ```
    #[must_use]
    pub const fn exceeded(&self, used_tokens: u32, iterations: u32, elapsed_secs: u64) -> bool {
        used_tokens > self.max_tokens
            || iterations > self.max_iterations
            || elapsed_secs > self.timeout_secs
    }
}

/// Delegation signal between agents.
///
/// Agents use this signal to delegate work to other agents or signal completion.
///
/// # Variants
///
/// * `Delegate` - Delegate execution to another agent
/// * `Complete` - Signal successful completion with a result
/// * `Error` - Signal an error condition with a message
///
/// # Examples
///
/// ```rust
/// use airsspec_core::agent::types::{AgentId, DelegationSignal};
///
/// let signal = DelegationSignal::Delegate {
///     target: AgentId::new("reviewer"),
///     query: "Review my changes".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelegationSignal {
    /// Delegate execution to another agent.
    Delegate {
        /// The target agent to delegate to.
        target: AgentId,

        /// The query or task to delegate.
        query: String,
    },

    /// Signal successful completion with a result.
    Complete {
        /// The result of the agent's execution.
        result: String,
    },

    /// Signal an error condition.
    Error {
        /// The error message.
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_id_new() {
        let agent_id = AgentId::new("builder");
        assert_eq!(agent_id.as_str(), "builder");
    }

    #[test]
    fn test_agent_id_from() {
        let agent_id: AgentId = "reviewer".into();
        assert_eq!(agent_id.as_str(), "reviewer");
    }

    #[test]
    fn test_agent_id_into_inner() {
        let agent_id = AgentId::new("builder");
        let inner: String = agent_id.into_inner();
        assert_eq!(inner, "builder");
    }

    #[test]
    fn test_agent_id_display() {
        let agent_id = AgentId::new("builder");
        assert_eq!(format!("{agent_id}"), "builder");
    }

    #[test]
    fn test_agent_id_eq() {
        let id1 = AgentId::new("builder");
        let id2 = AgentId::new("builder");
        let id3 = AgentId::new("reviewer");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_agent_id_hash() {
        use std::collections::HashSet;

        let id1 = AgentId::new("builder");
        let id2 = AgentId::new("builder");
        let id3 = AgentId::new("reviewer");

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        set.insert(id3);

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_agent_id_ord() {
        let id1 = AgentId::new("builder");
        let id2 = AgentId::new("reviewer");

        assert!(id1 < id2);
    }

    #[test]
    fn test_agent_id_as_ref() {
        let agent_id = AgentId::new("builder");
        let s: &str = agent_id.as_ref();
        assert_eq!(s, "builder");
    }

    #[test]
    fn test_budget_new() {
        let budget = Budget::new(10000, 100, 300);
        assert_eq!(budget.max_tokens, 10000);
        assert_eq!(budget.max_iterations, 100);
        assert_eq!(budget.timeout_secs, 300);
    }

    #[test]
    fn test_budget_exceeded_not_exceeded() {
        let budget = Budget::new(10000, 100, 300);
        assert!(!budget.exceeded(5000, 50, 150));
    }

    #[test]
    fn test_budget_exceeded_tokens() {
        let budget = Budget::new(10000, 100, 300);
        assert!(budget.exceeded(15000, 50, 150));
    }

    #[test]
    fn test_budget_exceeded_iterations() {
        let budget = Budget::new(10000, 100, 300);
        assert!(budget.exceeded(5000, 150, 150));
    }

    #[test]
    fn test_budget_exceeded_timeout() {
        let budget = Budget::new(10000, 100, 300);
        assert!(budget.exceeded(5000, 50, 450));
    }

    #[test]
    fn test_delegation_signal_delegate() {
        let signal = DelegationSignal::Delegate {
            target: AgentId::new("reviewer"),
            query: "Review my changes".to_string(),
        };

        match signal {
            DelegationSignal::Delegate { target, query } => {
                assert_eq!(target.as_str(), "reviewer");
                assert_eq!(query, "Review my changes");
            }
            _ => panic!("Expected Delegate variant"),
        }
    }

    #[test]
    fn test_delegation_signal_complete() {
        let signal = DelegationSignal::Complete {
            result: "Task completed successfully".to_string(),
        };

        match signal {
            DelegationSignal::Complete { result } => {
                assert_eq!(result, "Task completed successfully");
            }
            _ => panic!("Expected Complete variant"),
        }
    }

    #[test]
    fn test_delegation_signal_error() {
        let signal = DelegationSignal::Error {
            message: "Something went wrong".to_string(),
        };

        match signal {
            DelegationSignal::Error { message } => {
                assert_eq!(message, "Something went wrong");
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_serialization_agent_id() {
        let agent_id = AgentId::new("builder");

        let json = serde_json::to_string(&agent_id).expect("Failed to serialize");
        assert!(json.contains("builder"));

        let deserialized: AgentId = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.as_str(), "builder");
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_serialization_budget() {
        let budget = Budget::new(10000, 100, 300);

        let json = serde_json::to_string(&budget).expect("Failed to serialize");
        assert!(json.contains("10000"));

        let deserialized: Budget = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.max_tokens, budget.max_tokens);
        assert_eq!(deserialized.max_iterations, budget.max_iterations);
        assert_eq!(deserialized.timeout_secs, budget.timeout_secs);
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_serialization_delegation_signal() {
        let signal = DelegationSignal::Complete {
            result: "Done".to_string(),
        };

        let json = serde_json::to_string(&signal).expect("Failed to serialize");
        assert!(json.contains("Done"));

        let deserialized: DelegationSignal =
            serde_json::from_str(&json).expect("Failed to deserialize");

        match deserialized {
            DelegationSignal::Complete { result } => {
                assert_eq!(result, "Done");
            }
            _ => panic!("Expected Complete variant"),
        }
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_clone_agent_id() {
        let agent_id = AgentId::new("builder");
        let cloned = agent_id.clone();
        assert_eq!(cloned.as_str(), "builder");
    }

    #[test]
    fn test_clone_budget() {
        let budget = Budget::new(10000, 100, 300);
        let cloned = budget;
        assert_eq!(cloned.max_tokens, 10000);
    }

    #[test]
    fn test_copy_budget() {
        let budget = Budget::new(10000, 100, 300);
        let copied = budget; // Budget is Copy
        assert_eq!(copied.max_tokens, 10000);
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_clone_delegation_signal() {
        let signal = DelegationSignal::Complete {
            result: "Test".to_string(),
        };
        let cloned = signal.clone();

        match cloned {
            DelegationSignal::Complete { result } => {
                assert_eq!(result, "Test");
            }
            _ => panic!("Expected Complete variant"),
        }
    }
}
