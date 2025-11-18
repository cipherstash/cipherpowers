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
2. **Select appropriate agent:**
   - Rust code → `cipherpowers:rust-engineer`
   - Complex debugging → `cipherpowers:ultrathink-debugger`
   - Documentation → `cipherpowers:technical-writer`
   - General implementation → `general-purpose`
3. Dispatch agent with task instructions from plan
4. Follow each step exactly (plan has bite-sized steps)
5. Run verifications as specified
6. Mark as completed

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

## Related Skills

**Agent selection guidance:**
- Selecting Agents: `${CLAUDE_PLUGIN_ROOT}plugin/skills/selecting-agents/SKILL.md`

**Code review workflow:**
- Requesting Code Review: `${CLAUDE_PLUGIN_ROOT}plugin/skills/requesting-code-review/SKILL.md`

**Finishing work:**
- Finishing a Development Branch: `${CLAUDE_PLUGIN_ROOT}plugin/skills/finishing-a-development-branch/SKILL.md`

## Remember
- Review plan critically first
- Select the right agent for each task type
- Code review after every batch (mandatory)
- Follow plan steps exactly
- Don't skip verifications
- Reference skills when plan says to
- Between batches: just report and wait
- Stop when blocked, don't guess
