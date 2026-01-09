# Workflow: Feature (Full AI-DLC Cycle)

A step-by-step guide for implementing new features using the complete AI-DLC lifecycle.

> [!IMPORTANT]
> **Human-in-the-Loop Enforcement**
>
> This workflow includes **mandatory HALT points** after each artifact generation. The agent MUST stop and wait for explicit user approval before proceeding to the next phase. This enforces the "Trust But Verify" principle of AI-DLC.
>
> **Halt Points**:
> - After `requirements.md` → wait for approval → proceed to Inception
> - After `DAA.md` → wait for approval → proceed to Design
> - After `ADR-*.md` → wait for approval → proceed to Planning
> - After `RFC.md` + `bolts/` → wait for approval → proceed to Construction

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

### Gate - CRITICAL HALT POINT

> [!IMPORTANT]
> **HALT AND WAIT FOR USER APPROVAL**

After creating `requirements.md`, you MUST:

1. **STOP** execution immediately
2. **PRESENT** the requirements to the user:
   - Summary of the problem statement
   - Key requirements list
   - Success criteria
   - Scope (in/out)
3. **ASK** for explicit approval:
   > "I have created `requirements.md` for this UOW. Please review:
   > - Path: `.airsspec/uow/{uow-id}/requirements.md`
   >
   > Do you approve these requirements? (yes/no/changes)"

4. **WAIT** for user response before proceeding:
   - If `yes` → Update `status.yaml: status: PLANNED` and proceed to Inception
   - If `no` or `changes` → Ask for feedback, revise, then repeat

**Do NOT proceed to Inception until you have explicit user approval.**

**Verification**:
- [ ] `requirements.md` exists
- [ ] User has explicitly approved (you must receive "yes" or similar)
- [ ] `status.yaml` updated to: `status: PLANNED`

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

### Gate - CRITICAL HALT POINT

> [!IMPORTANT]
> **HALT AND WAIT FOR USER APPROVAL**

After creating `DAA.md`, you MUST:

1. **STOP** execution immediately
2. **PRESENT** the DAA to the user:
   - Summary of domain concepts identified
   - Key entities and value objects
   - Bounded contexts mapped
   - Ubiquitous language definitions
3. **ASK** for explicit approval:
   > "I have created `DAA.md` for this UOW. Please review:
   > - Path: `.airsspec/uow/{uow-id}/DAA.md`
   >
   > Do you approve this Domain Architecture Analysis? (yes/no/changes)"

4. **WAIT** for user response before proceeding:
   - If `yes` → Update `status.yaml: status: PLANNED` and proceed to Design
   - If `no` or `changes` → Ask for feedback, revise, then repeat

**Do NOT proceed to Design until you have explicit user approval.**

**Verification**:
- [ ] `DAA.md` exists
- [ ] User has explicitly approved (you must receive "yes" or similar)
- [ ] `status.yaml` updated to: `status: PLANNED`

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

### Gate - CRITICAL HALT POINT

> [!IMPORTANT]
> **HALT AND WAIT FOR USER APPROVAL**

After creating ADRs, you MUST:

1. **STOP** execution immediately
2. **PRESENT** each ADR to the user:
   - For each ADR: Title, Context, Decision, Consequences
   - Summary of decisions made
   - Any trade-offs or alternatives considered
3. **ASK** for explicit approval:
   > "I have created {n} ADRs for this UOW. Please review:
   > - Path: `.airsspec/uow/{uow-id}/ADR-*.md`
   >
   > ADRs created:
   > - ADR-001: {topic}
   > - ADR-002: {topic}
   > - ...
   >
   > Do you approve these Architecture Decision Records? (yes/no/changes)"

4. **WAIT** for user response before proceeding:
   - If `yes` → Update `status.yaml: status: PLANNED` and proceed to Planning
   - If `no` or `changes` → Ask for feedback, revise, then repeat

**Do NOT proceed to Planning until you have explicit user approval.**

**Verification**:
- [ ] At least one ADR exists
- [ ] User has explicitly approved all ADRs (you must receive "yes" or similar)
- [ ] `status.yaml` updated to: `status: PLANNED`

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

### Gate - CRITICAL HALT POINT

> [!IMPORTANT]
> **HALT AND WAIT FOR USER APPROVAL**

After creating RFC.md and bolt plans, you MUST:

1. **STOP** execution immediately
2. **PRESENT** the RFC and bolt structure to the user:
   - Summary of implementation strategy from RFC
   - List of Bolts created
   - For each Bolt: brief description, number of plans/tasks
3. **ASK** for explicit approval:
   > "I have created the RFC and bolt plans for this UOW. Please review:
   > - RFC Path: `.airsspec/uow/{uow-id}/RFC.md`
   > - Bolts Path: `.airsspec/uow/{uow-id}/bolts/`
   >
   > Bolts created:
   > - {bolt-1}: {description} ({n} plans, {n} tasks)
   > - {bolt-2}: {description} ({n} plans, {n} tasks)
   > - ...
   >
   > Do you approve the RFC and bolt plans? (yes/no/changes)"

4. **WAIT** for user response before proceeding:
   - If `yes` → Update `status.yaml: status: IN_PROGRESS` and proceed to Construction
   - If `no` or `changes` → Ask for feedback, revise, then repeat

**Do NOT proceed to Construction until you have explicit user approval.**

**Verification**:
- [ ] `RFC.md` exists and is complete
- [ ] All Bolts have been created with plans and tasks
- [ ] User has explicitly approved (you must receive "yes" or similar)
- [ ] `status.yaml` updated to: `status: IN_PROGRESS`

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

## Resuming After User Approval

When user provides approval to proceed to the next phase:

1. **Update the status file**:
   ```yaml
   # .airsspec/uow/{uow-id}/status.yaml
   status: PLANNED  # or IN_PROGRESS for post-RFC
   phase: {current-phase}
   ```

2. **Confirm transition**:
   > "Thank you for the approval. Proceeding to {next-phase} phase..."

3. **Continue** with the next step in this workflow

---

## Summary Checklist

| Step | Phase | Artifact | Gate (HALT) |
|------|-------|----------|-------------|
| 1 | Setup | `.airsspec/` | Structure exists |
| 2 | Init | `uow/{id}/` | UOW created |
| 3 | Ingestion | Sources | Sources selected |
| 4 | Research | `requirements.md` | **HALT: User approves** ✋ |
| 5 | Inception | `DAA.md` | **HALT: User approves** ✋ |
| 6 | Design | `ADR-*.md` | **HALT: User approves** ✋ |
| 7 | Planning | `RFC.md`, `bolts/` | **HALT: User approves** ✋ |
| 8 | Construction | `src/*`, `tests/*` | All tasks done |
| 9 | Complete | Archive | UOW completed |

> [!NOTE]
> ✋ = **HALT AND WAIT** for explicit user approval before proceeding

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
