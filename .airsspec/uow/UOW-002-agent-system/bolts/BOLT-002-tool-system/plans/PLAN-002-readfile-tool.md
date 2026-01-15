# PLAN-002: ReadFile Tool

## Objective

Implement `ReadFileTool` for reading file contents with sandbox validation.

## Context

First core tool implementing the `Tool` trait. Demonstrates sandbox integration and parameter schema definition.

## Steps

1. Create `src/tools/mod.rs` and `src/tools/read_file.rs`
2. Implement `ReadFileTool` struct
3. Implement `Tool` trait:
   - `name()` returns "read_file"
   - `description()` explains the tool
   - `parameter_schema()` returns JSON schema for parameters
   - `execute()` reads file with sandbox check
4. Add parameter validation (path must be string)
5. Use `tokio::fs::read_to_string()` for async file I/O
6. Write unit tests:
   - Successful read
   - File not found
   - Sandbox violation

## Expected Output

### Files Created:
- `crates/airsspec-tools/src/tools/mod.rs`
- `crates/airsspec-tools/src/tools/read_file.rs`

### Files Modified:
- `crates/airsspec-tools/src/lib.rs` — Export ReadFileTool

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] Can read allowed files
- [ ] Blocks unauthorized paths
- [ ] Parameter schema is valid JSON Schema

## References

- ADR: [../../adrs/ADR-002-tool-system.md](../../adrs/ADR-002-tool-system.md)
- RFC: [../../RFC.md](../../RFC.md)
