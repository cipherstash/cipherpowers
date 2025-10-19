# Workflow Migration to Pass/Fail Syntax Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use executing-plans to implement this plan task-by-task.

**Goal:** Migrate all workflows from deprecated arrow syntax (`→`) to simplified Pass/Fail labels.

**Architecture:** Systematic conversion of 13 files using new syntax conventions, creating wrapper scripts for complex conditionals, leveraging implicit defaults.

**Tech Stack:** Markdown workflows, Bash wrapper scripts, mise tasks, workflow executor tool

---

## Task 1: Create Wrapper Scripts

**Files:**
- Modify: `mise.toml`

**Step 1: Add check-has-changes task**

Add to mise.toml:
```toml
[tasks.check-has-changes]
description = "Check if there are changes to commit (exit 0 if changes, 1 if empty)"
run = """
if [[ -n $(git status --porcelain) ]]; then
  exit 0  # Has changes - proceed
else
  exit 1  # No changes - stop
fi
"""
```

**Step 2: Add check-tests-exist task**

Add to mise.toml:
```toml
[tasks.check-tests-exist]
description = "Check if all new/modified functions have tests"
run = """
# Get list of modified files
files=$(git diff --cached --name-only)

# Check for new functions without tests
for file in $files; do
  if [[ "$file" == *.rs ]] || [[ "$file" == *.py ]] || [[ "$file" == *.js ]]; then
    # Simple heuristic: if file has functions but no test markers
    if grep -q 'fn \|def \|function' "$file" 2>/dev/null; then
      # Check if corresponding test file exists
      test_file="${file/src/tests}"
      test_file="${test_file%.rs}_test.rs"
      if [[ ! -f "$test_file" ]]; then
        echo "Missing tests for $file"
        exit 1
      fi
    fi
  fi
done
exit 0
"""
```

**Step 3: Add check-docs-updated task**

Add to mise.toml:
```toml
[tasks.check-docs-updated]
description = "Check if documentation is updated for user-facing changes"
run = """
# Check if any src files modified
src_modified=$(git diff --cached --name-only | grep -E '(src/|lib/)' | wc -l)

# Check if any docs modified
docs_modified=$(git diff --cached --name-only | grep -E '(README|docs/|\.md$)' | wc -l)

# If src modified but no docs, might need docs
if [[ $src_modified -gt 0 ]] && [[ $docs_modified -eq 0 ]]; then
  echo "Warning: Source files modified but no documentation updated"
  echo "If this change is user-facing, please update docs"
  exit 1  # Fail by default, user can override
fi
exit 0
"""
```

**Step 4: Add check-atomic-commit task**

Add to mise.toml:
```toml
[tasks.check-atomic-commit]
description = "Check if changes serve a single atomic purpose"
run = """
# Check file count
files=$(git diff --cached --name-only | wc -l)
if [[ $files -gt 15 ]]; then
  echo "Large commit with $files files - consider splitting"
  exit 1
fi

# Check different subsystems modified
subsystems=$(git diff --cached --name-only | cut -d'/' -f1 | sort -u | wc -l)
if [[ $subsystems -gt 3 ]]; then
  echo "Commit touches $subsystems different subsystems - consider splitting"
  exit 1
fi

exit 0
"""
```

**Step 5: Test wrapper scripts**

```bash
# Verify mise.toml is valid
mise tasks list | grep check-

# Test each script
mise run check-has-changes
echo "Exit code: $?"
```

**Step 6: Commit wrapper scripts**

```bash
git add mise.toml
git commit -m "feat: add wrapper scripts for workflow conditionals

- check-has-changes: verify uncommitted changes exist
- check-tests-exist: ensure new functions have tests
- check-docs-updated: check docs updated for user-facing changes
- check-atomic-commit: verify commit is atomic"
```

---

## Task 2: Migrate Git Commit Algorithm

**Files:**
- Modify: `plugin/practices/git-commit-algorithm.md`

**Step 1: Read current file to understand structure**

```bash
cat plugin/practices/git-commit-algorithm.md | head -60
```

**Step 2: Convert Step 1 (check for changes)**

Replace:
```markdown
Step 1: Check: Have you made code changes?
        → YES: Go to Step 2
        → NO: Go to Step 8 (nothing to commit)
```

With:
```markdown
# Step 1: Check for code changes

Fail: Go to Step 8

```bash
mise run check-has-changes
```
```

**Step 3: Convert Step 2 (check tests exist)**

Replace:
```markdown
Step 2: Check: Do ALL new/modified functions have tests?
        → YES: Go to Step 3
        → NO: Go to Step 9 (incomplete - write tests first)
```

With:
```markdown
# Step 2: Verify test coverage

Fail: Go to Step 9

```bash
mise run check-tests-exist
```
```

**Step 4: Convert Step 3 (run tests)**

Replace:
```markdown
Step 3: Check: Do ALL tests pass?
        → YES: Go to Step 4
        → NO: Go to Step 9 (failing tests - fix before commit)
```

With:
```markdown
# Step 3: Run tests

Fail: Go to Step 9

```bash
mise run test
```
```

**Step 5: Convert Step 4 (run checks)**

Replace:
```markdown
Step 4: Check: Does `mise run check` pass (linting, formatting, types)?
        → YES: Go to Step 5
        → NO: Go to Step 9 (checks failing - fix before commit)
```

With:
```markdown
# Step 4: Run code quality checks

Fail: Go to Step 9

```bash
mise run check
```
```

**Step 6: Convert Step 5 (check docs)**

Replace:
```markdown
Step 5: Check: Is documentation updated for user-facing changes?
        → YES: Go to Step 6
        → NO: Go to Step 9 (missing docs - update before commit)
```

With:
```markdown
# Step 5: Verify documentation

Fail: Go to Step 9

```bash
mise run check-docs-updated
```
```

**Step 7: Convert Step 6 (check atomic)**

Replace:
```markdown
Step 6: Check: Do changes serve a single atomic purpose?
        → YES: Go to Step 7
        → NO: Go to Step 10 (split into multiple commits)
```

With:
```markdown
# Step 6: Verify atomic commit

Fail: Go to Step 10

```bash
mise run check-atomic-commit
```
```

**Step 8: Convert Step 7 (commit ready)**

Replace:
```markdown
Step 7: Commit is ready
        Stage files: git add [files]
        Commit: git commit (use conventional-commits.md format)
        STOP
```

With:
```markdown
# Step 7: Create commit

```bash
git add -p  # Review changes
git commit  # Use conventional-commits.md format
```
```

**Step 9: Convert Step 8 (no changes)**

Replace:
```markdown
Step 8: No changes to commit (continue working)
        STOP
```

With:
```markdown
# Step 8: No changes to commit

```bash
echo "No changes to commit - continue working"
exit 0
```
```

**Step 10: Convert Steps 9-10 (failure paths)**

Replace:
```markdown
Step 9: Fix issues before committing
        Address the failing condition
        Return to Step 1

Step 10: Split into multiple commits
         Stage subset: git add [specific files]
         Return to Step 7
```

With:
```markdown
# Step 9: Fix issues before committing

**Prompt:** Address the failing condition, then return to Step 1

# Step 10: Split into multiple commits

```bash
git add -p  # Stage specific changes
```

Pass: Go to Step 7
```

**Step 11: Test the converted workflow**

```bash
workflow --dry-run plugin/practices/git-commit-algorithm.md
```

**Step 12: Commit the migration**

```bash
git add plugin/practices/git-commit-algorithm.md
git commit -m "refactor: migrate git-commit-algorithm to Pass/Fail syntax

- Replace arrow conditionals with Pass/Fail labels
- Use mise wrapper scripts for complex checks
- Leverage implicit defaults (Pass continues, Fail stops)
- Maintain same logical flow with cleaner syntax"
```

---

## Task 3: Migrate TDD Enforcement Algorithm

**Files:**
- Modify: `plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md`

**Step 1: Find the algorithm section**

```bash
grep -n "Step.*:" plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md | head -10
```

**Step 2: Read the current algorithm**

```bash
sed -n '/## Algorithm/,/## /p' plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md
```

**Step 3: Convert Step 1 (check failing test)**

Replace:
```markdown
Step 1: Check: Does a failing test exist for the next piece of functionality?
        → YES: Go to Step 2
        → NO: Go to Step 4 (write test first)
```

With:
```markdown
# Step 1: Check for failing test

**Prompt:** Does a failing test exist for the next piece of functionality?

Fail: Go to Step 4
```

**Step 4: Convert Step 2 (write code)**

Replace:
```markdown
Step 2: Write the minimal code to make the test pass
        → Go to Step 3
```

With:
```markdown
# Step 2: Write minimal code

**Prompt:** Write the minimal code to make the test pass
```

**Step 5: Convert Step 3 (run test)**

Replace:
```markdown
Step 3: Run the test
        → PASS: Return to Step 1 (for next functionality)
        → FAIL: Fix code, repeat Step 3
```

With:
```markdown
# Step 3: Run the test

Pass: Go to Step 1
Fail: STOP (fix code and re-run)

```bash
mise run test
```
```

**Step 6: Convert Step 4 (write test)**

Replace:
```markdown
Step 4: Write a failing test for the functionality
        → Go to Step 5
```

With:
```markdown
# Step 4: Write failing test

**Prompt:** Write a failing test for the functionality
```

**Step 7: Convert Step 5 (verify test fails)**

Replace:
```markdown
Step 5: Run test to verify it fails
        → FAILS: Go to Step 2
        → PASSES: Go to Step 6 (test is invalid)
```

With:
```markdown
# Step 5: Verify test fails

Pass: Go to Step 6
Fail: Go to Step 2

```bash
mise run test
```
```

**Step 8: Convert Step 6 (invalid test)**

Replace:
```markdown
Step 6: Test passes without implementation (invalid test)
        Fix test to properly fail
        Return to Step 5
```

With:
```markdown
# Step 6: Fix invalid test

**Prompt:** Test passes without implementation - fix test to properly fail, then return to Step 5
```

**Step 9: Test the converted workflow**

```bash
workflow --dry-run plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md
```

**Step 10: Commit the migration**

```bash
git add plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md
git commit -m "refactor: migrate TDD enforcement to Pass/Fail syntax

- Convert arrow conditionals to Pass/Fail labels
- Use prompts for human decisions
- Simplify flow with implicit defaults"
```

---

## Task 4: Migrate Code Review Skill Workflow

**Files:**
- Modify: `plugin/skills/conducting-code-review/SKILL.md`

**Step 1: Find workflow sections with arrows**

```bash
grep -n "→" plugin/skills/conducting-code-review/SKILL.md | head -10
```

**Step 2: Read Section 1 algorithm**

```bash
sed -n '/## Section 1:/,/## Section 2:/p' plugin/skills/conducting-code-review/SKILL.md | head -40
```

**Step 3: Convert decision algorithm to Pass/Fail**

The section uses arrow syntax for a decision tree. Convert each step following the pattern from previous tasks.

Replace arrow-based decisions with Pass/Fail labels and prompts where appropriate.

**Step 4: Check for other workflow sections**

```bash
grep -A5 -B5 "Step.*:" plugin/skills/conducting-code-review/SKILL.md
```

**Step 5: Update each workflow section**

For each section with arrow syntax, apply the conversion pattern:
- Remove `→ Exit 0: Continue` (implicit default)
- Replace `→ Exit ≠ 0: STOP` with `Fail: STOP`
- Convert output checks to wrapper scripts
- Use prompts for human decisions

**Step 6: Test key workflows**

Extract and test any executable workflows:
```bash
# Extract workflow section to temp file
# Test with workflow tool
workflow --dry-run /tmp/test-workflow.md
```

**Step 7: Commit the migration**

```bash
git add plugin/skills/conducting-code-review/SKILL.md
git commit -m "refactor: migrate code review skill to Pass/Fail syntax

- Remove verbose arrow conditionals
- Leverage implicit defaults
- Use prompts for review decisions"
```

---

## Task 5: Update Documentation Examples

**Files:**
- Modify: `plugin/tools/workflow/README.md`
- Modify: `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md`
- Modify: Other files with example workflows

**Step 1: Update README examples**

Find sections showing old syntax:
```bash
grep -n "→" plugin/tools/workflow/README.md
```

For each example showing deprecated syntax, either:
- Update to show new Pass/Fail syntax
- Add comment indicating it's showing deprecated format

**Step 2: Update algorithmic enforcement skill**

```bash
grep -n "→" plugin/skills/meta/algorithmic-command-enforcement/SKILL.md
```

Update examples to use new syntax while preserving the algorithmic structure.

**Step 3: Find other documentation with arrows**

```bash
grep -l "→" plugin/**/*.md | grep -v MIGRATION | grep -v DEPRECATION
```

**Step 4: Update each file**

For each file:
- If it's an example, update to new syntax
- If it's a historical reference, add note about deprecation
- If it's an active workflow, migrate fully

**Step 5: Verify all active workflows migrated**

```bash
# Find remaining arrow syntax excluding migration docs
grep -l "Step.*:.*\n.*→" plugin/**/*.md | \
  grep -v MIGRATION | \
  grep -v DEPRECATION | \
  grep -v review-batch
```

**Step 6: Final test of key workflows**

```bash
# Test git-commit algorithm
workflow --dry-run plugin/practices/git-commit-algorithm.md

# Test other executable workflows
for file in $(find plugin -name "*.md" -exec grep -l "Step.*:" {} \;); do
  echo "Testing: $file"
  workflow --list "$file" 2>/dev/null && echo "Valid workflow"
done
```

**Step 7: Commit documentation updates**

```bash
git add -A plugin/
git commit -m "refactor: update documentation to use Pass/Fail syntax

- Update examples in README and skills
- Preserve historical references in MIGRATION/DEPRECATION
- Ensure all active workflows use new syntax"
```

---

## Task 6: Create Migration Summary

**Files:**
- Create: `docs/learning/2025-10-20-workflow-syntax-migration.md`

**Step 1: Write retrospective**

Create retrospective documenting:
- Migration scope (13 files)
- Key patterns discovered
- Wrapper script approach
- Benefits of implicit defaults
- Testing approach

**Step 2: Update CLAUDE.md**

Add entry to Learning section about the migration.

**Step 3: Final commit**

```bash
git add docs/learning/2025-10-20-workflow-syntax-migration.md CLAUDE.md
git commit -m "docs: add workflow migration retrospective

Document learnings from migrating 13 workflows to Pass/Fail syntax"
```

---

## Verification Checklist

After completing all tasks:

1. **No arrow syntax in active workflows:**
   ```bash
   grep -l "→" plugin/**/*.md | grep -v MIGRATION | grep -v DEPRECATION
   ```

2. **Wrapper scripts work:**
   ```bash
   mise tasks list | grep check-
   mise run check-has-changes && echo "Has changes"
   ```

3. **Key workflows executable:**
   ```bash
   workflow --dry-run plugin/practices/git-commit-algorithm.md
   ```

4. **Tests still pass:**
   ```bash
   cd plugin/tools/workflow && cargo test
   ```

---

## Notes for Engineer

**Context you need:**
- Old syntax: `→ Exit 0: Continue` and `→ Exit ≠ 0: STOP`
- New syntax: `Pass: Continue` and `Fail: STOP`
- Defaults are implicit: Pass→Continue, Fail→STOP
- Most conditionals can be removed entirely

**Testing each migration:**
- Use `workflow --dry-run <file>` to verify syntax
- Use `workflow --list <file>` to see steps
- Test wrapper scripts individually with `mise run <task>`

**Common patterns:**
1. Remove redundant `Pass: Continue` - it's the default
2. Replace `→ Exit ≠ 0: STOP` with just `Fail: STOP`
3. Convert output checks to wrapper scripts that return exit codes
4. Use `**Prompt:**` for human decision points

**If you get stuck:**
- Check `plugin/tools/workflow/README.md` for syntax reference
- Look at `plugin/tools/workflow/examples/` for patterns
- Run `workflow --help` for options