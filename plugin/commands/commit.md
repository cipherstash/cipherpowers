# Commit

Systematic git commit with atomic commits and conventional messages.

## MANDATORY: Skill Activation

**Load skill context:**
@${CLAUDE_PLUGIN_ROOT}skills/commit-workflow/SKILL.md

**Step 1 - EVALUATE:** State YES/NO for skill activation:
- Skill: "cipherpowers:commit-workflow"
- Applies to this task: YES/NO (reason)

**Step 2 - ACTIVATE:** If YES, use Skill tool NOW:
```
Skill(skill: "cipherpowers:commit-workflow")
```

⚠️ Do NOT proceed without completing skill evaluation and activation.

---

## Algorithmic Dispatch

**Decision tree (follow exactly, no interpretation):**

1. Is this a commit request?
   - YES → Continue to step 2
   - NO → This command was invoked incorrectly

2. Have you already dispatched to commit-agent agent?
   - YES → Wait for agent to complete
   - NO → Continue to step 3

3. **DISPATCH TO AGENT NOW:**

```
Use Task tool with:
  subagent_type: "cipherpowers:commit-agent"
  description: "Commit workflow"
  prompt: """
  [User's original request or task context]

  Follow the commit-workflow skill exactly as written.
  """
```

4. **STOP. Do not proceed in main context.**

## Why Algorithmic Dispatch?

- **100% reliability**: No interpretation, no rationalization
- **Agent enforcement**: Persuasion principles prevent shortcuts
- **Consistent quality**: Every commit follows non-negotiable workflow
- **Skill integration**: Agent reads commit-workflow skill automatically

## What the Agent Does

The commit-agent agent implements:
- Staging status check
- Diff review and understanding
- Atomic commit analysis
- Conventional commit message formatting
- Commit verification

**References:**
- Agent: `${CLAUDE_PLUGIN_ROOT}agents/commit-agent.md`
- Skill: `${CLAUDE_PLUGIN_ROOT}skills/commit-workflow/SKILL.md`
- Standards: `${CLAUDE_PLUGIN_ROOT}standards/conventional-commits.md`
