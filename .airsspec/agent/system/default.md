# Custom System Instructions

Add project-specific instructions here. These will be appended to the base system prompt.

## Project-Specific Rules

AirsSpec is a self-referential framework — it uses its own workflows and AI-DLC phases to develop itself.

### Context Awareness

When working on AirsSpec itself:
- Remember that this framework is dogfooding — using its own processes
- The `.airsspec/agent/` directory contains the project-local instruction copy
- Upstream instructions remain in `instructions/` for reference

### Development Priorities

1. **Documentation First**: All changes must be reflected in docs/ before implementation
2. **Instruction Consistency**: Any workflow changes should update instructions/workflows/
3. **Template Updates**: New artifact formats should be added to templates/

### Known Constraints

- No implementation code exists yet — this is a specification-only phase
- When implementation begins, it will likely be in Rust (based on project goals)
- Focus on spec quality over implementation details at this stage
