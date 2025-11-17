# Tool-Agnostic Command Vocabulary Design

**Date:** 2025-11-17
**Status:** Approved
**Author:** Design Session

## Overview

Transform CipherPowers from a mise-specific plugin to a tool-agnostic plugin that works across any build/test tooling ecosystem (npm, cargo, make, etc.).

## Problem

The plugin currently hardcodes references to `mise run test`, `mise run check`, `mise run build`, etc. This creates unnecessary coupling to mise and limits the plugin's usefulness for projects using other task runners.

## Goals

1. Remove all mise-specific command references from the plugin
2. Use standardized vocabulary that works across ecosystems
3. Projects document their specific commands in CLAUDE.md
4. Maintain clear, natural language in skills and agents

## Design Decisions

### 1. Standardized Command Vocabulary

The plugin uses four core command concepts:

- **test** - Run the project's test suite (unit tests, integration tests, etc.)
- **check** - Run quality checks (linting, formatting, type checking, etc.)
- **build** - Build the project (compile, bundle, package, etc.)
- **run** - Execute the application (start server, run CLI, etc.)

### 2. Natural Language References

Plugin files use natural language to describe commands:
- "run the project's tests" instead of `mise run test`
- "run quality checks" instead of `mise run check`
- "build the project" instead of `mise run build`
- "run the application" instead of `mise run X`

This approach:
- Works with Claude's semantic understanding
- Adapts naturally to project context
- Matches the skill philosophy of universal workflows
- Avoids template/placeholder syntax

### 3. Project Documentation Responsibility

Projects document their specific commands in CLAUDE.md using the standardized vocabulary:

```markdown
## Development Commands

- **Tests**: `npm test` - Run the project's test suite
- **Checks**: `npm run lint && npm run typecheck` - Run quality checks
- **Build**: `npm run build` - Build/compile the project
- **Run**: `npm start` - Execute the application
```

Agents read this documentation to understand the actual commands for each concept.

### 4. CLAUDE.md Template

New file: `plugin/templates/CLAUDE.md`

Provides:
- Standardized format for documenting commands
- Examples for common ecosystems (npm, cargo, Python)
- Clear guidance on what each command concept means

## Implementation Scope

### Files to Update

**Standards & Workflows:**
1. `plugin/standards/code-review.md` - Generic verification language
2. `plugin/workflows/test-check-build.md` - Replace all mise commands
3. `plugin/workflows/git-commit.md` - Generic pre-commit language

**Agents:**
4. `plugin/agents/rust-engineer.md` - Add CLAUDE.md reading instruction
5. `plugin/agents/ultrathink-debugger.md` - Generic command references
6. `plugin/agents/code-committer.md` - Generic pre-commit language

**Skills:**
7. `plugin/skills/commit-workflow/SKILL.md` - Natural language checks
8. `plugin/skills/conducting-code-review/SKILL.md` - Generic verification
9. `plugin/skills/tdd-enforcement-algorithm/SKILL.md` - Generic test running
10. `plugin/skills/test-driven-development/SKILL.md` - Tool-agnostic workflow
11. `plugin/skills/systematic-debugging/` - Update test scenarios
12. `plugin/skills/systematic-type-migration/SKILL.md` - Generic verification

**New Files:**
13. `plugin/templates/CLAUDE.md` - Project documentation template
14. `plugin/docs/configuring-project-commands.md` - Setup guide
15. Update main `CLAUDE.md` - Reference implementation

### Transformation Pattern

**Before:**
```markdown
**Run tests:**
- Command: `mise run test`
- Requirement: ALL tests MUST pass.
```

**After:**
```markdown
**Run tests:**
- Run the project's test suite (see CLAUDE.md for the specific command)
- Requirement: ALL tests MUST pass.
```

**Agent instruction addition:**
```markdown
Read the project's CLAUDE.md to understand the specific commands for
testing, checking, building, and running the application.
```

## Migration Strategy

### For CipherPowers Repository
1. Update main `CLAUDE.md` to follow the new template format
2. Document cipherpowers' own commands (currently mise-based)
3. Serves as reference implementation for other projects

### Documentation
1. Create configuration guide explaining tool-agnostic approach
2. Provide examples for common ecosystems
3. Explain benefits: flexibility, maintainability, polyglot support

### Rollout
1. Update all plugin files in one coherent change
2. Update cipherpowers' CLAUDE.md as example
3. Release with updated documentation

## Benefits

**For Users:**
- Plugin works with any build/test tooling
- Clear separation: plugin provides workflows, projects provide implementation
- Better for polyglot teams using different tools per project
- More intuitive: "run tests" is clearer than "mise run test"

**For Maintainers:**
- No tool-specific assumptions to maintain
- Easier to evolve: change vocabulary once, not scattered references
- More testable: skills describe universal patterns
- Better upstream contribution path to superpowers

## Non-Goals

- **Not** removing mise from cipherpowers' own development (we can still use mise)
- **Not** auto-detecting task runners (projects document explicitly)
- **Not** backward compatibility with mise-specific references
- **Not** creating abstraction layer or command wrapper tool

## Success Criteria

1. Zero references to "mise" in plugin files (except cipherpowers' own CLAUDE.md)
2. All command references use natural language
3. Template CLAUDE.md created and documented
4. CipherPowers' own CLAUDE.md updated as reference
5. Plugin works seamlessly with npm, cargo, and make-based projects
