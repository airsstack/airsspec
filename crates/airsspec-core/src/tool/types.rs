//! Core tool type definitions.
//!
//! This module defines tool management types.

// Layer 1: Standard library imports
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

// Layer 2: Third-party crate imports
use serde::{Deserialize, Serialize};

// Layer 3: Internal module imports (none - this is a types module)

/// Tool unique identifier.
///
/// Newtype pattern ensures type safety and prevents mixing with other IDs.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::tool::types::ToolId;
///
/// let tool_id = ToolId::new("read_file");
/// assert_eq!(tool_id.as_str(), "read_file");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolId(pub String);

impl ToolId {
    /// Creates a new tool ID from a string.
    ///
    /// # Arguments
    ///
    /// * `id` - The tool identifier string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::tool::types::ToolId;
    ///
    /// let tool_id = ToolId::new("bash");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns a reference to the inner string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::tool::types::ToolId;
    ///
    /// let tool_id = ToolId::new("read_file");
    /// assert_eq!(tool_id.as_str(), "read_file");
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
    /// use airsspec_core::tool::types::ToolId;
    ///
    /// let tool_id = ToolId::new("bash");
    /// let inner: String = tool_id.into_inner();
    /// assert_eq!(inner, "bash");
    /// ```
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for ToolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for ToolId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ToolId {}

impl PartialOrd for ToolId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ToolId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Hash for ToolId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<S> From<S> for ToolId
where
    S: Into<String>,
{
    fn from(id: S) -> Self {
        Self::new(id)
    }
}

impl AsRef<str> for ToolId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Tool input parameters.
///
/// Contains tool ID and JSON parameters for flexible execution.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::tool::types::{ToolId, ToolInput};
/// use serde_json::json;
///
/// let input = ToolInput {
///     tool_id: ToolId::new("read_file"),
///     params: json!({ "path": "file.txt" }),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInput {
    /// The tool to execute.
    pub tool_id: ToolId,

    /// Tool parameters as JSON value.
    pub params: serde_json::Value,
}

impl ToolInput {
    /// Creates a new tool input.
    ///
    /// # Arguments
    ///
    /// * `tool_id` - The tool to execute
    /// * `params` - Tool parameters as JSON value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::tool::types::{ToolId, ToolInput};
    /// use serde_json::json;
    ///
    /// let input = ToolInput::new(ToolId::new("bash"), json!({ "command": "ls" }));
    /// ```
    #[must_use]
    pub const fn new(tool_id: ToolId, params: serde_json::Value) -> Self {
        Self { tool_id, params }
    }
}

/// Tool execution result.
///
/// Contains execution outcome including success status, result, and errors.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::tool::types::ToolOutput;
/// use serde_json::json;
///
/// let output = ToolOutput {
///     success: true,
///     result: json!({ "content": "Hello, world!" }),
///     error: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    /// Whether the tool execution succeeded.
    pub success: bool,

    /// The tool output result.
    pub result: serde_json::Value,

    /// Error message if execution failed.
    pub error: Option<String>,
}

impl ToolOutput {
    /// Creates a successful tool output.
    ///
    /// # Arguments
    ///
    /// * `result` - The tool output result
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::tool::types::ToolOutput;
    /// use serde_json::json;
    ///
    /// let output = ToolOutput::success(json!({ "count": 42 }));
    /// assert!(output.success);
    /// ```
    #[must_use]
    pub const fn success(result: serde_json::Value) -> Self {
        Self {
            success: true,
            result,
            error: None,
        }
    }

    /// Creates a failed tool output.
    ///
    /// # Arguments
    ///
    /// * `error` - The error message
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::tool::types::ToolOutput;
    ///
    /// let output = ToolOutput::failure("File not found".to_string());
    /// assert!(!output.success);
    /// assert_eq!(output.error, Some("File not found".to_string()));
    /// ```
    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            success: false,
            result: serde_json::Value::Null,
            error: Some(error.into()),
        }
    }
}

impl Default for ToolOutput {
    fn default() -> Self {
        Self {
            success: false,
            result: serde_json::Value::Null,
            error: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_id_new() {
        let tool_id = ToolId::new("bash");
        assert_eq!(tool_id.as_str(), "bash");
    }

    #[test]
    fn test_tool_id_from() {
        let tool_id: ToolId = "bash".into();
        assert_eq!(tool_id.as_str(), "bash");
    }

    #[test]
    fn test_tool_id_into_inner() {
        let tool_id = ToolId::new("bash");
        let inner: String = tool_id.into_inner();
        assert_eq!(inner, "bash");
    }

    #[test]
    fn test_tool_id_serialization() {
        let tool_id = ToolId::new("bash");

        // Test serialization
        if let Ok(serialized) = serde_json::to_string(&tool_id) {
            assert!(serialized.contains("bash"));

            // Test deserialization
            if let Ok(deserialized) = serde_json::from_str::<ToolId>(&serialized) {
                assert_eq!(deserialized.as_str(), "bash");
            } else {
                panic!("Failed to deserialize ToolId");
            }
        } else {
            panic!("Failed to serialize ToolId");
        }
    }

    #[test]
    fn test_tool_input_serialization() {
        let input = ToolInput::new(ToolId::new("bash"), json!({ "command": "ls" }));

        // Test serialization
        if let Ok(serialized) = serde_json::to_string(&input) {
            // Test deserialization
            if let Ok(deserialized) = serde_json::from_str::<ToolInput>(&serialized) {
                assert_eq!(deserialized.tool_id.as_str(), "bash");
                assert_eq!(deserialized.params["command"], "ls");
            } else {
                panic!("Failed to deserialize ToolInput");
            }
        } else {
            panic!("Failed to serialize ToolInput");
        }
    }

    #[test]
    fn test_tool_output_serialization() {
        let output = ToolOutput::success(json!({ "count": 42 }));

        // Test serialization
        if let Ok(serialized) = serde_json::to_string(&output) {
            // Test deserialization
            if let Ok(deserialized) = serde_json::from_str::<ToolOutput>(&serialized) {
                assert!(deserialized.success);
                assert_eq!(deserialized.result["count"], 42);
            } else {
                panic!("Failed to deserialize ToolOutput");
            }
        } else {
            panic!("Failed to serialize ToolOutput");
        }
    }

    #[test]
    fn test_tool_id_display() {
        let tool_id = ToolId::new("bash");
        assert_eq!(format!("{tool_id}"), "bash");
    }

    #[test]
    fn test_tool_id_eq() {
        let id1 = ToolId::new("bash");
        let id2 = ToolId::new("bash");
        let id3 = ToolId::new("read");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_tool_id_hash() {
        use std::collections::HashSet;

        let id1 = ToolId::new("bash");
        let id2 = ToolId::new("bash");
        let id3 = ToolId::new("read");

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        set.insert(id3);

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_tool_id_ord() {
        let id1 = ToolId::new("bash");
        let id2 = ToolId::new("read");

        assert!(id1 < id2);
    }

    #[test]
    fn test_tool_id_as_ref() {
        let tool_id = ToolId::new("bash");
        let s: &str = tool_id.as_ref();
        assert_eq!(s, "bash");
    }

    #[test]
    fn test_tool_input_new() {
        let input = ToolInput::new(ToolId::new("bash"), json!({ "command": "ls" }));
        assert_eq!(input.tool_id.as_str(), "bash");
        assert_eq!(input.params["command"], "ls");
    }

    #[test]
    fn test_tool_output_success() {
        let output = ToolOutput::success(json!({ "count": 42 }));
        assert!(output.success);
        assert_eq!(output.result["count"], 42);
        assert!(output.error.is_none());
    }

    #[test]
    fn test_tool_output_failure() {
        let output = ToolOutput::failure("File not found".to_string());
        assert!(!output.success);
        assert_eq!(output.error, Some("File not found".to_string()));
        assert_eq!(output.result, serde_json::Value::Null);
    }

    #[test]
    fn test_tool_output_default() {
        let output = ToolOutput::default();
        assert!(!output.success);
        assert_eq!(output.result, serde_json::Value::Null);
        assert!(output.error.is_none());
    }
}
