# PLAN-004: Tool Traits

## Objective

Define tool execution and registry traits in `src/tool/traits.rs`.

## Context

- **ADR Reference**: [ADR-002-contract.md](../../../adrs/ADR-002-contract.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/tool/traits.rs`
2. Define `Tool` trait:
   - `fn id(&self) -> &ToolId`
   - `fn name(&self) -> &str`
   - `fn description(&self) -> &str`
   - `async fn execute(&self, input: ToolInput) -> Result<ToolOutput, ToolError>`
3. Define `ToolRegistry` trait:
   - `fn register(&mut self, tool: Box<dyn Tool>)`
   - `fn get(&self, id: &ToolId) -> Option<&dyn Tool>`
   - `fn list(&self) -> Vec<&ToolId>`
4. Add `Send + Sync` bounds

## Verification

- [ ] `cargo build -p airsspec-core` passes
