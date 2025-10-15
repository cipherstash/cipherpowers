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
