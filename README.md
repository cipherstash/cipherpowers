# CipherPowers

A Claude Code plugin providing comprehensive development workflows, skills, and documentation standards for teams.

## Overview

CipherPowers is a "batteries included" starter kit for development teams using Claude Code. It provides:

- **Skills**: Reusable process knowledge for workflows, planning, and code quality
- **Commands**: Slash commands for common development tasks
- **Agents**: Specialized subagents for planning, review, and execution
- **Documentation**: Standards, guidelines, and best practices

## Installation

### Prerequisites

1. Install [Claude Code](https://claude.com/claude-code)
2. Install [superpowers](https://github.com/obra/superpowers-skills) for universal skills

### Install CipherPowers

```bash
# Clone the repository
git clone <repository-url> cipherpowers

# Install as Claude Code plugin
# (Follow Claude Code plugin installation instructions)
```

## Structure

```
cipherpowers/
├── .claude-plugin/
│   └── plugin.json        # Plugin metadata
├── commands/              # Slash commands
│   ├── plan.md           # Work planning workflow
│   ├── work.md           # Active work management
│   └── ...
├── agents/                # Subagent prompts
│   ├── work-planner.md   # Planning specialist
│   ├── code-reviewer.md  # Review specialist
│   └── ...
├── skills/                # Org-specific skills
│   ├── planning-work/
│   ├── linear-workflow/
│   └── ...
├── tools/
│   └── find-skills        # Unified skill discovery
├── docs/
│   ├── practices/         # Coding standards & guidelines
│   ├── examples/          # Templates & examples
│   └── ...
└── README.md
```

## Usage

### Commands

Slash commands provide quick access to common workflows:

```
/plan           # Create detailed implementation plan
/work           # Complete workflow for implementing planned work
/code-review    # Request code review
/commit         # Create structured commit
/execute        # Execute an active work plan
```

### Skills

Skills are discoverable using the custom find-skills tool:

```bash
# Search all skills (cipherpowers + superpowers)
find-skills "planning"

# Search only local skills
find-skills --local "workflow"

# Search only superpowers
find-skills --upstream "testing"
```

### Documentation

Reference materials in `docs/`:

- `practices/` - Coding standards, git conventions, testing guidelines
- `examples/` - Templates and real-world examples

## Architecture

### Three-Layer Design

**1. Skills Layer** (`skills/`)

Reusable process knowledge following the superpowers framework:
- Testable with subagents (TDD for documentation)
- Rich metadata for discovery
- References to upstream superpowers skills

**2. Automation Layer** (`commands/`, `agents/`)

Workflow automation:
- Commands dispatch to agents
- Agents follow skills for methodology
- Project-specific orchestration

**3. Documentation Layer** (`docs/`)

Supporting materials:
- Standards and guidelines
- Templates and examples
- Team reference documentation

### Integration with Superpowers

CipherPowers extends superpowers with organization-specific content:

- Custom `find-skills` searches both collections
- Commands reference skills from either location
- Skills can be developed locally, then contributed upstream

## Creating Skills

When adding new skills to `skills/`:

1. **Check superpowers first** - Don't duplicate existing skills
2. **Follow TDD** - Test with subagents before writing
3. **Read the meta-skill** - `${SUPERPOWERS_SKILLS_ROOT}/skills/meta/writing-skills/SKILL.md`
4. **Use TodoWrite** - Track the skill creation checklist
5. **Consider upstream** - Universal skills can be contributed back

## Contributing

### Upstreaming Skills

Universal skills developed in cipherpowers can be contributed to superpowers:

1. Test thoroughly with subagents
2. Ensure it's broadly applicable (not org-specific)
3. Follow superpowers contribution guidelines
4. Submit PR to [superpowers-skills](https://github.com/obra/superpowers-skills)

### Local Development

Organization-specific skills stay in `skills/`:
- Linear workflow integration
- Team-specific conventions
- Internal tooling guidance

## Team Customization

Fork cipherpowers to create your own team toolkit:

1. Fork this repository
2. Customize commands and agents for your workflow
3. Add org-specific skills
4. Update practices documentation
5. Share with team

## License

[Add your license here]

## Credits

Built on [superpowers](https://github.com/obra/superpowers-skills) by @obra
