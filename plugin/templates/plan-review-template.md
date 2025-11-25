---
name: Plan Review Template
description: Structured format for evaluating implementation plan quality before execution
when_to_use: when reviewing implementation plans to ensure they address all quality criteria
related_practices: code-review.md, development.md, testing.md
version: 1.0.0
---

# Plan Review - {Date}

## Metadata
- **Reviewer:** [Agent name/type, e.g., plan-review-agent]
- **Date:** {YYYY-MM-DD HH:mm:ss}
- **Plan Location:** [Path to plan file]
- **Context:** [Standalone review / Independent review #[1/2] for dual-verification]

## Status: [BLOCKED | APPROVED WITH SUGGESTIONS | APPROVED]

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
