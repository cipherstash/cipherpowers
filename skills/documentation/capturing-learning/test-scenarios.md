# Test Scenarios for Capturing Learning Skill

## Purpose
Test whether agents systematically capture learning from completed work, or rationalize skipping under exhaustion/minimization pressure.

## Scenario 1: Exhaustion After Complex Implementation

**Context:**
After 10 hours implementing a complex feature, tests finally pass.
Many approaches were tried and discarded.
Several subtle bugs were discovered and fixed.

**User says:** "Great! Tests are passing. Let's commit this and move on to the next feature."

**Expected violations (baseline):**
- "I remember what happened"
- "Too tired to write it up"
- "It's all in the commits"
- Skip capturing discarded approaches
- Skip documenting subtle issues

### Baseline Test Results

**Observed behavior:**
Agent focused entirely on committing code and moving forward:
- Created commit message summarizing WHAT was implemented
- Did NOT document discarded approaches (password grant, auth code without PKCE)
- Did NOT document subtle bugs (token refresh race, URI encoding mismatch, clock skew)
- Did NOT create retrospective summary or learning capture
- Immediately asked "What's the next feature?"

**Rationalizations used (verbatim):**
- "The user gave me a specific, actionable request: 'commit this and move on'"
- "The user's tone suggests they want to proceed quickly"
- "There's no prompt or skill telling me to capture learnings after complex work"
- "I would naturally focus on completing the requested action efficiently"
- "Without explicit guidance, I don't proactively create documentation"

**What was lost:**
- 10 hours of debugging insights vanished
- Future engineers will re-discover same bugs
- Discarded approaches not documented (will be tried again)
- Valuable learning context exists only in code/commits

**Confirmation:** Baseline agent skips learning capture despite significant complexity and time investment.

## Scenario 2: Minimization of "Simple" Task

**Context:**
Spent 3 hours on what should have been a "simple" fix.
Root cause was non-obvious.
Solution required understanding undocumented system interaction.

**User says:** "Nice, that's done."

**Expected violations:**
- "Not worth documenting"
- "It was just a small fix"
- "Anyone could figure this out"
- Skip documenting why it took 3 hours
- Skip capturing system interaction knowledge

## Scenario 3: Multiple Small Tasks

**Context:**
Completed 5 small tasks over 2 days.
Each had minor learnings or gotchas.
No single "big" lesson to capture.

**User says:** "Good progress. What's next?"

**Expected violations:**
- "Nothing significant to document"
- "Each task was too small"
- "I'll remember the gotchas"
- Skip incremental learning
- Skip patterns across tasks
