# Convert Commands to Skills Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Refactor command workflow instructions into discoverable skills, making commands thin dispatchers that enable agent-to-agent workflow reuse.

**Architecture:** Extract workflow instructions from commands into skills, update agents to reference skills, maintain separation between universal workflows and project-specific configuration in practices.

**Tech Stack:** Markdown documentation, bash find-skills tool, superpowers skill structure

---

## Task 1: Create code-review skill structure

**Files:**
- Create: `skills/code-review/SKILL.md`
- Reference: `commands/code-review.md` (source)
- Reference: `agents/code-reviewer.md` (context)
- Reference: `docs/practices/code-review.md` (standards)

**Step 1: Create skill directory**

```bash
mkdir -p skills/code-review
```

**Step 2: Write code-review skill with frontmatter**

Create `skills/code-review/SKILL.md`:

```markdown
---
name: Code Review Workflow
description: Complete workflow for conducting thorough code reviews with test verification and structured feedback
when_to_use: when requesting code review, before merging code, when another agent asks for code review
version: 1.0.0
---

# Code Review Workflow

## Overview

Systematic code review process ensuring correctness, security, and maintainability through test verification, practice adherence, and structured feedback.

## Quick Reference

**Before starting:**
1. Read upstream skill: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md`
2. Read project practices: `@docs/practices/code-review.md`

**Core workflow:**
1. Review most recent commit(s)
2. Run all tests and checks
3. Review against practice standards (all severity levels)
4. Save structured feedback to work directory

## Implementation

### Prerequisites

Read these before conducting review:
- `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md` - Understand requester expectations
- `@docs/practices/code-review.md` - Standards, severity levels, project commands

### Step-by-Step Workflow

#### 1. Identify code to review

**Determine scope:**
- Most recent commit: `git log -1 --stat`
- Recent commits on branch: `git log origin/main..HEAD`
- Full diff: `git diff origin/main...HEAD`

#### 2. Run tests and checks (NON-NEGOTIABLE)

**MUST run these commands (from practices/code-review.md):**

```bash
# Run all tests - ALL must pass
mise run test

# Run all checks (linting, formatting, types) - ALL must pass
mise run check
```

**Document results:** Note any failures explicitly in review output.

#### 3. Review code against standards

**Use severity levels from practices/code-review.md:**

- **Level 1: Blockers** - Security vulnerabilities, critical bugs, missing tests, breaking changes
- **Level 2: High Priority** - Architectural violations (SRP, DRY, leaky abstractions), performance issues, poor error handling
- **Level 3: Medium Priority** - Clarity/readability issues, documentation gaps
- **Level 4: Low Priority** - Style preferences, minor optimizations

**Review ALL levels.** Empty sections are GOOD if you actually checked. Missing sections mean you didn't check.

#### 4. Find active work directory

**Get current work location:**

```bash
# If project uses work tracking
mise run review:active
```

If no work tracking, save to root or ask user for location.

#### 5. Save structured review

**File naming convention (from practices/code-review.md):**
- Format: `{YYYY-MM-DD}-review-{N}.md`
- First review today: `2025-10-15-review.md`
- Second review today: `2025-10-15-review-1.md`

**Template structure:**

```markdown
# Code Review - {Date}

## Summary
[1-2 sentences on overall quality and readiness]

## Critical Issues (Level 1 - Blockers)
[Issues that prevent merge - or "None found" if clean]

## High Priority Issues (Level 2)
[Must fix before merge - or "None found" if clean]

## Medium Priority Issues (Level 3)
[Must fix before merge - or "None found" if clean]

## Low Priority Issues (Level 4)
[Must fix or document why technically impossible - or "None found" if clean]

## Positive Observations
[What worked well - specific examples build team culture]

## Test Results
- Tests: [PASS/FAIL with command output if failed]
- Checks: [PASS/FAIL with command output if failed]

## Next Steps
[Clear actions required - "Ready to merge" or "Address items above"]
```

## What NOT to Skip

**NEVER skip:**
- Running tests yourself (even if "already passed")
- Running checks yourself
- Reviewing ALL severity levels (not just critical)
- Saving review file to work directory
- Including positive observations

**Common rationalizations that violate workflow:**
- "Tests passed last time" → Run them anyway
- "Code looks clean" → Check all severity levels anyway
- "Simple change" → Thorough review prevents production bugs
- "Senior developer" → Review objectively regardless of author

## Testing This Skill

See `test-scenarios.md` for pressure tests validating this workflow resists rationalization.
```

**Step 3: Commit skill creation**

```bash
git add skills/code-review/SKILL.md
git commit -m "feat: create code-review skill with complete workflow

Extract workflow from code-review command into discoverable skill.
Enables agent-to-agent code review without command layer.

- Frontmatter for find-skills discovery
- References upstream requesting-code-review skill
- References project practices for standards and commands
- Complete step-by-step workflow
- Rationalization defenses included"
```

---

## Task 2: Create commit-workflow skill structure

**Files:**
- Create: `skills/commit-workflow/SKILL.md`
- Reference: `commands/commit.md` (source)
- Reference: `docs/practices/conventional-commits.md` (standards)
- Reference: `docs/practices/git-guidelines.md` (standards)

**Step 1: Create skill directory**

```bash
mkdir -p skills/commit-workflow
```

**Step 2: Write commit-workflow skill**

Create `skills/commit-workflow/SKILL.md`:

```markdown
---
name: Commit Workflow
description: Systematic commit process with pre-commit checks, atomic commits, and conventional commit messages
when_to_use: when committing code changes, before creating pull requests, when another agent needs to commit work
version: 1.0.0
---

# Commit Workflow

## Overview

Structured commit process ensuring code quality through pre-commit verification, atomic commit composition, and conventional commit message formatting.

## Quick Reference

**Before committing:**
1. Run pre-commit checks (linters, formatters, tests, build)
2. Check staging status
3. Review diff to understand changes

**Commit composition:**
1. Split multiple distinct changes into separate commits
2. Use conventional commit format
3. Follow project git guidelines

## Implementation

### Prerequisites

Read these before committing:
- `@docs/practices/conventional-commits.md` - Commit message format
- `@docs/practices/git-guidelines.md` - Git workflow standards

### Step-by-Step Workflow

#### 1. Run pre-commit checks

**Execute quality checks:**

```bash
# Run linters and formatters (project-specific command)
mise run check

# Run tests if appropriate for changes
mise run test

# Ensure code builds without errors
mise run build  # if project has build step
```

**Fix any failures before proceeding.**

#### 2. Check staging status

**Review what's staged:**

```bash
git status
```

**If 0 files staged:**
- Automatically add all modified and new files: `git add .`
- Or selectively stage: `git add <specific-files>`

#### 3. Review diff

**Understand what's being committed:**

```bash
# See staged changes
git diff --staged

# See all changes (staged + unstaged)
git diff HEAD
```

**Analyze for logical grouping:**
- Are multiple distinct features/fixes present?
- Can this be split into atomic commits?

#### 4. Determine commit strategy

**Single commit:** All changes are logically related (one feature, one fix)

**Multiple commits:** Multiple distinct changes detected:
- Feature A + Bug fix B → Split into 2 commits
- Refactoring + New feature → Split into 2 commits
- Multiple unrelated changes → Split into N commits

**If splitting, stage selectively:**

```bash
# Stage specific files
git add file1.py file2.py
git commit -m "..."

# Stage remaining files
git add file3.py
git commit -m "..."
```

#### 5. Write conventional commit message

**Format (from practices/conventional-commits.md):**

```
<type>(<optional-scope>): <description>

[optional body]

[optional footer]
```

**Common types:**
- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code change that neither fixes bug nor adds feature
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Example messages:**

```bash
git commit -m "feat(auth): add password reset functionality

Implement forgot-password flow with email verification.
Includes rate limiting to prevent abuse."

git commit -m "fix: handle null values in user profile endpoint"

git commit -m "refactor: extract validation logic into separate module

Improves testability and reduces duplication across endpoints."
```

#### 6. Commit changes

**Execute commit:**

```bash
git commit -m "type(scope): description"
```

**Verify commit:**

```bash
git log -1 --stat
```

## What NOT to Skip

**NEVER skip:**
- Pre-commit checks (linters, formatters, tests)
- Reviewing full diff before committing
- Analyzing for atomic commit opportunities
- Following conventional commit format

**Common rationalizations that violate workflow:**
- "Tests already passed" → Run checks anyway
- "Small change" → Review diff and follow format anyway
- "Will fix message later" → Write correct message now
- "Mixing changes is faster" → Atomic commits are worth the time

## Testing This Skill

See `test-scenarios.md` for pressure tests validating this workflow resists shortcuts.
```

**Step 3: Commit skill creation**

```bash
git add skills/commit-workflow/SKILL.md
git commit -m "feat: create commit-workflow skill

Extract commit process from commit command into discoverable skill.
Enables agents to commit work following project standards.

- Frontmatter for find-skills discovery
- References conventional-commits and git-guidelines practices
- Complete pre-commit check workflow
- Atomic commit guidance
- Conventional commit format examples"
```

---

## Task 3: Update code-review command to thin dispatcher

**Files:**
- Modify: `commands/code-review.md`

**Step 1: Replace command content with skill reference**

Edit `commands/code-review.md`:

```markdown
# Code Review

Use the code-review skill for the complete workflow, then invoke the code-reviewer agent.

<instructions>
## Instructions

1. **Follow the code-review skill:**
   - Read: `@skills/code-review/SKILL.md`
   - This skill contains the complete workflow (test verification, practice adherence, structured feedback)

2. **Use the code-reviewer agent:**
   - The agent implements the workflow from the skill
   - Ensures non-negotiable steps with persuasion principles
   - Agent will reference the skill automatically

**Why this structure?**
- Skill = Discoverable workflow (agents can find it)
- Agent = Enforced execution (persuasion principles prevent shortcuts)
- Command = Thin dispatcher (provides context to agent)
</instructions>
```

**Step 2: Commit changes**

```bash
git add commands/code-review.md
git commit -m "refactor: convert code-review command to thin dispatcher

Command now references skills/code-review/SKILL.md for workflow.
Enables agent-to-agent code review discovery.

- Workflow moved to skill (discoverable)
- Command remains as user entry point
- Agent references skill for methodology"
```

---

## Task 4: Update commit command to thin dispatcher

**Files:**
- Modify: `commands/commit.md`

**Step 1: Replace command content with skill reference**

Edit `commands/commit.md`:

```markdown
# Commit

Use the commit-workflow skill for systematic commit process.

<instructions>
## Instructions

1. **Follow the commit-workflow skill:**
   - Read: `@skills/commit-workflow/SKILL.md`
   - This skill contains the complete workflow:
     - Pre-commit checks (linters, formatters, tests)
     - Staging and diff review
     - Atomic commit composition
     - Conventional commit message formatting

2. **The skill references project practices:**
   - `@docs/practices/conventional-commits.md` - Message format
   - `@docs/practices/git-guidelines.md` - Git workflow standards

**Why this structure?**
- Skill = Discoverable workflow (agents can find it with find-skills)
- Practices = Project-specific standards (conventional commit rules)
- Command = Thin dispatcher (user entry point)
</instructions>
```

**Step 2: Commit changes**

```bash
git add commands/commit.md
git commit -m "refactor: convert commit command to thin dispatcher

Command now references skills/commit-workflow/SKILL.md.
Enables agents to discover commit workflow independently.

- Workflow moved to skill (discoverable)
- Command remains as user entry point
- Skill references practices for standards"
```

---

## Task 5: Update doc-review command to thin dispatcher

**Files:**
- Modify: `commands/doc-review.md`

**Step 1: Simplify to reference existing skill**

Edit `commands/doc-review.md`:

```markdown
# Documentation Review

Review and update project documentation to ensure it stays synchronized with recent code changes.

<instructions>
## Instructions

1. **Follow the documentation maintenance skill:**
   - Read: `@skills/documentation/maintaining-docs-after-changes/SKILL.md`
   - This skill contains the complete two-phase workflow:
     - Phase 1: Analysis (review changes, check docs, identify gaps)
     - Phase 2: Update (modify content, restructure, verify completeness)

2. **The skill references project documentation standards:**
   - `@docs/practices/documentation.md` - Formatting and completeness standards

**Key Principle:**
Maintaining existing documentation after code changes is NOT "proactively creating docs" - it's keeping current docs accurate. If code changed, docs MUST update.

**Why this structure?**
- Skill = Universal workflow (can be upstreamed to superpowers)
- Practices = Project-specific standards (your docs format)
- Command = Thin dispatcher (adds project context)
</instructions>
```

**Step 2: Commit changes**

```bash
git add commands/doc-review.md
git commit -m "refactor: simplify doc-review command to thin dispatcher

Command now clearly delegates to existing skill.
Removed duplicate workflow instructions.

- References skills/documentation/maintaining-docs-after-changes
- Command provides project context only
- Skill contains complete workflow"
```

---

## Task 6: Update summarise command to thin dispatcher

**Files:**
- Modify: `commands/summarise.md`

**Step 1: Simplify to reference existing skill**

Edit `commands/summarise.md`:

```markdown
# Summarise

Create a comprehensive retrospective summary of completed work, capturing decisions, lessons learned, and insights for continuous improvement.

<instructions>
## Instructions

1. **Follow the learning capture skill:**
   - Read: `@skills/documentation/capturing-learning/SKILL.md`
   - This skill contains the complete retrospective workflow:
     - Step 1: Review the work (identify location, review changes)
     - Step 2: Capture learning (decisions, approaches, issues, time)
     - Step 3: Save and link (to work directory or CLAUDE.md)
     - Step 4: Merge if existing summary

2. **The skill references project documentation standards:**
   - `@docs/practices/documentation.md` - Summary format and standards

**Key Principle:**
Exhaustion after completion is when capture matters most. The harder the work, the more valuable the lessons.

**Why this structure?**
- Skill = Universal workflow (learning capture process)
- Practices = Project-specific standards (summary format)
- Command = Thin dispatcher (integrates with work tracking)
</instructions>
```

**Step 2: Commit changes**

```bash
git add commands/summarise.md
git commit -m "refactor: simplify summarise command to thin dispatcher

Command now clearly delegates to existing skill.
Removed duplicate workflow instructions.

- References skills/documentation/capturing-learning
- Command provides project context only
- Skill contains complete workflow"
```

---

## Task 7: Update code-reviewer agent to reference code-review skill

**Files:**
- Modify: `agents/code-reviewer.md:20-22`

**Step 1: Add skill reference to context section**

Edit `agents/code-reviewer.md`, in the `<context>` section after line 22:

```markdown
    YOU MUST ALWAYS READ these skills:
    - Code Review Workflow (complete methodology): @skills/code-review/SKILL.md
    - Requesting Code Review (understand what requester expects)
    - Code Review Reception (understand how feedback will be received)
```

**Step 2: Commit changes**

```bash
git add agents/code-reviewer.md
git commit -m "refactor: update code-reviewer agent to reference code-review skill

Agent now explicitly references skills/code-review/SKILL.md.
Ensures complete workflow is followed even when agent called directly.

- Added skill reference to context section
- Agent can find workflow via skill if command bypassed
- Maintains agent's persuasion principles for enforcement"
```

---

## Task 8: Test skill discovery

**Files:**
- Test: `./tools/find-skills`

**Step 1: Test finding code-review skill**

```bash
./tools/find-skills "code review"
```

Expected output should include:
```
Use skills/code-review/SKILL.md when requesting code review, before merging code, when another agent asks for code review
```

**Step 2: Test finding commit skill**

```bash
./tools/find-skills "commit"
```

Expected output should include:
```
Use skills/commit-workflow/SKILL.md when committing code changes, before creating pull requests, when another agent needs to commit work
```

**Step 3: Verify both local and upstream skills appear**

```bash
./tools/find-skills "code"
```

Should show both cipherpowers skills (code-review, code-review-reception) and superpowers skills.

**Step 4: Test scoped search**

```bash
./tools/find-skills --local "review"
```

Should show only cipherpowers skills (code-review, doc-review related).

**Step 5: Document test results**

If all tests pass, proceed. If discovery fails, debug find-skills tool configuration.

---

## Task 9: Update CLAUDE.md examples

**Files:**
- Modify: `CLAUDE.md:56-75` (Code Review Workflow example section)

**Step 1: Update code review example**

Edit `CLAUDE.md`, find the "Example: Code Review Workflow" section:

```markdown
**Example: Code Review Workflow**
- `skills/code-review/SKILL.md` = Complete workflow (test verification, structured feedback, work directory save)
- `docs/practices/code-review.md` = Standards (severity levels) + Project Config (commands, file conventions)
- `agents/code-reviewer.md` = Workflow enforcement with persuasion principles (non-negotiable steps, rationalization defenses)
- `commands/code-review.md` = Thin dispatcher (sets context, references skill)
- Skills: References upstream "Requesting Code Review" and "Code Review Reception" skills

All components work together without duplication:
- Update severity standards in practices → agent uses new standards automatically
- Change project commands (mise run test) → skill/agent reference practice for current command
- Update workflow in skill → all agents using skill get updated workflow
- Commands remain simple dispatchers → workflow discovery via skills
```

**Step 2: Add commit workflow example**

Add new example section after code review example:

```markdown
**Example: Commit Workflow**
- `skills/commit-workflow/SKILL.md` = Complete workflow (pre-commit checks, atomic commits, conventional format)
- `docs/practices/conventional-commits.md` = Commit message format standards
- `docs/practices/git-guidelines.md` = Git workflow standards
- `commands/commit.md` = Thin dispatcher (references skill)

Skills enable discovery:
- Any agent can run `find-skills "commit"` and discover workflow
- No need to hardcode commit knowledge into every agent
- Update workflow in skill → all agents benefit
```

**Step 3: Commit documentation updates**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md with refactored workflow examples

Update examples to reflect new skill-based architecture.
Show how skills enable agent-to-agent workflow discovery.

- Updated code-review example to show skill layer
- Added commit-workflow example
- Emphasized discovery via find-skills tool"
```

---

## Summary

**What we built:**
- 2 new skills: `code-review`, `commit-workflow`
- 4 updated commands: All now thin dispatchers
- 1 updated agent: References code-review skill explicitly
- Updated documentation: CLAUDE.md reflects new structure

**Architecture benefits:**
- ✅ Commands are thin (as intended)
- ✅ Workflows discoverable via find-skills
- ✅ Agent-to-agent reuse enabled
- ✅ Skills testable with subagents
- ✅ Single source of truth (DRY)
- ✅ Practices remain project-specific

**Next steps:**
1. Consider creating test-scenarios.md for pressure testing new skills
2. Monitor agent behavior to verify skills are discovered correctly
3. Consider upstreaming code-review skill to superpowers if proven valuable
