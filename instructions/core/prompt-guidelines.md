# Prompt Engineering Guidelines

<purpose>
Mandatory prompt engineering guidelines for all AirsSpec instructions, agents, and workflows.
</purpose>

> [!IMPORTANT]
> **MANDATORY REFERENCE**: All instructions MUST follow these guidelines. This document synthesizes best practices from Claude 4 documentation, Anthropic's prompt engineering guides, and Claude Code best practices.

---

## Sources

This document synthesizes knowledge from:
- [Claude 4 Best Practices](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-4-best-practices) — Anthropic Platform Docs
- [Prompt Engineering Best Practices](https://claude.com/blog/best-practices-for-prompt-engineering) — Claude Blog
- [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices) — Anthropic Engineering

---

## Core Principles

### 1. Be Explicit

<rule>
State exactly what to do. Do not assume the agent will infer intent.
Use direct action verbs: READ, WRITE, CREATE, VERIFY, HALT, ASK.
</rule>

<good>
READ `$INSTRUCTIONS_SOURCE/core/philosophy.md` before proceeding.
CREATE `requirements.md` using the template at `$INSTRUCTIONS_SOURCE/templates/requirements.md`.
HALT and ASK if requirements are ambiguous.
</good>

<bad>
You should read the philosophy document.
The requirements document should be created.
Consider reviewing the philosophy.
</bad>

**Why**: Claude 4.x models respond exceptionally well to clear, explicit instructions. Vague prompts lead to vague outputs.

---

### 2. Provide Context and Motivation

<rule>
Explain WHY a rule exists. Context helps the agent make better decisions in edge cases.
</rule>

<good>
Do not modify source code during the Research phase.

REASON: The Research phase produces requirements only. Code changes would bypass
the architectural decision process (DAA → ADR → RFC) and create untraced
implementations that violate the "Trust But Verify" principle.
</good>

<bad>
Do not modify source code during the Research phase.
</bad>

**Why**: Explaining motivation helps Claude understand your underlying goals and make better judgment calls.

---

### 3. Use Structured Sections

<rule>
Organize instructions with clear XML-style sections.
This creates unambiguous boundaries between concerns.
</rule>

**Required sections for ALL instructions:**

| Section | Purpose | Required |
|---------|---------|----------|
| `<purpose>` | Single sentence: what this instruction achieves | ✅ Yes |
| `<references>` | Mandatory reference documents to read first | ✅ Yes |
| `<prerequisites>` | What MUST exist before starting | ✅ Yes |
| `<actions>` | Explicit numbered steps to perform | ✅ Yes |
| `<output>` | Expected artifacts with validation criteria | ✅ Yes |
| `<next>` | Where to proceed after completion | ✅ Yes |

**Optional sections (use when applicable):**

| Section | Purpose |
|---------|---------|
| `<tools>` | Allowed and blocked tools with reasons |
| `<when_uncertain>` | What to do when the situation is unclear |
| `<examples>` | Concrete examples of expected behavior |
| `<never>` | Explicit list of prohibited actions |
| `<tips>` | Helpful hints for better execution |

---

### 4. Permission for Uncertainty

<rule>
Explicitly ALLOW agents to ask questions rather than assume.
This reduces hallucination and improves accuracy.
</rule>

<good>
<when_uncertain>
If user requirements are ambiguous, incomplete, or contradictory:

1. HALT execution immediately
2. LIST the specific questions that need clarification
3. WAIT for user response before proceeding

Do not guess. Do not assume. Do not infer missing requirements.
Ask explicitly.
</when_uncertain>
</good>

**Why**: Claude models are trained to be helpful, which can lead to confident-sounding incorrect answers. Giving explicit permission to express uncertainty reduces hallucination.

---

### 5. Explicit Tool Constraints

<rule>
For each phase, explicitly list:
- ALLOWED tools with specific use cases
- BLOCKED tools with clear reasons why
</rule>

<good>
<tools>
<allowed>
| Tool | Use Case |
|------|----------|
| `read_file` | Read sources, existing documentation, artifacts |
| `write_file` | Write to `$UOW_PATH/{id}/requirements.md` ONLY |
| `search_web` | External research when internal sources are insufficient |
</allowed>

<blocked>
| Tool | Reason |
|------|--------|
| `write_code` | Not in Research phase. Code changes require ADR approval first. |
| `run_command` | Command execution happens only in Construction phase. |
| `edit_file` | Editing existing code bypasses the specification process. |
</blocked>
</tools>
</good>

**Why**: Phase-locked tool constraints (the "Cognitive Cleanroom" principle) prevent context pollution and ensure proper artifact flow.

---

### 6. Verification Checklists

<rule>
Every instruction MUST have explicit success criteria.
Use checkboxes that can be verified programmatically or by the agent.
</rule>

<good>
<output>
<required>
| Artifact | Path |
|----------|------|
| Requirements document | `$UOW_PATH/{uow-id}/requirements.md` |
</required>

<validation>
- [ ] File exists at the specified path
- [ ] Contains YAML frontmatter with: version, status, author, created_at
- [ ] Contains "Problem Statement" section (non-empty)
- [ ] Contains "Success Criteria" section with at least 3 items
- [ ] Contains "Scope" section with both "In Scope" and "Out of Scope"
- [ ] All referenced sources use relative paths and exist
- [ ] No open questions marked as "blocking"
</validation>
</output>
</good>

**Why**: Explicit validation criteria enable automated checking and prevent incomplete outputs.

---

### 7. Avoid Over-Engineering

<rule>
Instructions should request the minimum necessary work.
Do not encourage extra features, abstractions, or "improvements."
</rule>

<good>
CREATE only the artifacts required by this phase.
Do not add extra sections, features, or "improvements" beyond what is specified.
Do not refactor surrounding code when fixing a bug.
Do not add error handling for scenarios that cannot occur.
The right amount of complexity is the minimum needed for the current task.
</good>

<bad>
Create a comprehensive requirements document with all possible sections.
Feel free to add any additional context you think might be helpful.
</bad>

**Why**: Claude 4.x models (especially Opus) tend to over-engineer. Explicit minimalism constraints prevent scope creep.

---

### 8. State Persistence

<rule>
All state MUST be persisted to the filesystem.
Agents should reference files, not conversation memory.
</rule>

<good>
WRITE progress to `$UOW_PATH/{id}/status.yaml` after each phase transition.
READ status from disk on startup — do not rely on conversation history.
APPEND session logs to `$SESSION_PATH/{session-id}.jsonl` for audit trail.
</good>

<bad>
Remember that we decided to use PostgreSQL in our earlier discussion.
</bad>

**Why**: The "Filesystem as Truth" principle ensures crash recovery, auditability, and human oversight.

---

## Instruction Template

Apply this template to ALL instruction files:

```markdown
# [Title]

<purpose>
[Single sentence: what this instruction achieves]
</purpose>

<references>
MANDATORY: Read these documents before proceeding.
- [path-variables.md](./path-variables.md) — Path variable definitions
- [prompt-guidelines.md](./prompt-guidelines.md) — This document
</references>

<prerequisites>
Before starting, verify:
- [ ] [Condition 1 — be specific]
- [ ] [Condition 2 — be specific]
</prerequisites>

<actions>
1. [VERB] [explicit target with path variables]
2. [VERB] [explicit target with path variables]
...
</actions>

<tools>
<allowed>
| Tool | Use Case |
|------|----------|
| tool_name | when and how to use |
</allowed>

<blocked>
| Tool | Reason |
|------|--------|
| tool_name | why blocked in this phase |
</blocked>
</tools>

<when_uncertain>
If [specific condition], then [specific action].
Do not assume — ask.
</when_uncertain>

<output>
<required>
| Artifact | Path |
|----------|------|
| Name | `$VARIABLE/path/to/file` |
</required>

<validation>
- [ ] [Explicit check 1]
- [ ] [Explicit check 2]
</validation>
</output>

<next>
Proceed to: `$INSTRUCTIONS_SOURCE/[path/to/next/instruction.md]`
</next>
```

---

## Anti-Patterns

<never>
These patterns MUST be avoided in all instructions:

| Anti-Pattern | Problem | Instead |
|--------------|---------|---------|
| Passive voice | Unclear who acts | Use imperative: "CREATE", "READ" |
| "You should..." | Indirect, weak | Use direct: "CREATE the file" |
| "Consider..." | Optional feeling | Use explicit: "EVALUATE and DECIDE" |
| Hardcoded paths | Breaks customization | Use path variables |
| Missing validation | No success criteria | Add `<validation>` checklist |
| Assumed context | Relies on memory | Reference files explicitly |
| Vague scope | Encourages over-engineering | Be specific about boundaries |
</never>

---

## Writing Style Guide

### Action Verbs

Use these verbs consistently:

| Verb | Meaning |
|------|---------|
| READ | Load file content into context |
| WRITE | Create a new file |
| CREATE | Same as WRITE (for artifacts) |
| UPDATE | Modify existing file |
| VERIFY | Check that conditions are met |
| VALIDATE | Run the validation checklist |
| HALT | Stop execution |
| ASK | Request user clarification |
| PROCEED | Move to next step/phase |
| DELEGATE | Hand off to another agent |

### Formatting

- Use **bold** for emphasis on critical words
- Use `backticks` for file paths, tool names, variable names
- Use tables for structured comparisons
- Use checklists `- [ ]` for verification items
- Use XML tags for section boundaries

---

## Summary

| Principle | Key Rule |
|-----------|----------|
| Be Explicit | Direct action verbs, no passive voice |
| Provide Context | Explain WHY rules exist |
| Use Structure | XML sections with clear boundaries |
| Allow Uncertainty | Explicit permission to ask questions |
| Constrain Tools | Phase-locked allowed/blocked lists |
| Verify Output | Checklists for success criteria |
| Stay Minimal | Only what's required, no extras |
| Persist State | Filesystem is the source of truth |

---

**Previous**: [path-variables.md](./path-variables.md) — Path variable definitions
**Next**: [README.md](./README.md) — Core instructions entrypoint
