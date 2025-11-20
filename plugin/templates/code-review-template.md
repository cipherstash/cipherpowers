---
name: Code Review Template
description: Structured format for saving code review feedback with severity-based categorization
when_to_use: when conducting code reviews and saving structured feedback to work directories
related_practices: code-review.md
version: 1.0.0
---

# Code Review - {Date}

## Status: [BLOCKED | APPROVED WITH NON-BLOCKING SUGGESTIONS | APPROVED]

<!--
Status guidance:
- BLOCKED: Has BLOCKING issues that must be fixed before merge
- APPROVED WITH NON-BLOCKING SUGGESTIONS: Ready to merge, but consider addressing suggestions
- APPROVED: Clean, ready to merge with no issues
-->


## Test Results
- Status: [PASS/FAIL]
- Details: [test failures or "all passed"]


## Check Results
- Status: [PASS/FAIL]
- Details: [warnings or "clean"]


## Next Steps
[Actions required]


## BLOCKING (Must Fix Before Merge)

[Issues or "None"]

**[Issue title]:**
- Description: [clear description]
- Location: [file:line]
- Action: [specific, actionable suggestion]

**[Issue title]:**
- Description: [clear description]
- Location: [file:line]
- Action: [specific, actionable suggestion]


## NON-BLOCKING (May Be Deferred)

[Issues or "None"]

**[Issue title]:**
- Description: [clear description]
- Location: [file:line]
- Action: [specific & actionable suggestion]

**[Issue title]:**
- Description: [clear description]
- Location: [file:line]
- Action: [specific & actionable suggestion]


## Checklist

**Security & Correctness:**
- [ ] No security vulnerabilities (SQL injection, XSS, CSRF, exposed secrets)
- [ ] No insecure dependencies or deprecated cryptographic functions
- [ ] No critical logic bugs (meets acceptance criteria)
- [ ] No race conditions, deadlocks, or data races
- [ ] No unhandled errors, rejected promises, or panics
- [ ] No breaking API or schema changes without migration plan

**Testing:**
- [ ] All tests passing (unit, integration, property-based where applicable)
- [ ] New logic has corresponding tests
- [ ] Tests cover edge cases and error conditions
- [ ] Tests verify behavior (not implementation details)
- [ ] Property-based tests for mathematical/algorithmic code with invariants
- [ ] Tests are isolated (independent, don't rely on other tests)
- [ ] Test names are clear and use structured arrange-act-assert patterns

**Architecture:**
- [ ] Single Responsibility Principle (functions/files have one clear purpose)
- [ ] No non-trivial duplication (logic that if changed in one place would need changing elsewhere)
- [ ] Clean separation of concerns (business logic separate from data marshalling)
- [ ] No leaky abstractions (internal details not exposed)
- [ ] No over-engineering (YAGNI - implement only current requirements)
- [ ] No tight coupling (excessive dependencies between modules)
- [ ] Proper encapsulation (internal details not exposed across boundaries)
- [ ] Modules can be understood and tested in isolation

**Error Handling:**
- [ ] No swallowed exceptions or silent failures
- [ ] Error messages provide sufficient context for debugging
- [ ] Fail-fast on invariants where appropriate

**Code Quality:**
- [ ] Simple, not clever (straightforward solutions over complex ones)
- [ ] Clear, descriptive naming (variables, functions, classes)
- [ ] Type safety maintained
- [ ] Follows language idioms and project patterns consistently
- [ ] No magic numbers or hardcoded strings (use named constants)
- [ ] Consistent approaches when similar functionality exists elsewhere
- [ ] Comments explain "why" not "what" (code should be self-documenting)
- [ ] Rationale provided for non-obvious design decisions
- [ ] Doc comments for public APIs

**Process:**
- [ ] Tests and checks run before submission (no skipped quality gates, evidence of verification)
- [ ] No obvious performance issues (N+1 queries, inefficient algorithms on hot paths)
- [ ] ALL linter warnings addressed by fixing root cause (disable/allow/ignore ONLY when unavoidable)
- [ ] Requirements met exactly (no scope creep)
- [ ] No unnecessary reinvention (appropriate use of existing libraries/patterns)

