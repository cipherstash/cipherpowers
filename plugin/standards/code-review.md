---
name: Code Reviews
description: Foster a culture of high-quality, maintainable, and secure code through structured reviews that prioritize feedback based on impact and provide clear, actionable suggestions.
when_to_use: when reviewing code changes before merge to ensure correctness, clarity, security, and maintainability
related_practices: testing.md, development.md
version: 1.0.0
---

# Code Reviews

The goal of code reviews is not to simply find errors, but to foster a culture of high-quality, maintainable, and secure code. Prioritize feedback based on impact, and provide clear, actionable suggestions.

## Code Review Principles

1.  **Correctness First**: The code must work as intended and fulfill the requirements.
2.  **Clarity is Paramount**: The code must be easy for a future developer to understand. Readability outweighs cleverness. Unambiguous naming and clear control flow are non-negotiable.
3.  **Question Intent, Then Critique**: Before flagging a potential issue, first try to understand the author's intent. Frame feedback constructively (e.g., "This function appears to handle both data fetching and transformation. Was this intentional? Separating these concerns might improve testability.").
4.  **Provide Actionable Suggestions**: Never just point out a problem. Always propose a concrete solution, a code example, or a direction for improvement.
5.  **Automate the Trivial**: For purely stylistic or linting issues that can be auto-fixed, apply them directly and note them in the report.

## Review Checklist & Severity

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
-   **Breaking API or Data Schema Changes**:
    -   Any modification to a public API contract or database schema that is not part of a documented, backward-compatible migration plan.

**Architecture & Performance:**
-   **Architectural Violations**:
    -   **Single Responsibility Principle (SRP)**: Functions that have multiple, distinct responsibilities or operate at different levels of abstraction (e.g., mixing business logic with low-level data marshalling).
    -   **Duplication (Non-Trivial DRY)**: Duplicated logic that, if changed in one place, would almost certainly need to be changed in others. *This does not apply to simple, repeated patterns where an abstraction would be more complex than the duplication.*
    -   **Leaky Abstractions**: Components that expose their internal implementation details, making the system harder to refactor.
-   **Serious Performance Issues**:
    -   Obvious N+1 query patterns in database interactions.
    -   Inefficient algorithms or data structures used on hot paths.
-   **Poor Error Handling**:
    -   Swallowing exceptions or failing silently.
    -   Error messages that lack sufficient context for debugging.

### NON-BLOCKING (Can Be Deferred or Follow-up)

**Clarity & Documentation:**
-   Ambiguous or misleading variable, function, or class names.
-   Overly complex conditional logic that could be simplified or refactored into smaller functions.
-   "Magic numbers" or hardcoded strings that should be named constants.
-   Lack of comments for complex, non-obvious algorithms or business logic.
-   Missing doc comments for public-facing functions.

**Polish:**
-   Style preferences (minor naming improvements, formatting not caught by linters).
-   Minor optimizations (performance improvements with negligible impact).
-   Future considerations (suggestions for future refactoring).
-   Mise run check issues that don't affect functionality (linting, formatting that can be auto-fixed).

## Project Configuration

### Review File Conventions

**Location:**
- Current active work directory (use project task to find: `mise run review:active`)
- Pattern: `.work/{feature-name}/` or similar

**Naming:**
- Format: `{YYYY-MM-DD}-review-{N}.md`
- If multiple reviews on same date, increment N
- Examples:
  - `2025-10-15-review.md` (first review of the day)
  - `2025-10-15-review-1.md` (second review of the day)

### Commands

**Run tests:**
- Command: `mise run test`
- Requirement: ALL tests MUST pass

**Run checks:**
- Command: `mise run check`
- Requirement: ALL checks MUST pass (linting, formatting, type checking)

**Find active work directory:**
- Command: `mise run review:active`
- Returns: Path to current work directory for saving review

### Review Template

Save review using this structure:

```markdown
# Code Review - {Date}

## Summary
[1-2 sentences]

## BLOCKING (Must Fix Before Merge)
[Issues or "None found"]

## NON-BLOCKING (Can Be Deferred)
[Issues or "None found"]

## Positive Observations
[Specific examples]

## Test Results
- Tests: [PASS/FAIL]
- Checks: [PASS/FAIL]

## Next Steps
[Actions required]
```
