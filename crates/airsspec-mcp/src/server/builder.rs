//! Builder for constructing the `AirsSpec` MCP server.
//!
//! [`McpServerBuilder`] validates workspace configuration, creates the
//! handler with stub providers, builds the stdio transport, and returns
//! a ready-to-run [`McpServer`](airsprotocols_mcp::McpServer).

use std::path::PathBuf;
use std::sync::Arc;

use airsprotocols_mcp::McpServer;
use airsprotocols_mcp::protocol::ServerInfo;
use airsprotocols_mcp::protocol::Transport;
use airsprotocols_mcp::transport::adapters::stdio::StdioTransportBuilder;

use super::error::ServerError;
use super::handler::AirsSpecHandler;

/// Builder for constructing the `AirsSpec` MCP server.
///
/// Configures workspace path and debug mode, then builds a fully
/// configured [`McpServer`] with stdio transport.
///
/// # Examples
///
/// ```no_run
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use airsspec_mcp::server::McpServerBuilder;
///
/// let server = McpServerBuilder::new()
///     .workspace_path(std::path::PathBuf::from("/path/to/workspace"))
///     .debug(true)
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct McpServerBuilder {
    /// Workspace root directory.
    workspace_path: Option<PathBuf>,
    /// Whether debug mode is enabled.
    debug: bool,
}

impl McpServerBuilder {
    /// Create a new builder with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            workspace_path: None,
            debug: false,
        }
    }

    /// Set the workspace root directory.
    ///
    /// If not set, defaults to the current working directory.
    #[must_use]
    pub fn workspace_path(mut self, path: PathBuf) -> Self {
        self.workspace_path = Some(path);
        self
    }

    /// Enable or disable debug mode.
    #[must_use]
    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    /// Build the MCP server.
    ///
    /// Validates the workspace path, creates the handler with stub
    /// providers, builds the stdio transport, and returns a configured
    /// [`McpServer`].
    ///
    /// # Errors
    ///
    /// Returns [`ServerError::InvalidWorkspace`] if the workspace path
    /// does not exist or is not a directory.
    /// Returns [`ServerError::Transport`] if transport creation fails.
    pub async fn build(self) -> Result<McpServer<impl Transport>, ServerError> {
        // 1. Determine workspace path
        let workspace_path = match self.workspace_path {
            Some(path) => path,
            None => std::env::current_dir().map_err(|e| {
                ServerError::InvalidWorkspace(format!("cannot determine current directory: {e}"))
            })?,
        };

        // 2. Validate workspace path
        if !workspace_path.exists() {
            return Err(ServerError::InvalidWorkspace(format!(
                "workspace path does not exist: {}",
                workspace_path.display()
            )));
        }
        if !workspace_path.is_dir() {
            return Err(ServerError::InvalidWorkspace(format!(
                "workspace path is not a directory: {}",
                workspace_path.display()
            )));
        }

        // 3. Create server info
        let server_info = ServerInfo {
            name: String::from("airsspec"),
            version: String::from(env!("CARGO_PKG_VERSION")),
        };

        // 4. Create handler (writes directly to stdout, no transport ref needed)
        let handler = Arc::new(AirsSpecHandler::new(server_info));

        // 5. Build stdio transport with handler
        let session_id = uuid::Uuid::new_v4().to_string();
        let transport = StdioTransportBuilder::new()
            .with_message_handler(handler)
            .with_session_id(session_id)
            .build()
            .await
            .map_err(ServerError::Transport)?;

        // 6. Wrap in McpServer lifecycle manager
        Ok(McpServer::new(transport))
    }
}

impl Default for McpServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_builder_validates_nonexistent_path() {
        let result = McpServerBuilder::new()
            .workspace_path(PathBuf::from("/nonexistent/path/to/workspace"))
            .build()
            .await;

        let Err(err) = result else {
            panic!("expected error for nonexistent path");
        };
        let msg = err.to_string();
        assert!(
            msg.contains("does not exist"),
            "expected 'does not exist' in error: {msg}"
        );
    }

    #[tokio::test]
    async fn test_builder_validates_file_not_directory() {
        let temp = tempfile::NamedTempFile::new().expect("failed to create temp file");
        let result = McpServerBuilder::new()
            .workspace_path(temp.path().to_path_buf())
            .build()
            .await;

        let Err(err) = result else {
            panic!("expected error for file path");
        };
        let msg = err.to_string();
        assert!(
            msg.contains("not a directory"),
            "expected 'not a directory' in error: {msg}"
        );
    }

    #[test]
    fn test_builder_default_values() {
        let builder = McpServerBuilder::new();
        let debug_output = format!("{builder:?}");
        assert!(debug_output.contains("workspace_path: None"));
        assert!(debug_output.contains("debug: false"));
    }

    #[test]
    fn test_builder_fluent_api() {
        let builder = McpServerBuilder::new()
            .workspace_path(PathBuf::from("/some/path"))
            .debug(true);

        let debug_output = format!("{builder:?}");
        assert!(debug_output.contains("/some/path"));
        assert!(debug_output.contains("debug: true"));
    }
}
