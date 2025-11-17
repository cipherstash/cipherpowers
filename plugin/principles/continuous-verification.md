---
name: Continuous Verification
description: Run verification checks during development, not just before merge, to catch issues incrementally and reduce debugging time
when_to_use: during active development after implementing each function/module, before marking tasks complete
applies_to: all projects
related_practices: testing.md, development.md
version: 1.0.0
---

# Continuous Verification

> **NOTE:** This principle exists because agents frequently skip checks and tests under pressure. This should be enforced through an algorithmic workflow (similar to `git-commit.md` workflow) rather than relying on agents following instructions. Consider creating an enforced skill/workflow to prevent rationalization.

## Core principle

Run verification checks **during development**, not just before merge.

**Why:** Agents under time pressure skip checks/tests, leading to batch debugging sessions that waste 5-10x more time than incremental verification would have taken.

## Watch mode (recommended)

For active development, run the project's quality checks (see CLAUDE.md for the specific command).

This typically runs all verification steps:
- Fast compilation check
- Unit tests
- Linting (warnings as errors)
- Formatting verification

**Language-specific examples:**

Projects document their specific commands in CLAUDE.md. Common patterns:

```bash
# Rust projects
cargo check && cargo test --lib && cargo clippy -- -D warnings && cargo fmt --check

# Python projects
mypy . && pytest && ruff check . && black --check .

# TypeScript projects
tsc --noEmit && jest && eslint . && prettier --check .
```

## Why during development

### Issues caught incrementally (easier to fix)

**Evidence from retrospectives:**
- Fix 1 error immediately: ~5 minutes
- Fix 16 errors in batch: 30-45 minutes
- No documentation rework needed (claims match reality)
- Faster overall development (incremental wins)

**Example from BattleSpace type-safety retrospective:**
> "30 minutes fixing 16 clippy errors in batch would have been 5 minutes if fixed incrementally"

### Prevents "works on my machine" syndrome

**Scenario without continuous verification:**
1. Agent implements 5 functions
2. Agent claims "implementation complete"
3. User runs tests → 12 failures
4. 30 minutes debugging what went wrong
5. Documentation claims features work (they don't)

**Scenario with continuous verification:**
1. Agent implements function 1
2. Agent runs quality checks (see CLAUDE.md) → 2 failures
3. Agent fixes immediately (2 minutes)
4. Repeat for functions 2-5
5. All tests pass, documentation accurate

## When to run

**Mandatory checkpoints:**
- ✅ After implementing each function/module
- ✅ Before marking task as complete
- ✅ Before documenting results
- ✅ Before committing code

**Optional (but recommended):**
- After significant refactoring
- After updating dependencies
- Before asking for code review

## Integration with workflow

Verification is part of **definition-of-done**, not cleanup.

**Task completion checklist:**
```markdown
Task complete when:
- [ ] Implementation done
- [ ] Tests written
- [ ] Verification passed (run quality checks per CLAUDE.md)
- [ ] Documentation updated
```

**NOT complete if:**
- ❌ "Implementation done but tests skipped for now"
- ❌ "Code works but clippy warnings to fix later"
- ❌ "Feature complete but formatting to clean up"

## Manual commands

If you need to run checks individually:

**Rust:**
```bash
cargo check                              # Fast compilation check
cargo test --lib                         # Unit tests only
cargo clippy --all-targets -- -D warnings # Linting
cargo fmt --check                        # Formatting
```

**Python:**
```bash
mypy .                   # Type checking
pytest                   # Tests
ruff check .             # Linting
black --check .          # Formatting
```

**TypeScript:**
```bash
tsc --noEmit            # Type checking
jest                    # Tests
eslint .                # Linting
prettier --check .      # Formatting
```

## Common rationalizations (red flags)

If you're thinking ANY of these, you're violating continuous verification:

| Rationalization | Reality |
|-----------------|---------|
| "I'll run tests when all features done" | 12 test failures to debug in batch vs 2 per function |
| "Tests passed locally yesterday" | Code changed since then, run again |
| "Just a small change, doesn't need checks" | Small changes cause production bugs too |
| "Running checks wastes time" | 5 min incremental < 30-45 min batch debugging |
| "Will fix clippy warnings before commit" | You'll forget, or run out of time |
| "Code compiles, good enough for now" | Clippy/tests find bugs that compiler doesn't |

**All of these mean:** STOP. Run quality checks (see CLAUDE.md for command). NOW.

## Evidence from retrospectives

**BattleSpace Type-Safety Retrospective:**
- 16 clippy errors discovered in batch
- 30 minutes to fix all errors
- Would have been ~5 minutes if fixed incrementally
- Documentation claimed features worked (they didn't until after fix)

**Key insight:** Batch debugging takes 5-10x longer than incremental verification.

## Project configuration

### Commands

**Quality checks command:**

Projects document their quality checks command in CLAUDE.md (see `Development Commands` section).

- What it runs: ALL verification steps with project-specific configuration
- Requirement: MUST pass before marking task complete
- Never skip this command

**Why project-defined commands instead of direct tool commands:**
- Project-specific configuration (flags, options)
- Consistent workflow across different tools/languages
- One command, all checks
- Easier to enforce algorithmically

## Future: Enforcement via workflow

**Current state:** This is a "principle" that agents frequently violate under pressure.

**Future state:** Should be enforced via algorithmic workflow similar to `git-commit.md`:

```markdown
# Task Completion Workflow

## 1. Check implementation done

Is implementation complete?

- PASS: CONTINUE
- FAIL: STOP finish implementation first

## 2. Run verification

Run the project's quality checks (see CLAUDE.md for specific command)

- PASS: CONTINUE
- FAIL: STOP fix failures first

## 3. Update documentation

Is documentation updated?

- PASS: CONTINUE
- FAIL: STOP update docs first

## 4. Mark task complete

Task is now complete.
```

**Rationale:** Algorithms achieve 100% compliance, imperatives achieve 0-33% compliance under pressure.

## References

**Related Skills:**
- `${CLAUDE_PLUGIN_ROOT}plugin/skills/verification-before-completion/SKILL.md` - Final verification checklist
- `${CLAUDE_PLUGIN_ROOT}/workflows/test-check-build.md` - Algorithmic verification workflow

**Related Practices:**
- `${CLAUDE_PLUGIN_ROOT}/principles/testing.md` - Testing principles
- `${CLAUDE_PLUGIN_ROOT}/principles/development.md` - Development principles

**Retrospectives:**
- BattleSpace type-safety retrospective: 30 min batch vs 5 min incremental
- Multiple instances of agents skipping checks leading to failed PRs
