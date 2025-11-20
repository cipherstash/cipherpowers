---
name: Conducting Code Review
description: Complete workflow for conducting thorough code reviews with test verification and structured feedback
when_to_use: when conducting code review, when another agent asks you to review code, after being dispatched by requesting-code-review skill
version: 3.0.0
---

# Conducting Code Review

## Overview

Systematic code review process ensuring correctness, security, and maintainability through test verification, practice adherence, and structured feedback.

## Quick Reference

**Before starting:**
1. Read upstream skill: `${CLAUDE_PLUGIN_ROOT}skills/requesting-code-review/SKILL.md`
2. Read project practices: `${CLAUDE_PLUGIN_ROOT}standards/code-review.md`

**Core workflow:**
1. Review most recent commit(s)
2. Run all tests and checks
3. Review against practice standards (all severity levels)
4. Save structured feedback to work directory

## Implementation

### Prerequisites

Read these before conducting review:
- `${CLAUDE_PLUGIN_ROOT}skills/requesting-code-review/SKILL.md` - Understand requester expectations
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

#### 4. Save structured review

**Template location:**
`${CLAUDE_PLUGIN_ROOT}templates/code-review-template.md`

**YOU MUST use this exact structure:**

```markdown
# Code Review - {Date}

## Status: [BLOCKED | APPROVED WITH NON-BLOCKING SUGGESTIONS | APPROVED]

## Test Results
- Status: [PASS/FAIL]
- Details: [test failures or "all passed"]

## Check Results
- Status: [PASS/FAIL]
- Details: [warnings or "clean"]

## Next Steps
[Actions required]

## BLOCKING (Must Fix Before Merge)
[Issues or "None"]

**[Issue title]:**
- Description: [clear description]
- Location: [file:line]
- Action: [specific, actionable suggestion]

## NON-BLOCKING (May Be Deferred)
[Issues or "None"]

**[Issue title]:**
- Description: [clear description]
- Location: [file:line]
- Action: [specific & actionable suggestion]

## Checklist
[Check all applicable items across 6 categories: Security & Correctness, Testing, Architecture, Error Handling, Code Quality, Process]
```

**File naming:** See `${CLAUDE_PLUGIN_ROOT}standards/code-review.md` for `.work` directory location and naming convention (`{YYYY-MM-DD}-review-{N}.md`).

**Do NOT create custom section structures.** Use template exactly. Additional context (verification commands, files changed, commit history) may be added at the end, but core template sections are mandatory.


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
- "Template is too simple, adding sections" → Template structure is mandatory. No custom sections.
- "My format is more thorough" → Thoroughness goes IN the template sections, not around them.

## Related Skills

**Requestion code review:**
- Requesting Code Review: `${CLAUDE_PLUGIN_ROOT}skills/requesting-code-review/SKILL.md`

**When receiving feedback on your review:**
- Code Review Reception: `${CLAUDE_PLUGIN_ROOT}skills/receiving-code-review/SKILL.md`

## Testing This Skill

See `test-scenarios.md` for pressure tests validating this workflow resists rationalization.
