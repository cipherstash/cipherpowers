# Fix

Use the rust-engineer subagent to address and fix code review feedback.

<context>
  ## Context

  1. Read and understand the most recent code review in the Current Active Work Directory
    - use `mise run review:active` to find the most recent code review
  2. If no active review can be found, stop working, and inform the user.
   - wait for clarification of next steps
</context>

<instructions>
  ## Instructions for address and fixing code review feedback.

  1. Read and understand the review

  2. Execute the recommend changes, continuing until all are complete
    - always include low priority issues unless otherwise directed
    - follow code standards and practices
    - clarify any ambiguity or ask the user for help if implementation issues arise
    - always implement tests for self-contained logic and calculations

  3. Track and update the review to mark completed work

  4. Use `mise run check` to check code formatting and style
</instructions>
