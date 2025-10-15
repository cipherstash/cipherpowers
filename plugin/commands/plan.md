# Plan

Use the work-planner subagent to analyse, understand, specify & plan work for implementation.

<context>
  ## Context

  1. Read files in the current work directory if present:
    - `analysis.md`
    - `requirements.md`
    - Or any other relevant context files

  2. If no current work directory is present:
    - Read project roadmap or backlog documentation
    - Read recent work summaries or sprint notes
</context>

<instructions>
  ## Instructions for work planning

  1. Create a comprehensive, detailed plan for the specified work
   - include task & subtask checklists
   - follow the writing-plans skill: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/writing-plans/SKILL.md`

  2. Clarify scope boundaries & technical considerations by asking numbered questions as needed to ensure clear requirements before proceeding.
    - focus on reducing scope & working in small, incremental steps.

  3. Create the plan as `plan.md` in the current work directory.

  4. Ask the user to review the plan
    - wait for explicit approval
    - iterate on revisions as requested
</instructions>
