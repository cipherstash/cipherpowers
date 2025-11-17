---
name: Code Reviews
description: Foster a culture of high-quality, maintainable, and secure code through structured reviews that prioritize feedback based on impact and provide clear, actionable suggestions.
when_to_use: when reviewing code changes before merge to ensure correctness, clarity, security, and maintainability
related_practices: testing.md, development.md
version: 1.0.0
---

# Code Reviews

The goal of code reviews is not to simply find errors, but to foster a culture of high-quality, maintainable, and secure code. Prioritize feedback based on impact, and provide clear, actionable suggestions.

## Code review principles

1.  **Correctness first**
    The code must work as intended and fulfill the requirements.

2.  **Clarity is paramount**
    The code must be easy for a future developer to understand. Readability outweighs cleverness. Unambiguous naming and clear control flow are non-negotiable.

3.  **Question intent, then critique**
    Before flagging a potential issue, first try to understand the author's intent. Frame feedback constructively (e.g., "This function appears to handle both data fetching and transformation. Was this intentional? Separating these concerns might improve testability.").

4.  **Provide actionable suggestions**
    Never just point out a problem. Always propose a concrete solution, a code example, or a direction for improvement.

5.  **Automate the trivial**
    For purely stylistic or linting issues that can be auto-fixed, apply them directly and note them in the report.

6.  **Highlight good code**
    Call out code that exemplifies quality. Specific examples reinforce good practices and build team culture. Look for: clear abstractions, effective tests, thoughtful design, clear naming.

## Review checklist & severity

You will evaluate code and categorize feedback into the following severity levels.

### BLOCKING (Must Fix Before Merge)

**Security & Correctness:**
-   **Security Vulnerabilities**:
    -   Any potential for SQL injection, XSS, CSRF, or other common vulnerabilities.
    -   Improper handling of secrets, hardcoded credentials, or exposed API keys.
    -   Insecure dependencies or use of deprecated cryptographic functions.
-   **Critical Logic Bugs**:
    -   Code that demonstrably fails to meet the acceptance criteria of the ticket.
    -   Race conditions, deadlocks, or unhandled promise rejections.
-   **Missing or Inadequate Tests**:
    -   New logic, especially complex business logic, that is not accompanied by tests.
    -   Tests that only cover the "happy path" without addressing edge cases or error conditions.
    -   Brittle tests that rely on implementation details rather than public-facing behavior.
    -   Tests that verify implementation details instead of behavior (break on refactor even when behavior unchanged).
    -   Missing property-based tests for mathematical/algorithmic code with invariants.
-   **Breaking API or Data Schema Changes**:
    -   Any modification to a public API contract or database schema that is not part of a documented, backward-compatible migration plan.

**Architecture & Performance:**
-   **Architectural Violations**:
    -   **Single Responsibility Principle (SRP)**:
        -   Functions that have multiple, distinct responsibilities or operate at different levels of abstraction (e.g., mixing business logic with low-level data marshalling)
        -   Files that contain multiple unrelated responsibilities (e.g., mixing HTTP handlers, database queries, and business logic in one file)
        -   Modules that lack clear cohesion (unrelated functionality grouped together)
    -   **Duplication (Non-Trivial DRY)**: Duplicated logic that, if changed in one place, would almost certainly need to be changed in others. *This does not apply to simple, repeated patterns where an abstraction would be more complex than the duplication.*
    -   **Poor Modularity & Coupling**:
        -   Excessive dependencies between modules (tight coupling)
        -   Lack of encapsulation (internal implementation details exposed across module boundaries)
        -   Modules that cannot be understood or tested in isolation
    -   **Leaky Abstractions**: Components that expose their internal implementation details, making the system harder to refactor.
    -   **Over-Engineering**:
        -   Unnecessary abstraction layers without clear current need
        -   Complex solutions where simple implementations suffice (violates "Simple, not elegant")
        -   YAGNI violations - implementing features beyond current requirements
-   **Serious Performance Issues**:
    -   Obvious N+1 query patterns in database interactions.
    -   Inefficient algorithms or data structures used on hot paths.
-   **Poor Error Handling**:
    -   Swallowing exceptions or failing silently.
    -   Error messages that lack sufficient context for debugging.

**Verification & Process:**
-   **Missing Evidence of Testing**:
    -   No indication that tests/checks were run before submission
    -   Code changes submitted without running the project's test suite and quality checks
    -   Logic changes without corresponding test updates
-   **Skipped Quality Gates**:
    -   Changes that bypass required verification steps
    -   Commits made without tests passing
    -   Code that hasn't been verified shouldn't merge

### NON-BLOCKING (Can Be Deferred or Follow-up)

**Clarity & Documentation:**
-   Ambiguous or misleading variable, function, or class names.
-   Overly complex conditional logic that could be simplified or refactored into smaller functions.
-   "Magic numbers" or hardcoded strings that should be named constants.
-   Lack of comments for complex, non-obvious algorithms or business logic.
-   Missing doc comments for public-facing functions.
-   Not following established language idioms or project patterns.
-   Inconsistent approaches when similar functionality exists elsewhere in codebase.
-   Comments that explain "what" instead of "why" (what is already clear from code).
-   Missing rationale for non-obvious design decisions.

**Polish:**
-   Style preferences (minor naming improvements, formatting not caught by linters).
-   Minor optimizations (performance improvements with negligible impact).
-   Future considerations (suggestions for future refactoring).
-   Quality check issues that don't affect functionality (linting, formatting that can be auto-fixed).

### Highlights (Examples of Quality Code)

When writing reviews, call out specific examples that demonstrate:

**Simplicity & Design:**
- Straightforward solutions over clever implementations ("Simple, not elegant")
- Minimal abstraction - only what's currently needed (YAGNI applied well)
- Clear, direct code flow without unnecessary complexity
- Well-applied design patterns where appropriate (not over-engineered)

**Testing Excellence:**
- Comprehensive behavior-focused tests (not implementation-focused)
- Property-based tests for algorithms with invariants
- Excellent edge case coverage
- Clear test names that document expected behavior
- Good use of test fixtures and helpers for readability

**Code Quality:**
- Excellent naming that makes code self-documenting
- Following project patterns and language idioms consistently
- Effective use of type systems to prevent bugs
- Clear abstractions with well-defined boundaries

**Documentation:**
- Comments that explain "why" (rationale for non-obvious decisions)
- Public API documentation with usage examples
- Clear commit messages that explain context
- Good inline documentation for complex algorithms

**Process Excellence:**
- Evidence of verification (tests/checks run before submission)
- Atomic commits with clear scope
- Well-structured PR descriptions explaining changes
- Proactive identification of edge cases

## Project configuration

### Code review file

**Template**
`plugin/templates/code-review-template.md`

**Location:**
- Current active work directory (consult project's CLAUDE.md or task runner to find active work directory)
- Pattern: `.work/{feature-name}/` or similar

**Naming:**
- Format: `{YYYY-MM-DD}-review-{N}.md`
- If multiple reviews on same date, increment N
- Examples:
  - `2025-10-15-review.md` (first review of the day)
  - `2025-10-15-review-1.md` (second review of the day)

### Commands

Projects document their specific commands in CLAUDE.md. Consult the project's CLAUDE.md for:

**Run tests:**
- Run the project's test suite (see CLAUDE.md for the specific command)
- Requirement: ALL tests MUST pass.

**Run checks:**
- Run the project's quality checks (see CLAUDE.md for the specific command)
- Requirement: ALL checks MUST pass (linting, formatting, type checking)

