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

## Workflow Syntax

Workflows use conventional markdown syntax. No special parser knowledge needed - if you know markdown, you know workflow syntax.

### Steps (H1 Headers)

Steps are level 1 headers with "Step N:" prefix:

```markdown
# Step 1: Description of what this step does
# Step 2: Another step description
# Step 3: Final step
```

**Requirements:**
- Use `# ` (H1, one hash)
- Include "Step N:" prefix
- Number sequentially (1, 2, 3...)
- Clear, concise description

**Example:**
```markdown
# Step 1: Run all tests
# Step 2: Check code formatting
# Step 3: Commit changes
```

### Commands (Bash Code Blocks)

Commands are bash code blocks that will be executed:

````markdown
```bash
mise run test
```

```bash quiet
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
# Step 1: Check for changes

```bash quiet
git status --porcelain
```

# Step 2: Run tests

```bash
mise run test
```
````

### Conditionals (Arrow Notation)

Conditionals control flow based on command results:

```markdown
→ Exit 0: Continue
→ Exit ≠ 0: STOP (fix tests)
→ If output empty: STOP (nothing to commit)
→ If output contains "error": STOP (found errors)
→ Otherwise: Continue
→ Exit 0: Go to Step 5
```

**Syntax:**
- Start with `→` or `->`
- Condition: `Exit 0`, `Exit ≠ 0`, `If output empty`, `If output contains "text"`, `Otherwise`
- Action: `Continue`, `STOP`, `STOP (message)`, `Go to Step N`

**Enforcement mode behavior:**
- `STOP` works as written
- `Continue` and `Go to Step X` ignored (automatic progression)

**Guided mode behavior:**
- All conditionals work as written
- Full control flow enabled

**Example:**
```markdown
# Step 1: Run tests

```bash
mise run test
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (tests must pass before commit)

# Step 2: Check for unstaged changes

```bash quiet
git diff --check
```

→ If output empty: Continue
→ Otherwise: STOP (whitespace errors found)
```

### Prompts (Bold Text)

Prompts ask yes/no questions during execution:

```markdown
**Prompt:** Are all functions covered by tests?

**Prompt:** Have you reviewed the changes?
```

**Syntax:**
- Use `**Prompt:**` (bold "Prompt:")
- Follow with question
- Execution waits for y/n answer
- Answering 'n' or Enter stops workflow (exit 2)

**When to use:**
- Human judgment required
- Cannot be automated (test coverage judgment, review quality)
- Yes/no decision point

**Example:**
```markdown
# Step 3: Verify test coverage

**Prompt:** Do ALL new/modified functions have tests?

# Step 4: Commit changes

```bash
git commit
```
```

## Complete Syntax Example

Here's a workflow using all syntax elements:

```markdown
# Step 1: Check for changes

```bash quiet
git status --porcelain
```

→ If output empty: STOP (nothing to commit)
→ Otherwise: Continue

# Step 2: Verify tests exist

**Prompt:** Do ALL new/modified functions have tests?

# Step 3: Run test suite

```bash
mise run test
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (fix failing tests)

# Step 4: Check formatting

```bash quiet
mise run fmt -- --check
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (run mise fmt to format)

# Step 5: Commit changes

```bash
git add .
git commit
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (commit failed)
```

**This workflow demonstrates:**
- Steps (H1 headers with numbers)
- Commands (bash blocks, some quiet)
- Conditionals (exit codes, output checks)
- Prompts (manual verification)
- Flow control (STOP with messages, Continue)

## Examples by Type

### Simple Sequential Workflow

For linear processes with no branching:

```markdown
# Step 1: Setup

```bash
mise install
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (setup failed)

# Step 2: Build

```bash
mise run build
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (build failed)

# Step 3: Test

```bash
mise run test
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (tests failed)
```

**Use for:** CI/CD steps, setup processes, verification workflows

### Workflow with Prompts

For processes requiring human judgment:

```markdown
# Step 1: Generate documentation

```bash
mise run docs
```

# Step 2: Review documentation

**Prompt:** Is the documentation complete and accurate?

# Step 3: Publish documentation

```bash
mise run publish-docs
```
```

**Use for:** Release processes, review workflows, quality checks

### Workflow with Conditional Flow (Guided Mode)

For processes with dynamic paths:

```markdown
# Step 1: Check if migration needed

```bash quiet
ls migrations/*.sql 2>/dev/null | wc -l
```

→ If output empty: Go to Step 3
→ Otherwise: Continue

# Step 2: Run migrations

```bash
mise run migrate
```

# Step 3: Start application

```bash
mise run start
```
```

**Use for:** Conditional setup, smart workflows, context-aware processes

### Complex Enforcement Workflow

Real example - git commit algorithm:

```markdown
# Step 1: Verify changes exist

```bash quiet
git status --porcelain
```

→ If output empty: STOP (nothing to commit)
→ Otherwise: Continue

# Step 2: Check tests pass

```bash
mise run test
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (fix tests before committing)

# Step 3: Verify test coverage

**Prompt:** Do ALL new/modified functions have tests?

# Step 4: Check formatting

```bash quiet
mise run fmt -- --check
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (run mise fmt to format code)

# Step 5: Check for debugging code

```bash quiet
git diff --cached | grep -E 'console\.log|debugger|TODO'
```

→ If output empty: Continue
→ Otherwise: STOP (remove debugging code)

# Step 6: Verify atomic commit

**Prompt:** Does this commit represent ONE logical change?

# Step 7: Create commit

```bash
git commit
```
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
# Step 1: Check precondition

```bash quiet
test -f required-file.txt
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (required-file.txt not found)
```

### Pattern: Multiple Checks Before Action

```markdown
# Step 1: Check condition A
# Step 2: Check condition B
# Step 3: Check condition C
# Step 4: Perform action (only if all checks passed)
```

### Pattern: Confirmation Before Destructive Action

```markdown
# Step 1: Show what will be deleted

```bash
ls files-to-delete/
```

# Step 2: Confirm deletion

**Prompt:** Proceed with deletion?

# Step 3: Delete files

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

- **Executing workflows:** `@${CLAUDE_PLUGIN_ROOT}skills/workflow/executing-workflows/SKILL.md`
- **Workflow practice:** `@${CLAUDE_PLUGIN_ROOT}practices/workflow.md`
- **Workflow tool README:** `@${CLAUDE_PLUGIN_ROOT}tools/workflow/README.md`
- **Example workflows:** `@${CLAUDE_PLUGIN_ROOT}tools/workflow/examples/`
- **Git commit algorithm:** `@${CLAUDE_PLUGIN_ROOT}practices/git-commit-algorithm.md`
