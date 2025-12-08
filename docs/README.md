# CipherPowers Documentation

All project documentation organized by developer intent.

## Quick Navigation

| I want to... | Go to |
|--------------|-------|
| **Build** something | [BUILD/](BUILD/) |
| **Understand** how things work | [UNDERSTAND/](UNDERSTAND/) |
| **Look up** a reference | [LOOKUP/](LOOKUP/) |

## Structure

```
docs/
├── BUILD/           # Guides for building features
│   ├── WORKFLOW.md  # Brainstorm → Plan → Execute
│   └── configuring-project-commands.md
├── UNDERSTAND/      # Deep dives and architecture
│   └── (architecture docs)
└── LOOKUP/          # Quick references (<30 sec lookups)
    ├── COMMANDS.md  # Command reference
    ├── SKILLS.md    # Skills catalog
    └── AGENTS.md    # Agent reference
```

## By Time Budget

**5 minutes:** Read [BUILD/WORKFLOW.md](BUILD/WORKFLOW.md) for the core workflow

**20 minutes:** Browse [LOOKUP/](LOOKUP/) for commands, skills, and agents

**1 hour:** Work through the complete workflow with a real feature

## By Role

**Plugin User** - Just want to use commands
1. [../README.md](../README.md) - Installation
2. [LOOKUP/COMMANDS.md](LOOKUP/COMMANDS.md) - Command reference

**Workflow Adopter** - Want to learn the methodology
1. [BUILD/WORKFLOW.md](BUILD/WORKFLOW.md) - Core workflow
2. [LOOKUP/SKILLS.md](LOOKUP/SKILLS.md) - Available skills

**Contributor** - Want to extend the plugin
1. [../CLAUDE.md](../CLAUDE.md) - Architecture
2. `plugin/templates/` - Component templates

**Team Lead** - Evaluating for team adoption
1. [../README.md](../README.md) - Overview
2. [BUILD/WORKFLOW.md](BUILD/WORKFLOW.md) - Workflow benefits
3. [../CLAUDE.md](../CLAUDE.md) - Architecture decisions

## See Also

- [INDEX.md](INDEX.md) - Full documentation index
- [../CLAUDE.md](../CLAUDE.md) - Architecture overview
- [../AGENTS.md](../AGENTS.md) - Universal AI instructions
