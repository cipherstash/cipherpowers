# Fix Review

Use the code-reviewer agent to verify implementation of code review recommendations.

<context>
  ## Context

  1. Find most recent code review in active work directory
     - Use `mise run review:active` to locate directory
  2. If no review found, STOP and inform user
</context>

<instructions>
  ## Instructions

  1. Read most recent review file from active work directory
  2. Dispatch to code-reviewer agent with context:
     - Previous review recommendations
     - Commits since that review
  3. Agent will verify:
     - Recommendations were addressed
     - Alternative approaches are acceptable if they solve underlying issue
     - New issues introduced by changes
  4. Agent will save new review if additional issues found

  See @docs/practices/code-review.md for standards and configuration.
</instructions>
