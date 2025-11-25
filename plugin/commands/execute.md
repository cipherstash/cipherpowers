# Execute

Execute implementation plans with automatic agent selection, batch-level code review, and retrospective completion.

## Algorithmic Workflow

**Decision tree (follow exactly, no interpretation):**

1. Is this a plan execution request?
   - YES → Continue to step 2
   - NO → This command was invoked incorrectly

2. Does a plan exist to execute?
   - YES → Continue to step 3
   - NO → Run `/plan` first to create implementation plan, then return here

3. **MANDATORY: Skill Activation**

**Load skill context:**
@${CLAUDE_PLUGIN_ROOT}skills/executing-plans/SKILL.md

**Step 1 - EVALUATE:** State YES/NO for skill activation:
- Skill: "cipherpowers:executing-plans"
- Applies to this task: YES/NO (reason)

**Step 2 - ACTIVATE:** If YES, use Skill tool NOW:
```
Skill(skill: "cipherpowers:executing-plans")
```

⚠️ Do NOT proceed without completing skill evaluation and activation.

4. **FOLLOW THE SKILL EXACTLY:**
   - The skill defines the complete execution methodology
   - Automatic agent selection (hybrid keyword/LLM analysis)
   - Batch execution (3 tasks per batch)
   - Code review after each batch
   - Retrospective capture when complete

5. **STOP when execution is complete.**

## Why Algorithmic Workflow?

- **100% reliability**: No interpretation, no skipping plan creation
- **Skill integration**: Automatic discovery via Skill tool
- **Agent orchestration**: Skill handles agent selection and dispatch
- **Quality gates**: Code review checkpoints prevent cascading issues

## What the Skill Does

The executing-plans skill provides:
- Load and parse implementation plan
- Automatic agent selection (rust-agent, ultrathink-debugger, etc.)
- Batch execution with review checkpoints
- Code review after each batch (automatic dispatch to code-review-agent)
- Retrospective capture when work completes
- Integration with selecting-agents skill

**References:**
- Skill: `${CLAUDE_PLUGIN_ROOT}skills/executing-plans/SKILL.md`
- Agent Selection: `${CLAUDE_PLUGIN_ROOT}skills/selecting-agents/SKILL.md`
- Code Review: Automatic dispatch to cipherpowers:code-review-agent
- Integration: Seamless workflow → `/brainstorm` → `/plan` → `/execute`

