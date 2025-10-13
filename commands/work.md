# Work

Complete workflow for implementing planned work with code review and documentation.

<instructions>
  ## Workflow Steps

  1. Implement the plan
    - Follow the executing-plans skill: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md`
    - Execute tasks in batches with review checkpoints
    - Follow code standards and practices documented in `docs/practices/`

  2. Commit work
    - Use the `/commit` command to create conventional commits
    - Follow git guidelines in `docs/practices/git-guidelines.md`

  3. Request code review
    - Use the `/code-review` command to dispatch code-reviewer subagent
    - Follow code review practices in `docs/practices/code-review.md`

  4. Address feedback
    - Implement code review recommendations
    - Use the receiving-code-review skill: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/receiving-code-review/SKILL.md`

  5. Review again if needed
    - Request another code review after addressing feedback
    - Repeat until all feedback is resolved
    - Maximum 1-2 iterations to avoid loops
    - Clarify any ambiguity with the user

  6. Document the work
    - Create or update documentation as needed
    - Follow documentation practices in `docs/practices/documentation.md`

  7. Commit documentation changes
    - Use the `/commit` command for documentation updates

  <important>
    - Use TodoWrite to track high-level steps in this workflow
    - Reference skills for detailed process guidance
    - Ask for help when blocked or unclear
  </important>
</instructions>
