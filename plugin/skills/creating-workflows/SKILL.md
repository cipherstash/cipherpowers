---
name: Creating Workflows
description: Design and write markdown-based workflows with steps, commands, conditionals, and prompts
when_to_use: when documenting multi-step process, when creating algorithmic enforcement workflow, when existing workflow needs modification
applies_to: developers, agents creating workflows
related_skills: executing-workflows
version: 2.0.0
---

# Creating Workflows

## Overview

Design and write markdown-based workflows that are both readable documentation and executable processes. Use conventional markdown syntax (headers, code blocks, PASS/FAIL labels) to create workflows that work in both enforcement and guided modes.

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
## 1. Run tests

```bash
mise run test
```

- PASS: CONTINUE
- FAIL: STOP fix tests
```

**Avoid:**
```markdown
## 1. Run tests based on file type

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
- Use `CONTINUE` knowing it will be ignored (automatic)
- No `GOTO N` (sequential only)
- Examples: git-commit, TDD enforcement

**Guided workflows:**
- Use `CONTINUE`, `GOTO N`, `STOP`
- Design flow for flexibility
- Document why skips might happen
- Examples: execute-plan, feature implementation

**Same workflow, different modes** - syntax supports both.

## Workflow Syntax

Workflows use conventional markdown syntax. No special parser knowledge needed - if you know markdown, you know workflow syntax.

### Steps (H2 Headers)

Steps are level 2 headers with numbered prefix:

```markdown
## 1. Description of what this step does
## 2. Another step description
## 3. Final step
```

**Requirements:**
- Use `## ` (H2, two hashes)
- Number sequentially (1, 2, 3...)
- Flexible separator after number: `. : - )` or space
- Clear, concise description

**Examples:**
```markdown
## 1. Run all tests
## 2: Check code formatting
## 3 - Commit changes
```

### Commands (Bash Code Blocks)

Commands are bash code blocks that will be executed:

````markdown
```bash
mise run test
```

```bash
git status --porcelain
```
````

**Features:**
- Use ` ```bash ` fence
- Commands execute in shell
- Add `quiet` flag to hide output unless fails
- Multiple commands per step supported

**Quiet mode:**
- Normal: Shows all output
- Quiet: Shows output only on failure
- Use quiet for status checks, verbose for main actions

**Example:**
````markdown
## 1. Check for changes

```bash
git status --porcelain
```

## 2. Run tests

```bash
mise run test
```
````

### Conditionals (PASS/FAIL Lists)

Conditionals control flow based on command exit codes using list syntax:

```markdown
- PASS: CONTINUE
- PASS: GOTO 5
- FAIL: STOP fix tests
- FAIL: CONTINUE
```

**Convention:**
- Exit code 0 = PASS
- Exit code non-zero = FAIL

**Implicit defaults (most common):**
- Commands without lists use: PASS → CONTINUE, FAIL → STOP
- Prompts without lists use: Always CONTINUE
- 90% of steps use implicit defaults (no list needed)

**Available actions:**
- `CONTINUE` - Proceed to next step
- `STOP` - Stop workflow with no message
- `STOP message` - Stop with helpful message (no parentheses)
- `GOTO N` - Jump to specific step number

**Enforcement mode behavior:**
- `STOP` works as written
- `CONTINUE` and `GOTO N` ignored (automatic progression)

**Guided mode behavior:**
- All conditionals work as written
- Full control flow enabled

**Atomic conditionals principle:**
Either use defaults (no list) OR override both branches (2-item list).

**Example:**
```markdown
## 1. Run tests

```bash
mise run test
```

- PASS: CONTINUE
- FAIL: STOP tests must pass before commit

## 2. Check for unstaged changes

```bash
git diff --check
```

- PASS: CONTINUE
- FAIL: STOP whitespace errors found
```

**Complex conditions:** Use wrapper scripts to translate logic to exit codes:

```bash
# mise-task: check-has-changes
git status --porcelain | grep -q . && exit 0 || exit 1
```

```markdown
## 1. Check for changes

```bash
mise run check-has-changes
```

- PASS: CONTINUE
- FAIL: STOP nothing to commit
```

### Prompts (Implicit Text)

Prompts ask yes/no questions during execution. Steps without code blocks are treated as prompts:

```markdown
## 2. Verify test coverage

Are all functions covered by tests?

## 5. Review changes

Have you reviewed the changes?
```

**Syntax:**
- No `**Prompt:**` prefix needed (implicit)
- Just write the question as step content
- Execution waits for y/n answer
- Answering 'n' or Enter stops workflow (exit 2)

**When to use:**
- Human judgment required
- Cannot be automated (test coverage judgment, review quality)
- Yes/no decision point

**Example:**
```markdown
## 3. Verify test coverage

Do ALL new/modified functions have tests?

## 4. Commit changes

```bash
git commit
```
```

## Complete Syntax Example

Here's a workflow using all syntax elements:

```markdown
# Git Commit Workflow

## 1. Check for changes

```bash
mise run check-has-changes
```

- PASS: CONTINUE
- FAIL: STOP nothing to commit

## 2. Verify tests exist

Do ALL new/modified functions have tests?

## 3. Run test suite

```bash
mise run test
```

- PASS: CONTINUE
- FAIL: STOP fix failing tests

## 4. Check formatting

```bash
mise run fmt -- --check
```

- PASS: CONTINUE
- FAIL: STOP run mise fmt to format

## 5. Commit changes

```bash
git add .
git commit
```
```

**This workflow demonstrates:**
- Workflow title (H1 header)
- Steps (H2 headers with sequential numbers)
- One command per step (enforced)
- Conditionals (PASS/FAIL lists)
- Prompts (implicit - steps without code blocks)
- Explicit conditionals with list syntax

## Examples by Type

### Simple Sequential Workflow

For linear processes with no branching:

```markdown
# Setup and Test

## 1. Setup

```bash
mise install
```

- PASS: CONTINUE
- FAIL: STOP setup failed

## 2. Build

```bash
mise run build
```

- PASS: CONTINUE
- FAIL: STOP build failed

## 3. Test

```bash
mise run test
```

- PASS: CONTINUE
- FAIL: STOP tests failed
```

**Use for:** CI/CD steps, setup processes, verification workflows

### Workflow with Prompts

For processes requiring human judgment:

```markdown
# Documentation Publishing

## 1. Generate documentation

```bash
mise run docs
```

## 2. Review documentation

Is the documentation complete and accurate?

## 3. Publish documentation

```bash
mise run publish-docs
```
```

**Use for:** Release processes, review workflows, quality checks

### Workflow with Conditional Flow (Guided Mode)

For processes with dynamic paths, use wrapper scripts:

```markdown
# Application Startup

## 1. Check if migration needed

```bash
mise run check-has-migrations  # Returns 0 if migrations exist, 1 if none
```

- PASS: GOTO 3
- FAIL: CONTINUE

## 2. Run migrations

```bash
mise run migrate
```

## 3. Start application

```bash
mise run start
```
```

**Wrapper script** (in mise-task or separate file):
```bash
# mise-task: check-has-migrations
[ -n "$(ls migrations/*.sql 2>/dev/null)" ] && exit 0 || exit 1
```

**Use for:** Conditional setup, smart workflows, context-aware processes

### Complex Enforcement Workflow

Real example - git commit algorithm:

```markdown
# Git Commit Readiness

## 1. Verify changes exist

```bash
mise run check-has-changes
```

- PASS: CONTINUE
- FAIL: STOP nothing to commit

## 2. Check tests pass

```bash
mise run test
```

- PASS: CONTINUE
- FAIL: STOP fix tests before committing

## 3. Verify test coverage

Do ALL new/modified functions have tests?

## 4. Check formatting

```bash
mise run fmt -- --check
```

- PASS: CONTINUE
- FAIL: STOP run mise fmt to format code

## 5. Check for debugging code

```bash
mise run check-no-debug
```

- PASS: CONTINUE
- FAIL: STOP remove debugging code

## 6. Verify atomic commit

Does this commit represent ONE logical change?

## 7. Create commit

```bash
git commit
```
```

**Wrapper scripts:**
```bash
# mise-task: check-has-changes
git status --porcelain | grep -q . && exit 0 || exit 1

# mise-task: check-no-debug
git diff --cached | grep -qE 'console\.log|debugger|TODO' && exit 1 || exit 0
```

**Use for:** Git workflows, code review triggers, TDD enforcement

## Testing Workflows

Before using a workflow, verify it works:

### 1. Dry Run

Preview execution without running commands:

```bash
workflow --dry-run path/to/workflow.md
```

**Checks:**
- All steps parsed correctly
- Commands visible
- Prompts identified
- No syntax errors

### 2. List Steps

Verify structure:

```bash
workflow --list path/to/workflow.md
```

**Checks:**
- Correct number of steps
- Step numbers sequential
- Commands counted correctly

### 3. Test Execution

Run in test environment:

```bash
# Create test scenario
cd /tmp/test-workflow
git init

# Run workflow
workflow path/to/workflow.md

# Verify behavior matches expectations
```

### 4. Test Both Modes

If workflow has Continue/GoTo, test both modes:

```bash
# Test enforcement (should ignore Continue/GoTo)
workflow workflow.md

# Test guided (should respect Continue/GoTo)
workflow --guided workflow.md
```

**Verify:**
- Enforcement executes all steps sequentially
- Guided follows control flow as designed
- STOP works in both modes

## Common Patterns

### Pattern: Early Exit on Missing Precondition

```markdown
## 1. Check precondition

```bash
test -f required-file.txt
```

- PASS: CONTINUE
- FAIL: STOP required-file.txt not found
```

### Pattern: Multiple Checks Before Action

```markdown
## 1. Check condition A
## 2. Check condition B
## 3. Check condition C
## 4. Perform action (only if all checks passed)
```

### Pattern: Confirmation Before Destructive Action

```markdown
# File Deletion

## 1. Show what will be deleted

```bash
ls files-to-delete/
```

## 2. Confirm deletion

Proceed with deletion?

## 3. Delete files

```bash
rm -rf files-to-delete/
```
```

## Remember

- **Document the process, not just the happy path** - include error cases
- **Meaningful STOP messages** - tell user what to fix
- **Test workflows before using** - dry-run, list, test execution
- **Keep it simple** - complex logic in agent, simple steps in workflow
- **Executable documentation** - real commands, not pseudocode
- **Consider both modes** - design for enforcement OR guided, document which

## References

- **Executing workflows:** `@${CLAUDE_PLUGIN_ROOT}skills/executing-workflows/SKILL.md`
- **Workflow practice:** `@${CLAUDE_PLUGIN_ROOT}standards/workflow.md`
- **Workflow tool README:** `@${CLAUDE_PLUGIN_ROOT}tools/workflow/README.md`
- **Example workflows:** `@${CLAUDE_PLUGIN_ROOT}tools/workflow/examples/`
- **Git commit algorithm:** `@${CLAUDE_PLUGIN_ROOT}workflows/git-commit.md`
