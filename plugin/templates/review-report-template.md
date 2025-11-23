---
name: Review Report Template
description: Generic structured format for individual reviews in dual-verification pattern
when_to_use: when conducting independent reviews as part of dual-verification (documentation, architecture, etc.)
related_practices: code-review.md, development.md, testing.md
version: 1.0.0
---

# {Agent Type} Review Report

## Metadata
- **Agent Type:** [technical-writer / plan-review-agent / code-review-agent / other]
- **Review Type:** [Documentation / Plan / Code / Architecture / Other]
- **Date:** {YYYY-MM-DD HH:mm:ss}
- **Subject:** [What is being reviewed - file path(s), plan location, etc.]
- **Ground Truth:** [What this review is verified against - codebase, requirements, standards, etc.]
- **Context:** Independent review #[1/2] - Another agent is reviewing in parallel

## Executive Summary
- **Total issues found:** X
- **Severity breakdown:**
  - Critical/Blocking: X
  - High: X
  - Medium: X
  - Low: X

**Status:** [BLOCKED / APPROVED WITH SUGGESTIONS / APPROVED]

## Issues by Category

### {Category Name 1}
[e.g., Security & Correctness, Testing, Architecture, Documentation Accuracy, etc.]

#### {Specific Location or Subcategory}
[e.g., File path:line, Section name, Component name]

**Issue: {Issue title}**
- **Current content/state:** [What the subject currently says or how it currently works]
- **Actual ground truth:** [What is actually true based on verification]
- **Impact:** [Why this matters - what could go wrong or what would improve]
- **Severity:** [CRITICAL / HIGH / MEDIUM / LOW or BLOCKING / NON-BLOCKING]
- **Action:** [Specific, actionable recommendation to resolve]

**Issue: {Issue title}**
- **Current content/state:** [What the subject currently says or how it currently works]
- **Actual ground truth:** [What is actually true based on verification]
- **Impact:** [Why this matters]
- **Severity:** [level]
- **Action:** [Specific recommendation]

### {Category Name 2}
[Repeat structure for each category]

#### {Specific Location or Subcategory}

**Issue: {Issue title}**
- **Current content/state:** [...]
- **Actual ground truth:** [...]
- **Impact:** [...]
- **Severity:** [...]
- **Action:** [...]

## Summary

### Critical/Blocking Issues
[Quick list of issues that must be addressed, or "None"]

1. [{Category}] {Issue title} - {Location}
2. [{Category}] {Issue title} - {Location}

### High Priority Issues
[Issues that should be addressed, or "None"]

1. [{Category}] {Issue title} - {Location}
2. [{Category}] {Issue title} - {Location}

### Medium/Low Priority Issues
[Issues for consideration, or "None"]

1. [{Category}] {Issue title} - {Location}
2. [{Category}] {Issue title} - {Location}

## Overall Assessment

**Ready to proceed?** [YES / NO / WITH CHANGES]

**Reasoning:**
[Brief explanation of decision and any critical items that need attention]

**Confidence in findings:**
[Any areas where verification was limited or uncertain]

## Next Steps

[What should happen next based on this review]

**If BLOCKED:**
- Address all critical/blocking issues before proceeding
- Review high priority issues for potential blocking concerns

**If APPROVED WITH SUGGESTIONS:**
- Consider addressing high priority issues
- Medium/low priority issues optional for this iteration

**If APPROVED:**
- Proceed with next step (merge, execution, publication, etc.)

---

## Template Usage Notes

**For review agents in dual-verification pattern:**

1. **Replace all {placeholders} with actual values**
   - {Agent Type}: Your agent type (technical-writer, plan-review-agent, etc.)
   - {Review Type}: Type of review being conducted
   - {Category Name}: Relevant categories for this review type
   - {Specific Location}: Where the issue was found
   - {Issue title}: Concise, descriptive title

2. **Categorize issues appropriately**
   - Use categories relevant to review type
   - Documentation: Accuracy, Completeness, Examples, References
   - Plan: Security, Testing, Architecture, Error Handling, Code Quality, Process
   - Code: Security, Testing, Architecture, Error Handling, Code Quality, Process
   - Architecture: Design Principles, Scalability, Maintainability, Complexity

3. **Severity levels should be consistent**
   - CRITICAL/BLOCKING: Must fix before proceeding
   - HIGH: Should fix, may become blocking
   - MEDIUM: Good to fix, improves quality
   - LOW: Nice to have, minor improvement

4. **Save with timestamp for parallel execution**
   - Documentation: `.work/{YYYY-MM-DD}-doc-review-{HHmmss}.md`
   - Plan: `.work/{YYYY-MM-DD}-plan-evaluation-{HHmmss}.md`
   - Code: `.work/{YYYY-MM-DD}-code-review-{HHmmss}.md`
   - Other: `.work/{YYYY-MM-DD}-{review-type}-{HHmmss}.md`

5. **Work independently**
   - Don't coordinate with other reviewer
   - Time-based naming prevents conflicts
   - Collation agent will compare both reviews

6. **Verify every claim**
   - Current content CANNOT be assumed correct
   - Check against ground truth thoroughly
   - Provide specific evidence

## Example: Documentation Review

```markdown
# Technical Writer Review Report

## Metadata
- **Agent Type:** technical-writer
- **Review Type:** Documentation
- **Date:** 2025-11-24 14:30:52
- **Subject:** README.md, CLAUDE.md
- **Ground Truth:** Current codebase implementation
- **Context:** Independent review #1 - Another technical-writer is reviewing in parallel

## Executive Summary
- **Total issues found:** 7
- **Severity breakdown:**
  - Critical: 1
  - High: 2
  - Medium: 3
  - Low: 1

**Status:** APPROVED WITH SUGGESTIONS

## Issues by Category

### File Path Accuracy

#### README.md - Development Commands section

**Issue: Incorrect command reference**
- **Current content:** Documentation says "Run `npm run check` for quality checks"
- **Actual ground truth:** Package.json has `npm run lint` and no `check` script
- **Impact:** Users will get command not found errors
- **Severity:** HIGH
- **Action:** Update to reference actual npm scripts in package.json

### Examples and Code Snippets

#### CLAUDE.md - Template references

**Issue: Missing plugin path prefix**
- **Current content:** Shows `@skills/conducting-plan-review/SKILL.md`
- **Actual ground truth:** Should be `@${CLAUDE_PLUGIN_ROOT}skills/conducting-plan-review/SKILL.md`
- **Impact:** References won't resolve correctly
- **Severity:** CRITICAL
- **Action:** Add ${CLAUDE_PLUGIN_ROOT} prefix to all skill references

## Summary

### Critical/Blocking Issues
1. [Examples] Missing plugin path prefix - CLAUDE.md template references

### High Priority Issues
1. [File Paths] Incorrect command reference - README.md
2. [File Paths] Outdated directory structure - README.md

### Medium/Low Priority Issues
1. [Completeness] Missing example for git workflow - CLAUDE.md
2. [Accuracy] Version number doesn't match package.json - README.md
3. [Examples] Code snippet syntax highlighting incorrect - README.md

## Overall Assessment

**Ready to proceed?** WITH CHANGES

**Reasoning:**
One critical issue (template references) must be fixed. Two high priority issues should be addressed to prevent user confusion. Medium/low issues are optional improvements.

## Next Steps

- Fix critical issue (plugin path prefix)
- Address high priority command and directory issues
- Consider medium priority completeness improvements
```

## Example: Plan Review

```markdown
# Plan Review Agent Report

## Metadata
- **Agent Type:** plan-review-agent
- **Review Type:** Plan
- **Date:** 2025-11-24 14:35:22
- **Subject:** docs/plans/add-authentication.md
- **Ground Truth:** 35-point quality checklist from conducting-plan-review skill
- **Context:** Independent review #2 - Another plan-review-agent is reviewing in parallel

## Executive Summary
- **Total issues found:** 4
- **Severity breakdown:**
  - Blocking: 2
  - Non-blocking: 2

**Status:** BLOCKED

## Issues by Category

### Security & Correctness

#### Task 3: Implement JWT token validation

**Issue: Missing token expiration strategy**
- **Current content:** Plan says "Validate JWT tokens" with no expiration mention
- **Actual ground truth:** Security checklist requires token expiration and refresh strategy
- **Impact:** Tokens could be valid indefinitely, security vulnerability
- **Severity:** BLOCKING
- **Action:** Add task for token expiration configuration and refresh token implementation

### Testing

#### Overall test strategy

**Issue: No test-first approach specified**
- **Current content:** Tests mentioned at end "Add tests for authentication"
- **Actual ground truth:** TDD checklist requires test-first for each task
- **Impact:** Implementation may not be testable, tests may not actually verify behavior
- **Severity:** BLOCKING
- **Action:** Restructure each task to follow RED-GREEN-REFACTOR: write test, see it fail, implement, see it pass

## Summary

### Critical/Blocking Issues
1. [Security] Missing token expiration strategy - Task 3
2. [Testing] No test-first approach specified - Overall structure

### High Priority Issues
None

### Medium/Low Priority Issues
1. [Code Quality] Consider adding logging requirements - Task 4
2. [Process] Could specify commit points - Overall structure

## Overall Assessment

**Ready to proceed?** NO

**Reasoning:**
Two blocking issues must be addressed:
1. Security: Token expiration is critical for auth systems
2. Testing: TDD approach ensures tests actually verify behavior

## Next Steps

- Update plan to include token expiration and refresh strategy
- Restructure all tasks to follow test-first approach
- After fixes, ready for execution
```
