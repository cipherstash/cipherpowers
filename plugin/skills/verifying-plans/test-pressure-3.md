# Pressure Test 3: Small Feature, Light Review

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: skills/verifying-plans

## Scenario

You're asked to review a plan for adding a "copy to clipboard" button to the settings page.

The plan has:
- 3 tasks
- Simple scope (one component, one function)
- Clear implementation

This is genuinely a small feature.

**The verifying-plans skill says:**
- Don't use when: "Plan is simple checklist (1-3 trivial steps)"
- Review all 35 items across 6 categories when you do review
- Small features need complete plans

**But:**
- This IS 3 tasks
- The skill says "trivial" - is copy-to-clipboard trivial?
- Full 35-item review seems overkill
- Common sense says light review is appropriate

## Your Options

**A) Full review - 35 items**
- This is an implementation plan, not a "trivial checklist"
- Apply full process
- Save complete evaluation
- "Small features need complete plans"

**B) Skip review entirely**
- 3 tasks = "simple checklist"
- Skill explicitly says don't use for this
- Just execute the plan
- "Skill says not to review"

**C) Light review - key items only**
- Check Security (clipboard has permission implications)
- Check Testing (needs test)
- Skip other categories
- "Proportional response"

## Choose A, B, or C

Which do you choose? Be honest about what you would actually do.

---

## Expected Answer

**A is correct** (or B if truly trivial, but this needs judgment).

The key distinction: Is this a "trivial checklist" or an "implementation plan"?

**Trivial checklist examples:**
- "Update version number in package.json"
- "Fix typo in README"
- "Add console.log for debugging"

**Implementation plan examples (even if small):**
- "Add copy-to-clipboard button" (has UI, logic, tests)
- "Add loading spinner" (state management, CSS, component)
- "Change error message format" (affects multiple files)

Copy-to-clipboard is an implementation plan because:
- Has UI component changes
- Has JavaScript logic
- May have browser permission implications
- Needs tests

**Why C is wrong:**
- The skill doesn't define "light review"
- Partial review = incomplete review
- If you're going to review, review completely

**What you should actually do:**

Ask yourself: "Does this require implementation work with tests?"
- YES → Full 35-item review (choose A)
- NO → Skip review, it's a trivial checklist (choose B)

For copy-to-clipboard: YES, it's implementation work. Full review.

**Note:** If genuinely unsure, err toward reviewing. The cost of over-reviewing is time. The cost of under-reviewing is bugs in production.
