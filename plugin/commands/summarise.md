# Summarise

Create a comprehensive retrospective summary of completed work, capturing decisions, lessons learned, and insights for continuous improvement.

<instructions>
## Instructions

1. **Use the retrospective-writer agent:**
   - Agent: `retrospective-writer`
   - This agent follows the capturing-learning skill:
     - Step 1: Review the work (identify location, review changes)
     - Step 2: Capture learning (decisions, approaches, issues, time)
     - Step 3: Save and link (to work directory or CLAUDE.md)

2. **The agent references project documentation standards:**
   - `@plugin/practices/documentation.md` - Summary format and standards

**Key Principle:**
Exhaustion after completion is when capture matters most. The harder the work, the more valuable the lessons.

**Why this structure?**
- Agent = Enforces workflow with non-negotiable steps
- Skill = Universal workflow (learning capture process)
- Practices = Project-specific standards (summary format)
- Command = Thin dispatcher (integrates with work tracking)
</instructions>
