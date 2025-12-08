# Project Documentation

This directory contains documentation **about the CipherPowers project itself** - not the plugin that ships to users.

## What's Here

| File | Purpose |
|------|---------|
| [INDEX.md](INDEX.md) | Master documentation index with purpose annotations |
| [configuring-project-commands.md](configuring-project-commands.md) | Guide for tool-agnostic command configuration |

## What's NOT Here

**Plugin documentation** lives in `plugin/docs/`:
- [WORKFLOW.md](../plugin/docs/WORKFLOW.md) - Core workflow guide
- [COMMANDS.md](../plugin/docs/COMMANDS.md) - Command reference
- [SKILLS.md](../plugin/docs/SKILLS.md) - Skills catalog
- [AGENTS.md](../plugin/docs/AGENTS.md) - Agent reference

## docs/ vs plugin/docs/

| Directory | Ships with Plugin | Purpose |
|-----------|-------------------|---------|
| `docs/` | No | Project development documentation |
| `plugin/docs/` | Yes | User-facing reference documentation |

## For End-User Projects

If you're using CipherPowers in your own project and want to organize documentation:

1. Use the `cipherpowers:organizing-documentation` skill
2. Follow the BUILD/FIX/UNDERSTAND/LOOKUP structure
3. See `plugin/skills/organizing-documentation/SKILL.md` for the complete guide

## See Also

- [INDEX.md](INDEX.md) - Full documentation index
- [../CLAUDE.md](../CLAUDE.md) - Architecture overview
- [../plugin/docs/README.md](../plugin/docs/README.md) - Plugin documentation hub
