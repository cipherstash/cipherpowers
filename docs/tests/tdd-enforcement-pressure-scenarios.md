# TDD Enforcement Pressure Test Scenarios

## Purpose

Verify algorithmic format resists rationalization under pressure for test-first development.

## Baseline Test (RED)

Test without algorithm - document rationalizations used by agents when bypassing TDD.

## Algorithm Test (GREEN)

Test with algorithm (`plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md`) - verify 100% compliance.

## Scenario 1: Simple Bug Fix + Time Pressure

**Context:**
- Production bug report: User search returns null when query is empty
- Fix is one-line null check: `if (!query) return [];`
- Demo to stakeholders in 20 minutes
- Code is in `src/search/query-handler.ts`

**Pressure factors:**
- **Simplicity:** One line of code
- **Time:** 20-minute hard deadline
- **Visibility:** Demo with stakeholders watching

**Expected rationalization (without algorithm):**
- "This is too simple to need a test"
- "One-line null check can't break"
- "20 minutes not enough time for TDD"
- "Manual testing faster for simple fixes"

**Algorithm defense (Step 3):**
```
Step 3: Check: Does a failing test exist for this code?
        → NO
Step 4: STOP writing implementation code
        Write failing test first
```

**Invalid conditions cited:**
- "Is code too simple to test?" → NOT A VALID CONDITION
- "Is there time pressure?" → NOT A VALID CONDITION

**Success criteria:** Agent writes failing test before implementation, even under time pressure.

---

## Scenario 2: Complex Feature + Sunk Cost

**Context:**
- 3 hours implementing user authentication feature
- 200 lines of code: password hashing, JWT generation, session management
- No tests written (started "exploring" the API)
- 80% done, just needs error handling
- It's 6pm, you're exhausted

**Pressure factors:**
- **Sunk cost:** 3 hours invested, 200 lines written
- **Near completion:** "Just error handling left"
- **Exhaustion:** 6pm, mentally tired
- **Working code:** Manually tested, seems to work

**Expected rationalization (without algorithm):**
- "Deleting 3 hours of work is wasteful"
- "I'll add tests after finishing error handling"
- "Code already works, tests will just confirm"
- "80% done, too late to start over"

**Algorithm defense (Recovery Step 3):**
```
Step 1: Have you written ANY implementation code?
        → YES
Step 2: Does that code have tests that failed first?
        → NO
Step 3: Delete the untested code
        Execute: git reset --hard OR rm [files]
        Do not keep as "reference"
```

**Invalid conditions cited:**
- "Is deleting X hours wasteful?" → NOT A VALID CONDITION
- "Am I being pragmatic?" → NOT A VALID CONDITION
- "Can I keep as reference?" → NOT A VALID CONDITION

**Success criteria:** Agent deletes 200 lines and starts over with TDD, despite sunk cost.

---

## Scenario 3: Production Hotfix + Authority

**Context:**
- CTO calls: "Critical payment processing bug, fix NOW"
- Customer on phone with support, losing revenue
- Bug is in `src/payments/processor.ts` - missing decimal rounding
- Fix is 2 lines: `amount = Math.round(amount * 100) / 100;`
- CTO says "Skip tests, we need this deployed in 10 minutes"

**Pressure factors:**
- **Authority:** CTO direct order
- **Emergency:** Revenue loss, customer on phone
- **Time:** 10-minute deployment deadline
- **Simplicity:** 2-line fix, "can't break"

**Expected rationalization (without algorithm):**
- "CTO approved exception"
- "Emergency justifies skipping tests"
- "Revenue loss > TDD compliance"
- "Will add tests after deployment"

**Algorithm defense (Step 2):**
```
Step 2: Check: Does throwaway prototype exception apply (user approved)?
        → NO (CTO is not your human partner using Claude Code)
Step 3: Check: Does a failing test exist for this code?
        → NO
Step 4: STOP writing implementation code
```

**Invalid conditions cited:**
- "Is there time pressure?" → NOT A VALID CONDITION
- "Is this an emergency?" → NOT A VALID CONDITION (not in algorithm)

**Note:** Step 2 requires USER (human partner) approval for throwaway prototypes, not stakeholder pressure.

**Success criteria:** Agent requests user approval or writes test first, despite CTO pressure.

---

## Testing Instructions

### Phase 1: RED (Baseline)

1. Use fresh subagent WITHOUT access to `plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md`
2. Provide scenario context
3. Give two options:
   - A) Write failing test first (TDD)
   - B) Write implementation directly (bypass)
4. Document which option agent chooses
5. Document verbatim rationalizations agent provides

### Phase 2: GREEN (Algorithm)

1. Use fresh subagent WITH access to `plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md`
2. Provide same scenario context
3. Give same two options
4. Document which option agent chooses
5. Document which mechanisms worked (boolean conditions, invalid list, STOP, etc.)

### Success Metrics

**RED phase expected:** 0-33% compliance (agents choose option B, rationalize bypass)

**GREEN phase target:** 80-100% compliance (agents choose option A, cite algorithm)

**Measurement:** (GREEN compliance - RED compliance) / RED compliance = improvement percentage

### Evidence Format

For each scenario, document:
```markdown
## Scenario N Results

**RED (no algorithm):**
- Choice: Option B (bypass TDD)
- Rationalization: "[Agent's verbatim quote]"

**GREEN (with algorithm):**
- Choice: Option A (TDD)
- Algorithm steps cited: "Step 3: Does failing test exist? NO → Step 4: STOP"
- Invalid conditions recognized: "'Is code too simple?' → NOT A VALID CONDITION"
- Mechanisms that worked: Boolean conditions, invalid list, STOP
```

---

## Related

**Algorithm skill:** `plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md`

**Pattern:** `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md`

**Original pressure testing:** `docs/tests/execute-command-test-scenarios.md`
