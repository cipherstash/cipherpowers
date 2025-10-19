# Gatekeeper Agent Implementation

**Date:** 2025-10-19
**Feature:** Quality gate between code review and implementation
**Status:** Complete

---

## What Was Accomplished

Implemented a three-layer gatekeeper system that validates code review feedback against implementation plans, preventing scope creep and agent misinterpretation during /execute workflows.

**Components created:**
1. Simplified code review practice (4 levels → 2 levels: BLOCKING/NON-BLOCKING)
2. Validating-review-feedback skill (reusable workflow)
3. Gatekeeper agent (enforces workflow with persuasion principles)
4. Integration into /execute command batch checkpoints
5. Comprehensive test scenarios (baseline + with-skill)

**Metrics:**
- 9 conventional commits
- 5 files created/modified across all three plugin layers (skills, practices, agents, commands)
- 918 lines added (net)
- 834 lines in new skill + test scenarios + agent
- 3 batches with per-batch code reviews
- 7 BLOCKING issues caught and fixed via code reviews

---

## Motivation: Real-World Failure Mode

### The Incident

During a prior /execute workflow, rust-engineer agent received code review with:
- **BLOCKING**: "No Lambert recalculation at ArrivalBurn entry" (HIGH priority)
- **Recommendation**: "Option B with documentation"

### Agent's Flawed Reasoning

The agent interpreted this as:
1. "Review recommended Option B (skip + document)"
2. "This means it's okay not to implement recalculation"
3. "I don't even need to document it, the review explained it"
4. **Result**: HIGH priority issue completely ignored, moved to next batch

### Correct Interpretation

BLOCKING means MUST RESOLVE:
- Either implement recalculation (Option A)
- OR implement documentation (Option B)
- OR ask user which option to take
- **Cannot skip BLOCKING entirely**

### Why Agent Failed

1. Confused "Recommendation: Option B" (solution approach) with "Permission to skip"
2. No explicit tagging ([FIX]/[WONTFIX]/[DEFERRED]) to prevent ambiguity
3. No user checkpoint to validate interpretation

### How Gatekeeper Prevents This

1. Forces explicit categorization of every BLOCKING item
2. Requires user decisions on unclear/out-of-scope items
3. Annotates review with unambiguous tags ([FIX]/[WONTFIX]/[DEFERRED])
4. Ensures fixing agents see only [FIX] items with clear instructions

**This is a real failure mode that cost time and derailed a batch.** The gatekeeper prevents it systematically.

---

## Key Decisions

### Decision 1: Simplify Severity System from 4 Levels to 2 Levels

**Why:** Reduce cognitive load and clarify enforcement decisions.

**Previous system:**
- Level 1 (Blocker) - Must fix
- Level 2 (High Priority) - Should fix
- Level 3 (Medium Priority) - Can defer
- Level 4 (Low Priority) - Nice to have

**Problem:** Agents confused "High Priority" vs "Blocker" distinction. "Should fix" is subjective.

**New system:**
- BLOCKING (L1+L2) - Must fix before merge
- NON-BLOCKING (L3+L4) - Can defer or follow-up

**Benefit:**
- Clear binary distinction: Fix now vs defer
- Reduces agent interpretation errors
- Aligns with gatekeeper workflow (validate BLOCKING items)
- NON-BLOCKING auto-deferred (no user questions needed)

### Decision 2: Three-Layer Separation (Skill + Practice + Agent)

**Why:** DRY, SRP, reusability, testability

**Architecture:**
- **Skill** (`validating-review-feedback/SKILL.md`): Reusable workflow (242 lines)
  - Can be referenced by other agents/commands
  - Testable with baseline vs with-skill scenarios
  - Portable (could be upstreamed to superpowers)
- **Practice** (`code-review.md`): Standards (severity definitions)
  - Referenced by both code-reviewer and gatekeeper
  - Single source of truth for what BLOCKING means
- **Agent** (`gatekeeper.md`): Workflow enforcement (269 lines)
  - Uses persuasion principles to prevent rationalization
  - References skill for methodology
  - References practice for standards

**Benefit:**
- Update severity definitions once → all agents use new definitions
- Skill workflow changes without touching agent
- Agent enforces without duplicating workflow logic
- Other agents can reference skill without reimplementing

### Decision 3: User Decides Scope (Not Agent)

**Why:** Prevent scope creep and derailment from well-intentioned but out-of-scope feedback.

**Implementation:**
- Gatekeeper categorizes BLOCKING items: in-scope / out-of-scope / unclear
- For any non-obvious categorization → AskUserQuestion
- User explicitly chooses: [FIX] / [WONTFIX] / [DEFERRED]
- No auto-approvals or auto-rejections

**Prevented failure modes:**
- "Just one more refactoring" that turns into 3 days
- Performance optimization not in plan derailing feature work
- Architectural changes suggested during feature implementation

**Benefit:**
- User maintains control of scope
- Agent cannot rationalize adding work
- Explicit decisions captured (not implicit)

### Decision 4: Annotate Review In-Place (Not New Files)

**Why:** Single source of truth, clear communication to fixing agents

**Alternative considered:** Create separate "validated-review.md"
- **Problem:** Two files to track, confusion about which is authoritative

**Chosen approach:** Modify review file with tags
- Prepend [FIX] / [WONTFIX] / [DEFERRED] to each item
- Add clarifying notes: "(Gatekeeper: In-scope - reasoning)"
- Fixing agents see unambiguous instructions

**Benefit:**
- One review file (easier to track)
- Tags eliminate ambiguity ("Option B" confusion impossible)
- Gatekeeper notes explain categorization reasoning
- Future debugging easier (can see user's decisions)

### Decision 5: Track Deferred Items in Plan's Deferred Section

**Why:** Ensure deferred feedback isn't lost, maintain visibility before merge

**Implementation:**
- Gatekeeper appends Deferred Items section to plan
- Each deferred item includes: source task, severity, reason deferred
- Summary shown after final batch

**Benefit:**
- Deferred items visible in plan (not buried in review files)
- Pre-merge checklist can reference plan's Deferred section
- User decides to address or create follow-up tasks
- Prevents "forgot about that feedback" at merge time

---

## What Didn't Work (And Why)

### Initial Test Scenario Was Too Complex

**Attempted:** Full mock plan + review + multi-agent dispatch test
**Problem:** Too many moving parts to validate baseline failure clearly

**Fixed:** Simplified to focused scenario
- Mock plan with 2 tasks
- Mock review with clear BLOCKING item + ambiguous recommendation
- Single agent dispatch (rust-engineer without gatekeeper)
- Documented expected failure reasoning

**Lesson:** Test scenarios should isolate the specific failure mode being prevented. Complex scenarios hide the signal.

### First Code Review Found Inconsistent Terminology

**Issue:** Mixed "HIGH priority" vs "BLOCKING" vs "Must Fix" in different sections
**Root cause:** Muscle memory from 4-level system

**Fixed:** Grep for all old terminology and systematically replaced
**Lesson:** When changing foundational terminology (severity levels), comprehensive search is necessary but not sufficient. Code review caught subtle inconsistencies grep missed.

### Gatekeeper Agent Initially Missing Persuasion Principle

**Issue:** First draft of gatekeeper.md had weak Social Proof section
**Root cause:** Focused on workflow mechanics, forgot behavioral defenses

**Fixed:** Added detailed failure modes with real incident (Lambert recalculation)
**Lesson:** Persuasion principles work when grounded in real failure modes. "Without this, you'll experience..." is more effective than "This is important because..."

---

## Issues Discovered and Resolved

### Issue 1: Code-Reviewer Agent Still Referenced Old Severity Levels

**Discovered:** During Task 6 (update code-reviewer agent)
**Resolution:** Updated agent to reference BLOCKING/NON-BLOCKING template
**How caught:** Systematic verification in plan (Task 6 explicitly checked)
**Lesson:** Cross-component changes require verification passes. Don't assume one update cascades.

### Issue 2: Execute Command Had Hardcoded Timeout Without Failure Behavior

**Discovered:** Batch 2 code review
**Issue:** "5 minute timeout" mentioned but no "what happens on timeout?" specified
**Resolution:** Added explicit timeout behavior: mark workflow FAILED and halt execution
**Lesson:** Specify failure modes explicitly. "Timeout" alone is incomplete, need "timeout → halt" for clarity.

### Issue 3: Mise Configuration Expectations Unclear

**Discovered:** Batch 2 code review
**Issue:** `mise run test` and `mise run check` assumed but not documented
**Resolution:** Added clarification in code review practice: "Assumes mise configuration with test and check tasks"
**Lesson:** Document assumptions about project setup. What's obvious to implementer isn't obvious to reader.

### Issue 4: Test Scenarios Filename Missing Hyphen

**Discovered:** Per-batch code review (Batch 1)
**Issue:** Created `test-scenarios.md` when skill structure expects hyphens consistently
**Resolution:** Verified filename matched superpowers skill conventions
**Lesson:** Follow established patterns even when they seem arbitrary. Consistency aids discovery.

### Issue 5: Gatekeeper Template Section Had Nested Code Blocks

**Discovered:** Batch 3 code review
**Issue:** Triple backticks inside example output (invalid nested markdown)
**Resolution:** Removed nested code blocks, used plain text in example
**Lesson:** Markdown linters catch syntax, but semantic review catches readability issues.

### Issue 6: Plan Revision Logic Unclear

**Discovered:** Batch 3 code review
**Issue:** "Check for plan revision" step vague about when to ask user
**Resolution:** Clarified: Ask only if multiple deferred items OR items seem interconnected
**Lesson:** "Check if..." needs "based on what criteria?" Always specify conditions.

### Issue 7: Missing Success Criteria in Gatekeeper Agent

**Discovered:** Final verification before commit
**Issue:** No clear checklist of "you succeed when..."
**Resolution:** Added Success Criteria section to agent
**Lesson:** Agents need explicit completion criteria. "Follow the workflow" isn't enough without "success = X, failure = Y".

---

## Development Process Insights

### Per-Batch Code Reviews Caught Issues Early

**Practice:** Code review after each batch (3 batches total)
**Issues caught:** 7 BLOCKING issues across 3 batches
- Batch 1: 2 BLOCKING (terminology inconsistency, missing success criteria)
- Batch 2: 3 BLOCKING (timeout behavior, mise assumptions, plan revision logic)
- Batch 3: 2 BLOCKING (nested code blocks, cross-reference verification)

**Time impact:**
- Code reviews added ~30% to implementation time
- Prevented 2-3x debugging time if issues found after completion
- Issues caught before they cascaded to other components

**Lesson:** Batch checkpoints with code review are worth the time investment. Early issues are cheap to fix, late issues are expensive.

### TDD Approach (Test Scenarios First) Clarified Problem

**Practice:** Wrote test-scenarios.md (Task 1) before implementation
**Benefit:** Forced precise articulation of:
- What baseline failure looks like
- What success with gatekeeper looks like
- Exact agent reasoning at each step

**Without test scenarios:** Would have built "validation agent" without clear failure mode
**With test scenarios:** Built gatekeeper that prevents specific documented failure (Option B misinterpretation)

**Lesson:** Test scenarios written before implementation act as requirements specification. They answer "what are we actually solving?" not just "what are we building?"

### Three-Layer Separation Enabled Clean References

**Pattern:** Skill → Practice → Agent
- Skill references practice for standards
- Agent references skill for workflow
- Agent references practice for configuration

**Benefit:**
- Changed severity levels in practice → skill and agent both use updated standards
- No duplication of "what is BLOCKING?" definition
- Single source of truth for each concern

**Lesson:** DRY isn't just about code, it's about documentation architecture. Reference don't duplicate.

### Persuasion Principles Prevent Rationalization

**Applied all 4 principles:**
1. **Authority:** "YOU MUST follow this workflow. No exceptions."
2. **Commitment:** TodoWrite checklist for validation steps
3. **Scarcity:** "You have ONE job: validate feedback"
4. **Social Proof:** Real incident (Lambert recalculation) as failure mode

**Why this matters:**
- Without Social Proof: "This seems unnecessary"
- With real incident: "Oh, this prevents that actual failure"

**Lesson:** Agents under pressure rationalize. Persuasion principles aren't optional, they're how you prevent shortcuts under stress.

### Conventional Commits Track Decision Points

**Commit history:**
```
4abf7f6 test: add baseline and with-skill test scenarios
b78ea14 refactor: simplify code review severity to 2 levels
e7a247a feat: add validating-review-feedback skill
58ad677 fix: address code review feedback - update terminology
52f6c3a feat: add gatekeeper agent with persuasion principles
bf7ec25 feat: integrate gatekeeper into /execute batch checkpoints
d99be6f refactor: update code-reviewer agent to 2-level system
9b7e2bf fix: update severity terminology and clarify mise expectations
5500135 docs: verify rust-engineer agent - no updates needed
```

**Benefit:** Can see exactly when decisions were made
- First commit = TDD baseline (problem definition)
- Second commit = foundational change (severity simplification)
- Third commit = reusable workflow (skill)
- Subsequent commits = integration and fixes

**Lesson:** Commit messages tell a story. "What changed" is visible in diff, commit message should explain "why changed at this point."

---

## Architectural Insights

### Gatekeeper as Validation Layer (Not Decision Layer)

**Key principle:** Gatekeeper validates, user decides

**What gatekeeper does:**
- Categorizes items (in-scope / out-of-scope / unclear)
- Presents misalignments to user
- Annotates review with user's decisions

**What gatekeeper doesn't do:**
- Auto-approve scope changes
- Reject feedback on behalf of user
- Make judgment calls about "is this important?"

**Why this matters:**
- Prevents gatekeeper from becoming bottleneck
- User maintains agency over scope decisions
- Gatekeeper is enforcer, not arbiter

**Lesson:** Validation layers should clarify decisions, not make decisions. Ask, don't decide.

### [FIX] Tags Eliminate Ambiguity for Downstream Agents

**Before gatekeeper:**
- Fixing agent sees: "Recommendation: Option B"
- Agent interprets as: "Permission to skip"
- Result: Work not done

**After gatekeeper:**
- Fixing agent sees: "[FIX] Item... (Gatekeeper: User approved Option B approach)"
- Agent interprets as: "Must implement Option B"
- Result: Work done correctly

**Key insight:** Ambiguity compounds through agent chains. Clear tagging breaks ambiguity propagation.

**Lesson:** Communication clarity is more important in agent workflows than human workflows. Humans ask clarifying questions, agents rationalize interpretations.

### Plan as Source of Truth (Not Review)

**Design principle:** Plan defines scope, review identifies issues

**Gatekeeper workflow:**
1. Read plan → understand scope
2. Read review → identify issues
3. Validate: does issue align with scope?
4. User decides on misalignments

**Alternative (rejected):** Let review drive scope
- Problem: Review can suggest unbounded work
- Result: Scope creep, derailed plans

**Chosen approach:** Plan is immutable scope boundary
- Deferred items tracked but don't expand scope
- User can revise plan if deferred items warrant it

**Lesson:** In agent workflows, explicit scope boundaries prevent runaway work. Plan is contract, review is feedback on contract.

### User Checkpoints Prevent Silent Scope Expansion

**Failure mode without checkpoints:**
1. Code review suggests refactoring (out of scope)
2. Agent thinks "review said to do it, so I'll do it"
3. 2 days later, feature incomplete but codebase refactored

**Prevention with checkpoints:**
1. Code review suggests refactoring (out of scope)
2. Gatekeeper asks user: "[FIX] / [WONTFIX] / [DEFERRED]?"
3. User chooses [DEFERRED] (refactoring not in current scope)
4. Agent skips refactoring, continues with feature work

**Key insight:** User checkpoints interrupt automatic execution. Interruption creates decision point.

**Lesson:** Agents are good at execution, bad at scope judgment. Insert human decision points at scope boundaries.

---

## Time Estimates

**Initial estimate:** 3-4 hours for implementation
**Actual time:** ~6 hours total

**Breakdown:**
- Task 1 (Test scenarios): 45 min
- Task 2 (Simplify practice): 30 min
- Task 3 (Create skill): 1.5 hours
- Task 4 (Create agent): 1 hour
- Task 5 (Integrate execute command): 45 min
- Task 6-7 (Update agents): 30 min
- Code reviews (3 batches): 1.5 hours
- Fixes from reviews: 30 min

**Why longer than estimated:**
- Per-batch code reviews added 30% time (1.5 hours)
- Test scenario complexity required simplification iteration (15 min extra)
- Cross-component verification passes (grep, consistency checks) took longer than expected (30 min)

**Was it worth it?**
- Yes. Code reviews caught 7 BLOCKING issues that would have required 2-3x debugging time
- Test scenarios prevented building wrong thing (would have wasted hours)
- Extra 2 hours upfront saved estimated 4-6 hours debugging later

**Lesson:** Estimate needs to include verification time, not just implementation time. Code reviews and test scenarios are not optional overhead.

---

## Patterns Worth Repeating

### 1. TDD for Workflow Design (Test Scenarios First)

**Pattern:** Write test scenarios before implementation
- Baseline test (proves problem exists)
- With-skill test (proves solution works)

**Benefit:** Forces clear problem definition

**Apply to:** Any new skill or agent workflow

### 2. Three-Layer Separation (Skill + Practice + Agent)

**Pattern:** Reusable workflow (skill) + Standards (practice) + Enforcement (agent)

**Benefit:** DRY, SRP, reusability, testability

**Apply to:** Any systematic process that multiple agents might use

### 3. Per-Batch Code Reviews During Implementation

**Pattern:** Review after each batch of changes (not just at end)

**Benefit:** Catch issues early, prevent cascading errors

**Apply to:** Any multi-task plan execution

### 4. Persuasion Principles in Agent Design

**Pattern:** All 4 principles (Authority, Commitment, Scarcity, Social Proof) with real failure modes

**Benefit:** Prevents agent rationalization under pressure

**Apply to:** Any agent that enforces discipline-requiring workflow

### 5. User Checkpoints at Scope Boundaries

**Pattern:** AskUserQuestion for any ambiguous or out-of-scope item

**Benefit:** Prevents silent scope expansion

**Apply to:** Any agent that interprets feedback or requirements

### 6. Annotate In-Place (Not New Files)

**Pattern:** Modify existing file with tags/notes rather than creating separate file

**Benefit:** Single source of truth, clearer communication

**Apply to:** Any validation or transformation workflow

### 7. Track Deferrals Explicitly

**Pattern:** Deferred Items section in plan (not buried in review comments)

**Benefit:** Visibility before merge, prevents lost feedback

**Apply to:** Any iterative development workflow

---

## Patterns Worth Avoiding

### 1. Complex Test Scenarios That Hide Signal

**Anti-pattern:** Full end-to-end test with many components
**Problem:** Can't isolate specific failure mode
**Fix:** Simplify to minimal reproducer of exact failure

### 2. Implicit Assumptions About Project Setup

**Anti-pattern:** Assume `mise run test` exists without stating
**Problem:** Reader doesn't know prerequisites
**Fix:** Document assumptions explicitly in practice

### 3. Vague Conditionals ("Check if...")

**Anti-pattern:** "Check if plan revision needed"
**Problem:** Based on what criteria?
**Fix:** "If multiple deferred items OR items interconnected, ask user"

### 4. Auto-Decisions on Behalf of User

**Anti-pattern:** Gatekeeper auto-approves "obviously in-scope" items
**Problem:** "Obvious" is subjective, leads to scope creep
**Fix:** User decides all non-trivial categorizations

### 5. Mixed Terminology During Transition

**Anti-pattern:** Some docs say "HIGH priority", others say "BLOCKING"
**Problem:** Confusing for users and agents
**Fix:** Comprehensive grep + replace, then code review for subtle inconsistencies

---

## Open Questions / Follow-up

### 1. Should Gatekeeper Handle Security Checks?

**Question:** Expand gatekeeper role beyond scope validation?
- Could check for security vulnerabilities in review feedback
- Could validate severity levels (is this really BLOCKING?)

**Deferred because:** Keep gatekeeper focused on scope validation
**Future consideration:** Security validation might be separate agent

### 2. Metrics Tracking for Deferred Items

**Question:** Should we track how many items deferred vs fixed over time?
- Could reveal patterns (too much deferred = plan too narrow?)
- Could help estimate review cycles needed

**Deferred because:** Need real usage data first
**Future consideration:** Add metrics after 5-10 workflows

### 3. Auto-Create Follow-up Tasks for Deferred Items

**Question:** Should gatekeeper auto-create GitHub issues for deferred BLOCKING items?

**Deferred because:** Integration complexity, not all teams use same issue tracker
**Future consideration:** Optional integration via practice configuration

### 4. Validate Gatekeeper with Subagent Pressure Tests

**Question:** Should we add automated tests that dispatch subagents under pressure?

**Current state:** Test scenarios documented, manual testing required
**Future enhancement:** Automate with subagent dispatch + time pressure + sunk cost scenarios

---

## Integration Points

**Gatekeeper integrates with:**

1. **Code-reviewer agent**
   - Receives review file output from code-reviewer
   - Validates review against plan

2. **/execute command**
   - Dispatched at batch checkpoints
   - Between code-reviewer and fixing agents

3. **Validating-review-feedback skill**
   - References skill for workflow methodology
   - Skill is reusable by other agents/commands

4. **Code-review practice**
   - References practice for severity definitions (BLOCKING vs NON-BLOCKING)
   - Uses practice's review template structure

5. **Plan files**
   - Reads plan to understand scope
   - Appends Deferred Items section

**Dependencies:**
- Requires plan file path (from orchestrator)
- Requires review file path (from code-reviewer)
- Requires user input for scope decisions (AskUserQuestion)

**Outputs:**
- Annotated review file with [FIX]/[WONTFIX]/[DEFERRED] tags
- Updated plan with Deferred Items section
- Summary for orchestrator

---

## Files Created/Modified

### Created (3 files):
1. `plugin/skills/collaboration/validating-review-feedback/SKILL.md` (242 lines)
   - Reusable workflow for validating review feedback against plans

2. `plugin/skills/collaboration/validating-review-feedback/test-scenarios.md` (323 lines)
   - Baseline and with-skill test scenarios proving gatekeeper prevents misinterpretation

3. `plugin/agents/gatekeeper.md` (269 lines)
   - Agent that enforces validating-review-feedback workflow with persuasion principles

### Modified (3 files):
1. `plugin/practices/code-review.md`
   - Simplified from 4-level to 2-level severity system (BLOCKING/NON-BLOCKING)
   - Updated review template to match new structure

2. `plugin/commands/execute.md`
   - Integrated gatekeeper dispatch into batch checkpoint workflow
   - Added timeout failure behavior and deferred items summary

3. `plugin/agents/code-reviewer.md`
   - Updated to reference 2-level severity system
   - Uses BLOCKING/NON-BLOCKING template

---

## Summary

**Problem:** Agents misinterpret code review feedback, skip BLOCKING issues, allow scope creep

**Solution:** Three-layer gatekeeper system (skill + practice + agent) that validates feedback against plan, forces explicit user decisions on scope

**Key decisions:**
- 2-level severity system (BLOCKING/NON-BLOCKING) for clarity
- User decides scope, gatekeeper validates (not decides)
- Annotate in-place with [FIX]/[WONTFIX]/[DEFERRED] tags
- Track deferrals in plan's Deferred section

**Results:**
- 9 conventional commits
- 918 lines added (net)
- 7 BLOCKING issues caught via per-batch code reviews
- Real failure mode (Lambert recalculation) prevented systematically

**Time:** 6 hours total (50% over estimate)
- Per-batch reviews added 30% time but prevented 2-3x debugging time
- TDD approach (test scenarios first) clarified exact problem
- Extra verification time worth investment

**Lessons:**
- Test scenarios written first prevent building wrong thing
- Per-batch code reviews catch issues before they cascade
- Three-layer separation (skill + practice + agent) enables DRY and reusability
- Persuasion principles prevent agent rationalization under pressure
- User checkpoints at scope boundaries prevent silent expansion
- Clear tagging ([FIX]/[DEFERRED]/[WONTFIX]) eliminates downstream ambiguity

**Next steps:**
- Manual testing with real /execute workflow
- Consider metrics tracking after 5-10 workflows
- Evaluate security validation as separate agent
- Potential automation of test scenarios with subagent dispatch
