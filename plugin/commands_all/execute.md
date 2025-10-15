# Execute Plan

Use the work-orchestrator subagent to implement the active work plan.

<context>
  ## Context

  1. Read files in the Current Active Work Directory:
    - `plan.md`
  2. Find the Current Active Work Directory if required:
    - use `mise run work:active`
  3. If no `plan.md` can be found, stop working, and inform the user.
   - wait for clarification of next steps
</context>

<instructions>
  ## Instructions for code implementation

  1. Follow the `plan.md` found in the Current Active Work Directory

  2. Execute the listed tasks and subtasks, continuing until all tasks are complete
    - follow code standards and practices
    - clarify any ambiguity or ask the user for help if implementation issues arise
    - implement tests for self-contained logic and calculations as appropriate

  3. Track and update the plan to mark completed tasks and subtasks

  4. Use `mise run check` to check code formatting and style

  <important>
  5. Always ask the user to review the implementation
    - code changes require manual testing by the user
    - do not commit work without explicit direction by the user
    - wait for explicit approval before committing
    - iterate on revisions as requested
  </important>

</instructions>
