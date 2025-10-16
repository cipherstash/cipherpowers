# Execute (Algorithmic Version)

Execute implementation plans with automatic agent selection, batch-level code review, and retrospective completion.

## Decision Algorithm

**Run this algorithm BEFORE starting any work:**

```
1. Check: Does a file matching `docs/plans/*.md` OR `plans/*.md` exist?
   → YES: Go to step 2
   → NO: Go to step 7

2. Check: Is the task exploration/research only (no commits)?
   → YES: Go to step 7
   → NO: Go to step 3

3. Check: Is the task a single atomic command (run test, check status)?
   → YES: Go to step 7
   → NO: Go to step 4

4. YOU MUST USE /execute
   → Execute: `/execute [plan-file-path]`
   → STOP reading this algorithm
   → Do not proceed to step 5

5. [UNREACHABLE - if you reach here, you violated step 4]

6. [UNREACHABLE - if you reach here, you violated step 4]

7. Proceed without /execute (valid cases only)
```

## Already Started Without /execute?

**Run this recovery algorithm:**

```
1. Check: Have you written ANY code?
   → YES: Go to step 2
   → NO: Go to step 5

2. Check: Does that code have tests?
   → YES: Go to step 5
   → NO: Go to step 3

3. Delete the untested code
   → Execute: rm [files] OR git reset --hard
   → Go to step 4

4. Check: Does a plan file exist for remaining work?
   → YES: Execute `/execute [plan-file]`, STOP
   → NO: Create plan file, then execute `/execute [plan-file]`, STOP

5. Check: Are there remaining tasks requiring commits?
   → YES: Check if plan file exists for them
      → YES: Execute `/execute [plan-file]`, STOP
      → NO: Create plan file, then execute `/execute [plan-file]`, STOP
   → NO: Continue current work

6. [UNREACHABLE - all paths lead to STOP]
```

## Condition Evaluation Rules

**When evaluating conditions:**

- "Is task simple?" → NOT A VALID CONDITION (not in algorithm)
- "Is task just markdown?" → NOT A VALID CONDITION (not in algorithm)
- "Is there time pressure?" → NOT A VALID CONDITION (not in algorithm)
- "Will /execute take too long?" → NOT A VALID CONDITION (not in algorithm)
- "Should I be pragmatic?" → NOT A VALID CONDITION (not in algorithm)

**Valid questions only:**
- Does file X exist? (boolean: yes/no)
- Does code have tests? (boolean: yes/no)
- Is task exploration? (boolean: yes/no)
- Are commits required? (boolean: yes/no)

**Any other reasoning = algorithm violation**

## Why Algorithm Not Suggestion

**This is not a suggestion for these reasons:**

1. It's an algorithm with boolean conditions
2. Algorithms execute deterministically
3. You cannot "choose" whether to follow an algorithm
4. "Being pragmatic" is not a valid branch condition
5. Time pressure does not change boolean evaluation
6. Task simplicity is not tested by any condition
7. Your preferences are not algorithm inputs

**Algorithm inputs are:**
- File existence (checkable fact)
- Code test status (checkable fact)
- Task type (checkable fact)
- Commit requirements (checkable fact)

**Not algorithm inputs:**
- Your opinion on complexity
- Time pressure
- Sunk cost
- Exhaustion
- "Pragmatism"

## Test Your Understanding

**Before doing ANY work, answer:**

```
Q1: Does a file exist at docs/plans/my-task.md?
   → If YES: What does step 4 say to do?
   → If NO: What does step 7 say?

Q2: I wrote code 2 hours ago without tests. What does recovery algorithm step 3 say?
   → Answer: _____________

Q3: "These are simple markdown tasks" - is this a valid algorithm condition?
   → Answer: YES / NO
   → Why: _____________

Q4: What happens if I reach step 5 in the main algorithm?
   → Answer: _____________
```

**Correct answers:**
- Q1: If YES → "YOU MUST USE /execute", execute command. If NO → Proceed without
- Q2: "Delete the untested code"
- Q3: NO - Task simplicity is not tested by any condition
- Q4: Impossible - step 4 has STOP command, step 5 is unreachable

**If you got any wrong: RE-READ the algorithm before proceeding**

<instructions>
## Instructions

This command orchestrates plan execution in main Claude context, dispatching to specialized agents as needed:

1. **Load and analyze plan** - Read plan file, determine agent requirements
2. **Announce execution strategy** - Show which agents will handle which task types
3. **Execute in batches** - 3 tasks per batch with appropriate agents
4. **Review checkpoints** - Invoke code-reviewer after each batch
5. **Retrospective completion** - Prompt for learning capture when all tasks done

[Rest of instructions same as original execute.md...]

</instructions>
