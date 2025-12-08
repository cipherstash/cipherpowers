# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## CipherPowers

Claude Code plugin providing development workflow skills, commands, and agents for consistent team practices.

**Architecture:** Three-layer plugin separating skills (reusable workflows), automation (commands/agents), and documentation (standards/practices).

See `plugin/docs/` for detailed documentation:
- `plugin/docs/WORKFLOW.md` - Brainstorm → Plan → Execute workflow
- `plugin/docs/SKILLS.md` - Complete skills reference
- `plugin/docs/AGENTS.md` - Specialized agents reference
- `plugin/docs/COMMANDS.md` - Available commands

## Multi-Agent Compatibility

CipherPowers provides both `AGENTS.md` (universal, multi-agent compatible) and `CLAUDE.md` (Claude-specific extended documentation). For maintaining instruction files, use the `cipherpowers:maintaining-instruction-files` skill.

## Development Commands

CipherPowers uses mise for task orchestration. See `mise.toml` for available tasks.

- **Run**: N/A - This is a plugin, not a runnable application
- **check-has-changes**: `mise run check-has-changes`

The plugin is tool-agnostic and works with any build/test tooling. See `docs/configuring-project-commands.md` for details.

## Architecture Overview

### 1. Skills Layer (`plugin/skills/`)

Reusable workflows documented as testable, discoverable guides.

**Key principles:**
- Written following TDD: test with subagents before writing
- Include rich `when_to_use` frontmatter for discovery
- Follow consistent SKILL.md structure

See `plugin/docs/SKILLS.md` for complete skills reference.

### 2. Automation Layer (`plugin/commands/`, `plugin/agents/`)

Commands dispatch to skills; agents enforce workflows.

**Commands:** Thin dispatchers - `/cipherpowers:brainstorm`, `/cipherpowers:plan`, `/cipherpowers:execute`, `/cipherpowers:code-review`, `/cipherpowers:commit`, `/cipherpowers:verify`, `/cipherpowers:summarise`

**Agents:** Follow thin skill-delegation pattern (~30-50 lines each). See `plugin/docs/AGENTS.md` for complete reference.

### 3. Documentation Layer (`plugin/standards/`, `plugin/examples/`)

Standards, guidelines, and reference materials supporting skills.

## Environment Variables

**CLAUDE_PLUGIN_ROOT**: Path to plugin installation. Use for all plugin-relative paths:
```markdown
@${CLAUDE_PLUGIN_ROOT}skills/skill-name/SKILL.md
@${CLAUDE_PLUGIN_ROOT}standards/practice-name.md
```

## Directory Structure

**`./docs/`** - Project documentation (not shipped with plugin)
**`./plugin/`** - Plugin content shipped to users:
- `plugin/skills/` - Organization-specific skills
- `plugin/commands/` - Slash commands
- `plugin/agents/` - Specialized subagent prompts
- `plugin/standards/` - Project conventions and practices
- `plugin/templates/` - Templates for agents, practices, skills
- `plugin/hooks/` - Gate configuration
- `plugin/docs/` - Extended documentation

## Skills and Practices Discovery

**Skills:** Automatically discovered. Use `Skill(skill: "cipherpowers:skill-name")` in conversations.

**Practices:** Browse `plugin/standards/` directory. Each includes YAML frontmatter with `name`, `description`, `when_to_use`.

## Plugin Development

When developing CipherPowers components:

1. **Use templates:** Start from `plugin/templates/` directory
2. **Follow TDD:** Test skills with subagents before writing
3. **Reference properly:** Use `${CLAUDE_PLUGIN_ROOT}` for all paths
4. **Keep commands thin:** Dispatch to agents or skills, don't contain logic
5. **Skills auto-discover:** Once created in `plugin/skills/`, automatically available

See `plugin/templates/` for:
- `agent-template.md` - Agent structure
- `skill-template.md` - Skill structure with when_to_use frontmatter
- `practice-template.md` - Practice structure

## Quality Hooks

Gate configurations in `plugin/hooks/gates.json`. Requires turboshovel plugin for hooks runtime.

## Learning and Retrospectives

Capture lessons using `/cipherpowers:summarise`. See `plugin/skills/capturing-learning/SKILL.md` for methodology.
