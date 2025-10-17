# CipherPowers

Comprehensive development toolkit for Claude Code that enforces consistent workflows across your team. CipherPowers provides reusable skills, specialized agents, and project standards through a modular plugin architecture, ensuring everyone follows the same practices for code review, testing, documentation, and planning.

## Prerequisites

Before installing CipherPowers, you need:

1. **Claude Code CLI** - Install from [claude.ai/code](https://claude.ai/code)
2. **Superpowers Plugin** - CipherPowers builds on the superpowers plugin for universal skills

### Installing Superpowers Dependency

CipherPowers requires the superpowers plugin. Install it first:

```bash
# Add superpowers plugin to your Claude Code configuration
# Follow instructions at: https://github.com/clavcode/superpowers
```

## Installation

1. **Clone CipherPowers repository:**
   ```bash
   git clone https://github.com/cipherstash/cipherpowers.git ~/.config/claude/plugins/cipherpowers
   ```

2. **Verify superpowers is installed:**
   ```bash
   # In a Claude Code session, check that superpowers commands work:
   # /brainstorm, /write-plan, /execute-plan should be available
   ```

3. **Verify CipherPowers installation:**
   ```bash
   # Start Claude Code in any project
   # Type /code-review to verify CipherPowers commands are available
   ```

## Getting Started

Once installed, CipherPowers provides specialized slash commands in Claude Code:

**Try your first command:**
```
# In any Claude Code session
/code-review
```

This will trigger a structured code review using the code-reviewer agent.

## Available Commands

### CipherPowers Commands

**Plan Execution:**
- `/execute [plan-file]` - Execute implementation plans with automatic agent selection, batch-level code review, and retrospective completion

**Code Quality:**
- `/code-review` - Manual code review trigger
- `/commit` - Commit with conventional format

**Documentation:**
- `/doc-review` - Sync documentation with code changes
- `/summarise` - Capture learning and create retrospectives

### Superpowers Commands (Required Dependency)

CipherPowers integrates with these superpowers commands:

**Planning:**
- `/brainstorm` - Refine ideas using Socratic method (superpowers)
- `/write-plan` - Create detailed implementation plans (superpowers)
- `/execute-plan` - Execute plans in batches with review checkpoints (superpowers)

## Discovery Tools

**Find Skills:**
```bash
./plugin/tools/find-skills "pattern"
./plugin/tools/find-skills --local "pattern"    # cipherpowers only
./plugin/tools/find-skills --upstream "pattern" # superpowers only
```

**Find Practices:**
```bash
./plugin/tools/find-practices "pattern"
./plugin/tools/find-practices --local "pattern"    # cipherpowers only
./plugin/tools/find-practices --upstream "pattern" # marketplace only
```

## Direct References

In agents and commands, use:
- `@${CLAUDE_PLUGIN_ROOT}plugin/practices/practice-name.md` - Direct practice reference
- `@${SUPERPOWERS_SKILLS_ROOT}/skills/category/skill-name/SKILL.md` - Skill reference
- `${CLAUDE_PLUGIN_ROOT}practices/practice-name.md` - Relative to plugin root
- `${CLAUDE_PLUGIN_ROOT}skills/category/skill-name/SKILL.md` - Relative to plugin root

## Key Features

**Algorithmic Workflow Enforcement (Oct 2025)**
- Converted TDD, code review trigger, and git commit workflows to algorithmic format
- Each includes: decision algorithm, recovery algorithm, invalid conditions, self-test
- Pressure test scenarios validate resistance to common rationalizations
- Skills: `tdd-enforcement-algorithm/`, `conducting-code-review` (trigger section)
- Practice: `git-commit-algorithm.md`
- Pattern: 0% → 100% compliance improvement under pressure (time, sunk cost, authority)

## Troubleshooting

**Commands not appearing in Claude Code:**
- Verify CipherPowers is cloned to `~/.config/claude/plugins/cipherpowers`
- Restart Claude Code session
- Check `${CLAUDE_PLUGIN_ROOT}` environment variable is set

**Superpowers commands not working:**
- Verify superpowers plugin is installed first
- Check `${SUPERPOWERS_SKILLS_ROOT}` environment variable is set
- Superpowers must be installed before CipherPowers

**Discovery tools not finding skills:**
- Ensure you're running from repository root
- Use `./plugin/tools/find-skills` with proper path
- Check both plugins are installed correctly

## Documentation

**Quick Start:** This README covers installation and basic usage.

**Deep Dive:** See `CLAUDE.md` for complete architecture details, plugin development guide, and team usage patterns. Read CLAUDE.md when you want to:
- Understand the three-layer architecture (skills, automation, documentation)
- Create custom agents, commands, or practices
- Learn about algorithmic workflow enforcement
- Contribute to CipherPowers development

## License

[Add your license here]
