# Plan

Create detailed implementation plans with bite-sized tasks ready for execution.

## Algorithmic Workflow

**Decision tree (follow exactly, no interpretation):**

1. Is this a planning request?
   - YES → Continue to step 2
   - NO → This command was invoked incorrectly

2. **USE WRITING-PLANS SKILL:**

```
Use Skill tool with:
  skill: "cipherpowers:writing-plans"
```

4. **FOLLOW THE SKILL EXACTLY:**
   - The skill defines the complete planning methodology
   - Create detailed plan file in `.work` directory
   - Break work into bite-sized, independent tasks
   - Include verification steps and success criteria

5. **STOP when plan is complete and saved.**

## Why Algorithmic Workflow?

- **100% reliability**: No interpretation, no skipping brainstorming
- **Skill integration**: Automatic discovery via Skill tool
- **Consistent structure**: Every plan follows proven template
- **Ready for execution**: Plans integrate with `/execute` command

## What the Skill Does

The writing-plans skill provides:
- When to use planning vs direct implementation
- How to structure tasks for agent execution
- Task granularity guidelines (bite-sized, independent)
- Verification and success criteria
- Integration with code review checkpoints

**References:**
- Skill: `${CLAUDE_PLUGIN_ROOT}skills/writing-plans/SKILL.md`
- Template: Used by skill for consistent structure
- Integration: Seamless workflow → `/brainstorm` → `/plan` → `/execute`

