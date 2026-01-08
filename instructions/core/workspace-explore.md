# Workspace Exploration

This instruction guides you through exploring a project directory to generate the `WORKSPACE.md` metadata file.

---

## Purpose

Before setting up `.airsspec/`, you must understand the project:
- What language/framework is used?
- What is the existing structure?
- What conventions are already in place?

The output is `WORKSPACE.md` — a structured summary of the project.

---

## Exploration Steps

### Step 1: Identify Project Root

The project root typically contains:
- VCS markers: `.git/`
- Build/config files: `Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`, etc.
- Documentation: `README.md`

**Action**: List the root directory and identify the project type.

### Step 2: Detect Language & Framework

Look for signature files:

| File | Language/Framework |
|------|-------------------|
| `Cargo.toml` | Rust |
| `package.json` | Node.js / JavaScript |
| `pyproject.toml`, `requirements.txt` | Python |
| `go.mod` | Go |
| `pom.xml`, `build.gradle` | Java |
| `Gemfile` | Ruby |
| `composer.json` | PHP |

**Action**: Read the primary config file to extract:
- Project name
- Version
- Dependencies
- Build targets

### Step 3: Map Directory Structure

Identify key directories:

| Directory | Purpose |
|-----------|---------|
| `src/`, `lib/` | Source code |
| `tests/`, `spec/` | Tests |
| `docs/` | Documentation |
| `examples/` | Usage examples |
| `benches/` | Benchmarks |
| `.github/`, `.gitlab/` | CI/CD configuration |

**Action**: List directories to depth 2-3 to understand the layout.

### Step 4: Read Existing Documentation

Priority order:
1. `README.md` — Project overview
2. `CONTRIBUTING.md` — Contribution guidelines
3. `docs/` directory — Extended documentation
4. `CHANGELOG.md` — Version history

**Action**: Read `README.md` to extract:
- Project description
- Installation instructions
- Usage patterns
- Architecture notes

### Step 5: Identify Entry Points

For executable projects:
- `src/main.rs`, `src/bin/` — Rust binaries
- `src/index.ts`, `src/main.ts` — TypeScript
- `app.py`, `main.py` — Python
- `cmd/` — Go commands

For libraries:
- `src/lib.rs` — Rust library
- `src/index.ts` — TypeScript module
- `__init__.py` — Python package

**Action**: Note the primary entry points.

### Step 6: Check for Existing Conventions

Look for:
- Code style: `.editorconfig`, `.prettierrc`, `rustfmt.toml`
- Linting: `.eslintrc`, `clippy.toml`
- Pre-commit: `.pre-commit-config.yaml`
- Existing `.airsspec/` directory (resume vs initialize)

---

## Generate WORKSPACE.md

After exploration, create `.airsspec/WORKSPACE.md` with this structure:

```markdown
# Workspace: [Project Name]

## Overview
[Brief description from README]

## Project Type
- **Language**: [e.g., Rust]
- **Framework**: [e.g., Tokio/Axum]
- **Build Tool**: [e.g., Cargo]

## Structure
```
[Directory tree to depth 2]
```

## Entry Points
- **Main Binary**: [path]
- **Library**: [path]

## Key Dependencies
- [dep1]: [purpose]
- [dep2]: [purpose]

## Conventions
- **Code Style**: [tool]
- **Testing**: [framework]
- **CI/CD**: [platform]

## Notes
[Any special observations about the project]
```

---

## Example: Exploring the AirsSpec Project

For this project (`/Users/hiraq/Projects/airsstack/airsspec`):

1. **Root files**: `Cargo.toml` (if exists), `README.md`, `AIRSDLC.md`
2. **Language**: Rust (or spec-only if no Cargo.toml yet)
3. **Structure**:
   ```
   airsspec/
   ├── docs/          # Specifications
   ├── researches/    # Research documents
   └── instructions/  # AI instructions (this directory)
   ```
4. **Documentation**: Heavy spec documentation in `docs/`

---

**Next**: [workspace-setup.md](./workspace-setup.md) — Bootstrap the `.airsspec/` directory
