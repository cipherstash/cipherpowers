---
description: Implement findings from verification reports
argument-hint: [scope] [collation-file]
---

# Revise

Implement findings from verification reports. Works with collation reports produced by `/verify`.

## Usage

```
/cipherpowers:revise [scope] [collation-file]
```

- `$1` - scope: `common`, `exclusive`, `all` (default: all)
- `$2` - collation file path (optional)

## Scope Reference

| Scope      | Use Case                                                          |
|------------|-------------------------------------------------------------------|
| common     | Findings agreed by both reviewers (can proceed immediately)       |
| exclusive  | Findings unique to one reviewer (requires cross-check completion) |
| all        | All findings (both common and exclusive)                          |

<instructions>
## Instructions

## MANDATORY: Skill Activation

Use and follow the revising-findings skill exactly as written.

Path: `${CLAUDE_PLUGIN_ROOT}skills/revising-findings/SKILL.md`
Tool: `Skill(skill: "cipherpowers:revising-findings")`

Do NOT proceed without completing skill activation.
</instructions>
