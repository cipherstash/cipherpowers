# Plan

Create detailed implementation plans with bite-sized tasks ready for execution.

<instructions>
## Instructions

1. **Invoke the superpowers write-plan command:**
   - Use SlashCommand tool: `/superpowers:write-plan`
   - This creates a structured plan with step-by-step tasks

2. **The writing-plans skill provides the methodology:**
   - Read: `@${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/writing-plans/SKILL.md`
   - When to use: After brainstorming is complete and you need implementation tasks
   - Output: Detailed plan file in `docs/plans/` or `plans/` directory

3. **After creating a plan, execute it:**
   - Use `/execute [plan-file-path]` to execute the plan with automatic agent selection
   - Or use `/superpowers:execute-plan` if you need the basic superpowers workflow

4. **Plan structure:**
   - Tasks broken down for engineers with zero codebase context
   - Each task includes step-by-step instructions and expected outcomes
   - Tasks are sized for 3-task batches with review checkpoints

**Why this structure?**
- Skill = Universal plan writing methodology (superpowers)
- Command = Thin wrapper (CipherPowers entry point)
- Integration = Seamless workflow from brainstorm → plan → execute
</instructions>
