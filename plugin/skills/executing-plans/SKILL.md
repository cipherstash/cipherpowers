---
name: executing-plans
description: Use when partner provides a complete implementation plan to execute in controlled batches with review checkpoints - loads plan, reviews critically, executes tasks in batches, reports for review between batches
---

# Executing Plans

## Overview

Load plan, review critically, execute tasks in batches, report for review between batches.

**Core principle:** Batch execution with checkpoints for architect review.

**Announce at start:** "I'm using the executing-plans skill to implement this plan."

## The Process

### Step 1: Load and Review Plan
1. Read plan file
2. Review critically - identify any questions or concerns about the plan
3. If concerns: Raise them with your human partner before starting
4. If no concerns: Create TodoWrite and proceed

### Step 2: Execute Batch
**Default: First 3 tasks**

For each task:
1. Mark as in_progress
2. **Select appropriate agent using semantic understanding (NOT keyword matching):**

   **Analyze task requirements:**
   - What is the task type? (implementation, debugging, review, docs)
   - What is the complexity? (simple fix vs multi-component investigation)
   - What technology? (Rust vs other languages)

   **Agent selection:**
   - Rust implementation → `cipherpowers:rust-engineer`
   - Complex, multi-layered debugging → `cipherpowers:ultrathink-debugger`
   - Documentation updates → `cipherpowers:technical-writer`
   - General implementation → `general-purpose`

   **IMPORTANT:** Analyze the task semantically. Don't just match keywords.
   - ❌ "don't use ultrathink" → ultrathink-debugger (keyword match)
   - ✅ "don't use ultrathink" → general-purpose (semantic understanding)

   See selecting-agents skill for detailed selection criteria.

3. **Dispatch agent with embedded following-plans skill:**

   **Include in agent prompt:**
   ```
   IMPORTANT: You MUST follow the plan exactly as specified.

   Read and follow: @${CLAUDE_PLUGIN_ROOT}skills/following-plans/SKILL.md

   This skill defines when you can make changes vs when you must report BLOCKED.

   REQUIRED: Your completion report MUST include STATUS:
   - STATUS: OK (task completed as planned)
   - STATUS: BLOCKED (plan approach won't work, need approval for deviation)

   The plan approach was chosen for specific reasons during design.
   Do NOT rationalize "simpler" approaches without approval.
   ```

4. Follow each step exactly (plan has bite-sized steps)
5. Run verifications as specified
6. **Check agent completion status:**
   - STATUS: OK → Mark as completed, continue
   - STATUS: BLOCKED → STOP, handle escalation (see Handling BLOCKED Status)
   - No STATUS → Agent violated protocol, escalate

### Step 3: Review Batch (REQUIRED)

**REQUIRED SUB-SKILL:** Use cipherpowers:requesting-code-review

After batch complete:
1. Follow requesting-code-review skill to dispatch code-reviewer agent
2. Fix BLOCKING issues before continuing to next batch
3. Address NON-BLOCKING feedback or defer with justification

**Code review is mandatory between batches. No exceptions.**

### Step 4: Report
When batch complete:
- Show what was implemented
- Show verification output
- Say: "Ready for feedback."

### Step 5: Continue
Based on feedback:
- Apply changes if needed
- Execute next batch
- Repeat until complete

### Step 6: Complete Development

After all tasks complete and verified:
- Announce: "I'm using the finishing-a-development-branch skill to complete this work."
- **REQUIRED SUB-SKILL:** Use cipherpowers:finishing-a-development-branch
- Follow that skill to verify tests, present options, execute choice

## When to Stop and Ask for Help

**STOP executing immediately when:**
- Hit a blocker mid-batch (missing dependency, test fails, instruction unclear)
- Plan has critical gaps preventing starting
- You don't understand an instruction
- Verification fails repeatedly

**Ask for clarification rather than guessing.**

## When to Revisit Earlier Steps

**Return to Review (Step 1) when:**
- Partner updates the plan based on your feedback
- Fundamental approach needs rethinking

**Don't force through blockers** - stop and ask.

## Handling BLOCKED Status

When an agent reports STATUS: BLOCKED:

1. **Read the BLOCKED reason carefully**
   - What does agent say won't work?
   - What deviation does agent want to make?

2. **Review plan and design context**
   - Why was this approach chosen?
   - Was the agent's "simpler" approach already considered and rejected?

3. **Ask user what to do** via AskUserQuestion:
   ```
   Agent reported BLOCKED on: {task}

   Reason: {agent's reason}

   Plan specified: {planned approach}
   Agent wants: {agent's proposed deviation}

   Options:
   1. Trust agent - approve deviation from plan
   2. Revise plan - update task with different approach
   3. Enforce plan - agent must follow plan as written
   4. Investigate - need more context before deciding
   ```

4. **Execute user decision:**
   - Approve → Update plan, re-dispatch agent with approved approach
   - Revise → Update plan file, re-dispatch agent
   - Enforce → Re-dispatch agent with stronger "follow plan" guidance
   - Investigate → Pause execution, gather more information

**Never approve deviations without user input.**

## Related Skills

**Agent selection guidance:**
- Selecting Agents: `${CLAUDE_PLUGIN_ROOT}skills/selecting-agents/SKILL.md`

**Code review workflow:**
- Requesting Code Review: `${CLAUDE_PLUGIN_ROOT}skills/requesting-code-review/SKILL.md`

**Finishing work:**
- Finishing a Development Branch: `${CLAUDE_PLUGIN_ROOT}skills/finishing-a-development-branch/SKILL.md`

**Plan compliance:**
- Following Plans: `${CLAUDE_PLUGIN_ROOT}skills/following-plans/SKILL.md`

## Remember
- Review plan critically first
- Embed following-plans skill in agent prompts
- Select the right agent using semantic understanding (not keyword matching)
- Check for STATUS in agent completions
- Handle BLOCKED status by asking user (never auto-approve deviations)
- Code review after every batch (mandatory)
- Don't skip verifications
- Reference skills when plan says to
- Between batches: just report and wait
- Stop when blocked, don't guess
