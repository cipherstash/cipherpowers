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

## 1. Check for commits

Have you made commits to a feature branch?

- PASS: CONTINUE
- FAIL: GOTO 6

## 2. Check for existing review

Have these commits been reviewed?

- PASS: CONTINUE
- FAIL: GOTO 4

## 3. Check for change request

If the commits have been reviewed, were changes requested due to BLOCKING issues?

- PASS: CONTINUE
- FAIL: GOTO 5

## 4. Check for completion

Are you about to merge, create PR, or mark work complete?

- PASS: CONTINUE
- FAIL: GOTO 6

## 5. Request code review

Use `${CLAUDE_PLUGIN_ROOT}plugin/skills/requesting-code-review/SKILL.md`

STOP - Work is not complete without code review.

## 6. Continue

No commits OR already reviewed OR still working - continue


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
1. Read upstream skill: `${CLAUDE_PLUGIN_ROOT}plugin/skills/requesting-code-review/SKILL.md`
2. Read project practices: `${CLAUDE_PLUGIN_ROOT}standards/code-review.md`

**Core workflow:**
1. Review most recent commit(s)
2. Run all tests and checks
3. Review against practice standards (all severity levels)
4. Save structured feedback to work directory

## Implementation

### Prerequisites

Read these before conducting review:
- `${CLAUDE_PLUGIN_ROOT}plugin/skills/requesting-code-review/SKILL.md` - Understand requester expectations
- `${CLAUDE_PLUGIN_ROOT}standards/code-review.md` - Standards, severity levels, project commands

### Step-by-Step Workflow

#### 1. Identify code to review

**Determine scope:**
- Most recent commit: `git log -1 --stat`
- Recent commits on branch: `git log origin/main..HEAD`
- Full diff: `git diff origin/main...HEAD`

#### 2. Run tests and checks (NON-NEGOTIABLE)

**MUST run:**
- Run project check command
- Run project test command
- Run project build command

**Document results:** Note any failures explicitly in review output.

#### 3. Review code against standards

**Read standards from practices:**

```bash
# Standards live in practices, not in this skill
${CLAUDE_PLUGIN_ROOT}standards/code-review.md
```

**Review ALL severity levels:**
1. BLOCKING (Must Fix Before Merge) - from practices
2. NON-BLOCKING (Can Be Deferred) - from practices

**Empty sections are GOOD if you actually checked.** Missing sections mean you didn't check.

#### 4. Find active work directory

**Get current work location:**

Projects may provide a command to find the active work directory (check CLAUDE.md frontmatter for custom commands).

If no work tracking, save to root or ask user for location.

#### 5. Save structured review

**File naming and template:**

See `${CLAUDE_PLUGIN_ROOT}standards/code-review.md` for:
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
- Requesting Code Review: `${CLAUDE_PLUGIN_ROOT}plugin/skills/requesting-code-review/SKILL.md`

**When receiving feedback on your review:**
- Code Review Reception: `${CLAUDE_PLUGIN_ROOT}plugin/skills/receiving-code-review/SKILL.md`

## Testing This Skill

See `test-scenarios.md` for pressure tests validating this workflow resists rationalization.
