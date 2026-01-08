# Output Constraints

This instruction defines the output format and tool usage constraints for AI agents.

---

## JSON Output Protocol

All agent responses must follow the **Thought-Action** protocol:

### Response Schema

```json
{
  "thought": "Reasoning for why I am taking this action...",
  "action": {
    "tool_name": "<tool_id>",
    "params": {
      "<param1>": "<value1>",
      "<param2>": "<value2>"
    }
  }
}
```

### Response Types

| Type | Action Structure |
|------|------------------|
| **Tool Call** | `{ "tool_name": "...", "params": {...} }` |
| **Finish** | `{ "tool_name": "FINISH", "params": { "result": "..." } }` |
| **Delegate** | `{ "tool_name": "DELEGATE", "params": { "agent": "...", "query": "..." } }` |

### Example: Tool Call

```json
{
  "thought": "I need to check if the PRD exists before proceeding.",
  "action": {
    "tool_name": "read_file",
    "params": {
      "path": ".airsspec/uow/feature-payment/requirements.md"
    }
  }
}
```

### Example: Finish

```json
{
  "thought": "I have completed the PRD. The user should review it.",
  "action": {
    "tool_name": "FINISH",
    "params": {
      "result": "Created requirements.md with 3 requirements. Ready for review."
    }
  }
}
```

---

## Error Handling

### Invalid JSON Response
If the response is not valid JSON, the system injects an error prompt:

```
SYSTEM ERROR: Invalid JSON response.
Expected format: { "thought": "...", "action": { "tool_name": "...", "params": {...} } }
Please correct and retry.
```

### Missing Parameters
If required parameters are missing:

```
SYSTEM ERROR: Missing required field 'params.path' for tool 'write_file'.
Schema: { "path": string, "content": string }
Please correct and retry.
```

### Budget Deduction
- Each error consumes 1 retry token (separate from loop budget)
- After max retries, agent halts and reports failure

---

## Tool Invocation Rules

### Permission Checks
Before executing a tool:
1. Is the tool in `agent.allowed_tools`?
2. Is the target path within `agent.allowed_paths`?

### Sandboxing
- **Write operations**: Only to paths within `.airsspec/` and designated UOW directories
- **Command execution**: Only approved commands (no arbitrary shell access in non-Construction phases)

### Result Capture
All tool outputs are:
1. Captured as `OBSERVATION` in context
2. Appended to session JSONL (Frozen memory)

---

## Phase-Specific Tool Constraints

Reference: Each phase has specific allowed and blocked tools.

| Phase | Allowed | Blocked |
|-------|---------|---------|
| INGESTION | `read_file`, `list_dir` | `write_code`, `run_command` |
| RESEARCH | `search_web`, `read_file`, `write_file` (sources only) | `write_code`, `run_command` |
| INCEPTION | `read_file`, `write_file` (DAA only) | `write_code` |
| DESIGN | `read_file`, `read_code`, `write_file` (ADRs only) | `write_code` (src/*) |
| PLANNING | `read_file`, `read_code`, `write_file` (RFC, plans) | `write_code` (src/*) |
| CONSTRUCTION | **ALL TOOLS** | None (but must link to Task) |

See individual phase instructions in `phases/` for details.

---

## Artifact Format

### Frontmatter

All markdown artifacts should include YAML frontmatter:

```yaml
---
version: "1.0"
status: draft | review | approved
author: <agent-id>
created_at: <ISO-8601>
---
```

### Required Sections by Artifact

| Artifact | Required Sections |
|----------|-------------------|
| requirements.md | Problem Statement, Success Criteria, Scope |
| DAA.md | Domain Model, Bounded Contexts, Entities |
| ADR-*.md | Context, Decision, Consequences |
| RFC.md | Summary, Motivation, Design, Implementation Plan |
| PLAN-*.md | Objective, Steps, Verification |
| TASK-*.md | Plan Reference, Execution Output |

---

## Retry Configuration

From `airsspec.toml`:

```toml
[retry]
max_llm_retries = 3       # Retries for LLM failures
max_tool_retries = 2      # Retries for tool failures
backoff_base_ms = 500     # Initial backoff delay
backoff_max_ms = 5000     # Maximum backoff delay
```

### Retry Flow

```
Error → Check retry count → Under limit? → Exponential backoff → Retry
                               ↓
                          Over limit → Halt & Report
```

---

## Summary

| Constraint | Rule |
|------------|------|
| **Output Format** | JSON with `thought` + `action` |
| **Tool Access** | Phase-locked, path-sandboxed |
| **Error Handling** | Inject error context, retry with backoff |
| **Artifact Format** | YAML frontmatter + required sections |

---

**Next**: Choose a workflow:
- [Feature Workflow](../workflows/feature.md)
- [Hotfix Workflow](../workflows/hotfix.md)
