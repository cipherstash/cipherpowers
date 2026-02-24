---
name: verifying-plan-execution
description: Verify batch implementation matches plan specification exactly - checks plan adherence, not code quality
when_to_use: after executing plan batches, during /cipherpowers:verify execute, when checking if implementation matches plan
version: 1.0.0
---

# Verifying Plan Execution

## Overview

Verify that implementation matches plan specification exactly. This is plan adherence verification, NOT code quality review.

**Your only job:** Did implementation match what the plan specified?
**Not your job:** Is the code good? (that's code-review-agent)

**Announce at start:** "I'm using the verifying-plan-execution skill to check plan adherence."

## When to Use

- After executing plan batches
- During `/cipherpowers:verify execute`
- When checking if implementation matches plan specification
- Before proceeding to next batch

## Verification Workflow

### Step 1: Read Plan Tasks

For the specified batch, extract:
1. Task number/identifier
2. Complete specification of what should be implemented
3. Verification criteria
4. Expected files/locations

### Step 2: Read Implementation Changes

Review all code changes:
1. Use git diff or file reads
2. Identify modified/created files
3. Understand what was actually implemented

### Step 3: Verify Each Task

For each task:

```
Task [N]: [specification from plan]

Verification:
- Required: [what plan specified]
- Found: [what implementation contains]
- Status: COMPLETE / INCOMPLETE / DEVIATED

COMPLETE = Task implemented exactly as specified
INCOMPLETE = Task partially done, missing requirements, or skipped
DEVIATED = Task done differently than plan specified
```

### Step 4: Categorize Issues

- **BLOCKING:** Task INCOMPLETE or DEVIATED (must fix before next batch)
- **NON-BLOCKING:** Minor discrepancies that don't affect correctness

### Step 5: Save Report

Save to: `.work/{YYYY-MM-DD}-verify-execute-{HHmmss}.md`

Report structure:
```markdown
# Execute Completion Review - Batch [N]

## Metadata
- Review Date: {YYYY-MM-DD HH:mm:ss}
- Batch: [number]
- Plan File: [path]
- Tasks Reviewed: [identifiers]

## Summary
- Tasks Complete: X/Y
- Tasks Incomplete: X/Y
- Tasks Deviated: X/Y
- BLOCKING Issues: X
- NON-BLOCKING Issues: X

## BLOCKING (Must Fix)
[issues with Task, Plan specified, Implementation, Status, Impact, Action]

## NON-BLOCKING
[same structure or "None"]

## Tasks Verified Complete
[verified tasks with confirmation]

## Overall Assessment
COMPLETE / INCOMPLETE / PARTIAL
```

### Step 6: Report STATUS

End with: `STATUS: OK` (all complete) or `STATUS: BLOCKED` (issues found)

## Remember

- Focus on plan adherence, not code quality
- COMPLETE means exact match to specification
- DEVIATED means different approach (even if working)
- Save report before completing
- Announce file path in final response
