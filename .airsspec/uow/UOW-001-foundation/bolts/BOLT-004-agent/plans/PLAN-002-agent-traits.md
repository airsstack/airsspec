# PLAN-002: Agent Traits

## Objective

Define agent and executor traits in `src/agent/traits.rs`.

## Context

- **ADR Reference**: [ADR-004-agent.md](../../../adrs/ADR-004-agent.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/agent/traits.rs`
2. Define `Agent` trait:
   - `fn id(&self) -> &AgentId`
   - `fn config(&self) -> &AgentConfig`
   - `fn allowed_tools(&self) -> &[ToolId]`
   - `fn allowed_paths(&self) -> &[PathBuf]`
   - `async fn execute(&self, context: AgentContext) -> Result<AgentOutput, AgentError>`
3. Define `AgentExecutor` trait:
   - `async fn run(&self, agent: &dyn Agent, budget: Budget) -> Result<ExecutionResult, ExecutionError>`
4. Define `AgentContext` struct and `AgentOutput` struct
5. Add `Send + Sync` bounds

## Verification

- [ ] `cargo build -p airsspec-core` passes
