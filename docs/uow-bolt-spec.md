# Specification: Unit of Work (UOW) & Bolt Architecture

**Version**: 2.0
**Status**: Draft
**Context**: AI-Native Development Lifecycle (AI-DLC)

## 1. Overview

This specification defines the "Cognitive Cleanroom" architecture for the AirsStack AI-DLC. It establishes the **Unit of Work (UOW)** as the high-level container for objectives and the **Bolt** as the atomic execution module. This structure is designed to enforce "Fractional Cognition" for AI agents, ensuring high context relevance and minimal hallucination.

## 2. The Hierarchy

The architecture follows a strict 3-Layer Hierarchy:

1.  **UOW (Epic)**: The "What" and "Why". Purely specificational.
2.  **Bolt (Module)**: The "How" (Strategy). A logical grouping of plans.
3.  **Task (Atom)**: The "Action". A single execution step.

## 3. The Unit of Work (UOW)

- **Role**: Epic Container.
- **Philosophy**: A transaction that isolates a feature or change request from the rest of the system. It creates a boundary around the "Problem Space".
- **Rule**: A UOW **MUST NOT** contain implementation code, diffs, or patches directly. It acts solely as a container for requirements, state, and strategy modules.

### 3.1 Directory Structure

```text
.airsspec/uow/{UOW_ID}/
├── status.yaml        # Lifecycle State
├── requirements.md    # The Goal (PRD/User Story)
├── DAA.md             # Domain Architecture Analysis
├── ADR-*.md           # Architecture Decision Records
├── RFC.md             # Implementation Strategy
└── bolts/             # Execution Modules
    ├── {BOLT_ID}/
    └── ...
```

### 3.2 Artifacts

#### `requirements.md` (The Goal)
The source of truth for the UOW. It describes the business value, requirements, and success criteria.

```markdown
# Goal: Implement User Authentication

## Context
We need to add email/password login.

## Requirements
- [ ] User can sign up
- [ ] User can login
- [ ] Password must be hashed
```

#### `status.yaml` (The State)
A deterministic file tracking the lifecycle of the Epic.

```yaml
id: UOW-20241229-auth
status: IN_PROGRESS  # [DRAFT, PLANNED, IN_PROGRESS, BLOCKED, COMPLETED]
created_at: 2024-12-29T10:00:00Z
progress:
  total_bolts: 2
  completed_bolts: 0
```

## 4. The Bolt

- **Role**: Execution Module.
- **Philosophy**: Manages complexity by breaking the UOW into domain-specific contexts (e.g., Database, API, Frontend).
- **Rule**: A Bolt strictly owns the *Strategy* (`plans/`) and the *Execution* (`tasks/`). It enforces boundaries so an agent working on "Database" does not get distracted by "CSS".

### 4.1 Directory Structure

```text
bolts/{BOLT_ID}/
├── status.yaml        # Module State
├── plans/             # Strategy Definitions
│   ├── 01-schema.md
│   ├── 02-seeds.md
│   └── ...
└── tasks/             # Execution Logs
    ├── 01-schema.md   # Link to plans/01-schema.md
    ├── 02-seeds.md
    └── ...
```

### 4.2 Artifacts

#### `plans/` (The Strategy)
A directory of markdown files. Each file describes a specific, atomic set of steps to achieve a part of the module's goal.

#### `tasks/` (The Execution)
A directory of markdown files tracking the actual execution results of each plan.
*   **Rule**: **1 Task = 1 Plan**. There is a strict 1-to-1 mapping.

**Example Task File (`tasks/01-schema.md`)**:
```markdown
# Task: Create Users Table
**Plan Reference**: `../plans/01-schema.md`

## Execution Output
(Agent writes logs, notes, and verification results here during execution)
```

#### `status.yaml` (The Module State)

```yaml
id: BOLT-01-database
parent_uow: UOW-20241229-auth
status: PENDING
```

## 5. Workflow Transitions

1.  **Inception**: UOW created in `DRAFT`. `requirements.md` written.
2.  **Planning**: AI analyzes requirements, creates `bolts/` and `plans/`. UOW moves to `PLANNED`.
3.  **Execution**: Agents pick up Plans, create Task files, and execute work. Bolt status updates to `IN_PROGRESS`.
4.  **Completion**: All Tasks done -> Bolt status `COMPLETED`. All Bolts done -> UOW status `COMPLETED`.
