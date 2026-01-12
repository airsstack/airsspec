# Validation Gates

## Purpose

These validators MUST be run at specific checkpoints during task execution.
**Failing a validator = STOP execution and fix before proceeding.**

---

## When to Run

Run these validators:
- **BEFORE** marking any task complete
- **AFTER** implementing code changes
- **BEFORE** presenting results to user for approval

---

## Pre-Commit Validator (Rust)

### Check 1: mod.rs/lib.rs Compliance (§4.3)

Verify no type re-exports exist in module root files:

```bash
# Find any type re-exports in mod.rs or lib.rs
# This searches for "pub use ...::SomeType" patterns
find . -name "mod.rs" -o -name "lib.rs" 2>/dev/null | \
  xargs grep -E "pub use .+::[A-Z]" 2>/dev/null || echo "PASS: No type re-exports"
```

**Expected**: `"PASS: No type re-exports"`

**If output shows matches**: 
- ❌ You have violated §4.3
- STOP and remove the type re-exports
- mod.rs and lib.rs should contain ONLY `pub mod x;` declarations

---

### Check 2: Clippy Clean

Verify zero linter warnings:

```bash
cargo clippy --all-targets --all-features -- -D warnings 2>&1
```

**Expected**: Zero warnings (exit code 0)

**If warnings exist**: 
- ❌ FIX all warnings before proceeding
- Do NOT proceed with warnings present

---

### Check 3: Tests Pass

Verify all tests pass:

```bash
cargo test 2>&1
```

**Expected**: All tests pass (exit code 0)

**If failures exist**: 
- ❌ FIX failing tests before proceeding
- Do NOT proceed with test failures

---

### Check 4: Import Organization (§2.1) - Manual Review

Verify imports follow 3-layer organization in all modified files:

1. **Layer 1**: Standard library (`use std::...`)
2. **Layer 2**: External crates (`use serde::...`, `use tokio::...`)
3. **Layer 3**: Internal modules (`use crate::...`, `use super::...`)

**How to check**: Review each modified file and verify import order.

---

## Post-Validation Output

**After running all validators, output this summary:**

> **Validation Results**
>
> | Check | Result |
> |-------|--------|
> | §4.3 mod.rs/lib.rs compliance | {PASS/FAIL} |
> | Clippy (zero warnings) | {PASS/FAIL} |
> | Tests | {PASS/FAIL} |
> | §2.1 Import organization | {PASS/FAIL} |
>
> **All validators passed**: {YES/NO}
>
> **Ready for approval**: {YES/NO}

---

## Validation Gate Rules

> [!CAUTION]
> **BLOCKING**: If ANY validator fails:
>
> 1. **DO NOT** mark the task complete
> 2. **DO NOT** present to user for approval
> 3. **FIX** the issue first
> 4. **RE-RUN** the validator
> 5. **REPEAT** until all validators pass

---

## Quick Reference

| Validator | Command | Pass Condition |
|-----------|---------|----------------|
| §4.3 mod.rs | `find . -name "mod.rs" -o -name "lib.rs" \| xargs grep -E "pub use .+::[A-Z]"` | No output |
| Clippy | `cargo clippy --all-targets --all-features -- -D warnings` | Exit code 0 |
| Tests | `cargo test` | Exit code 0 |
| §2.1 Imports | Manual review | 3-layer order in all files |
