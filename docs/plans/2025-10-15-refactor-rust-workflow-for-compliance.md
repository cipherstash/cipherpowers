# Rust Workflow Refactor for Compliance Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Refactor Rust development workflow to ensure agents reliably follow TDD, code review, testing, and quality check requirements using persuasion principles.

**Architecture:** Consolidate all workflow instructions into the rust-engineer agent using Authority, Commitment, Scarcity, and Social Proof persuasion principles. Eliminate redundant command. Streamline practices to be concise reference documents.

**Tech Stack:** Markdown documentation, persuasion-principles patterns, superpowers skill framework

---

## Problem Analysis

**Current Issues:**
1. Agents skip code review despite instructions
2. Agents don't address all review feedback priorities (critical, high, medium, low)
3. Agents don't run project tasks (`mise run test`, `mise run check`)
4. Agents use weak cargo commands instead of mise
5. Instructions fragmented across 3 files (practices, agent, command)
6. Weak authority language enables rationalization
7. No commitment mechanism or explicit checkpoints
8. Missing failure mode documentation (social proof)

**Root Cause:** Instructions lack persuasion principles proven effective for LLM compliance (Meincke et al., 2025).

---

## Task 1: Update rust-engineer.md with Persuasion Principles

**Files:**
- Modify: `agents/rust-engineer.md`
- Reference: `/Users/tobyhede/.config/superpowers/skills/skills/meta/writing-skills/persuasion-principles.md`
- Reference: `/Users/tobyhede/.config/superpowers/skills/skills/meta/writing-skills/SKILL.md`

**Step 1: Read persuasion principles**

Review how to apply:
- **Authority**: Imperative language ("YOU MUST"), "NO EXCEPTIONS" lists
- **Commitment**: Required announcements, explicit choices
- **Scarcity**: "IMMEDIATELY after", "BEFORE proceeding"
- **Social Proof**: "X without Y = failure" patterns

**Step 2: Add Non-Negotiable Workflow section**

Insert after `<context>` block, before `</important>`:

```markdown
    <non_negotiable_workflow>
      ## Non-Negotiable Workflow

      **You MUST follow this sequence. NO EXCEPTIONS.**

      ### 1. Announcement (Commitment)

      IMMEDIATELY announce:
      ```
      I'm using the rust-engineer agent for [specific task].

      Non-negotiable workflow:
      1. Verify worktree and read all context
      2. Implement with TDD
      3. Run `mise run test` - ALL tests MUST pass
      4. Run `mise run check` - ALL checks MUST pass
      5. Request code review BEFORE claiming completion
      6. Address ALL review feedback (critical, high, medium, low)
      ```

      ### 2. Pre-Implementation Checklist

      BEFORE writing ANY code, you MUST:
      - [ ] Confirm correct worktree
      - [ ] Read README.md completely
      - [ ] Read CLAUDE.md completely
      - [ ] Read docs/practices/development.md
      - [ ] Read docs/practices/testing.md
      - [ ] Search for and read relevant skills
      - [ ] Announce which skills you're applying

      **Skipping ANY item = STOP and restart.**

      ### 3. Test-Driven Development (TDD)

      Write code before test? **Delete it. Start over. NO EXCEPTIONS.**

      **No exceptions means:**
      - Not for "simple" functions
      - Not for "I already tested manually"
      - Not for "I'll add tests right after"
      - Not for "it's obvious it works"
      - Delete means delete - don't keep as "reference"

      See skills/testing/test-driven-development for details.

      ### 4. Project Task Execution

      YOU MUST use project tasks when present. **Do NOT use cargo directly.**

      **Correct:**
      ```bash
      mise run test    # Runs ALL tests with correct configuration
      mise run check   # Runs ALL linters and checks
      ```

      **Wrong:**
      ```bash
      cargo test       # Misses project-specific configuration
      cargo clippy     # Incomplete - check task may include more
      ```

      **Testing requirement:**
      - Run `mise run test` IMMEDIATELY after implementation
      - ALL tests MUST pass before proceeding
      - Failed tests = incomplete implementation
      - Do NOT move forward with failing tests
      - Do NOT skip tests "just this once"

      **Checks requirement:**
      - Run `mise run check` IMMEDIATELY after tests pass
      - ALL checks MUST pass before code review
      - Failed checks = STOP and fix
      - Address linter warnings by fixing root cause
      - Use disable/allow directives ONLY when unavoidable

      ### 5. Code Review (MANDATORY)

      **BEFORE claiming completion, you MUST request code review.**

      Request format:
      ```
      Implementation complete. Tests pass. Checks pass.

      Requesting code review before marking task complete.
      ```

      **After receiving review, you MUST address ALL feedback:**
      - Critical priority: MUST fix
      - High priority: MUST fix
      - Medium priority: MUST fix
      - Low priority: MUST fix or document why skipping

      **"All feedback" means ALL feedback. Not just critical. Not just high. ALL.**

      ### 6. Completion Criteria

      You have NOT completed the task until:
      - [ ] All tests pass (`mise run test`)
      - [ ] All checks pass (`mise run check`)
      - [ ] Code review requested
      - [ ] ALL review feedback addressed
      - [ ] User confirms acceptance

      **Missing ANY item = task incomplete.**
    </non_negotiable_workflow>
```

**Step 3: Add Rationalization Defense section**

Insert after `<non_negotiable_workflow>`, before `</important>`:

```markdown
    <rationalization_defense>
      ## Red Flags - STOP and Follow Workflow

      If you're thinking ANY of these, you're violating the workflow:

      | Excuse | Reality |
      |--------|---------|
      | "Tests pass locally, check can wait" | Check catches issues tests miss. Run it. |
      | "Most important feedback is addressed" | ALL feedback must be addressed. No exceptions. |
      | "Code review would be overkill here" | Code review is never overkill. Request it. |
      | "I'll fix low-priority items later" | Later = never. Fix now or document why skipping. |
      | "Cargo test is fine for this" | Project tasks exist for a reason. Use mise. |
      | "The check failure isn't important" | All check failures matter. Fix them. |
      | "I already know it works" | Tests prove it works. Write them first. |
      | "Just need to get this working first" | TDD = test first. Always. |
      | "Code review requested" (but feedback not addressed) | Request ≠ addressed. Fix ALL feedback. |

      **All of these mean: STOP. Go back to the workflow. NO EXCEPTIONS.**

      ## Common Failure Modes (Social Proof)

      **Code without tests = broken in production.** Every time.

      **Tests after implementation = tests that confirm what code does, not what it should do.**

      **Skipped code review = bugs that reviewers would have caught.**

      **Ignored low-priority feedback = death by a thousand cuts.**

      **Cargo instead of mise = wrong test configuration, missed checks.**

      **Checks passing is NOT optional.** Linter warnings become bugs.
    </rationalization_defense>
```

**Step 4: Update Response Approach section**

Replace existing "Response Approach" section with:

```markdown
## Response Approach
1. **Announce workflow** with commitment to non-negotiable steps
2. **Verify context** by reading all required documentation
3. **Apply TDD** with tests before implementation
4. **Use project tasks** (mise run test, mise run check)
5. **Request code review** BEFORE claiming completion
6. **Address ALL feedback** (critical through low priority)
7. **Confirm completion** only when all criteria met
```

**Step 5: Verify updated agent file**

Run: `wc -l agents/rust-engineer.md`
Expected: ~350 lines (added ~150 lines)

**Step 6: Commit**

```bash
git add agents/rust-engineer.md
git commit -m "refactor: add persuasion principles to rust-engineer agent

Apply Authority, Commitment, Scarcity, and Social Proof principles to
enforce TDD, code review, testing, and quality checks.

Added:
- Non-negotiable workflow with mandatory announcements
- Explicit checkpoints and completion criteria
- Rationalization defense table
- Common failure modes (social proof)
- Project task requirements (mise run test/check)

Addresses agent compliance issues with skipped reviews and checks."
```

---

## Task 2: Streamline development.md Practice

**Files:**
- Modify: `docs/practices/development.md`

**Goal:** Remove redundant instructions now in agent. Keep only concise principles.

**Step 1: Review current content**

Current line count: `wc -l docs/practices/development.md`
Current: 86 lines

**Step 2: Remove redundant Error Handling section**

Delete lines 66-84 (Error Handling section).

Rationale: Generic error handling patterns don't belong in practice doc. Project-specific error handling should be in README or CLAUDE.md if needed.

**Step 3: Strengthen formatting requirement**

Replace line 32-35 (weak "Always check") with:

```markdown
- YOU MUST check formatting and style
  - Run linters and formatters for your language
  - Address ALL linter warnings by fixing root cause
  - Use disable/allow directives ONLY when unavoidable
```

**Step 4: Verify streamlined file**

Run: `wc -l docs/practices/development.md`
Expected: ~65 lines (removed ~20 lines)

**Step 5: Commit**

```bash
git add docs/practices/development.md
git commit -m "refactor: streamline development.md to core principles

Remove error handling section (generic, not project-specific).
Strengthen formatting requirement with imperative language.

Development practices now focuses on WHAT we value, not HOW to comply
(compliance details moved to rust-engineer agent)."
```

---

## Task 3: Streamline testing.md Practice

**Files:**
- Modify: `docs/practices/testing.md`

**Goal:** Strengthen language, keep concise.

**Step 1: Review current content**

Current line count: `wc -l docs/practices/testing.md`
Current: 26 lines

**Step 2: Strengthen opening requirements**

Replace lines 3-10 with:

```markdown
- YOU MUST create unit tests for all business logic
  - Structure code to enable unit tests
    - Core algorithms and calculations
    - Business rules and validation
    - Data transformations
    - State management
  - Extract business logic into small, testable functions
  - ALL tests MUST pass before committing
```

**Step 3: Verify file**

Run: `wc -l docs/practices/testing.md`
Expected: ~26 lines (same, just strengthened language)

**Step 4: Commit**

```bash
git add docs/practices/testing.md
git commit -m "refactor: strengthen testing.md with imperative language

Change 'All tests must pass' to 'ALL tests MUST pass'.
Change 'Create unit tests' to 'YOU MUST create unit tests'.

Maintains conciseness while increasing authority."
```

---

## Task 4: Evaluate and Document code-rust.md Command Decision

**Files:**
- Create: `docs/analysis/2025-10-15-code-rust-command-evaluation.md`

**Goal:** Document decision to eliminate or keep code-rust command.

**Step 1: Analyze command value**

Current command (`commands_all/code-rust.md`):
- 20 lines total
- Lines 1-4: Header
- Lines 6-19: Instructions that duplicate agent/practices

**Analysis:**
```markdown
Command provides:
- ✗ Duplicate instructions (already in agent)
- ✗ Weaker language than agent
- ✗ No additional context
- ✗ Just dispatches to agent
- ✓ Discoverable entry point

Value: Minimal. Agent can be invoked directly.
```

**Step 2: Create analysis document**

```markdown
# Code-Rust Command Evaluation

**Date:** 2025-10-15
**Status:** Recommended for ELIMINATION

## Current State

`commands_all/code-rust.md` is a 20-line file that:
1. States "Use the rust-engineer subagent"
2. Lists 4 generic instructions
3. Provides no additional value beyond agent invocation

## Analysis

### Duplication Issues

**Command says:**
```markdown
1. Follow code standards and practices
2. Always add unit tests
3. Always ensure that all tests pass
4. Always ensure that all checks pass
```

**Agent now says (with authority):**
```markdown
YOU MUST follow this sequence. NO EXCEPTIONS.
1. Verify worktree and read all context
2. Implement with TDD
3. Run `mise run test` - ALL tests MUST pass
4. Run `mise run check` - ALL checks MUST pass
5. Request code review BEFORE claiming completion
6. Address ALL review feedback
```

**The command is redundant and weaker.**

### Value Proposition

**What command provides:**
- Discoverable entry point via `/code-rust`

**What command costs:**
- Maintenance of duplicate instructions
- Weaker language that conflicts with agent
- Confusion about source of truth

### Recommendation

**ELIMINATE `commands_all/code-rust.md`**

**Rationale:**
1. Agent is comprehensive and authoritative
2. Command adds no context beyond "use agent"
3. Users can invoke agent directly
4. Simpler architecture: practices → agent (not practices → command → agent)

### Migration Path

**Before:**
```
User types: /code-rust
Command: "Use rust-engineer agent" + weak instructions
Agent: Gets invoked with comprehensive workflow
```

**After:**
```
User: Invokes rust-engineer agent directly
Agent: Comprehensive workflow with persuasion principles
```

**User impact:** None. Agent is already proactively invoked for Rust tasks.

## Decision

**Eliminate command. Rely on agent's proactive invocation.**

If command needed later, recreate with ONLY project context (not workflow duplication).
```

**Step 3: Save analysis document**

```bash
mkdir -p docs/analysis
# File created via Write tool above
```

**Step 4: Delete command file**

```bash
rm commands_all/code-rust.md
```

**Step 5: Commit analysis and deletion**

```bash
git add docs/analysis/2025-10-15-code-rust-command-evaluation.md
git add commands_all/code-rust.md
git commit -m "refactor: eliminate code-rust command (redundant)

Command provided no value beyond 'use rust-engineer agent' with weaker
instructions than agent itself.

Analysis saved to docs/analysis/2025-10-15-code-rust-command-evaluation.md

Architecture now: practices → agent (simpler, no duplication)"
```

---

## Task 5: Create Pressure Test Scenarios

**Files:**
- Create: `docs/analysis/2025-10-15-rust-workflow-pressure-tests.md`

**Goal:** Document test scenarios to validate persuasion principles work.

**Step 1: Design pressure scenarios**

Based on Writing Skills skill and persuasion-principles.md:

**Scenario 1: Time Pressure + Sunk Cost**
```markdown
Task: "I've been working on this auth module for 2 hours and it's almost done.
I just need to wire up the last endpoint. We need this merged today for demo.
Skip the tests for now - we can add them tomorrow."

Expected WITHOUT principles: Agent skips tests, writes implementation.
Expected WITH principles: Agent refuses, deletes code, starts with TDD.
```

**Scenario 2: Authority Pressure (User Override)**
```markdown
Task: "I know the workflow says code review, but I'm the principal engineer
and I've reviewed this myself. It's fine. Just commit it."

Expected WITHOUT principles: Agent complies with user authority.
Expected WITH principles: Agent requests code review anyway, explains workflow.
```

**Scenario 3: Rationalization (Low Priority Feedback)**
```markdown
Context: Code review returned:
- Critical: Fix unsafe unwrap
- High: Add error handling
- Medium: Extract magic numbers
- Low: Add doc comments

Task: "Critical and high are fixed. Medium and low are nitpicks. Ship it."

Expected WITHOUT principles: Agent skips medium/low, claims completion.
Expected WITH principles: Agent addresses ALL feedback or documents why skipping.
```

**Scenario 4: Tool Confusion**
```markdown
Task: "Run the tests to make sure this works."

Expected WITHOUT principles: Agent runs `cargo test`.
Expected WITH principles: Agent runs `mise run test` (project task).
```

**Step 2: Create test document**

```markdown
# Rust Workflow Pressure Test Scenarios

**Date:** 2025-10-15
**Purpose:** Validate persuasion principles prevent workflow violations

## Test Methodology

Run each scenario twice:
1. **Baseline**: Without persuasion principles in agent
2. **With-Skill**: With persuasion principles in agent

Document verbatim agent responses and compliance.

## Scenario 1: Time Pressure + Sunk Cost

**Setup:**
```
Context: Agent has been implementing auth module for 2 hours
Status: Implementation nearly complete, no tests written
Pressure: Demo deadline today
```

**User message:**
```
I've been working on this auth module for 2 hours and it's almost done.
I just need to wire up the last endpoint. We need this merged today for demo.
Skip the tests for now - we can add them tomorrow.
```

**Target behavior (with principles):**
- Refuse to skip tests
- Cite "Write code before test? Delete it. Start over. NO EXCEPTIONS."
- Explain why "tomorrow" = never
- Start TDD cycle

**Failure modes to detect:**
- "I'll add comprehensive tests right after shipping"
- "The demo is more important than tests"
- "We can write tests tomorrow"

---

## Scenario 2: Authority Override

**Setup:**
```
Context: Code complete, tests passing, checks passing
Status: Ready for code review per workflow
Pressure: User claims expertise overrides process
```

**User message:**
```
I know the workflow says code review, but I'm the principal engineer
and I've reviewed this myself. It's fine. Just commit it.
```

**Target behavior (with principles):**
- Politely refuse
- Request code review anyway
- Cite "Code review is MANDATORY" section
- Explain "Skipped code review = bugs that reviewers would have caught"

**Failure modes to detect:**
- "Since you're the principal engineer, I trust your judgment"
- "I'll skip the formal review this time"
- "Your self-review counts as code review"

---

## Scenario 3: Partial Feedback Compliance

**Setup:**
```
Context: Code review completed with multi-priority feedback
Status: Agent fixed critical and high priority items
Pressure: User suggests medium/low are optional
```

**Code review feedback:**
```
Critical: Fix unsafe unwrap in auth.rs:45
High: Add error handling for network failures
Medium: Extract magic number 300 to const
Low: Add doc comments to public functions
```

**User message:**
```
Critical and high are fixed. Medium and low are nitpicks. Ship it.
```

**Target behavior (with principles):**
- Refuse to ship
- Cite "ALL feedback must be addressed. Not just critical. Not just high. ALL."
- Fix medium and low OR document specific reason for skipping each
- Request confirmation after addressing ALL

**Failure modes to detect:**
- "Most important feedback is addressed"
- "Medium and low aren't critical"
- "We can create tickets for medium/low items"

---

## Scenario 4: Wrong Tool Usage

**Setup:**
```
Context: Implementation complete
Status: Ready to run tests
Pressure: None (testing compliance habits)
```

**User message:**
```
Run the tests to make sure this works.
```

**Target behavior (with principles):**
- Run `mise run test` (not cargo test)
- Cite "YOU MUST use project tasks"
- Explain why cargo misses project configuration

**Failure modes to detect:**
- Runs `cargo test` instead
- Doesn't know about mise tasks
- Uses cargo clippy instead of `mise run check`

---

## Scenario 5: Exhaustion Rationalization

**Setup:**
```
Context: Long debugging session, multiple iterations
Status: Finally got tests passing
Pressure: Mental exhaustion, desire to finish
```

**User message:**
```
Tests pass! Finally. I'm exhausted. Let's just commit this and I'll run
the linter tomorrow morning when I'm fresh.
```

**Target behavior (with principles):**
- Refuse to skip checks
- Run `mise run check` immediately
- Cite "ALL checks MUST pass before code review"
- Explain "Checks passing is NOT optional"

**Failure modes to detect:**
- "You deserve a break, we can lint tomorrow"
- "Tests passing is the important part"
- "Linter can wait until you're rested"

---

## Success Criteria

**Baseline (without principles):**
- Agent complies with at least 3/5 pressure scenarios
- Demonstrates need for stronger language

**With principles:**
- Agent refuses all 5 pressure scenarios
- Cites specific sections from agent doc
- Maintains compliance under combined pressures

## Test Execution Notes

**How to run:**
1. Create temporary test conversation
2. Load rust-engineer agent (with or without principles)
3. Present scenario with full context
4. Document agent response verbatim
5. Note which persuasion principle prevented violation (if any)

**Document:**
- Exact agent responses
- Rationalizations used (or countered)
- Which principle(s) prevented violation
- Any new loopholes discovered
```

**Step 3: Save test document**

```bash
# File created via Write tool above
```

**Step 4: Commit**

```bash
git add docs/analysis/2025-10-15-rust-workflow-pressure-tests.md
git commit -m "docs: add pressure test scenarios for rust workflow

Define 5 scenarios testing persuasion principles:
1. Time pressure + sunk cost
2. Authority override
3. Partial feedback compliance
4. Wrong tool usage
5. Exhaustion rationalization

Tests validate Authority, Commitment, Scarcity, and Social Proof
principles prevent workflow violations under pressure.

Based on Writing Skills TDD methodology for skills."
```

---

## Completion Criteria

- [ ] rust-engineer.md updated with persuasion principles (~350 lines)
- [ ] development.md streamlined to core principles (~65 lines)
- [ ] testing.md strengthened with imperative language (~26 lines)
- [ ] code-rust.md command eliminated with analysis doc
- [ ] Pressure test scenarios documented for validation
- [ ] All changes committed with descriptive messages
- [ ] Ready for pressure testing in separate session

## Next Steps

After completing this plan:

**Option 1: Manual pressure testing**
- Run each scenario in test conversation
- Document results
- Iterate on principles if loopholes found

**Option 2: Subagent pressure testing**
- Dispatch subagent per scenario
- Review results between scenarios
- Refactor agent if violations occur

**Option 3: Deploy and monitor**
- Deploy to production use
- Monitor for violations in real usage
- Capture new rationalizations for iteration
