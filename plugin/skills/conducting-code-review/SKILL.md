---
name: Conducting Code Review
description: Complete workflow for conducting thorough code reviews with test verification and structured feedback
when_to_use: when you have uncommitted changes OR completed work OR about to merge, to determine if code review is required. Also when conducting code review, when another agent asks you to review code, after being dispatched by requesting-code-review skill
version: 3.0.0
---

# Conducting Code Review

## Overview

Systematic code review process ensuring correctness, security, and maintainability through test verification, practice adherence, and structured feedback.

## Decision Algorithm: When Code Review is Required

Use this algorithm to determine if code review is required before merge/PR/completion:

# Step 1: Check for commits

**Prompt:** Have you made commits to a feature branch?

Fail: Go to Step 6

# Step 2: Check for existing review

**Prompt:** Have these commits been reviewed?

Pass: Go to Step 6

# Step 3: Check for completion

**Prompt:** Are you about to merge, create PR, or mark work complete?

Fail: Go to Step 6

# Step 4: Request code review

Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md`

**Prompt:** STOP - do not merge/PR/complete until review done

# Step 5: [UNREACHABLE - if you reach here, you violated Step 4]

# Step 6: Continue

**Prompt:** No commits OR already reviewed OR still working - continue

### INVALID Conditions for Skipping Review

These rationalizations are **NOT VALID ALGORITHM CONDITIONS:**

- "Are changes too small?" → NOT A VALID CONDITION
- "Am I a senior developer?" → NOT A VALID CONDITION
- "Is there time pressure?" → NOT A VALID CONDITION
- "Did I review it myself?" → NOT A VALID CONDITION
- "Is this a hotfix?" → NOT A VALID CONDITION (still needs review)
- "Are tests passing?" → NOT A VALID CONDITION (tests ≠ review)

**All of these mean:** Run the algorithm. Follow what it says.

### Self-Test: Review Trigger

**Q1: I committed 3 changes, ready to merge. What does Step 4 say?**

Answer: Request code review, STOP, do not merge until reviewed

**Q2: "These are just documentation changes, too small for review" - is this valid?**

Answer: NO. Listed under INVALID conditions

**Q3: Tests are all passing. Do I still need review?**

Answer: YES. "Are tests passing?" is NOT A VALID CONDITION

## Quick Reference

**Before starting:**
1. Read upstream skill: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md`
2. Read project practices: `${CLAUDE_PLUGIN_ROOT}practices/code-review.md`

**Core workflow:**
1. Review most recent commit(s)
2. Run all tests and checks
3. Review against practice standards (all severity levels)
4. Save structured feedback to work directory

## Implementation

### Prerequisites

Read these before conducting review:
- `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md` - Understand requester expectations
- `${CLAUDE_PLUGIN_ROOT}practices/code-review.md` - Standards, severity levels, project commands

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

**Read standards from practices:**

```bash
# Standards live in practices, not in this skill
${CLAUDE_PLUGIN_ROOT}practices/code-review.md
```

**Review ALL four severity levels:**
1. Level 1: Blockers (from practices)
2. Level 2: High Priority (from practices)
3. Level 3: Medium Priority (from practices)
4. Level 4: Low Priority (from practices)

**Empty sections are GOOD if you actually checked.** Missing sections mean you didn't check.

#### 4. Find active work directory

**Get current work location:**

```bash
# If project uses work tracking
mise run review:active
```

If no work tracking, save to root or ask user for location.

#### 5. Save structured review

**File naming and template:**

See `${CLAUDE_PLUGIN_ROOT}practices/code-review.md` for:
- File naming convention (`{YYYY-MM-DD}-review-{N}.md`)
- Complete template structure with all sections
- Examples of review file organization

**Use template exactly as specified in practices.**

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

## Related Skills

**Before using this skill:**
- Requesting Code Review: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md`

**When receiving feedback on your review:**
- Code Review Reception: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/receiving-code-review/SKILL.md`

## Testing This Skill

See `test-scenarios.md` for pressure tests validating this workflow resists rationalization.
