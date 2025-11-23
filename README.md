# CipherPowers

Comprehensive development toolkit for Claude Code that enforces consistent workflows across your team. CipherPowers provides reusable skills, specialized agents, and project standards through a modular plugin architecture, ensuring everyone follows the same practices for code review, testing, documentation, and planning.

## Acknowledgements

CipherPowers was originally inspired by and built upon the [Superpowers plugin](https://github.com/clavcode/superpowers). While CipherPowers is now a standalone plugin, we're grateful for the foundational concepts and patterns that Superpowers introduced to the Claude Code ecosystem.

## Prerequisites

Before installing CipherPowers, you need:

1. **Claude Code CLI** - Install from [claude.ai/code](https://claude.ai/code)

## Installation

CipherPowers uses Claude Code's marketplace system for plugin installation.

### Option 1: Install from GitHub (Recommended for Users)

```bash
# In Claude Code, add the CipherPowers marketplace
/plugin marketplace add cipherstash/cipherpowers

# Install the plugin
/plugin install cipherpowers@cipherpowers
```

### Option 2: Local Development Installation

For plugin development or testing local changes:

```bash
# Clone repository to any location
git clone https://github.com/cipherstash/cipherpowers.git ~/cipherpowers

# In Claude Code, add as local marketplace
/plugin marketplace add ~/cipherpowers

# Install the plugin
/plugin install cipherpowers@cipherpowers
```

### Verify Installation

Commands should appear immediately after installation. Test with:
```
/brainstorm
/code-review
```

If commands don't appear, restart your Claude Code session.

## Setup

### Optional: Quality Hooks Configuration

Configure project-specific quality gates for automated test/check enforcement:

**Step 1: Copy example configuration**

The plugin provides example configurations you can copy:

```bash
# Copy example configuration (recommended)
mkdir -p .claude
cp ${CLAUDE_PLUGIN_ROOT}/hooks/examples/strict.json .claude/gates.json

# Customize for your project's commands
vim .claude/gates.json
```

**Note:** `${CLAUDE_PLUGIN_ROOT}` is automatically set by Claude Code when the plugin loads.

**Alternatively, create configuration manually:**

```bash
mkdir -p .claude
cat > .claude/gates.json << 'EOF'
{
  "gates": {
    "quality-check": {
      "commands": ["npm test", "npm run lint"],
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "PostToolUse": ["quality-check"],
    "SubagentStop": ["quality-check"]
  }
}
EOF
```

See `plugin/hooks/SETUP.md` in the installed plugin for detailed configuration guide.

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
- Saves implementation plan to `.work/` directory

**Note:** Design documents from `/brainstorm` are saved to `docs/plans/`

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
- `/brainstorm` - Refine ideas using Socratic method
- `/plan` - Create detailed implementation plans
- `/plan-review` - Evaluate implementation plans before execution
- `/execute [plan-file]` - Execute implementation plans with automatic agent selection, batch-level code review, and retrospective completion

**Code Quality:**
- `/code-review` - Manual code review trigger
- `/commit` - Commit with conventional format

**Documentation:**
- `/doc-review` - Sync documentation with code changes
- `/summarise` - Capture learning and create retrospectives

## Skills and Practices

**Skills:** Automatically discovered by Claude Code. All skills in `plugin/skills/` are available via the Skill tool.

**Practices:** Browse `plugin/standards/` directory directly or reference practices using environment variables:

**Direct references in agents/commands:**
- `@${CLAUDE_PLUGIN_ROOT}standards/practice-name.md` - Direct practice reference

## Key Features

**Quality Hooks (November 2025)**
- Automated quality enforcement via Claude Code's hook system
- Runs project test/check commands automatically when agents modify code
- Project-level configuration with `gates.json` (supports any build tooling)
- Configurable actions: BLOCK (enforce), CONTINUE (warn), STOP, or chain to other gates
- Two hook points: PostToolUse (after code edits), SubagentStop (when agents complete)
- Multiple example configurations: strict, permissive, pipeline, convention-based, TypeScript-specific, plan execution
- Ready-to-use context injection examples for code review, planning, and TDD

**Algorithmic Workflow Enforcement (October 2025)**
- Converted TDD, code review trigger, and git commit workflows to algorithmic format
- Each includes: decision algorithm, recovery algorithm, invalid conditions, self-test
- Pressure test scenarios validate resistance to common rationalizations
- Skills: `tdd-enforcement-algorithm/`, `conducting-code-review` (trigger section)
- Pattern: 0% → 100% compliance improvement under pressure (time, sunk cost, authority)

## Troubleshooting

### Commands not appearing in Claude Code

1. **Verify plugin installation:**
   ```
   /plugin list
   ```
   CipherPowers should appear in the installed plugins list.

2. **Reinstall if necessary:**
   ```
   /plugin uninstall cipherpowers
   /plugin install cipherpowers@cipherpowers
   ```

3. **Restart Claude Code session** if commands still don't appear.

### Plugin installed but commands fail to load files

This usually means you installed using the old direct-clone method instead of the marketplace system.

**Solution:** Uninstall and reinstall using marketplace method:
```bash
# Remove old installation if you used direct clone
rm -rf ~/.config/claude/plugins/cipherpowers  # or ~/.claude/plugins/cipherpowers

# In Claude Code, install via marketplace
/plugin marketplace add cipherstash/cipherpowers
/plugin install cipherpowers@cipherpowers
```

### Skills not available

- Skills are auto-discovered - no manual discovery needed
- Verify plugin is installed: `/plugin list`
- Check that `${CLAUDE_PLUGIN_ROOT}` is set correctly (ask Claude Code to show the value)

### Different Claude config directory location

Claude Code may use different config directories:
- `~/.claude/` (some installations)
- `~/.config/claude/` (other installations)

The marketplace-based installation handles this automatically. If you're having issues, ensure you're using the marketplace installation method (not direct cloning).

## Documentation

**Quick Start:** This README covers installation and basic usage.

**Quality Hooks:** See `plugin/hooks/` directory for:
- `README.md` - Overview and examples
- `SETUP.md` - Project-level configuration guide
- `CONVENTIONS.md` - Convention-based context injection
- `INTEGRATION_TESTS.md` - Testing procedures
- `examples/` - Six gate configurations: strict.json, permissive.json, pipeline.json, convention-based.json, typescript-gates.json, plan-execution.json
- `examples/context/` - Ready-to-use context injection files for code review, planning, and TDD

**Deep Dive:** See `CLAUDE.md` (auto-loaded by Claude Code and serves as reference documentation) for complete architecture details, plugin development guide, and team usage patterns. Read CLAUDE.md when you want to:
- Understand the three-layer architecture (skills, automation, documentation)
- Create custom agents, commands, or practices
- Learn about quality hooks and algorithmic workflow enforcement
- Contribute to CipherPowers development

## License

See [LICENSE.md](LICENSE.md)
