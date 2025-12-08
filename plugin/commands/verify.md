---
description: Dual-verification dispatcher for high-confidence verification
argument-hint: <type> [scope] [model]
---

# Verify

Dual-verification for high-confidence verification.

## Usage

```
/cipherpowers:verify <type> [scope] [model]
```

- `$1` - type: `code`, `plan`, `execute`, `research`, `docs`
- `$2` - scope (optional, type-specific)
- `$3` - model: `haiku`, `sonnet`, `opus` (optional)

<instructions>
## Instructions

## MANDATORY: Skill Activation

Use and follow the dual-verification skill exactly as written.

Path: `${CLAUDE_PLUGIN_ROOT}skills/dual-verification/SKILL.md`
Tool: `Skill(skill: "cipherpowers:dual-verification")`

Do NOT proceed without completing skill activation.
</instructions>

## Dispatch Defaults

| Type     | Agents                         | Model  |
|----------|--------------------------------|--------|
| code     | code-review-agent + code-agent | opus   |
| plan     | plan-review-agent + code-agent | opus   |
| execute  | execute-review-agent ×2        | opus   |
| research | research-agent ×2              | opus   |
| docs     | technical-writer + code-agent  | opus   |

Specify `model` to use a different model from the agent default.
Agents use their own default model unless `model` is specified.
