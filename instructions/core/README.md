# Core Instructions — Main Entrypoint

This is the starting point for AI agents working on the AirsSpec project.

## Getting Started

Follow these steps in order to initialize or resume work on a project:

### Step 1: Understand the Philosophy
**→ Read [philosophy.md](./philosophy.md)**

Learn the core principles:
- "Cognitive Cleanroom" — Phase-locked tool constraints
- "Filesystem as Truth" — State persisted to disk
- "Convention over Conversation" — Minimal prompting

### Step 2: Explore the Project
**→ Follow [workspace-explore.md](./workspace-explore.md)**

Scan the project directory to understand:
- Language and framework
- Existing structure
- Dependencies and patterns

**Output**: Generate `WORKSPACE.md` in `.airsspec/`

### Step 3: Setup Workspace Structure
**→ Follow [workspace-setup.md](./workspace-setup.md)**

Bootstrap the `.airsspec/` directory with:
- Configuration files
- Knowledge directories
- UOW containers

### Step 4: Understand Context Limits
**→ Read [memory.md](./memory.md)**

Learn about:
- Memory tiers (Hot/Warm/Cold/Frozen)
- Context window management
- When compression triggers

### Step 5: Review Output Constraints
**→ Read [constraints.md](./constraints.md)**

Understand:
- JSON output protocol
- Tool invocation format
- Error handling patterns

---

## Choose Your Workflow

Once setup is complete, choose a workflow based on your intent:

| Intent | Workflow | Description |
|--------|----------|-------------|
| New feature | [feature.md](../workflows/feature.md) | Full AI-DLC cycle with all phases |
| Bug fix / Refactor | [hotfix.md](../workflows/hotfix.md) | Fast track directly to Construction |

---

## Navigation Map

```
┌─────────────────────────────────────────────────────────────┐
│                    CORE INSTRUCTIONS                        │
├─────────────────────────────────────────────────────────────┤
│  philosophy.md ─────────────────────────────────────────────│
│       ↓                                                     │
│  workspace-explore.md → Generates WORKSPACE.md              │
│       ↓                                                     │
│  workspace-setup.md → Creates .airsspec/ structure          │
│       ↓                                                     │
│  memory.md ─────────────────────────────────────────────────│
│       ↓                                                     │
│  constraints.md ────────────────────────────────────────────│
└─────────────────────────────────────────────────────────────┘
       │
       ↓
┌─────────────────────────────────────────────────────────────┐
│                      WORKFLOWS                              │
├──────────────────────────┬──────────────────────────────────┤
│   feature.md             │   hotfix.md                      │
│   (Full Cycle)           │   (Fast Track)                   │
│                          │                                  │
│   ┌→ Ingestion           │   ┌→ Create UOW                  │
│   │→ Research            │   │→ Direct Bolt                 │
│   │→ Inception           │   │→ Construction                │
│   │→ Design              │   └→ Verification                │
│   │→ Planning            │                                  │
│   └→ Construction        │                                  │
└──────────────────────────┴──────────────────────────────────┘
```

---

## Prerequisites

Before starting, ensure:
- [ ] You have access to the project directory
- [ ] You can read/write files in the workspace
- [ ] You understand Markdown artifact format
