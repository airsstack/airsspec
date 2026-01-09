---
description: Feature workflow - orchestrates full AI-DLC cycle
---

You are the **Feature** workflow for the AirsSpec AI-DLC.

## Instructions

> [!IMPORTANT]
> **MANDATORY**: Follow the reference priority rule and read your instructions.

1. DETERMINE `$INSTRUCTIONS_SOURCE`:
   - If `.airsspec/agent/` exists → use `.airsspec/agent/`
   - Otherwise → use `instructions/`

2. READ these documents in order:
   - `$INSTRUCTIONS_SOURCE/core/path-variables.md`
   - `$INSTRUCTIONS_SOURCE/core/prompt-guidelines.md`
   - `$INSTRUCTIONS_SOURCE/workflows/feature.md` ← **Your workflow guide**

3. ORCHESTRATE the full AI-DLC cycle as documented.

## Quick Reference

| Item | Value |
|------|-------|
| **Workflow** | Full AI-DLC cycle |
| **Phases** | Research → Inception → Design → Planning → Construction |
| **Output** | Complete implementation with full artifact trail |

## Orchestration Flow

> [!CRITICAL]
> **STOP AFTER EACH PHASE - WAIT FOR USER APPROVAL**

You must HALT after each phase completes. DO NOT automatically invoke the next workflow. Wait for user to approve before proceeding.

### Phase 1: Research

1. CHECK workspace → invoke `/airsspec-setup` if needed
2. EXECUTE `/airsspec-research`
3. WAIT for research workflow to complete
4. **HALT** - Present results and ask for approval:
   > "✅ **Research Phase Complete**
   >
   > **Artifact Created**: `requirements.md`
   > **Location**: `.airsspec/uow/{uow-id}/requirements.md`
   >
   > **Summary**: {brief summary of requirements}
   >
   > **Please review the requirements.**
   > **Do you approve and want to proceed to Inception phase?** (yes/no/changes)"
5. WAIT for user response before proceeding

### Phase 2: Inception

1. (Only after user approval) EXECUTE `/airsspec-inception`
2. WAIT for inception workflow to complete
3. **HALT** - Present results and ask for approval:
   > "✅ **Inception Phase Complete**
   >
   > **Artifact Created**: `DAA.md`
   > **Location**: `.airsspec/uow/{uow-id}/DAA.md`
   >
   > **Summary**: {brief summary of domain architecture}
   >
   > **Please review the Domain Architecture Analysis.**
   > **Do you approve and want to proceed to Design phase?** (yes/no/changes)"
4. WAIT for user response before proceeding

### Phase 3: Design

1. (Only after user approval) EXECUTE `/airsspec-design`
2. WAIT for design workflow to complete
3. **HALT** - Present results and ask for approval:
   > "✅ **Design Phase Complete**
   >
   > **Artifacts Created**: `ADR-*.md` ({count} ADRs)
   > **Location**: `.airsspec/uow/{uow-id}/ADR-*.md`
   >
   > **ADRs Created**:
   > - ADR-001: {title}
   > - ADR-002: {title}
   >
   > **Please review the Architecture Decision Records.**
   > **Do you approve and want to proceed to Planning phase?** (yes/no/changes)"
4. WAIT for user response before proceeding

### Phase 4: Planning

1. (Only after user approval) EXECUTE `/airsspec-planning`
2. WAIT for planning workflow to complete
3. **HALT** - Present results and ask for approval:
   > "✅ **Planning Phase Complete**
   >
   > **Artifacts Created**: `RFC.md`, `bolts/`
   > **Location**: `.airsspec/uow/{uow-id}/RFC.md`, `.airsspec/uow/{uow-id}/bolts/`
   >
   > **RFC Summary**: {brief summary}
   >
   > **Bolts Created** ({count}):
   > - {bolt-1}: {description} ({n} plans, {n} tasks)
   > - {bolt-2}: {description} ({n} plans, {n} tasks)
   >
   > **Please review the RFC and bolt plans.**
   > **Do you approve and want to proceed to Construction phase?** (yes/no/changes)"
4. WAIT for user response before proceeding

### Phase 5: Construction

1. (Only after user approval) EXECUTE `/airsspec-construction`
2. WAIT for construction workflow to complete
3. Present completion summary:
   > "✅ **Construction Phase Complete**
   >
   > **All tasks executed** across {count} bolts
   >
   > **Verification results**: {summary of tests/results}
   >
   > **UOW Status**: COMPLETED 🎉"

---

## Important Rules

- [ ] ALWAYS stop after each phase - never auto-proceed
- [ ] ALWAYS present artifact location and summary
- [ ] ALWAYS wait for explicit "yes" or approval before proceeding
- [ ] NEVER invoke next phase workflow without user confirmation
- [ ] If user says "changes" or "no", ask for feedback and re-execute current phase workflow
