# Workflow: Feature (Full AI-DLC Cycle)

A step-by-step guide for implementing new features using the complete AI-DLC lifecycle.

---

## When to Use

Use this workflow when:
- Building a new feature from scratch
- Implementing a significant enhancement
- Working on anything that needs architectural decisions

**Not for**: Quick fixes, bug patches, or refactors → Use [hotfix.md](./hotfix.md)

---

## Prerequisites

Before starting, ensure:
- [ ] Project has been explored ([workspace-explore.md](../core/workspace-explore.md))
- [ ] `.airsspec/` directory is set up ([workspace-setup.md](../core/workspace-setup.md))
- [ ] You understand the memory model ([memory.md](../core/memory.md))
- [ ] You understand output constraints ([constraints.md](../core/constraints.md))

---

## Step 1: Initialize Workspace (if needed)

If `.airsspec/` doesn't exist:

1. Follow [workspace-explore.md](../core/workspace-explore.md) to scan the project
2. Follow [workspace-setup.md](../core/workspace-setup.md) to create the structure

**Verification**:
- [ ] `.airsspec/WORKSPACE.md` exists
- [ ] `.airsspec/airsspec.toml` exists
- [ ] Directory structure is complete

---

## Step 2: Create Unit of Work (UOW)

Create a container for this feature:

### 2.1 Choose UOW ID
Format: `{type}-{short-name}`
Examples: `feature-payment`, `feature-auth`, `feature-notifications`

### 2.2 Create UOW Directory

```
.airsspec/uow/{uow-id}/
├── status.yaml
└── (artifacts will be added)
```

### 2.3 Initialize Status

Use template from `templates/uow/status.yaml`:

```yaml
id: {uow-id}
status: DRAFT
created_at: <ISO-8601>
progress:
  total_bolts: 0
  completed_bolts: 0
```

**Verification**:
- [ ] UOW directory exists
- [ ] `status.yaml` is initialized

---

## Step 3: Ingestion

**Reference**: [phases/ingestion.md](../phases/ingestion.md)

### Actions
1. Review `.airsspec/sources/` for available knowledge
2. Add any new relevant documents (PDFs, specs, references)
3. Review `.airsspec/knowledge/playbooks/` for available patterns

### Output
- Sources cataloged
- User selects which sources to mount for this UOW

---

## Step 4: Research → Requirements

**Reference**: [phases/research.md](../phases/research.md)

### Actions
1. Gather context from mounted sources
2. Clarify user intent (ask questions if vague)
3. Research external information if needed
4. Draft `requirements.md`

### Output
```
.airsspec/uow/{uow-id}/requirements.md
```

### Gate
- [ ] `requirements.md` exists
- [ ] User approves requirements
- [ ] Update `status.yaml`: `status: PLANNED`

---

## Step 5: Inception → DAA

**Reference**: [phases/inception.md](../phases/inception.md)

### Actions
1. Analyze requirements for domain concepts
2. Define entities, value objects, aggregates
3. Map bounded contexts
4. Establish ubiquitous language
5. Draft `DAA.md`

### Output
```
.airsspec/uow/{uow-id}/DAA.md
```

### Gate
- [ ] `DAA.md` exists
- [ ] User approves DAA
- [ ] Update `status.yaml`: `status: PLANNED`

---

## Step 6: Design → ADRs

**Reference**: [phases/design.md](../phases/design.md)

### Actions
1. Review DAA and available playbooks
2. Present playbook options to user
3. Apply selected playbooks to domain model
4. Create ADRs for each major decision

### Output
```
.airsspec/uow/{uow-id}/ADR-001-{topic}.md
.airsspec/uow/{uow-id}/ADR-002-{topic}.md
...
```

### Gate
- [ ] At least one ADR exists
- [ ] All ADRs approved
- [ ] Update `status.yaml`: `status: PLANNED`

---

## Step 7: Planning → RFC + Bolts

**Reference**: [phases/planning.md](../phases/planning.md)

### Actions
1. Synthesize DAA + ADRs into RFC
2. Decompose work into Bolts
3. For each Bolt, create plans
4. For each plan, create corresponding task

### Output
```
.airsspec/uow/{uow-id}/RFC.md
.airsspec/uow/{uow-id}/bolts/
├── {bolt-1}/
│   ├── status.yaml
│   ├── plans/
│   │   └── PLAN-001.md
│   └── tasks/
│       └── TASK-001.md
├── {bolt-2}/
│   └── ...
```

### Gate
- [ ] `RFC.md` exists and is approved
- [ ] All Bolts have plans and tasks
- [ ] Update `status.yaml`: `status: IN_PROGRESS`

---

## Step 8: Construction → Code

**Reference**: [phases/construction.md](../phases/construction.md)

### Actions
For each Bolt (in dependency order):
1. Update Bolt status to `IN_PROGRESS`
2. For each Task in the Bolt:
   - Read the corresponding Plan
   - Implement the code
   - Verify (run tests, lints)
   - Update Task with execution output
3. Update Bolt status to `COMPLETED`

### Output
- Source code in `src/`
- Tests in `tests/`
- Updated Task files with execution logs

### Loop
```
foreach Bolt in bolts/ {
    foreach Task in Bolt.tasks/ {
        Execute(Task)
        Verify(Task)
        Document(Task)
    }
    Mark Bolt COMPLETED
}
```

---

## Step 9: Completion

### Actions
1. Verify all Bolts are `COMPLETED`
2. Run full test suite
3. Update UOW state to `COMPLETED`
4. Archive knowledge (update `.airsspec/knowledge/library/`)

### Final State

```json
{
  "id": "{uow-id}",
  "phase": "COMPLETED",
  "completed_at": "<ISO-8601>",
  ...
}
```

---

## Summary Checklist

| Step | Phase | Artifact | Gate |
|------|-------|----------|------|
| 1 | Setup | `.airsspec/` | Structure exists |
| 2 | Init | `uow/{id}/` | UOW created |
| 3 | Ingestion | Sources | Sources selected |
| 4 | Research | `requirements.md` | Requirements approved |
| 5 | Inception | `DAA.md` | DAA approved |
| 6 | Design | `ADR-*.md` | ADRs approved |
| 7 | Planning | `RFC.md`, `bolts/` | RFC approved |
| 8 | Construction | `src/*`, `tests/*` | All tasks done |
| 9 | Complete | Archive | UOW completed |

---

## Error Recovery

### Missing Artifact
If a gate artifact is missing:
1. System halts and notifies
2. User chooses: Manual creation or AI generation
3. Resume after artifact exists

### Failed Task
If a task fails:
1. Document failure in Task file
2. Assess impact (blocker? recoverable?)
3. Create remediation plan or escalate to user

---

**Alternative Workflow**: [hotfix.md](./hotfix.md) — Skip to Construction
