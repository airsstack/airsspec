# Research: The Theory of AI-Native Development (AI-DLC)

**Date**: 2025-12-29
**Subject**: Advanced Analysis of AirSDLC, Unit of Work Patterns, and Cognitive Architectures
**Status**: Deep Dive / Brainstorming Output
**Original Framework**: [AWS AI-Driven Development Lifecycle](https://aws.amazon.com) by Raja SP (Principal Solutions Architect, AWS)

## 1. Executive Summary: The "Fractional Cognition" Problem

Traditional Software Development Life Cycles (SDLC) assume a **Continuous Cognition** model: a human developer holds the system architecture in their head (Long-Term Memory) while editing a function (Working Memory).

**AI Agents operate on "Fractional Cognition"**:
*   **Statelessness**: They have *no* Long-Term Memory (LTM) beyond what is in the Context Window.
*   **Context Decay**: As the window fills, reasoning capability degrades ("Lost in the Middle" phenomenon).
*   **Hallucination**: When context is missing, they "predict" (guess) probability distributions rather than reasoning.

**Conclusion**: We cannot treat AI Agents as "Faster Humans". We must build a **Cognitive Exoskeleton** around them. **AirSDLC** is the specific implementation of the **AWS AI-DLC** philosophy for the AirsStack ecosystem.

---

## 2. Origin: The AWS AI-DLC Framework

This research builds directly upon the **AI-Driven Development Lifecycle (AI-DLC)** introduced by Raja SP at AWS.

### 2.1 Core Tenants from AWS
1.  **Shift from "AI Assisted" to "AI Native"**: In traditional flows, humans drive and AI assists. In AI-DLC, **AI drives execution** and humans provide **Oversight**.
2.  **The "Mob" Rituals**: Replacing asynchronous code reviews with synchronous "Mob" sessions to validate context *before* code is generated.
    *   **Mob Elaboration** (Inception): Validating the "WHAT".
    *   **Mob Construction** (Execution): Validating the "HOW".
3.  **Bolts over Sprints**: Moving from week-long "Sprints" to hour-long atomic units called **"Bolts"**.

---

## 3. Theoretical Basis: The "Cognitive Cleanroom"

We extend the AWS framework with specific architectural patterns to enforce the "Oversight" required by AI-DLC.

### 3.1 The Unit of Work (UOW) as Transactional Memory
In database theory, a **Unit of Work** ensures Atomicity, Consistency, Isolation, and Durability (ACID). We apply this to *Agent Memory*.

*   **Atomicity**: An Agent's session is either fully successful (all specs met) or fully discarded. We never "merge partial thoughts".
*   **Isolation**: The `uow/` folder acts as a **Sandbox**. By physically separating the Agent from the repo root, we prevent "Context Pollution". The Agent *cannot* see legacy code unless we explicitly `mount` it.
*   **Durability**: The `plans/` and `history/` folders act as a **Write-Ahead Log (WAL)**. If the Agent crashes, we replay the Plan.

### 3.2 Hierarchical Task Networks (HTN) vs. Chain of Thought
Most Agents use "Chain of Thought" (CoT). This is linear and fragile.
AirSDLC implements **Hierarchical Task Networks (HTN)**:
1.  **Goal**: "Implement Login" (High Abstraction).
2.  **Method (Bolt)**: "Database Layer" -> "API Layer" (Decomposition).
3.  **Primitive (Plan)**: "Create SQL Table" (Atomic Action).

**Why HTN?**
*   **Error Boundaries**: If "API Layer" fails, we don't rollback "Database Layer".
*   **Cognitive Load**: The Agent only needs to hold *one* primitive in context at a time. This keeps the token count low and reasoning high.

---

## 4. The "Trust Gap" & The Proof of Work
The central problem in AI adoption is the **Trust Gap**: "Did the AI write this code because it understands the system, or because it saw a similar pattern on StackOverflow?"

### 4.1 Traceability as Proof
AirSDLC enforces a **Cryptographic-like Chain of Trust**:
1.  **Code** is valid only if derived from a **Bolt**.
2.  **Bolt** is valid only if derived from an **ADR** (Architectural Decision Record).
3.  **ADR** is valid only if derived from a **DAA** (Domain Analysis).

If you change a line of Code, you *must* have a Plan. This prevents "Drift".

---

## 5. Architectural Implementation: "The Spec IS The Computer"

We are moving away from "The File System is a Folder" to "The File System is a Database".

*   **`.airsspec/knowledge/`**: The **Read-Only ROM**. The immutable truths of the project.
*   **`.airsspec/uow/active/`**: The **RAM**. The volatile, working memory of the current transaction.
*   **`uow/.../bolts/`**: The **CPU Threads**. The execution context for specific processes.

### 5.1 The Role of the "Mob"
In this architecture, the Human Roles shift:
*   **Not a Coder**: The AI is the Coder.
*   **The Architect (writer of ROM)**: Defining the `knowledge/` and `ADRs`.
*   **The Garbage Collector (Reviewer)**: Ensuring the `uow/` (RAM) is flushed correctly to Main (Disk) without memory leaks (bugs).
