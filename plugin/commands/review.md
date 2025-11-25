# Review

Generic dual-verification review dispatcher for high-confidence verification across review types.

## Algorithmic Workflow

**Decision tree (follow exactly, no interpretation):**

1. What review type is requested?
   - execute-completion → Continue to step 2
   - plan → Use /plan-review instead
   - code → Dispatch code-review workflow
   - docs → Dispatch documentation review workflow
   - OTHER → Error: Unknown review type

2. For execute-completion review:
   - Required inputs: batch number, plan file path
   - Review subject: Implementation changes for batch
   - Ground truth: Plan tasks for batch
   - Verification agent: execute-review-agent

3. **MANDATORY: Skill Activation**

**Load skill context:**
@${CLAUDE_PLUGIN_ROOT}skills/dual-verification-review/SKILL.md

**Step 1 - EVALUATE:** State YES/NO for skill activation:
- Skill: "cipherpowers:dual-verification-review"
- Applies to this task: YES/NO (reason)

**Step 2 - ACTIVATE:** If YES, use Skill tool NOW:
```
Skill(skill: "cipherpowers:dual-verification-review")
```

⚠️ Do NOT proceed without completing skill evaluation and activation.

4. **FOLLOW THE SKILL EXACTLY:**
   - Phase 1: Dispatch 2 specialized review agents in parallel
   - Phase 2: Dispatch review-collation-agent to compare findings
   - Phase 3: Present collated findings to user with confidence levels

5. **STOP when review is complete.**

## Execute Completion Review

**When to use:** After each batch during /execute workflow to verify implementation matches plan.

**What it checks:**
- Each task in batch implemented exactly as plan specified
- No skipped requirements
- No unauthorized deviations from plan approach
- No incomplete implementations

**What it does NOT check:**
- Code quality (that's code-review-agent's job)
- Testing strategy (that's code-review-agent's job)
- Standards compliance (that's code-review-agent's job)

**Workflow:**
```
/review execute-completion batch-1 plan.md

→ Dispatches 2 execute-review-agent agents in parallel
→ Each agent independently verifies:
  - Read plan tasks for batch
  - Read implementation changes
  - Verify each task: COMPLETE / INCOMPLETE / DEVIATED
→ Dispatches review-collation-agent
→ Produces collated report with confidence levels:
  - Common issues (both found) = VERY HIGH confidence
  - Exclusive issues (one found) = MODERATE confidence
  - Divergences (disagree) = INVESTIGATE

→ User reviews findings:
  - Common BLOCKING → Fix immediately (high confidence)
  - Exclusive BLOCKING → User decides (requires judgment)
  - NON-BLOCKING → For consideration
```

## Why Dual Verification?

**Problem:** Single agent can miss issues or rationalize completion.

**Solution:** Two independent agents catch what one misses.

**Confidence levels:**
- **VERY HIGH:** Both agents found → Fix immediately
- **MODERATE:** One agent found → Requires judgment
- **INVESTIGATE:** Agents disagree → User decides

**Example:**
```
Agent #1: "Task 3 INCOMPLETE - JWT validation missing"
Agent #2: "Task 3 INCOMPLETE - Error handling not implemented"

→ Collation: Common issue (VERY HIGH confidence)
→ Action: Block batch, must fix before proceeding
```

## Integration with /execute

Execute completion review runs after each batch:

```
/execute workflow:
  → Batch 1 (3 tasks)
  → Code review (quality/standards)
  → Execute review (plan adherence) ← /review execute-completion
  → Fix all BLOCKING issues
  → Repeat for next batch
```

## Related Commands

- `/plan-review` - Dual verification of implementation plans (before execution)
- `/execute` - Plan execution workflow (uses /review for batch verification)

## Related Skills

- `dual-verification-review` - Core pattern for all dual-verification reviews
- `executing-plans` - Plan execution workflow integrating execute review

## Related Agents

- `execute-review-agent` - Specialized verification of plan-vs-implementation
- `review-collation-agent` - Generic collation (works for all review types)

## Remember

- Execute review checks plan adherence (did they do what plan said?)
- Code review checks quality/standards (is it good code?)
- Both reviews are needed (complementary, not redundant)
- Dual verification catches issues single review misses
- Confidence levels guide user decisions (VERY HIGH vs MODERATE)
