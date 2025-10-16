# Recommend

Use the work-planner subagent to analyse & understand the roadmap and recent work in order to recommend the next work to plan in detail.

<context>
  ## Context

  1. Read the following essential documents:
    - @README.md
    - @CLAUDE.md
    - any and all files for the glob `README_*.md`

  2. READ the roadmap:
     - @docs/work/roadmap.md

  3. Read the summaries of the most recent work
    - run `mise run work:recent` to find and list the most recent summary documents
    - read the files
</context>

<instructions>
  ## Instructions for work planning

  Think carefully

  1. Analyse & understand the current state of the project

  2. Make any necessary changes to the roadmap to reflect the current state

  3. Recommend the next work to plan in detail
</instructions>

<output>
  Return your recommendation in a clear, concise format that includes:
  - the recommended work item name (kebab-case, max 5 words)
  - rationale for why this is the next logical step
  - any updates made to the roadmap
  - brief overview of what the work entails
</output>
