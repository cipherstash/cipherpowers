# Code Review

Thorough code review with test verification and structured feedback.

## Usage

```
/cipherpowers:code-review [--model=<sonnet|opus|haiku>]
```

**Model guidance:**
- `opus` - Deep analysis, security-critical code, complex architecture
- `sonnet` - Balanced quality/speed (default if not specified)
- `haiku` - Quick reviews, simple changes

## MANDATORY: Skill Activation

**Load skill context:**
@${CLAUDE_PLUGIN_ROOT}skills/conducting-code-review/SKILL.md

**Step 1 - EVALUATE:** State YES/NO for skill activation:
- Skill: "cipherpowers:conducting-code-review"
- Applies to this task: YES/NO (reason)

**Step 2 - ACTIVATE:** If YES, use Skill tool NOW:
```
Skill(skill: "cipherpowers:conducting-code-review")
```

⚠️ Do NOT proceed without completing skill evaluation and activation.

---

## Algorithmic Dispatch

**Decision tree (follow exactly, no interpretation):**

1. Is this a code review request?
   - YES → Continue to step 2
   - NO → This command was invoked incorrectly

2. Have you already dispatched to code-review-agent agent?
   - YES → Wait for agent to complete
   - NO → Continue to step 3

3. **DISPATCH TO AGENT NOW:**

```
Use Task tool with:
  subagent_type: "cipherpowers:code-review-agent"
  model: [from --model arg if provided, otherwise omit to use default]
  description: "Code review workflow"
  prompt: """
  [User's original request or task context]

  Follow the conducting-code-review skill exactly as written.

  Review the recent changes and provide structured feedback.
  """
```

**Model parameter rules:**
- If user specified `--model=X` → pass `model: X` to Task tool
- If no model specified → omit model parameter (agent default applies)

4. **STOP. Do not proceed in main context.**

## Why Algorithmic Dispatch?

- **100% reliability**: No interpretation, no rationalization
- **Agent enforcement**: Persuasion principles prevent rubber-stamping
- **Consistent quality**: Every review runs tests, checks all severity levels
- **Skill integration**: Agent reads conducting-code-review skill automatically

## What the Agent Does

The code-review-agent agent implements:
- Identify code to review (git commands)
- Run all project tests and checks
- Review against practice standards (ALL severity levels)
- Save structured feedback to work directory
- No approval without thorough review

**References:**
- Agent: `${CLAUDE_PLUGIN_ROOT}agents/code-review-agent.md`
- Skill: `${CLAUDE_PLUGIN_ROOT}skills/conducting-code-review/SKILL.md`
- Standards: `${CLAUDE_PLUGIN_ROOT}standards/code-review.md`
