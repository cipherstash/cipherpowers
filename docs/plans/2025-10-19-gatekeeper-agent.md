# Gatekeeper Agent Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use executing-plans to implement this plan task-by-task.

**Goal:** Add gatekeeper agent that validates code review feedback against implementation plans, preventing scope creep and derailment during /execute workflows.

**Architecture:** Three-component design with separation of concerns: (1) Simplified code-review.md practice with 2 severity levels, (2) validating-review-feedback skill containing reusable workflow, (3) gatekeeper agent that enforces the workflow and annotates reviews with [FIX]/[WONTFIX]/[DEFERRED] tags. The /execute command dispatches gatekeeper between code-reviewer and fixing agents.

**Tech Stack:** Markdown (practices, skills, agents, commands), persuasion principles for agent design (Authority, Commitment, Scarcity, Social Proof)

---

## Motivation: Real-World Failure Mode

**Recent incident:** During /execute workflow, rust-engineer agent received code review with:
- **BLOCKING**: "No Lambert recalculation at ArrivalBurn entry" (HIGH priority)
- **Recommendation**: "Option B with documentation"

**Agent's flawed reasoning:**
- "Review recommended Option B (skip + document)"
- "This means it's okay not to implement recalculation"
- "I don't even need to document it, the review explained it"
- **Result**: HIGH priority issue completely ignored, moved to next batch

**Correct interpretation:**
- HIGH priority = MUST RESOLVE
- Either implement recalculation (Option A)
- OR implement documentation (Option B)
- OR ask user which option to take
- **Cannot skip HIGH priority entirely**

**Why agent failed:**
- Confused "Recommendation: Option B" (solution approach) with "Permission to skip"
- No explicit tagging ([FIX]/[WONTFIX]/[DEFERRED]) to prevent ambiguity
- No user checkpoint to validate interpretation

**Gatekeeper prevents this by:**
1. Forcing explicit categorization of every BLOCKING item
2. Requiring user decisions on unclear/out-of-scope items
3. Annotating review with unambiguous tags ([FIX]/[WONTFIX]/[DEFERRED])
4. Ensuring fixing agents see only [FIX] items with clear instructions

---

## Task 1: Create Baseline Test (TDD: Prove Problem Exists)

**Goal:** Document baseline behavior WITHOUT gatekeeper, showing agents misinterpret review feedback.

**Files:**
- Create: `plugin/skills/collaboration/validating-review-feedback/test-scenarios.md`

**Step 1: Create skill directory structure**

```bash
mkdir -p plugin/skills/collaboration/validating-review-feedback
```

**Step 2: Write baseline test scenario**

Create `plugin/skills/collaboration/validating-review-feedback/test-scenarios.md`:

```markdown
# Test Scenarios: Validating Review Feedback

## Baseline Test: Agent Misinterprets Review Recommendations

**Goal:** Prove that without gatekeeper, agents misinterpret code review feedback as permission to skip BLOCKING issues.

### Setup

**Mock plan:**
```markdown
# Orbital Mechanics Feature

## Task 1: Add ArrivalBurn state
- Implement state transition
- Calculate burn parameters

## Task 2: Add tests for ArrivalBurn
- Unit tests for state logic
- Integration tests for burn calculations
```

**Mock review (from code-reviewer):**
```markdown
# Code Review - Batch 2

## BLOCKING (Must Fix Before Merge)

### No Lambert recalculation at ArrivalBurn entry
The current implementation assumes Lambert solution from transfer is still valid when entering ArrivalBurn state. For long transfers, this can become stale.

**Recommendation:** Option B - Add documentation explaining why recalculation is skipped for this iteration. Include TODO for future enhancement.

### Missing unit tests for state transitions
No test coverage for ArrivalBurn state entry/exit logic.

## NON-BLOCKING (Can Be Deferred)

### Variable naming: 'data' is too generic
Consider renaming to 'burnParameters' for clarity.
```

### Dispatch rust-engineer WITHOUT gatekeeper

**Prompt:**
```
Fix all BLOCKING issues found in Batch 2 code review.

Review file: {path-to-review}
Plan file: {path-to-plan}
```

### Expected Agent Failure

**Agent reasoning (flawed):**
1. Sees "Recommendation: Option B"
2. Interprets as "skip recalculation, documentation is enough"
3. Thinks "review explained it, I don't need to add docs"
4. Reports: "✅ All blocking issues resolved"

**What agent actually fixes:**
- ✅ Missing unit tests (clearly actionable)
- ❌ Lambert recalculation (skipped entirely, no documentation added)

**Why this fails:**
- BLOCKING = must resolve (either implement OR document, not skip)
- Agent confused "solution suggestion" with "permission to ignore"
- No checkpoint to validate interpretation

### Success Criteria for Baseline

❌ Agent skips HIGH priority item entirely
❌ No user consultation on ambiguous recommendation
❌ Reports completion despite unresolved BLOCKING issue

**Baseline proves:** Without gatekeeper, agents misinterpret review feedback and skip BLOCKING issues when recommendations suggest alternatives.

---

## With-Skill Test: Gatekeeper Enforces Resolution

**Goal:** Verify gatekeeper prevents the baseline failure by forcing explicit categorization and user decisions.

### Same Setup (plan + review from baseline)

### Dispatch gatekeeper agent BEFORE rust-engineer

**Prompt:**
```
Validate review feedback against plan.

Plan file: {path-to-plan}
Review file: {path-to-review}
Batch: 2
```

### Expected Gatekeeper Behavior

**Phase 1: Parse review**
- Extract 2 BLOCKING items
- Extract 1 NON-BLOCKING item

**Phase 2: Validate against plan**
- Item 1 (Lambert recalculation):
  - **Unclear**: Recommendation suggests documentation, but is that in scope?
  - Plan says "calculate burn parameters" but doesn't specify recalculation strategy
- Item 2 (missing tests):
  - **In-scope**: Task 2 explicitly requires tests

**Phase 3: Present misalignment to user**

Uses AskUserQuestion:
```
Question: "Should we address this BLOCKING issue in the current scope?"

BLOCKING Item: No Lambert recalculation at ArrivalBurn entry
Categorization: Unclear
Reasoning: Plan requires burn calculations but doesn't specify recalculation strategy. Review recommends documentation alternative.

Options:
- [FIX] Yes, fix now (add documentation as recommended)
- [WONTFIX] No, reject feedback
- [DEFERRED] Defer to follow-up
```

**User chooses: [FIX]** (accepts recommendation to document)

**Phase 4: Annotate review**

Gatekeeper modifies review file:
```markdown
## BLOCKING (Must Fix Before Merge)

### [FIX] No Lambert recalculation at ArrivalBurn entry
The current implementation assumes Lambert solution from transfer is still valid when entering ArrivalBurn state. For long transfers, this can become stale.

**Recommendation:** Option B - Add documentation explaining why recalculation is skipped for this iteration. Include TODO for future enhancement.

(Gatekeeper: In-scope - user approved Option B documentation approach)

### [FIX] Missing unit tests for state transitions
No test coverage for ArrivalBurn state entry/exit logic.

(Gatekeeper: In-scope - Task 2 requires tests)

## NON-BLOCKING (Can Be Deferred)

(Gatekeeper: All NON-BLOCKING items deferred by default)

### [DEFERRED] Variable naming: 'data' is too generic
Consider renaming to 'burnParameters' for clarity.
```

**Phase 5: Update plan**

No deferred items (user approved both BLOCKING as [FIX]).

**Phase 6: Return summary**

```
Gatekeeper Validation Complete - Batch 2

BLOCKING Items:
- 2 marked [FIX] (Lambert documentation, unit tests)
- 0 marked [DEFERRED]
- 0 marked [WONTFIX]

NON-BLOCKING Items:
- 1 marked [DEFERRED] (variable naming)

Plan Status:
- Deferred items added: no
- Plan revision needed: no

Files Updated:
- Annotated review: {review-path}

Next Steps for Orchestrator:
Proceed to rust-engineer with annotated review. Fix ONLY [FIX] items.
```

### Dispatch rust-engineer WITH annotated review

**Prompt:**
```
Fix ONLY items marked [FIX] in the annotated review.
Do NOT address items marked [DEFERRED] or [WONTFIX].

Review file: {path-to-annotated-review}
```

### Expected Agent Success

**Agent sees:**
- [FIX] Lambert recalculation → Add Option B documentation
- [FIX] Missing tests → Write unit tests
- [DEFERRED] Variable naming → SKIP

**Agent reasoning:**
1. Clear [FIX] tag = must address
2. Review includes "Option B documentation" recommendation
3. Implements: Add doc comment explaining no recalculation + TODO
4. Implements: Add unit tests
5. Reports: "✅ All [FIX] items resolved"

**What agent actually fixes:**
- ✅ Lambert recalculation (documentation added per Option B)
- ✅ Missing unit tests
- ⏭️ Variable naming (correctly skipped, marked [DEFERRED])

### Success Criteria

✅ Gatekeeper identifies unclear item (Lambert recalculation)
✅ Gatekeeper uses AskUserQuestion (not auto-deciding)
✅ User explicitly approves Option B approach
✅ Review annotated with [FIX] tags and clarifying notes
✅ Rust-engineer sees unambiguous instructions
✅ Both BLOCKING items resolved correctly

**With-skill proves:** Gatekeeper prevents misinterpretation by forcing explicit categorization and user validation of ambiguous feedback.

---

## Additional Test Scenario: Scope Creep Prevention

**Goal:** Verify gatekeeper blocks out-of-scope BLOCKING feedback from derailing plan.

### Setup

**Mock plan:**
```markdown
# Auth Feature Plan

## Task 1: Add basic username/password auth
- Login endpoint
- Password hashing
- Session creation

## Task 2: Add session validation middleware
- Check session on protected routes
- Return 401 if invalid
```

**Mock review:**
```markdown
# Code Review - 2025-10-19

## BLOCKING (Must Fix Before Merge)

### Security vulnerability: passwords stored in plain text
The current implementation stores passwords without hashing.

### SRP violation: auth handler does too much
The handleAuth function validates input, hashes passwords, creates sessions, and writes to DB. Should be split into separate functions.

### Missing tests for session validation
No test coverage for the middleware in Task 2.

## NON-BLOCKING (Can Be Deferred)

### Variable naming: 'data' is too generic
Consider renaming to 'userData' for clarity.
```

### Expected Gatekeeper Behavior

**Validation:**
- Security vulnerability → In-scope (Task 1 requires password hashing)
- SRP violation → **Out-of-scope** (plan doesn't mention code architecture refactoring)
- Missing tests → In-scope (Task 2 mentioned)

**User question for SRP violation:**
```
BLOCKING Item: SRP violation: auth handler does too much
Categorization: Out-of-scope
Reasoning: Plan focuses on basic auth implementation. Architectural refactoring not mentioned in plan scope.

Options:
- [FIX] Yes, fix now
- [WONTFIX] No, reject feedback
- [DEFERRED] Defer to follow-up
```

**User chooses: [DEFERRED]**

**Annotated review:**
```markdown
## BLOCKING (Must Fix Before Merge)

### [FIX] Security vulnerability: passwords stored in plain text
...
(Gatekeeper: In-scope - Task 1 requires password hashing)

### [DEFERRED] SRP violation: auth handler does too much
...
(Gatekeeper: Out-of-scope - architectural refactoring not in current plan)

### [FIX] Missing tests for session validation
...
(Gatekeeper: In-scope - Task 2 requires tests)
```

**Plan updated with Deferred section:**
```markdown
---

## Deferred Items

### From Batch 1 Review (2025-10-19-review.md)
- **[DEFERRED]** SRP violation in auth handler
  - Source: Task 1
  - Severity: BLOCKING (architectural)
  - Reason: Out of scope for basic auth implementation
```

### Success Criteria

✅ Gatekeeper identifies SRP violation as out-of-scope
✅ User makes explicit decision to defer
✅ Deferred item tracked in plan
✅ Rust-engineer fixes only 2 items ([FIX]), skips SRP violation
✅ Plan remains focused on original scope

**Proves:** Gatekeeper prevents scope creep by getting user validation before adding work beyond plan.
```

**Step 3: Verify test file**

Read created test scenarios.

Run: `cat plugin/skills/collaboration/validating-review-feedback/test-scenarios.md`

**Step 4: Commit**

```bash
git add plugin/skills/collaboration/validating-review-feedback/test-scenarios.md
git commit -m "test: add baseline and with-skill test scenarios for gatekeeper"
```

---

## Task 2: Simplify Code Review Practice to Two Severity Levels

**Goal:** Consolidate 4-level severity system (L1-L4) into 2-level system (BLOCKING/NON-BLOCKING) where BLOCKING = L1+L2 and NON-BLOCKING = L3+L4.

**Files:**
- Modify: `plugin/practices/code-review.md`

**Step 1: Verify current structure**

Check if file has expected severity section before modifying.

```bash
grep -in "Review Checklist & Severity" plugin/practices/code-review.md
head -80 plugin/practices/code-review.md | tail -60
```

Expected: Four-level severity system (Level 1-4) around lines 21-69

**Step 2: Read current code-review.md**

Verify current structure before modifying.

Run: `cat plugin/practices/code-review.md`

**Step 3: Update severity section**

Replace the four-level severity section with two levels:

```markdown
## Review Checklist & Severity

You will evaluate code and categorize feedback into the following severity levels.

### BLOCKING (Must Fix Before Merge)

**Security & Correctness:**
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

**Architecture & Performance:**
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

### NON-BLOCKING (Can Be Deferred or Follow-up)

**Clarity & Documentation:**
-   Ambiguous or misleading variable, function, or class names.
-   Overly complex conditional logic that could be simplified or refactored into smaller functions.
-   "Magic numbers" or hardcoded strings that should be named constants.
-   Lack of comments for complex, non-obvious algorithms or business logic.
-   Missing doc comments for public-facing functions.

**Polish:**
-   Style preferences (minor naming improvements, formatting not caught by linters).
-   Minor optimizations (performance improvements with negligible impact).
-   Future considerations (suggestions for future refactoring).
-   Mise run check issues that don't affect functionality (linting, formatting that can be auto-fixed).
```

**Step 4: Update review template**

Find and replace template section with simplified two-section structure:

```markdown
### Review Template

Save review using this structure:

​```markdown
# Code Review - {Date}

## Summary
[1-2 sentences]

## BLOCKING (Must Fix Before Merge)
[Issues or "None found"]

## NON-BLOCKING (Can Be Deferred)
[Issues or "None found"]

## Positive Observations
[Specific examples]

## Test Results
- Tests: [PASS/FAIL]
- Checks: [PASS/FAIL]

## Next Steps
[Actions required]
​```
```

**Step 5: Verify changes**

Read the modified file to ensure formatting is correct.

Run: `cat plugin/practices/code-review.md`
Expected: Two severity levels (BLOCKING/NON-BLOCKING), updated template

**Step 6: Commit**

```bash
git add plugin/practices/code-review.md
git commit -m "refactor: simplify code review severity to 2 levels (BLOCKING/NON-BLOCKING)"
```

---

## Task 3: Create Validating Review Feedback Skill

**Goal:** Create reusable skill documenting the workflow for validating code review feedback against implementation plans.

**Files:**
- Create: `plugin/skills/collaboration/validating-review-feedback/SKILL.md`

**Step 1: Write skill file**

Create `plugin/skills/collaboration/validating-review-feedback/SKILL.md`:

```markdown
---
name: Validating Review Feedback
description: Validate code review feedback against implementation plan to prevent scope creep and derailment
when_to_use: when orchestrating plan execution with code review checkpoints, after receiving review feedback, before dispatching fixes to agents
related_skills: conducting-code-review
related_practices: code-review.md
version: 1.0.0
---

# Validating Review Feedback

## Overview

When orchestrating plan execution, code review feedback must align with the plan's goals. This workflow validates BLOCKING feedback against the plan, gets user decisions on misalignments, and annotates the review file to guide fixing agents.

**Core principle:** User decides scope changes, not the agent. Validate → Ask → Annotate.

**Announce at start:** "I'm using the validating-review-feedback skill to validate this review against the plan."

## The Workflow

### Phase 1: Parse Review Feedback

**Step 1: Read review file**
- Path provided by orchestrator
- Expected format: BLOCKING and NON-BLOCKING sections

**Step 2: Extract items**
- Parse all BLOCKING items into list
- Parse all NON-BLOCKING items (for awareness only)
- Preserve original wording and line numbers

### Phase 2: Validate Against Plan

**Step 1: Read plan file**
- Path provided by orchestrator
- Understand original scope and goals

**Step 2: Categorize each BLOCKING item**

For each BLOCKING item, determine:

- **In-scope**: Required by plan OR directly supports plan goals OR fixes bugs introduced while implementing plan
- **Out-of-scope**: Would require work beyond current plan (new features, refactoring unrelated code, performance optimizations not in plan)
- **Unclear**: Needs user judgment (edge cases, architectural decisions, ambiguous recommendations)

**Step 3: Document reasoning**

For each categorization, note brief reasoning:
- In-scope: "Task 3 requires auth validation"
- Out-of-scope: "SRP refactoring not in plan scope"
- Unclear: "Review recommends documentation alternative - needs user decision"

### Phase 3: Present Misalignments to User

**When:** Any BLOCKING items categorized as out-of-scope or unclear

**Step 1: Show misaligned items**

For each misaligned item:
```
BLOCKING Item: [exact text from review]
Categorization: [Out-of-scope / Unclear]
Reasoning: [why it doesn't clearly align with plan]
```

**Step 2: Ask user about each item**

Use AskUserQuestion for each misaligned BLOCKING item:

```
Question: "Should we address this BLOCKING issue in the current scope?"
Options:
  - "[FIX] Yes, fix now" (Add to current scope)
  - "[WONTFIX] No, reject feedback" (User disagrees with review)
  - "[DEFERRED] Defer to follow-up" (Valid but out of scope)
```

**Step 3: Check for plan revision**

If user selected [DEFERRED] for multiple items or items seem interconnected:
- Ask: "Do you want to revise the plan to accommodate these deferred items?"
- If yes: Set `plan_revision_needed` flag

### Phase 4: Annotate Review File

**Step 1: Add tags to BLOCKING items**

For each BLOCKING item in original review file:
- Prepend `[FIX]` if in-scope or user approved
- Prepend `[WONTFIX]` if user rejected
- Prepend `[DEFERRED]` if user deferred

**Step 2: Add clarifying notes**

For each tagged item, add Gatekeeper note explaining categorization:
- `(Gatekeeper: In-scope - {reasoning})`
- `(Gatekeeper: Out-of-scope - {reasoning})`
- `(Gatekeeper: User approved - {decision})`

**Step 3: Tag all NON-BLOCKING items**

- Prepend `[DEFERRED]` to all NON-BLOCKING items
- Add note: "(Gatekeeper: NON-BLOCKING items deferred by default)"

**Step 4: Write annotated review**

Save back to same review file path with annotations.

Example annotated review:

```markdown
# Code Review - 2025-10-19

## Summary
Found 3 BLOCKING issues and 2 NON-BLOCKING suggestions.

## BLOCKING (Must Fix Before Merge)

### [FIX] Security vulnerability in auth endpoint
Missing input validation on user-provided email parameter allows potential injection attacks.
(Gatekeeper: In-scope - required by Task 2 auth implementation)

### [DEFERRED] SRP violation in data processing module
The processUserData function handles both validation and database writes.
(Gatekeeper: Out-of-scope - refactoring not in current plan)

### [FIX] Missing tests for preference storage
No test coverage for the new user preference persistence logic.
(Gatekeeper: In-scope - Task 3 requires test coverage)

## NON-BLOCKING (Can Be Deferred)

(Gatekeeper: All NON-BLOCKING items deferred by default)

### [DEFERRED] Ambiguous variable name in utils
The variable 'data' in formatUserData could be more descriptive like 'userData'.

### [DEFERRED] Consider extracting magic number
The timeout value 5000 appears in multiple places.
```

### Phase 5: Update Plan with Deferred Items

**When:** Any items marked [DEFERRED]

**Step 1: Check if plan has Deferred section**

- Read plan file
- Look for `## Deferred Items` section

**Step 2: Create or append to Deferred section**

Add to end of plan file:

```markdown
---

## Deferred Items

Items deferred during code review - must be reviewed before merge.

### From Batch N Review ({review-filename})
- **[DEFERRED]** {Item description}
  - Source: Task X
  - Severity: BLOCKING or NON-BLOCKING
  - Reason: {why deferred}
```

**Step 3: Save updated plan**

Write plan file with deferred items section.

### Phase 6: Return Summary

Provide summary to orchestrator:

```
Validation complete:
- {N} BLOCKING items marked [FIX] (ready for fixing agent)
- {N} BLOCKING items marked [DEFERRED] (added to plan)
- {N} BLOCKING items marked [WONTFIX] (rejected by user)
- {N} NON-BLOCKING items marked [DEFERRED]
- Plan revision needed: {yes/no}

Annotated review saved to: {review-file-path}
Plan updated with deferred items: {plan-file-path}
```

## Key Principles

| Principle | Application |
|-----------|-------------|
| **User decides scope** | Never auto-approve out-of-scope items, always ask |
| **Annotate in place** | Modify review file with tags, don't create new files |
| **Track deferrals** | All deferred items must go in plan's Deferred section |
| **Clear communication** | Tags ([FIX]/[WONTFIX]/[DEFERRED]) guide fixing agent |
| **No silent filtering** | User must explicitly decide on every misalignment |

## Error Handling

**No BLOCKING items found:**
- Still tag all NON-BLOCKING as [DEFERRED]
- Return summary indicating clean review

**User marks all BLOCKING as [WONTFIX]:**
- Annotate review accordingly
- Return to orchestrator with plan_revision_needed suggestion
- Orchestrator should pause and ask user about plan validity

**Plan file not found:**
- Error immediately
- Cannot validate without plan context

**Review file not parseable:**
- Error immediately
- Show user the review file format issue

## Integration

**Called by:**
- Gatekeeper agent (enforces this workflow)
- /execute command (via gatekeeper dispatch)

**Requires:**
- Plan file path (from orchestrator)
- Review file path (from code-reviewer agent)

**Produces:**
- Annotated review file (with [FIX]/[WONTFIX]/[DEFERRED] tags)
- Updated plan file (with Deferred Items section)
- Summary for orchestrator

## Test Scenarios

See `test-scenarios.md` for baseline and with-skill tests proving this workflow prevents scope creep and misinterpretation.
```

**Step 2: Verify skill file**

Read the created file to ensure proper formatting.

Run: `cat plugin/skills/collaboration/validating-review-feedback/SKILL.md`

**Step 3: Commit**

```bash
git add plugin/skills/collaboration/validating-review-feedback/SKILL.md
git commit -m "feat: add validating-review-feedback skill"
```

---

## Task 4: Create Gatekeeper Agent

**Goal:** Create gatekeeper agent that enforces the validating-review-feedback workflow using persuasion principles.

**Files:**
- Create: `plugin/agents/gatekeeper.md`

**Step 1: Read agent template**

Reference the template structure for consistency.

Run: `cat plugin/templates/agent-template.md`

**Step 2: Create gatekeeper agent file**

Create `plugin/agents/gatekeeper.md`:

```markdown
# Gatekeeper Agent

You are the **Gatekeeper** - the quality gate between code review and implementation.

Your role: Validate code review feedback against the implementation plan, prevent scope creep, and ensure only in-scope work proceeds to fixing agents.

---

## Authority Principle: Non-Negotiable Workflow

YOU MUST follow this exact workflow. No exceptions. No shortcuts.

### Step 1: Announce and Read

**ANNOUNCE:**
"I'm the Gatekeeper agent. I'm using the validating-review-feedback skill to validate this review against the plan."

**READ these files in order:**

1. **Validation workflow (REQUIRED):**
   @${CLAUDE_PLUGIN_ROOT}plugin/skills/collaboration/validating-review-feedback/SKILL.md

2. **Severity definitions (REQUIRED):**
   @${CLAUDE_PLUGIN_ROOT}plugin/practices/code-review.md

3. **Plan file (path in prompt):**
   Read to understand scope and goals

4. **Review file (path in prompt):**
   Read to extract BLOCKING and NON-BLOCKING items

### Step 2: Execute Validation Workflow

Follow the validating-review-feedback skill workflow EXACTLY:

1. **Parse** review feedback (BLOCKING vs NON-BLOCKING)
2. **Validate** each BLOCKING item against plan (in-scope / out-of-scope / unclear)
3. **Present** misalignments to user via AskUserQuestion
4. **Annotate** review file with [FIX] / [WONTFIX] / [DEFERRED] tags
5. **Update** plan file with Deferred Items section
6. **Return** summary to orchestrator

### Step 3: Return Control

After annotation complete:
- Provide summary (X items [FIX], Y items [DEFERRED], etc.)
- Indicate if plan revision needed
- End agent execution (orchestrator decides next steps)

---

## Commitment Principle: Track Progress

**BEFORE starting validation, create TodoWrite todos:**

```
Gatekeeper Validation:
- [ ] Read validation skill and code review practice
- [ ] Parse review feedback (BLOCKING/NON-BLOCKING)
- [ ] Validate BLOCKING items against plan
- [ ] Present misalignments to user
- [ ] Annotate review file with tags
- [ ] Update plan with deferred items
- [ ] Return summary to orchestrator
```

**Mark each todo complete as you finish it.**

---

## Scarcity Principle: One Job Only

You have ONE job: **Validate review feedback against the plan.**

### What You DO:
✅ Read plan and review files
✅ Categorize BLOCKING items (in-scope / out-of-scope / unclear)
✅ Ask user about misalignments
✅ Annotate review file with [FIX] / [WONTFIX] / [DEFERRED]
✅ Update plan with deferred items
✅ Return summary

### What You DON'T Do:
❌ Fix code yourself
❌ Propose alternative solutions to review feedback
❌ Add scope beyond the plan
❌ Skip user questions to "save time"
❌ Make scope decisions on behalf of the user
❌ Dispatch other agents
❌ Modify the plan scope (only add Deferred section)

---

## Social Proof Principle: Failure Modes

**Without this validation, teams experience:**

1. **Misinterpreted Recommendations** (Real incident)
   - Review says "Option B - Add documentation"
   - Agent thinks "skip implementation, no doc needed"
   - HIGH priority issue ignored completely
   - **Gatekeeper prevents:** Forces [FIX] tag + user validation of unclear recommendations

2. **Scope Creep**
   - "Just one more refactoring" turns into 3 days of work
   - Plan goals lost in well-intentioned improvements
   - **Gatekeeper prevents:** Out-of-scope items require explicit user approval

3. **Derailed Plans**
   - Review suggests performance optimization not in plan
   - Engineer spends week optimizing instead of finishing features
   - **Gatekeeper prevents:** [DEFERRED] tag + plan tracking

4. **Exhaustion-Driven Acceptance**
   - Engineer too tired to push back on out-of-scope feedback
   - "Fine, I'll fix it" leads to never-ending review cycles
   - **Gatekeeper prevents:** User makes scope decisions upfront, not agent under pressure

5. **Lost Focus**
   - Original plan goals forgotten
   - Feature ships late because of unrelated improvements
   - **Gatekeeper prevents:** Plan remains source of truth, deferred items tracked separately

**Your validation prevents these failures.**

---

## Rationalization Defenses

### "This BLOCKING issue is obviously in scope"
**→ NO.** Ask the user. What's "obvious" to you may not align with user's goals. You don't make scope decisions.

### "The review says 'Option B' so I should mark it [DEFERRED]"
**→ NO.** "Option B" is a recommended solution approach, not permission to skip. If unclear, ask user: [FIX] with Option B, [DEFERRED], or [WONTFIX]?

### "The review has no BLOCKING items, I can skip validation"
**→ NO.** Still parse and annotate. Tag all NON-BLOCKING items as [DEFERRED] and update plan if needed.

### "The user is busy, I won't bother them with questions"
**→ NO.** User questions prevent scope creep. A 30-second question saves 3 hours of misdirected work. Always ask about misalignments.

### "This item is clearly wrong, I'll mark it [WONTFIX] automatically"
**→ NO.** User decides what feedback to accept or reject. Present it and let them choose.

### "I'll just add a note instead of using AskUserQuestion"
**→ NO.** Use AskUserQuestion for misaligned BLOCKING items. Notes get ignored. Explicit questions get answers.

### "The plan is wrong, I'll update it to match the review"
**→ NO.** Plan defines scope. Review doesn't override plan. If plan needs revision, user decides.

### "I can combine asking about multiple items into one question"
**→ NO.** Ask about each misaligned BLOCKING item separately using AskUserQuestion. Bundling forces user to accept/reject as a group.

---

## Required Input (Provided by Orchestrator)

You will receive in your prompt:

```
Plan file: {absolute-path-to-plan.md}
Review file: {absolute-path-to-review.md}
Batch number: {N}
```

**If any input missing:**
- Error immediately
- Do NOT proceed without plan and review paths

---

## Output Format

After completing validation, return this summary:

```
Gatekeeper Validation Complete - Batch {N}

BLOCKING Items:
- {N} marked [FIX] (in-scope, ready for fixing agent)
- {N} marked [DEFERRED] (out-of-scope, added to plan)
- {N} marked [WONTFIX] (rejected by user)

NON-BLOCKING Items:
- {N} marked [DEFERRED] (auto-deferred)

Plan Status:
- Deferred items added: {yes/no}
- Plan revision needed: {yes/no}

Files Updated:
- Annotated review: {review-file-path}
- Updated plan: {plan-file-path}

Next Steps for Orchestrator:
{Recommended action: proceed to fixing, pause for plan revision, etc.}
```

---

## Example Interaction

**Orchestrator provides:**
```
Plan file: /Users/dev/project/.worktrees/auth/docs/plans/2025-10-19-auth.md
Review file: /Users/dev/project/.worktrees/auth/.work/auth/2025-10-19-review.md
Batch number: 2
```

**You execute:**
1. Read validation skill
2. Read code review practice
3. Read plan file (understand scope: add basic auth, no fancy features)
4. Read review file (3 BLOCKING items, 2 NON-BLOCKING)
5. Validate:
   - Item 1: "Missing input validation" → In-scope (Task 1 requires validation)
   - Item 2: "SRP violation in auth handler" → Out-of-scope (refactoring not in plan)
   - Item 3: "Missing tests" → In-scope (Task 2 requires tests)
6. Present Item 2 to user via AskUserQuestion
7. User chooses [DEFERRED]
8. Annotate review:
   - Item 1: [FIX]
   - Item 2: [DEFERRED]
   - Item 3: [FIX]
   - All NON-BLOCKING: [DEFERRED]
9. Update plan with Deferred section
10. Return summary

**You return:**
```
Gatekeeper Validation Complete - Batch 2

BLOCKING Items:
- 2 marked [FIX] (input validation, missing tests)
- 1 marked [DEFERRED] (SRP violation)
- 0 marked [WONTFIX]

NON-BLOCKING Items:
- 2 marked [DEFERRED] (variable naming, magic numbers)

Plan Status:
- Deferred items added: yes
- Plan revision needed: no

Files Updated:
- Annotated review: /Users/dev/project/.worktrees/auth/.work/auth/2025-10-19-review.md
- Updated plan: /Users/dev/project/.worktrees/auth/docs/plans/2025-10-19-auth.md

Next Steps for Orchestrator:
Proceed to fixing agent with annotated review. Fix only [FIX] items.
```

---

## Success Criteria

You succeed when:
✅ All BLOCKING items have tags ([FIX] / [WONTFIX] / [DEFERRED])
✅ All NON-BLOCKING items tagged [DEFERRED]
✅ User explicitly decided on every out-of-scope or unclear BLOCKING item
✅ Plan updated with deferred items
✅ Clear summary provided to orchestrator

You fail when:
❌ BLOCKING items lack tags
❌ Scope decision made without user input
❌ Deferred items not added to plan
❌ Validation skipped because "review looks clean"
❌ "Option B" recommendation misinterpreted as permission to skip
```

**Step 3: Verify agent file**

Read the created file to ensure proper formatting.

Run: `cat plugin/agents/gatekeeper.md`

**Step 4: Commit**

```bash
git add plugin/agents/gatekeeper.md
git commit -m "feat: add gatekeeper agent with persuasion principles"
```

---

## Task 5: Update Execute Command to Dispatch Gatekeeper

**Goal:** Integrate gatekeeper agent dispatch into /execute command's batch review checkpoint workflow.

**Files:**
- Modify: `plugin/commands/execute.md`

**Step 1: Verify current structure**

Check if execute.md has "Batch Review Checkpoint" section.

```bash
grep -n "Batch Review Checkpoint\|batch.*review\|code-reviewer" plugin/commands/execute.md | head -10
```

Expected: Section describing batch review workflow

**Step 2: Read current execute command**

Understand current batch checkpoint flow.

Run: `cat plugin/commands/execute.md`

**Step 3: Locate batch checkpoint workflow**

Find the section describing what happens after each batch completion (likely includes code-reviewer dispatch).

If "Batch Review Checkpoint" section exists, modify it.
If not, add new section after batch completion logic.

**Step 4: Add gatekeeper dispatch after code-reviewer**

Insert gatekeeper workflow between code-reviewer and fixing agent dispatch:

```markdown
### Batch Review Checkpoint

After completing batch N:

**Step 1: Dispatch code-reviewer agent**

Reviews all changes in batch using code review practice.

Dispatch: code-reviewer agent
Input: Changed files in batch
Output: Review file at `{work-dir}/{YYYY-MM-DD}-review.md`

**Step 2: Dispatch gatekeeper agent**

Validates review feedback against plan, gets user decisions on scope.

Dispatch: gatekeeper agent
Input:
- Plan file: `{work-dir}/plan.md` or `docs/plans/{YYYY-MM-DD}-{feature}.md`
- Review file: `{work-dir}/{YYYY-MM-DD}-review.md` (from code-reviewer)
- Batch number: {N}

Output: Annotated review file with [FIX]/[WONTFIX]/[DEFERRED] tags

**Step 3: Read annotated review**

Parse the gatekeeper's annotated review file.

Extract count of [FIX] items (items that must be addressed now).

**Step 4: Handle validation results**

**If gatekeeper reports "plan revision needed":**
- Pause execution
- Show user the deferred items
- Ask: "Plan revision needed based on deferred feedback. Update plan and resume, or continue?"
- Wait for user decision (5 minute timeout)
- **Timeout behavior:** If no response within 5 minutes, mark workflow as FAILED and halt execution

**If 0 items marked [FIX]:**
- Announce: "Batch {N} review clean - no blocking issues to fix"
- Proceed to batch {N+1}

**If >0 items marked [FIX]:**
- Dispatch fixing agent with:
  - Annotated review file path
  - Instruction: "Fix ONLY items marked [FIX]. Do not address [DEFERRED] or [WONTFIX] items."
- After fixes applied:
  - Run tests: `mise run test`
  - Run checks: `mise run check`
  - If pass → Continue to batch {N+1}
  - If fail → Repeat from Step 1 (new review cycle with incremented review filename)

**Step 5: Track deferred items**

Maintain running list of deferred items across all batches (stored in plan's Deferred section by gatekeeper).

After final batch, show summary:
```
Execution Complete

Total deferred items: {N} BLOCKING + {M} NON-BLOCKING
See plan file Deferred section for details.

Next: Address deferred items or create follow-up tasks?
```
```

**Step 5: Verify changes**

Read the modified section to ensure proper integration.

Run: `grep -A 40 "Batch Review Checkpoint" plugin/commands/execute.md`

**Step 6: Commit**

```bash
git add plugin/commands/execute.md
git commit -m "feat: integrate gatekeeper agent into /execute batch checkpoints"
```

---

## Task 6: Update Code-Reviewer Agent References

**Goal:** Update code-reviewer agent to reference simplified 2-level severity system.

**Files:**
- Modify: `plugin/agents/code-reviewer.md`

**Step 1: Read code-reviewer agent**

Check current severity level references.

Run: `cat plugin/agents/code-reviewer.md`

**Step 2: Find severity references**

Search for mentions of "Level 1", "Level 2", "Level 3", "Level 4" or numbered severity levels.

Run: `grep -n "Level [1-4]\|L[1-4]" plugin/agents/code-reviewer.md || echo "No numbered levels found"`

**Step 3: Update severity language**

Replace any references to 4-level system with 2-level system:

- "Level 1" or "L1" or "Blocker" → "BLOCKING"
- "Level 2" or "L2" or "High Priority" → "BLOCKING"
- "Level 3" or "L3" or "Medium Priority" → "NON-BLOCKING"
- "Level 4" or "L4" or "Low Priority" → "NON-BLOCKING"

Ensure agent references:
```markdown
**Review standards:**
@${CLAUDE_PLUGIN_ROOT}plugin/practices/code-review.md
(Use BLOCKING vs NON-BLOCKING categories)
```

**Step 4: Update template reference**

Ensure agent uses 2-section template:
```markdown
## BLOCKING (Must Fix Before Merge)
[Issues or "None found"]

## NON-BLOCKING (Can Be Deferred)
[Issues or "None found"]
```

**Step 5: Verify changes**

Read the updated agent file.

Run: `cat plugin/agents/code-reviewer.md | grep -A 5 "BLOCKING\|NON-BLOCKING"`

**Step 6: Commit**

```bash
git add plugin/agents/code-reviewer.md
git commit -m "refactor: update code-reviewer agent to use 2-level severity system"
```

---

## Task 7: Verify Rust-Engineer Agent (Check for Updates)

**Goal:** Check if rust-engineer agent references code review practice and update if necessary.

**Files:**
- Read: `plugin/agents/rust-engineer.md`
- Modify if needed: `plugin/agents/rust-engineer.md`

**Step 1: Read rust-engineer agent**

Check if it references code-review.md or severity levels.

Run: `cat plugin/agents/rust-engineer.md`

**Step 2: Search for code review references**

Run: `grep -n "code-review\|Level [1-4]\|L[1-4]\|severity" plugin/agents/rust-engineer.md || echo "No code review references found"`

**Step 3: Update if needed**

**If rust-engineer references code review:**
- Update any 4-level severity mentions to 2-level system
- Ensure practice reference is correct: `@${CLAUDE_PLUGIN_ROOT}plugin/practices/code-review.md`
- Commit changes

**If no code review references found:**
- No changes needed
- Document in commit message

**Step 4: Commit**

```bash
# If changes made:
git add plugin/agents/rust-engineer.md
git commit -m "refactor: update rust-engineer code review references to 2-level system"

# If no changes needed:
git commit --allow-empty -m "docs: verify rust-engineer agent - no code review updates needed"
```

---

## Verification Steps

After completing all tasks:

**Step 1: Verify plugin environment**

Confirm plugin is loaded correctly with required environment variables.

```bash
echo "CLAUDE_PLUGIN_ROOT: ${CLAUDE_PLUGIN_ROOT}"
echo "SUPERPOWERS_SKILLS_ROOT: ${SUPERPOWERS_SKILLS_ROOT}"
```

Expected: Both variables set to plugin installation paths

**Step 2: Verify file structure**

```bash
ls -la plugin/practices/code-review.md
ls -la plugin/skills/collaboration/validating-review-feedback/SKILL.md
ls -la plugin/skills/collaboration/validating-review-feedback/test-scenarios.md
ls -la plugin/agents/gatekeeper.md
ls -la plugin/commands/execute.md
```

Expected: All files exist

**Step 3: Verify commit history**

```bash
git log --oneline -8
```

Expected: 7-8 commits following conventional commit format

**Step 4: Check for consistency**

```bash
grep -r "Level [1-4]\|L[1-4]" plugin/agents/ plugin/practices/ || echo "No old severity levels found - good!"
```

Expected: No matches (all updated to BLOCKING/NON-BLOCKING)

**Step 5: Verify references**

```bash
grep -r "validating-review-feedback" plugin/agents/gatekeeper.md
grep -r "CLAUDE_PLUGIN_ROOT" plugin/agents/gatekeeper.md | head -3
```

Expected:
- Gatekeeper agent references skill
- Uses correct environment variable paths set by plugin system

**Step 6: Run test scenario (manual)**

Follow test scenarios in `plugin/skills/collaboration/validating-review-feedback/test-scenarios.md`:
1. Run baseline test (agent without gatekeeper)
2. Run with-skill test (agent with gatekeeper)
3. Verify gatekeeper prevents misinterpretation and scope creep

---

## Notes

**Key dependencies:**
- Gatekeeper agent depends on validating-review-feedback skill
- Both depend on updated code-review.md practice
- /execute command depends on gatekeeper agent
- Test scenarios validate the entire workflow

**Testing approach:**
- TDD: Test scenarios written BEFORE implementation
- Baseline test proves problem exists without gatekeeper
- With-skill tests verify gatekeeper prevents failures
- Manual testing with mock files initially
- Future: Could automate with subagent dispatch tests

**Real-world validation:**
- Test scenarios based on actual incident (Lambert recalculation)
- Failure mode: Agent misinterpreted "Option B" as permission to skip
- Gatekeeper prevents: Forces explicit [FIX] tagging + user validation

**Future enhancements:**
- Expand gatekeeper role to other quality gates (security checks, performance thresholds)
- Add metrics tracking (how many items deferred vs fixed)
- Integration with issue tracking (auto-create tickets for deferred items)
- Automated test runner for test scenarios
