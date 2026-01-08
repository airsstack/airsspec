# AirsSpec Templates

This directory contains reusable templates for AI-DLC artifacts.

## Usage

Copy templates to your UOW directory when starting a new phase:

```bash
# Research phase
cp templates/uow/requirements.md .airsspec/uow/{uow-id}/

# Inception phase
cp templates/uow/DAA.md .airsspec/uow/{uow-id}/

# Design phase
cp templates/uow/ADR.md .airsspec/uow/{uow-id}/ADR-001-topic.md

# Planning phase
cp templates/uow/RFC.md .airsspec/uow/{uow-id}/
```

## Template Structure

```
templates/
├── uow/                   # Unit of Work artifacts
│   ├── requirements.md    # Research → PRD
│   ├── DAA.md             # Inception → Domain model
│   ├── ADR.md             # Design → Architecture decisions
│   ├── RFC.md             # Planning → Implementation strategy
│   └── status.yaml        # UOW lifecycle state
├── bolt/                  # Bolt artifacts
│   ├── status.yaml        # Bolt module state
│   ├── PLAN.md            # Execution plan
│   └── TASK.md            # Task tracking
└── airsspec.toml.example  # Example configuration
```

## Artifact Flow

| Phase | Template | Output |
|-------|----------|--------|
| Research | `requirements.md` | `uow/{id}/requirements.md` |
| Inception | `DAA.md` | `uow/{id}/DAA.md` |
| Design | `ADR.md` | `uow/{id}/ADR-*.md` |
| Planning | `RFC.md` | `uow/{id}/RFC.md` |
| Planning | `PLAN.md` | `uow/{id}/bolts/{id}/plans/PLAN-*.md` |
| Construction | `TASK.md` | `uow/{id}/bolts/{id}/tasks/TASK-*.md` |
