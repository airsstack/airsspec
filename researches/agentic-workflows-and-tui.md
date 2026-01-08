# Research: Agentic Spec Frameworks and TUI Patterns

**Status**: Draft
**Topic**: Agentic Workflows, Spec-Driven Development, and TUI Interfaces.

## 1. Agentic Coding & Spec-Driven Development (SDD)

### 1.1 The Convergence
Modern development is shifting from "Chat with AI" to "Agentic Workflow" where tools don't just answer questions but execute complex, multi-step plans.
*   **Agentic Tools**: Independent systems (like *Claude Code*, *GitHub Copilot Agent Mode*, *OpenCode*) that can plan, debug, and execute.
*   **Spec-Driven Development (SDD)**: A methodology where the *Specification* is the Single Source of Truth.
    *   *Approach*: "Spec-First". Agents write code *against* a rigorous spec, preventing drift.
    *   *Tools*: *GitHub SpecKit*, *Amazon Kiro*, *Tessl*.

### 1.2 Key Concepts for AirsSpec
*   **Structured Intent**: SDD provides the "Guardrails" for the Agent. Without a spec, agents hallucinate. With a spec, they simply "implement intent".
*   **The Blueprint**: The detailed spec (DAA/ADR) acts as the blueprint.
*   **Orchestration**: Managing the state between "Planning" (Spec) and "Execution" (Code).

## 2. Human-in-the-Loop (HITL) Interaction Patterns

How do humans control autonomy?
*   **Human-in-the-Loop (HITL)**: Agent pauses for specific approval before high-stakes actions (e.g., specific code generation, deployment).
*   **Human-on-the-Loop**: Agent runs autonomously, human watches a live dashboard and hits "Emergency Stop" if it drifts.
*   **Supervisor/Worker Model**: Human is the "Supervisor" who delegates to "Worker Agents" and reviews their aggregated output.
*   **Review Briefs**: Instead of chatting, the Agent prepares a "Checklist" of decisions. The human ticks boxes to approve/reject.

## 3. Ratatui & TUI Design Patterns

**Ratatui** (Rust TUI) is the standard for building rich terminal interfaces in Rust.

### 3.1 Why Ratatui?
*   **Performance**: Immediate mode rendering.
*   **Widget Ecosystem**: Built-in support for:
    *   **Lists/Tables**: Perfect for reviewing file changes or tasks.
    *   **Tabs**: Switching between "Code View", "Spec View", and "Logs".
    *   **Popups/Modals**: Critical for "Blocking" interactions (e.g., "Approve this Plan?").

### 3.2 Relevant UI Patterns for AirsSpec
1.  **The "Review Dashboard"**:
    *   *Left Pane*: File Tree / Task List.
    *   *Right Pane*: Diff Viewer / Content Preview.
    *   *Bottom Bar*: Status & Hotkeys (`[A]pprove`, `[R]eject`, `[E]dit`).
2.  **The "Live Activity Monitor" (Human-on-the-Loop)**:
    *   *Log Stream*: Rolling logs of agent internal thoughts.
    *   *Progress Gauge*: "Analyzing 3 files...", "Generating Tests...".
    *   *Spinner*: Visual feedback for async work.
3.  **The "Decision Matrix"**:
    *   A simple customized List Widget with Checkboxes.
    *   "I found 3 conflicts. Choose resolution:"
        *   `[x] Use Local Version`
        *   `[ ] Use Remote`

## 4. Synthesis for AirsSpec
*   **Adoption**: We should adopt the **"Review Brief"** pattern implemented via a **Ratatui Dashboard**.
*   **Workflow**:
    1.  Agent runs headless (async).
    2.  Agent hits a "Decision Point" (Phase Boundary).
    3.  Agent launches the TUI (Dashboard).
    4.  User interacts (ticks boxes, views diffs).
    5.  User hits "Approve".
    6.  TUI closes, Agent resumes headless mode.
