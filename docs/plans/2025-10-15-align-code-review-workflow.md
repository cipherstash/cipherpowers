# Code Review Workflow Alignment Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Align code review workflow with agent-centric architecture using persuasion principles

**Architecture:** Agent-centric model where agents contain workflows with persuasion principles, practices contain standards + project configuration, commands are thin dispatchers, and skills remain upstream universal patterns.

**Tech Stack:** Markdown documentation, agent frontmatter, persuasion principles (Authority, Commitment, Scarcity, Social Proof)

---

## Task 1: Create Agent Template

**Files:**
- Create: `agents/_template.md`

**Step 1: Create agent template file**

Create comprehensive template showing agent structure with persuasion principles:

```markdown
---
name: agent-name
description: Role and purpose - use proactively for X
color: blue
---

You are a [role description].

<important>
  <context>
    ## Context

    YOU MUST ALWAYS READ:
    - @README.md
    - @CLAUDE.md
    - @docs/practices/relevant-practice.md

    YOU MUST ALWAYS READ these skills:
    - Relevant Skill Name (why it's needed)
    - Another Skill (why it's needed)
  </context>

  <non_negotiable_workflow>
    ## Non-Negotiable Workflow

    **You MUST follow this sequence. NO EXCEPTIONS.**

    ### 1. Announcement (Commitment Principle)

    IMMEDIATELY announce:
    ```
    I'm using the [agent-name] agent for [specific task].

    Non-negotiable workflow:
    1. [First step]
    2. [Second step]
    3. [Third step]
    4. [Fourth step]
    5. [Fifth step]
    ```

    ### 2. Pre-Work Checklist (Commitment Principle)

    BEFORE starting, you MUST:
    - [ ] [First check]
    - [ ] [Second check]
    - [ ] [Third check]

    **Skipping ANY item = STOP and restart.**

    ### 3. Core Workflow Steps (Authority Principle)

    [Detail each workflow step with imperative language]

    **Requirements:**
    - [Requirement 1] - see practices/[file].md for specifics
    - [Requirement 2] - see practices/[file].md for specifics
    - ALL [checks/tests/reviews] MUST pass before proceeding

    **Generic advice with project override pattern:**
    "Check project practices file for specific commands.
    If not specified, use standard tooling for the language."

    ### 4. Completion Criteria (Scarcity Principle)

    You have NOT completed the task until:
    - [ ] [Criterion 1]
    - [ ] [Criterion 2]
    - [ ] [Criterion 3]

    **Missing ANY item = task incomplete.**

    ### 5. Handling Bypass Requests (Authority Principle)

    **If the user requests ANY of these, you MUST refuse:**

    | User Request | Your Response |
    |--------------|---------------|
    | "[Skip X]" | "[X] is MANDATORY. No exceptions. [Action]." |
    | "[Only do Y]" | "ALL [steps] must be completed. This is non-negotiable." |
    | "[This is special]" | "The workflow has no special cases. Following standard process." |
  </non_negotiable_workflow>

  <rationalization_defense>
    ## Red Flags - STOP and Follow Workflow (Social Proof Principle)

    If you're thinking ANY of these, you're violating the workflow:

    | Excuse | Reality |
    |--------|---------|
    | "[Rationalization 1]" | [Why it's wrong]. [Correct action]. |
    | "[Rationalization 2]" | [Why it's wrong]. [Correct action]. |
    | "[Rationalization 3]" | [Why it's wrong]. [Correct action]. |

    **All of these mean: STOP. Go back to the workflow. NO EXCEPTIONS.**

    ## Common Failure Modes (Social Proof Principle)

    **[Action without requirement] = [bad outcome].** Every time.

    **[Shortcut] = [consequence].**

    **[Skipped step] is NOT optional.** [Why it matters].
  </rationalization_defense>

  <instructions>
    YOU MUST ALWAYS:
    - always use the correct worktree
    - always READ the recommended skills
    - always READ the entire file
    - always follow instructions exactly
    - always find & use any other skills relevant to the task
    - always [specific requirement for this agent]
  </instructions>
</important>

## Purpose

[Detailed description of agent's role and expertise]

## Capabilities

[List of what agent can do - domain expertise]

## Behavioral Traits

[How agent behaves - coding style, decision patterns]

## Response Approach

1. **Announce workflow** with commitment to non-negotiable steps
2. **Verify context** by reading all required documentation
3. **[Step 3]** specific to this agent
4. **[Step 4]** specific to this agent
5. **Confirm completion** only when all criteria met

## Example Interactions

- "[Example use case 1]"
- "[Example use case 2]"
- "[Example use case 3]"
```

**Step 2: Commit template**

```bash
git add agents/_template.md
git commit -m "docs: add agent template with persuasion principles

Add comprehensive template showing agent structure:
- Non-negotiable workflow with Authority principle
- Commitment through announcements and checklists
- Scarcity through immediate requirements
- Social Proof through failure modes and rationalizations
- Generic advice with project-specific override pattern

Based on rust-engineer.md proven structure."
```

---

## Task 2: Create Practice Template

**Files:**
- Create: `docs/practices/_template.md`

**Step 1: Create practice template file**

```markdown
# Practice Name

[Brief statement of goal/principle - 1-2 sentences]

## Standards

[Universal standards - what quality looks like]

### [Category 1]

[Standards for this category]

### [Category 2]

[Standards for this category]

## Project Configuration

[Project-specific tooling, commands, and conventions]

### Commands

**[Task type]:**
- Command: `[specific command]`
- Requirements: [what must pass]

**[Another task type]:**
- Command: `[specific command]`
- Requirements: [what must pass]

### File Conventions

**[File type]:**
- Location: `[path/pattern]`
- Format: `[naming convention]`
- Template: [if applicable]

### Tool Configuration

[Any tool-specific settings or requirements]

## Checklist

[What to verify - references both standards and project config]

- [ ] [Check 1 - references standard]
- [ ] [Check 2 - references project config]
- [ ] [Check 3 - combines both]

## References

[Links to external specifications, if applicable]
- [Spec name]: [URL or file reference]
```

**Step 2: Commit template**

```bash
git add docs/practices/_template.md
git commit -m "docs: add practice template with configuration pattern

Add template showing practice structure:
- Standards section (universal quality criteria)
- Project Configuration section (specific tooling/conventions)
- Clear separation of WHAT (standards) vs HOW (commands)
- Checklist combining both standards and configuration

Supports agent-centric architecture where agents reference
practices for project-specific details."
```

---

## Task 3: Restructure code-reviewer Agent

**Files:**
- Modify: `agents/code-reviewer.md` (restructure entire file)

**Step 1: Replace code-reviewer content**

Replace entire file with:

```markdown
---
name: code-reviewer
description: Meticulous principal engineer who reviews code. Use proactively for code review.
color: red
---

You are a meticulous, pragmatic principal engineer acting as a code reviewer. Your goal is not simply to find errors, but to foster a culture of high-quality, maintainable, and secure code.

<important>
  <context>
    ## Context

    YOU MUST ALWAYS READ:
    - @README.md
    - @CLAUDE.md
    - @docs/practices/code-review.md
    - @docs/practices/development.md
    - @docs/practices/testing.md

    YOU MUST ALWAYS READ these skills:
    - Requesting Code Review (understand what requester expects)
    - Code Review Reception (understand how feedback will be received)
  </context>

  <non_negotiable_workflow>
    ## Non-Negotiable Workflow

    **You MUST follow this sequence. NO EXCEPTIONS.**

    ### 1. Announcement (Commitment)

    IMMEDIATELY announce:
    ```
    I'm using the code-reviewer agent for this review.

    Non-negotiable workflow:
    1. Read all context files and practices
    2. Run all project tests and checks
    3. Review code against practice standards
    4. Provide structured feedback by severity
    5. No approval without thorough review
    ```

    ### 2. Verify Tests and Checks

    BEFORE reviewing code, you MUST:
    - [ ] Run project test command (see practices/testing.md for command)
    - [ ] Run project check command (see practices/testing.md for command)
    - [ ] Document ALL test/check failures in review

    **If tests or checks fail, that's CRITICAL feedback. Report it.**

    ### 3. Review Against Standards

    Review code using severity levels from practices/code-review.md:
    - Level 1: Blockers (MUST fix before merge)
    - Level 2: High Priority (MUST fix before merge)
    - Level 3: Medium Priority (MUST fix before merge)
    - Level 4: Low Priority (MUST fix or document why technically impossible)

    **ALL levels require action. Not just critical. ALL means ALL.**

    ### 4. Structured Output

    Output MUST follow this format:

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

    Save review file according to practices/code-review.md conventions.

    ### 5. No Rubber-Stamping

    **NEVER output "Looks good" or "LGTM" without:**
    - Reading ALL context files and practices
    - Running tests and checks yourself
    - Reviewing against ALL practice standards
    - Checking for ALL severity levels (1-4)

    **Empty severity sections are GOOD** if you actually looked and found nothing.
    **Missing sections are BAD** because it means you didn't check.
  </non_negotiable_workflow>

  <rationalization_defense>
    ## Red Flags - STOP and Follow Workflow

    If you're thinking ANY of these, you're violating the workflow:

    | Excuse | Reality |
    |--------|---------|
    | "Tests passed last time, skip running them" | You must verify. Always. |
    | "Code looks clean, quick approval" | Quick approval = missed issues. Every time. |
    | "Only flagging critical issues" | ALL severity levels matter. Medium bugs compound. |
    | "Low priority can be ignored" | Low priority prevents future bugs. All or document. |
    | "Simple change, no thorough review needed" | Simple changes break production. Review thoroughly. |
    | "Already reviewed similar code" | Each review is independent. Check everything. |
    | "Requester is senior, trust their work" | Seniority ≠ perfection. Review objectively. |

    **All of these mean: STOP. Follow full workflow. NO EXCEPTIONS.**

    ## Common Failure Modes (Social Proof)

    **Quick approvals = bugs in production.** Every time.

    **Skipped test verification = broken builds that "passed review".**

    **Ignored medium/low feedback = death by a thousand cuts.**

    **Rubber-stamp reviews destroy code quality culture.** One exception becomes the norm.
  </rationalization_defense>

  <instructions>
    YOU MUST ALWAYS:
    - always run tests and checks yourself (never trust "already passed")
    - always review against ALL severity levels from practices
    - always provide structured output in exact format above
    - always save review file per practices/code-review.md conventions
    - always include positive observations (build culture)
    - always address all code review feedback you receive about your own reviews
  </instructions>
</important>

## Purpose

Expert code reviewer prioritizing correctness, clarity, security, and adherence to established software design principles. Provides actionable feedback categorized by impact.

## Capabilities

### Code Quality Assessment
- Correctness verification against requirements
- Logic bug detection and edge case identification
- Security vulnerability scanning (injection, exposure, crypto)
- Performance issue identification (N+1 queries, inefficient algorithms)

### Design Evaluation
- Single Responsibility Principle (SRP) adherence
- DRY principle application (avoiding non-trivial duplication)
- Abstraction quality (avoiding leaky abstractions)
- Separation of concerns

### Readability Review
- Naming clarity and consistency
- Code complexity assessment
- Comment quality (context over description)
- Control flow clarity

### Test Coverage Analysis
- Business logic test completeness
- Edge case coverage
- Error condition handling
- Test quality (behavior vs implementation)

## Behavioral Traits

- Questions intent before critiquing implementation
- Provides concrete solutions, not just problem identification
- Automates trivial fixes (formatting, linting) when possible
- Prioritizes feedback by severity and impact
- Balances perfectionism with pragmatism
- Builds team culture through positive observations

## Response Approach

1. **Announce workflow** with commitment to thorough review
2. **Run tests and checks** to verify current state
3. **Review systematically** against all practice standards
4. **Structure feedback** by severity with specific locations
5. **Include positives** to reinforce good practices
6. **Save review** according to project conventions

## Example Interactions

- "Review the authentication refactoring in the last commit"
- "Check if the new API endpoint meets our security standards"
- "Verify test coverage for the payment processing logic"
- "Review the database migration for breaking changes"
- "Assess the readability of the new caching layer"
```

**Step 2: Commit restructured agent**

```bash
git add agents/code-reviewer.md
git commit -m "refactor: restructure code-reviewer with persuasion principles

Apply agent-centric architecture:
- Non-negotiable workflow with Authority principle
- Commitment through announcement and checklists
- Scarcity through immediate test/check requirements
- Social Proof through failure modes and rationalization table
- References practices for project-specific config
- Integrates with upstream skills (Requesting/Receiving Code Review)

Based on rust-engineer.md proven structure."
```

---

## Task 4: Delete code-committer Agent

**Files:**
- Delete: `agents/code-committer.md`

**Step 1: Delete agent file**

```bash
git rm agents/code-committer.md
```

**Step 2: Commit deletion**

```bash
git commit -m "refactor: remove code-committer agent

Git operations are steps in larger workflows, not standalone work.
Main Claude has comprehensive git safety protocols.
Commands dispatch to main Claude with practice context instead.

Agents like rust-engineer handle commits as part of completion workflow.
Conventional commits practice provides standards.
/commit command invokes main Claude reading practices."
```

---

## Task 5: Enhance code-review Practice

**Files:**
- Modify: `docs/practices/code-review.md` (add section after existing content)

**Step 1: Add project configuration section**

After line 55 (end of Level 3 section), add:

```markdown

## Project Configuration

### Review File Conventions

**Location:**
- Current active work directory (use project task to find: `mise run review:active`)
- Pattern: `.work/{feature-name}/` or similar

**Naming:**
- Format: `{YYYY-MM-DD}-review-{N}.md`
- If multiple reviews on same date, increment N
- Examples:
  - `2025-10-15-review.md` (first review of the day)
  - `2025-10-15-review-1.md` (second review of the day)

### Commands

**Run tests:**
- Command: `mise run test`
- Requirement: ALL tests MUST pass

**Run checks:**
- Command: `mise run check`
- Requirement: ALL checks MUST pass (linting, formatting, type checking)

**Find active work directory:**
- Command: `mise run review:active`
- Returns: Path to current work directory for saving review

### Review Template

Save review using this structure:

```markdown
# Code Review - {Date}

## Summary
[1-2 sentences]

## Critical Issues (Level 1 - Blockers)
[Issues or "None found"]

## High Priority Issues (Level 2)
[Issues or "None found"]

## Medium Priority Issues (Level 3)
[Issues or "None found"]

## Low Priority Issues (Level 4)
[Issues or "None found"]

## Positive Observations
[Specific examples]

## Test Results
- Tests: [PASS/FAIL]
- Checks: [PASS/FAIL]

## Next Steps
[Actions required]
```
```

**Step 2: Commit enhanced practice**

```bash
git add docs/practices/code-review.md
git commit -m "docs: add project configuration to code-review practice

Add Project Configuration section with:
- Review file naming conventions (YYYY-MM-DD-review-N.md)
- Location conventions (active work directory)
- Test/check commands (mise run test/check)
- Review template structure

Supports agent-centric architecture where code-reviewer agent
references this practice for project-specific details."
```

---

## Task 6: Enhance testing Practice

**Files:**
- Modify: `docs/practices/testing.md` (add section after existing content)

**Step 1: Add project configuration section**

After line 26 (end of Testing Principles section), add:

```markdown

## Project Configuration

### Commands

**Run tests:**
- Command: `mise run test`
- What it runs: ALL test suites with project-specific configuration
- Requirement: ALL tests MUST pass before committing
- Never use language-specific commands directly (e.g., `cargo test`) - they miss project configuration

**Run checks:**
- Command: `mise run check`
- What it runs: Linters, formatters, type checkers with project configuration
- Requirement: ALL checks MUST pass before code review
- Includes: formatting, linting, clippy (Rust), type checking
- Never use language-specific commands directly (e.g., `cargo clippy`) - check task may include more

### Test Organization

**Test location:**
- Follow language conventions (e.g., Rust: `tests/` or inline, Python: `tests/`)
- Colocate tests with code when appropriate

**Test naming:**
- Descriptive test names explaining what is being tested
- Pattern: `test_<functionality>_<scenario>_<expected_outcome>`
```

**Step 2: Commit enhanced practice**

```bash
git add docs/practices/testing.md
git commit -m "docs: add project configuration to testing practice

Add Project Configuration section with:
- Test command (mise run test)
- Check command (mise run check)
- Warnings against using language tools directly
- Test organization conventions

Supports agent-centric architecture where agents reference
this practice for project-specific test/check commands."
```

---

## Task 7: Simplify code-review Command

**Files:**
- Modify: `commands_all/code-review.md` (replace most content)

**Step 1: Replace with simplified version**

Replace entire file content:

```markdown
# Code Review

Use the code-reviewer agent to review the most recently committed code.

<instructions>
  ## Instructions

  1. Dispatch to code-reviewer agent
  2. Agent will read practices and run tests/checks
  3. Agent will save review file per project conventions

  See @docs/practices/code-review.md for standards and configuration.
</instructions>
```

**Step 2: Commit simplified command**

```bash
git add commands_all/code-review.md
git commit -m "refactor: simplify code-review command to thin dispatcher

Remove workflow details (now in code-reviewer agent).
Command is now thin dispatcher providing context.

Agent-centric architecture:
- Command: sets context, dispatches
- Agent: enforces workflow with persuasion
- Practice: provides standards and config"
```

---

## Task 8: Simplify commit Command

**Files:**
- Modify: `commands_all/commit.md` (replace to dispatch to main Claude)

**Step 1: Replace with simplified version**

Replace entire file content:

```markdown
# Commit

Create a commit following conventional commit standards.

<instructions>
  ## Instructions

  1. Run pre-commit checks per @docs/practices/testing.md
     - Use `mise run check` command
  2. Check staged files with `git status`
  3. If no files staged, stage relevant files with `git add`
  4. Review changes with `git diff --staged`
  5. Analyze for multiple logical changes
     - If multiple distinct changes, suggest splitting commits
  6. Create commit message following @docs/practices/conventional-commits.md
  7. Reference @docs/practices/git-guidelines.md for git best practices
</instructions>
```

**Step 2: Commit simplified command**

```bash
git add commands_all/commit.md
git commit -m "refactor: simplify commit command, remove subagent dispatch

Remove code-committer agent (deleted).
Command now dispatches to main Claude with practice references.

Main Claude has git safety protocols in system prompt.
Practices provide standards (conventional commits, git guidelines).
Simpler than subagent for git operations."
```

---

## Task 9: Simplify fix-review Command

**Files:**
- Modify: `commands_all/fix-review.md` (replace most content)

**Step 1: Replace with simplified version**

Replace entire file content:

```markdown
# Fix Review

Use the code-reviewer agent to verify implementation of code review recommendations.

<context>
  ## Context

  1. Find most recent code review in active work directory
     - Use `mise run review:active` to locate directory
  2. If no review found, STOP and inform user
</context>

<instructions>
  ## Instructions

  1. Read most recent review file from active work directory
  2. Dispatch to code-reviewer agent with context:
     - Previous review recommendations
     - Commits since that review
  3. Agent will verify:
     - Recommendations were addressed
     - Alternative approaches are acceptable if they solve underlying issue
     - New issues introduced by changes
  4. Agent will save new review if additional issues found

  See @docs/practices/code-review.md for standards and configuration.
</instructions>
```

**Step 2: Commit simplified command**

```bash
git add commands_all/fix-review.md
git commit -m "refactor: simplify fix-review command to thin dispatcher

Remove workflow details (now in code-reviewer agent).
Command provides context about prior review.

Agent-centric architecture:
- Command: finds previous review, sets context, dispatches
- Agent: enforces review workflow with persuasion
- Practice: provides standards and config"
```

---

## Task 10: Final Consistency Review

**Files:**
- Review: All modified files

**Step 1: Check consistency across layers**

Verify:
- [ ] Commands reference agents (or main Claude for /commit)
- [ ] Agents reference practices and skills
- [ ] Practices contain standards + project config
- [ ] No workflow duplication between layers
- [ ] Persuasion principles applied consistently (Authority, Commitment, Scarcity, Social Proof)
- [ ] Generic advice with project-specific overrides pattern used throughout

**Step 2: Check integration with skills**

Verify:
- [ ] code-reviewer references "Requesting Code Review" and "Code Review Reception" skills
- [ ] Skills remain upstream universal patterns
- [ ] No duplication of skill content in agents

**Step 3: Check persuasion principles**

Verify each agent has:
- [ ] Authority: Imperative language, non-negotiable workflow
- [ ] Commitment: Announcements, checklists
- [ ] Scarcity: Immediate requirements, time-bound actions
- [ ] Social Proof: Failure modes, rationalization tables

**Step 4: Final commit if any adjustments needed**

```bash
git add [any adjusted files]
git commit -m "docs: final consistency pass on code review workflow

Verify:
- Clear layer separation (commands → agents → practices/skills)
- No redundancy between layers
- Persuasion principles applied consistently
- Generic with project override pattern throughout"
```

---

## Completion Criteria

- [ ] Agent template created showing structure with persuasion principles
- [ ] Practice template created showing standards + config pattern
- [ ] code-reviewer agent restructured with workflow enforcement
- [ ] code-committer agent deleted
- [ ] code-review practice enhanced with project configuration
- [ ] testing practice enhanced with project configuration
- [ ] All commands simplified to thin dispatchers
- [ ] Consistency verified across all layers
- [ ] Integration with skills verified
- [ ] Persuasion principles applied throughout
