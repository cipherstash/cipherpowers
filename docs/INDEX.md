# Documentation Index

Master index with purpose annotations.

## Root Files

| File | Purpose | Audience |
|------|---------|----------|
| [README.md](../README.md) | Project overview, installation | All users |
| [CLAUDE.md](../CLAUDE.md) | Claude Code guidance, architecture | Claude Code |
| [AGENTS.md](../AGENTS.md) | Universal AI instructions | All AI assistants |

## BUILD - Guides for Building Features

| File | Purpose | When to Use |
|------|---------|-------------|
| [WORKFLOW.md](BUILD/WORKFLOW.md) | Brainstorm → Plan → Execute workflow | Starting new features |
| [configuring-project-commands.md](BUILD/configuring-project-commands.md) | Tool-agnostic command setup | Setting up new projects |

## UNDERSTAND - Deep Dives

| File | Purpose | When to Use |
|------|---------|-------------|
| (architecture docs) | System design explanations | Learning how things work |

## LOOKUP - Quick References

| File | Purpose | When to Use |
|------|---------|-------------|
| [COMMANDS.md](LOOKUP/COMMANDS.md) | All slash commands | Looking up command syntax |
| [SKILLS.md](LOOKUP/SKILLS.md) | Complete skills catalog | Finding the right skill |
| [AGENTS.md](LOOKUP/AGENTS.md) | Specialized agents reference | Understanding agent roles |

## Plugin Internals

| Location | Purpose |
|----------|---------|
| `plugin/skills/` | Skill implementations (invoke via Skill tool) |
| `plugin/commands/` | Slash command definitions |
| `plugin/agents/` | Specialized agent prompts |
| `plugin/standards/` | Project conventions and practices |
| `plugin/templates/` | Templates for new components |

## Navigation by Role

**New user:**
1. [README.md](../README.md) - Overview
2. [BUILD/WORKFLOW.md](BUILD/WORKFLOW.md) - Learn the workflow

**Daily use:**
- [LOOKUP/COMMANDS.md](LOOKUP/COMMANDS.md) - Command reference
- [LOOKUP/SKILLS.md](LOOKUP/SKILLS.md) - Available skills

**Contributing:**
- [CLAUDE.md](../CLAUDE.md) - Architecture
- `plugin/templates/` - Component templates
