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
workflow plugin/workflows/git-commit.md

# Execute plan (guided mode)
workflow --guided docs/work/2025-10-19-feature/plan.md
```

## Execution Modes Explained

### Enforcement Mode (Default)

**Purpose:** Algorithmic compliance - prevent rationalization under pressure.

**Behavior:**
- Steps execute sequentially (1 → 2 → 3...)
- Only `STOP` conditionals respected
- `Continue` and `Go to Step X` ignored (automatic progression)
- No way to skip steps

**When to use:**
- Git commit readiness checks
- TDD enforcement (test must exist before code)
- Code review triggers (must review before merge)
- Security checks, compliance workflows
- Any task where 100% compliance required

**Example:**
```bash
# Git commit algorithm - must complete all 10 steps
workflow plugin/workflows/git-commit.md
```

**Risk prevented:** Agent rationalizes "I can skip this step because..." → 33% compliance drops to 0%

### Guided Mode (--guided)

**Purpose:** Flexible guidance - prevent "I don't need the workflow" while allowing adaptation.

**Behavior:**
- All conditionals enabled (Continue, GoTo, STOP)
- Agent can skip steps via conditionals
- Workflow guides process, agent adapts to context

**When to use:**
- Execute plan (tasks vary, judgment calls needed)
- Complex refactoring (approach depends on findings)
- Feature implementation (design emerges during work)
- Any process with context-dependent decisions

**Example:**
```bash
# Execute plan - tasks might be skipped based on context
workflow --guided docs/work/2025-10-19-feature/plan.md
```

**Risk prevented:** Agent rationalizes "I don't need to use the workflow tool" → avoids tool entirely

### Why Two Modes?

**Two risks to prevent:**

**Risk 1:** Agent rationalizes "I don't need the workflow tool at all"
- **Solution:** Mandate workflow tool usage in instructions
- **Both modes prevent this** - agent must invoke tool

**Risk 2:** Agent rationalizes "I can skip this step because..."
- **Solution:** Enforcement mode removes conditional flow
- **Only enforcement mode prevents this** - no skipping possible

**Guided mode still requires tool usage** (prevents Risk 1) but **allows flexibility** (accepts Risk 2 when appropriate).

## Common Scenarios

### Scenario 1: Workflow Stops with Message

```bash
workflow plugin/workflows/git-commit.md
```

Output:
```
✗ Failed (exit 1)
→ Condition matched: STOP (tests failing)
→ Workflow stopped
```

**Action:**
1. Read the stop message: "tests failing"
2. Fix the issue (make tests pass)
3. Re-run workflow: `workflow plugin/workflows/git-commit.md`

### Scenario 2: Guided Mode Skips Steps

```bash
workflow --guided docs/work/feature/plan.md
```

Output:
```
✓ Passed (exit 0)
→ Condition matched: Go to Step 5
→ Step 2: Task description
→ Step 5: Later task
```

**Action:**
- Steps 3-4 skipped as designed (GoTo worked)
- Document why skipped if significant
- Continue with workflow

### Scenario 3: Workflow Parse Error

```bash
workflow path/to/workflow.md
```

Output:
```
Error: workflow.md:23: Invalid conditional syntax
```

**Action:**
1. Check workflow file at line 23
2. See creating-workflows skill for syntax
3. Fix syntax error
4. Re-run workflow

### Scenario 4: Dry Run Preview

```bash
workflow --dry-run plugin/workflows/git-commit.md
```

**When to use:**
- Understand workflow before executing
- Verify workflow structure
- Check how many steps involved

**Then execute:**
```bash
workflow plugin/workflows/git-commit.md
```

## Remember

- **Always use workflow tool when workflow file exists** - don't rationalize manual execution
- **Enforcement mode for algorithmic tasks** - prevents step skipping
- **Guided mode for flexible processes** - prevents tool avoidance
- **Read stop messages carefully** - they tell you what to fix
- **Re-run from beginning after fixes** - don't try to resume mid-workflow
- **Document significant skips in guided mode** - explain why when reviewing work

## References

- **Creating workflows:** `@${CLAUDE_PLUGIN_ROOT}skills/workflow/creating-workflows/SKILL.md`
- **Workflow practice:** `@${CLAUDE_PLUGIN_ROOT}standards/workflow.md`
- **Workflow tool README:** `@${CLAUDE_PLUGIN_ROOT}tools/workflow/README.md`
