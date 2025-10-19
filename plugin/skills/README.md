# CipherPowers Skills

Organization-specific skills following the superpowers framework.

## Creating Skills

See `${SUPERPOWERS_SKILLS_ROOT}/skills/meta/writing-skills/SKILL.md` for the complete process.

**Key principles:**
1. Test with subagents BEFORE writing (TDD for docs)
2. Use TodoWrite for skill creation checklist
3. Include rich `when_to_use` frontmatter
4. Follow SKILL.md structure

## Scope

Skills in this directory are:
- Organization-specific workflows
- Extensions to superpowers skills
- Universal skills under development (before upstreaming)

## Discovery

Skills are automatically discovered by Claude Code's native Skill tool. All skills in `plugin/skills/` are available in Claude Code sessions.

To use a skill in conversation, Claude uses the `Skill` tool with the skill name. For example:
- `Skill(command: "cipherpowers:conducting-code-review")`
- `Skill(command: "superpowers:brainstorming")`

Skills from both cipherpowers and upstream superpowers are available seamlessly.

## Structure

Each skill directory contains:
- `SKILL.md` - Main reference (required)
- Supporting files only when needed (tools, heavy reference)
