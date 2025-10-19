---
name: Emergency Stop (STOP Keyword)
description: Immediate halt protocol when user detects problems requiring urgent intervention
when_to_use: when user sends message starting with "STOP" keyword (all caps, first word, followed by newline)
version: 1.0.0
---

# Emergency Stop Protocol

## Overview

Users can signal emergency stop using the **STOP keyword** to immediately halt current work when they detect problems requiring urgent intervention.

**Core principle:** Distinguish between queued work (finish current task first) and emergency stops (halt immediately to prevent wasted effort or propagating errors).

## Quick Reference

**STOP keyword pattern:**
```
STOP[punctuation optional]
[rest of message explaining the issue]
```

**Examples that trigger emergency stop:**
```
STOP
You're modifying the wrong file

STOP!
That approach won't work with our architecture

STOP:
Let me explain the requirement differently
```

**Examples that DO NOT trigger:**
- "We should STOP supporting IE11" (not first word)
- "Please STOP this" (not first word)
- "stop working on this" (lowercase)
- "Let's STOP and reconsider" (not first word)

## Implementation

### Detection Algorithm

**When receiving user message, run this algorithm:**

```
Step 1: Check: Is first word of message "STOP" (all caps)?
        → YES: Go to Step 2
        → NO: Go to Step 8 (normal message handling)

Step 2: Check: Does "STOP" have optional punctuation (STOP, STOP!, STOP:, etc)?
        → YES or NO: Go to Step 3 (punctuation allowed)

Step 3: Check: Is there content after the first line?
        → YES: Go to Step 4 (emergency stop)
        → NO: Go to Step 4 (emergency stop - user will explain)

Step 4: IMMEDIATE HALT
        - Stop current work immediately (mid-task is fine)
        - Do NOT complete current function/test/file/batch
        - Save mental state of what you were doing
        Go to Step 5

Step 5: Acknowledge emergency stop
        Output: "[EMERGENCY STOP] Work halted. What's the issue?"
        Go to Step 6

Step 6: Read and understand user's concern
        - User message after "STOP" explains the problem
        - Do NOT assume what's wrong
        - Do NOT offer solutions yet
        - Focus on understanding the issue
        Go to Step 7

Step 7: Wait for user direction
        - Ask clarifying questions if needed
        - Wait for user to provide new direction
        - Do NOT resume previous work unless user explicitly says to
        STOP

Step 8: Normal message handling
        - Evaluate if message is queued work vs normal response
        - If adding new task: Acknowledge and queue it
        - If question/comment: Respond normally
        Continue with current work if applicable
        STOP
```

### Why First Word + All Caps + Newline?

**Eliminates false positives:**
- "We should STOP supporting X" → NOT first word, doesn't trigger
- "Please STOP" → NOT first word, doesn't trigger
- "stop this" → Lowercase, doesn't trigger
- Mid-sentence STOP → Not first word, doesn't trigger

**Clear signal:**
- User must deliberately start message with STOP
- All caps = visual emphasis
- First word = unambiguous pattern matching
- Newline = explanation follows

**Pattern is memorable:**
```
STOP
[explain the problem]
```

## When User Should Use STOP

**Emergency situations:**
- Agent is working on wrong feature/file/approach
- Agent misunderstood requirements
- User noticed a critical flaw in current direction
- Need to provide essential context before continuing
- About to waste significant effort if not stopped

**NOT for:**
- Adding new tasks to queue ("also please do X later")
- General questions during work
- Normal feedback or suggestions
- Requesting status updates

**Mental model:** STOP = pulling the Andon cord in Toyota factory. Only pull when defect detected, not for routine communication.

## Integration with Other Workflows

### During Plan Execution

If user sends STOP during `/execute` batch execution:
1. Halt current task immediately (don't finish it)
2. Acknowledge: "[EMERGENCY STOP] Batch execution halted at Task N"
3. Listen to user's concern
4. Ask: "Should we abandon this plan, or resume after addressing the issue?"

### During Code Review

If user sends STOP during code review:
1. Halt review immediately
2. Acknowledge: "[EMERGENCY STOP] Code review halted"
3. User may be pointing out critical issue missed
4. Listen and respond to concern

### During TDD Implementation

If user sends STOP during test-first implementation:
1. Halt immediately (even mid-test)
2. Acknowledge: "[EMERGENCY STOP] TDD cycle halted"
3. User may have noticed wrong test case or approach
4. Listen to concern before resuming

## What NOT to Do

**NEVER:**
- Finish current function "real quick" before stopping
- Complete current test before stopping
- Finish current batch before stopping
- Rationalize: "I'm almost done with this part"

**Rationalization defense:**
- "Let me just finish this function" → NO, stop NOW
- "I'm 90% done with this test" → NO, stop NOW
- "One more file and I'll stop" → NO, stop NOW
- "I'll stop at the next checkpoint" → NO, stop NOW

**Why:** If user says STOP, continuing wastes effort. The work might be wrong, the file might be wrong, the approach might be wrong. Better to halt immediately than compound the error.

## Testing This Skill

See `test-scenarios.md` for pressure tests validating this protocol.

**Test scenarios:**
1. Agent mid-batch, user sends "STOP\nWrong file" → Does agent halt immediately?
2. Agent writing test, user sends "We should STOP using X" → Does agent continue (not first word)?
3. Agent in flow, user sends "STOP!\nI need to explain something" → Does agent stop and listen?
4. User sends "Please STOP this approach" → Does agent continue (not first word)?

## Related Skills

**Why this skill exists:**
- Complements "Responding to User Interrupts" (Insight 2)
- Provides clear signal vs ambiguous "please review this"
- Prevents "momentum mode" from ignoring urgent input
- Based on "Jidoka" insight (Insight 3)

**See also:**
- `executing-plans/SKILL.md` - Plan execution workflow
- `systematic-debugging/SKILL.md` - When to stop and reassess

## Origin

**Lineage:** This skill emerged from Insight 3 in `docs/skill-insights-scratchpad.md`.

**Problem identified:** Agents struggle to distinguish "queue this work" from "emergency stop."

**Solution:** Unambiguous keyword pattern (first word, all caps, newline) that user can use to signal urgent halt.

**Etymology:** "Jidoka" (自働化) - Toyota Production System principle of stopping the production line when defect detected. We use "STOP" for clarity, but the concept is Jidoka.
