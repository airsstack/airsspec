---
description: Full AI-DLC workflow for implementing new features
---

# Feature Workflow

## Core Instructions

Follow `instructions/workflows/feature.md` for detailed guidance.

## Quick Reference

| Item | Value |
|------|-------|
| **Workflow** | Full AI-DLC cycle |
| **Template** | `templates/uow/status.yaml` |
| **Use Case** | New features, major enhancements |

## Phases

| Phase | Workflow | Output |
|-------|----------|--------|
| Research | `/airsspec-research` | `requirements.md` |
| Inception | `/airsspec-inception` | `DAA.md` |
| Design | `/airsspec-design` | `ADR-*.md` |
| Planning | `/airsspec-planning` | `RFC.md`, `bolts/` |
| Construction | `/airsspec-construction` | Source code |

## Steps

1. Check `.airsspec/` exists (run `/airsspec-setup` if not)

2. Create UOW container with `status.yaml`

3. Run phases in order, invoking each workflow:
   - `/airsspec-research` → wait for approval
   - `/airsspec-inception` → wait for approval
   - `/airsspec-design` → wait for approval
   - `/airsspec-planning` → wait for approval
   - `/airsspec-construction` → verify completion

4. Update UOW status to `COMPLETED`
