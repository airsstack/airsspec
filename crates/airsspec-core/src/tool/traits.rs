//! Tool execution and registry traits.
//!
//! This module defines the core traits for tool management.
//! Tools are registered and executed by agents during the Construction phase.

// Layer 1: Standard library imports
// (None needed)

// Layer 2: Third-party crate imports
use async_trait::async_trait;

// Layer 3: Internal module imports
use crate::error::ToolError;
use crate::tool::types::{ToolId, ToolInput, ToolOutput};

/// Trait for tool execution.
///
/// This trait defines the interface for tools that can be executed by agents.
/// Implementations of this trait wrap external tool functionality and
/// provide a consistent interface for execution.
///
/// # Examples
///
/// ```rust,ignore
/// use airsspec_core::tool::types::{Tool, ToolId, ToolInput, ToolOutput};
/// use airsspec_core::error::ToolError;
/// use async_trait::async_trait;
///
/// struct MyTool;
///
/// #[async_trait]
/// impl Tool for MyTool {
///     fn id(&self) -> &ToolId {
///         &ToolId::new("bash")
///     }
///
///     fn name(&self) -> &str {
///         "Bash"
///     }
///
///     fn description(&self) -> &str {
///         "Execute bash commands"
///     }
///
///     async fn execute(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
///         // Implementation here
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait Tool: Send + Sync {
    /// Returns the unique identifier for this tool.
    ///
    /// The tool ID is used to register and retrieve the tool
    /// from the registry.
    ///
    /// # Returns
    ///
    /// A reference to the tool's unique identifier.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let tool_id = tool.id();
    /// println!("Tool ID: {}", tool_id);
    /// ```
    fn id(&self) -> &ToolId;

    /// Returns the human-readable name for this tool.
    ///
    /// This is a short name used for display purposes.
    ///
    /// # Returns
    ///
    /// The tool's human-readable name.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let name = tool.name();
    /// println!("Tool name: {}", name);
    /// ```
    fn name(&self) -> &str;

    /// Returns a description of this tool's functionality.
    ///
    /// This provides context to agents about when and how to use the tool.
    ///
    /// # Returns
    ///
    /// A description of the tool's functionality.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let description = tool.description();
    /// println!("Tool description: {}", description);
    /// ```
    fn description(&self) -> &str;

    /// Executes the tool with the provided input.
    ///
    /// This method performs the actual tool execution and returns
    /// the output or an error if execution fails.
    ///
    /// # Arguments
    ///
    /// * `input` - The tool input parameters
    ///
    /// # Returns
    ///
    /// The tool output if execution was successful, or an error.
    ///
    /// # Errors
    ///
    /// Returns a `ToolError` if tool execution fails.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let input = ToolInput::new(tool_id.clone(), json!({ "command": "ls" }));
    /// let output = tool.execute(input).await?;
    /// if output.success {
    ///     println!("Tool output: {}", output.result);
    /// }
    /// ```
    async fn execute(&self, input: ToolInput) -> Result<ToolOutput, ToolError>;
}

/// Trait for managing a registry of tools.
///
/// This trait defines the interface for registering, retrieving,
/// and listing available tools in a tool registry.
///
/// # Examples
///
/// ```rust,ignore
/// use airsspec_core::tool::types::{ToolRegistry, Tool, ToolId};
/// use std::sync::Arc;
///
/// struct MyRegistry;
///
/// impl ToolRegistry for MyRegistry {
///     fn register(&mut self, tool: Box<dyn Tool>) {
///         // Implementation here
///         todo!()
///     }
///
///     fn get(&self, id: &ToolId) -> Option<&dyn Tool> {
///         // Implementation here
///         todo!()
///     }
///
///     fn list(&self) -> Vec<&ToolId> {
///         // Implementation here
///         todo!()
///     }
/// }
/// ```
pub trait ToolRegistry: Send + Sync {
    /// Registers a tool in the registry.
    ///
    /// # Arguments
    ///
    /// * `tool` - The tool to register
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let tool = Box::new(MyTool);
    /// registry.register(tool);
    /// ```
    fn register(&mut self, tool: Box<dyn Tool>);

    /// Retrieves a tool by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The tool ID to look up
    ///
    /// # Returns
    ///
    /// `Some(&dyn Tool)` if the tool was found, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let tool_id = ToolId::new("bash");
    /// if let Some(tool) = registry.get(&tool_id) {
    ///     println!("Found tool: {}", tool.name());
    /// }
    /// ```
    fn get(&self, id: &ToolId) -> Option<&dyn Tool>;

    /// Lists all registered tool IDs.
    ///
    /// # Returns
    ///
    /// A vector of references to all registered tool IDs.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let tool_ids = registry.list();
    /// for id in tool_ids {
    ///     println!("Tool: {}", id);
    /// }
    /// ```
    fn list(&self) -> Vec<&ToolId>;
}
