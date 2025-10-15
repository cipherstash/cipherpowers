# Refactor Documentation Skills Structure

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Separate reusable documentation workflows (skills) from project-specific standards (practices) for maximum reusability and eliminate duplication.

**Architecture:** Extract generic documentation standards from the maintaining-after-changes skill into docs/practices/, create a new capturing-learning skill for retrospective summaries, and update both skills to reference shared standards.

**Tech Stack:** Markdown documentation, TDD testing methodology for skills

---

## Task 1: Extract Standards from Skill to Practices

**Files:**
- Modify: `skills/documentation/maintaining-docs-after-changes/SKILL.md:121-160`
- Verify: `docs/practices/documentation.md` (ensure it's comprehensive)

**Step 1: Read current documentation standards in skill**

Read lines 121-160 of `skills/documentation/maintaining-docs-after-changes/SKILL.md` to identify what's being duplicated.

**Step 2: Verify docs/practices/documentation.md is comprehensive**

Read `docs/practices/documentation.md` and confirm it contains:
- Formatting and structure guidelines
- Content completeness requirements
- README organization patterns

Expected: All standards from skill already exist in practices (they do - nearly identical)

**Step 3: Replace standards section with reference**

In `skills/documentation/maintaining-docs-after-changes/SKILL.md`, replace lines 121-160 with:

```markdown
## Documentation Standards

Follow project documentation standards defined in:
- @docs/practices/documentation.md

**Key standards to verify:**
- Formatting and structure (headings, examples, status indicators)
- Content completeness (usage examples, troubleshooting)
- README organization (concise main file, README_*.md for specialized docs)
```

**Step 4: Verify skill still makes sense**

Read the modified skill end-to-end to ensure:
- Reference to practices is clear
- No broken flow from removing inline standards
- "Common Mistakes" section still makes sense (it does - it's workflow-specific)

Expected: Skill remains coherent, now references practices for standards

**Step 5: Commit the change**

```bash
git add skills/documentation/maintaining-docs-after-changes/SKILL.md
git commit -m "refactor: extract doc standards from skill to practices

Replace inline documentation standards with reference to
docs/practices/documentation.md to eliminate duplication.

Standards are project-specific and apply to all documentation
work, not just maintaining-after-changes workflow."
```

---

## Task 2: Create Capturing Learning Skill (TDD - RED Phase)

**Files:**
- Create: `skills/documentation/capturing-learning/SKILL.md`
- Create: `skills/documentation/capturing-learning/test-scenarios.md`

**Step 1: Create test scenarios file**

Create `skills/documentation/capturing-learning/test-scenarios.md` with pressure scenarios:

```markdown
# Test Scenarios for Capturing Learning Skill

## Purpose
Test whether agents systematically capture learning from completed work, or rationalize skipping under exhaustion/minimization pressure.

## Scenario 1: Exhaustion After Complex Implementation

**Context:**
After 10 hours implementing a complex feature, tests finally pass.
Many approaches were tried and discarded.
Several subtle bugs were discovered and fixed.

**User says:** "Great! Let's ship it."

**Expected violations (baseline):**
- "I remember what happened"
- "Too tired to write it up"
- "It's all in the commits"
- Skip capturing discarded approaches
- Skip documenting subtle issues

## Scenario 2: Minimization of "Simple" Task

**Context:**
Spent 3 hours on what should have been a "simple" fix.
Root cause was non-obvious.
Solution required understanding undocumented system interaction.

**User says:** "Nice, that's done."

**Expected violations:**
- "Not worth documenting"
- "It was just a small fix"
- "Anyone could figure this out"
- Skip documenting why it took 3 hours
- Skip capturing system interaction knowledge

## Scenario 3: Multiple Small Tasks

**Context:**
Completed 5 small tasks over 2 days.
Each had minor learnings or gotchas.
No single "big" lesson to capture.

**User says:** "Good progress. What's next?"

**Expected violations:**
- "Nothing significant to document"
- "Each task was too small"
- "I'll remember the gotchas"
- Skip incremental learning
- Skip patterns across tasks
```

**Step 2: Run baseline test WITHOUT skill**

Dispatch subagent with Scenario 1, document behavior:
- Do they suggest creating summary?
- What rationalizations do they use?
- What gets skipped?

Expected: Agent skips capturing learning, uses exhaustion as reason

**Step 3: Document baseline results**

Add results to test-scenarios.md under "Baseline Results" section with observed rationalizations verbatim.

**Step 4: Commit test scenarios**

```bash
git add skills/documentation/capturing-learning/test-scenarios.md
git commit -m "test: add baseline scenarios for capturing-learning skill

RED phase: Create pressure scenarios and document baseline
behavior before writing the skill."
```

---

## Task 3: Write Capturing Learning Skill (TDD - GREEN Phase)

**Files:**
- Create: `skills/documentation/capturing-learning/SKILL.md`

**Step 1: Create skill with frontmatter**

Create `skills/documentation/capturing-learning/SKILL.md`:

```markdown
---
name: Capturing Learning from Completed Work
description: Systematic retrospective to capture decisions, lessons, and insights from completed work
when_to_use: when completing significant work, after debugging sessions, before moving to next task, when work took longer than expected, or when approaches were discarded
version: 1.0.0
languages: all
---

# Capturing Learning from Completed Work

## Overview

**Context is lost rapidly without systematic capture.** After completing work, engineers move to the next task and forget valuable lessons, discarded approaches, and subtle issues discovered. This skill provides a systematic retrospective workflow to capture learning while context is fresh.

## When to Use

Use this skill when:
- Completing significant features or complex bugfixes
- After debugging sessions (especially multi-hour sessions)
- Work took longer than expected
- Multiple approaches were tried and discarded
- Subtle bugs or non-obvious issues were discovered
- Before moving to next task (capture fresh context)
- Sprint/iteration retrospectives

**When NOT to use:**
- Trivial changes (typo fixes, formatting)
- Work that went exactly as expected with no learnings
- When learning is already documented elsewhere

## Critical Principle

**Exhaustion after completion is when capture matters most.**

The harder the work, the more valuable the lessons. "Too tired" means the learning is significant enough to warrant documentation.
```

**Step 2: Add Common Rationalizations table**

Add section addressing baseline violations:

```markdown
## Common Rationalizations (And Why They're Wrong)

| Rationalization | Reality |
|----------------|---------|
| "I remember what happened" | Memory fades in days. Future you won't remember details. |
| "Too tired to write it up" | Most tired = most learning. 10 minutes now saves hours later. |
| "It's all in the commits" | Commits show WHAT changed, not WHY you chose this approach. |
| "Not worth documenting" | If you spent >30 min on it, someone else will too. Document it. |
| "It was too simple/small" | If it wasn't obvious to you at first, it won't be obvious to others. |
| "Anyone could figure this out" | You didn't know it before. Document for past-you. |
| "Nothing significant happened" | Every task teaches something. Capture incremental learning. |

**None of these are valid reasons to skip capturing learning.**
```

**Step 3: Add What to Capture checklist**

```markdown
## What to Capture

**✅ MUST document:**
- [ ] Brief description of what was accomplished
- [ ] Key decisions made (and why)
- [ ] Approaches that were tried and discarded (and why they didn't work)
- [ ] Non-obvious issues discovered (and how they were solved)
- [ ] Time spent vs. initial estimate (if significantly different, why?)
- [ ] Things that worked well (worth repeating)
- [ ] Things that didn't work well (worth avoiding)
- [ ] Open questions or follow-up needed

**Common blind spots:**
- Discarded approaches (most valuable learning often comes from what DIDN'T work)
- Subtle issues (small bugs that took disproportionate time)
- Implicit knowledge (things you learned but didn't realize were non-obvious)
```

**Step 4: Add Implementation workflow**

```markdown
## Implementation

### Step 1: Review the Work

Before writing, review what was done:
- Check git diff to see all changes
- Review commit messages for key decisions
- List approaches tried (including failed ones)
- Note time spent and estimates

### Step 2: Capture in Structure

Create or update summary in appropriate location:

**For work tracking systems:**
- Use project's work directory structure
- Common: `docs/work/summary.md` or iteration-specific file

**For non-tracked work:**
- Add to CLAUDE.md under relevant section
- Or create dated file in `docs/learning/YYYY-MM-DD-topic.md`

**Minimal structure:**
```markdown
## [Work Item / Feature Name]

**What:** Brief description (1-2 sentences)

**Key Decisions:**
- Decision 1 (why)
- Decision 2 (why)

**What Didn't Work:**
- Approach X (why it failed, what we learned)
- Approach Y (why it failed)

**Issues Discovered:**
- Issue 1 (how solved)
- Issue 2 (how solved)

**Time Notes:**
Estimated X hours, took Y hours. [Explain if significant difference]

**Open Questions:**
- Question 1
- Question 2
```

### Step 3: Link to Implementation

Connect learning to codebase:
- Reference key files modified
- Link to commits or PRs
- Cross-reference to CLAUDE.md if patterns emerged

### Step 4: Make it Searchable

Ensure future discoverability:
- Use descriptive headings
- Include error messages if debugging
- Tag with relevant technology/pattern names
```

**Step 5: Add Real-World Impact section**

```markdown
## Real-World Impact

**Without systematic capture:**
- Repeat same failed approaches (waste time)
- Forget subtle issues (encounter again later)
- Lose context on decisions (question past choices)
- Can't transfer knowledge to team
- Learning stays with individual

**With this workflow:**
- Failed approaches documented (others avoid same path)
- Subtle issues captured (searchable solutions)
- Decision rationale preserved (future maintenance easier)
- Knowledge shared across team
- Organization builds learning repository
```

**Step 6: Commit the skill**

```bash
git add skills/documentation/capturing-learning/SKILL.md
git commit -m "feat: add capturing-learning skill (GREEN phase)

Create skill to systematically capture learning from completed
work, addressing common rationalizations from baseline testing."
```

---

## Task 4: Test Skill and Refine (TDD - REFACTOR Phase)

**Files:**
- Modify: `skills/documentation/capturing-learning/SKILL.md` (if needed)
- Modify: `skills/documentation/capturing-learning/test-scenarios.md`

**Step 1: Run test WITH skill**

Dispatch subagent with Scenario 1 and the skill:
- Does agent follow workflow?
- Does agent resist rationalizations?
- Does agent capture all key items?

Expected: Agent systematically captures learning

**Step 2: Document results**

Add to test-scenarios.md under "With Skill Results" section:
- Compliance verification
- Whether rationalizations resisted
- Completeness of capture

**Step 3: Identify any new loopholes**

If agent found ways around the skill:
- Document new rationalizations
- Add explicit counters to skill
- Re-test

Expected: Skill successfully enforces learning capture

**Step 4: Commit test results**

```bash
git add skills/documentation/capturing-learning/test-scenarios.md
git commit -m "test: verify capturing-learning skill effectiveness

GREEN/REFACTOR phase: Skill successfully resists rationalizations
and enforces systematic learning capture."
```

---

## Task 5: Update Summarise Command

**Files:**
- Modify: `commands_all/summarise.md` → `commands/summarise.md`

**Step 1: Read current summarise.md**

Read `commands_all/summarise.md` to understand current structure.

Expected: References mise work:active, has inline instructions

**Step 2: Create new command referencing skill**

Create `commands/summarise.md`:

```markdown
# Summarise

Create a comprehensive retrospective summary of completed work, capturing decisions, lessons learned, and insights for continuous improvement.

<context>
## Context

**Learning Capture Methodology:**
Follow the "Capturing Learning from Completed Work" skill:
- @skills/documentation/capturing-learning/SKILL.md

**Documentation Standards:**
- @docs/practices/documentation.md

**Key Principle:**
Exhaustion after completion is when capture matters most. The harder the work, the more valuable the lessons.
</context>

<instructions>
## Instructions

Follow the systematic retrospective workflow from the capturing-learning skill:

### Step 1: Review the Work

1. **Identify work location:**
   - Check if project uses work tracking (e.g., `mise run work:active`)
   - Or determine appropriate summary location

2. **Review what was done:**
   - Run `git diff [base-branch]...HEAD` to see all changes
   - Review commit messages for key decisions
   - List approaches tried (including failed ones)
   - Note time spent vs. estimates

### Step 2: Capture Learning

Create or update summary covering:

**✅ MUST include:**
- [ ] Brief description of what was accomplished
- [ ] Key decisions made (with rationale)
- [ ] Approaches tried and discarded (what didn't work and why)
- [ ] Non-obvious issues discovered (and solutions)
- [ ] Time spent vs. estimate (if significantly different, explain)
- [ ] Things that worked well (worth repeating)
- [ ] Things that didn't work well (worth avoiding)
- [ ] Open questions or follow-up needed

**Common blind spots to check:**
- Discarded approaches (most valuable learning)
- Subtle issues that took disproportionate time
- Implicit knowledge (things you learned but didn't realize were non-obvious)

### Step 3: Save and Link

1. **Save to appropriate location:**
   - Work directory: `summary.md` (or iteration-specific)
   - No work tracking: Add to CLAUDE.md or `docs/learning/YYYY-MM-DD-topic.md`

2. **Link to implementation:**
   - Reference key files modified
   - Link to commits or PRs
   - Cross-reference CLAUDE.md if patterns emerged

3. **Make it searchable:**
   - Use descriptive headings
   - Include error messages if debugging occurred
   - Tag with relevant technology/pattern names

### Step 4: Merge if Existing

If summary already exists:
- Merge new learnings with existing content
- Ensure chronological or logical flow
- Remove redundancy

</instructions>
```

**Step 3: Verify command completeness**

Check that command:
- References the skill for methodology
- Includes critical checklist items
- Highlights blind spots
- Provides clear save locations

**Step 4: Commit updated command**

```bash
git add commands/summarise.md
git commit -m "refactor: update summarise command to reference skill

Update command to dispatch to capturing-learning skill for
methodology. Simplifies command to dispatcher role while
ensuring systematic learning capture workflow."
```

---

## Task 6: Update doc-review Command Reference

**Files:**
- Verify: `commands/doc-review.md`

**Step 1: Read current doc-review command**

Read `commands/doc-review.md` to check its current state.

Expected: Should already reference maintaining-docs-after-changes skill (was updated recently)

**Step 2: Verify practices reference**

Check if command mentions `@docs/practices/documentation.md`.

Expected: Should be present in context section

**Step 3: Add note about skill/practices separation if needed**

If not already clear, ensure context section mentions:

```markdown
**Key Principle:**
Skills provide workflow (how), practices provide standards (what).
```

**Step 4: Commit if changes needed**

If changes made:
```bash
git add commands/doc-review.md
git commit -m "docs: clarify skill vs practices separation in doc-review"
```

Otherwise: No changes needed, verification complete

---

## Task 7: Final Verification

**Files:**
- Read: All modified files

**Step 1: Verify separation of concerns**

Check that:
- `docs/practices/documentation.md` = standards only
- Skills in `skills/documentation/` = workflow only
- Commands in `commands/` = dispatchers only

Expected: Clean separation achieved

**Step 2: Verify cross-references work**

Check all `@` references:
- `maintaining-docs-after-changes/SKILL.md` → `@docs/practices/documentation.md`
- `capturing-learning/SKILL.md` → `@docs/practices/documentation.md` (if used)
- `commands/doc-review.md` → `@skills/documentation/maintaining-docs-after-changes/SKILL.md`
- `commands/summarise.md` → `@skills/documentation/capturing-learning/SKILL.md`

Expected: All references valid

**Step 3: Check for remaining duplication**

Search for duplicated content:
- Documentation standards should only be in `docs/practices/documentation.md`
- Workflow specifics only in skills
- No inline duplication

Expected: No duplication found

**Step 4: Review plan outcomes**

Verify achieved:
- ✅ Standards extracted to practices (single source)
- ✅ New capturing-learning skill created with TDD
- ✅ Commands updated to reference skills
- ✅ Clean separation: standards (docs/) vs workflows (skills/) vs dispatchers (commands/)
- ✅ Maximum reusability (skills can be upstreamed, practices are project-specific)

**Step 5: Create summary commit if needed**

If any small fixes were made during verification:
```bash
git add .
git commit -m "chore: final verification of documentation structure refactor"
```

---

## Completion

All tasks complete. Structure now has:

**`docs/practices/documentation.md`**
- Single source of truth for documentation standards
- Project-specific conventions
- Referenced by all documentation workflows

**`skills/documentation/maintaining-docs-after-changes/`**
- Workflow for syncing docs with code changes
- References practices for standards
- TDD-tested, ready for upstream

**`skills/documentation/capturing-learning/`**
- Workflow for retrospective learning capture
- References practices for standards
- TDD-tested

**`commands/doc-review.md` & `commands/summarise.md`**
- Dispatch to respective skills
- Provide context and trigger execution
- No inline duplication

**Benefits achieved:**
- DRY: Standards in one place
- SRP: Each component has single responsibility
- Reusability: Skills are universal, practices are project-specific
- Testability: Skills have TDD test scenarios
- Maintainability: Changes to standards don't require updating skills
