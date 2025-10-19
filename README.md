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
   # /execute-plan or /superpowers:brainstorm should be available
   ```

3. **Verify CipherPowers installation:**
   ```bash
   # Start Claude Code in any project
   # Type /brainstorm or /code-review to verify CipherPowers commands are available
   ```

## Getting Started

Once installed, CipherPowers provides specialized slash commands in Claude Code:

**Try the planning workflow:**
```
# Start with an idea
/brainstorm

# Create an implementation plan
/plan

# Execute the plan with automatic agent selection
/execute [plan-file-path]
```

**Or trigger code quality checks directly:**
```
# In any Claude Code session
/code-review
```

## Recommended Workflow

For best results when implementing new features or tackling complex tasks, follow this three-step workflow:

### 1. Brainstorm (`/brainstorm`)
**When:** You have an idea but need to refine requirements and explore design options.

**What it does:**
- Interactive Socratic dialogue to clarify your thinking
- Explores edge cases, constraints, and trade-offs
- Refines vague ideas into concrete designs
- Results in a clear understanding ready for planning

**Skip if:** You already have a fully-detailed design spec.

### 2. Plan (`/plan`)
**When:** After brainstorming, when you're ready to break down the implementation.

**What it does:**
- Creates structured implementation plan with bite-sized tasks
- Each task sized for 3-task execution batches
- Includes step-by-step instructions and expected outcomes
- Saves plan to `docs/plans/` or `plans/` directory

**Skip if:** The task is trivial (single file, < 10 lines of code).

### 3. Execute (`/execute [plan-file]`)
**When:** You have a plan file and are ready to implement.

**What it does:**
- Automatically selects specialized agents for each task type
- Executes in 3-task batches with code review checkpoints
- Ensures all feedback addressed before proceeding
- Prompts for retrospective when complete

**Why this matters:**
- Prevents scope creep and forgotten requirements
- Catches issues early through batch-level code review
- Maintains consistency across large implementations
- Captures learning for future reference

**Alternative:** For simple tasks without a plan, work directly in the session and use `/code-review` and `/commit` manually.

## Available Commands

### CipherPowers Commands

**Planning Workflow:**
- `/brainstorm` - Refine ideas using Socratic method (invokes superpowers brainstorming)
- `/plan` - Create detailed implementation plans (invokes superpowers plan writing)
- `/execute [plan-file]` - Execute implementation plans with automatic agent selection, batch-level code review, and retrospective completion

**Code Quality:**
- `/code-review` - Manual code review trigger
- `/commit` - Commit with conventional format

**Documentation:**
- `/doc-review` - Sync documentation with code changes
- `/summarise` - Capture learning and create retrospectives

### Superpowers Commands (Required Dependency)

CipherPowers provides wrapper commands for superpowers workflows. The following superpowers command is also available:

**Direct Superpowers Access:**
- `/execute-plan` - Execute plans in batches with review checkpoints (superpowers base workflow, use `/execute` for enhanced CipherPowers version)

## Discovery Tools

**Skills:**

Skills are automatically discovered by Claude Code's native Skill tool. No manual discovery needed.

In conversation, Claude uses skills via:
- `Skill(command: "cipherpowers:conducting-code-review")`
- `Skill(command: "superpowers:brainstorming")`

To reference skills directly in agents/commands:
```markdown
@${CLAUDE_PLUGIN_ROOT}/skills/category/skill-name/SKILL.md
@${SUPERPOWERS_SKILLS_ROOT}/skills/category/skill-name/SKILL.md
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
