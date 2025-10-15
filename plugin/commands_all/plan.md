# Plan

Use the work-planner subagent to analyse, understand, specify & plan work for implementation.

<context>
  ## Context

  1. Read files in the Current Active Work Directory if present:
    - `analysis.md`

  2. If no Current Active Work Directory is present:
    - READ the roadmap:
     - `docs/work/roadmap.md`
    - READ the most recent summaries
      - use `mise run work:recent`
</context>

<instructions>
  ## Instructions for work planning

  1. Create a comprehensive, detailed plan for the specified work
   - include task & subtask checklists

  2. Clarify scope boundaries & technical considerations by asking numbered questions as needed to ensure clear requirements before proceeding.
    - focus on reducing scope & working in small, incremental steps.

  3. Create the plan as `plan.md` in the Current Active Work Directory directory.

  4. Ask the user to review the plan
    - wait for explicit approval
    - iterate on revisions as requested
</instructions>

