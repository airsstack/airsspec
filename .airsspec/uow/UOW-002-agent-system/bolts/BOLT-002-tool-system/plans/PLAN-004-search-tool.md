# PLAN-004: Search Tool

## Objective

Implement `SearchTool` for searching file contents using regex patterns.

## Context

Third core tool. Enables agents to find relevant content across multiple files. Uses regex for pattern matching.

## Steps

1. Create `src/tools/search.rs`
2. Implement `SearchTool` struct
3. Implement `Tool` trait:
   - `name()` returns "search"
   - `parameter_schema()` includes pattern and optional path
   - `execute()` searches files with regex
4. Add dependency on `regex` crate
5. Implement search logic:
   - Walk directory tree (respecting sandbox)
   - Apply regex to file contents
   - Return matches with line numbers and context
6. Limit results (e.g., max 100 matches) to prevent overwhelming output
7. Write unit tests:
   - Find matches
   - Multiple files
   - No matches found
   - Invalid regex pattern

## Expected Output

### Files Created:
- `crates/airsspec-tools/src/tools/search.rs`

### Files Modified:
- `crates/airsspec-tools/Cargo.toml` — Add regex dependency
- `crates/airsspec-tools/src/tools/mod.rs` — Export SearchTool
- `crates/airsspec-tools/src/lib.rs` — Export SearchTool

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] Regex search works correctly
- [ ] Results include line numbers
- [ ] Respects sandbox boundaries
- [ ] Limits output size

## References

- ADR: [../../adrs/ADR-002-tool-system.md](../../adrs/ADR-002-tool-system.md)
- RFC: [../../RFC.md](../../RFC.md)
