# Quality Hooks

Automated quality enforcement via Claude Code's hook system. Runs project test and check commands automatically when agents modify code or complete work.

## Quick Start

**1. Create project-level configuration:**

```bash
# Recommended: .claude/gates.json
mkdir -p .claude
cp ${CLAUDE_PLUGIN_ROOT}/hooks/examples/strict.json .claude/gates.json

# Customize commands for your project
vim .claude/gates.json
```

**2. Update commands to match your tooling:**

```json
{
  "gates": {
    "check": {"command": "npm run lint"},  // ← Your command
    "test": {"command": "npm test"}        // ← Your command
  }
}
```

See **[SETUP.md](./SETUP.md)** for detailed configuration guide.

## Overview

Quality hooks integrate with Claude Code's event system to automatically run quality gates (test, check, build commands) at strategic points:

- **PostToolUse**: Runs after code modifications (Edit, Write tools)
- **SubagentStop**: Runs when specialized agents complete their work

**Configuration is project-specific** - each project has its own `gates.json` with commands tailored to that project's tooling.

## Components

### 1. Hook Registration (`hooks.json`)

Registers hook scripts with Claude Code:

```json
{
  "hooks": {
    "PostToolUse": [{
      "matcher": ".*",
      "hooks": [{"type": "command", "command": "${CLAUDE_PLUGIN_ROOT}/hooks/post-tool-use.sh"}]
    }],
    "SubagentStop": [{
      "matcher": ".*",
      "hooks": [{"type": "command", "command": "${CLAUDE_PLUGIN_ROOT}/hooks/subagent-stop.sh"}]
    }]
  }
}
```

### 2. Gate Configuration (`gates.json`)

**Project-level configuration** - hooks search for `gates.json` in this order:

1. `.claude/gates.json` (recommended - project-specific)
2. `gates.json` (project root)
3. `${CLAUDE_PLUGIN_ROOT}/hooks/gates.json` (plugin default fallback)

Self-contained configuration defining gates, commands, and actions:

```json
{
  "gates": {
    "check": {
      "description": "Run quality checks",
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write"],
      "gates": ["check"]
    },
    "SubagentStop": {
      "enabled_agents": ["rust-engineer"],
      "gates": ["check", "test"]
    }
  }
}
```

### 3. Hook Scripts

- **post-tool-use.sh**: Runs gates after code editing tools
- **subagent-stop.sh**: Runs gates when specialized agents complete
- **shared-functions.sh**: Common gate execution logic

## Gate Actions

Gates support four action types:

- **CONTINUE**: Proceed (default on pass)
- **BLOCK**: Prevent agent from proceeding (default on fail)
- **STOP**: Stop Claude entirely
- **{gate_name}**: Chain to another gate (subroutine call)

## Gate Chaining

Actions can reference other gates, creating complex workflows:

```json
{
  "gates": {
    "format": {
      "command": "mise run format",
      "on_pass": "check"
    },
    "check": {
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

When format passes → calls check gate → if check passes → continues

## Configuration

### Enable/Disable for Tools

```json
{
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write", "mcp__serena__replace_symbol_body"],
      "gates": ["check"]
    }
  }
}
```

### Enable/Disable for Agents

```json
{
  "hooks": {
    "SubagentStop": {
      "enabled_agents": ["rust-engineer", "code-reviewer"],
      "gates": ["check", "test"]
    }
  }
}
```

### Per-Gate Actions

```json
{
  "gates": {
    "check": {
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "command": "mise run test",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    }
  }
}
```

## Example Configurations

### Strict Mode

Block on all failures:

```json
{
  "gates": {
    "check": {"command": "mise run check", "on_fail": "BLOCK"},
    "test": {"command": "mise run test", "on_fail": "BLOCK"}
  }
}
```

### Permissive Mode

Warn only:

```json
{
  "gates": {
    "check": {"command": "mise run check", "on_fail": "CONTINUE"},
    "test": {"command": "mise run test", "on_fail": "CONTINUE"}
  }
}
```

### Gate Pipeline

Chain gates sequentially:

```json
{
  "gates": {
    "format": {"command": "mise run format", "on_pass": "check"},
    "check": {"command": "mise run check", "on_pass": "test"},
    "test": {"command": "mise run test"}
  },
  "hooks": {
    "SubagentStop": {"gates": ["format"]}
  }
}
```

## Testing

Test gates manually:

```bash
# Source functions
source plugin/hooks/shared-functions.sh

# Test gate execution
run_gate "check" "plugin/hooks/gates.json"
echo "Exit code: $?"

# Test with test configuration
run_gate "always-pass" "plugin/hooks/gates.test.json"
```

Test hooks with mock input:

```bash
# PostToolUse
echo '{"tool_name": "Edit", "cwd": "/test"}' | plugin/hooks/post-tool-use.sh

# SubagentStop
echo '{"agent_name": "rust-engineer", "cwd": "/test"}' | plugin/hooks/subagent-stop.sh
```

## Error Handling

- **Missing gate**: STOP with error message
- **Missing command**: STOP with error message
- **Invalid action**: STOP with error message
- **Gate failure with BLOCK**: Agent cannot proceed
- **Gate failure with STOP**: Claude halts entirely
- **Gate failure with CONTINUE**: Warning message, execution continues

## Benefits

1. **Consistent Quality**: All agent work validated automatically
2. **Early Detection**: Issues caught at edit time or completion
3. **Flexible Enforcement**: Configure actions per gate
4. **Self-Contained**: All configuration in gates.json
5. **Tool-Agnostic**: Works with any CLI tool
6. **Composable**: Build complex workflows via chaining
