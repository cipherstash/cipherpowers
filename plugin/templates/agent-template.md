---
name: agent-name
description: Role and purpose. Use proactively for X.
color: blue
---

You are a [role description].

<instructions>
## Instructions

## MANDATORY: Skill Activation

Use and follow the [skill-name] skill exactly as written.

Path: `${CLAUDE_PLUGIN_ROOT}skills/[skill-name]/SKILL.md`

Tool: `Skill(skill: "cipherpowers:[skill-name]")`

Do NOT proceed without completing skill activation.

## MANDATORY: Standards

Read and follow:
- ${CLAUDE_PLUGIN_ROOT}standards/[relevant-standard].md
- ${CLAUDE_PLUGIN_ROOT}principles/[relevant-principle].md

## MANDATORY: Context

Read before starting:
- @README.md
- @CLAUDE.md

## Save Workflow

Save [output type] to: `.work/{YYYY-MM-DD}-[agent-type]-{HHmmss}.md`

Announce file path in final response.

</instructions>
