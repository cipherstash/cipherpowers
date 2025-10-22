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

## Executable Workflow

Run with: `workflow plugin/workflows/git-commit.md`

---

## 1. Check for changes

```bash
mise run check-has-changes
```

- PASS: CONTINUE
- FAIL: STOP nothing to commit

## 2. Verify test coverage

Do ALL new/modified functions have tests?

## 3. Run quality checks

```bash
${CLAUDE_PLUGIN_ROOT}plugin/tools/workflow/run plugin/workflows/test-check-build.md
```

- PASS: CONTINUE
- FAIL: STOP fix failing checks before committing

## 4. Check documentation

Is documentation updated for user-facing changes?

## 5. Verify atomic commit

Do changes serve a single atomic purpose?

## 6. Create commit

```bash
git add -p  # Review changes
git commit  # Use conventional-commits.md format
```

## 7. No changes to commit

```bash
echo "No changes to commit - continue working"
exit 0
```

## 8. Fix issues before committing

Address the failing condition, then return to Step 1

## 9. Split into multiple commits

```bash
git add -p  # Stage specific changes
```

- PASS: GOTO 6
- FAIL: STOP

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

**Q1: I have code changes, all tests pass, checks pass, docs updated, atomic change. What does Step 6 say?**

Answer: Commit is ready - stage files and commit with conventional format

**Q2: Tests are failing but I want to commit WIP. Is this valid?**

Answer: NO. Step 3 → NO leads to Step 8 (fix before commit). "WIP commit" is INVALID condition

**Q3: I'm exhausted and want to save progress. Should I commit failing checks?**

Answer: NO. Use git stash. "Exhaustion" is NOT A VALID CONDITION

**Q4: Changes touch 3 unrelated concerns. What does Step 5 result in?**

Answer: NO → Go to Step 9 (split into multiple commits)

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

- Step 3: Runs test-check-build workflow (tests, checks, build must all pass)
- Step 4: "Is documentation updated?" → YES/NO (not "will users notice?")
- Step 5: "Do changes serve a single atomic purpose?" → YES/NO (not "is it close enough?")

**Evidence:** Based on `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md` pattern showing 0% → 100% compliance improvement in pressure testing.

## Common Failure Modes Prevented

| Rationalization | How Algorithm Prevents |
|-----------------|------------------------|
| "WIP commit to save" | NOT A VALID CONDITION - Use git stash (algorithm doesn't check "need to save") |
| "Will fix tests later" | NOT A VALID CONDITION - Step 3 runs test-check-build (tests must pass NOW) |
| "Time pressure" | NOT A VALID CONDITION - Time not in algorithm, only readiness checks |
| "Manual testing sufficient" | NOT A VALID CONDITION - Step 2 checks automated tests exist |
| "Formatting trivial" | Step 3 includes checks - YES/NO, no importance weighting |
| "Changes too small to break" | NOT A VALID CONDITION - Complexity not in algorithm |

## Project Configuration

**Workflows referenced:**
- `test-check-build.md` - Runs tests, checks, and build (Step 3)

**Commands referenced:**
- `mise run check-has-changes` - Verify there are changes to commit (Step 1)
- `mise run test` - Run all tests (via test-check-build)
- `mise run check` - Run linting, formatting, types (via test-check-build)
- `mise run build` - Verify code builds (via test-check-build)

**Adjust for your project:**
If using different commands (npm, cargo, etc.), update test-check-build.md to match your project's commands.

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
