# CipherPowers Documentation Index

Master index with purpose annotations for all project documentation.

## Root Documentation

| File | Purpose | Audience |
|------|---------|----------|
| [README.md](../README.md) | Project overview, installation, quick start | All users |
| [CLAUDE.md](../CLAUDE.md) | Claude Code guidance, architecture overview | Claude Code |
| [AGENTS.md](../AGENTS.md) | Universal AI assistant instructions | All AI assistants |

## Plugin Reference (`plugin/docs/`)

Detailed documentation for CipherPowers plugin components.

| File | Purpose | When to Use |
|------|---------|-------------|
| [WORKFLOW.md](../plugin/docs/WORKFLOW.md) | Brainstorm → Plan → Execute workflow | Starting new features |
| [COMMANDS.md](../plugin/docs/COMMANDS.md) | All slash commands reference | Looking up command syntax |
| [SKILLS.md](../plugin/docs/SKILLS.md) | Complete skills catalog | Finding the right skill |
| [AGENTS.md](../plugin/docs/AGENTS.md) | Specialized agents reference | Understanding agent roles |

## Project Documentation (`docs/`)

| File | Purpose | When to Use |
|------|---------|-------------|
| [configuring-project-commands.md](configuring-project-commands.md) | Tool-agnostic command configuration | Setting up new projects |

## Plugin Internals

For plugin development and contribution:

| Location | Purpose |
|----------|---------|
| `plugin/skills/` | Skill implementations (invoke via Skill tool) |
| `plugin/commands/` | Slash command definitions |
| `plugin/agents/` | Specialized agent prompts |
| `plugin/standards/` | Project conventions and practices |
| `plugin/templates/` | Templates for new components |

## Navigation by Task

**Starting with CipherPowers:**
1. [README.md](../README.md) - Overview and installation
2. [WORKFLOW.md](../plugin/docs/WORKFLOW.md) - Learn the workflow

**Using CipherPowers daily:**
- [COMMANDS.md](../plugin/docs/COMMANDS.md) - Command reference
- [SKILLS.md](../plugin/docs/SKILLS.md) - Available skills

**Contributing to CipherPowers:**
- [CLAUDE.md](../CLAUDE.md) - Architecture and development
- `plugin/templates/` - Templates for new components
