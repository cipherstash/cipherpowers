# Execute

Execute implementation plans with automatic agent selection, batch-level code review, and retrospective completion.

<instructions>
## Instructions

This command orchestrates plan execution in main Claude context, dispatching to specialized agents as needed:

1. **Load and analyze plan** - Read plan file, determine agent requirements
2. **Announce execution strategy** - Show which agents will handle which task types
3. **Execute in batches** - 3 tasks per batch with appropriate agents
4. **Review checkpoints** - Invoke code-reviewer after each batch
5. **Retrospective completion** - Prompt for learning capture when all tasks done

### 1. Follow the executing-plans skill

Read: `@${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md`

This skill provides:
- Plan loading and critical review
- Batch execution pattern (default 3 tasks)
- Verification requirements
- Checkpoint reporting

</instructions>
