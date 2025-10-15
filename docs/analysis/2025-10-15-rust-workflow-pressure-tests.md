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
