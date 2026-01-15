# PLAN-001: Tool Registry and Sandbox

## Objective

Create `airsspec-tools` crate with tool registry and sandbox security enforcement.

## Context

From ADR-002, we need a registry-based tool system with path-based sandbox validation to ensure agents can only access authorized files.

## Steps

1. Create `crates/airsspec-tools/` directory structure
2. Add `Cargo.toml` with dependencies
3. Create `src/registry.rs`:
   - `ToolRegistry` struct with HashMap<String, Box<dyn Tool>>
   - `register()` method
   - `execute()` method with sandbox validation
   - `list_tools()` method
4. Create `src/sandbox.rs`:
   - `Sandbox` struct with allowed_roots and deny_patterns
   - `validate_read()` and `validate_write()` methods
   - Path canonicalization logic
   - Security policy enforcement
5. Create `src/error.rs` with error types:
   - `ToolNotFound`
   - `SecurityViolation`
   - `ExecutionError`
6. Add default sandbox policy:
   - Allowed roots: `.airsspec/`
   - Deny patterns: `.env*`, `*.key`, `*.pem`, `.git/*`

## Expected Output

### Files Created:
- `crates/airsspec-tools/Cargo.toml`
- `crates/airsspec-tools/src/lib.rs`
- `crates/airsspec-tools/src/registry.rs`
- `crates/airsspec-tools/src/sandbox.rs`
- `crates/airsspec-tools/src/error.rs`

### Files Modified:
- `Cargo.toml` (workspace) — Add airsspec-tools member

## Verification

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes
- [ ] Registry can store and retrieve tools
- [ ] Sandbox validates paths correctly
- [ ] Security violations are caught

## References

- ADR: [../../adrs/ADR-002-tool-system.md](../../adrs/ADR-002-tool-system.md)
- RFC: [../../RFC.md](../../RFC.md)
