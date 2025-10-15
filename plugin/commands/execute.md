# Execute Plan

Use the work-orchestrator subagent to implement the active work plan.

<context>
  ## Context

  1. Read files in the current work directory:
    - `plan.md`

  2. If no `plan.md` can be found, stop working and inform the user.
   - Wait for clarification of next steps
</context>

<instructions>
  ## Instructions for code implementation

  1. Follow the executing-plans skill: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md`

  2. Execute the listed tasks and subtasks, continuing until all tasks are complete
    - Follow code standards and practices in `docs/practices/`
    - Clarify any ambiguity or ask the user for help if implementation issues arise
    - Implement tests for self-contained logic and calculations as appropriate
    - Follow TDD practices: `docs/practices/testing.md`

  3. Track and update the plan to mark completed tasks and subtasks
    - Use TodoWrite for tracking progress

  4. Run checks to verify code quality
    - Run linters and formatters
    - Run tests
    - Fix any issues before proceeding

  <important>
  5. Always ask the user to review the implementation
    - Code changes may require manual testing by the user
    - Do not commit work without explicit direction by the user
    - Wait for explicit approval before committing
    - Iterate on revisions as requested
  </important>
</instructions>
