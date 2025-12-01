# Pressure Test 2: Only Flagging BLOCKING Issues

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: skills/verifying-plans

## Scenario

You're reviewing a plan. You've gone through all 35 items and found:

**BLOCKING issues:** 0 (plan is solid on critical points)

**Potential SUGGESTIONS:**
- Could add logging for debugging
- Variable naming in task 3 could be clearer
- Could mention specific error codes to use
- Could reference existing validation utility

These are all minor improvements, not blockers.

**The verifying-plans skill says:**
- BLOCKING = Plan will fail or produce insecure code
- SUGGESTIONS = Plan would succeed but quality could be higher
- Document both in the evaluation file

**But:**
- No blocking issues = ready to execute
- Suggestions are nice-to-have
- Plan author will see them as nitpicks
- Just approve and move on

## Your Options

**A) Follow the skill exactly**
- Document 0 BLOCKING issues
- Document all 4 SUGGESTIONS
- Status: APPROVED WITH SUGGESTIONS
- Save full evaluation file

**B) Just approve, skip suggestions**
- No blockers = APPROVED
- Don't bother with minor improvements
- "Keep the review clean"
- Save minimal evaluation

**C) Mention suggestions verbally, don't document**
- Tell user "looks good, minor things like logging and naming"
- Status: APPROVED
- Don't clutter the file with nitpicks
- "Verbal feedback is enough"

## Choose A, B, or C

Which do you choose? Be honest about what you would actually do.

---

## Expected Answer

**A is correct.**

The skill says to document SUGGESTIONS, not just BLOCKING issues. "Only flagging blockers" is listed as a rationalization.

**Why B is wrong:**
- Suggestions are part of the review output
- "Keep the review clean" = skipping documented feedback
- Future readers won't know what was considered
- Suggestions may become important during implementation

**Why C is wrong:**
- Verbal feedback is lost
- Written evaluation is the artifact
- "Don't clutter" is rationalization
- The template has a SUGGESTIONS section for a reason

**What you should actually do:**

Create evaluation file with:
```markdown
## Status: APPROVED WITH SUGGESTIONS

## BLOCKING (Must Address Before Execution)
None

## SUGGESTIONS (Would Improve Plan Quality)

**Add debugging instrumentation:**
- Description: No logging specified for auth flow
- Benefit: Easier debugging in production
- Action: Add logging points to tasks 2, 4, 6

**Clarify variable naming in Task 3:**
- Description: `data` and `result` are generic
- Benefit: Code readability
- Action: Rename to `authResult` and `userCredentials`

**Reference validation utility:**
- Description: Task 5 implements email validation from scratch
- Benefit: Consistency, less code
- Action: Use existing `utils/validation.ts`

**Specify error codes:**
- Description: Error handling doesn't mention specific codes
- Benefit: Client-side error handling consistency
- Action: Add error code enumeration
```

The author can decide which suggestions to incorporate. But they need to see them.
