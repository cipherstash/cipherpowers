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

❌ Agent skips BLOCKING item entirely
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
