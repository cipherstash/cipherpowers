# Code Review

Thorough code review with test verification and structured feedback.

## Algorithmic Dispatch

**Decision tree (follow exactly, no interpretation):**

1. Is this a code review request?
   - YES → Continue to step 2
   - NO → This command was invoked incorrectly

2. Have you already dispatched to code-reviewer agent?
   - YES → Wait for agent to complete
   - NO → Continue to step 3

3. **DISPATCH TO AGENT NOW:**

```
Use Task tool with:
  subagent_type: "cipherpowers:code-reviewer"
  description: "Code review workflow"
  prompt: """
  [User's original request or task context]

  Follow the conducting-code-review skill exactly as written.

  Review the recent changes and provide structured feedback.
  """
```

4. **STOP. Do not proceed in main context.**

## Why Algorithmic Dispatch?

- **100% reliability**: No interpretation, no rationalization
- **Agent enforcement**: Persuasion principles prevent rubber-stamping
- **Consistent quality**: Every review runs tests, checks all severity levels
- **Skill integration**: Agent reads conducting-code-review skill automatically

## What the Agent Does

The code-reviewer agent implements:
- Identify code to review (git commands)
- Run all project tests and checks
- Review against practice standards (ALL severity levels)
- Save structured feedback to work directory
- No approval without thorough review

**References:**
- Agent: `${CLAUDE_PLUGIN_ROOT}agents/code-reviewer.md`
- Skill: `${CLAUDE_PLUGIN_ROOT}skills/conducting-code-review/SKILL.md`
- Standards: `${CLAUDE_PLUGIN_ROOT}standards/code-review.md`
