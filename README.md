# AirsSpec

**Knowledge-Grounded, Spec-Driven Development Framework**

AirsSpec is a unified framework for **Research-Driven Development**. It bridges the gap between high-level knowledge synthesis (inspired by Google NotebookLM) and low-level execution (inspired by Spec Kit and OpenSpec).

It serves as the **technical engine** for the [**AirSDLC Methodology**](AIRSDLC.md), orchestrating the entire lifecycle through the **Model Context Protocol (MCP)**.

## 🚀 Vision

AirsSpec moves beyond "AI coding assistants" to **Spec-Driven Engineering**. In this paradigm, precise markdown specifications are not just documentation—they are **executable contracts** that drive the AI agents.

By combining the **AWS AI-Driven Development Lifecycle (AI-DLC)** with rigorous **Spec-Driven Development (SDD)**, AirsSpec ensures that AI agents act as reliable engineers, not just unpredictable chatbots.

## 🌟 Core Philosophy

> **"No Code without Spec, No Spec without Knowledge."**

AirsSpec enforces the **AirSDLC Manifesto**:
1.  **Traceability**: Every line of code traces back to a **Bolt**, which traces to an **ADR**, which traces to a **DAA**, which traces to the **PRD**.
2.  **Context Isolation**: Agents act through an "MCP Firewall," accessing only verified knowledge, not raw file systems.
3.  **Human Validation**: AI proposes plans and models; Humans validate them before execution.

## 🏗️ Foundation: AirSDLC & AWS AI-DLC

AirsSpec operationalizes the **AirSDLC** framework, which is an implementation of the **AWS AI-Driven Development Lifecycle (AI-DLC)**. It brings structure to AI development through specific artifacts and workflows:

*   **Bolts**: Atomic units of work (replacing sprints).
*   **Mob Elaboration/Construction**: Real-time human-AI validation loops.
*   **Dual Workflow**: Supporting both "Full" (DAA-based) and "Lightweight" (TIP-based) paths.

---

## 📦 AI-DLC Phases

AirsSpec implements a **6-phase AI Development Lifecycle**:

| Phase | Name | Goal | Artifact |
|-------|------|------|----------|
| 0 | **Ingestion** | Load knowledge sources | Sources cataloged |
| 1 | **Research** | Define requirements | `requirements.md` |
| 2 | **Inception** | Define domain model | `DAA.md` |
| 3 | **Design** | Define architecture | `ADR-*.md` |
| 4 | **Planning** | Define execution strategy | `RFC.md`, `bolts/` |
| 5 | **Construction** | Build & verify | Source code |

Each phase has **gate conditions** that must be satisfied before proceeding.

> 🐕 **Dogfooding**: AirsSpec uses its own workflows to develop itself. The framework is built using the same AI-DLC phases and agent integrations it provides.

---

## 📂 Project Structure

```
airsspec/
├── docs/                    # Architecture documentation
│   ├── ai-dlc-phases.md     # Phase definitions
│   ├── architecture.md      # System architecture
│   ├── uow-bolt-spec.md     # UOW & Bolt specification
│   └── ...
├── instructions/            # AI agent instructions
│   ├── core/               # Foundation & setup
│   ├── phases/             # Phase-specific guides
│   └── workflows/          # End-to-end workflows
├── .opencode/agent/        # OpenCode custom agents
├── .agent/workflows/       # AntiGravity workflows
└── AGENTS.md               # Agent instructions (cross-platform)
```

---

## 🛠️ Agent Integration

AirsSpec provides custom agents and workflows for AI coding tools:

### OpenCode Agents

Invoke with `@agent_name`:

| Agent | Purpose |
|-------|---------|
| `@airsspec` | Main orchestrator |
| `@airsspec-feature` | Full AI-DLC cycle |
| `@airsspec-hotfix` | Fast track for fixes |
| `@git-commit` | Conventional Commits |

### AntiGravity Workflows

Invoke with `/workflow`:

| Workflow | Purpose |
|----------|---------|
| `/airsspec` | Main guide |
| `/airsspec-feature` | Full AI-DLC cycle |
| `/airsspec-hotfix` | Fast track for fixes |
| `/git-commit` | Conventional Commits |

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [AI-DLC Phases](docs/ai-dlc-phases.md) | 6-phase lifecycle details |
| [Architecture](docs/architecture.md) | System design |
| [UOW & Bolt Spec](docs/uow-bolt-spec.md) | Work unit architecture |
| [User Journey](docs/user-journey-and-workflow.md) | User experience flow |
| [Multi-Agent Architecture](docs/multi-agent-architecture.md) | Agent coordination |
| [Instructions](instructions/README.md) | AI agent instructions |

---

## 🚦 Getting Started

1. **For AI Agents**: Start with `instructions/core/README.md`
2. **For Humans**: Read `docs/user-journey-and-workflow.md`
3. **Initialize Workspace**: Run `/airsspec-setup` (AntiGravity) or follow `instructions/core/workspace-setup.md`

---

## 📝 Contributing

Commits follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

---

## 📄 License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
