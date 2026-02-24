---
name: Collation Report Template
description: Structured format for collating two independent reviews with confidence levels, cross-check validation, and verification
when_to_use: when collating dual-verification reviews (plan reviews, code reviews, documentation reviews)
related_practices: code-review.md, development.md
version: 2.0.0
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
- **Cross-check Status:** [PENDING / COMPLETE]
- **Cross-check File:** [path to cross-check report, if complete]

## Executive Summary
- **Total unique issues identified:** X
- **Common issues (VERY HIGH confidence):** X → `/revise common`
- **Exclusive issues (pending cross-check):** X
  - VALIDATED: X (confirmed)
  - INVALIDATED: X (can skip)
  - UNCERTAIN: X (user decides)
- **Divergences (resolved during collation):** X

**Overall Status:** [BLOCKED / APPROVED WITH CHANGES / APPROVED]
**Revise Ready:** [common | all] (based on cross-check status)

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

## Exclusive Issues (Pending Cross-check)
Only one reviewer found these issues. Cross-check will validate against ground truth.

**Confidence: MODERATE** - One reviewer found these. Cross-check validates whether they actually apply.

**Cross-check Status:** [PENDING / COMPLETE]

### Found by Reviewer #1 Only

#### BLOCKING / CRITICAL
[List exclusive blocking issues from Reviewer #1, or "None"]

**[Issue title]** ([Location])
- **Found by:** Reviewer #1
- **Description:** [what was found]
- **Severity:** BLOCKING
- **Reasoning:** [why reviewer flagged as blocking]
- **Confidence:** MODERATE (pending cross-check)
- **Cross-check:** [PENDING / VALIDATED / INVALIDATED / UNCERTAIN]
- **Evidence:** [cross-check findings, if complete]
  - Example VALIDATED: "Confirmed - file `src/auth.ts` has no rate limiting middleware"
  - Example INVALIDATED: "File exists at `src/utils/helpers.ts:42`, reviewer missed it"
  - Example UNCERTAIN: "Could not locate endpoint to verify claim"

#### NON-BLOCKING / LOWER PRIORITY
[List exclusive non-blocking issues from Reviewer #1, or "None"]

**[Issue title]** ([Location])
- **Found by:** Reviewer #1
- **Description:** [what was suggested]
- **Severity:** NON-BLOCKING
- **Benefit:** [potential improvement]
- **Confidence:** MODERATE (pending cross-check)
- **Cross-check:** [PENDING / VALIDATED / INVALIDATED / UNCERTAIN]
- **Evidence:** [cross-check findings, if complete]

### Found by Reviewer #2 Only

#### BLOCKING / CRITICAL
[List exclusive blocking issues from Reviewer #2, or "None"]

**[Issue title]** ([Location])
- **Found by:** Reviewer #2
- **Description:** [what was found]
- **Severity:** BLOCKING
- **Reasoning:** [why reviewer flagged as blocking]
- **Confidence:** MODERATE (pending cross-check)
- **Cross-check:** [PENDING / VALIDATED / INVALIDATED / UNCERTAIN]
- **Evidence:** [cross-check findings, if complete]

#### NON-BLOCKING / LOWER PRIORITY
[List exclusive non-blocking issues from Reviewer #2, or "None"]

**[Issue title]** ([Location])
- **Found by:** Reviewer #2
- **Description:** [what was suggested]
- **Severity:** NON-BLOCKING
- **Benefit:** [potential improvement]
- **Confidence:** MODERATE (pending cross-check)
- **Cross-check:** [PENDING / VALIDATED / INVALIDATED / UNCERTAIN]
- **Evidence:** [cross-check findings, if complete]

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

### Immediate Actions → `/revise common`
[Common issues - both reviewers found them with VERY HIGH confidence. Can start immediately.]

- [ ] **[Issue]:** [brief action item]
- [ ] **[Issue]:** [brief action item]

### After Cross-check → `/revise exclusive`
[Exclusive issues pending cross-check validation]

**VALIDATED (implement):**
- [ ] **[Issue]** (Reviewer #[1/2]): [brief description]
  - Evidence: [what cross-check found]

**INVALIDATED (skip):**
- [ ] ~~**[Issue]**~~ (Reviewer #[1/2]): [brief description]
  - Reason: [why cross-check invalidated]

**UNCERTAIN (user decides):**
- [ ] **[Issue]** (Reviewer #[1/2]): [brief description]
  - Context: [what cross-check found but couldn't determine]

### For Consideration (NON-BLOCKING)
[Improvement suggestions found by one or both reviewers]

- [ ] **[Issue]:** [brief suggestion]
  - Benefit: [how this improves quality]
  - Found by: [Both / Reviewer #1 / Reviewer #2]
  - Cross-check: [VALIDATED / INVALIDATED / UNCERTAIN / N/A for common]

### Divergences (Resolved)
[Areas where reviewers disagreed - resolved during collation]

- [ ] **[Issue]:** [divergence description]
  - Resolution: [which perspective was correct]
  - Action: [what to do based on resolution]

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

### Parallel Workflow (Recommended)

1. **Now:** `/revise common` - Start implementing common issues immediately
2. **Background:** Cross-check validates exclusive issues
3. **When ready:** `/revise exclusive` - Implement validated exclusive issues
4. **Or:** `/revise all` - Implement everything actionable

### Sequential Workflow

**If BLOCKED:**
1. `/revise common` - Address all common BLOCKING issues (VERY HIGH confidence)
2. Wait for cross-check to complete
3. Review UNCERTAIN exclusive issues (user decides)
4. `/revise exclusive` - Address VALIDATED exclusive issues

**If APPROVED WITH CHANGES:**
1. `/revise common` - Address common issues
2. `/revise exclusive` - Address VALIDATED exclusive issues (optional)
3. Review NON-BLOCKING suggestions for future

**If APPROVED:**
- Proceed with execution/merge
- Optional: `/revise` for any improvements

### Cross-check States

| State | Meaning | Action |
|-------|---------|--------|
| VALIDATED | Cross-check confirmed issue exists | Implement via `/revise exclusive` |
| INVALIDATED | Cross-check found issue doesn't apply | Skip (auto-excluded from `/revise`) |
| UNCERTAIN | Cross-check couldn't determine | User reviews and decides |
