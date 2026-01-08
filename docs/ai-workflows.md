# Brainstorming: AI Workflows in AirsSpec

## 1. Executive Summary
This document synthesizes internal research (UOW/Bolt specifications) with external industry patterns (GitHub Copilot, OpenSpec, Kiro) to define the optimal "User Experience" for AirsSpec.

**Core Insight**: While tools like Copilot focus on "Velocity" (Idea -> Code), AirsSpec focuses on "Integrity" (Context -> Spec -> Code). The workflow must reflect this "Cognitive Cleanroom" approach.

---

## 2. Proposed Workflows: "The Protocol of Intent"

We define three distinct workflows based on the user's intent: The **Creator**, The **Maintainer**, and The **Auditor**.

### Workflow A: The Creator (Greenfield / New Feature)
*User Intent: "I want to build a Payment Gateway."*

**The Flow**:
1.  **Ingestion (The Library)**
    *   **User Action**: Manually places documents (PDFs, Markdown) into the `researches/` directory. No CLI command needed.
    *   **System**: (Optional) Background indexing.
    *   **Philosophy**: "If it's in the folder, it's known information."

2.  **Ignition (The Consultant)**
    *   **User Action**: `airsspec start "Payment Gateway"`
    *   **AI Role ("The Consultant")**: instead of just asking "What now?", the AI proactively suggests:
        > "A 'Payment Gateway' typically requires a **Ledger**, **Tokenization**, and **Idempotency** handling.
        > I see you have `stripe-docs` in your library. Should I use those as the primary constraint?
        > Do you have existing architectural decisions for the **Ledger**?"
    *   **Benefit**: The AI guides the user, reducing "Blank Canvas Paralysis".

3.  **Mounting (The Context Scope)**
    *   **Concept**: "Mounting" is the act of explicitly selecting which "Books" from the "Library" are active in the Agent's "Working Memory".
    *   **Mechanism**:
        *   User: `mount researches/pci-dss.md`
        *   **System**: Executes **Context Compression Engine**.
            *   *Input*: Raw Markdown/PDF.
            *   *Process*: Summarizes/Distills key constraints relative to the current UOW.
            *   *Output*: A token-optimized summary in `.airsspec/uow/.../context/`.
    *   **Why?**: "Fractional Cognition". We don't just dump text; we compress it to maximize the Agent's attention span.

4.  **Elaboration (The Mob Ritual)**
    *   **Interactive Session**: A collaborative dialogue to write `requirements.md` and `ADRs`.
    *   **Script Example**:
        *   **AI**: "I'm reviewing the `PCI-DSS` mount. It says we cannot store CVV. How do you want to handle recurring payments?"
        *   **User**: "Use a Provider Token."
        *   **AI**: "Noted. **Decision Recorded**: Creating `adr-001-use-tokens.md`. I will also add a requirement to the DAA."
    *   **Output**:
        *   **DAA**: The Domain Model (What).
        *   **ADR**: The Technical Decision Log (How - e.g., "Use BigInt", "Use Tokenization"). Both are needed for Planning.

5.  **Architecture (The Blueprint)**
    *   **System**: Locks `requirements.md`. Transitions to Design.
    *   **AI Role ("The Architect")**: Generates `bolts/` structure AND specific **Action Plans**.
        *   `bolts/01-db/plans/01-create-schema.md`: "Task: Write SQL migration."
        *   `bolts/02-api/plans/01-define-routes.md`: "Task: Create Actix routes."
    *   **User Action**: Approves the *Strategy* (The Bolt list) and the *Tactics* (The Action Plans).

6.  **Construction (The Factory)**
    *   **System**: Parallel execution of Bolts.
    *   **AI Role ("The Engineer")**: Example: "I am working on Bolt 01. I have read Plan 01. Writing code..."
    *   **User Action**: Passive monitoring via `status.yaml`.

---

### Workflow B: The Maintainer (Brownfield / Bugfix)
*User Intent: "Fix the race condition in the ledger."*

**The Flow**:
1.  **Isolation (The Cleanroom)**
    *   **User Action**: `airsspec fix "Ledger Race Condition"`
    *   **System**: Creates `.airsspec/uow/uow-002-fix-race/`.
    *   **Critical Clarification**: **We do NOT copy code.** We only reference the *path*.
    *   **User Action**: `mount path /Users/me/projects/my-app/src/ledger` and `mount file researches/concurrency.md`.

2.  **Diagnosis (The Plan)**
    *   **AI Role ("The Detective")**: Reads the *referenced* external code vs. research.
    *   **Output**: A `plan.md` proposing the specific refactor.

3.  **Surgery (The Fix)**
    *   **System**: Agent writes changes to the **external path** defined in the mount.
    *   **Verification**: Runs tests in the external path.

---

## 3. Comparative Analysis

| Feature | GitHub Copilot Workspace | OpenSpec / Kiro | **AirsSpec (Proposed)** |
| :--- | :--- | :--- | :--- |
| **Primary Goal** | Developer Velocity | Spec Alignment | **Cognitive Integrity** |
| **Context Model** | "Context Window" (Implicit) | "Spec File" (Explicit) | **"Cognitive Cleanroom" (Isolated)** |
| **Architecture** | Chat-based | Spec-based | **Transactional (UOW/Bolt)** |
| **Human Role** | Pilot / Reviewer | Spec Author | **Architect / Auditor** |
| **State** | Fluid (Chat History) | Versioned (Git) | **Durable (WAL / Status.yaml)** |

### Why AirsSpec? (The "Why")
*   **Copilot** is great for "Help me write this function."
*   **OpenSpec** is great for "Help me align on requirements."
*   **AirsSpec** is for **"I need to trust an Agent to build a complex system asynchronously without polluting the codebase."**

---

## 4. User Benefits

### 1. "Fractional Cognition" Support
*   **Problem**: Users (and Agents) forget context.
*   **Solution**: AirsSpec's UOW structure means you can pause a task for 3 days, come back, and the `status.yaml` + `plans/` restore full context instantly. The "State" is on disk, not in RAM.

### 2. The "Hallucination Firewall"
*   **Problem**: Agents guess when they don't know constraints.
*   **Solution**: The Agent acts on the intersection of **DAA** (Requirements) and **ADR** (Decisions).
    *   **ADR as Log**: "We chose PostgreSQL" is an ADR.
    *   **DAA as Context**: "We need a Ledger" is DAA.
    *   **Result**: The Agent creates a "PostgreSQL Ledger Table" because it combined both inputs. It cannot choose MongoDB because the ADR forbids it.

### 3. "Audit-Ready" Code
*   **Problem**: "Why did the AI write this?"
*   **Solution**: Every line of code traces back to a `Task`, which traces to a `Plan`, which traces to a `Bolt`, which traces to a `UOW`, which traces to a `Research` file.

---

## 5. Next Steps for Discussion
1.  **Refine the "Consultant" Persona**: How "imaginative" should the AI be during Ignition? Should it suggest entire architectures?
2.  **Mounting UX**: Drag-and-drop vs. CLI?

