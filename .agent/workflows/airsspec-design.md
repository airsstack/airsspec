---
description: Design phase - creates ADR-*.md (Architecture Decision Records)
---

# Design Phase

## Core Instructions

Follow `instructions/phases/design.md` for detailed guidance.

## Quick Reference

| Item | Value |
|------|-------|
| **Input** | `DAA.md` (approved) |
| **Output** | `ADR-*.md` |
| **Template** | `templates/uow/ADR.md` |
| **Formula** | `ADR = DAA + (n × Playbooks)` |
| **Next** | `/airsspec-planning` |

## Steps

// turbo
1. Read `DAA.md`

// turbo
2. List available playbooks in `.airsspec/knowledge/playbooks/`

3. User selects playbooks to apply

4. Create ADR files using the template

5. Get user approval before proceeding

## Transition

- [ ] At least one ADR exists
- [ ] All ADRs are approved
