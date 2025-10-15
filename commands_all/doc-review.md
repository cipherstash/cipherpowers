# Documentation Review

Use the doc-writer subagent to thoroughly review the project documentation.

<context>
## Context
Documentation practices:
- @docs/practices/documentation.md
</context>

<instructions>
## Instructions

  <analysis>
      ### Documentation analysis

      1. Perform a `git diff` to understand the most recent code changes
      2. Examine & analyze the diff and review the changes against the project documentation:
        - @README.md
        - @CLAUDE.md
        - any and all files for the glob `README_*.md`
        - @docs/work/roadmap.md
      3. Check for and document any new lessons learned or best practices in CLAUDE.md
      4. Identify new best practices discovered during implementation
      5. Note any implementation challenges and solutions
      6. Cross-reference documentation with the recent implementation to ensure accuracy
  </analysis>

  <update>
    ### Documentation updates

    1. Update the project documentation to reflect the latest changes
      - Add new sections if needed
      - Document specific implementation details for complex components
      - Update any implementation guidance based on experience
      - Document known issues or limitations
      - Update usage examples to include new functionality
      - Include a summary of any new troubleshooting tips
      - Check & confirm that documented commands & tasks reflect the latest changes
    2. Restructure the documentation for maximum clarity
      - Remove or update any content that is no longer relevant or is redundant
      - Group related topics together
      - Split large files if needed
    3. Provide a summary of documentation updates after completion, including:
      - Files updated
      - Major changes to documentation
      - New best practices documented
  </update>

</instructions>
