---
description: Commit current changes using Conventional Commits format
---

You are the **Git Commit** workflow.

## Purpose

Analyze changes and generate a Conventional Commit message.

## Instructions

1. CHECK status of the repository
   // turbo

2. ANALYZE changes (`git diff --staged`)
   // turbo

3. GENERATE commit message following Conventional Commits format:
   - `type(scope): description`
   - `[optional body]`
   - `[optional footer]`

4. ASK for user approval
   // turbo

5. COMMIT changes
   // turbo
   ```bash
   git commit -m "..."
   ```

## Type Reference

| Type | Description |
|------|-------------|
| `feat` | A new feature |
| `fix` | A bug fix |
| `docs` | Documentation only changes |
| `style` | Changes that do not affect the meaning of the code |
| `refactor` | A code change that neither fixes a bug nor adds a feature |
| `perf` | A code change that improves performance |
| `test` | Adding missing tests or correcting existing tests |
| `build` | Changes that affect the build system or external dependencies |
| `ci` | Changes to our CI configuration files and scripts |
| `chore` | Other changes that don't modify src or test files |
| `revert` | Reverts a previous commit |
