# Summarise

Use the doc-writer subagent to create a comprehensive, thorough structured summary of the completed work for learning and continuous improvement. Capture insights, analyze decisions, and identify actionable improvements from recent work.

<context>
## Context
Documentation practices:
- @docs/practices/documentation.md
</context>

<instructions>
## Instructions

1. use `mise run work:active` to find the currently active work directory
2. Read any existing summary document
3. Create a comprehensive summary covering (but not limited to):
  - brief description of completed work
  - key decisions
  - open questions or blockers
  - problems or issues encountered
  - options or approaches that were discarded
  - insights and lessons learned
5. Merge with any existing summary document
6. Save the summary into the currently active work directory
  - save the file as `summary.md`
  - there is only ever a single summary document
</instructions>
