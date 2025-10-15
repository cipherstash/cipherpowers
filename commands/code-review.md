# Code Review

Use the code-review skill for the complete workflow, then invoke the code-reviewer agent.

<instructions>
## Instructions

1. **Follow the code-review skill:**
   - Read: `@skills/code-review/SKILL.md`
   - This skill contains the complete workflow (test verification, practice adherence, structured feedback)

2. **Use the code-reviewer agent:**
   - The agent implements the workflow from the skill
   - Ensures non-negotiable steps with persuasion principles
   - Agent will reference the skill automatically

**Why this structure?**
- Skill = Discoverable workflow (agents can find it)
- Agent = Enforced execution (persuasion principles prevent shortcuts)
- Command = Thin dispatcher (provides context to agent)
</instructions>
