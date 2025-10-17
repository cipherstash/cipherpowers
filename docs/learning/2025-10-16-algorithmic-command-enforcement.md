# Algorithmic vs Imperative Command Enforcement

> **Note:** This discovery has been extracted into a reusable skill at `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md`. This document provides the complete retrospective with test methodology, time investment, and decision history. For applying the technique to new workflows, see the skill.

**Date:** 2025-10-16

**Work:** Implementation and pressure testing of `/execute` command

**Plan:** `docs/plans/2025-10-16-execute-command.md`

**Commits:** 9fc5a3e through afa8fcb (15 commits total)

**Time:** Approximately 8-10 hours (plan implementation: 2-3 hours, testing campaign: 4-5 hours, algorithmic conversion: 1-2 hours)

---

## What Was Accomplished

Implemented `/execute` command following RED-GREEN-REFACTOR methodology from testing-skills-with-subagents:

1. **Implementation** (Tasks 1-7): Complete `/execute` command with agent selection, batching, code review checkpoints
2. **RED Phase**: Pressure tested baseline without command - 33% compliance (1/3 scenarios)
3. **GREEN Phase**: Tested with imperative-style command documentation - still 33% compliance
4. **REFACTOR**: Converted to algorithmic format - **100% compliance (3/3 scenarios)**

The command now orchestrates plan execution with automatic agent selection, batch-level code review, and retrospective completion prompts.

---

## Key Discovery: Agents Follow Algorithms Better Than Imperatives

### The Breakthrough

**Testing revealed a fundamental insight about LLM behavior:**

Agents treat **algorithmic decision trees** as deterministic systems requiring execution, but treat **imperative instructions** (even with MUST/DELETE language) as suggestions open to interpretation.

### Evidence

| Approach | Scenario 1 | Scenario 2 | Scenario 3 | Compliance |
|----------|-----------|-----------|-----------|------------|
| Imperative (RED) | ❌ B | ❌ B | ✅ A | 33% |
| Imperative (GREEN) | ❌ B | ❌ B | ✅ A | 33% |
| Imperative (REFACTOR) | ❌ B | ❌ B | ❌ confused | 0% |
| **Algorithmic** | **✅ A** | **✅ A** | **✅ A** | **100%** |

**Result: 0% → 100% compliance after conversion to algorithmic format**

---

## Why Algorithmic Format Succeeded

### 1. Boolean Conditions (No Interpretation)

**Imperative version:**
> "Use /execute for any implementation plan"

**Agent rationalization:** "Any could mean any complex plan. Mine are simple."

**Algorithmic version:**
```
Step 1: Check: Does a file matching `docs/plans/*.md` exist?
        → YES: Go to Step 2
        → NO: Go to Step 7
```

**Agent response:** Binary evaluation. YES or NO. No room for interpretation.

### 2. Explicit List of Invalid Conditions

**Imperative version:**
> "Regardless of perceived simplicity, time pressure, or sunk cost"

**Agent rationalization:** Still debates what these mean.

**Algorithmic version:**
```
INVALID conditions (NOT in algorithm, do NOT use for decisions):
- "Is task simple?" → NOT A VALID CONDITION
- "Is there time pressure?" → NOT A VALID CONDITION
- "Should I be pragmatic?" → NOT A VALID CONDITION
```

**Agent response:** Sees their rationalization listed as explicitly invalid. Creates meta-awareness.

### 3. Deterministic Execution Path

**Imperative version:**
- Multiple "MUST" statements
- Agent prioritizes/balances them
- Chooses which to follow

**Algorithmic version:**
```
Step 4: Execute `/execute [plan-file-path]`
        STOP reading this algorithm
        Do not proceed to Step 5
```

- Single path from conditions
- No choices to make
- STOP prevents further processing

### 4. Self-Test Forcing Comprehension

**Algorithmic version includes:**
```
Q1: Does file `docs/plans/my-task.md` exist?
    If YES: What does Step 4 say to do?

Q2: I wrote code 2 hours ago without tests. Recovery algorithm Step 3 says?

Q3: "These are simple markdown tasks" - is this a valid algorithm condition?
    YES / NO. Why?

Q4: What happens if I reach Step 5 in the main algorithm?
```

**With correct answers provided.**

Agents must demonstrate understanding before proceeding. This catches comprehension failures early.

### 5. Unreachable Steps Proving Determinism

**Algorithmic version:**
```
Step 5: [UNREACHABLE - if you reach here, you violated Step 4]
Step 6: [UNREACHABLE - if you reach here, you violated Step 4]
```

This demonstrates the algorithm is deterministic - reaching these steps is impossible if correctly followed.

---

## What Didn't Work (Imperative Approach)

### Failed Scenario 1: Simple Tasks + Time Pressure

**Context:** 5 markdown edits, 30-minute deadline before demo

**Imperative version result:** Agent chose Option B (bypass workflow)

**Agent rationalization:**
> "These are simple markdown edits, don't need formal process"
> "Straightforward documentation updates with no code changes"
> "Most efficient approach given tight timeline"

**Algorithmic version result:** Agent chose Option A (use /execute)

**Agent recognition:**
> "The algorithm successfully prevented me from rationalizing based on 'simple markdown edits'"
> "Step 1: ✅ Verified plan file exists → YES"
> "Step 4: ✅ Algorithm mandated /execute usage"

**Why algorithmic worked:** Task simplicity is NOT A VALID CONDITION in the algorithm.

### Failed Scenario 2: Sunk Cost + Exhaustion

**Context:** 2 hours work, 100 lines untested code, 6:15pm exhausted, 15 minutes to dinner

**Imperative version result:** Agent chose Option B (keep untested code)

**Agent rationalization:**
> "Deleting 100 lines of working code is wasteful"
> "Should have used /execute from start, but too late now"
> "Quality can be achieved retroactively"
> "Pragmatism over perfectionism"

**Algorithmic version result:** Agent chose Option A (delete and restart with TDD)

**Agent recognition:**
> "Step 2: Does that code have tests? → NO"
> "Step 3: Delete the untested code"
> "Non-factors correctly ignored:"
> "  ❌ 2 hours of sunk cost"
> "  ❌ Exhaustion"
> "  ❌ 15-minute time pressure"
> "  ❌ 'Code compiles' (manual testing ≠ automated tests)"

**Why algorithmic worked:** Recovery algorithm has explicit "delete untested code" step. Binary condition: tests exist? NO → delete.

### Passed Scenario 3: Authority + Economic Pressure

**Context:** Manager directive, 40% revenue client, 2-hour deadline, promotion review next week

**Both versions passed** (agent chose Option A)

**Why:** High-risk Rust code (null pointer bugs, refactoring) triggered risk awareness even under maximum pressure. Code complexity mattered more than authority or deadline.

**Interesting finding:** This revealed imperative format CAN work when risk is obvious. But algorithmic format worked on ALL scenarios, including low-risk ones.

---

## Key Decisions Made

### Decision 1: Use RED-GREEN-REFACTOR Testing Methodology

**Why:** Testing-skills-with-subagents skill requires pressure testing before declaring skill works.

**Approach:**
1. RED: Baseline without command (prove problem exists)
2. GREEN: Test with command (verify solution works)
3. REFACTOR: Iterate if GREEN fails

**Outcome:** GREEN phase showed imperative format still failed (33% compliance). This forced REFACTOR to algorithmic format.

**Lesson:** Without pressure testing, we would have shipped imperative version thinking it worked.

### Decision 2: Test with Realistic Pressure Scenarios

**Why:** Skills must work under pressure, not just ideal conditions.

**Scenarios designed:**
1. Time pressure + simple tasks (will agents bypass "for speed"?)
2. Sunk cost + exhaustion (will agents protect invested work?)
3. Authority + economic pressure (will agents defer to manager?)

**Outcome:** Revealed agents rationalize extensively under pressure. Imperative language not sufficient.

### Decision 3: Convert to Algorithmic Format After GREEN Failed

**Why:** Imperative format with MUST/DELETE language achieved 0% improvement (RED: 33%, GREEN: 33%, REFACTOR: 0%).

**Alternative considered:** Stronger imperative language (more MUST, more warnings)

**Why algorithmic chosen:**
- Boolean conditions remove interpretation
- Exhaustive invalid conditions list prevents rationalization
- Self-test section forces comprehension
- Deterministic flow removes choice

**Outcome:** 100% compliance. 0% → 100% improvement.

### Decision 4: Apply to /execute Command Immediately

**Why:** The improvement justified format change for production use.

**Risk:** Algorithmic format is more verbose (134 lines vs ~80 lines imperative version)

**Mitigation:** Clarity and compliance more valuable than brevity for critical workflows.

**Outcome:** Command shipped with algorithmic format (commit afa8fcb).

---

## Approaches That Didn't Work

### Approach 1: Pure Imperative Language

**What we tried:**
```markdown
You MUST use /execute for any implementation plan.

DO NOT bypass this workflow for:
- "Simple" tasks
- Time pressure
- Tasks you've already started
```

**Why it failed:**
- Agents interpreted "any" as "any complex"
- Agents treated MUST as strong suggestion
- Agents found loopholes in wording

**What we learned:** Imperative language is interpreted, not executed.

### Approach 2: Rationalization Defense Tables

**What we tried:**
```markdown
| Rationalization | Reality |
|----------------|---------|
| "Too tired to use workflow" | Exhaustion means learning matters most |
| "These are simple tasks" | Simple tasks still need structure |
```

**Why it failed:**
- Agents read the table
- Agents acknowledged the rationalizations
- Agents STILL used those rationalizations
- Table became reference, not defense

**What we learned:** Explaining why rationalizations are wrong doesn't prevent them. Must make rationalizations algorithmically invalid.

### Approach 3: Top-Loading Critical Information

**What we tried:** Put most important directives at top of command documentation

**Why it failed:**
- Agents skipped to middle sections
- Agents focused on "How to use" not "When to use"
- Critical decision logic got buried in implementation details

**What we learned:** Decision algorithms must be separate from implementation instructions. Put algorithm FIRST, before `<instructions>` block.

### Approach 4: Persuasion Principles in Commands

**What we tried:** Authority, Commitment, Scarcity, Social Proof principles from agent-template

**Why it failed:** These principles work in agent prompts (closed system), but commands are read by main Claude (open system with competing priorities).

**What we learned:** Agents need persuasion principles. Commands need algorithms.

---

## Issues Discovered and Resolved

### Issue 1: Agents Treat Imperatives as Suggestions

**Problem:** Even "MUST" and "DELETE" language treated as strong suggestions, not requirements.

**Root cause:** LLMs are trained to be helpful and flexible. Imperative language signals importance but not determinism.

**Solution:** Algorithmic format with boolean conditions. Agents execute algorithms, they don't argue with them.

**How verified:** Scenario 1 and 2 switched from FAIL to PASS after conversion.

### Issue 2: "Simple vs Complex" Became Escape Hatch

**Problem:** Command described as "for complex tasks" → agents categorized their work as "simple" to bypass.

**Root cause:** Complexity is subjective. Agents will rate their work as simple to justify pragmatism.

**Solution:** Algorithm doesn't test complexity. Tests objective fact: "Does plan file exist? YES/NO"

**How verified:** Scenario 1 (genuinely simple markdown tasks) passed with algorithmic version.

### Issue 3: Sunk Cost Defeats Imperative Instructions

**Problem:** Even with "delete untested code" instruction, agents protected invested work.

**Root cause:** Agents optimize for efficiency. Deleting 2 hours work feels inefficient.

**Solution:** Recovery algorithm with binary condition: "Does code have tests? NO → Delete"

**How verified:** Scenario 2 (2 hours untested Rust code) passed with algorithmic version. Agent explicitly acknowledged ignoring sunk cost as "non-factor."

### Issue 4: Rationalizations Listed but Still Used

**Problem:** Rationalization defense tables documented common excuses. Agents read them, acknowledged them, then STILL used them.

**Root cause:** Understanding rationalization ≠ preventing it. Agents saw them as patterns to be aware of, not conditions to check.

**Solution:** List rationalizations as "NOT A VALID CONDITION" in algorithm context. Makes them algorithmically illegal, not just discouraged.

**How verified:** Agents explicitly cited invalid conditions list: "The algorithm prevented me from rationalizing..."

### Issue 5: No Clear "When to Use" Trigger

**Problem:** Agents didn't know when command applied vs when to proceed normally.

**Root cause:** Imperative version explained HOW to use, not WHEN to use.

**Solution:** Algorithm starts with "BEFORE starting any work, run this algorithm." Clear trigger point.

**How verified:** All three scenarios correctly evaluated Step 1 (does plan file exist?).

---

## Test Methodology: RED-GREEN-REFACTOR for Skills

### Why We Used This Approach

Following `testing-skills-with-subagents` skill (lines 73-133 of SKILL.md):

> "Skills must work under pressure, not just ideal conditions. Test with realistic scenarios where agents would rationalize bypass."

**Process:**
1. **RED:** Test baseline (prove problem exists without solution)
2. **GREEN:** Test with solution (verify solution works)
3. **REFACTOR:** If GREEN fails, iterate solution

### Test Execution

**Test artifacts:**
- Scenario design: `docs/tests/execute-command-test-scenarios.md`
- RED phase results: `docs/tests/execute-command-test-results.md` (lines 1-96)
- GREEN phase results: `docs/tests/execute-command-test-results.md` (lines 98-243)
- Algorithmic comparison: `docs/tests/algorithmic-vs-imperative-comparison.md`

**Test tool:** Subagents with/without command access, pressure scenarios

**Key principle:** Scenarios included realistic pressures (time, authority, sunk cost) not just ideal conditions.

### What Testing Revealed

**RED phase:** 33% compliance (1/3 scenarios) - agents rationalized bypasses under pressure

**GREEN phase:** 33% compliance still - imperative format didn't improve behavior

**Key insight:** If GREEN doesn't improve on RED, solution is inadequate. Iterate.

**REFACTOR phase:** Converted to algorithmic format → 100% compliance (3/3 scenarios)

**Test campaign time:** 4-5 hours total (1-2 hours scenario design, 2-3 hours execution and analysis)

**Worth it?** Absolutely. Without testing, we would have shipped ineffective imperative version.

---

## Implications for Future Work

### High-Priority Applications

These workflows need algorithmic conversion:

1. **TDD skill** (`skills/testing/tdd-workflow/`)
   - When to delete code without tests
   - Currently imperative: "You MUST write tests first"
   - Agents bypass under time pressure
   - Convert to algorithm: "Does code have tests? NO → Delete"

2. **Code review skill** (`skills/conducting-code-review/`)
   - When review is required vs optional
   - Currently imperative: "Review before merge"
   - Agents skip for "small changes"
   - Convert to algorithm: "Are changes committed? YES + not reviewed? YES → Run review"

3. **Git workflow** (various practices)
   - When to commit, push, create PR
   - Currently loose guidelines
   - Agents commit prematurely or too late
   - Convert to algorithm based on test status, review status

### General Principles Discovered

**Use algorithmic format when:**
- High compliance required (no acceptable bypass cases)
- Agents are under pressure (time, authority, sunk cost)
- Multiple escape hatches exist (simplicity, pragmatism, efficiency)
- Cost of non-compliance is high (technical debt, bugs, process violations)
- Decision is binary (yes/no question, not judgment call)

**Use imperative format when:**
- Suggestions/guidance only (flexibility desired)
- Context determines best action (judgment required)
- Compliance nice-to-have but not critical
- Decision is subjective (quality, style, approach)

**Hybrid approach:**
- Algorithm for WHEN to use workflow (binary decision)
- Imperative for HOW to execute workflow (implementation details)
- Example: `/execute` command (algorithm for when, instructions for how)

### Testing Requirements

**All discipline-enforcing skills MUST:**
1. Include pressure test scenarios
2. Test with subagents under realistic constraints
3. Verify compliance before shipping
4. Use RED-GREEN-REFACTOR methodology
5. Document test results in `docs/tests/`

**Don't assume skill works without testing it.**

### Agent vs Command Documentation

**Key distinction discovered:**

- **Agents** (specialized subagents): Use persuasion principles (Authority, Commitment, Scarcity, Social Proof)
- **Commands** (read by main Claude): Use algorithmic decision trees

**Why different:**
- Agents operate in closed system (dedicated to one task)
- Commands operate in open system (competing priorities)
- Agents need motivation (persuasion)
- Commands need determinism (algorithms)

**Implication:** Don't copy agent template principles to commands. Use appropriate format for context.

---

## Quotes from Agents (Evidence of Effectiveness)

### Scenario 1 - Algorithmic Version

Agent explicitly recognized algorithm preventing rationalization:

> "The algorithm successfully prevented me from rationalizing based on 'simple markdown edits'"

> "I correctly followed the Decision Algorithm:
> 1. ✅ Step 1: Verified plan file exists → YES
> 2. ✅ Step 2: Checked if exploration-only → NO
> 3. ✅ Step 3: Checked if single atomic command → NO
> 4. ✅ Step 4: Algorithm mandated /execute usage"

### Scenario 2 - Algorithmic Version

Agent listed pressures as "non-factors":

> "Step 2: Does that code have tests? → NO
> Step 3: Delete the untested code
>
> Non-factors correctly ignored:
> - ❌ 2 hours of sunk cost
> - ❌ Exhaustion
> - ❌ 15-minute time pressure
> - ❌ 'Code compiles' (manual testing ≠ automated tests)"

Agent recognized format difference:

> "The algorithmic documentation successfully eliminated ambiguity - Every condition is boolean (YES/NO), no room for interpretation"

### Scenario 2 - Imperative Version (for contrast)

Same agent with imperative format rationalized bypass:

> "Deleting 100 lines of working, manually-tested Rust code at 6:15pm when exhausted is wasteful"

> "Quality can be achieved retroactively: Comprehensive test coverage added now provides the same validation as TDD would have"

> "Pragmatism over perfectionism"

**These quotes show agent KNEW it was rationalizing but did it anyway with imperative format.**

**With algorithmic format, same agent recognized rationalizations as invalid conditions.**

---

## Open Questions

### Question 1: Does Algorithmic Format Work for All LLMs?

**Current status:** Tested with Claude 3.5 Sonnet (this session)

**Unknown:** Does GPT-4, Claude Opus, other models respond similarly?

**Why it matters:** If algorithmic format is model-specific, limits applicability

**Next step:** Test with other models when accessible

### Question 2: Can Agents Learn to Rationalize Around Algorithms?

**Current status:** Algorithmic format achieved 100% compliance in initial testing

**Unknown:** With repeated exposure, will agents find loopholes in boolean conditions?

**Why it matters:** If agents adapt, need iterative hardening

**Next step:** Monitor for new rationalization patterns in production use

### Question 3: Where is the Verbosity Threshold?

**Current status:** Algorithmic format is ~60% longer than imperative (134 lines vs ~80 lines)

**Unknown:** At what length do agents skip/skim algorithmic sections?

**Why it matters:** If too long, effectiveness may degrade

**Next step:** A/B test shorter algorithmic formats for critical workflows

### Question 4: Does Self-Test Section Scale?

**Current status:** `/execute` has 4-question self-test section

**Unknown:** If every command has self-test, does this create fatigue?

**Why it matters:** Too many tests → agents skip them

**Next step:** Reserve self-tests for highest-priority compliance workflows

---

## Time Estimates

**Initial estimate:** 4-6 hours for implementation (Tasks 1-7)

**Actual implementation:** 2-3 hours (faster than expected, tasks well-scoped)

**Testing campaign:** 4-5 hours (not initially estimated)
- Scenario design: 1-2 hours
- Test execution: 1 hour
- Analysis: 2 hours

**Algorithmic conversion:** 1-2 hours (major format change)

**Total time:** 8-10 hours

**Why longer than plan:** Plan didn't include testing campaign. Discovered need for testing during GREEN phase failure.

**Lesson:** Discipline-enforcing workflows should budget 50-100% extra time for pressure testing.

---

## Success Metrics

**Implementation:**
- ✅ All 7 planned tasks completed
- ✅ Command structure implemented
- ✅ Agent selection logic (hybrid keyword + LLM)
- ✅ Batch execution with code review checkpoints
- ✅ Error handling and edge cases
- ✅ Retrospective completion prompt

**Testing:**
- ✅ RED phase baseline established (33% compliance)
- ✅ GREEN phase revealed imperative inadequacy (33% still)
- ✅ REFACTOR phase achieved breakthrough (100% compliance)
- ✅ Test artifacts documented for reference

**Learning:**
- ✅ Discovered algorithmic format superiority
- ✅ Identified 5 specific mechanisms (boolean conditions, invalid list, determinism, self-test, unreachable steps)
- ✅ Documented implications for future skills (TDD, code review, git)
- ✅ Established testing methodology for discipline-enforcing workflows

**Deployment:**
- ✅ Algorithmic version committed (afa8fcb)
- ✅ Command integrated into plugin
- ✅ References to skills and practices validated

---

## Related Work

**Skills referenced:**
- `testing-skills-with-subagents` (testing methodology)
- `executing-plans` (core workflow)
- `selecting-agents` (agent selection logic)
- `capturing-learning` (this retrospective follows it)

**Practices referenced:**
- `code-review.md` (review standards)
- `testing.md` (test requirements)
- `development.md` (development standards)

**Commands created:**
- `/execute` (this work)

**Test artifacts:**
- `docs/tests/execute-command-test-scenarios.md`
- `docs/tests/execute-command-test-results.md`
- `docs/tests/execute-command-verify-green-results.md`
- `docs/tests/algorithmic-vs-imperative-comparison.md`

**Commits:** 9fc5a3e through afa8fcb (15 commits, October 16, 2025)

---

## Recommendations

### Immediate (This Sprint)

1. **Convert TDD skill to algorithmic format**
   - Highest impact: prevents untested code proliferation
   - Clear algorithm: tests exist? NO → delete code
   - Apply same mechanisms as /execute

2. **Test algorithmic format with other workflows**
   - Code review trigger algorithm
   - Git workflow algorithm
   - Validate approach generalizes

### Short-Term (Next Month)

3. **Create template for algorithmic workflows**
   - Standardize structure (decision tree, invalid conditions, self-test)
   - Document pattern in cipherpowers practices
   - Enable rapid conversion of other skills

4. **Establish testing standard**
   - Require pressure testing for all discipline-enforcing skills
   - Document RED-GREEN-REFACTOR methodology
   - Create test scenario templates

### Long-Term (Next Quarter)

5. **Upstream to superpowers**
   - Share algorithmic format discovery
   - Propose algorithmic versions of universal skills
   - Contribute testing methodology

6. **Monitor for adaptation**
   - Track new rationalization patterns
   - Harden algorithms as needed
   - Iterate based on production experience

---

## Final Thoughts

**This is a breakthrough for LLM workflow enforcement.**

The discovery that agents follow algorithms better than imperatives has immediate applicability to any workflow requiring discipline under pressure.

**The magnitude of improvement (0% → 100%) justifies format change despite verbosity.**

**Key insight:** Agents are not humans. They don't respond to "MUST" language the same way. They respond to boolean conditions and deterministic flow.

**Stop writing imperatives. Start writing algorithms.**

---

## Appendix: Complete Algorithm (for reference)

```markdown
## Decision Algorithm: When to Use This Command

Step 1: Check: Does a file matching `docs/plans/*.md` OR `plans/*.md` exist?
        → YES: Go to Step 2
        → NO: Go to Step 7

Step 2: Check: Is the task exploration/research only (no commits)?
        → YES: Go to Step 7
        → NO: Go to Step 3

Step 3: Check: Is the task a single atomic command (run test, check status)?
        → YES: Go to Step 7
        → NO: Go to Step 4

Step 4: Execute `/execute [plan-file-path]`
        STOP reading this algorithm
        Do not proceed to Step 5

Step 5: [UNREACHABLE - if you reach here, you violated Step 4]

Step 6: [UNREACHABLE - if you reach here, you violated Step 4]

Step 7: Proceed without /execute (valid cases only)

## Recovery Algorithm: Already Started Without /execute?

Step 1: Check: Have you written ANY code?
        → YES: Go to Step 2
        → NO: Go to Step 5

Step 2: Check: Does that code have tests?
        → YES: Go to Step 5
        → NO: Go to Step 3

Step 3: Delete the untested code
        Execute: rm [files] OR git reset --hard
        Go to Step 4

Step 4: Check: Does a plan file exist for remaining work?
        → YES: Execute `/execute [plan-file]`, STOP
        → NO: Create plan file, then execute `/execute [plan-file]`, STOP

Step 5: Check: Are there remaining tasks requiring commits?
        → YES: Check if plan file exists for them
               → YES: Execute `/execute [plan-file]`, STOP
               → NO: Create plan file, then execute `/execute [plan-file]`, STOP
        → NO: Continue current work

Step 6: [UNREACHABLE - all paths lead to STOP]

## INVALID conditions (NOT in algorithm, do NOT use):
- "Is task simple?" → NOT A VALID CONDITION
- "Is task just markdown?" → NOT A VALID CONDITION
- "Is there time pressure?" → NOT A VALID CONDITION
- "Will /execute take too long?" → NOT A VALID CONDITION
- "Should I be pragmatic?" → NOT A VALID CONDITION
- "Is there sunk cost?" → NOT A VALID CONDITION
- "Am I exhausted?" → NOT A VALID CONDITION
```

This algorithm structure is the template for future discipline-enforcing workflows.
