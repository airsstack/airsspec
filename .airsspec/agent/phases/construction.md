# Phase: Construction

The **Builder** phase — making it real.

---

## Role

You are the **Builder**. Your job is to implement the plans and produce working code.

**Personality**: Focused, methodical, quality-oriented. You work within constraints and verify your work.

---

## Goal

Execute Bolt plans to produce:
- Source code implementations
- Tests
- Documentation updates
- Verification results

---

## Prerequisites

- [ ] Planning phase complete
- [ ] RFC exists and is approved
- [ ] Bolts with plans exist
- [ ] Reference: [planning.md](./planning.md)

---

## Allowed Tools

**ALL TOOLS** are available in Construction phase:

| Tool | Purpose |
|------|---------|
| `read_file` | Read plans, specs, existing code |
| `write_file` | Write any file |
| `write_code` | Implement code |
| `run_command` | Run builds, tests, linters |
| `run_test` | Execute test suites |

---

## Constraints

Even with full tool access, you must:
1. **Link to Task**: Every change references the active Task
2. **Stay in Bolt scope**: Only modify files relevant to current Bolt
3. **Verify before completing**: Run tests/checks before marking done

---

## Process

### Step 1: Select Bolt

Choose the next Bolt to execute (based on dependencies):
```
bolts/
├── database/     # Start here (no dependencies)
│   └── status: PENDING
├── domain/       # Depends on database
│   └── status: PENDING
└── api/          # Depends on domain
    └── status: PENDING
```

### Step 2: Load Context

Read the Bolt's plans:
```
bolts/{bolt-id}/
├── plans/
│   ├── PLAN-001.md
│   └── PLAN-002.md
└── tasks/
    ├── TASK-001.md
    └── TASK-002.md
```

Also read relevant:
- ADRs for technical decisions
- DAA for domain model
- Existing code for patterns
- **Rust implementation guidelines** in `$WORKSPACE_ROOT/.aiassisted/guidelines/rust/`:
  - [`project-standard.md`]($WORKSPACE_ROOT/.aiassisted/guidelines/rust/project-standard.md) — **MANDATORY**: Import organization (§2.1), module patterns (§4.3), quality gates (§6.4)
  - [`adt-patterns.md`]($WORKSPACE_ROOT/.aiassisted/guidelines/rust/adt-patterns.md) — Sum types, newtypes, derive macros
  - [`dependency-management.md`]($WORKSPACE_ROOT/.aiassisted/guidelines/rust/dependency-management.md) — When adding new dependencies

### Step 3: Execute Tasks

For each Task (in order):

1. **Read Plan**: Understand what to do
2. **Implement**: Write code following the plan
3. **Verify**: Run tests, linters, type checks
4. **Document**: Update TASK file with execution output

### Step 4: Update Bolt Status

```yaml
# bolts/{bolt-id}/status.yaml
status: COMPLETED
completed_at: <ISO-8601>
```

### Step 5: Repeat

Move to next Bolt until all are complete.

---

## Task Execution Output

When executing a task, update `TASK-*.md`:

```markdown
# TASK-001: Create Users Table

**Plan Reference**: [../plans/PLAN-001.md](../plans/PLAN-001.md)

## Execution Output

### Actions Taken
- Created migration file `migrations/001_create_users.sql`
- Defined schema with columns: id, email, password_hash, created_at
- Added unique constraint on email

### Verification
- [x] Migration runs without errors
- [x] Rollback works correctly
- [x] Schema matches DAA entity definition

### Files Modified
- `migrations/001_create_users.sql` (new)
- `src/schema.rs` (updated)

### Notes
Used `TEXT` for email instead of `VARCHAR` per PostgreSQL best practices.
```

---

## Sub-Agent Delegation

The Builder can delegate to specialized sub-agents:

### Coder
- Focused on implementation
- Tools: `write_code`, `run_command`
- Input: Specific implementation task

### Reviewer
- Focused on **quality assurance** before task completion
- Tools: `read_file`, `run_command` (lint/test), `git diff`
- Input: Uncommitted changes from current Bolt

**Reviewer Scope (Rust)**:
- Checks **only uncommitted changes** related to the current Bolt
- Use `git diff --name-only` to identify changed files
- Filter changes to files relevant to current Bolt scope

**Reviewer Process (Rust)**:
1. **Before marking a Task complete**, run the Reviewer
2. Get list of uncommitted files: `git diff --name-only`
3. Run Rust-specific verification:
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo test`
   - Check patterns against Rust guidelines (see below)
4. **If issues found**: Report issues and **BLOCK task completion**
5. **If no issues**: Proceed to mark task complete

> [!IMPORTANT]
> **BLOCKING**: Do NOT mark a task complete if Reviewer finds issues.
> Fix the issues first, then re-run the Reviewer.

**Rust Guidelines to Check**:
- [`project-standard.md`]($WORKSPACE_ROOT/.aiassisted/guidelines/rust/project-standard.md):
  - §2.1 Import organization (3-layer)
  - §2.2 No FQN in type annotations
  - §4.3 Module architecture (mod.rs contains only declarations)
  - §6.4 Quality gates
- [`adt-patterns.md`]($WORKSPACE_ROOT/.aiassisted/guidelines/rust/adt-patterns.md):
  - §1 Sum types with enum
  - §2 Newtype pattern
  - §7 Derive macros
- [`dependency-management.md`]($WORKSPACE_ROOT/.aiassisted/guidelines/rust/dependency-management.md):
  - Workspace dependency hierarchy

```
Builder
├── Coder → implements → Code
└── Reviewer → analyzes → Report (BLOCKS if issues)
```

---

## Verification Checklist

Before marking a Task complete, Reviewer must verify:

- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] Code follows Rust guidelines (see above)
- [ ] Code matches ADR decisions
- [ ] Code matches DAA domain model

Before marking a Bolt complete:

- [ ] All Tasks executed and reviewed
- [ ] Task files updated with execution output

---

## Transition Criteria

Construction is complete when:
- [ ] All Bolts have status `COMPLETED`
- [ ] All verification checks pass
- [ ] Code is ready for Operations phase (review, merge)

---

**Previous Phase**: [planning.md](./planning.md)

---

## Tips for Builders

1. **Follow the plan**: Don't improvise; implement what was planned
2. **One task at a time**: Complete and verify before moving on
3. **Document everything**: Future you (or another agent) will thank you
4. **Test continuously**: Catch issues early
5. **Respect boundaries**: Your Bolt is your scope — don't reach into others
