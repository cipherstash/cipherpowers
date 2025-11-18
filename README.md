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

## Setup

After cloning the CipherPowers repository, no additional setup is required - the plugin is ready to use immediately.

**Optional: Quality Hooks Configuration**

Configure project-specific quality gates for automated test/check enforcement:

```bash
# Copy example configuration (recommended)
mkdir -p .claude
cp ~/.config/claude/plugins/cipherpowers/plugin/hooks/examples/strict.json .claude/gates.json

# Customize for your project's commands
vim .claude/gates.json
```

See `plugin/hooks/SETUP.md` for detailed configuration guide.

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

## Skills and Practices

**Skills:** Automatically discovered by Claude Code. All skills in `plugin/skills/` are available via the Skill tool.

**Practices:** Browse `plugin/standards/` directory directly or reference practices using environment variables:

**Direct references in agents/commands:**
- `@${CLAUDE_PLUGIN_ROOT}plugin/standards/practice-name.md` - Direct practice reference
- `@${SUPERPOWERS_SKILLS_ROOT}/skills/category/skill-name/SKILL.md` - Upstream skill reference

## Key Features

**Quality Hooks (Nov 2025)**
- Automated quality enforcement via Claude Code's hook system
- Runs project test/check commands automatically when agents modify code
- Project-level configuration with `gates.json` (supports any build tooling)
- Configurable actions: BLOCK (enforce), CONTINUE (warn), STOP, or chain to other gates
- Two hook points: PostToolUse (after code edits), SubagentStop (when agents complete)
- See `plugin/hooks/` for setup and examples (strict, permissive, pipeline modes)

**Algorithmic Workflow Enforcement (Oct 2025)**
- Converted TDD, code review trigger, and git commit workflows to algorithmic format
- Each includes: decision algorithm, recovery algorithm, invalid conditions, self-test
- Pressure test scenarios validate resistance to common rationalizations
- Skills: `tdd-enforcement-algorithm/`, `conducting-code-review` (trigger section)
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

**Skills not available:**
- Skills are auto-discovered - no manual discovery needed
- Check `${CLAUDE_PLUGIN_ROOT}` environment variable is set
- Verify both cipherpowers and superpowers plugins are installed correctly

## Documentation

**Quick Start:** This README covers installation and basic usage.

**Quality Hooks:** See `plugin/hooks/` directory for:
- `README.md` - Overview and examples
- `SETUP.md` - Project-level configuration guide
- `INTEGRATION_TESTS.md` - Testing procedures
- `examples/` - Strict, permissive, and pipeline configurations

**Deep Dive:** See `CLAUDE.md` for complete architecture details, plugin development guide, and team usage patterns. Read CLAUDE.md when you want to:
- Understand the three-layer architecture (skills, automation, documentation)
- Create custom agents, commands, or practices
- Learn about quality hooks and algorithmic workflow enforcement
- Contribute to CipherPowers development

## License

[Add your license here]
