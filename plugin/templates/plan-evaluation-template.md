---
name: Plan Evaluation Template
description: Structured format for evaluating implementation plan quality before execution
when_to_use: when reviewing implementation plans to ensure they address all quality criteria
related_practices: code-review.md, development.md, testing.md
version: 1.0.0
---

# Plan Evaluation - {Date}

## Metadata
- **Reviewer:** [Agent name/type, e.g., plan-review-agent]
- **Date:** {YYYY-MM-DD HH:mm:ss}
- **Plan Location:** [Path to plan file]
- **Context:** [Standalone review / Independent review #[1/2] for dual-verification]

## Status: [BLOCKED | APPROVED WITH SUGGESTIONS | APPROVED]

<!--
Status guidance:
- BLOCKED: Plan has critical gaps that must be addressed before execution
- APPROVED WITH SUGGESTIONS: Plan is executable but could be improved
- APPROVED: Plan is comprehensive and ready for execution
-->

## Plan Summary
- **Feature:** [Feature name]
- **Scope:** [Brief description of what the plan covers]
- **Estimated Effort:** [From plan if specified]

## BLOCKING (Must Address Before Execution)

[Issues or "None"]

**[Issue title]:**
- Description: [what's missing or problematic in the plan]
- Impact: [why this blocks execution]
- Action: [what needs to be added/changed in the plan]

## SUGGESTIONS (Would Improve Plan Quality)

[Suggestions or "None"]

**[Suggestion title]:**
- Description: [what could be improved]
- Benefit: [how this would help execution]
- Action: [optional improvement to consider]

## Plan Quality Checklist

**Security & Correctness:**
- [ ] Plan addresses potential security vulnerabilities in design
- [ ] Plan identifies dependency security considerations
- [ ] Plan includes acceptance criteria that match requirements
- [ ] Plan considers concurrency/race conditions if applicable
- [ ] Plan includes error handling strategy
- [ ] Plan addresses API/schema compatibility

**Testing:**
- [ ] Plan includes test strategy (unit, integration, property-based where needed)
- [ ] Plan specifies test-first approach (TDD steps)
- [ ] Plan identifies edge cases to test
- [ ] Plan emphasizes behavior testing over implementation testing
- [ ] Plan includes test isolation requirements
- [ ] Plan specifies clear test names and structure (arrange-act-assert)

**Architecture:**
- [ ] Plan maintains Single Responsibility Principle
- [ ] Plan avoids duplication (identifies shared logic)
- [ ] Plan separates concerns clearly
- [ ] Plan avoids over-engineering (YAGNI - only current requirements)
- [ ] Plan minimizes coupling between modules
- [ ] Plan maintains encapsulation boundaries
- [ ] Plan keeps modules testable in isolation

**Error Handling:**
- [ ] Plan specifies error handling approach (fail-fast vs graceful)
- [ ] Plan includes error message requirements
- [ ] Plan identifies invariants to enforce

**Code Quality:**
- [ ] Plan emphasizes simplicity over cleverness
- [ ] Plan includes naming conventions or examples
- [ ] Plan maintains type safety approach
- [ ] Plan follows project patterns and idioms
- [ ] Plan avoids magic numbers (uses named constants)
- [ ] Plan specifies where rationale comments are needed
- [ ] Plan includes public API documentation requirements

**Process:**
- [ ] Plan includes verification steps for each task
- [ ] Plan identifies performance considerations
- [ ] Plan includes linting/formatting verification
- [ ] Plan scope matches requirements exactly (no scope creep)
- [ ] Plan leverages existing libraries/patterns appropriately
- [ ] Plan includes commit strategy (atomic commits)

## Plan Structure Quality

**Task Granularity:**
- [ ] Tasks are bite-sized (2-5 minutes each)
- [ ] Tasks are independent (can be done in any order where dependencies allow)
- [ ] Each task has clear success criteria

**Completeness:**
- [ ] Exact file paths specified for all tasks
- [ ] Complete code examples (not "add validation")
- [ ] Exact commands with expected output
- [ ] References to relevant skills/practices where applicable

**TDD Approach:**
- [ ] Each task follows RED-GREEN-REFACTOR pattern
- [ ] Write test → Run test (fail) → Implement → Run test (pass) → Commit

## Assessment

**Ready for execution?** [YES / NO / WITH CHANGES]

**Reasoning:**
[Brief explanation of decision and any critical items that need attention]

**Estimated effort:** [If significantly different from plan's estimate, note why]

## Next Steps

[What should happen next - execute as-is, update plan, clarify requirements, etc.]

---

## Template Usage Notes

**For plan reviewers:**

### File Naming for Dual-Verification
When conducting independent reviews as part of dual-verification:
- Save to: `.work/{YYYY-MM-DD}-plan-evaluation-{HHmmss}.md`
- Example: `.work/2025-11-24-plan-evaluation-143052.md`
- Time-based naming prevents conflicts when two agents review in parallel

### Writing Effective BLOCKING Issues

**Good BLOCKING issue:**
```markdown
**Missing authentication checks:**
- Description: Plan specifies API endpoints in Task 3 but doesn't include authentication/authorization strategy
- Impact: Security vulnerability - endpoints would be accessible without auth
- Action: Add task before Task 3 to implement JWT middleware and specify which endpoints require auth
```

**Poor BLOCKING issue:**
```markdown
**Security problem:**
- Description: Not secure enough
- Impact: Bad
- Action: Fix security
```

**What makes it good:**
- ✅ Specific location in plan (Task 3)
- ✅ Clear description of what's missing
- ✅ Concrete impact (security vulnerability)
- ✅ Actionable next step (add JWT middleware task)

### Writing Effective SUGGESTIONS

**Good SUGGESTION:**
```markdown
**Consider adding logging for debugging:**
- Description: Plan doesn't specify logging requirements for API errors
- Benefit: Would make production debugging easier when authentication fails
- Action: Optional - Add logging task or note in Task 3 to log failed auth attempts
```

**Poor SUGGESTION:**
```markdown
**Logging:**
- Description: Should have logging
- Benefit: Helpful
- Action: Add logging
```

**What makes it good:**
- ✅ Explains what would improve (debugging capability)
- ✅ Specific about where (API error logging)
- ✅ Clear it's optional (won't block execution)
- ✅ Actionable (where to add it)

### BLOCKING vs SUGGESTIONS

**Use BLOCKING when:**
- Security vulnerability in design
- Missing error handling strategy
- No test strategy or TDD approach
- Tasks too large (>5 minutes)
- Missing exact file paths or commands
- Scope doesn't match requirements

**Use SUGGESTIONS when:**
- Could add logging for debugging
- Could improve variable naming
- Could add documentation
- Could consider performance optimization
- Could leverage existing pattern

**Rule of thumb:**
- BLOCKING = Plan will fail during execution or produce insecure/incorrect code
- SUGGESTIONS = Plan would succeed but quality could be higher

### Checklist Usage

Check items that the plan addresses appropriately. Leave unchecked if:
- Plan doesn't address it at all
- Plan addresses it but inadequately
- Not applicable to this plan (still note in assessment)

Don't check items you didn't actually verify. Better to have some unchecked items with clear reasoning than to check everything without thorough review.

### Examples

#### Example 1: Well-Written Evaluation

```markdown
# Plan Evaluation - 2025-11-24

## Metadata
- **Reviewer:** plan-review-agent
- **Date:** 2025-11-24 14:30:15
- **Plan Location:** docs/plans/add-user-authentication.md
- **Context:** Independent review #1 for dual-verification

## Status: BLOCKED

## Plan Summary
- **Feature:** User authentication with JWT
- **Scope:** Add login endpoint, JWT middleware, protect existing API endpoints
- **Estimated Effort:** 2-3 hours

## BLOCKING (Must Address Before Execution)

**Missing token expiration strategy:**
- Description: Plan specifies JWT token creation in Task 2 but doesn't include expiration time or refresh token strategy
- Impact: Tokens could be valid indefinitely (security risk) or users forced to re-login frequently (poor UX)
- Action: Add to Task 2: token expiration configuration (recommend 1h access + 7d refresh) and refresh endpoint

**No test-first approach:**
- Description: Plan lists tests at end "Task 5: Add tests for authentication"
- Impact: Implementation may not be testable, tests may test implementation details not behavior
- Action: Restructure each task to follow TDD: write test first, see it fail, implement, see it pass, commit

## SUGGESTIONS (Would Improve Plan Quality)

**Consider rate limiting:**
- Description: Login endpoint in Task 1 doesn't mention rate limiting
- Benefit: Prevents brute force attacks on credentials
- Action: Optional - Add rate limiting middleware (e.g., 5 attempts per IP per minute) in Task 1 or separate task

## Plan Quality Checklist

**Security & Correctness:**
- [ ] Plan addresses potential security vulnerabilities in design (missing: token expiration)
- [x] Plan includes error handling strategy
- [x] Plan considers API compatibility

**Testing:**
- [x] Plan includes test strategy
- [ ] Plan specifies test-first approach (TDD steps) (tests listed at end, not test-first)
- [x] Plan identifies edge cases to test

[Rest of checklist...]

## Assessment

**Ready for execution?** NO

**Reasoning:**
Two blocking issues must be addressed:
1. Token expiration is critical for security
2. Test-first approach ensures implementation is testable

After addressing these, plan structure is good with bite-sized tasks and exact file paths.

## Next Steps

Update plan to:
1. Add token expiration and refresh strategy to Task 2
2. Restructure all tasks to follow TDD (write test → fail → implement → pass → commit)
3. Optionally consider rate limiting suggestion

After updates, plan will be ready for execution.
```

#### Example 2: Approved Plan

```markdown
# Plan Evaluation - 2025-11-24

## Metadata
- **Reviewer:** plan-review-agent
- **Date:** 2025-11-24 15:45:30
- **Plan Location:** docs/plans/refactor-config-loading.md
- **Context:** Standalone review

## Status: APPROVED WITH SUGGESTIONS

## Plan Summary
- **Feature:** Refactor configuration loading to use environment variables
- **Scope:** Extract hardcoded config to .env, add validation, update tests
- **Estimated Effort:** 1 hour

## BLOCKING (Must Address Before Execution)

None

## SUGGESTIONS (Would Improve Plan Quality)

**Consider adding config documentation:**
- Description: Plan doesn't include updating README with new environment variables
- Benefit: Users would know which env vars are required and their purpose
- Action: Optional - Add task to document required env vars in README

## Plan Quality Checklist

**Security & Correctness:**
- [x] Plan addresses security (no secrets in code)
- [x] Plan includes error handling (config validation)
- [x] Plan includes acceptance criteria

**Testing:**
- [x] Plan includes test strategy
- [x] Plan specifies test-first approach
- [x] Plan identifies edge cases (missing vars, invalid values)

**Architecture:**
- [x] Maintains SRP (config loading separate)
- [x] Avoids duplication (centralized config)

[All items checked or appropriately unchecked...]

## Assessment

**Ready for execution?** YES

**Reasoning:**
Plan is comprehensive and addresses all quality criteria:
- Clear TDD approach for each task
- Security handled (no hardcoded secrets)
- Error handling specified (validation)
- Tasks are bite-sized (2-3 minutes each)
- Exact file paths provided

One optional suggestion to improve documentation.

## Next Steps

Execute plan as-is. Optionally add README documentation task.
