# Collated Review Report - Agent Consistency Review

## Metadata
- **Review Type:** Agent Consistency Review
- **Date:** 2025-12-08 15:30:00
- **Reviewers:** technical-writer #1, code-agent #2
- **Subject:** plugin/agents/** (14 agents total)
- **Review Files:**
  - Review #1: `.work/2025-12-08-verify-agents-1.md`
  - Review #2: `.work/2025-12-08-verify-agents-2.md`
- **Cross-check Status:** PENDING
- **Cross-check File:** N/A (pending)

## Executive Summary
- **Total unique issues identified:** 2
- **Common issues (VERY HIGH confidence):** 2 → `/revise common`
- **Exclusive issues (pending cross-check):** 0
  - VALIDATED: N/A
  - INVALIDATED: N/A
  - UNCERTAIN: N/A
- **Divergences (resolved during collation):** 1

**Overall Status:** APPROVED WITH CHANGES
**Revise Ready:** common (both critical agents identified by both reviewers)

## Common Issues (High Confidence)
Both reviewers independently found these issues.

**Confidence: VERY HIGH** - Both reviewers found these issues independently, making them very likely to be real problems.

### BLOCKING / CRITICAL

**ultrathink-debugger.md - Over-engineered with workflow logic** (plugin/agents/ultrathink-debugger.md)
- **Reviewer #1 finding:** 116 lines (vs ~30-35 expected), contains extensive workflow logic, announcement script, specialization guide, complex scenario techniques, red flags table, completion criteria, purpose/behavioral traits. BLOCKING - violates thin skill-delegation pattern significantly
- **Reviewer #2 finding:** 115 lines (362% of exemplar average), contains announcement template, specialization guidance, detailed techniques, red flags table, completion criteria, purpose/behavioral traits outside instructions. OVER-ENGINEERED
- **Confidence:** VERY HIGH (both found independently)
- **Severity consensus:** BLOCKING
- **Action required:** Extract ~60-80 lines of workflow content to systematic-debugging skill. Reduce agent to ~35-40 lines following exemplar pattern (65% reduction)

**review-collation-agent.md - Contains workflow logic in agent** (plugin/agents/review-collation-agent.md)
- **Reviewer #1 finding:** 84 lines, hybrid pattern with content after `</instructions>`, contains non-negotiable requirements, final message format template, red flags table, purpose/capabilities sections. MODERATE severity - consider standardizing but not blocking
- **Reviewer #2 finding:** 83 lines (262% of exemplar average), contains detailed "Non-Negotiable Requirements", "Final Message Format" with exact template, red flags table, purpose/capabilities outside instructions. OVER-ENGINEERED
- **Confidence:** VERY HIGH (both found independently)
- **Severity consensus:** BLOCKING (elevated from Reviewer #1's MODERATE based on pattern consistency)
- **Action required:** Extract workflow logic to dual-verification skill (non-negotiable requirements, final message format, red flags table, purpose/capabilities). Reduce from 83-84 lines to ~30 lines (64% reduction)

### NON-BLOCKING / LOWER PRIORITY

None

## Exclusive Issues (Pending Cross-check)
Only one reviewer found these issues. Cross-check will validate against ground truth.

**Confidence: MODERATE** - One reviewer found these. Cross-check validates whether they actually apply.

**Cross-check Status:** PENDING

### Found by Reviewer #1 Only

#### BLOCKING / CRITICAL
None

#### NON-BLOCKING / LOWER PRIORITY
None

### Found by Reviewer #2 Only

#### BLOCKING / CRITICAL
None

#### NON-BLOCKING / LOWER PRIORITY
None

## Divergences (Requires Investigation)
Reviewers disagree or have contradictory findings.

**Confidence: INVESTIGATE** - Reviewers have different conclusions. Verification analysis included.

**path-test-agent.md - Classification difference** (plugin/agents/path-test-agent.md)
- **Reviewer #1 perspective:** 68 lines, test agent exception, not production pattern. Recommendation: No action required - test agent exception (N/A severity)
- **Reviewer #2 perspective:** 67 lines, special purpose (not production), acceptable as-is. Recommendation: Consider moving to plugin/tests/ directory (optional), add comment clarifying test infrastructure (⚠️ SPECIAL severity)
- **Verification Analysis:**
  - **Verifying agent:** review-collation-agent
  - **Correct perspective:** Both have merit - this is a convergent finding, not a divergence
  - **Reasoning:** Both reviewers agree this is a test/utility agent with different requirements from production agents. Line count difference (68 vs 67) is negligible. Both recommend accepting as-is with optional improvements (moving to tests/ directory, adding clarifying comment)
  - **Recommendation:** Accept as test utility exception. Optional: move to plugin/tests/ if that directory exists, add comment clarifying purpose
- **Confidence:** RESOLVED (convergent finding)
- **Action required:** None (optional improvements can be considered later)

## Recommendations

### Immediate Actions → `/revise common`
[Common issues - both reviewers found them with VERY HIGH confidence. Can start immediately.]

- [ ] **ultrathink-debugger.md:** Extract workflow logic to systematic-debugging skill
  - Extract announcement template (lines 37-48) → skill opening section
  - Extract specialization guide (lines 50-59) → skill `when_to_use` frontmatter
  - Extract complex scenario techniques (lines 61-87) → skill detailed procedures
  - Extract red flags table (lines 89-95) → skill enforcement section
  - Extract completion criteria (lines 97-103) → skill workflow steps
  - Extract purpose/behavioral traits (lines 105-116) → skill description
  - Reduce agent to ~35-40 lines following exemplar pattern

- [ ] **review-collation-agent.md:** Extract workflow logic to dual-verification skill
  - Extract non-negotiable requirements → skill workflow
  - Extract final message format → skill output section
  - Extract red flags table → skill enforcement section
  - Extract purpose/capabilities → skill description
  - Reduce agent to ~30 lines following exemplar pattern

### After Cross-check → `/revise exclusive`
[Exclusive issues pending cross-check validation]

**VALIDATED (implement):**
None

**INVALIDATED (skip):**
None

**UNCERTAIN (user decides):**
None

### For Consideration (NON-BLOCKING)
[Improvement suggestions found by one or both reviewers]

- [ ] **path-test-agent.md:** Clarify test agent status
  - Benefit: Makes clear this is test infrastructure, not production pattern
  - Found by: Both reviewers (convergent finding)
  - Cross-check: N/A (documentation improvement)
  - Optional: Move to plugin/tests/ directory if it exists
  - Optional: Add comment in file clarifying test purpose

### Divergences (Resolved)
[Areas where reviewers disagreed - resolved during collation]

- [ ] **path-test-agent.md classification:** Convergent finding, not divergence
  - Resolution: Both reviewers correctly identified this as test utility exception
  - Action: Accept as-is (optional improvements listed above)

## Overall Assessment

**Ready to proceed?** WITH CHANGES

**Reasoning:**
Both reviewers independently identified the same two critical issues (ultrathink-debugger.md and review-collation-agent.md) as over-engineered agents containing workflow logic that should be extracted to skills. This high agreement (VERY HIGH confidence) indicates clear violations of the thin skill-delegation pattern.

The remaining 12 agents show strong compliance (79-86% depending on how path-test-agent is counted), demonstrating that the exemplar pattern is well-established and achievable. The two non-compliant agents are clear outliers that need refactoring to match the established pattern.

**Critical items requiring attention:**
1. ultrathink-debugger.md - Reduce from 115-116 lines to ~35-40 lines (65% reduction)
2. review-collation-agent.md - Reduce from 83-84 lines to ~30 lines (64% reduction)

**Confidence level:**
- **High confidence issues (common):** 2 agents need major refactoring (both reviewers independently identified with identical assessments)
- **Moderate confidence issues (exclusive):** 0 exclusive issues
- **Investigation required (divergences):** 1 divergence resolved as convergent finding (path-test-agent is test utility exception)

## Next Steps

[What should happen next based on overall assessment]

### Parallel Workflow (Recommended)

1. **Now:** `/revise common` - Start refactoring ultrathink-debugger and review-collation-agent immediately
2. **Background:** Cross-check validates if there are any missed agents or patterns
3. **When ready:** Review cross-check findings for additional insights
4. **Optional:** Implement non-blocking improvements (path-test-agent clarifications)

### Sequential Workflow

**If APPROVED WITH CHANGES:**
1. `/revise common` - Address common issues (refactor 2 agents)
2. Review NON-BLOCKING suggestions for future (path-test-agent optional improvements)
3. Proceed with documentation updates to reflect pattern compliance

### Detailed Action Plan

**Priority 1 (BLOCKING - ultrathink-debugger.md):**

Create or enhance `systematic-debugging` skill:
1. Add announcement template to skill opening section
2. Add specialization guidance to skill `when_to_use` frontmatter:
   - Multi-component failures (data flows through 3+ layers)
   - Environment-specific issues (works locally, fails in production/CI)
   - Timing/concurrency issues (intermittent, race conditions)
   - Integration failures (external APIs, authentication)
   - Production emergencies requiring forensics
3. Add complex scenario techniques to skill body:
   - Multi-Component Systems diagnostics
   - Environment-Specific Failures procedures
   - Timing/Concurrency Issues investigation
   - Integration Failures debugging
4. Add red flags table to skill enforcement:
   - "I see the issue, skip process" → Complex bugs DECEIVE
   - "Fix where error appears" → Symptom ≠ root cause
   - "Should work now" → NO claims without verification
   - "No time for process" → Systematic is FASTER
5. Add completion criteria to skill workflow:
   - Root cause identified (not symptoms)
   - Fix addresses root cause
   - Verification command run with evidence
   - No regression in related functionality

Reduce ultrathink-debugger.md to:
```markdown
---
name: ultrathink-debugger
description: Complex debugging specialist for multi-component systems and production issues
color: red
---

You are an ultrathink expert debugging specialist for complex, multi-layered software problems.

<instructions>
## Instructions

## MANDATORY: Skill Activation

Use and follow these debugging skills:

**Primary:**
- Systematic Debugging: `${CLAUDE_PLUGIN_ROOT}skills/systematic-debugging/SKILL.md`
- Tool: `Skill(skill: "cipherpowers:systematic-debugging")`

**Supporting (when applicable):**
- Root Cause Tracing: `${CLAUDE_PLUGIN_ROOT}skills/root-cause-tracing/SKILL.md`
- Defense in Depth: `${CLAUDE_PLUGIN_ROOT}skills/defense-in-depth/SKILL.md`

Do NOT proceed without activating systematic-debugging skill.

## MANDATORY: Context

Read before starting:
- @README.md
- @CLAUDE.md

## MANDATORY: Standards

- ${CLAUDE_PLUGIN_ROOT}principles/testing.md
- ${CLAUDE_PLUGIN_ROOT}principles/development.md

</instructions>
```

**Priority 1 (BLOCKING - review-collation-agent.md):**

Enhance `dual-verification` skill:
1. Add non-negotiable requirements to skill workflow:
   - Read BOTH reviews completely before starting
   - Use template EXACTLY (no custom sections)
   - Mark exclusive issues as "pending cross-check"
   - Save report to .work/ directory
   - Announce `/cipherpowers:revise common` availability in final message
2. Add final message format to skill output section:
   ```
   Collated report saved to: [path]

   **Executive Summary:**
   - Common issues: X (VERY HIGH confidence) → `/cipherstash:revise common` ready
   - Exclusive issues: X (pending cross-check)
   - Divergences: X (resolved during collation)

   **Status:** [BLOCKED / APPROVED WITH CHANGES / APPROVED]

   **Next steps:**
   - `/cipherstash:revise common` - Start implementing common issues now
   - Cross-check will validate exclusive issues in background
   ```
3. Add red flags table to skill enforcement:
   - "Reviews mostly agree, skip detailed comparison" → Compare systematically
   - "Exclusive issue probably wrong" → Present with MODERATE confidence
   - "Divergence is minor, I'll pick one" → Resolve during collation
   - "Template is too simple" → Use template exactly
   - "I should add analysis" → Your job is collation, not a third review
4. Add collation methodology to skill body:
   - Common issues → VERY HIGH confidence (both found independently)
   - Exclusive issues → MODERATE confidence (pending cross-check)
   - Divergence resolution process

Reduce review-collation-agent.md to:
```markdown
---
name: review-collation-agent
description: Systematic collation of dual independent reviews (works for any review type)
color: cyan
---

You are the Review Collator - the systematic analyst who compares two independent reviews.

<instructions>
## Instructions

## MANDATORY: Skill Activation

Use and follow the dual-verification skill exactly as written.

Path: `${CLAUDE_PLUGIN_ROOT}skills/dual-verification/SKILL.md`

Tool: `Skill(skill: "cipherpowers:dual-verification")`

Do NOT proceed without completing skill activation.

## MANDATORY: Context

Read before starting:
- @README.md
- @CLAUDE.md

## Save Workflow

Use template: `${CLAUDE_PLUGIN_ROOT}templates/verify-collation-template.md`

Save to: `.work/{YYYY-MM-DD}-verify-{type}-collated-{HHmmss}.md`

Announce file path in final response.

</instructions>
```

**Priority 2 (OPTIONAL - path-test-agent.md):**
1. Add comment at top of file: `<!-- TEST AGENT: This is test infrastructure for verifying path resolution, not a production agent -->`
2. Consider moving to `plugin/tests/` directory if it exists
3. Update documentation to clarify test agent exceptions to pattern

### Cross-check States

| State | Meaning | Action |
|-------|---------|--------|
| VALIDATED | Cross-check confirmed issue exists | Implement via `/revise exclusive` |
| INVALIDATED | Cross-check found issue doesn't apply | Skip (auto-excluded from `/revise`) |
| UNCERTAIN | Cross-check couldn't determine | User reviews and decides |

## Collation Analysis

### Agreement Statistics
- **Total agents reviewed:** 14
- **Agents flagged by both reviewers:** 2 (ultrathink-debugger.md, review-collation-agent.md)
- **Agents flagged by only Reviewer #1:** 0
- **Agents flagged by only Reviewer #2:** 0
- **Agreement rate:** 100% (both reviewers identified identical critical issues)

### Pattern Compliance (Combined Analysis)
- **Fully compliant agents:** 11 (79%)
  - commit-agent.md (28 lines)
  - code-exec-agent.md (29-30 lines)
  - code-agent.md (31-32 lines)
  - code-review-agent.md (32-33 lines)
  - gatekeeper.md (34-35 lines)
  - plan-review-agent.md (35-36 lines)
  - rust-exec-agent.md (35-36 lines)
  - research-agent.md (37-38 lines)
  - rust-agent.md (37-38 lines)
  - execute-review-agent.md (40-41 lines)
  - technical-writer.md (45-46 lines)

- **Test/utility exception:** 1 (7%)
  - path-test-agent.md (67-68 lines) - Both reviewers accept as test utility

- **Non-compliant (requiring refactor):** 2 (14%)
  - review-collation-agent.md (83-84 lines) → target ~30 lines
  - ultrathink-debugger.md (115-116 lines) → target ~35-40 lines

### Exemplar Pattern Validation

Both reviewers identified the same exemplar pattern:
- **Target:** 30-50 lines (exemplars: 28-38 lines)
- **Structure:** YAML frontmatter → persona → `<instructions>` → skill activation → context → standards → save workflow → `</instructions>`
- **Principle:** Thin skill-delegation (workflow logic in skills, not agents)
- **Characteristics:** No red flags tables, no detailed procedures, no rationalization defenses in agents

### Content Extraction Summary

**From ultrathink-debugger.md (60-80 lines to extract):**
1. Announcement template → systematic-debugging skill
2. Specialization guidance → systematic-debugging skill `when_to_use`
3. Complex scenario techniques → systematic-debugging skill body
4. Red flags table → systematic-debugging skill enforcement
5. Completion criteria → systematic-debugging skill workflow
6. Purpose/behavioral traits → systematic-debugging skill description

**From review-collation-agent.md (40-50 lines to extract):**
1. Non-negotiable requirements → dual-verification skill workflow
2. Final message format → dual-verification skill output section
3. Red flags table → dual-verification skill enforcement
4. Purpose/capabilities → dual-verification skill description

**Total extraction:** ~100-130 lines of content moving from agents to skills
**Total reduction:** ~198 lines → ~70 lines (65% reduction)

### Quality Observations

**Positive findings (both reviewers noted):**
1. 79% compliance rate demonstrates successful pattern adoption
2. Exemplar agents provide clear model (commit-agent, code-review-agent, rust-exec-agent)
3. Consistent use of YAML frontmatter across all agents
4. Correct `${CLAUDE_PLUGIN_ROOT}` path references throughout
5. Proper `<instructions>` wrapper usage in all agents
6. Clear skill activation syntax (Path + Tool pattern)

**Architectural strengths (both reviewers noted):**
1. Clear separation of concerns in compliant agents
2. Skill-centric architecture (agents reference, don't duplicate)
3. Single source of truth in skills
4. Maintainable through delegation pattern

### Reviewer Alignment Analysis

**Perfect alignment on:**
- ultrathink-debugger.md as most critical issue (both: BLOCKING/OVER-ENGINEERED)
- review-collation-agent.md as second critical issue (both: workflow logic in agent)
- path-test-agent.md as acceptable test utility exception
- Exemplar pattern characteristics (30-40 lines, pure delegation)
- Content to extract (same sections identified by both)
- Target reductions (both calculated ~64-65% reduction)

**Minor differences:**
- Line counts differ by 1 (Reviewer #1: 116 & 84, Reviewer #2: 115 & 83) - measurement variance
- Reviewer #1 initially rated review-collation-agent as MODERATE; Reviewer #2 rated as BLOCKING
- Resolution: Elevated to BLOCKING based on pattern consistency principle

**Convergent findings:**
- Both identified same 11 compliant agents
- Both identified same 2 non-compliant agents
- Both recommended same extraction strategy
- Both used exemplars for pattern validation
