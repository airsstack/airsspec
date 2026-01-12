# TASK-001: JSONL Persistence

**Plan Reference**: [../plans/PLAN-001-jsonl-persistence.md](../plans/PLAN-001-jsonl-persistence.md)

## Status

- [x] Completed

## Execution Output

### Actions Taken

The `JsonlPersistence` implementation was already complete in `src/persistence.rs`. The following fixes were applied:

1. **Fixed string comparison issues** - Added dereference operators to fix compilation errors where `&str` was compared with `str`:
   - Line 96: `*line == FRONTMATTER_DELIMITER`
   - Line 101: `*line == FRONTMATTER_DELIMITER`
   - Line 135: `*line == FRONTMATTER_DELIMITER`
   - Line 143: `*line == FRONTMATTER_DELIMITER`

2. **Added missing imports in tests**:
   - Added `use serde::{Deserialize, Serialize};` to `test_parse_frontmatter_valid`
   - Added `use airsspec_core::error::AirsspecError;` to `test_airsspec_error_from_artifact_error`

3. **Fixed clippy warnings**:
   - Added `#[must_use]` attributes to `extract_frontmatter()` and `extract_body()` methods
   - Inlined format args (changed `{}` to `{e}`) in error messages
   - Added `#[allow(clippy::unwrap_used)]` to tests using `unwrap()`
   - Added `#[allow(clippy::default_constructed_unit_structs)]` to `test_jsonl_persistence_default`
   - Fixed needless borrow in `test_artifact_store_exists_false`
   - Removed duplicate test definitions that were accidentally created during editing

### Verification

- [x] `cargo build -p airsspec-artifacts` passes without errors
- [x] `cargo clippy -p airsspec-artifacts --all-targets --all-features -- -D warnings` passes
- [x] `cargo fmt --check --manifest-path crates/airsspec-artifacts/Cargo.toml` passes
- [x] `cargo test -p airsspec-artifacts` passes (17 tests passed)
- [x] Code follows project-standard.md patterns:
  - §2.1: 3-layer import organization followed
  - §2.2: No FQN in type annotations
  - §4.3: Module architecture (mod.rs contains only declarations)

### Implementation Details

The `JsonlPersistence` struct implements the `ArtifactStore` trait with:

1. **`read(path)`**: Reads file content from filesystem
2. **`write(path, content)`**: Writes content to file, creating parent directories if needed
3. **`exists(path)`**: Checks if file exists
4. **Frontmatter parsing helpers**:
   - `extract_frontmatter()`: Extracts YAML content between `---` markers
   - `extract_body()`: Extracts markdown body after frontmatter
   - `parse_frontmatter<T>()`: Deserializes YAML frontmatter into typed structures

### Files Modified

- `crates/airsspec-artifacts/src/persistence.rs`:
  - Fixed string comparison issues (4 locations)
  - Added `#[must_use]` attributes (2 locations)
  - Inlined format args (4 locations)
  - Added `#[allow]` attributes to tests (6 locations)
  - Removed duplicate test definitions (3 duplicates)

### Notes

The implementation was already complete with comprehensive test coverage. Only compilation and linting fixes were required. All 17 unit tests pass, including:
- Frontmatter extraction tests (valid, empty, no frontmatter, incomplete)
- Body extraction tests (with frontmatter, no frontmatter, empty)
- Frontmatter parsing tests (valid, empty, no frontmatter, invalid YAML)
- ArtifactStore trait tests (read, write, exists, error handling)
