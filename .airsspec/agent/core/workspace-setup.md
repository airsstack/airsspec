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
├── agent/                    # Project-local instruction source ($PROJECT_AGENT_PATH)
│   ├── system/
│   │   └── default.md        # Custom system prompt additions
│   ├── core/                 # Core instructions (copied from instructions/core/)
│   │   ├── README.md
│   │   ├── path-variables.md
│   │   ├── prompt-guidelines.md
│   │   ├── philosophy.md
│   │   ├── constraints.md
│   │   ├── memory.md
│   │   ├── workspace-explore.md
│   │   └── workspace-setup.md
│   ├── phases/               # Phase instructions (copied from instructions/phases/)
│   │   ├── ingestion.md
│   │   ├── research.md
│   │   ├── inception.md
│   │   ├── design.md
│   │   ├── planning.md
│   │   └── construction.md
│   ├── workflows/            # Workflow instructions (copied from instructions/workflows/)
│   │   ├── feature.md
│   │   └── hotfix.md
│   └── README.md             # Instructions entry point
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

### Step 4: Copy Core Instructions

**IMPORTANT**: Copy all core instructions from `$INSTRUCTIONS_SOURCE` to `.airsspec/agent/` following the reference priority rule.

> **REASON**: According to the reference priority rule in `path-variables.md`, when `.airsspec/agent/` exists, it becomes the `$PROJECT_AGENT_PATH` and takes precedence over the upstream `$CORE_INSTRUCTIONS_PATH`. This enables per-project customization without affecting the upstream instructions.

Copy the instruction hierarchy:

```bash
# Copy core instructions
cp -r instructions/core .airsspec/agent/

# Copy phase instructions
cp -r instructions/phases .airsspec/agent/

# Copy workflow instructions
cp -r instructions/workflows .airsspec/agent/

# Copy main instruction entry point
cp instructions/README.md .airsspec/agent/
```

This creates the project-local instruction structure:
```
.airsspec/agent/
├── core/                     # Core instructions
├── phases/                   # Phase-specific guides
├── workflows/                # End-to-end workflows
├── system/                   # System overrides
│   └── default.md            # Custom system prompt
└── README.md                 # Instruction entry point
```

Now agents will reference `$PROJECT_AGENT_PATH` instead of upstream instructions, enabling customization at the project level.

### Step 5: Create System Override Directory

```bash
mkdir -p .airsspec/agent/system
```

Create placeholder for custom system prompt:

```markdown
# .airsspec/agent/system/default.md

# Custom System Instructions

Add project-specific instructions here. These will be appended to the base system prompt.

## Project-Specific Rules

(Define any project-specific constraints or preferences)
```

### Step 6: Create Plugin Directory

```bash
mkdir -p .airsspec/plugins
```

### Step 7: Create Knowledge Directories

```bash
mkdir -p .airsspec/sources
mkdir -p .airsspec/knowledge/library
mkdir -p .airsspec/knowledge/vectors
mkdir -p .airsspec/knowledge/playbooks
```

### Step 8: Create Context Directory

```bash
mkdir -p .airsspec/contexts/agent
```

### Step 9: Create UOW Directory

```bash
mkdir -p .airsspec/uow
```

---

## Verification Checklist

After setup, verify the structure:

### Core Structure
- [ ] `.airsspec/WORKSPACE.md` exists with project metadata
- [ ] `.airsspec/airsspec.toml` exists with configuration

### Instructions Copied
- [ ] `.airsspec/agent/README.md` exists (instruction entry point)
- [ ] `.airsspec/agent/system/default.md` exists
- [ ] `.airsspec/agent/core/` directory exists with core instructions
- [ ] `.airsspec/agent/phases/` directory exists with phase instructions
- [ ] `.airsspec/agent/workflows/` directory exists with workflow instructions

### Directories Created
- [ ] `.airsspec/sources/` directory exists
- [ ] `.airsspec/knowledge/library/` directory exists
- [ ] `.airsspec/knowledge/vectors/` directory exists
- [ ] `.airsspec/knowledge/playbooks/` directory exists
- [ ] `.airsspec/contexts/agent/` directory exists
- [ ] `.airsspec/uow/` directory exists
- [ ] `.airsspec/plugins/` directory exists

### Instruction Files Verified
Verify that all instruction files were copied correctly:
- [ ] `.airsspec/agent/core/README.md` exists
- [ ] `.airsspec/agent/core/path-variables.md` exists
- [ ] `.airsspec/agent/core/prompt-guidelines.md` exists
- [ ] `.airsspec/agent/core/philosophy.md` exists
- [ ] `.airsspec/agent/core/constraints.md` exists
- [ ] `.airsspec/agent/core/memory.md` exists
- [ ] `.airsspec/agent/core/workspace-explore.md` exists
- [ ] `.airsspec/agent/core/workspace-setup.md` exists
- [ ] `.airsspec/agent/phases/ingestion.md` exists
- [ ] `.airsspec/agent/phases/research.md` exists
- [ ] `.airsspec/agent/phases/inception.md` exists
- [ ] `.airsspec/agent/phases/design.md` exists
- [ ] `.airsspec/agent/phases/planning.md` exists
- [ ] `.airsspec/agent/phases/construction.md` exists
- [ ] `.airsspec/agent/workflows/feature.md` exists
- [ ] `.airsspec/agent/workflows/hotfix.md` exists

---

## For the AirsSpec Project

When setting up `.airsspec/` for the AirsSpec project itself:

1. **WORKSPACE.md** should reference:
   - Existing `docs/` for specifications
   - Existing `researches/` for research documents
   - Existing `instructions/` for AI instructions

2. **airsspec.toml** project name: `airsspec`
   - LLM provider: `anthropic` (since AirsSpec uses Anthropic agents)
   - Model: `claude-sonnet-4-20250514`

3. **Agent Instructions**: The `.airsspec/agent/` directory will be populated from the existing `instructions/` directory. Since AirsSpec is dogfooding itself, this creates a self-referential structure where the project uses its own instructions for development.

4. **Playbooks** can be populated from `docs/`:
   - Copy relevant patterns from `docs/architecture.md`
   - Extract reusable patterns from `docs/multi-agent-architecture.md`

---

**Next**: [memory.md](./memory.md) — Understand context and memory management
