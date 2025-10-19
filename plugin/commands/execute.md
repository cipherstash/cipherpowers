# Execute

Execute implementation plans with automatic agent selection, batch-level code review, and retrospective completion.

## Decision Algorithm: When to Use This Command

**BEFORE starting any work, run this algorithm:**

```
Step 1: Check: Does a file matching `docs/plans/*.md` OR `plans/*.md` exist?
        → YES: Go to Step 2
        → NO: Go to Step 7

Step 2: Check: Is the task exploration/research only (no commits)?
        → YES: Go to Step 7
        → NO: Go to Step 3

Step 3: Check: Is the task a single atomic command (run test, check status)?
        → YES: Go to Step 7
        → NO: Go to Step 4

Step 4: Execute `/execute [plan-file-path]`
        STOP reading this algorithm
        Do not proceed to Step 5

Step 5: [UNREACHABLE - if you reach here, you violated Step 4]

Step 6: [UNREACHABLE - if you reach here, you violated Step 4]

Step 7: Proceed without /execute (valid cases only)
```

### Recovery Algorithm: Already Started Without /execute?

**If you've already started work, run this recovery algorithm:**

```
Step 1: Check: Have you written ANY code?
        → YES: Go to Step 2
        → NO: Go to Step 5

Step 2: Check: Does that code have tests?
        → YES: Go to Step 5
        → NO: Go to Step 3

Step 3: Delete the untested code
        Execute: rm [files] OR git reset --hard
        Go to Step 4

Step 4: Check: Does a plan file exist for remaining work?
        → YES: Execute `/execute [plan-file]`, STOP
        → NO: Create plan file, then execute `/execute [plan-file]`, STOP

Step 5: Check: Are there remaining tasks requiring commits?
        → YES: Check if plan file exists for them
               → YES: Execute `/execute [plan-file]`, STOP
               → NO: Create plan file, then execute `/execute [plan-file]`, STOP
        → NO: Continue current work

Step 6: [UNREACHABLE - all paths lead to STOP]
```

## Condition Evaluation Rules

**Valid conditions (these are checkable facts):**
- Does file X exist? (boolean: yes/no)
- Does code have tests? (boolean: yes/no)
- Is task exploration? (boolean: yes/no)
- Are commits required? (boolean: yes/no)

**INVALID conditions (NOT in algorithm, do NOT use for decisions):**
- "Is task simple?" → NOT A VALID CONDITION
- "Is task just markdown?" → NOT A VALID CONDITION
- "Is there time pressure?" → NOT A VALID CONDITION
- "Will /execute take too long?" → NOT A VALID CONDITION
- "Should I be pragmatic?" → NOT A VALID CONDITION
- "Is there sunk cost?" → NOT A VALID CONDITION
- "Am I exhausted?" → NOT A VALID CONDITION

**Any reasoning beyond the algorithm conditions = algorithm violation.**

## Why This is an Algorithm, Not a Suggestion

1. Algorithms execute deterministically
2. Boolean conditions cannot be argued with
3. You cannot "choose" whether to follow an algorithm
4. Time pressure does not change boolean evaluation
5. Task simplicity is not tested by any condition
6. Your preferences are not algorithm inputs
7. "Being pragmatic" is not a valid branch condition

## Self-Test: Do You Understand the Algorithm?

**Answer these before proceeding with ANY work:**

```
Q1: Does file `docs/plans/my-task.md` exist?
    If YES: What does Step 4 say to do?
    If NO: What does Step 7 say?

Q2: I wrote code 2 hours ago without tests. Recovery algorithm Step 3 says?

Q3: "These are simple markdown tasks" - is this a valid algorithm condition?
    YES / NO
    Why?

Q4: What happens if I reach Step 5 in the main algorithm?
```

**Correct answers:**
- Q1: YES → Execute `/execute [plan-file]`, STOP. NO → Proceed without
- Q2: "Delete the untested code"
- Q3: NO - Task simplicity is not tested by any algorithm condition
- Q4: Impossible - Step 4 has STOP, Step 5 is unreachable

**If you got any answer wrong: RE-READ the algorithm before proceeding.**

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

1. Read: `@${CLAUDE_PLUGIN_ROOT}plugin/skills/selecting-agents/SKILL.md`
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

**Step 1: Dispatch code-reviewer agent**

Reviews all changes in batch using code review practice.

Dispatch: code-reviewer agent
Input: Changed files in batch
Output: Review file at `{work-dir}/{YYYY-MM-DD}-review.md`

**Step 2: Dispatch gatekeeper agent**

Validates review feedback against plan, gets user decisions on scope.

Dispatch: gatekeeper agent
Input:
- Plan file: `{work-dir}/plan.md` or `docs/plans/{YYYY-MM-DD}-{feature}.md`
- Review file: `{work-dir}/{YYYY-MM-DD}-review.md` (from code-reviewer)
- Batch number: {N}

Output: Annotated review file with [FIX]/[WONTFIX]/[DEFERRED] tags

**Step 3: Read annotated review**

Parse the gatekeeper's annotated review file.

Extract count of [FIX] items (items that must be addressed now).

**Step 4: Handle validation results**

**If gatekeeper reports "plan revision needed":**
- Pause execution
- Show user the deferred items
- Ask: "Plan revision needed based on deferred feedback. Update plan and resume, or continue?"
- Wait for user decision (5 minute timeout)
- **Timeout behavior:** If no response within 5 minutes, mark workflow as FAILED and halt execution

**If 0 items marked [FIX]:**
- Announce: "Batch {N} review clean - no blocking issues to fix"
- Proceed to batch {N+1}

**If >0 items marked [FIX]:**
- Dispatch fixing agent with:
  - Annotated review file path
  - Instruction: "Fix ONLY items marked [FIX]. Do not address [DEFERRED] or [WONTFIX] items."
- After fixes applied:
  - Run tests: `mise run test`
  - Run checks: `mise run check`
  - If pass → Continue to batch {N+1}
  - If fail → Repeat from Step 1 (new review cycle with incremented review filename)

**Step 5: Track deferred items**

Maintain running list of deferred items across all batches (stored in plan's Deferred section by gatekeeper).

After final batch, show summary:
```
Execution Complete

Total deferred items: {N} BLOCKING + {M} NON-BLOCKING
See plan file Deferred section for details.

Next: Address deferred items or create follow-up tasks?
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
1. Create a plan first? Use /brainstorm → /plan
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
- `${CLAUDE_PLUGIN_ROOT}plugin/skills/selecting-agents/SKILL.md` - Agent selection guide
- `${CLAUDE_PLUGIN_ROOT}plugin/skills/documentation/capturing-learning/SKILL.md` - Retrospective workflow

**Practices:**
- `${CLAUDE_PLUGIN_ROOT}plugin/practices/code-review.md` - Review standards and severity levels
- `${CLAUDE_PLUGIN_ROOT}plugin/practices/testing.md` - Test requirements
- `${CLAUDE_PLUGIN_ROOT}plugin/practices/development.md` - Development standards

**Commands:**
- `/code-review` - Manual code review trigger
- `/summarise` - Manual retrospective trigger
- `/brainstorm` → `/plan` - Plan creation workflow
