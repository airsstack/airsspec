---
description: Initialize the .airsspec workspace structure
---

# Workspace Setup

This workflow initializes the `.airsspec/` directory for a new project.

## Prerequisites

- You have access to the project directory
- No existing `.airsspec/` directory (or you want to reset it)

## Steps

1. First, explore the project structure:
   // turbo
   Follow `instructions/core/workspace-explore.md` to scan the project and generate `WORKSPACE.md`.

2. Create the `.airsspec/` directory structure:
   // turbo
   Follow `instructions/core/workspace-setup.md` to bootstrap:
   - `airsspec.toml` configuration
   - `sources/` directory
   - `knowledge/` directory
   - `uow/` directory

3. Verify the structure exists:
   // turbo
   List contents of `.airsspec/` to confirm setup.

## Expected Output

```
.airsspec/
├── WORKSPACE.md
├── airsspec.toml
├── sources/
├── knowledge/
│   ├── library/
│   ├── playbooks/
│   └── vectors/
├── contexts/
└── uow/
```

## Next Steps

After setup is complete:
- Add source documents to `.airsspec/sources/`
- Add playbooks to `.airsspec/knowledge/playbooks/`
- Run `/airsspec-feature` or `/airsspec-hotfix` to start work
