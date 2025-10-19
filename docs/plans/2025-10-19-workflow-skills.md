# Workflow Skills Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Create two skills documenting workflow executor usage - one for executing workflows (high frequency, all agents) and one for creating workflows (low frequency, developers/agents writing workflows).

**Architecture:** Two separate skills following CipherPowers architecture. Executing skill provides algorithmic decision tree for when/how to invoke tool. Creating skill documents syntax and design principles. Both reference workflow.md practice for conventions.

**Tech Stack:** Markdown documentation, YAML frontmatter, skill discovery system

---

## Task 1: Create Executing Workflows Skill Structure

**Files:**
- Create: `plugin/skills/workflow/executing-workflows/SKILL.md`

**Step 1: Create directory structure**

```bash
mkdir -p plugin/skills/workflow/executing-workflows
```

**Step 2: Create skill file with frontmatter and overview**

Create `plugin/skills/workflow/executing-workflows/SKILL.md`:

```markdown
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

## Related Skills

- **Creating workflows:** `@${CLAUDE_PLUGIN_ROOT}skills/workflow/creating-workflows/SKILL.md`
- **Workflow practice:** `@${CLAUDE_PLUGIN_ROOT}practices/workflow.md`
```

**Step 3: Verify file created**

```bash
ls -la plugin/skills/workflow/executing-workflows/SKILL.md
```

Expected: File exists

**Step 4: Test discovery**

```bash
./plugin/tools/find-skills "executing"
./plugin/tools/find-skills "workflow"
```

Expected: New skill appears in results

**Step 5: Commit**

```bash
git add plugin/skills/workflow/
git commit -m "feat(skills): add executing-workflows skill structure

Create directory and initial frontmatter/overview for workflow
execution skill. Establishes when_to_use criteria and relationships."
```

---

## Task 2: Add Algorithmic Decision Tree to Executing Skill

**Files:**
- Modify: `plugin/skills/workflow/executing-workflows/SKILL.md`

**Step 1: Add decision tree section**

Append to `plugin/skills/workflow/executing-workflows/SKILL.md`:

```markdown
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
```

**Step 2: Verify markdown renders correctly**

```bash
# Preview with markdown viewer
cat plugin/skills/workflow/executing-workflows/SKILL.md
```

Expected: Proper markdown structure, clear decision tree

**Step 3: Test skill discovery with new content**

```bash
./plugin/tools/find-skills "enforcement"
./plugin/tools/find-skills "algorithmic"
```

Expected: Skill appears with relevant search terms

**Step 4: Commit**

```bash
git add plugin/skills/workflow/executing-workflows/SKILL.md
git commit -m "feat(skills): add algorithmic decision tree to executing-workflows

Three-step algorithm: Does workflow exist? Which mode? Handle results.
Includes exit code handling and common workflow examples."
```

---

## Task 3: Add Execution Modes and Examples to Executing Skill

**Files:**
- Modify: `plugin/skills/workflow/executing-workflows/SKILL.md`

**Step 1: Add execution modes section**

Append to `plugin/skills/workflow/executing-workflows/SKILL.md`:

```markdown
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
workflow plugin/practices/git-commit-algorithm.md
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
workflow plugin/practices/git-commit-algorithm.md
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
3. Re-run workflow: `workflow plugin/practices/git-commit-algorithm.md`

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
workflow --dry-run plugin/practices/git-commit-algorithm.md
```

**When to use:**
- Understand workflow before executing
- Verify workflow structure
- Check how many steps involved

**Then execute:**
```bash
workflow plugin/practices/git-commit-algorithm.md
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
- **Workflow practice:** `@${CLAUDE_PLUGIN_ROOT}practices/workflow.md`
- **Workflow tool README:** `@${CLAUDE_PLUGIN_ROOT}tools/workflow/README.md`
```

**Step 2: Verify markdown renders correctly**

```bash
cat plugin/skills/workflow/executing-workflows/SKILL.md | tail -100
```

Expected: Clean formatting, clear scenarios

**Step 3: Test complete skill is discoverable**

```bash
./plugin/tools/find-skills "workflow execution"
./plugin/tools/find-skills "guided mode"
```

Expected: Skill appears with good description

**Step 4: Commit**

```bash
git add plugin/skills/workflow/executing-workflows/SKILL.md
git commit -m "feat(skills): add execution modes and scenarios to executing-workflows

Document enforcement vs guided modes, when to use each, common
scenarios with concrete examples, and prevention of rationalization risks."
```

---

## Task 4: Create Creating Workflows Skill Structure

**Files:**
- Create: `plugin/skills/workflow/creating-workflows/SKILL.md`

**Step 1: Create skill file with frontmatter and overview**

Create `plugin/skills/workflow/creating-workflows/SKILL.md`:

```markdown
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
```

**Step 2: Verify file created**

```bash
ls -la plugin/skills/workflow/creating-workflows/SKILL.md
```

Expected: File exists

**Step 3: Test discovery**

```bash
./plugin/tools/find-skills "creating workflow"
./plugin/tools/find-skills "write workflow"
```

Expected: New skill appears in results

**Step 4: Commit**

```bash
git add plugin/skills/workflow/creating-workflows/SKILL.md
git commit -m "feat(skills): add creating-workflows skill structure

Create directory and initial content for workflow creation skill.
Establishes when to create, design principles, and enforcement vs guided."
```

---

## Task 5: Add Workflow Syntax Guide to Creating Skill

**Files:**
- Modify: `plugin/skills/workflow/creating-workflows/SKILL.md`

**Step 1: Add syntax sections**

Append to `plugin/skills/workflow/creating-workflows/SKILL.md`:

```markdown
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
```

**Step 2: Verify markdown renders correctly**

```bash
cat plugin/skills/workflow/creating-workflows/SKILL.md | tail -100
```

Expected: Code examples render properly

**Step 3: Test skill discovery**

```bash
./plugin/tools/find-skills "syntax"
./plugin/tools/find-skills "prompt"
```

Expected: Skill appears with syntax-related searches

**Step 4: Commit**

```bash
git add plugin/skills/workflow/creating-workflows/SKILL.md
git commit -m "feat(skills): add complete syntax guide to creating-workflows

Document steps, commands, conditionals, prompts with examples.
Show enforcement vs guided behavior for each element."
```

---

## Task 6: Add Examples and Testing to Creating Skill

**Files:**
- Modify: `plugin/skills/workflow/creating-workflows/SKILL.md`

**Step 1: Add examples section**

Append to `plugin/skills/workflow/creating-workflows/SKILL.md`:

```markdown
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
```

**Step 2: Verify markdown renders correctly**

```bash
cat plugin/skills/workflow/creating-workflows/SKILL.md | tail -150
```

Expected: Examples are clear and well-formatted

**Step 3: Test complete skill discovery**

```bash
./plugin/tools/find-skills "creating workflow"
./plugin/tools/find-skills "workflow examples"
```

Expected: Skill appears with rich description

**Step 4: Commit**

```bash
git add plugin/skills/workflow/creating-workflows/SKILL.md
git commit -m "feat(skills): add examples and testing to creating-workflows

Include examples by type, testing strategies, common patterns, and
complete reference to workflow tool documentation."
```

---

## Task 7: Update Skill Discovery System

**Files:**
- Verify: `plugin/tools/find-skills` works with new skills

**Step 1: Test discovery with various search terms**

```bash
# Test executing skill discovery
./plugin/tools/find-skills "execute"
./plugin/tools/find-skills "enforcement"
./plugin/tools/find-skills "guided mode"

# Test creating skill discovery
./plugin/tools/find-skills "create"
./plugin/tools/find-skills "syntax"
./plugin/tools/find-skills "writing workflow"
```

Expected: Both skills appear with appropriate searches

**Step 2: Test related skills references**

```bash
# Should show both skills as related
./plugin/tools/find-skills "workflow"
```

Expected: Both executing-workflows and creating-workflows appear

**Step 3: Verify when_to_use descriptions are clear**

```bash
./plugin/tools/find-skills --local "workflow" | grep -A 2 "when_to_use"
```

Expected: Clear, distinct when_to_use for each skill

**Step 4: No commit needed** (testing only, no code changes)

---

## Task 8: Create Skill Index Documentation

**Files:**
- Create: `plugin/skills/workflow/README.md`

**Step 1: Create workflow skills index**

Create `plugin/skills/workflow/README.md`:

```markdown
# Workflow Skills

Skills for using and creating markdown-based executable workflows.

## Skills

### Executing Workflows

**File:** `executing-workflows/SKILL.md`

**When to use:** When task has existing workflow file, when algorithmic enforcement needed, when following multi-step process

**What it covers:**
- Algorithmic decision tree (Does workflow exist? Which mode? Handle results)
- Enforcement vs guided mode selection
- Command patterns and exit code handling
- Common scenarios and troubleshooting

**Target audience:** All agents (high frequency usage)

### Creating Workflows

**File:** `creating-workflows/SKILL.md`

**When to use:** When documenting multi-step process, when creating algorithmic enforcement workflow, when existing workflow needs modification

**What it covers:**
- When to create workflows vs other documentation
- Design principles (simplicity, executable documentation, enforcement vs guided)
- Complete syntax guide (steps, commands, conditionals, prompts)
- Examples by type and common patterns
- Testing workflows before deployment

**Target audience:** Developers and agents creating workflows (low frequency usage)

## Discovery

```bash
# Find workflow skills
./plugin/tools/find-skills "workflow"

# Find execution guidance
./plugin/tools/find-skills "executing"
./plugin/tools/find-skills "enforcement"

# Find creation guidance
./plugin/tools/find-skills "creating workflow"
./plugin/tools/find-skills "syntax"
```

## Related

- **Workflow practice:** `@${CLAUDE_PLUGIN_ROOT}practices/workflow.md` - Work directory conventions
- **Workflow tool:** `@${CLAUDE_PLUGIN_ROOT}tools/workflow/` - Executor implementation
- **Git commit algorithm:** `@${CLAUDE_PLUGIN_ROOT}practices/git-commit-algorithm.md` - Example enforcement workflow
- **Algorithmic enforcement:** `@${CLAUDE_PLUGIN_ROOT}skills/meta/algorithmic-command-enforcement/SKILL.md` - Why workflows work

## Architecture

These skills follow CipherPowers three-layer architecture:

1. **Skills layer** - Process knowledge (these skills)
2. **Automation layer** - Commands/agents that invoke workflows
3. **Documentation layer** - Practices and standards (workflow.md)

**Separation of concerns:**
- Skills = How to use/create workflows (process)
- Practices = Conventions and standards (what)
- Tool = Implementation (execution)

See `CLAUDE.md` for complete architecture documentation.
```

**Step 2: Verify README renders correctly**

```bash
cat plugin/skills/workflow/README.md
```

Expected: Clear index structure

**Step 3: Test README is discoverable**

```bash
ls -la plugin/skills/workflow/README.md
```

Expected: File exists and is readable

**Step 4: Commit**

```bash
git add plugin/skills/workflow/README.md
git commit -m "docs(skills): add workflow skills index README

Document both executing and creating skills, discovery patterns,
related resources, and architecture context."
```

---

## Task 9: Update CLAUDE.md with Skill References

**Files:**
- Modify: `CLAUDE.md`

**Step 1: Add workflow skills to skills section**

Find the Skills Layer section in `CLAUDE.md` and add workflow skills reference:

```markdown
### 1. Skills Layer (`skills/`)

Reusable process knowledge documented using the superpowers framework. Skills are testable, discoverable guides for techniques and workflows.

**Key principles:**
- Written following TDD: test with subagents before writing
- Include rich `when_to_use` frontmatter for discovery
- Follow superpowers SKILL.md structure
- Can reference upstream superpowers skills

**Organization-specific skills:**

**Workflow:**
- **Executing workflows** (`skills/workflow/executing-workflows/`) - Use workflow executor tool with enforcement or guided modes
- **Creating workflows** (`skills/workflow/creating-workflows/`) - Design and write markdown-based executable workflows

**Documentation:**
- **Maintaining docs** (`skills/documentation/maintaining-docs-after-changes/`) - Two-phase sync process
- **Capturing learning** (`skills/documentation/capturing-learning/`) - Retrospective capture process

**Meta:**
- **Algorithmic enforcement** (`skills/meta/algorithmic-command-enforcement/`) - Why algorithms > imperatives
- **Using skills** (`skills/using-skills/`) - CipherPowers skill discovery

**Testing:**
- **TDD enforcement** (`skills/testing/tdd-enforcement-algorithm/`) - Prevent code before tests

**Collaboration:**
- **Code review** (`skills/conducting-code-review/`) - Complete review workflow
- **Commit workflow** (`skills/commit-workflow/`) - Atomic commits with conventional format
- **Selecting agents** (`skills/selecting-agents/`) - Choose right agent for task
```

**Step 2: Update Using the Discovery Tools section**

Add workflow skills examples:

```markdown
## Using the Discovery Tools

**find-skills**: Discover available skills

```bash
# From repository root or via relative path
./plugin/tools/find-skills "search pattern"

# Examples
./plugin/tools/find-skills "workflow"          # Find workflow skills
./plugin/tools/find-skills "executing"         # Find execution guidance
./plugin/tools/find-skills "code review"       # Find review skills

# With scope flags
./plugin/tools/find-skills --local "pattern"      # cipherpowers only
./plugin/tools/find-skills --upstream "pattern"   # superpowers only
```
```

**Step 3: Verify changes render correctly**

```bash
cat CLAUDE.md | grep -A 30 "Workflow:"
```

Expected: Workflow skills clearly documented

**Step 4: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: add workflow skills to CLAUDE.md architecture

Document executing and creating workflow skills in skills layer,
add discovery examples, integrate with architecture documentation."
```

---

## Task 10: Manual Testing and Validation

**Files:**
- None (testing only)

**Step 1: Test executing-workflows skill discovery**

```bash
./plugin/tools/find-skills "execute"
./plugin/tools/find-skills "enforcement mode"
./plugin/tools/find-skills "guided mode"
```

Expected: executing-workflows skill appears with all searches

**Step 2: Test creating-workflows skill discovery**

```bash
./plugin/tools/find-skills "create"
./plugin/tools/find-skills "workflow syntax"
./plugin/tools/find-skills "write workflow"
```

Expected: creating-workflows skill appears with all searches

**Step 3: Read both skills end-to-end**

```bash
cat plugin/skills/workflow/executing-workflows/SKILL.md
cat plugin/skills/workflow/creating-workflows/SKILL.md
```

**Verify:**
- Clear structure (overview → content → references)
- Good frontmatter (name, description, when_to_use)
- Cross-references work (@ syntax paths correct)
- No typos or formatting issues
- Examples are accurate

**Step 4: Test skill references resolve**

```bash
# Check references exist
ls plugin/practices/workflow.md
ls plugin/tools/workflow/README.md
ls plugin/practices/git-commit-algorithm.md
```

Expected: All referenced files exist

**Step 5: Verify skills follow template**

Compare with `plugin/templates/skill-template.md`:

```bash
diff -u plugin/templates/skill-template.md plugin/skills/workflow/executing-workflows/SKILL.md | head -20
```

Expected: Structure matches template (frontmatter, overview, sections)

**Step 6: No commit needed** (testing only)

---

## Summary

**What we built:**
- Two workflow skills (executing and creating)
- Complete syntax documentation
- Algorithmic decision tree for execution
- Examples by type
- Testing strategies
- Skill discovery integration
- Architecture documentation

**Files created:**
- `plugin/skills/workflow/executing-workflows/SKILL.md` - Execution guidance
- `plugin/skills/workflow/creating-workflows/SKILL.md` - Creation guidance
- `plugin/skills/workflow/README.md` - Index and overview

**Files modified:**
- `CLAUDE.md` - Added workflow skills to architecture

**Discovery:**
```bash
# Find execution guidance
./plugin/tools/find-skills "executing"
./plugin/tools/find-skills "enforcement"

# Find creation guidance
./plugin/tools/find-skills "creating workflow"
./plugin/tools/find-skills "syntax"
```

**How to use:**
1. Agents find skills via find-skills tool
2. Read appropriate skill (executing vs creating)
3. Follow algorithmic decision tree (executing)
4. Use syntax guide and examples (creating)

**Integration:**
- Skills reference workflow.md practice for conventions
- Skills reference workflow tool for implementation
- Skills follow CipherPowers architecture (skills → automation → docs)

**Next steps:**
- Implement --guided flag in workflow tool (separate plan)
- Create workflow files for existing processes
- Test skills with agents under pressure
- Convert imperative instructions to executable workflows
