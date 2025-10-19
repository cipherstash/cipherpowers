---
name: Executing Workflows
description: Use workflow executor tool to run markdown-based workflows with enforcement or guided modes
when_to_use: when task has existing workflow file, when algorithmic enforcement needed, when following multi-step process
applies_to: all agents
related_skills: creating-workflows
version: 1.0.0
---

# Executing Workflows

## Overview

Execute markdown-based workflows using the workflow executor tool. Use enforcement mode for algorithmic compliance (100% vs 33% imperative) or guided mode for flexible process guidance.

**Announce at start:** "I'm using the workflow executor to run [workflow-name]."

**Core principle:** Always use the workflow tool when a workflow file exists. Don't rationalize "I can follow it manually" - that's how compliance drops from 100% to 33%.

## When to Use This Skill

**Use workflow executor when:**
- Workflow markdown file exists for the task
- Algorithmic enforcement required (git commit, TDD, code review triggers)
- Following documented multi-step process
- Need verifiable compliance with process

**Don't use when:**
- No workflow file exists (consider if one should - see creating-workflows skill)
- Single-step task (no process to enforce)
- Pure research/exploration (no defined steps)

## Algorithmic Decision Tree

Follow this algorithm to decide how to execute:

### Step 1: Does workflow file exist?

Check if workflow file exists for this task.

**→ NO:** Should a workflow exist for this task?
  - Algorithmic task (binary decisions, no judgment)? → Create workflow first (see creating-workflows skill)
  - Complex multi-step process? → Consider creating workflow
  - Simple task? → Proceed without workflow

**→ YES:** Continue to Step 2

### Step 2: Which execution mode?

**Is algorithmic enforcement required?**

Enforcement required when:
- Task has binary decisions (tests pass/fail, files exist/missing)
- No judgment calls (clear right/wrong answers)
- Compliance critical (git commit, TDD, security checks)
- Examples: git-commit-algorithm, TDD enforcement, pre-merge checks

**→ YES (Enforcement required):** Use enforcement mode

```bash
workflow path/to/workflow.md
```

**→ NO (Flexibility needed):** Use guided mode

```bash
workflow --guided path/to/workflow.md
```

Guided mode when:
- Process has judgment calls (which approach, how much detail)
- Context varies (different projects, different phases)
- Agent needs flexibility (execute-plan, feature implementation)
- Examples: execute-plan, complex refactoring, exploratory work

### Step 3: Execute and handle results

Run the command. Handle exit codes:

**Exit 0 (Success):**
- Workflow completed successfully
- Continue with next task

**Exit 1 (Stopped - condition failed):**
- Read stop message: "STOP (fix tests)", "STOP (nothing to commit)", etc.
- Fix the issue identified
- Re-run workflow from beginning

**Exit 2 (User cancelled - prompt answered no):**
- User/agent answered 'no' to prompt
- Address the prompt concern
- Re-run workflow when ready

**Exit 3 (Error - workflow issue):**
- Workflow file error (parse failure, invalid syntax)
- Check workflow file for issues
- See creating-workflows skill for syntax
- Fix workflow and re-run

## Command Reference

### Execution Commands

```bash
# Enforcement mode (default) - sequential, STOP only
workflow path/to/workflow.md

# Guided mode - enables Continue/GoTo
workflow --guided path/to/workflow.md

# Dry run - preview steps without executing
workflow --dry-run path/to/workflow.md

# List steps - show workflow structure
workflow --list path/to/workflow.md
```

### Common Workflows

```bash
# Git commit enforcement
workflow plugin/practices/git-commit-algorithm.md

# Execute plan (guided mode)
workflow --guided docs/work/2025-10-19-feature/plan.md
```

## Related Skills

- **Creating workflows:** `@${CLAUDE_PLUGIN_ROOT}skills/workflow/creating-workflows/SKILL.md`
- **Workflow practice:** `@${CLAUDE_PLUGIN_ROOT}practices/workflow.md`
