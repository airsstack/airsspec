# Research: Spec-Driven Development (SDD) Frameworks

**Date**: 2025-12-27
**Subject**: Analysis of GitHub SpecKit, OpenSpec, and AI Engineering Concepts

## 1. Executive Summary
Spec-Driven Development (SDD) is evolving from a human-centric documentation practice into an AI-native engineering discipline. Modern frameworks like **GitHub SpecKit** and **Fission-AI OpenSpec** treat specifications not as passive text, but as **executable contracts** that drive code generation, verification, and validation. For `airsspec`, adopting this paradigm means moving beyond "planning" into "authoritative specification" where the spec is the single source of truth.

## 2. Framework Analysis

### 2.1 GitHub Spec Kit
*   **Core Philosophy**: Transforms "vibe coding" (ad-hoc prompting) into structured, repeatable engineering.
*   **Workflow**: `Specify` (Intent) -> `Plan` (Architecture) -> `Tasks` (Atomic Units) -> `Implement` (Code).
*   **Key Innovation**:
    *   **Living Specs**: Specs evolve with the code.
    *   **Structure**: Uses Templates to enforce consistency (Context, Requirements, Assessment).
    *   **AI Agnostic**: Designed to work with any LLM/Agent.
*   **Takeaway for AirsSpec**: Adopting a structured "Specify -> Plan -> Task" pipeline is essential for scalability.

### 2.2 Fission-AI OpenSpec
*   **Core Philosophy**: "Spec-Anchored Alignment". A single unified specification document is the authoritative reference.
*   **Workflow**:
    *   **Change Proposal**: Draft changes to the spec first.
    *   **Review**: AI/Human validates the *spec change* before any code is written.
    *   **Implement**: Code is generated *strictly* to satisfy the new spec.
*   **Key Innovation**:
    *   **Brownfield Support**: Excellent for modifying existing systems by capturing current state.
    *   **Slash Commands**: Integrates via simplified CLI interactions.
*   **Takeaway for AirsSpec**: The concept of "Spec First, Code Second" must be enforced via tooling (e.g., pre-commit checks or agent guardrails).

## 3. General Concepts: Spec-Driven Development in AI Engineering

### 3.1 The Shift to "spec-as-code"
In AI engineering, the Specification replaces the manually written implementation details. The human role shifts to:
1.  **Requirement Architecture**: Defining *what* to build with extreme precision.
2.  **Validation**: Verifying the AI output against the spec.

### 3.2 Validation & Verification (V&V)
*   **Verification**: "Did we build the product right?" (Does code match spec?)
    *   *AI Implementation*: Agents can auto-generate unit tests *from* the spec features.
*   **Validation**: "Did we build the right product?" (Does spec match user intent?)
    *   *AI Implementation*: "Elaboration Loops" where the AI challenges the user's spec for edge cases before coding.

### 3.3 The "SDD Loop"
1.  **Draft Spec**: User defines intent in structured markdown.
2.  **Lint/Validate**: System checks spec for ambiguity (using AI).
3.  **Generate Plan**: Agent breaks spec into atomic tasks.
4.  **Execute**: Agent writes code.
5.  **Audit**: Agent (or separate "Auditor" model) compares Code vs. Spec.

## 4. Recommendations for AirsSpec

1.  **Adopt "Executable Specs"**: The `specs/` directory should not just be documentation; it should be readable by agents to generate test cases.
2.  **Implement "Spec-First" Guardrails**: Prevent agents from writing code (in `uow/`) unless a corresponding `spec/` or `adr/` exists.
3.  **Introduce "Elaboration Phase"**: Explicit workflow step where users and agents refine the spec before `construction`.
4.  **Verification Agents**: Dedicated `@airsspec-auditor` role (already planned) should specifically validate against the *Spec* document, not just general code quality.
