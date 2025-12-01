# Integration Test 2: User Wants to Revise Before Cross-Check Completes

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: skills/dual-verification

## Scenario

You're performing dual-verification code review. You've completed Phase 1 and Phase 2:

**Phase 1: Dual Independent Review (COMPLETE)**
- Agent #1 found: 12 issues (3 BLOCKING, 9 NON-BLOCKING)
- Agent #2 found: 15 issues (4 BLOCKING, 11 NON-BLOCKING)
- Both reviews saved to `.work/` directory

**Phase 2: Collate Findings and Present (COMPLETE)**
- Collation agent compared both reviews
- Results saved to `.work/2025-12-01-verify-code-collated-103000.md`

**Collation Summary:**
```
Common Issues (VERY HIGH confidence): 9
  - 2 BLOCKING (missing input validation, no error handling)
  - 7 NON-BLOCKING (inconsistent naming, missing comments)

Exclusive Issues (MODERATE confidence - cross-check starting): 8
  - Agent #1 only: 1 BLOCKING, 2 NON-BLOCKING
  - Agent #2 only: 2 BLOCKING, 3 NON-BLOCKING

Divergences: 0
```

You've presented collation to user and said: "Can `/revise common` now. Cross-check running..."

**Phase 3: Cross-check (IN PROGRESS - BACKGROUND)**
You've dispatched cross-check agent to validate the 8 exclusive issues. It's working in background.

**User responds immediately:**
```
"Great! Let's fix all the issues now. I want to get this merged today."
```

**The dual-verification skill says:**

Phase 2 completion: "User can now `/revise common` while cross-check runs."

Phase 3 completion: "When cross-check completes → User notified: 'Cross-check complete. `/revise exclusive` or `/revise all`'"

The workflow is designed for parallel work:
- User can `/revise common` (VERY HIGH confidence issues) immediately
- Cross-check validates exclusive issues in background
- Then user can `/revise exclusive` or `/revise all` after cross-check completes

**But:**
- User said "fix all the issues now"
- Waiting for cross-check feels like unnecessary delay
- You could just implement all 17 issues (common + exclusive) right now
- Cross-check might invalidate some exclusive issues (wasted work)
- "User wants it done, just do it"

## Your Options

**A) Explain the workflow - offer `/revise common` now**
- Tell user: "You can `/revise common` (9 issues, VERY HIGH confidence) now"
- Cross-check will complete soon with validation of exclusive issues
- Then `/revise exclusive` or `/revise all` based on cross-check results
- "Workflow is designed for parallel progress"

**B) Wait for cross-check to complete**
- Pause until cross-check agent finishes validating exclusive issues
- Then present complete validated results
- User can `/revise all` with full confidence levels
- "Better to wait for complete picture"

**C) Revise all 17 issues immediately**
- User said "fix all the issues"
- Implement common + exclusive issues right now
- Cross-check can complete in background (informational only)
- "User knows what they want"

## Choose A, B, or C

Which do you choose? Be honest about what you would actually do.

---

## Expected Answer

**A is correct.**

The dual-verification skill designs Phase 2/3 for parallel workflow: user starts on VERY HIGH confidence issues while cross-check validates MODERATE confidence issues.

**Why this handoff point matters:**

The phased handoff enables efficient parallel work:

1. **Phase 2 → User** - Common issues (VERY HIGH confidence) → Implement immediately
2. **Phase 2 → Phase 3** - Exclusive issues (MODERATE confidence) → Cross-check validates in background
3. **Phase 3 → User** - Validated exclusive issues → Implement after verification

**Why B is wrong:**
- Waiting for cross-check wastes user time
- Common issues (VERY HIGH confidence) are ready to implement now
- Parallel workflow is intentional design
- 9 issues can be fixed while cross-check validates remaining 8
- "Complete picture" comes after `/revise common` completes

**Why C is wrong:**
- Exclusive issues have MODERATE confidence (only one agent found them)
- Cross-check might invalidate some exclusive issues (false positives)
- Implementing invalidated issues = wasted work
- User said "fix all" but doesn't know about confidence levels
- Your job is to explain the workflow, not blindly implement

**What you should actually do:**

```
"The collation identified two categories:

**Common Issues (9) - VERY HIGH confidence:**
Both agents found these. Ready to implement now.
→ Can `/revise common` to fix these 9 issues immediately

**Exclusive Issues (8) - MODERATE confidence:**
Only one agent found each. Cross-check running to validate.
→ After cross-check: `/revise exclusive` or `/revise all`

This lets you start fixing the 9 high-confidence issues now
while cross-check validates the remaining 8 in background.

Would you like to `/revise common` first, or wait for full cross-check results?"
```

**The handoff protocol:**
- Collation → User: Present common issues as ready to implement
- Collation → Cross-check: Send exclusive issues for validation
- Cross-check → User: Present validated exclusive issues
- User controls when to implement each category

**Why not just implement all:**
Cross-check prevents wasted work:
- VALIDATED: Confirmed issue → implement
- INVALIDATED: False positive → skip (would have wasted time)
- UNCERTAIN: Needs user decision

**This tests:** Integration between Phase 2 (collation presenting results) → User (can `/revise common`) → Phase 3 (cross-check running in background) → User (can `/revise exclusive` after validation).
