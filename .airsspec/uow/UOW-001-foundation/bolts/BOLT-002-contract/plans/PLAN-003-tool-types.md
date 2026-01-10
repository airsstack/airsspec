# PLAN-003: Tool Types

## Objective

Define tool types in `src/tool/types.rs`.

## Context

- **ADR Reference**: [ADR-002-contract.md](../../../adrs/ADR-002-contract.md)
- **Crate**: `airsspec-core`

## Steps

1. Create `src/tool/` directory
2. Create `src/tool/mod.rs` with module exports
3. Create `src/tool/types.rs` with:
   - `ToolId` newtype struct
   - `ToolInput` struct: `tool_id`, `params` (serde_json::Value)
   - `ToolOutput` struct: `success`, `result`, `error`
4. Implement `Hash`, `PartialEq`, `Eq` for `ToolId`
5. Derive serialization traits

## Verification

- [ ] `cargo build -p airsspec-core` passes
