---
name: Development Workflow
description: Sequence development work in small increments through analysis, planning, implementation, verification, review, and summarization with organized work directories. Includes executable workflow syntax specification.
when_to_use: when organizing and tracking development work from initial research through final documentation, or when creating executable workflow files
applies_to: all projects
related_practices: git-guidelines.md, code-review.md
version: 2.0.0
---

# Development Workflow

## Overview

Development work is sequenced in small increments, roughly equivalent to an "epic" or "feature".

At a high level, the flow is:

- **analysis**: research, analyze and recommend potential approaches or solutions
- **planning**: create detailed task breakdown
- **implementation**: execute and implement tasks
- **verification**: test and verify the implementation
- **review**: code and documentation review
- **summarise**: document implementation, experiments and lessons (including discarded approaches)

The flow is not necessarily linear - we react and respond as new information, constraints and opportunities arise.
Analysis and planning may be merged into a single step if the work is well-understood or simple to implement. For example, updating dependencies or fixing an issue does not need the same level of research and analysis as adding a completely new feature.

In practice, implementation and verification are an iterative feedback loop. We implement and verify each task in turn.

## Executable Workflows

Workflows can be written in markdown format and executed using the `workflow` CLI tool (`plugin/tools/workflow`). This provides algorithmic enforcement of processes, preventing rationalization under pressure.

**Two execution modes:**
- **Enforcement mode (default):** Sequential execution, only STOP conditionals respected (100% compliance)
- **Guided mode (--guided):** Full control flow enabled, workflow serves as guide

**See also:**
- Tool documentation: `plugin/tools/workflow/README.md`
- Creating workflows: `plugin/skills/workflow/creating-workflows/SKILL.md`
- Executing workflows: `plugin/skills/workflow/executing-workflows/SKILL.md`

## Workflow Syntax

Executable workflows use clean markdown with ALLCAPS keywords and minimal syntax.

### Keywords (ALLCAPS)

- `PASS` / `FAIL` - Condition branches (based on exit code)
- `GOTO N` - Jump to step N
- `STOP message` - Halt execution with optional message
- `CONTINUE` - Proceed to next step

### Structure

**Workflow title (required):**
```markdown
# My Workflow Title
```

**Steps (H2, sequential numbering):**
```markdown
## 1. First step
## 2. Second step
## 3. Third step
```

**Commands (code blocks):**
```markdown
## 1. Run tests

```bash
cargo test
```
```

**Prompts (no code block):**
```markdown
## 2. Review changes

Are these changes focused on a single logical change?
```

**Explicit conditionals (list syntax):**
```markdown
## 3. Verify atomicity

Are changes focused on single logical change?

- PASS: CONTINUE
- FAIL: GOTO 5
```

### Implicit Defaults

Steps without explicit conditionals use defaults:
- **Command steps:** PASS→CONTINUE, FAIL→STOP
- **Prompt steps:** Always CONTINUE

Only override when behavior differs from defaults.

### Atomic Conditional Principle

**Either:**
- No list → Use implicit defaults
- List present → Must have exactly 2 items (PASS and FAIL)

**Invalid:**
```markdown
- FAIL: STOP  ❌ Missing PASS branch
```

**Valid:**
```markdown
- PASS: CONTINUE
- FAIL: STOP fix first
```

### Validation

```bash
# Validate workflow structure without executing
workflow --validate workflow.md

# Dry run (show steps, don't execute commands)
workflow --dry-run workflow.md

# Execute in enforcement mode
workflow workflow.md

# Execute in guided mode (all conditionals enabled)
workflow --guided workflow.md
```

**Validation checks:**
- Exactly one H1 (workflow title)
- All steps use H2 (`##`)
- Sequential numbering (1, 2, 3...)
- GOTO targets exist
- Conditional lists have exactly 2 items (PASS and FAIL)
- No duplicate PASS or FAIL branches
- Keywords are ALLCAPS
- No "Step" keyword in headers

### Example Workflow

```markdown
# Git Commit Readiness

## 1. Check for changes

```bash quiet
check-has-changes
```

- PASS: CONTINUE
- FAIL: STOP nothing to commit

## 2. Run tests

```bash
cargo test
```

- PASS: CONTINUE
- FAIL: STOP fix tests first

## 3. Review atomicity

Are changes focused on single logical change?

- PASS: CONTINUE
- FAIL: GOTO 5

## 4. Commit

```bash
git commit
```

## 5. Split changes

Break into separate commits first.
```

**Note:** Steps 2 and 4 could omit explicit conditionals (would use implicit defaults), but shown here for clarity.

## Work directory

Each unit of work is given a short, descriptive `name`.
The work `name` may also be used as the name of any associated git branch.
Each work unit has an associated work directory (customize location for your project).

Work directories are named with:
  - descriptive name
  - kebab-case
  - maximum of 5 words
  - date prefix (using ISO 8601) for clarity and ease of collation: `{YYYY-MM-DD}-{name}`

The outputs of each step of the development workflow is saved in the work directory.

Workflow step naming convention:

| Step            | Filename                 |
|-----------------|--------------------------|
| analysis        | analysis.md              |
| planning        | plan.md                  |
| implementation  | -                        |
| verification    | -                        |
| review          | `{YYYY-MM-DD}-review`    |
| summarise       | summary.md               |

The file list is not exhaustive, other files can be generated if appropriate.
The output of implementation and verification should generally be working code with tests.


<examples>
## Examples

### Example work directory names

  - 2025-10-03-user-authentication
  - 2025-03-15-password-reset-flow
  - 2025-03-16-user-profile-dashboard
  - 2025-03-17-api-rate-limiting


### Example work directory structure
```
docs/work
├── 2025-10-03-user-authentication
    └── 2025-10-03-review.md
    └── analysis.md
    └── plan.md
    └── summary.md
```

</examples>
