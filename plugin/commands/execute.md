---
description: Execute implementation plans in batches with specialised agents
argument-hint: [plan-file] [agent] [model]
---

# Execute

Execute implementation plans.

## Usage

```
/cipherpowers:execute [plan-file] [agent] [model]
```

- `$1` - plan file path (default: searches for plan in working directory)
- `$2` - agent to use (default: selected by task type from plan)
- `$3` - model: `haiku`, `sonnet`, `opus` (default: per agent below)

<instructions>
## Instructions

## MANDATORY: Skill Activation

Use and follow the executing-plans skill exactly as written.

Path: `${CLAUDE_PLUGIN_ROOT}skills/executing-plans/SKILL.md`
Tool: `Skill(skill: "cipherpowers:executing-plans")`

Do NOT proceed without completing skill activation.
</instructions>


## Dispatch Defaults

When no agent is specified, task type determines the agent:

| Task Type   | Agent               | Model  |
|-------------|---------------------|--------|
| rust code   | rust-exec-agent     | haiku  |
| code        | code-exec-agent     | haiku  |
| code-revew  | code-review-agent   | opus   |
| docs        | technical-writer    | opus   |
| debugging   | ultrathink-debugger | opus   |

Specify `agent` to use a different agent (e.g., `rust-agent` instead of `rust-exec-agent`).

Specify `model` to use a different model from the agent default.
Agents use their own default model unless `model` is specified.

