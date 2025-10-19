---
name: Creating Workflows
description: Design and write markdown-based workflows with steps, commands, conditionals, and prompts
when_to_use: when documenting multi-step process, when creating algorithmic enforcement workflow, when existing workflow needs modification
applies_to: developers, agents creating workflows
related_skills: executing-workflows
version: 1.0.0
---

# Creating Workflows

## Overview

Design and write markdown-based workflows that are both readable documentation and executable processes. Use conventional markdown syntax (headers, code blocks, arrows, bold) to create workflows that work in both enforcement and guided modes.

**Announce at start:** "I'm creating a workflow for [task-name]."

**Core principle:** Every multi-step process should be documented as workflow, even if partial. Executable documentation beats separate docs and scripts.

## When to Create Workflows

**Create workflow when:**
- Multi-step process worth documenting
- Algorithmic decisions (binary choices, clear right/wrong)
- Repeatable process used by multiple agents/developers
- Process where compliance matters

**Even if:**
- Not all steps are executable (document what you can)
- Process might change (easier to update than redocument)
- Seems simple (simple processes benefit from documentation too)

**Don't create when:**
- Single-step task (just execute it)
- Purely exploratory (no defined steps yet)
- Process is truly ad-hoc (won't be repeated)

## Design Principles

### 1. Simplicity First

Complex logic lives in agents, not workflows. Workflows describe steps, agents make judgment calls.

**Good:**
```markdown
# Step 1: Run tests

```bash
mise run test
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (fix tests)
```

**Avoid:**
```markdown
# Step 1: Run tests based on file type

If TypeScript files changed:
  Run TypeScript tests
Else if Rust files changed:
  Run Rust tests
Else if both changed:
  Run all tests
```

**Reason:** Let agent detect file changes and call appropriate workflow. Keep workflow focused.

### 2. Executable Documentation

Workflow should be readable by humans AND executable by tool. Single source of truth.

**Good:**
- Clear step descriptions
- Actual commands that work
- Meaningful conditional messages

**Avoid:**
- Pseudocode instead of real commands
- Vague descriptions ("do the thing")
- Conditionals without clear messages

### 3. Enforcement vs Guided Design

Same syntax works in both modes. Choose based on usage:

**Enforcement workflows:**
- Only use `STOP` with meaningful messages
- Use `Continue` knowing it will be ignored (automatic)
- No `Go to Step X` (sequential only)
- Examples: git-commit, TDD enforcement

**Guided workflows:**
- Use `Continue`, `Go to Step X`, `STOP`
- Design flow for flexibility
- Document why skips might happen
- Examples: execute-plan, feature implementation

**Same workflow, different modes** - syntax supports both.

## Related Skills

- **Executing workflows:** `@${CLAUDE_PLUGIN_ROOT}skills/workflow/executing-workflows/SKILL.md`
- **Workflow practice:** `@${CLAUDE_PLUGIN_ROOT}practices/workflow.md`
