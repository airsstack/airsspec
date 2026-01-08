# AirSDLC: The Methodology

**AI-Responsible Software Development Lifecycle**

AirSDLC is the methodological backbone of the AirsStack ecosystem. It is a practical, open-source implementation of the **AWS AI-Driven Development Lifecycle (AI-DLC)** framework, extended with **Domain-Driven Design (DDD)** principles and rigorous artifact definitions.

While **AirsSpec** provides the *tools* (MCP, Knowledge Engine), **AirSDLC** provides the *rules*.

## 🏛️ Foundation: AWS AI-DLC

AirSDLC builds upon the core concepts introduced by AWS (Raja SP) in the [AI-DLC White Paper](https://prod.d13rzhkk8cj2z0.amplifyapp.com/):

*   **AI-Powered Execution**: AI is not just a chatbot; it creates plans, models domains, and writes code.
*   **Human Oversight**: Humans elevate from "coders" to **Validators, Curators, and Strategists**.
*   **Bolts**: Replacing "sprints" with shorter, atomic work cycles measured in hours, not weeks.
*   **Mob Elaboration**: Cross-functional teams validating AI proposals in real-time.

**AirSDLC Extensions:**
*   **DDD-First**: Enforces technology-agnostic domain modeling (DAA) before technical design.
*   **Structured Artifacts**: Defines strict schemas for DAA, ADR, TIP, and RFC.
*   **Workflow Flexibility**: Offers distinct paths for Features, Maintenance, and Emergencies.

---

## 🔄 The Lifecycle Phases

The lifecycle is designed as a **Sequential Knowledge Handoff**.

### Phase 1: Inception (The "WHAT")
**Goal**: Translate business intent into a pure, technology-agnostic domain model.

*   **Input**: PRD (Product Requirements Document).
*   **Decision**:
    *   *Complex/High Risk?* → **Full Workflow** (Generate **AI-DAA**).
    *   *Simple/Low Risk?* → **Lightweight Workflow** (Write **TIP**).
*   **Key Ritual**: **Validation Gate**. Humans must certify that the DAA/TIP accurately reflects the business rules.

### Phase 2: Design (The "HOW")
**Goal**: Synthesize the domain model with technical constraints to make immutable architectural decisions.

*   **Input**: Validated DAA/TIP + Engineer's RFC (Request for Comments).
*   **Process**:
    *   **Consult Playbook**: Identify applicable patterns (e.g., Outbox, Circuit Breaker).
    *   **AI Sparring**: Use AI to challenge the design, propose alternatives, and find edge cases.
*   **Output**: **AI-ADR** (Architectural Decision Record). This is the "Executable Specification."

### Phase 3: Construction (The "BUILD")
**Goal**: Execute the ADR using atomic units of work.

*   **Input**: Finalized AI-ADR.
*   **Process**:
    *   **Break into Bolts**: Decompose ADR into tasks (e.g., "BOLT-42: Implement DB Migration").
    *   **AI Implementation**: AI generates boilerplate and tests based on the ADR.
    *   **Human Refinement**: Engineers handle complex logic and review.
*   **Output**: Production-Ready Code + Tests.

### Phase 4: Operations (The "RUN")
**Goal**: Monitor with context and learn from incidents.

*   **Artifacts**: **Post-Mortem** and **Fix-It Bolt**.
*   **Feedback Loop**: Incidents trigger updates to the **Architectural Playbook** or **ADR** amendments.

---

## 🛤️ Workflow Variations (The Complexity Slider)

AirSDLC adapts the paperwork to the task's complexity.

| Scenario | Workflow Type | Required Artifacts |
| :--- | :--- | :--- |
| **New Complex Feature** | **Full Workflow** | Research → DAA → RFC → ADR → Bolts |
| **Simple Feature** | **Lightweight Workflow** | TIP → RFC → ADR → Bolts |
| **Refactor / Maintenance** | **Fast Path** | TIP → Single Bolt |
| **Production Incident** | **Emergency Path** | Fix-It Bolt (Post-Mortem after fix) |

### ⚡ The Fast Path (Maintenance)
For refactors or small updates where the business domain is unchanged.
1.  **Draft TIP**: Define the goal (e.g., "Switch auth error library").
2.  **Approve**: Skip RFC/ADR loops.
3.  **Execute**: Create a single Bolt linked to the TIP.

### 🚨 The Emergency Path (Firefighting)
For critical production incidents requiring immediate resolution.
1.  **Trace**: Identify the failing Bolt/ADR.
2.  **Fix-It Bolt**: Create a micro-task ("FIX-001") linked to the incident.
3.  **Deploy**: Fix first, document later.
4.  **Post-Mortem**: Update the Knowledge Base and Playbook after resolution.

---

## 💎 The Artifacts

AirsSpec agents are designed to generate, validate, and manage these specific file types.

| Artifact | Full Name | Purpose | Generator |
| :--- | :--- | :--- | :--- |
| **PRD** | Product Requirements Doc | Business goals, success metrics, user stories. | PM / Human |
| **AI-DAA** | Domain Architecture Analysis | **Tech-neutral** domain model (Bounded Contexts, Aggregates). | AI (Verifier) |
| **TIP** | Tech Implementation Proposal | Lightweight technical plan for simple features/refactors. | Engineer |
| **RFC** | Request for Comments | Proposal for technical design, citing DAA/TIP. | Engineer |
| **AI-ADR** | Arch. Decision Record | **Immutable** technical decision. The "Truth" for coders. | AI (Manager) |
| **Bolt** | - | Atomic unit of implementation (task). | Planner |
| **Fix-It Bolt**| - | Emergency repair task. | Implementer |
| **Playbook**| Architectural Playbook | Library of approved patterns (e.g., "OUTBOX-001"). | Architects |

---

## ⚡ The Automation Flow

AirsSpec automates this flow via MCP tools:

1.  **Explorer** gathers context for the PRD.
2.  **Verifier** synthesizes the PRD into an **AI-DAA** (checking against the Knowledge Base).
3.  **Manager** takes the DAA + RFC and facilitates the **Design Session**, producing an **AI-ADR**.
4.  **Planner** reads the ADR and creates **Bolts** (Tasks).
5.  **Implementer** executes Bolts, writing code that satisfies the ADR.
6.  **Auditor** scans the final code, ensuring it traces back to the ADR and DAA.

---

## 📜 The Manifesto Principles

1.  **Traceability is Non-Negotiable**:
    `Code` → `Bolt` → `ADR` → `RFC` → `DAA/TIP` → `PRD`.
2.  **AI Needs Context, Not Just Prompts**:
    Agents must be "mounted" with the relevant DAA and ADR before writing a single line of code.
3.  **Artifacts > Code**:
    The code is just one possible implementation of the ADR. The ADR is the enduring knowledge.
4.  **Human as Validator**:
    AI proposes; Human validates. No artifact moves to the next phase without explicit human approval.
