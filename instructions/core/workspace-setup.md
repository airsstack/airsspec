# Workspace Setup

This instruction guides you through bootstrapping the `.airsspec/` directory structure.

---

## Prerequisites

- [ ] Completed [workspace-explore.md](./workspace-explore.md)
- [ ] Generated `WORKSPACE.md` content (not yet written to disk)

---

## Directory Structure

Create the following structure:

```
.airsspec/
├── WORKSPACE.md              # Project metadata (from exploration)
├── airsspec.toml             # Configuration
│
├── agent/                    # User overrides for agent behavior
│   ├── system/
│   │   └── default.md        # Custom system prompt additions
│   └── core/
│       └── (agent overrides)
│
├── plugins/                  # Installed plugins
│   └── (plugin directories)
│
├── sources/                  # Raw knowledge sources
│   └── (PDFs, docs, external references)
│
├── knowledge/
│   ├── library/              # Synthesized documents (Warm memory)
│   ├── vectors/              # Lance vector index (Cold memory)
│   └── playbooks/            # Reusable architecture patterns
│
├── contexts/
│   └── agent/                # Session logs (Frozen memory)
│       └── {session-id}.jsonl
│
└── uow/                      # Units of Work
    └── (created per feature/task)
```

---

## Step-by-Step Setup

### Step 1: Create Root Directory

```bash
mkdir -p .airsspec
```

### Step 2: Write WORKSPACE.md

Write the `WORKSPACE.md` content generated during exploration:

```bash
# Path: .airsspec/WORKSPACE.md
```

### Step 3: Create Configuration File

Create `airsspec.toml` with default settings:

```toml
# .airsspec/airsspec.toml

[project]
name = "<project-name>"
version = "0.1.0"

[llm]
provider = "openai"  # or "anthropic", "ollama"
model = "gpt-4"

[retry]
max_llm_retries = 3
max_tool_retries = 2
backoff_base_ms = 500
backoff_max_ms = 5000

[memory]
hot_window_size = 30
compression_threshold = 0.8
```

### Step 4: Create Agent Override Directories

```bash
mkdir -p .airsspec/agent/system
mkdir -p .airsspec/agent/core
```

Create placeholder for custom system prompt:

```markdown
# .airsspec/agent/system/default.md

# Custom System Instructions

Add project-specific instructions here. These will be appended to the base system prompt.

## Project-Specific Rules

(Define any project-specific constraints or preferences)
```

### Step 5: Create Plugin Directory

```bash
mkdir -p .airsspec/plugins
```

### Step 6: Create Knowledge Directories

```bash
mkdir -p .airsspec/sources
mkdir -p .airsspec/knowledge/library
mkdir -p .airsspec/knowledge/vectors
mkdir -p .airsspec/knowledge/playbooks
```

### Step 7: Create Context Directory

```bash
mkdir -p .airsspec/contexts/agent
```

### Step 8: Create UOW Directory

```bash
mkdir -p .airsspec/uow
```

---

## Verification Checklist

After setup, verify the structure:

- [ ] `.airsspec/WORKSPACE.md` exists with project metadata
- [ ] `.airsspec/airsspec.toml` exists with configuration
- [ ] `.airsspec/agent/system/default.md` exists
- [ ] `.airsspec/sources/` directory exists
- [ ] `.airsspec/knowledge/library/` directory exists
- [ ] `.airsspec/knowledge/vectors/` directory exists
- [ ] `.airsspec/knowledge/playbooks/` directory exists
- [ ] `.airsspec/contexts/agent/` directory exists
- [ ] `.airsspec/uow/` directory exists

---

## For the AirsSpec Project

When setting up `.airsspec/` for the AirsSpec project itself:

1. **WORKSPACE.md** should reference:
   - Existing `docs/` for specifications
   - Existing `researches/` for research documents
   - Existing `instructions/` for AI instructions

2. **airsspec.toml** project name: `airsspec`

3. **Playbooks** can be populated from `docs/`:
   - Copy relevant patterns from `docs/architecture.md`
   - Extract reusable patterns from `docs/multi-agent-architecture.md`

---

**Next**: [memory.md](./memory.md) — Understand context and memory management
