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
1. Check staging status
2. Review diff to understand changes
3. Analyze for atomic commit opportunities

**Commit composition:**
1. Split multiple distinct changes into separate commits
2. Use conventional commit format
3. Follow project git guidelines

**Note:** Quality gates (PostToolUse, SubagentStop hooks) automatically enforce pre-commit checks, so code quality is already verified.

## Implementation

### Prerequisites

Read these before committing:
- `${CLAUDE_PLUGIN_ROOT}standards/conventional-commits.md` - Commit message format
- `${CLAUDE_PLUGIN_ROOT}standards/git-guidelines.md` - Git workflow standards

### Step-by-Step Workflow

**Note:** Pre-commit quality checks (linters, tests, build) are enforced automatically by quality gates (PostToolUse, SubagentStop hooks). By the time you're committing, gates have already verified code quality.

#### 1. Check staging status

**Review what's staged:**

```bash
git status
```

**If 0 files staged:**
- Automatically add all modified and new files: `git add .`
- Or selectively stage: `git add <specific-files>`

#### 2. Review diff

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

#### 3. Determine commit strategy

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

#### 4. Write conventional commit message

**Format (from standards/conventional-commits.md):**

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

#### 5. Commit changes

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
- Reviewing full diff before committing
- Analyzing for atomic commit opportunities
- Following conventional commit format
- Verifying commit was created correctly

**Common rationalizations that violate workflow:**
- "Small change" → Review diff and follow format anyway
- "Will fix message later" → Write correct message now
- "Mixing changes is faster" → Atomic commits are worth the time
- "Don't need to verify" → Always check commit with git log

**Note:** Pre-commit quality checks are enforced by quality gates automatically - no need to run them manually at commit time.

## Testing This Skill

See `test-scenarios.md` for pressure tests validating this workflow resists shortcuts.
