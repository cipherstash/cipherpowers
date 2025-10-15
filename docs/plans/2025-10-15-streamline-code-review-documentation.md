# Streamline Code Review Documentation Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Eliminate redundancy in code review documentation by separating methodology (skills), enforcement (agents), and standards (practices) following DRY, SRP, and the CipherPowers three-layer architecture.

**Architecture:** Rename and streamline `skills/code-review/` to `skills/conducting-code-review/` containing pure workflow methodology. Update `agents/code-reviewer.md` to enforce the skill with persuasion principles. Consolidate all standards (severity levels, templates, commands) into `docs/practices/code-review.md`. Remove all duplicated content.

**Tech Stack:** Markdown documentation, CipherPowers plugin architecture, Superpowers skills framework

---

## Task 1: Consolidate Standards in Practices

**Files:**
- Modify: `docs/practices/code-review.md`

**Step 1: Move severity level details to practices**

Update the severity levels section in `docs/practices/code-review.md` to include complete definitions currently spread across agent and skill:

```markdown
### Level 1: Blockers (Must Fix Before Merge)

-   **Security Vulnerabilities**:
    -   Any potential for SQL injection, XSS, CSRF, or other common vulnerabilities.
    -   Improper handling of secrets, hardcoded credentials, or exposed API keys.
    -   Insecure dependencies or use of deprecated cryptographic functions.
-   **Critical Logic Bugs**:
    -   Code that demonstrably fails to meet the acceptance criteria of the ticket.
    -   Race conditions, deadlocks, or unhandled promise rejections.
-   **Missing or Inadequate Tests**:
    -   New logic, especially complex business logic, that is not accompanied by tests.
    -   Tests that only cover the "happy path" without addressing edge cases or error conditions.
    -   Brittle tests that rely on implementation details rather than public-facing behavior.
-   **Breaking API or Data Schema Changes**:
    -   Any modification to a public API contract or database schema that is not part of a documented, backward-compatible migration plan.

### Level 2: High Priority (Strongly Recommend Fixing Before Merge)

-   **Architectural Violations**:
    -   **Single Responsibility Principle (SRP)**: Functions that have multiple, distinct responsibilities or operate at different levels of abstraction (e.g., mixing business logic with low-level data marshalling).
    -   **Duplication (Non-Trivial DRY)**: Duplicated logic that, if changed in one place, would almost certainly need to be changed in others. *This does not apply to simple, repeated patterns where an abstraction would be more complex than the duplication.*
    -   **Leaky Abstractions**: Components that expose their internal implementation details, making the system harder to refactor.
-   **Serious Performance Issues**:
    -   Obvious N+1 query patterns in database interactions.
    -   Inefficient algorithms or data structures used on hot paths.
-   **Poor Error Handling**:
    -   Swallowing exceptions or failing silently.
    -   Error messages that lack sufficient context for debugging.

### Level 3: Medium Priority (Consider for Follow-up)

-   **Clarity and Readability**:
    -   Ambiguous or misleading variable, function, or class names.
    -   Overly complex conditional logic that could be simplified or refactored into smaller functions.
    -   "Magic numbers" or hardcoded strings that should be named constants.
-   **Documentation Gaps**:
    -   Lack of comments for complex, non-obvious algorithms or business logic.
    -   Missing doc comments for public-facing functions.

### Level 4: Low Priority (Nice to Have)

-   **Style Preferences**: Minor naming improvements, formatting that isn't caught by linters
-   **Minor Optimizations**: Performance improvements with negligible impact
-   **Future Considerations**: Suggestions for future refactoring
```

**Step 2: Add review principles section**

Add a new "Review Principles" section after severity levels:

```markdown
## Review Principles

1. **Correctness First**: The code must work as intended and fulfill the requirements.
2. **Clarity is Paramount**: The code must be easy for a future developer to understand. Readability outweighs cleverness.
3. **Question Intent, Then Critique**: Before flagging an issue, understand the author's intent. Frame feedback constructively.
4. **Provide Actionable Suggestions**: Never just point out a problem. Always propose a concrete solution.
5. **Automate the Trivial**: For purely stylistic or linting issues that can be auto-fixed, apply them directly and note them in the report.
```

**Step 3: Verify all project configuration is complete**

Ensure the "Project Configuration" section contains:
- Review file conventions (location, naming pattern)
- Commands (mise run test, mise run check, mise run review:active)
- Complete review template structure

**Step 4: Commit practices consolidation**

```bash
git add docs/practices/code-review.md
git commit -m "docs: consolidate code review standards in practices

Move complete severity level definitions and review principles
to practices document. This becomes single source of truth for
standards that skills and agents reference."
```

---

## Task 2: Rename and Streamline Code Review Skill

**Files:**
- Rename: `skills/code-review/` → `skills/conducting-code-review/`
- Modify: `skills/conducting-code-review/SKILL.md`

**Step 1: Rename the skill directory**

```bash
git mv skills/code-review skills/conducting-code-review
```

**Step 2: Update skill frontmatter**

Replace frontmatter in `skills/conducting-code-review/SKILL.md`:

```markdown
---
name: Conducting Code Review
description: Complete workflow for conducting thorough code reviews with test verification and structured feedback
when_to_use: when conducting code review, when another agent asks you to review code, after being dispatched by requesting-code-review skill
version: 2.0.0
---
```

**Step 3: Rewrite skill to reference practices instead of duplicating**

Replace the "Review code against standards" section (current lines 58-66):

```markdown
#### 3. Review code against standards

**Read standards from practices:**

```bash
# Standards live in practices, not in this skill
@docs/practices/code-review.md
```

**Review ALL four severity levels:**
1. Level 1: Blockers (from practices)
2. Level 2: High Priority (from practices)
3. Level 3: Medium Priority (from practices)
4. Level 4: Low Priority (from practices)

**Empty sections are GOOD if you actually checked.** Missing sections mean you didn't check.
```

**Step 4: Update template section to reference practices**

Replace the "Save structured review" section (current lines 80-115):

```markdown
#### 5. Save structured review

**File naming and template:**

See `@docs/practices/code-review.md` for:
- File naming convention (`{YYYY-MM-DD}-review-{N}.md`)
- Complete template structure with all sections
- Examples of review file organization

**Use template exactly as specified in practices.**
```

**Step 5: Remove duplicate review principles**

Remove the "Review Principles" section (it's now in practices).

**Step 6: Add upstream skill references**

Add a "Related Skills" section at the end:

```markdown
## Related Skills

**Before using this skill:**
- Requesting Code Review: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md`

**When receiving feedback on your review:**
- Code Review Reception: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/receiving-code-review/SKILL.md`
```

**Step 7: Commit skill changes**

```bash
git add skills/conducting-code-review/
git commit -m "refactor: rename and streamline code review skill

- Rename code-review -> conducting-code-review (clearer purpose)
- Remove duplicated severity levels (reference practices)
- Remove duplicated template structure (reference practices)
- Add upstream skill references for discoverability
- Focus on pure workflow methodology"
```

---

## Task 3: Update Code Reviewer Agent to Enforce Skill

**Files:**
- Modify: `agents/code-reviewer.md`

**Step 1: Update context section to mandate skill**

Replace the "YOU MUST ALWAYS READ" section (lines 13-24):

```markdown
    ## Context

    YOU MUST ALWAYS READ IN THIS ORDER:

    1. **Upstream Skills** (universal methodology):
       - Requesting Code Review: @${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md
       - Code Review Reception: @${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/receiving-code-review/SKILL.md

    2. **Local Skill** (complete workflow):
       - Conducting Code Review: @skills/conducting-code-review/SKILL.md

    3. **Project Standards** (what quality looks like):
       - Code Review Standards: @docs/practices/code-review.md
       - Development Standards: @docs/practices/development.md
       - Testing Standards: @docs/practices/testing.md

    4. **Project Context**:
       - README.md: @README.md
       - Architecture: @CLAUDE.md
```

**Step 2: Update workflow announcement to reference skill**

Replace the announcement section (lines 32-43):

```markdown
    ### 1. Announcement (Commitment)

    IMMEDIATELY announce:
    ```
    I'm using the code-reviewer agent with conducting-code-review skill.

    Non-negotiable workflow (from skill):
    1. Read all context files, practices, and skills
    2. Identify code to review (git commands)
    3. Run all project tests and checks
    4. Review code against practice standards (ALL severity levels)
    5. Save structured feedback to work directory
    6. No approval without thorough review
    ```
```

**Step 3: Simplify workflow steps to reference skill**

Replace the detailed workflow steps (lines 45-98) with enforcement:

```markdown
    ### 2. Follow Conducting Code Review Skill

    YOU MUST follow every step in @skills/conducting-code-review/SKILL.md:

    - [ ] Step 1: Identify code to review (skill defines git commands)
    - [ ] Step 2: Run tests and checks (skill references practices for commands)
    - [ ] Step 3: Review against standards (skill references practices for severity levels)
    - [ ] Step 4: Find active work directory (skill defines process)
    - [ ] Step 5: Save structured review (skill references practices for template)

    **The skill defines HOW. You enforce that it gets done.**

    ### 3. No Skipping Steps

    **EVERY step in the skill is mandatory:**
    - Running tests yourself (even if "already passed")
    - Running checks yourself
    - Reviewing ALL severity levels (not just critical)
    - Saving review file to work directory
    - Including positive observations

    **If you skip ANY step, you have violated this workflow.**
```

**Step 4: Update rationalization defense to reference skill steps**

Update the rationalization defense table (lines 114-126):

```markdown
    | Excuse | Reality |
    |--------|---------|
    | "Tests passed last time, skip running them" | Skill Step 2 is mandatory. Run tests. Always. |
    | "Code looks clean, quick approval" | Skill Step 3 requires ALL severity levels. No shortcuts. |
    | "Only flagging critical issues" | Skill defines 4 levels. Review all or you failed. |
    | "Low priority can be ignored" | Skill Step 3: Review ALL levels. Document findings. |
    | "Simple change, no thorough review needed" | Simple changes break production. Follow skill completely. |
    | "Already reviewed similar code" | Each review is independent. Skill applies every time. |
    | "Requester is senior, trust their work" | Seniority ≠ perfection. Skill workflow is non-negotiable. |
```

**Step 5: Remove detailed capability and behavioral sections**

Remove the "Purpose", "Capabilities", "Design Evaluation", "Readability Review", "Test Coverage Analysis", "Behavioral Traits", "Response Approach", and "Example Interactions" sections (lines 150-205).

These are methodology details that belong in the skill, not the agent enforcer.

**Step 6: Commit agent updates**

```bash
git add agents/code-reviewer.md
git commit -m "refactor: update code-reviewer agent to enforce skill

- Mandate reading conducting-code-review skill
- Reference skill for methodology (not duplicate)
- Keep persuasion principles and rationalization defenses
- Remove duplicated workflow details (in skill)
- Agent enforces THAT work gets done, skill defines HOW"
```

---

## Task 4: Update Code Review Command to Reference New Skill

**Files:**
- Modify: `commands/code-review.md` (if it exists)

**Step 1: Check if code-review command exists**

```bash
ls -la commands/code-review.md 2>/dev/null || echo "Command does not exist"
```

**Step 2: If command exists, update skill reference**

If the command exists, update any references from `skills/code-review/SKILL.md` to `skills/conducting-code-review/SKILL.md`.

**Step 3: Commit command updates if changed**

```bash
# Only if command was modified
git add commands/code-review.md
git commit -m "refactor: update code-review command skill reference

Point to renamed conducting-code-review skill."
```

---

## Task 5: Verify No Broken References

**Files:**
- Search: All markdown files in repository

**Step 1: Search for old skill references**

```bash
grep -r "skills/code-review" --include="*.md" .
```

**Step 2: Update any remaining references**

For each file found, update references from `skills/code-review/` to `skills/conducting-code-review/`.

**Step 3: Search for duplicated content patterns**

```bash
# Check for duplicated severity level definitions
grep -r "Level 1: Blockers" --include="*.md" .

# Check for duplicated template structures
grep -r "## Critical Issues (Level 1" --include="*.md" .
```

**Step 4: Remove any remaining duplication**

If duplication is found outside of practices, remove it and add reference to practices instead.

**Step 5: Commit any additional cleanup**

```bash
git add .
git commit -m "refactor: fix remaining code review documentation references

Update all references to point to conducting-code-review skill
and practices document."
```

---

## Task 6: Update CLAUDE.md Example

**Files:**
- Modify: `CLAUDE.md`

**Step 1: Update code review workflow example**

Find the "Example: Code Review Workflow" section in CLAUDE.md (around line 85) and update it:

```markdown
**Example: Code Review Workflow**
- `skills/conducting-code-review/SKILL.md` = Complete workflow (test verification, structured feedback, work directory save)
- `docs/practices/code-review.md` = Standards (severity levels) + Project Config (commands, file conventions)
- `agents/code-reviewer.md` = Workflow enforcement with persuasion principles (non-negotiable steps, rationalization defenses)
- `commands/code-review.md` = Thin dispatcher (sets context, references skill)
- Skills: References upstream "Requesting Code Review" and "Code Review Reception" skills

All components work together without duplication:
- Update severity standards in practices → agent uses new standards automatically
- Change project commands (mise run test) → skill/agent reference practice for current command
- Update workflow in skill → agent enforces updated workflow
- Commands remain simple dispatchers → workflow discovery via skills
```

**Step 2: Commit CLAUDE.md update**

```bash
git add CLAUDE.md
git commit -m "docs: update code review example with new structure

Reflect renamed skill and clarify separation of concerns between
skill (methodology), agent (enforcement), and practices (standards)."
```

---

## Task 7: Final Verification

**Files:**
- All modified files

**Step 1: Run project checks**

```bash
mise run check
```

Expected: PASS (no broken links, formatting issues)

**Step 2: Verify git status is clean**

```bash
git status
```

Expected: "nothing to commit, working tree clean"

**Step 3: Review commit history**

```bash
git log --oneline -7
```

Expected: 6 commits following atomic commit principles (one commit per task)

**Step 4: Create summary of changes**

Document the changes:
- 1 skill renamed and streamlined
- 1 agent updated to enforce skill
- 1 practices document consolidated
- 1 CLAUDE.md example updated
- All duplication removed

---

## Success Criteria

- [ ] Severity levels defined in ONE place (`docs/practices/code-review.md`)
- [ ] Review template defined in ONE place (`docs/practices/code-review.md`)
- [ ] Commands defined in ONE place (`docs/practices/code-review.md`)
- [ ] Workflow methodology in ONE place (`skills/conducting-code-review/SKILL.md`)
- [ ] Enforcement in ONE place (`agents/code-reviewer.md`)
- [ ] Agent references skill for methodology
- [ ] Skill references practices for standards
- [ ] No duplicated content across files
- [ ] All references updated to new skill name
- [ ] CLAUDE.md example reflects new structure
