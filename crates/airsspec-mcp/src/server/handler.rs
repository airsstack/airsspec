//! Central message handler for the `AirsSpec` MCP server.
//!
//! This module provides [`AirsSpecHandler`], the core JSON-RPC routing component
//! that implements [`MessageHandler`](airsprotocols_mcp::MessageHandler) from
//! `airsprotocols-mcp`. The handler routes incoming requests to the appropriate
//! provider (tools, resources, prompts) and writes responses directly to stdout.
//!
//! # Architecture
//!
//! The handler is split into two layers:
//!
//! 1. **Routing + Response Construction (testable):** [`AirsSpecHandler::route_request`]
//!    takes a [`JsonRpcRequest`] and returns a [`JsonRpcResponse`] -- no I/O.
//! 2. **I/O Layer (thin):** The private `send_response` method writes a
//!    [`JsonRpcResponse`] to stdout. The [`MessageHandler::handle_message`]
//!    implementation glues the two layers together.
//!
//! Tests call `route_request()` directly and assert on the returned response.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

use airsprotocols_mcp::McpResult;
use airsprotocols_mcp::protocol::constants::methods;
use airsprotocols_mcp::protocol::{
    CallToolRequest, CallToolResult, Content, GetPromptRequest, GetPromptResult, InitializeRequest,
    InitializeResponse, JsonRpcMessage, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse,
    ListPromptsResult, ListResourceTemplatesResult, ListResourcesResult, ListToolsResult,
    LoggingCapabilities, MessageContext, MessageHandler, PromptCapabilities, ReadResourceRequest,
    ReadResourceResult, ResourceCapabilities, ServerCapabilities, ServerInfo, ToolCapabilities,
    TransportError,
};
use airsprotocols_mcp::providers::{PromptProvider, ResourceProvider, ToolProvider};

// ---------------------------------------------------------------------------
// Stub providers (replaced by real implementations in Tasks 5.3-5.6)
// ---------------------------------------------------------------------------

/// Stub tool provider returning empty lists.
///
/// Will be replaced by real implementation in Task 5.3.
struct StubToolProvider;

#[async_trait]
impl ToolProvider for StubToolProvider {
    async fn list_tools(&self) -> McpResult<Vec<airsprotocols_mcp::protocol::Tool>> {
        Ok(vec![])
    }

    async fn call_tool(&self, name: &str, _arguments: Value) -> McpResult<Vec<Content>> {
        Err(airsprotocols_mcp::McpError::tool_not_found(name))
    }
}

/// Stub resource provider returning empty lists.
///
/// Will be replaced by real implementation in Task 5.5.
struct StubResourceProvider;

#[async_trait]
impl ResourceProvider for StubResourceProvider {
    async fn list_resources(&self) -> McpResult<Vec<airsprotocols_mcp::protocol::Resource>> {
        Ok(vec![])
    }

    async fn read_resource(&self, uri: &str) -> McpResult<Vec<Content>> {
        Err(airsprotocols_mcp::McpError::resource_not_found(uri))
    }
}

/// Stub prompt provider returning empty lists.
///
/// Will be replaced by real implementation in Task 5.6.
struct StubPromptProvider;

#[async_trait]
impl PromptProvider for StubPromptProvider {
    async fn list_prompts(&self) -> McpResult<Vec<airsprotocols_mcp::protocol::Prompt>> {
        Ok(vec![])
    }

    async fn get_prompt(
        &self,
        name: &str,
        _arguments: HashMap<String, String>,
    ) -> McpResult<(String, Vec<airsprotocols_mcp::protocol::PromptMessage>)> {
        Err(airsprotocols_mcp::McpError::prompt_not_found(name))
    }
}

// ---------------------------------------------------------------------------
// AirsSpecHandler
// ---------------------------------------------------------------------------

/// Central message handler for the `AirsSpec` MCP server.
///
/// Routes incoming JSON-RPC messages to the appropriate provider
/// (tools, resources, prompts) and writes responses directly to stdout.
///
/// This handler is self-contained -- it does not hold a reference to the
/// transport. The transport handles reading from stdin; the handler handles
/// writing to stdout.
///
/// # Dyn usage
///
/// The `Arc<dyn ...>` fields use dynamic dispatch because the provider
/// traits are defined by the external `airsprotocols-mcp` library and
/// already use `#[async_trait]` which boxes internally. Our own internal
/// traits (in `airsspec-core`) continue to use generics/static dispatch.
pub struct AirsSpecHandler {
    /// Server identity (name + version).
    server_info: ServerInfo,
    /// Advertised server capabilities.
    capabilities: ServerCapabilities,
    /// Tool provider for `tools/list` and `tools/call`.
    tool_provider: Arc<dyn ToolProvider>,
    /// Resource provider for `resources/list` and `resources/read`.
    resource_provider: Arc<dyn ResourceProvider>,
    /// Prompt provider for `prompts/list` and `prompts/get`.
    prompt_provider: Arc<dyn PromptProvider>,
}

impl std::fmt::Debug for AirsSpecHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AirsSpecHandler")
            .field("server_info", &self.server_info)
            .finish_non_exhaustive()
    }
}

impl AirsSpecHandler {
    /// Create a new handler with stub providers.
    ///
    /// The stub providers return empty lists and errors for all operations.
    /// Replace them with real implementations using [`with_providers`](Self::with_providers).
    #[must_use]
    pub fn new(server_info: ServerInfo) -> Self {
        Self {
            server_info,
            capabilities: Self::default_capabilities(),
            tool_provider: Arc::new(StubToolProvider),
            resource_provider: Arc::new(StubResourceProvider),
            prompt_provider: Arc::new(StubPromptProvider),
        }
    }

    /// Create a handler with custom providers.
    ///
    /// Used by Tasks 5.3-5.6 to plug in real tool, resource, and prompt
    /// implementations.
    #[must_use]
    pub fn with_providers(
        server_info: ServerInfo,
        tool_provider: Arc<dyn ToolProvider>,
        resource_provider: Arc<dyn ResourceProvider>,
        prompt_provider: Arc<dyn PromptProvider>,
    ) -> Self {
        Self {
            server_info,
            capabilities: Self::default_capabilities(),
            tool_provider,
            resource_provider,
            prompt_provider,
        }
    }

    /// Build the default set of server capabilities.
    fn default_capabilities() -> ServerCapabilities {
        ServerCapabilities {
            experimental: Some(serde_json::json!({})),
            logging: Some(LoggingCapabilities {}),
            prompts: Some(PromptCapabilities::default()),
            resources: Some(ResourceCapabilities::default()),
            tools: Some(ToolCapabilities::default()),
        }
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Serialize a result value into a success response, or return an
    /// internal error response if serialization fails.
    fn serialize_result(
        value: &impl serde::Serialize,
        id: airsprotocols_mcp::protocol::RequestId,
        context: &str,
    ) -> JsonRpcResponse {
        match serde_json::to_value(value) {
            Ok(v) => JsonRpcResponse::success(v, id),
            Err(e) => JsonRpcResponse::internal_error(
                &format!("failed to serialize {context}: {e}"),
                None,
                Some(id),
            ),
        }
    }

    // -----------------------------------------------------------------------
    // Layer 1: Pure routing -- returns response, testable
    // -----------------------------------------------------------------------

    /// Route a JSON-RPC request to the appropriate handler method.
    ///
    /// Returns a [`JsonRpcResponse`] -- does **not** perform I/O.
    /// This method is the primary target for unit testing.
    pub(crate) async fn route_request(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            methods::INITIALIZE => self.handle_initialize(request),
            methods::PING => Self::handle_ping(request),
            methods::TOOLS_LIST => self.handle_tools_list(request).await,
            methods::TOOLS_CALL => self.handle_tools_call(request).await,
            methods::RESOURCES_LIST => self.handle_resources_list(request).await,
            methods::RESOURCES_READ => self.handle_resources_read(request).await,
            methods::RESOURCES_TEMPLATES_LIST => {
                self.handle_resources_templates_list(request).await
            }
            methods::PROMPTS_LIST => self.handle_prompts_list(request).await,
            methods::PROMPTS_GET => self.handle_prompts_get(request).await,
            _ => JsonRpcResponse::method_not_found(&request.method, Some(request.id.clone())),
        }
    }

    // -- Initialize ---------------------------------------------------------

    fn handle_initialize(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        // Parse InitializeRequest from params (optional -- some clients send empty params)
        if let Some(ref params) = request.params
            && serde_json::from_value::<InitializeRequest>(params.clone()).is_err()
        {
            return JsonRpcResponse::invalid_params(
                "invalid initialize params",
                None,
                Some(request.id.clone()),
            );
        }

        let capabilities_value = match serde_json::to_value(&self.capabilities) {
            Ok(v) => v,
            Err(e) => {
                return JsonRpcResponse::internal_error(
                    &format!("failed to serialize capabilities: {e}"),
                    None,
                    Some(request.id.clone()),
                );
            }
        };

        let init_response =
            InitializeResponse::new(capabilities_value, self.server_info.clone(), None);

        Self::serialize_result(&init_response, request.id.clone(), "initialize response")
    }

    // -- Ping ---------------------------------------------------------------

    fn handle_ping(request: &JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse::success(serde_json::json!({}), request.id.clone())
    }

    // -- Tools --------------------------------------------------------------

    async fn handle_tools_list(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        match self.tool_provider.list_tools().await {
            Ok(tools) => {
                let result = ListToolsResult::new(tools);
                Self::serialize_result(&result, request.id.clone(), "tools list")
            }
            Err(e) => JsonRpcResponse::internal_error(
                &format!("tools list error: {e}"),
                None,
                Some(request.id.clone()),
            ),
        }
    }

    async fn handle_tools_call(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let Some(call_request) = request
            .params
            .as_ref()
            .and_then(|p| serde_json::from_value::<CallToolRequest>(p.clone()).ok())
        else {
            return JsonRpcResponse::invalid_params(
                "invalid tools/call params: expected {name, arguments}",
                None,
                Some(request.id.clone()),
            );
        };

        match self
            .tool_provider
            .call_tool(&call_request.name, call_request.arguments)
            .await
        {
            Ok(content) => {
                let result = CallToolResult::success(content);
                Self::serialize_result(&result, request.id.clone(), "tool result")
            }
            Err(e) => {
                // MCP convention: tool errors are returned as result with is_error=true
                let result = CallToolResult::error(vec![Content::text(e.to_string())]);
                Self::serialize_result(&result, request.id.clone(), "tool error")
            }
        }
    }

    // -- Resources ----------------------------------------------------------

    async fn handle_resources_list(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        match self.resource_provider.list_resources().await {
            Ok(resources) => {
                let result = ListResourcesResult::new(resources);
                Self::serialize_result(&result, request.id.clone(), "resources list")
            }
            Err(e) => JsonRpcResponse::internal_error(
                &format!("resources list error: {e}"),
                None,
                Some(request.id.clone()),
            ),
        }
    }

    async fn handle_resources_read(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let Some(read_request) = request
            .params
            .as_ref()
            .and_then(|p| serde_json::from_value::<ReadResourceRequest>(p.clone()).ok())
        else {
            return JsonRpcResponse::invalid_params(
                "invalid resources/read params: expected {uri}",
                None,
                Some(request.id.clone()),
            );
        };

        match self
            .resource_provider
            .read_resource(read_request.uri.as_str())
            .await
        {
            Ok(contents) => {
                let result = ReadResourceResult::new(contents);
                Self::serialize_result(&result, request.id.clone(), "resource result")
            }
            Err(e) => JsonRpcResponse::internal_error(
                &format!("resource read error: {e}"),
                None,
                Some(request.id.clone()),
            ),
        }
    }

    async fn handle_resources_templates_list(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        match self.resource_provider.list_resource_templates().await {
            Ok(templates) => {
                let result = ListResourceTemplatesResult::new(templates);
                Self::serialize_result(&result, request.id.clone(), "resource templates list")
            }
            Err(e) => JsonRpcResponse::internal_error(
                &format!("resource templates list error: {e}"),
                None,
                Some(request.id.clone()),
            ),
        }
    }

    // -- Prompts ------------------------------------------------------------

    async fn handle_prompts_list(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        match self.prompt_provider.list_prompts().await {
            Ok(prompts) => {
                let result = ListPromptsResult::new(prompts);
                Self::serialize_result(&result, request.id.clone(), "prompts list")
            }
            Err(e) => JsonRpcResponse::internal_error(
                &format!("prompts list error: {e}"),
                None,
                Some(request.id.clone()),
            ),
        }
    }

    async fn handle_prompts_get(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let Some(get_request) = request
            .params
            .as_ref()
            .and_then(|p| serde_json::from_value::<GetPromptRequest>(p.clone()).ok())
        else {
            return JsonRpcResponse::invalid_params(
                "invalid prompts/get params: expected {name, arguments}",
                None,
                Some(request.id.clone()),
            );
        };

        match self
            .prompt_provider
            .get_prompt(&get_request.name, get_request.arguments)
            .await
        {
            Ok((description, messages)) => {
                let result = GetPromptResult::new(Some(description), messages);
                Self::serialize_result(&result, request.id.clone(), "prompt result")
            }
            Err(e) => JsonRpcResponse::internal_error(
                &format!("prompt get error: {e}"),
                None,
                Some(request.id.clone()),
            ),
        }
    }

    // -----------------------------------------------------------------------
    // Layer 2: I/O -- writes to stdout
    // -----------------------------------------------------------------------

    /// Write a JSON-RPC response directly to stdout.
    ///
    /// This is the only method that performs I/O. All other handler methods
    /// are pure functions that return [`JsonRpcResponse`].
    async fn send_response(&self, response: JsonRpcResponse) {
        use tokio::io::AsyncWriteExt;

        let message = JsonRpcMessage::Response(response);
        match serde_json::to_string(&message) {
            Ok(json) => {
                let mut out = tokio::io::stdout();
                if let Err(e) = out.write_all(json.as_bytes()).await {
                    tracing::error!("Failed to write response to stdout: {e}");
                    return;
                }
                if let Err(e) = out.write_all(b"\n").await {
                    tracing::error!("Failed to write newline to stdout: {e}");
                    return;
                }
                if let Err(e) = out.flush().await {
                    tracing::error!("Failed to flush stdout: {e}");
                }
            }
            Err(e) => {
                tracing::error!("Failed to serialize response: {e}");
            }
        }
    }

    // -----------------------------------------------------------------------
    // Notification handling (fire-and-forget, no response)
    // -----------------------------------------------------------------------

    /// Handle a JSON-RPC notification.
    ///
    /// Notifications do not receive responses per the JSON-RPC 2.0 spec.
    fn handle_notification(notification: &JsonRpcNotification) {
        match notification.method.as_str() {
            methods::INITIALIZED => {
                tracing::info!("Client confirmed initialization");
            }
            other => {
                tracing::debug!("Received notification: {other}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// MessageHandler implementation
// ---------------------------------------------------------------------------

#[async_trait]
impl MessageHandler<()> for AirsSpecHandler {
    async fn handle_message(&self, message: JsonRpcMessage, _context: MessageContext<()>) {
        match message {
            JsonRpcMessage::Request(request) => {
                let response = self.route_request(&request).await;
                self.send_response(response).await;
            }
            JsonRpcMessage::Notification(ref notification) => {
                Self::handle_notification(notification);
            }
            JsonRpcMessage::Response(_) => {
                // Server should not receive responses; log and ignore
                tracing::warn!("Received unexpected response message");
            }
        }
    }

    async fn handle_error(&self, error: TransportError) {
        tracing::error!("Transport error: {error}");
    }

    async fn handle_close(&self) {
        tracing::info!("MCP transport closed");
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use airsprotocols_mcp::protocol::RequestId;
    use airsprotocols_mcp::protocol::constants::error_codes;

    /// Create a handler for testing with default stub providers.
    fn test_handler() -> AirsSpecHandler {
        AirsSpecHandler::new(ServerInfo {
            name: String::from("test-server"),
            version: String::from("0.1.0"),
        })
    }

    /// Build a JSON-RPC request for testing.
    fn make_request(method: &str, id: i64, params: Option<Value>) -> JsonRpcRequest {
        JsonRpcRequest::new(method, params, RequestId::new_number(id))
    }

    #[tokio::test]
    async fn test_handle_initialize_returns_server_info() {
        let handler = test_handler();
        let params = serde_json::json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": { "name": "test-client", "version": "1.0.0" }
        });
        let request = make_request("initialize", 1, Some(params));

        let response = handler.route_request(&request).await;

        assert!(response.error.is_none(), "expected success, got error");
        let result = response.result.expect("expected result");
        assert_eq!(result["serverInfo"]["name"], "test-server");
        assert_eq!(result["serverInfo"]["version"], "0.1.0");
        assert!(result["capabilities"]["tools"].is_object());
        assert!(result["capabilities"]["resources"].is_object());
        assert!(result["capabilities"]["prompts"].is_object());
    }

    #[tokio::test]
    async fn test_handle_ping_returns_empty_object() {
        let handler = test_handler();
        let request = make_request("ping", 2, None);

        let response = handler.route_request(&request).await;

        assert!(response.error.is_none());
        let result = response.result.expect("expected result");
        assert_eq!(result, serde_json::json!({}));
    }

    #[tokio::test]
    async fn test_handle_unknown_method_returns_error() {
        let handler = test_handler();
        let request = make_request("nonexistent/method", 3, None);

        let response = handler.route_request(&request).await;

        assert!(response.result.is_none(), "expected error, got result");
        let error = response.error.expect("expected error");
        assert_eq!(error["code"], error_codes::METHOD_NOT_FOUND);
        let message = error["message"].as_str().unwrap_or_default();
        assert!(
            message.contains("nonexistent/method"),
            "error message should contain method name, got: {message}"
        );
    }

    #[tokio::test]
    async fn test_handle_tools_list_returns_empty() {
        let handler = test_handler();
        let request = make_request("tools/list", 4, None);

        let response = handler.route_request(&request).await;

        assert!(response.error.is_none());
        let result = response.result.expect("expected result");
        assert_eq!(result["tools"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn test_handle_resources_list_returns_empty() {
        let handler = test_handler();
        let request = make_request("resources/list", 5, None);

        let response = handler.route_request(&request).await;

        assert!(response.error.is_none());
        let result = response.result.expect("expected result");
        assert_eq!(result["resources"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn test_handle_prompts_list_returns_empty() {
        let handler = test_handler();
        let request = make_request("prompts/list", 6, None);

        let response = handler.route_request(&request).await;

        assert!(response.error.is_none());
        let result = response.result.expect("expected result");
        assert_eq!(result["prompts"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn test_handle_tools_call_stub_returns_error() {
        let handler = test_handler();
        let params = serde_json::json!({
            "name": "spec_create",
            "arguments": {}
        });
        let request = make_request("tools/call", 7, Some(params));

        let response = handler.route_request(&request).await;

        // MCP convention: tool errors returned as result with is_error=true
        assert!(response.error.is_none(), "tool errors come back as result");
        let result = response.result.expect("expected result");
        assert_eq!(result["is_error"], true);
        let content = result["content"]
            .as_array()
            .expect("expected content array");
        assert!(!content.is_empty());
    }

    #[tokio::test]
    async fn test_handle_ping_preserves_request_id() {
        let handler = test_handler();
        let request = make_request("ping", 42, None);

        let response = handler.route_request(&request).await;

        assert_eq!(response.id, Some(RequestId::new_number(42)));
    }

    #[tokio::test]
    async fn test_notification_handling_does_not_panic() {
        let notification = JsonRpcNotification::new("notifications/initialized", None);
        // Should not panic
        AirsSpecHandler::handle_notification(&notification);
    }

    #[tokio::test]
    async fn test_handle_resources_templates_list_returns_empty() {
        let handler = test_handler();
        let request = make_request("resources/templates/list", 10, None);

        let response = handler.route_request(&request).await;

        assert!(response.error.is_none());
        let result = response.result.expect("expected result");
        assert_eq!(result["resource_templates"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn test_handle_prompts_get_stub_returns_error() {
        let handler = test_handler();
        let params = serde_json::json!({
            "name": "some-prompt",
            "arguments": {}
        });
        let request = make_request("prompts/get", 11, Some(params));

        let response = handler.route_request(&request).await;

        assert!(response.result.is_none());
        let error = response.error.expect("expected error");
        assert_eq!(error["code"], error_codes::INTERNAL_ERROR);
    }

    #[tokio::test]
    async fn test_handle_resources_read_stub_returns_error() {
        let handler = test_handler();
        let params = serde_json::json!({
            "uri": "airsspec:///specs"
        });
        let request = make_request("resources/read", 12, Some(params));

        let response = handler.route_request(&request).await;

        assert!(response.result.is_none());
        let error = response.error.expect("expected error");
        assert_eq!(error["code"], error_codes::INTERNAL_ERROR);
    }
}
