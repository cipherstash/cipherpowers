# CipherPowers

Claude Code plugin providing development workflow skills, commands, and agents for consistent team practices.

## Quick Start

CipherPowers is a plugin - install via Claude Code marketplace or local installation.

### Key Commands

- `/cipherpowers:brainstorm` - Interactive design refinement
- `/cipherpowers:plan` - Create implementation plans
- `/cipherpowers:execute` - Execute plans with agents
- `/cipherpowers:code-review` - Structured code review
- `/cipherpowers:commit` - Atomic commits with conventional format
- `/cipherpowers:verify` - Dual-verification workflows
- `/cipherpowers:summarise` - Capture learning retrospectives

### Development Commands

```bash
mise run check-has-changes   # Verify uncommitted changes
mise run check-tests-exist   # Verify tests exist
mise run check-docs-updated  # Verify docs updated
mise run check-atomic-commit # Verify atomic commit
```

## Architecture

Three-layer plugin architecture:

| Layer | Location | Purpose |
|-------|----------|---------|
| Skills | `plugin/skills/` | Reusable workflows (TDD, code review, debugging) |
| Automation | `plugin/commands/`, `plugin/agents/` | Commands dispatch to skills; agents enforce workflows |
| Standards | `plugin/standards/`, `plugin/principles/` | Project conventions and guidelines |

**Key principle:** Skills define HOW (workflows), practices define WHAT (standards), commands/agents orchestrate.

## Plugin Structure

```
plugin/
├── skills/          # Reusable workflows
├── commands/        # Slash commands
├── agents/          # Specialized subagents
├── standards/       # Project conventions
├── principles/      # Core philosophies
├── templates/       # Templates for new content
├── hooks/           # Gate configurations
└── scripts/         # Shell scripts
```

## Critical Guidelines

**For plugin development:**
- Skills follow TDD: test with subagents before writing
- Agents are thin (~30-50 lines): delegate to skills
- Use `${CLAUDE_PLUGIN_ROOT}` for all plugin references
- Commands dispatch to skills/agents, don't contain logic

**For using CipherPowers:**
- Discover skills via Skill tool: `Skill(skill: "cipherpowers:skill-name")`
- Check available commands via `/help`
- Follow skill announcements and workflows

## Environment Variables

- `CLAUDE_PLUGIN_ROOT` - Path to plugin installation (auto-set)
- `CIPHERPOWERS_MARKETPLACE_ROOT` - Optional marketplace path

## See Also

- `CLAUDE.md` - Extended Claude-specific documentation
- `plugin/docs/` - Detailed plugin documentation
- `docs/` - Project documentation (BUILD/FIX/UNDERSTAND/LOOKUP structure)
