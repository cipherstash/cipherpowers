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

## Debugging Hooks

If hooks aren't working as expected or agents seem stuck, debug logging is enabled by default.

**View logs:**

```bash
# Logs are written to: $TMPDIR/cipherpowers-hooks-YYYYMMDD.log
# On macOS: /var/folders/.../T/cipherpowers-hooks-YYYYMMDD.log
# On Linux: /tmp/cipherpowers-hooks-YYYYMMDD.log

# Watch logs in real-time
tail -f $TMPDIR/cipherpowers-hooks-$(date +%Y%m%d).log

# Or on macOS specifically
tail -f /var/folders/*/T/cipherpowers-hooks-$(date +%Y%m%d).log
```

**What gets logged:**
- Hook event type (PostToolUse, SubagentStop, UserPromptSubmit)
- Which gates.json config file is being used
- Which gates are running
- Gate command execution and exit codes
- Action handling (CONTINUE, BLOCK, STOP, chaining)
- Full flow from dispatcher → gate → action

**Common issues revealed by logging:**
- Gates not running: Check if tool/agent is in `enabled_tools`/`enabled_agents` list
- Wrong config: Check which gates.json path is being loaded
- Gate command failures: Check command output and exit codes
- Blocking behavior: See which action is being taken (BLOCK vs CONTINUE vs STOP)

**Disable logging (optional):**

```bash
export CIPHERPOWERS_HOOK_DEBUG=false
```

## Overview

Quality hooks integrate with Claude Code's event system to automatically run quality gates (test, check, build commands) at strategic points:

- **PostToolUse**: Runs after code modifications (Edit, Write tools)
- **SubagentStop**: Runs when specialized agents complete their work
- **UserPromptSubmit**: Runs when user submits a prompt (e.g., inject project commands)

**Configuration is project-specific** - each project has its own `gates.json` with commands tailored to that project's tooling.

## Components

### 1. Hook Registration (`hooks.json`)

Registers hook scripts with Claude Code. All hooks use the universal dispatcher:

```json
{
  "hooks": {
    "PostToolUse": [{
      "matcher": ".*",
      "hooks": [{"type": "command", "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher.sh"}]
    }],
    "SubagentStop": [{
      "matcher": ".*",
      "hooks": [{"type": "command", "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher.sh"}]
    }],
    "UserPromptSubmit": [{
      "matcher": ".*",
      "hooks": [{"type": "command", "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher.sh"}]
    }]
  }
}
```

The dispatcher routes hook events to configured gates based on `gates.json`.

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

- **dispatcher.sh**: Universal hook dispatcher - routes all hook events to configured gates
- **session-start.sh**: Special hook for session initialization (injects skills context)
- **shared-functions.sh**: Common gate execution logic (`run_gate`, `handle_action`)

### 4. Gate Scripts (`gates/`)

Gates can be either shell commands or custom scripts in `plugin/hooks/gates/`:

- **gates/commands.sh**: Built-in gate that **always runs first** on UserPromptSubmit events to inject project commands from CLAUDE.md frontmatter

**Special behavior for UserPromptSubmit:**
- `commands.sh` runs automatically before any configured gates
- Projects don't need to configure it in `gates.json`
- Additional gates can optionally be configured to run after commands injection

Custom gate scripts receive hook context via `HOOK_INPUT` environment variable and output JSON.

## Gate Actions

Gates support four action types:

- **CONTINUE**: Proceed (default on pass)
- **BLOCK**: Prevent agent from proceeding (default on fail)
- **STOP**: Stop Claude entirely
- **{gate_name}**: Chain to another gate (subroutine call)

## Creating Custom Gate Scripts

Gate scripts are executables in `plugin/hooks/gates/` that receive hook context and output JSON.

**Input:** `HOOK_INPUT` environment variable (JSON with hook event data)

**Output:** JSON with one of:
- `{"additionalContext": "..."}` - Add context to conversation
- `{"decision": "block", "reason": "..."}` - Block execution
- `{"continue": false, "message": "..."}` - Stop Claude

**Example:** `plugin/hooks/gates/commands.sh` injects project commands from CLAUDE.md:

```bash
#!/bin/bash
set -euo pipefail

# Parse hook input
USER_MESSAGE=$(echo "$HOOK_INPUT" | jq -r '.user_message // ""')

# Your logic here...

# Output JSON
jq -n --arg content "$result" '{
  additionalContext: $content
}'
```

**Register in gates.json (optional - for running after built-in commands.sh):**
```json
{
  "gates": {
    "my-custom-gate": {
      "description": "My custom gate",
      "command": "${CLAUDE_PLUGIN_ROOT}/hooks/gates/my-custom-gate.sh",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    }
  },
  "hooks": {
    "UserPromptSubmit": {
      "enabled": true,
      "gates": ["my-custom-gate"]
    }
  }
}
```

**Note:** `commands.sh` still runs first automatically, then `my-custom-gate` runs.

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
echo '{"hook_event": "PostToolUse", "tool_name": "Edit", "cwd": "/test"}' | plugin/hooks/dispatcher.sh

# SubagentStop
echo '{"hook_event": "SubagentStop", "agent_name": "rust-engineer", "cwd": "/test"}' | plugin/hooks/dispatcher.sh

# UserPromptSubmit
echo '{"hook_event": "UserPromptSubmit", "user_message": "run tests", "cwd": "/test"}' | plugin/hooks/dispatcher.sh
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
