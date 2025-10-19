---
name: Git Commit Readiness Algorithm
description: Algorithmic decision tree for determining when code is ready to commit
when_to_use: when you have code changes and are considering making a commit, before running git add or git commit
related_practices: git-guidelines.md, conventional-commits.md, testing.md
version: 1.0.0
---

# Git Commit Readiness Algorithm

## Overview

Algorithmic enforcement of commit readiness prevents premature commits with failing tests, incomplete work, or missing documentation. Uses boolean conditions (not subjective judgment) to determine if commit should proceed.

**Core principle:** Commits must pass deterministic quality checks. No exceptions for time pressure, exhaustion, or "will fix later" rationalizations.

## Decision Algorithm: When to Commit

```
Step 1: Check: Have you made code changes?
        → YES: Go to Step 2
        → NO: Go to Step 8 (nothing to commit)

Step 2: Check: Do ALL new/modified functions have tests?
        → YES: Go to Step 3
        → NO: Go to Step 9 (incomplete - write tests first)

Step 3: Check: Do ALL tests pass?
        → YES: Go to Step 4
        → NO: Go to Step 9 (failing tests - fix before commit)

Step 4: Check: Does `mise run check` pass (linting, formatting, types)?
        → YES: Go to Step 5
        → NO: Go to Step 9 (checks failing - fix before commit)

Step 5: Check: Is documentation updated for user-facing changes?
        → YES: Go to Step 6
        → NO: Go to Step 9 (missing docs - update before commit)

Step 6: Check: Do changes serve a single atomic purpose?
        → YES: Go to Step 7
        → NO: Go to Step 10 (split into multiple commits)

Step 7: Commit is ready
        Stage files: git add [files]
        Commit: git commit (use conventional-commits.md format)
        STOP

Step 8: No changes to commit (continue working)
        STOP

Step 9: Commit NOT ready - complete work first
        Fix identified issue (tests, checks, docs)
        Return to Step 1
        STOP

Step 10: Split changes into multiple commits
        Stage related files only: git add [subset]
        Commit atomic change: git commit
        Return to Step 1 for remaining changes
        STOP
```

## Executable Workflow

Run with: `workflow plugin/practices/git-commit-algorithm.md`

---

# Step 1: Check for changes

Fail: STOP (nothing to commit)

```bash quiet
mise run check-has-changes
```

# Step 2: Verify test coverage

**Prompt:** Do ALL new/modified functions have tests?

# Step 3: Run tests

Fail: STOP (fix tests before committing)

```bash
mise run test
```

# Step 4: Run checks

Fail: STOP (run mise check to see failures)

```bash
mise run check
```

# Step 5: Check documentation

**Prompt:** Is documentation updated for user-facing changes?

# Step 6: Verify atomic commit

**Prompt:** Do changes serve a single atomic purpose?

# Step 7: Commit changes

Fail: STOP (commit failed - check message format)

```bash
git commit
```

## INVALID Conditions for Committing Early

These rationalizations are **NOT VALID ALGORITHM CONDITIONS:**

- "Is there time pressure?" → NOT A VALID CONDITION
- "Will I fix tests/checks later?" → NOT A VALID CONDITION
- "Are changes too small to break?" → NOT A VALID CONDITION
- "Did I manually test it?" → NOT A VALID CONDITION
- "Is this a WIP commit?" → NOT A VALID CONDITION (use stash instead)
- "Am I exhausted and want to save progress?" → NOT A VALID CONDITION (use stash)

**All of these mean:** Run the algorithm. Follow what it says.

## Self-Test

**Q1: I have code changes, all tests pass, checks pass, docs updated, atomic change. What does Step 7 say?**

Answer: Commit is ready - stage files and commit with conventional format

**Q2: Tests are failing but I want to commit WIP. Is this valid?**

Answer: NO. Step 3 → NO leads to Step 9 (fix before commit). "WIP commit" is INVALID condition

**Q3: I'm exhausted and want to save progress. Should I commit failing checks?**

Answer: NO. Use git stash. "Exhaustion" is NOT A VALID CONDITION

**Q4: Changes touch 3 unrelated concerns. What does Step 6 result in?**

Answer: NO → Go to Step 10 (split into multiple commits)

## Integration with Existing Practices

**This algorithm provides:** WHEN to commit (readiness checks)

**See also:**
- `git-guidelines.md` - Branch naming, commit structure, splitting criteria
- `conventional-commits.md` - Commit message format (HOW to write commit)
- `testing.md` - Testing standards (what "tests pass" means)

**Workflow:**
1. **This algorithm** → Determines if commit is ready
2. **conventional-commits.md** → Formats the commit message
3. **git-guidelines.md** → Guides commit splitting and branching

## Why Algorithmic Format

**Previous guideline (imperative):**
> "Verify before committing: Ensure code is linted, builds correctly, and documentation is updated"

**Problem:** Agents interpreted "verify" as suggestion under time pressure.

**Agent rationalizations:**
- "I'll fix linting after" (time pressure)
- "Formatting is trivial" (minimizing importance)
- "WIP commit to save work" (exhaustion)
- "Tests can be added later" (deferring quality)

**Solution:** Algorithm Steps 3, 4, 5 have binary YES/NO checks. No interpretation possible.

- Step 3: "Do ALL tests pass?" → YES/NO (not "are they important?")
- Step 4: "Does mise run check pass?" → YES/NO (not "is formatting critical?")
- Step 5: "Is documentation updated?" → YES/NO (not "will users notice?")

**Evidence:** Based on `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md` pattern showing 0% → 100% compliance improvement in pressure testing.

## Common Failure Modes Prevented

| Rationalization | How Algorithm Prevents |
|-----------------|------------------------|
| "WIP commit to save" | NOT A VALID CONDITION - Use git stash (algorithm doesn't check "need to save") |
| "Will fix tests later" | NOT A VALID CONDITION - Step 3 checks tests NOW (not "will I fix later") |
| "Time pressure" | NOT A VALID CONDITION - Time not in algorithm, only readiness checks |
| "Manual testing sufficient" | NOT A VALID CONDITION - Step 2 checks automated tests exist |
| "Formatting trivial" | Step 4 checks "mise run check pass?" - YES/NO, no importance weighting |
| "Changes too small to break" | NOT A VALID CONDITION - Complexity not in algorithm |

## Project Configuration

**Commands referenced:**
- `mise run test` - Run all tests (must pass for Step 3)
- `mise run check` - Run linting, formatting, types (must pass for Step 4)

**Adjust for your project:**
If using different commands (npm, cargo, etc.), update Step 4 condition to match your project's check command.

## Testing

Pressure test scenarios: `docs/tests/git-commit-pressure-scenarios.md`

**Scenarios test algorithm under:**
- WIP commit + time pressure ("need to save work")
- Exhaustion + manual testing ("tested manually, formatting can wait")
- Multi-concern + deadline ("one commit faster than splitting")

**Method:** RED (baseline without algorithm) → GREEN (with algorithm) → measure compliance

**Success criteria:** 80%+ compliance improvement

## Related Skills

**Algorithmic pattern:** `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md`

**Git workflow:** See `git-guidelines.md` for branching, commit splitting details

**Commit format:** See `conventional-commits.md` for message structure
