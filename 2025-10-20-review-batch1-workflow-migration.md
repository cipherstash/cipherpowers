# Code Review - 2025-10-20

## Summary

Batch 1 (Tasks 1-3) attempted to migrate workflows from arrow syntax to Pass/Fail labels, creating wrapper scripts and converting two algorithm files. While the conceptual approach is sound and the wrapper scripts are well-designed, **both migrated workflow files have critical structural errors that prevent execution**. The workflow executor requires strictly sequential step numbering, but both files contain non-sequential steps or multiple algorithm sections.

## BLOCKING (Must Fix Before Merge)

### 1. Git Commit Algorithm Has Non-Sequential Steps

**File:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/practices/git-commit-algorithm.md`

**Issue:** The workflow has Steps 1-7 for the main flow, then jumps to Steps 8-10 for alternative paths. The workflow executor enforces sequential numbering (Step 1, 2, 3... with no gaps) but this structure has:
- Main flow: Steps 1-7
- Alternative paths: Steps 8, 9, 10

**Evidence:**
```bash
$ workflow --list plugin/practices/git-commit-algorithm.md
Error: Step numbers must be sequential. Expected Step 10, found Step 5
```

**Why blocking:** The workflow cannot be executed. This is the core deliverable of the migration.

**Root cause:** The original arrow syntax allowed non-sequential references ("Go to Step 9", "Go to Step 10") because it was descriptive documentation. The new Pass/Fail syntax is executable code that requires strict sequential numbering.

**Fix required:** Restructure to use strictly sequential steps. Alternative paths must either:
1. Be integrated into sequential flow using conditionals (e.g., Step 7 has `Fail: Go to Step 8` where Step 8 is the next sequential step)
2. Use a different workflow structure (e.g., separate sections are not part of the executable workflow)

### 2. TDD Algorithm Has Duplicate Step Numbers (Two Algorithms in One File)

**File:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md`

**Issue:** The file contains TWO separate algorithms:
1. "Decision Algorithm" - Steps 1-6
2. "Recovery Algorithm" - Steps 1-5 (duplicate numbering!)

Both start with Step 1, violating sequential numbering requirement.

**Evidence:**
```bash
$ workflow --list plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md
Error: Step numbers must be sequential. Expected Step 7, found Step 1
```

Step numbering found:
```
Line 35: # Step 1: Check for implementation code
Line 41: # Step 2: Check for prototype exception
Line 47: # Step 3: Check for failing test
Line 54: # Step 4: Write failing test first
Line 58: # Step 5: Verify test fails
Line 67: # Step 6: Proceed with implementation
Line 73: # Step 1: Check for implementation code  <-- DUPLICATE!
Line 79: # Step 2: Check for tests              <-- DUPLICATE!
```

**Why blocking:** Neither algorithm can be executed. The parser encounters Step 1 twice and fails.

**Root cause:** The migration converted two conceptually separate algorithms without recognizing that the workflow executor treats the entire file as a single sequential workflow.

**Fix required:** Either:
1. Split into two separate workflow files (Decision and Recovery)
2. Combine into single sequential algorithm with conditionals routing to appropriate logic
3. Make one algorithm non-executable (e.g., use different header format like "## Recovery Algorithm" instead of "# Step X")

### 3. Tests Failing in Workflow Executor

**File:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/tools/workflow/`

**Issue:** Two integration tests failed:
```
test_debug_mode_shows_evaluation ... FAILED
test_debug_mode_shows_fail_evaluation ... FAILED
```

**Error:** `Failed to execute workflow: Os { code: 2, kind: NotFound, message: "No such file or directory" }`

**Why blocking:** Failing tests indicate the tool itself may have issues, or the test fixtures are broken. Cannot merge with failing tests.

**Fix required:** Investigate test failures:
1. Check if test fixtures exist and are accessible
2. Verify debug mode implementation is correct
3. Ensure tests pass before merging migration changes

### 4. Wrapper Script Has Incorrect Regex in check-tests-exist

**File:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/mise.toml` (lines 24-27)

**Issue:** The wrapper script checks for test files using naive path transformation:
```bash
test_file="${file/src/tests}"
test_file="${test_file%.rs}_test.rs"
```

This logic assumes:
- All source files are in `src/` directory
- Test files are in `tests/` directory with `_test.rs` suffix

**Why blocking:** This will fail for:
- Rust projects using inline tests (tests in same file as code)
- Projects with different test file organization
- Files not in `src/` (e.g., `lib/`, nested modules)
- Other languages with different conventions

**Current behavior:** Will report "Missing tests" for legitimate test setups that don't match this exact pattern.

**Fix required:** Either:
1. Document that this is a heuristic and may have false positives/negatives
2. Make the wrapper script configurable per project
3. Use language-specific test detection (e.g., check for `#[cfg(test)]` in Rust)
4. Simplify to just checking if tests exist in repo, not per-file mapping

### 5. Missing Explanation of Prompt vs Command in Migrated Workflows

**Files:**
- `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/practices/git-commit-algorithm.md`
- `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md`

**Issue:** The migrated workflows use `**Prompt:**` syntax without explaining what this means or how it differs from code blocks. For example:

```markdown
# Step 2: Verify test coverage

**Prompt:** Do ALL new/modified functions have tests?
```

vs

```markdown
# Step 3: Run tests

Fail: STOP (fix tests before committing)

```bash
mise run test
```
```

**Why blocking:** Users (and the workflow executor) need to understand:
- What is a "prompt"? (Human decision point? LLM evaluation? Manual check?)
- How should they respond to prompts?
- What happens after a prompt? (Always continue? Wait for user input?)
- Why some checks are prompts and others are bash commands?

**Current state:** The syntax is used throughout but never defined. The workflow executor may not handle prompts correctly (needs verification).

**Fix required:**
1. Add explanation section in each file defining what prompts are
2. Document prompt behavior in workflow executor README
3. Verify workflow executor correctly handles prompt steps
4. Consider if prompts should have explicit Pass/Fail paths

## NON-BLOCKING (Can Be Deferred)

### 1. Wrapper Scripts Not Used Consistently in Git Commit Algorithm

**File:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/practices/git-commit-algorithm.md`

**Observation:** The migration plan (Task 1) created four wrapper scripts:
- `check-has-changes`
- `check-tests-exist`
- `check-docs-updated`
- `check-atomic-commit`

But only `check-has-changes` is actually used in the migrated workflow (Step 1). Steps 2, 5, and 6 use `**Prompt:**` instead of calling the wrapper scripts.

**Expected (from plan):**
```markdown
# Step 2: Verify test coverage
Fail: Go to Step 9
```bash
mise run check-tests-exist
```
```

**Actual:**
```markdown
# Step 2: Verify test coverage
**Prompt:** Do ALL new/modified functions have tests?
```

**Impact:** Minor - prompts may be intentional design choice, but inconsistent with the implementation plan which explicitly created wrapper scripts for these checks.

**Suggestion:** Either:
1. Use the wrapper scripts as planned (automated checks)
2. Remove unused wrapper scripts from mise.toml
3. Document why prompts are preferred over automated checks

### 2. Step 7 in Git Commit Algorithm Missing Fail Handler

**File:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/practices/git-commit-algorithm.md` (lines 59-65)

**Observation:**
```markdown
# Step 7: Create commit

```bash
git add -p  # Review changes
git commit  # Use conventional-commits.md format
```
```

This step has no conditional labels. What happens if:
- User cancels `git add -p`?
- `git commit` fails (empty message, pre-commit hook fails)?

**Original version had:**
```markdown
Step 7: Commit changes
        Fail: STOP (commit failed - check message format)
```

**Impact:** Minor - the simplified version may be intentional (assuming user handles failures), but loses error handling guidance.

**Suggestion:** Consider adding `Fail: STOP (fix and retry)` to handle commit failures explicitly.

### 3. Inconsistent Comment Style in mise.toml

**File:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/mise.toml`

**Observation:** Comments are inconsistent:
- Line 8-9: Inline comments after exit statements
- Line 26: No comment for path transformation
- Line 45: Escaped dot in regex `\\.md$` (correct) but inconsistent with earlier patterns

**Impact:** Minor readability issue, doesn't affect functionality.

**Suggestion:** Standardize comment style (e.g., always on separate line, or always inline).

### 4. Git Commit Algorithm Lost Detailed Failure Messages

**File:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/practices/git-commit-algorithm.md`

**Observation:** The original version had detailed failure labels:
- Old Step 3: `→ NO: Go to Step 9 (failing tests - fix before commit)`
- New Step 3: `Fail: STOP (fix tests before committing)`

While the new version is cleaner, it lost some context about what "Step 9" represented (generic fix step).

**Original structure:**
```
Step 9: Commit NOT ready - complete work first
        Fix identified issue (tests, checks, docs)
        Return to Step 1
```

**New structure:**
```
# Step 9: Fix issues before committing
**Prompt:** Address the failing condition, then return to Step 1
```

**Impact:** Minor - the new version is more concise but slightly less explicit about the iterative fix-and-retry pattern.

**Suggestion:** Consider adding example of the fix-retry cycle in the documentation section.

### 5. TDD Algorithm Step 5 Has Inverted Pass/Fail Logic

**File:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md` (lines 58-65)

**Observation:**
```markdown
# Step 5: Verify test fails

Pass: Go to Step 6
Fail: Go to Step 2
```

This is confusing because:
- Step name is "Verify test **fails**"
- But `Pass:` (test passes) goes to Step 6
- Shouldn't a failing test be the desired outcome here?

**Original version:**
```
Step 5: Run test to verify it fails
        → FAILS: Go to Step 2
        → PASSES: Go to Step 6 (test is invalid)
```

The original was clearer: we WANT the test to fail (hence `→ FAILS: Go to Step 2` to start implementing).

**Impact:** Minor confusion risk. The logic is technically correct (if `mise run test` passes = test doesn't fail = invalid test), but the wording is counterintuitive.

**Suggestion:** Clarify with comment or rename:
```markdown
# Step 5: Verify test fails correctly

Pass: Go to Step 6  # Test passed without code - invalid test!
Fail: Go to Step 2  # Test fails as expected - proceed to implement
```

### 6. Missing --quiet Flag Explanation for Step 1 in Git Commit Algorithm

**File:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/practices/git-commit-algorithm.md` (line 27)

**Observation:**
```markdown
```bash quiet
mise run check-has-changes
```
```

The `quiet` flag is used but not explained anywhere in the file.

**Impact:** Minor - users may not understand why some steps are quiet and others aren't.

**Suggestion:** Add comment explaining `quiet` suppresses output, or document in workflow syntax reference.

## Positive Observations

### 1. Wrapper Scripts Are Well-Designed

The mise wrapper scripts in `mise.toml` demonstrate good practices:
- Clear exit codes (0 = success/has changes, 1 = failure/no changes)
- Descriptive output messages
- Appropriate use of git commands (`--porcelain`, `--cached`, `--name-only`)
- Sensible heuristics for checks (15 files max, 3 subsystems max)

Example:
```bash
if [[ -n $(git status --porcelain) ]]; then
  exit 0  # Has changes - proceed
else
  exit 1  # No changes - stop
fi
```

Clean, clear, and follows shell best practices.

### 2. Migration Preserves Algorithmic Structure

Both migrated files maintain the algorithmic decision tree structure that's core to the enforcement pattern. The "INVALID Conditions" sections were preserved, which is critical for preventing rationalization.

Example from TDD algorithm:
```markdown
## INVALID Conditions (NOT in algorithm, do NOT use)

- "Is code too simple to test?" → NOT A VALID CONDITION
- "Is there time pressure?" → NOT A VALID CONDITION
```

This is exactly the right approach - the migration didn't lose the persuasion principles.

### 3. Simplified Syntax Reduces Verbosity

Where the migration works correctly, the new syntax is much cleaner:

**Old:**
```markdown
Step 3: Check: Do ALL tests pass?
        → YES: Go to Step 4
        → NO: Go to Step 9 (failing tests - fix before commit)
```

**New:**
```markdown
# Step 3: Run tests

Fail: STOP (fix tests before committing)

```bash
mise run test
```
```

The implicit "Pass: Continue" default reduces noise significantly. This is a good design choice.

### 4. Consistent Use of conventional-commits.md Reference

Both files correctly reference the conventional commits practice:
```markdown
git commit  # Use conventional-commits.md format
```

This maintains the layered architecture (algorithms determine WHEN, practices define HOW).

### 5. Comments in Bash Commands Improve Clarity

The migrated workflows include helpful inline comments:
```bash
git add -p  # Review changes
git commit  # Use conventional-commits.md format
```

This guides users without being verbose.

## Test Results

- **Tests:** FAILED (2 failures in workflow executor integration tests)
- **Checks:** Not run (no `mise run check` task configured)

**Test failures:**
```
test_debug_mode_shows_evaluation ... FAILED
test_debug_mode_shows_fail_evaluation ... FAILED
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

**Other tests:** 47 tests passed (23 lib tests + 24 main tests)

## Next Steps

### Must Fix (BLOCKING)

1. **Fix git-commit-algorithm.md sequential numbering**
   - Restructure to use strictly sequential steps (1, 2, 3, 4...)
   - Use conditionals (Pass/Fail + GoTo) to route to alternative paths
   - OR: Make Steps 8-10 non-executable documentation

2. **Fix TDD algorithm duplicate step numbers**
   - Split Decision and Recovery into separate workflow files, OR
   - Combine into single sequential workflow, OR
   - Make one algorithm non-executable (documentation only)

3. **Fix failing workflow executor tests**
   - Investigate "No such file or directory" error
   - Verify debug mode test fixtures exist
   - Ensure all tests pass before merge

4. **Fix or document check-tests-exist wrapper script limitations**
   - Add configuration for test file patterns per project
   - Document heuristic limitations
   - OR: Remove script if not reliable enough

5. **Document Prompt syntax**
   - Explain what prompts are in workflow syntax reference
   - Clarify how workflow executor handles prompts
   - Add examples of prompt usage

### Should Consider (NON-BLOCKING)

1. **Use wrapper scripts consistently** - Either use all 4 wrapper scripts as planned, or remove unused ones

2. **Add error handling to Step 7** - Handle `git commit` failures explicitly

3. **Clarify TDD Step 5 naming** - Make it clear we WANT the test to fail at this stage

4. **Document quiet flag** - Explain when and why to use `quiet` in bash blocks

## Files Reviewed

- `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/mise.toml` (created)
- `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/practices/git-commit-algorithm.md` (modified)
- `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md` (modified)
- `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/tools/workflow/` (test failures)

## Review Metadata

- **Reviewer:** code-reviewer agent
- **Date:** 2025-10-20
- **Batch:** Batch 1 (Tasks 1-3) of workflow migration plan
- **Commits reviewed:** Most recent commit (86131dc) + staged changes
- **Review skill used:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/skills/conducting-code-review/SKILL.md`
- **Standards applied:** `/Users/tobyhede/src/cipherpowers/.worktrees/workflow-migration/plugin/practices/code-review.md`
