# CipherPowers

Development toolkit for Claude Code that enforces consistent workflows across your team.

## TL;DR (5 minutes)

```bash
# Install
/plugin marketplace add cipherstash/cipherpowers
/plugin install cipherpowers@cipherpowers

# Use the workflow
/cipherpowers:brainstorm    # Refine ideas
/cipherpowers:plan          # Create plan
/cipherpowers:execute       # Execute with agents
```

**That's it.** The plugin handles agent selection, code review checkpoints, and quality gates automatically.

---

## Installation

```bash
# Add the CipherPowers marketplace
/plugin marketplace add cipherstash/cipherpowers

# Install the plugin
/plugin install cipherpowers@cipherpowers
```

**Local development:**
```bash
git clone https://github.com/cipherstash/cipherpowers.git ~/cipherpowers
/plugin marketplace add ~/cipherpowers
/plugin install cipherpowers@cipherpowers
```

## Workflow

**Brainstorm → Plan → Execute**

```
/cipherpowers:brainstorm    # Refine ideas
/cipherpowers:plan          # Create implementation plan
/cipherpowers:execute       # Execute with automatic agent selection
```

See [WORKFLOW.md](docs/BUILD/WORKFLOW.md) for detailed workflow guidance.

## Commands

### Planning
| Command | Description |
|---------|-------------|
| `/cipherpowers:brainstorm` | Refine ideas using Socratic method |
| `/cipherpowers:plan` | Create detailed implementation plans |
| `/cipherpowers:execute [plan]` | Execute plans with automatic agent selection |

### Verification
| Command | Description |
|---------|-------------|
| `/cipherpowers:verify plan` | Evaluate plan quality before execution |
| `/cipherpowers:verify execute` | Check implementation matches plan |
| `/cipherpowers:verify docs` | Find documentation issues |
| `/cipherpowers:verify research` | Verify research thoroughness |
| `/cipherpowers:verify code` | Dual code review |

### Code Quality
| Command | Description |
|---------|-------------|
| `/cipherpowers:code-review` | Trigger code review |
| `/cipherpowers:commit` | Commit with conventional format |

### Documentation
| Command | Description |
|---------|-------------|
| `/cipherpowers:summarise` | Capture learning and retrospectives |

See [COMMANDS.md](docs/LOOKUP/COMMANDS.md) for detailed command reference.

## Architecture

CipherPowers uses a three-layer architecture with clear boundaries:

| Layer | Contains | Purpose |
|-------|----------|---------|
| **Commands** | Skill reference + dispatch | Thin entry points (WHEN to invoke) |
| **Agents** | Skill reference + enforcement | Workflow enforcement (WHO executes) |
| **Skills** | Detailed workflow | Reusable process knowledge (HOW to do it) |

**Key principle:** Commands and agents are thin - they reference skills for the detailed workflow. This enables:
- Skills reusable across commands/agents
- Single source of truth for workflows
- Easy updates without touching multiple files

See [CLAUDE.md](CLAUDE.md) for full architecture documentation.

## Reference

- [COMMANDS.md](docs/LOOKUP/COMMANDS.md) - Command details and usage
- [AGENTS.md](docs/LOOKUP/AGENTS.md) - Specialized agent reference
- [SKILLS.md](docs/LOOKUP/SKILLS.md) - Available skills reference
- [WORKFLOW.md](docs/BUILD/WORKFLOW.md) - Detailed workflow guidance
- [CLAUDE.md](CLAUDE.md) - Architecture and plugin development
- [docs/](docs/) - Full documentation index

## Troubleshooting

**Commands not appearing:** Run `/plugin list` to verify installation. Reinstall if needed.

**Commands fail to load files:** Uninstall and reinstall using marketplace method (not direct clone).

**Skills not available:** Skills are auto-discovered. Check plugin installation and `${CLAUDE_PLUGIN_ROOT}` value.

## Acknowledgements

CipherPowers was originally inspired by and built upon the [Superpowers plugin](https://github.com/clavcode/superpowers).

## License

See [LICENSE.md](LICENSE.md)
