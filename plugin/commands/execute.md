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

### 2. Analyze plan and select agents

**Before starting execution, YOU MUST:**

1. Read the entire plan file
2. Analyze each task to determine required agent
3. Announce the execution strategy to the user

**Agent selection uses hybrid approach:**

#### Keyword Analysis (Fast Path)

Scan task descriptions for these patterns:

| Keywords | Agent | Rationale |
|----------|-------|-----------|
| "Rust", ".rs file", "cargo" | rust-engineer | Rust development with TDD |
| "debug", "investigate", "production issue", "CI failure", "race condition" | ultrathink-debugger | Complex debugging |
| "review", "code review" | code-reviewer | Code review workflow |
| "retrospective", "summary", "learning" | retrospective-writer | Learning capture |
| "docs", "documentation", "README" | technical-writer | Documentation sync |

**File path analysis:**
- Tasks mentioning `.rs` files → rust-engineer
- Tasks mentioning `docs/` or `.md` files → technical-writer (if code-related), else main Claude

#### LLM Analysis (Ambiguous Cases)

When keywords don't clearly indicate an agent:

1. Read: `@${CLAUDE_PLUGIN_ROOT}/skills/selecting-agents/SKILL.md`
2. Apply the decision guide to the task description
3. Consider:
   - Task complexity (simple vs complex debugging)
   - Task type (development vs documentation vs debugging)
   - Agent characteristics from skill

#### User Confirmation (Unclear Cases)

If hybrid analysis cannot confidently select an agent:

```
Task: [task description]

I cannot confidently determine which agent to use. Options:
- rust-engineer: Standard Rust development with TDD
- ultrathink-debugger: Complex debugging requiring deep investigation
- main Claude: General implementation without specialized workflow

Which agent should handle this task?
```

**Stop execution and wait for user response.**

### 3. Announce execution strategy

Before starting, announce:

```
Plan loaded: [plan file path]
Tasks: [N tasks total]

Agent assignments:
- Tasks 1-3: rust-engineer (Rust implementation)
- Tasks 4-5: main Claude (configuration updates)
- Task 6: rust-engineer (Rust testing)

Execution plan:
- Batch 1 (Tasks 1-3) → code review → fixes
- Batch 2 (Tasks 4-6) → code review → fixes
- Retrospective prompt

Ready to proceed?
```

Wait for user confirmation before starting execution.

### 4. Execute batches with agent dispatch

**For each batch (3 tasks):**

1. **Dispatch to agents:**
   - For each task in batch, use the Task tool to invoke the selected agent
   - Pass task description and relevant context
   - Wait for agent completion

2. **Verify task completion:**
   - Check tests pass (if applicable)
   - Check build succeeds (if applicable)
   - Confirm task marked complete in TodoWrite

3. **Handle failures:**
   - If task fails, agent attempts auto-fix (up to 3 attempts)
   - If still failing after 3 attempts: STOP execution
   - Report failure to user with context
   - Wait for user decision (fix manually, skip task, abort plan)

### 5. Code review checkpoint (after each batch)

**After batch completion, MANDATORY code review:**

1. **Announce review:**
   ```
   Batch [N] complete. Tasks [X-Y] implemented.

   Invoking code-reviewer agent for batch checkpoint.
   ```

2. **Invoke code-reviewer:**
   - Use Task tool with subagent_type=code-reviewer
   - Pass context: "Review changes from tasks [X-Y]"
   - Code-reviewer follows: `${CLAUDE_PLUGIN_ROOT}/skills/conducting-code-review/SKILL.md`
   - Code-reviewer references: `${CLAUDE_PLUGIN_ROOT}/practices/code-review.md`

3. **Review results:**
   - Read review file saved by code-reviewer
   - Identify feedback at all levels (Critical, High, Medium, Low)

4. **Address feedback:**
   - For each issue, dispatch to appropriate agent to fix
   - Re-run tests and checks after fixes
   - Mark as resolved when fixed

5. **Verify all feedback addressed:**
   - ALL feedback must be addressed (Critical through Low)
   - Do NOT proceed to next batch with unresolved feedback
   - If feedback cannot be addressed: document why + user approval required

6. **Report to user:**
   ```
   Batch [N] review complete.
   - Critical issues: [N] (all resolved)
   - High priority: [N] (all resolved)
   - Medium priority: [N] (all resolved)
   - Low priority: [N] (all resolved)

   Ready for next batch.
   ```

### 6. Plan completion

**After all batches complete and all reviews passed:**

1. **Verify completion:**
   - All tasks marked complete in TodoWrite
   - All tests passing
   - All checks passing
   - All code review feedback addressed

2. **Final verification:**
   - Run `mise run test` (if project uses mise)
   - Run `mise run check` (if project uses mise)
   - Confirm no failures

### 7. Retrospective prompt

**When plan is fully complete, prompt user:**

```
All tasks complete! 🎉

Plan: [plan file path]
Tasks completed: [N]
Batches: [N]
Code reviews: [N]

Ready to write a retrospective capturing what you learned?

This will invoke the retrospective-writer agent to follow:
- Skill: capturing-learning
- Practice: documentation standards

[Yes/No]
```

**If user confirms:**
- Use Task tool with subagent_type=retrospective-writer
- Pass context: "Plan: [path], Tasks: [summary]"
- Retrospective-writer saves summary to appropriate location

**If user declines:**
```
Execution complete. Retrospective skipped.

Reminder: You can write a retrospective later with /summarise
```

## Error Handling

### Agent Failures

**If agent reports failure:**
1. Read agent output to understand failure
2. Attempt auto-fix (up to 3 attempts):
   - Re-invoke same agent with error context
   - Agent attempts fix following its workflow
3. If 3 attempts exhausted:
   - STOP execution
   - Report to user with full context
   - Options: Manual fix, skip task, abort plan

### Test/Check Failures

**If tests or checks fail after implementation:**
1. Report failure output to user
2. Invoke appropriate agent to fix:
   - Test failures → same agent that wrote code
   - Check failures (lint/format) → same agent that wrote code
3. Re-run after fix
4. If failures persist after 3 attempts → STOP and ask user

### Missing Plan File

**If plan file doesn't exist:**
```
Error: Plan file not found: [path]

Did you mean to:
1. Create a plan first? Use /brainstorm → /write-plan
2. Specify a different path?
```

### Empty or Malformed Plan

**If plan file exists but has no tasks:**
```
Error: Plan file contains no tasks: [path]

Please ensure the plan has properly formatted tasks with:
- Task headers (### Task N: Name)
- Step-by-step instructions
- Expected outcomes
```

## Edge Cases

### Single Task Plans

Plans with < 3 tasks:
- Execute as single batch
- Still perform code review after batch
- Still prompt for retrospective

### Very Large Plans

Plans with > 15 tasks:
- Warn user about execution time
- Suggest breaking into multiple smaller plans
- Allow user to proceed or abort

### Agent Selection Conflicts

If task seems to require multiple agents:
- Ask user which agent should be primary
- Note in TodoWrite that secondary agent may be needed
- User can intervene during execution if needed

### Plan Updates During Execution

If user updates plan file during execution:
- Detect changes on next batch load
- Ask user: Continue with old plan or reload new version?
- If reload: Re-analyze agent assignments for remaining tasks

</instructions>

## Why This Structure?

**Command (not agent):**
- Main Claude orchestrates → maintains full conversation context
- Specialized agents → focused workflows with enforcement
- Clear handoffs → each agent completes its scope and returns

**Hybrid agent selection:**
- Keywords → fast and deterministic for obvious cases
- LLM analysis → intelligent handling of ambiguous tasks
- User confirmation → keeps human in the loop for unclear cases

**Per-batch code review:**
- Tests provide task-level verification (correctness)
- Code review provides batch-level validation (architecture, maintainability)
- Prevents compounding errors across batches
- Matches execute-plan checkpoint pattern

**Automatic agent dispatch:**
- Reduces cognitive load (user doesn't pick agents per-task)
- Enforces consistency (right agent for task type)
- References selecting-agents skill (same logic, automated)

**Retrospective prompt:**
- Captures learning while context is fresh
- Optional (user controls when/if to document)
- Integrates with existing retrospective-writer agent

## Related

**Skills:**
- `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` - Core execution workflow
- `${CLAUDE_PLUGIN_ROOT}/skills/selecting-agents/SKILL.md` - Agent selection guide
- `${CLAUDE_PLUGIN_ROOT}/skills/documentation/capturing-learning/SKILL.md` - Retrospective workflow

**Practices:**
- `${CLAUDE_PLUGIN_ROOT}/practices/code-review.md` - Review standards and severity levels
- `${CLAUDE_PLUGIN_ROOT}/practices/testing.md` - Test requirements
- `${CLAUDE_PLUGIN_ROOT}/practices/development.md` - Development standards

**Commands:**
- `/code-review` - Manual code review trigger
- `/summarise` - Manual retrospective trigger
- `/brainstorm` → `/write-plan` - Plan creation workflow
