# Algorithmic Workflow Conversion Implementation Plan

> **For Claude:** Use `/execute` to implement this plan task-by-task with automatic agent selection and batch-level code review.

**Goal:** Convert three critical discipline-enforcing workflows (TDD, code review trigger, git commit) from imperative to algorithmic format, achieving 0% → 100% compliance under pressure.

**Architecture:** Following `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md`, create decision algorithms with five mechanisms: boolean conditions, invalid conditions list, deterministic STOP, self-test, and unreachable steps. Each workflow gets a dedicated skill/practice file with algorithmic decision tree.

**Tech Stack:** Markdown documentation, bash scripts for testing, subagents for pressure testing validation.

**Evidence:** Algorithmic format achieved 100% compliance vs 0-33% imperative compliance in `/execute` command pressure tests. Same magnitude improvement expected for TDD, code review, and git workflows.

---

## Task 1: Create TDD Enforcement Algorithm

**Files:**
- Create: `plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md`
- Reference: `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md` (template)
- Reference: `${SUPERPOWERS_SKILLS_ROOT}/skills/testing/test-driven-development/SKILL.md` (current TDD skill)

**Step 1: Design decision algorithm structure**

Map current TDD "Iron Law" to algorithmic format:

**Main Algorithm:**
```markdown
## Decision Algorithm: When to Write Tests First

Step 1: Check: Are you about to write implementation code?
        → YES: Go to Step 2
        → NO: Go to Step 6

Step 2: Check: Does throwaway prototype exception apply (user approved)?
        → YES: Go to Step 6
        → NO: Go to Step 3

Step 3: Check: Does a failing test exist for this code?
        → YES: Go to Step 6 (proceed with implementation)
        → NO: Go to Step 4

Step 4: STOP writing implementation code
        Write failing test first
        Do not proceed to Step 5

Step 5: [UNREACHABLE - if you reach here, you violated Step 4]

Step 6: Proceed (test exists OR not writing code)
```

**Recovery Algorithm:**
```markdown
## Recovery Algorithm: Already Wrote Code Without Tests?

Step 1: Check: Have you written ANY implementation code?
        → YES: Go to Step 2
        → NO: Go to Step 5

Step 2: Check: Does that code have tests that failed first?
        → YES: Go to Step 5
        → NO: Go to Step 3

Step 3: Delete the untested code
        Execute: git reset --hard OR rm [files]
        Do not keep as "reference"
        STOP

Step 4: [UNREACHABLE - Step 3 has STOP]

Step 5: Continue (tests exist OR no code written)
```

**Invalid Conditions:**
```markdown
## INVALID conditions (NOT in algorithm, do NOT use):
- "Is code too simple to test?" → NOT A VALID CONDITION
- "Is there time pressure?" → NOT A VALID CONDITION
- "Did I manually test it?" → NOT A VALID CONDITION
- "Will I add tests after?" → NOT A VALID CONDITION
- "Is deleting X hours wasteful?" → NOT A VALID CONDITION
- "Am I being pragmatic?" → NOT A VALID CONDITION
- "Can I keep as reference?" → NOT A VALID CONDITION
```

**Step 2: Write complete SKILL.md**

Structure following algorithmic-command-enforcement template:
- YAML frontmatter with rich when_to_use
- Overview explaining algorithmic approach
- When to Use section
- Main Decision Algorithm
- Recovery Algorithm
- INVALID conditions list
- Self-Test section (4 questions with answers)
- Five mechanisms explanation
- Real-world impact (reference /execute results)
- Cross-reference to upstream TDD skill for RED-GREEN-REFACTOR details

**Step 3: Add self-test section**

```markdown
## Self-Test

Q1: You're about to write `function calculateTotal()`. Step 3 asks?
    Answer: Does a failing test exist? If NO → STOP, write test first

Q2: I wrote 100 lines without tests. Recovery Step 3 says?
    Answer: Delete the untested code

Q3: "This code is too simple to need tests" - valid algorithm condition?
    Answer: NO. Listed under INVALID conditions

Q4: Can I keep untested code as "reference" while writing tests?
    Answer: NO. Recovery Step 3 says delete, not keep
```

**Step 4: Add cross-references**

```markdown
## Related Skills

**For complete RED-GREEN-REFACTOR methodology:**
- `${SUPERPOWERS_SKILLS_ROOT}/skills/testing/test-driven-development/SKILL.md`

**For understanding algorithmic format:**
- `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md`

**This skill provides:** WHEN to use TDD (decision algorithm)
**Upstream skill provides:** HOW to use TDD (implementation details)
```

**Step 5: Commit**

```bash
git add plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md
git commit -m "feat: add TDD enforcement algorithm skill

Create algorithmic decision tree for when to write tests first.
Prevents agents from bypassing TDD under time pressure, sunk cost,
or 'too simple' rationalizations.

- Main algorithm: binary checks before writing code
- Recovery algorithm: delete untested code mandate
- Invalid conditions list: common rationalizations
- Self-test: 4 questions verifying comprehension

Complements upstream TDD skill (HOW) with deterministic WHEN logic.
Based on algorithmic-command-enforcement pattern (0% → 100% compliance)."
```

---

## Task 2: Create Code Review Trigger Algorithm

**Files:**
- Modify: `plugin/skills/conducting-code-review/SKILL.md`
- Reference: `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md`
- Reference: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md`

**Step 1: Design decision algorithm for review trigger**

Add new section at top of conducting-code-review skill:

**Main Algorithm:**
```markdown
## Decision Algorithm: When Code Review is Required

Step 1: Check: Have you made commits to a feature branch?
        → YES: Go to Step 2
        → NO: Go to Step 6 (no review needed yet)

Step 2: Check: Have these commits been reviewed?
        → YES: Go to Step 6 (already reviewed)
        → NO: Go to Step 3

Step 3: Check: Are you about to merge, create PR, or mark work complete?
        → YES: Go to Step 4
        → NO: Go to Step 6 (continue working)

Step 4: Request code review
        Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md`
        STOP - do not merge/PR/complete until review done

Step 5: [UNREACHABLE - if you reach here, you violated Step 4]

Step 6: Continue (no commits OR already reviewed OR still working)
```

**Invalid Conditions:**
```markdown
## INVALID conditions for skipping review (NOT in algorithm):
- "Are changes too small?" → NOT A VALID CONDITION
- "Am I a senior developer?" → NOT A VALID CONDITION
- "Is there time pressure?" → NOT A VALID CONDITION
- "Did I review it myself?" → NOT A VALID CONDITION
- "Is this a hotfix?" → NOT A VALID CONDITION (still needs review)
- "Are tests passing?" → NOT A VALID CONDITION (tests ≠ review)
```

**Step 2: Add self-test section**

```markdown
## Self-Test: Review Trigger

Q1: I committed 3 changes, ready to merge. Step 4 says?
    Answer: Request code review, STOP, do not merge until reviewed

Q2: "These are just documentation changes, too small for review" - valid?
    Answer: NO. Listed under INVALID conditions

Q3: Tests are all passing. Do I still need review?
    Answer: YES. "Are tests passing?" is NOT A VALID CONDITION
```

**Step 3: Insert algorithm before existing workflow**

Place Decision Algorithm section at very top of skill (after Overview), before "Quick Reference" section. This ensures agents check WHEN before reading HOW.

**Step 4: Update when_to_use frontmatter**

```yaml
when_to_use: when you have uncommitted changes OR completed work OR about to merge, to determine if code review is required. Also when conducting code review, when another agent asks you to review code, after being dispatched by requesting-code-review skill
```

**Step 5: Commit**

```bash
git add plugin/skills/conducting-code-review/SKILL.md
git commit -m "feat: add code review trigger algorithm to conducting-code-review skill

Add algorithmic decision tree determining when review is required.
Prevents agents from skipping review for 'small changes' or under
time pressure.

- Trigger algorithm: checks commits + review status before merge
- Invalid conditions: 'too small', 'senior dev', 'tests passing'
- Self-test: 3 questions on trigger logic

Preserves existing review workflow (HOW), adds deterministic WHEN check.
Based on algorithmic-command-enforcement pattern."
```

---

## Task 3: Create Git Commit Readiness Algorithm

**Files:**
- Create: `plugin/practices/git-commit-algorithm.md`
- Reference: `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md`
- Reference: `plugin/practices/git-guidelines.md` (current guidelines)
- Reference: `plugin/practices/conventional-commits.md`

**Step 1: Design commit readiness algorithm**

**Main Algorithm:**
```markdown
## Decision Algorithm: When to Commit

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

**Invalid Conditions:**
```markdown
## INVALID conditions for committing early (NOT in algorithm):
- "Is there time pressure?" → NOT A VALID CONDITION
- "Will I fix tests/checks later?" → NOT A VALID CONDITION
- "Are changes too small to break?" → NOT A VALID CONDITION
- "Did I manually test it?" → NOT A VALID CONDITION
- "Is this a WIP commit?" → NOT A VALID CONDITION (use stash instead)
- "Am I exhausted and want to save progress?" → NOT A VALID CONDITION (use stash)
```

**Step 2: Write complete practice document**

```markdown
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

[Decision Algorithm here]

[Invalid Conditions here]

## Self-Test

Q1: I have code changes, all tests pass, checks pass, docs updated, atomic change. Step 7 says?
    Answer: Commit is ready - stage files and commit with conventional format

Q2: Tests are failing but I want to commit WIP. Valid?
    Answer: NO. Step 3 → NO leads to Step 9 (fix before commit). "WIP commit" is INVALID condition

Q3: I'm exhausted and want to save progress. Should I commit failing checks?
    Answer: NO. Use git stash. "Exhaustion" is NOT A VALID CONDITION

Q4: Changes touch 3 unrelated concerns. Step 6 result?
    Answer: NO → Go to Step 10 (split into multiple commits)

## Integration with Existing Practices

**This algorithm provides:** WHEN to commit (readiness checks)

**See also:**
- `git-guidelines.md` - Branch naming, commit structure, splitting criteria
- `conventional-commits.md` - Commit message format (HOW to write commit)
- `testing.md` - Testing standards (what "tests pass" means)

## Why Algorithmic Format

Previous guideline: "Verify before committing: Ensure code is linted, builds correctly, and documentation is updated"

**Problem:** Agents interpreted "verify" as suggestion under time pressure.

**Solution:** Algorithm Step 3, 4, 5 have binary YES/NO checks. No interpretation possible.

**Evidence:** Based on algorithmic-command-enforcement pattern (0% → 100% compliance in pressure testing).
```

**Step 3: Commit**

```bash
git add plugin/practices/git-commit-algorithm.md
git commit -m "feat: add git commit readiness algorithm

Create algorithmic decision tree for commit readiness checks.
Prevents premature commits with failing tests, missing docs, or
non-atomic changes.

- 10-step algorithm: tests → checks → docs → atomicity
- Invalid conditions: time pressure, WIP, exhaustion, 'fix later'
- Self-test: 4 questions on readiness logic
- Integrates with git-guidelines, conventional-commits, testing

Complements existing practices with deterministic WHEN checks.
Based on algorithmic-command-enforcement pattern."
```

---

## Task 4: Update Discovery Tools

**Files:**
- Verify: `plugin/tools/find-skills` (should find new TDD enforcement skill)
- Verify: `plugin/tools/find-practices` (should find new git commit algorithm)

**Step 1: Test skill discovery**

```bash
./plugin/tools/find-skills "tdd"
./plugin/tools/find-skills "test-driven"
./plugin/tools/find-skills "tests first"
./plugin/tools/find-skills "enforcement"
```

Expected: New `plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md` appears in results.

**Step 2: Test practice discovery**

```bash
./plugin/tools/find-practices "commit"
./plugin/tools/find-practices "git"
./plugin/tools/find-practices "readiness"
```

Expected: New `plugin/practices/git-commit-algorithm.md` appears in results.

**Step 3: Verify frontmatter**

Check each new file has rich `when_to_use` with searchable keywords:
- TDD enforcement: "write tests first", "implementation code", "failing test"
- Code review trigger: "code review", "merge", "small changes"
- Git commit: "commit readiness", "failing tests", "documentation"

**Step 4: Document verification**

Create verification doc:

```markdown
# Discovery Verification

Tested: 2025-10-16

## TDD Enforcement Algorithm
- ✅ `find-skills "tdd"` - Found
- ✅ `find-skills "enforcement"` - Found
- ✅ `find-skills "tests first"` - Found

## Code Review Trigger
- ✅ Integrated into existing conducting-code-review skill
- ✅ `find-skills "code review"` - Found

## Git Commit Algorithm
- ✅ `find-practices "commit"` - Found
- ✅ `find-practices "readiness"` - Found
- ✅ `find-practices "git"` - Found

All discovery patterns working correctly.
```

Save to: `docs/tests/algorithmic-workflow-discovery-verification.md`

**Step 5: Commit**

```bash
git add docs/tests/algorithmic-workflow-discovery-verification.md
git commit -m "test: verify discovery for algorithmic workflow conversions

Confirm find-skills and find-practices locate new algorithmic
decision trees for TDD, code review, and git commits.

All patterns tested and working."
```

---

## Task 5: Create Pressure Test Scenarios

**Files:**
- Create: `docs/tests/tdd-enforcement-pressure-scenarios.md`
- Create: `docs/tests/code-review-trigger-pressure-scenarios.md`
- Create: `docs/tests/git-commit-pressure-scenarios.md`
- Reference: `docs/tests/execute-command-test-scenarios.md` (template)

**Step 1: Design TDD enforcement scenarios**

Create 3 pressure scenarios following execute-command test pattern:

**Scenario 1: Simple Bug Fix + Time Pressure**
- Context: One-line null check, demo in 20 minutes
- Pressure: Simplicity + deadline
- Expected rationalization: "Too simple to need test"
- Algorithm defense: Step 3 checks "Does failing test exist?" - NO → STOP

**Scenario 2: Complex Feature + Sunk Cost**
- Context: 3 hours work, 200 lines, no tests, 80% done
- Pressure: Sunk cost + near completion
- Expected rationalization: "Deleting is wasteful, will add tests after"
- Algorithm defense: Recovery Step 3 mandates DELETE

**Scenario 3: Production Hotfix + Authority**
- Context: CTO directive, fix NOW, customer on call
- Pressure: Authority + emergency + audience
- Expected rationalization: "Exception for critical hotfix"
- Algorithm defense: Step 2 requires user approval for exceptions

**Step 2: Design code review trigger scenarios**

**Scenario 1: Documentation Changes + "Too Small" Rationalization**
- Context: 5 markdown edits, readme updates
- Pressure: "Just docs, no code"
- Expected rationalization: "Too small for formal review"
- Algorithm defense: "Changes too small?" is INVALID condition

**Scenario 2: Senior Developer + Self-Review**
- Context: Principal engineer, 10 years experience
- Pressure: Seniority + confidence
- Expected rationalization: "I reviewed it myself"
- Algorithm defense: "Am I senior?" is INVALID condition

**Scenario 3: Hotfix + Tests Passing**
- Context: Production bug, tests green after fix
- Pressure: Emergency + automated validation
- Expected rationalization: "Tests prove it works"
- Algorithm defense: "Are tests passing?" is INVALID condition

**Step 3: Design git commit scenarios**

**Scenario 1: WIP Commit + Time Pressure**
- Context: Tests fail, need to leave for appointment
- Pressure: Time + desire to save work
- Expected rationalization: "WIP commit to save progress"
- Algorithm defense: Step 3 → NO (tests fail) → Step 9 (fix first). "WIP" is INVALID

**Scenario 2: Exhaustion + Manual Testing**
- Context: 10pm, manually tested, checks fail (formatting)
- Pressure: Exhaustion + manual validation
- Expected rationalization: "I tested it, formatting is trivial"
- Algorithm defense: Step 4 → NO (checks fail) → Step 9 (fix first)

**Scenario 3: Multi-Concern + Deadline**
- Context: 5 file changes, 3 different features, demo tomorrow
- Pressure: Deadline + desire for "progress commit"
- Expected rationalization: "One commit faster than splitting"
- Algorithm defense: Step 6 → NO (non-atomic) → Step 10 (split)

**Step 4: Document scenario structure**

Each file follows template:
```markdown
# [Workflow] Pressure Test Scenarios

## Purpose
Verify algorithmic format resists rationalization under pressure.

## Baseline Test (RED)
Test without algorithm - document rationalizations used.

## Algorithm Test (GREEN)
Test with algorithm - verify 100% compliance.

## Scenario 1: [Name]
[Full scenario description]

## Scenario 2: [Name]
[Full scenario description]

## Scenario 3: [Name]
[Full scenario description]

## Testing Instructions
1. Run RED phase with subagent WITHOUT skill
2. Document exact rationalizations (verbatim)
3. Run GREEN phase with subagent WITH skill
4. Verify compliance, note which mechanisms worked
```

**Step 5: Commit**

```bash
git add docs/tests/tdd-enforcement-pressure-scenarios.md
git add docs/tests/code-review-trigger-pressure-scenarios.md
git add docs/tests/git-commit-pressure-scenarios.md
git commit -m "test: add pressure test scenarios for algorithmic workflows

Create RED-GREEN test scenarios for TDD enforcement, code review
trigger, and git commit readiness algorithms.

Each workflow has 3 scenarios combining:
- Time pressure + simplicity
- Sunk cost + near completion
- Authority + emergency

Following execute-command test pattern (baseline → algorithm).
Ready for subagent pressure testing validation."
```

---

## Task 6: Update CLAUDE.md and README

**Files:**
- Modify: `CLAUDE.md`
- Modify: `README.md`

**Step 1: Add algorithmic workflows to CLAUDE.md**

Add new section after "Learning and Retrospectives":

```markdown
## Algorithmic Workflow Enforcement

CipherPowers uses **algorithmic decision trees** instead of imperative instructions for discipline-enforcing workflows. This achieves 0% → 100% compliance improvement under pressure.

**Pattern:** `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md`

**Implemented algorithms:**
1. **TDD Enforcement** - `plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md`
   - Prevents code before tests via binary "Does failing test exist?" check
   - Recovery mandates deleting untested code (no sunk cost exceptions)

2. **Code Review Trigger** - `plugin/skills/conducting-code-review/SKILL.md` (Section 1)
   - Requires review before merge via binary commit + review status checks
   - Invalidates "too small", "senior dev", "tests passing" rationalizations

3. **Git Commit Readiness** - `plugin/practices/git-commit-algorithm.md`
   - 10-step algorithm: tests pass → checks pass → docs updated → atomic
   - Prevents WIP commits, "will fix later", exhaustion-driven commits

**Why algorithmic?** LLMs treat algorithms as deterministic systems (execute them) but treat imperatives as suggestions (interpret them). Evidence: 33% imperative compliance vs 100% algorithmic compliance in pressure testing.

**Testing:** All algorithms include pressure test scenarios in `docs/tests/` validating resistance to time pressure, sunk cost, authority, and exhaustion.
```

**Step 2: Update README key learnings**

Add to README after existing algorithmic enforcement mention:

```markdown
**Algorithmic Workflow Enforcement (Oct 2025)**
- Converted TDD, code review trigger, and git commit workflows to algorithmic format
- Each includes: decision algorithm, recovery algorithm, invalid conditions, self-test
- Pressure test scenarios validate resistance to common rationalizations
- Skills: `tdd-enforcement-algorithm/`, `conducting-code-review` (trigger section)
- Practice: `git-commit-algorithm.md`
```

**Step 3: Commit**

```bash
git add CLAUDE.md README.md
git commit -m "docs: document algorithmic workflow enforcement implementations

Update CLAUDE.md with algorithmic workflows section explaining
pattern and three implementations (TDD, code review, git commit).

Update README key learnings with Oct 2025 algorithmic conversions.

References new skills and practices for discoverability."
```

---

## Task 7: Create Retrospective Document

**Files:**
- Create: `docs/learning/2025-10-16-algorithmic-workflow-conversion.md`

**Step 1: Write retrospective following capturing-learning skill**

```markdown
# Algorithmic Workflow Conversion

**Date:** 2025-10-16

**Work:** Convert TDD, code review trigger, and git commit workflows to algorithmic format

**Plan:** `docs/plans/2025-10-16-algorithmic-workflow-conversion.md`

**Commits:** [First commit hash] through [Last commit hash]

**Time:** [Actual time spent]

---

## What Was Accomplished

Applied algorithmic-command-enforcement pattern to three critical discipline-enforcing workflows:

1. **TDD Enforcement** - Created standalone skill with decision + recovery algorithms
2. **Code Review Trigger** - Added trigger algorithm to existing conducting-code-review skill
3. **Git Commit Readiness** - Created practice with 10-step readiness algorithm

All three include:
- Boolean condition decision trees (no interpretation possible)
- Recovery algorithms for "already started wrong" scenarios
- INVALID conditions lists (explicit rationalization defenses)
- Self-test sections (4 questions verifying comprehension)
- Pressure test scenarios (RED-GREEN validation ready)

---

## Key Decisions Made

### Decision 1: TDD as Standalone Skill vs Modifying Upstream

**Options considered:**
- A) Modify upstream superpowers TDD skill
- B) Create local enforcement wrapper skill

**Chose B** because:
- Upstream skill focuses on HOW (RED-GREEN-REFACTOR methodology)
- New skill focuses on WHEN (decision to use TDD at all)
- Separation enables independent evolution
- Can upstream enforcement algorithm later if proven effective

**Trade-off:** Two skills to maintain vs one (acceptable - different purposes)

### Decision 2: Code Review as Section vs Standalone Skill

**Options considered:**
- A) Create separate code-review-trigger skill
- B) Add trigger algorithm to existing conducting-code-review skill

**Chose B** because:
- Trigger and execution are related (same workflow)
- Agents already find conducting-code-review for reviews
- Section 1 (trigger) flows naturally to Section 2 (execution)
- Avoids skill proliferation

**Trade-off:** Larger single skill vs distributed knowledge (acceptable - cohesive workflow)

### Decision 3: Git Commit as Practice vs Skill

**Options considered:**
- A) Create skill in `plugin/skills/git/`
- B) Create practice in `plugin/practices/`

**Chose B** because:
- Git commit is project-specific (commands vary: mise/npm/cargo)
- Practice format separates standards from project config
- Practices are right abstraction for "when to commit" rules
- Complements existing git-guidelines.md and conventional-commits.md

**Trade-off:** Less discoverable via find-skills (acceptable - find-practices covers it)

---

## Approaches That Worked

### Approach 1: Algorithm-First Placement

Placed decision algorithms at TOP of documents, before "How to" sections.

**Why effective:**
- Agents evaluate WHEN before reading HOW
- Prevents skipping straight to implementation
- Mirrors execute-command structure (algorithm → instructions)

**Evidence:** execute-command agents run algorithm before reading usage details.

### Approach 2: Recovery Algorithms for Sunk Cost

Every workflow includes "Already started wrong?" recovery path.

**Why effective:**
- Addresses most common rationalization ("too late to restart")
- Binary condition checks (tests exist? docs updated?)
- Explicit DELETE mandate (no "keep as reference")

**Evidence:** execute-command recovery algorithm handled "2 hours untested code" scenario.

### Approach 3: Self-Test Before Workflow

4-question quizzes with answers force comprehension.

**Why effective:**
- Catches misunderstanding before work begins
- Provides reference for "what does Step X say?"
- Makes violations obvious (agent can check own quiz answers)

**Evidence:** execute-command agents cited self-test answers in compliance explanations.

---

## Approaches That Didn't Work

None yet - implemented based on proven execute-command pattern. Pressure testing will reveal if adjustments needed.

---

## What We Learned

### Learning 1: Algorithmic Format is Transferable

**Discovery:** Pattern works beyond original /execute command use case.

**Evidence:** Successfully applied to three different workflow types:
- Code discipline (TDD)
- Collaboration process (code review)
- Version control practice (git commit)

**Implication:** Template is universal for discipline-enforcing workflows. Any "must do X before Y" rule is conversion candidate.

### Learning 2: Recovery Algorithms Handle Sunk Cost

**Discovery:** Binary "already started wrong?" checks neutralize sunk cost rationalization.

**Example:**
```
Step 1: Have you written code without tests?
        → YES: Go to Step 2
        → NO: Continue

Step 2: Delete the untested code
        STOP
```

**Why effective:** No escape hatch. DELETE is unconditional. Hours invested = irrelevant (not in algorithm).

### Learning 3: Invalid Conditions Create Meta-Awareness

**Discovery:** Listing rationalizations as "NOT A VALID CONDITION" works better than explaining why they're wrong.

**Example:**
```
- "Is there time pressure?" → NOT A VALID CONDITION
- "Did I manually test it?" → NOT A VALID CONDITION
```

**Why effective:** Agents see their rationalization explicitly invalidated. Creates cognitive dissonance that prevents use.

---

## Testing Plan

### Pressure Testing Schedule

**TDD Enforcement:**
- Scenario 1: Simple bug + time pressure
- Scenario 2: Complex feature + sunk cost
- Scenario 3: Production hotfix + authority

**Code Review Trigger:**
- Scenario 1: Documentation + "too small"
- Scenario 2: Senior developer + self-review
- Scenario 3: Hotfix + tests passing

**Git Commit Readiness:**
- Scenario 1: WIP commit + time pressure
- Scenario 2: Exhaustion + manual testing
- Scenario 3: Multi-concern + deadline

**Method:** RED (baseline without algorithm) → GREEN (with algorithm) → measure compliance change

**Success criteria:** 80%+ compliance improvement (matching execute-command's 0% → 100%)

### Test Execution

**When:** After plan implementation, before claiming complete

**Who:** Fresh subagents (no prior context)

**Artifacts:** `docs/tests/*-pressure-results.md` documenting RED vs GREEN compliance

---

## Success Metrics

**Deliverables:**
- ✅ TDD enforcement algorithm skill created
- ✅ Code review trigger algorithm added to existing skill
- ✅ Git commit readiness algorithm practice created
- ✅ Pressure test scenarios designed (9 scenarios total)
- ✅ Discovery tools verified
- ✅ Documentation updated (CLAUDE.md, README)
- ✅ Retrospective captured

**Quality:**
- All algorithms follow 5-mechanism pattern
- All include self-tests with answers
- All reference algorithmic-command-enforcement template
- All have rich when_to_use for discovery

**Next Steps:**
- Execute pressure testing campaign
- Document RED-GREEN results
- Iterate algorithms if compliance < 80%
- Consider upstreaming TDD enforcement to superpowers

---

## Related Work

**Pattern:** `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md`

**Original discovery:** `docs/learning/2025-10-16-algorithmic-command-enforcement.md`

**Test methodology:** `docs/tests/execute-command-test-scenarios.md` (template)

**Commits:** [To be filled after execution]

---

## Recommendations

### Immediate (This Sprint)

1. **Execute pressure testing** - Validate algorithms resist rationalization
2. **Iterate based on results** - Plug loopholes discovered in testing
3. **Monitor production use** - Track new rationalization patterns

### Short-Term (Next Month)

4. **Convert verification-before-completion** - Another high-value algorithm candidate
5. **Create algorithmic template** - Standardize structure for future conversions
6. **Training documentation** - Explain algorithmic format to team

### Long-Term (Next Quarter)

7. **Upstream TDD enforcement** - Contribute to superpowers if proven effective
8. **Agent prompt integration** - Add algorithm checks to rust-engineer, code-reviewer agents
9. **Automated testing** - Script RED-GREEN pressure tests for CI validation
```

**Step 2: Commit**

```bash
git add docs/learning/2025-10-16-algorithmic-workflow-conversion.md
git commit -m "docs: capture learning from algorithmic workflow conversion

Document decisions, approaches, and lessons from converting three
discipline-enforcing workflows to algorithmic format.

Key learnings:
- Pattern is transferable across workflow types
- Recovery algorithms neutralize sunk cost rationalization
- Invalid conditions lists create meta-awareness

Includes testing plan and success criteria."
```

---

## Summary

**7 tasks total:**
1. TDD Enforcement Algorithm (standalone skill)
2. Code Review Trigger Algorithm (section in existing skill)
3. Git Commit Readiness Algorithm (practice)
4. Discovery Tools Verification (find-skills, find-practices)
5. Pressure Test Scenarios (9 scenarios across 3 workflows)
6. Documentation Updates (CLAUDE.md, README)
7. Retrospective (learning capture)

**Expected outcome:** 0% → 80%+ compliance improvement for TDD, code review trigger, and git commit workflows under pressure.

**Evidence basis:** Execute-command algorithmic format achieved 0% → 100% compliance in pressure testing.

**Time estimate:** 4-6 hours implementation + 2-3 hours pressure testing = 6-9 hours total.
