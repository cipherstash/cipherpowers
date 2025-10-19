# Workflow Skills

Skills for using and creating markdown-based executable workflows.

## Skills

### Executing Workflows

**File:** `executing-workflows/SKILL.md`

**When to use:** When task has existing workflow file, when algorithmic enforcement needed, when following multi-step process

**What it covers:**
- Algorithmic decision tree (Does workflow exist? Which mode? Handle results)
- Enforcement vs guided mode selection
- Command patterns and exit code handling
- Common scenarios and troubleshooting

**Target audience:** All agents (high frequency usage)

### Creating Workflows

**File:** `creating-workflows/SKILL.md`

**When to use:** When documenting multi-step process, when creating algorithmic enforcement workflow, when existing workflow needs modification

**What it covers:**
- When to create workflows vs other documentation
- Design principles (simplicity, executable documentation, enforcement vs guided)
- Complete syntax guide (steps, commands, conditionals, prompts)
- Examples by type and common patterns
- Testing workflows before deployment

**Target audience:** Developers and agents creating workflows (low frequency usage)

## Discovery

Skills are automatically discovered by Claude Code's native Skill tool.

**In conversation:**
- `Skill(command: "cipherpowers:executing-workflows")`
- `Skill(command: "cipherpowers:creating-workflows")`

**Direct references in agents/commands:**
```markdown
@${CLAUDE_PLUGIN_ROOT}/skills/workflow/executing-workflows/SKILL.md
@${CLAUDE_PLUGIN_ROOT}/skills/workflow/creating-workflows/SKILL.md
```

## Related

- **Workflow practice:** `@${CLAUDE_PLUGIN_ROOT}practices/workflow.md` - Work directory conventions
- **Workflow tool:** `@${CLAUDE_PLUGIN_ROOT}tools/workflow/` - Executor implementation
- **Git commit algorithm:** `@${CLAUDE_PLUGIN_ROOT}practices/git-commit-algorithm.md` - Example enforcement workflow
- **Algorithmic enforcement:** `@${CLAUDE_PLUGIN_ROOT}skills/meta/algorithmic-command-enforcement/SKILL.md` - Why workflows work

## Architecture

These skills follow CipherPowers three-layer architecture:

1. **Skills layer** - Process knowledge (these skills)
2. **Automation layer** - Commands/agents that invoke workflows
3. **Documentation layer** - Practices and standards (workflow.md)

**Separation of concerns:**
- Skills = How to use/create workflows (process)
- Practices = Conventions and standards (what)
- Tool = Implementation (execution)

See `CLAUDE.md` for complete architecture documentation.
