# Code Review

Use the code-reviewer subagent to review the code.

<instructions>
  ## Instructions

  1. Review the most recently committed code
    - Follow the requesting-code-review skill: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md`
    - Follow code review principles in `docs/practices/code-review.md`

  2. Ensure all tests pass:
    - Run project test command (e.g., `npm test`, `pytest`, `cargo test`)

  3. Ensure all checks pass:
    - Run linters and formatters
    - Run type checking if applicable

  4. Save the code review into the currently active work directory
    - Use clear markdown format
    - Save the file with date prefix and optional review count: `{YYYY-MM-DD}-review-{N}.md`
    - If multiple reviews exist for the same date, append a review count
</instructions>

<examples>
  ## Example review file names:

  ```
  2025-10-03-review.md
  2025-10-06-review.md
  2025-10-06-review-1.md
  ```
</examples>
