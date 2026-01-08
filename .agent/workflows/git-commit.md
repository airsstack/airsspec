---
description: Commit current changes using Conventional Commits format
---

# Git Commit Workflow

This workflow commits current changes following the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

## Conventional Commits Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Commit Types

| Type | Description |
|------|-------------|
| `feat` | New feature (correlates with MINOR in SemVer) |
| `fix` | Bug fix (correlates with PATCH in SemVer) |
| `docs` | Documentation only changes |
| `style` | Code style changes (formatting, whitespace) |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `perf` | Performance improvement |
| `test` | Adding or updating tests |
| `build` | Changes to build system or dependencies |
| `ci` | Changes to CI configuration |
| `chore` | Other changes that don't modify src or test files |

## Breaking Changes

Use `!` after type/scope for breaking changes:
```
feat!: remove deprecated API endpoints
feat(api)!: change authentication flow
```

## Steps

### Step 1: Check Status

// turbo
1. Run `git status` to see what files have changed.

// turbo
2. Run `git diff --stat` to see a summary of changes.

### Step 2: Stage Changes

3. Stage all changes:
   ```bash
   git add .
   ```
   
   Or stage specific files if needed.

### Step 3: Analyze Changes

4. Review the staged changes to determine:
   - What type of change is this? (feat, fix, docs, etc.)
   - Is there a specific scope? (e.g., api, cli, core)
   - Is this a breaking change?

### Step 4: Create Commit Message

5. Create a commit message following the format:

   **Simple commit:**
   ```
   <type>: <description>
   ```
   
   **With scope:**
   ```
   <type>(<scope>): <description>
   ```
   
   **With body:**
   ```
   <type>(<scope>): <description>
   
   <body explaining what and why>
   ```

### Step 5: Commit

6. Run the commit command:
   ```bash
   git commit -m "<type>(<scope>): <description>"
   ```

### Step 6: Verify

// turbo
7. Run `git log -1` to verify the commit was created correctly.

## Examples

**Feature:**
```
feat(instructions): add OpenCode and AntiGravity agent integration
```

**Bug fix:**
```
fix(research): correct artifact naming from PRD.md to requirements.md
```

**Documentation:**
```
docs(readme): update installation instructions
```

**Refactor:**
```
refactor(phases): consolidate phase instruction structure
```

**Breaking change:**
```
feat(api)!: change authentication flow

BREAKING CHANGE: JWT tokens now use RS256 instead of HS256
```

## Tips

- Keep the description under 72 characters
- Use imperative mood ("add" not "added")
- Don't capitalize first letter of description
- No period at the end of description
- Body should explain *what* and *why*, not *how*
