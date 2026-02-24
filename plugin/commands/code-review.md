---
description: Code review with structured feedback
argument-hint: [scope] [model]
---

# Code Review

Code review with structured feedback using specialised code-review-agent.

## Usage

```
/cipherpowers:code-review [scope] [model]
```

- `$1` - scope (optional, defaults to all changes)
- `$2` - model override: `haiku`, `sonnet`, `opus` (optional)


<instructions>
## Instructions

## MANDATORY: Skill Activation

Use and follow the requesting-code-review skill exactly as written.

Path: `${CLAUDE_PLUGIN_ROOT}skills/requesting-code-review/SKILL.md`

Tool: `Skill(skill: "cipherpowers:requesting-code-review")`

Do NOT proceed without completing skill activation.
</instructions>


## Dispatch Defaults

| Task Type   | Agent               | Model  |
|-------------|---------------------|--------|
| code-revew  | code-review-agent   | opus   |

Specify `model` to use a different model from the agent default.
Agents use their own default model unless `model` is specified.

