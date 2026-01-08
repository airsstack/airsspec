---
version: "1.0"
status: draft
author: spec-writer
created_at: <ISO-8601>
---

# DAA: [Feature Name]

## Overview

[High-level description of the domain]

## Bounded Contexts

### Context: [Name]

**Purpose**: [What this context handles]

**Entities**:
- `EntityName`: [Description]

**Value Objects**:
- `ValueName`: [Description]

**Aggregates**:
- `AggregateName`: [Root entity + associated entities]

**Domain Events**:
- `EventName`: [When emitted, what it means]

## Context Map

```
[Context A] <-- Shared Kernel --> [Context B]
[Context C] <-- Customer/Supplier --> [Context D]
```

## Ubiquitous Language

| Term | Definition |
|------|------------|
| Term1 | Definition |
| Term2 | Definition |

## Domain Rules

1. Rule 1: [Business invariant]
2. Rule 2: [Constraint]

## References

- Requirements: [requirements.md](./requirements.md)
