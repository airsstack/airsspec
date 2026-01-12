# Path Variables Reference

## Purpose

Single source of truth for all path variables used throughout AirsSpec instructions and guidelines.

> [!IMPORTANT]
> **MANDATORY REFERENCE**: All instructions, guidelines, agents, and workflows MUST reference this document before using any file paths.

---

## Variable Definitions

### Workspace Variables

| Variable | Definition | Description |
|----------|------------|-------------|
| `$WORKSPACE_ROOT` | Current working directory | Project root where `.airsspec/` exists or will be created |
| `$AIRSSPEC_PATH` | `$WORKSPACE_ROOT/.airsspec` | AirsSpec workspace directory |

---

### Agent & Instructions Variables

| Variable | Definition | Description |
|----------|------------|-------------|
| `$CORE_INSTRUCTIONS_PATH` | `$WORKSPACE_ROOT/instructions` | Upstream instruction source (default/fallback) |
| `$PROJECT_AGENT_PATH` | `$AIRSSPEC_PATH/agent` | Project-local instructions (takes priority when exists) |
| `$INSTRUCTIONS_SOURCE` | Resolved path | Either `$PROJECT_AGENT_PATH` or `$CORE_INSTRUCTIONS_PATH` |

---

### Content Variables

| Variable | Definition | Description |
|----------|------------|-------------|
| `$SOURCES_PATH` | `$AIRSSPEC_PATH/sources` | Raw knowledge sources (PDFs, docs, external refs) |
| `$KNOWLEDGE_PATH` | `$AIRSSPEC_PATH/knowledge` | Synthesized knowledge container |
| `$LIBRARY_PATH` | `$KNOWLEDGE_PATH/library` | Warm memory — synthesized summaries |
| `$VECTORS_PATH` | `$KNOWLEDGE_PATH/vectors` | Cold memory — vector embeddings |
| `$PLAYBOOKS_PATH` | `$KNOWLEDGE_PATH/playbooks` | Reusable architecture patterns |
| `$GUIDELINES_PATH` | `$WORKSPACE_ROOT/.aiassisted/guidelines` | Project-specific coding guidelines |
| `$TEMPLATES_PATH` | `$WORKSPACE_ROOT/templates` | Artifact templates (DAA, ADR, RFC, etc.) |
| `$RESEARCHES_PATH` | `$WORKSPACE_ROOT/researches` | Research documents (DDD principles, integrations, etc.) |

---

### Work Variables

| Variable | Definition | Description |
|----------|------------|-------------|
| `$UOW_PATH` | `$AIRSSPEC_PATH/uow` | Units of Work container |
| `$CONTEXTS_PATH` | `$AIRSSPEC_PATH/contexts` | Session contexts container |
| `$SESSION_PATH` | `$CONTEXTS_PATH/agent` | Agent session logs (JSONL files) |

---

### Configuration Variables

| Variable | Definition | Description |
|----------|------------|-------------|
| `$CONFIG_FILE` | `$AIRSSPEC_PATH/airsspec.toml` | Main configuration file |
| `$WORKSPACE_FILE` | `$AIRSSPEC_PATH/WORKSPACE.md` | Project metadata file |

---

## Reference Priority Rule

When resolving instruction paths, apply this priority:

```
IF $PROJECT_AGENT_PATH exists:
    $INSTRUCTIONS_SOURCE = $PROJECT_AGENT_PATH
ELSE:
    $INSTRUCTIONS_SOURCE = $CORE_INSTRUCTIONS_PATH
```

### Why This Matters

- **Before setup**: Agents use upstream `$CORE_INSTRUCTIONS_PATH`
- **After setup**: Instructions are copied to `$PROJECT_AGENT_PATH`, enabling per-project customization
- **Customization**: Users can modify files in `$PROJECT_AGENT_PATH` without affecting upstream

---

## Variable Resolution Order

At runtime, resolve variables in this sequence:

1. `$WORKSPACE_ROOT` = Result of `pwd` (current working directory)
2. `$AIRSSPEC_PATH` = `$WORKSPACE_ROOT/.airsspec`
3. Check if `$PROJECT_AGENT_PATH` exists
4. Set `$INSTRUCTIONS_SOURCE` based on existence check
5. All other variables derive from `$WORKSPACE_ROOT` or `$AIRSSPEC_PATH`

---

## Usage Examples

### Correct Usage

```markdown
## Steps

1. READ philosophy from `$INSTRUCTIONS_SOURCE/core/philosophy.md`
2. CREATE requirements at `$UOW_PATH/{uow-id}/requirements.md`
3. WRITE session log to `$SESSION_PATH/{session-id}.jsonl`
```

### Incorrect Usage

```markdown
## Steps

1. Read `instructions/core/philosophy.md`
2. Create `.airsspec/uow/{id}/requirements.md`
```

> [!WARNING]
> **Problems with hardcoded paths:**
> - Hardcoded paths don't respect reference priority
> - No variable substitution = fragile references
> - Inconsistent with project-local customization

---

## Path Existence Checks

Before using a path, verify existence when required:

| Path | Check Required | Action if Missing |
|------|----------------|-------------------|
| `$AIRSSPEC_PATH` | Yes (for setup) | Run workspace setup |
| `$PROJECT_AGENT_PATH` | Yes (for reference priority) | Fallback to `$CORE_INSTRUCTIONS_PATH` |
| `$UOW_PATH/{id}` | Yes (for phase work) | Create UOW container first |
| `$SOURCES_PATH` | No | Directory always exists after setup |

---

## Summary

| Category | Variables |
|----------|-----------|
| **Workspace** | `$WORKSPACE_ROOT`, `$AIRSSPEC_PATH` |
| **Instructions** | `$CORE_INSTRUCTIONS_PATH`, `$PROJECT_AGENT_PATH`, `$INSTRUCTIONS_SOURCE` |
| **Content** | `$SOURCES_PATH`, `$KNOWLEDGE_PATH`, `$LIBRARY_PATH`, `$VECTORS_PATH`, `$PLAYBOOKS_PATH`, `$GUIDELINES_PATH` |
| **Work** | `$UOW_PATH`, `$CONTEXTS_PATH`, `$SESSION_PATH` |
| **Config** | `$CONFIG_FILE`, `$WORKSPACE_FILE` |

---

**Next**: Read [prompt-guidelines.md](./prompt-guidelines.md) — Mandatory prompt engineering guidelines
