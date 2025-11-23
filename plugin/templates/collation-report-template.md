---
name: Collation Report Template
description: Structured format for collating two independent reviews with confidence levels and verification
when_to_use: when collating dual-verification reviews (plan reviews, code reviews, documentation reviews)
related_practices: code-review.md, development.md, testing.md
version: 1.0.0
---

# Collated Review Report - {Review Type}

## Metadata
- **Review Type:** [Plan Review / Code Review / Documentation Review / Other]
- **Date:** {YYYY-MM-DD HH:mm:ss}
- **Reviewers:** [Agent #1 type (e.g., plan-review-agent), Agent #2 type]
- **Subject:** [Path to plan file / Files reviewed / Content reviewed]
- **Review Files:**
  - Review #1: [path to first review]
  - Review #2: [path to second review]

## Executive Summary
- **Total unique issues identified:** X
- **Common issues (high confidence):** X
- **Exclusive issues (requires judgment):** X
- **Divergences (requires investigation):** X

**Overall Status:** [BLOCKED / APPROVED WITH CHANGES / APPROVED]

## Common Issues (High Confidence)
Both reviewers independently found these issues.

**Confidence: VERY HIGH** - Both reviewers found these issues independently, making them very likely to be real problems.

### BLOCKING / CRITICAL
[List common blocking/critical issues, or "None"]

**[Issue title]** ([Location])
- **Reviewer #1 finding:** [description and severity]
- **Reviewer #2 finding:** [description and severity]
- **Confidence:** VERY HIGH (both found independently)
- **Severity consensus:** BLOCKING
- **Action required:** [what must be done to resolve]

### NON-BLOCKING / LOWER PRIORITY
[List common non-blocking issues, or "None"]

**[Issue title]** ([Location])
- **Reviewer #1 finding:** [description and severity]
- **Reviewer #2 finding:** [description and severity]
- **Confidence:** VERY HIGH (both suggested independently)
- **Benefit:** [how this would improve quality]

## Exclusive Issues (Requires Judgment)
Only one reviewer found these issues.

**Confidence: MODERATE** - One reviewer found these. May be valid edge cases or may require judgment to assess.

### Found by Reviewer #1 Only

#### BLOCKING / CRITICAL
[List exclusive blocking issues from Reviewer #1, or "None"]

**[Issue title]** ([Location])
- **Found by:** Reviewer #1
- **Description:** [what was found]
- **Severity:** BLOCKING
- **Reasoning:** [why reviewer flagged as blocking]
- **Confidence:** MODERATE (requires judgment - only one reviewer found)
- **Recommendation:** [Review reasoning and decide whether to address]

#### NON-BLOCKING / LOWER PRIORITY
[List exclusive non-blocking issues from Reviewer #1, or "None"]

**[Issue title]** ([Location])
- **Found by:** Reviewer #1
- **Description:** [what was suggested]
- **Severity:** NON-BLOCKING
- **Benefit:** [potential improvement]
- **Confidence:** MODERATE (only one reviewer suggested)

### Found by Reviewer #2 Only

#### BLOCKING / CRITICAL
[List exclusive blocking issues from Reviewer #2, or "None"]

**[Issue title]** ([Location])
- **Found by:** Reviewer #2
- **Description:** [what was found]
- **Severity:** BLOCKING
- **Reasoning:** [why reviewer flagged as blocking]
- **Confidence:** MODERATE (requires judgment - only one reviewer found)
- **Recommendation:** [Review reasoning and decide whether to address]

#### NON-BLOCKING / LOWER PRIORITY
[List exclusive non-blocking issues from Reviewer #2, or "None"]

**[Issue title]** ([Location])
- **Found by:** Reviewer #2
- **Description:** [what was suggested]
- **Severity:** NON-BLOCKING
- **Benefit:** [potential improvement]
- **Confidence:** MODERATE (only one reviewer suggested)

## Divergences (Requires Investigation)
Reviewers disagree or have contradictory findings.

**Confidence: INVESTIGATE** - Reviewers have different conclusions. Verification analysis included.

[List divergences with verification, or "None"]

**[Issue title]** ([Location])
- **Reviewer #1 perspective:** [what Reviewer #1 says]
- **Reviewer #2 perspective:** [what Reviewer #2 says - different or contradictory]
- **Verification Analysis:**
  - **Verifying agent:** [cipherpowers:plan-review-agent or other]
  - **Correct perspective:** [Reviewer #1 / Reviewer #2 / Both have merit / Neither / Needs clarification]
  - **Reasoning:** [detailed explanation from verification agent]
  - **Recommendation:** [how to resolve this divergence]
- **Confidence:** [INVESTIGATE / Updated based on verification if resolved]
- **Action required:** [User decision needed / Follow verification recommendation / Other]

## Recommendations

### Immediate Actions (Common BLOCKING)
[Issues that should be addressed immediately - both reviewers found them with VERY HIGH confidence]

- [ ] **[Issue]:** [brief action item]
- [ ] **[Issue]:** [brief action item]

**Rationale:** Both reviewers independently identified these as blocking issues.

### Judgment Required (Exclusive BLOCKING)
[Issues where only one reviewer found blocking concerns - user should review reasoning and decide]

- [ ] **[Issue]** (Reviewer #[1/2] only): [brief description]
  - Review reasoning: [why this needs user judgment]

**Rationale:** Only one reviewer flagged as blocking. Assess whether this represents a real concern or edge case.

### For Consideration (NON-BLOCKING)
[Improvement suggestions found by one or both reviewers]

- [ ] **[Issue]:** [brief suggestion]
  - Benefit: [how this improves quality]
  - Found by: [Both / Reviewer #1 / Reviewer #2]

**Rationale:** These improvements are optional but would enhance quality.

### Investigation Needed (Divergences)
[Areas where reviewers disagree - verification analysis provided, but user makes final call]

- [ ] **[Issue]:** [divergence description]
  - Verification suggests: [recommendation from verification agent]
  - User should: [decide based on verification / investigate further / clarify requirements]

**Rationale:** Divergences indicate uncertainty that requires resolution before proceeding.

## Overall Assessment

**Ready to proceed?** [YES / NO / WITH CHANGES]

**Reasoning:**
[Brief explanation of decision based on collated findings]

**Critical items requiring attention:**
- [Item 1 if any]
- [Item 2 if any]

**Confidence level:**
- **High confidence issues (common):** [summary]
- **Moderate confidence issues (exclusive):** [summary]
- **Investigation required (divergences):** [summary]

## Next Steps

[What should happen next based on overall assessment]

**If BLOCKED:**
- Address all common BLOCKING issues (high confidence)
- Review and decide on exclusive BLOCKING issues (moderate confidence)
- Resolve divergences using verification recommendations

**If APPROVED WITH CHANGES:**
- Consider addressing common NON-BLOCKING suggestions (high confidence)
- Optionally review exclusive suggestions (moderate confidence)
- User decides on divergences if any remain

**If APPROVED:**
- Proceed with execution/merge
- Optional: Note suggestions for future improvements

---

## Template Usage Notes

**For collation agents:**
1. Replace all {placeholders} with actual values
2. Fill metadata section completely
3. Categorize ALL issues from both reviews
4. Include verification analysis for any divergences
5. Provide clear, actionable recommendations
6. Save to: `.work/{YYYY-MM-DD}-collated-{review-type}-{HHmmss}.md`

**Confidence levels explained:**
- **VERY HIGH:** Both reviewers found independently → Fix immediately
- **MODERATE:** One reviewer found → Requires judgment
- **INVESTIGATE:** Reviewers disagree → Verification provided, user decides

**When to use each section:**
- **Common Issues:** Same issue found by both (may have different wording)
- **Exclusive Issues:** Only one reviewer found
- **Divergences:** Reviewers contradict each other on same location/topic
