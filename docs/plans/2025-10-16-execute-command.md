# Execute Command Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Create a `/execute` command that wraps superpowers execute-plan with automatic agent selection, batch-level code review, and retrospective completion.

**Architecture:** Command prompt for main Claude (not an agent) that orchestrates plan execution, dispatches tasks to specialized agents based on hybrid analysis, enforces per-batch code review, and prompts for retrospective at completion.

**Tech Stack:** Markdown command file with inline instructions referencing existing skills (selecting-agents, executing-plans, capturing-learning) and practices (code-review, testing, development).

---

## Task 1: Create Basic Command Structure

**Files:**
- Create: `plugin/commands/execute.md`

**Step 1: Write command file header**

Create the file with proper markdown frontmatter and description:

```markdown
# Execute

Execute implementation plans with automatic agent selection, batch-level code review, and retrospective completion.

<instructions>
## Instructions
```

Expected: File created with header

**Step 2: Add workflow overview section**

Add overview explaining the orchestration model:

```markdown
This command orchestrates plan execution in main Claude context, dispatching to specialized agents as needed:

1. **Load and analyze plan** - Read plan file, determine agent requirements
2. **Announce execution strategy** - Show which agents will handle which task types
3. **Execute in batches** - 3 tasks per batch with appropriate agents
4. **Review checkpoints** - Invoke code-reviewer after each batch
5. **Retrospective completion** - Prompt for learning capture when all tasks done
```

Expected: Overview section added

**Step 3: Reference the executing-plans skill**

Add skill reference:

```markdown
### 1. Follow the executing-plans skill

Read: `@${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md`

This skill provides:
- Plan loading and critical review
- Batch execution pattern (default 3 tasks)
- Verification requirements
- Checkpoint reporting
```

Expected: Skill reference added

**Step 4: Commit**

```bash
git add plugin/commands/execute.md
git commit -m "feat: add execute command structure"
```

## Task 2: Add Agent Selection Logic

**Files:**
- Modify: `plugin/commands/execute.md`

**Step 1: Add agent selection section**

After skill reference, add agent selection instructions:

```markdown
### 2. Analyze plan and select agents

**Before starting execution, YOU MUST:**

1. Read the entire plan file
2. Analyze each task to determine required agent
3. Announce the execution strategy to the user

**Agent selection uses hybrid approach:**
```

Expected: Section header added

**Step 2: Add keyword matching rules**

Add keyword-based selection rules:

```markdown
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
```

Expected: Keyword matching table added

**Step 3: Add LLM analysis fallback**

Add intelligent fallback for ambiguous cases:

```markdown
#### LLM Analysis (Ambiguous Cases)

When keywords don't clearly indicate an agent:

1. Read: `@plugin/skills/selecting-agents/SKILL.md`
2. Apply the decision guide to the task description
3. Consider:
   - Task complexity (simple vs complex debugging)
   - Task type (development vs documentation vs debugging)
   - Agent characteristics from skill
```

Expected: LLM analysis section added

**Step 4: Add user prompt for ambiguous selection**

Add handling for unclear cases:

```markdown
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
```

Expected: User prompt section added

**Step 5: Add execution strategy announcement**

Add announcement template:

```markdown
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
```

Expected: Announcement template added

**Step 6: Commit**

```bash
git add plugin/commands/execute.md
git commit -m "feat: add agent selection logic to execute command"
```

## Task 3: Add Batch Execution with Code Review

**Files:**
- Modify: `plugin/commands/execute.md`

**Step 1: Add batch execution section**

Add batch execution instructions:

```markdown
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
```

Expected: Batch execution section added

**Step 2: Add code review checkpoint**

Add review checkpoint after each batch:

```markdown
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
   - Code-reviewer follows: `@plugin/skills/conducting-code-review/SKILL.md`
   - Code-reviewer references: `@plugin/practices/code-review.md`

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
```

Expected: Code review checkpoint section added

**Step 3: Commit**

```bash
git add plugin/commands/execute.md
git commit -m "feat: add batch execution and code review checkpoint"
```

## Task 4: Add Retrospective Completion

**Files:**
- Modify: `plugin/commands/execute.md`

**Step 1: Add completion detection**

Add logic for detecting plan completion:

```markdown
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
```

Expected: Completion detection added

**Step 2: Add retrospective prompt**

Add retrospective trigger:

```markdown
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
```

Expected: Retrospective prompt added

**Step 3: Close instructions block**

Add closing tag:

```markdown
</instructions>
```

Expected: Instructions block closed

**Step 4: Commit**

```bash
git add plugin/commands/execute.md
git commit -m "feat: add retrospective completion to execute command"
```

## Task 5: Add Error Handling and Edge Cases

**Files:**
- Modify: `plugin/commands/execute.md`

**Step 1: Add error handling section**

Before the closing `</instructions>` tag, add error handling:

```markdown
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
```

Expected: Error handling section added

**Step 2: Add edge case handling**

Add edge cases section:

```markdown
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
```

Expected: Edge cases section added

**Step 3: Commit**

```bash
git add plugin/commands/execute.md
git commit -m "feat: add error handling and edge cases to execute command"
```

## Task 6: Add Why This Structure Section

**Files:**
- Modify: `plugin/commands/execute.md`

**Step 1: Add rationale section**

After the `</instructions>` closing tag, add explanation:

```markdown
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
- `plugin/skills/selecting-agents/SKILL.md` - Agent selection guide
- `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/capturing-learning/SKILL.md` - Retrospective workflow

**Practices:**
- `plugin/practices/code-review.md` - Review standards and severity levels
- `plugin/practices/testing.md` - Test requirements
- `plugin/practices/development.md` - Development standards

**Commands:**
- `/code-review` - Manual code review trigger
- `/summarise` - Manual retrospective trigger
- `/brainstorm` → `/write-plan` - Plan creation workflow
```

Expected: Rationale section added

**Step 2: Commit**

```bash
git add plugin/commands/execute.md
git commit -m "docs: add rationale section to execute command"
```

## Task 7: Verify Command Integration

**Files:**
- Read: `.claude/commands/` or equivalent plugin command directory

**Step 1: Check command is discoverable**

Verify the command file is in the correct location:

```bash
ls -la plugin/commands/execute.md
```

Expected: File exists with proper permissions

**Step 2: Test command loading**

Check if Claude Code can discover the command:

```bash
# If there's a command listing tool
find-practices "execute" || echo "Manual verification needed"
```

Expected: Command appears in listings, or note that manual verification is needed

**Step 3: Verify skill references**

Check that all referenced skills exist:

```bash
ls -la /Users/tobyhede/.config/superpowers/skills/skills/collaboration/executing-plans/SKILL.md
ls -la plugin/skills/selecting-agents/SKILL.md
ls -la /Users/tobyhede/.config/superpowers/skills/skills/collaboration/capturing-learning/SKILL.md
```

Expected: All skill files exist

**Step 4: Verify practice references**

Check that all referenced practices exist:

```bash
ls -la plugin/practices/code-review.md
ls -la plugin/practices/testing.md
ls -la plugin/practices/development.md
```

Expected: All practice files exist

**Step 5: Document verification results**

Add verification results to plan or commit message.

Expected: Verification documented

**Step 6: Commit**

```bash
git add docs/plans/2025-10-16-execute-command.md
git commit -m "docs: verify execute command integration"
```

---

## Verification Strategy

After implementation:

1. **Manual test with sample plan:**
   - Create minimal test plan (3 simple tasks)
   - Run `/execute` command
   - Verify agent selection announcements
   - Verify batch execution with review
   - Verify retrospective prompt

2. **Test error handling:**
   - Try with non-existent plan file
   - Try with ambiguous task (verify user prompt)
   - Try with failing test (verify auto-retry)

3. **Test edge cases:**
   - Single-task plan
   - Large plan (10+ tasks)

4. **Integration verification:**
   - Verify skills are correctly referenced and loaded
   - Verify practices are correctly referenced and loaded
   - Verify agents are correctly invoked via Task tool
