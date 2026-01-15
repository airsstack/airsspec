# PLAN-003: WriteFile Tool

## Objective

Implement `WriteFileTool` for writing file contents with sandbox validation.

## Context

Second core tool. More restrictive than ReadFileTool — must validate write permissions and create parent directories.

## Steps

1. Create `src/tools/write_file.rs`
2. Implement `WriteFileTool` struct
3. Implement `Tool` trait:
   - `name()` returns "write_file"
   - `parameter_schema()` includes path and content
   - `execute()` writes file with sandbox validation
4. Add logic to create parent directories if missing
5. Enforce write-specific sandbox rules
6. Write unit tests:
   - Successful write
   - Create parent directories
   - Sandbox blocks writes to sensitive locations
   - Deny patterns prevent .env writes

## Expected Output

### Files Created:
- `crates/airsspec-tools/src/tools/write_file.rs`

### Files Modified:
- `crates/airsspec-tools/src/tools/mod.rs` — Export WriteFileTool
- `crates/airsspec-tools/src/lib.rs` — Export WriteFileTool

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] Can write to allowed locations
- [ ] Creates parent directories
- [ ] Blocks sensitive file writes
- [ ] Deny patterns work correctly

## References

- ADR: [../../adrs/ADR-002-tool-system.md](../../adrs/ADR-002-tool-system.md)
- RFC: [../../RFC.md](../../RFC.md)
