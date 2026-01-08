---
version: "1.0"
status: draft
author: manager
created_at: <ISO-8601>
---

# RFC: [Feature Name] Implementation

## Summary

[One paragraph summary of the implementation approach]

## Motivation

[Why are we building this? Link to PRD.]

## Design Overview

[High-level technical approach. Link to ADRs.]

## Implementation Plan

### Bolt: database

- PLAN-001: Create schema migrations
- PLAN-002: Add seed data

### Bolt: api

- PLAN-001: Define route handlers
- PLAN-002: Implement validation

### Bolt: domain

- PLAN-001: Implement core entities
- PLAN-002: Implement business logic

## Dependencies

[What must be done first? What blocks what?]

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Risk 1 | Medium | High | Mitigation strategy |
| Risk 2 | Low | Medium | Mitigation strategy |

## Timeline

| Bolt | Estimated Duration |
|------|-------------------|
| database | 1 day |
| api | 2 days |
| domain | 1 day |

## References

- Requirements: [requirements.md](./requirements.md)
- DAA: [DAA.md](./DAA.md)
- ADRs: [ADR-001.md](./ADR-001.md), [ADR-002.md](./ADR-002.md)
