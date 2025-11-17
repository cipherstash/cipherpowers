# Quality Gates Design

**Date**: 2025-11-17
**Status**: Approved
**Author**: Design Session

## Overview

Automated quality enforcement via Claude Code's hook system. Runs project test and check commands automatically when agents modify code or complete work.

## Requirements

- **Event-specific triggers**: Different gates at different hook points
- **Action-based handling**: Configurable actions on pass/fail (CONTINUE, BLOCK, STOP, or custom commands)
- **Project-level config**: Configurable which agents use hooks
- **Tool-agnostic**: Uses canonical command vocabulary (test, check, build, run)
- **Integration**: Extends Claude Code's existing hook system

## Architecture

### Hook Registration

**File**: `plugin/hooks/hooks.json`

Registers quality gate hooks with Claude Code:

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": ".*",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/post-tool-use.sh"
          }
        ]
      }
    ],
    "SubagentStop": [
      {
        "matcher": ".*",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/subagent-stop.sh"
          }
        ]
      }
    ]
  }
}
```

### Quality Gates Configuration

**File**: `plugin/hooks/gates.json`

Defines what gates run when:

```json
{
  "gates": {
    "check": {
      "description": "Run project quality checks (formatting, linting, types)",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "description": "Run project test suite"
    },
    "build": {
      "description": "Run project build",
      "on_fail": "CONTINUE"
    },
    "reticulate": {
      "description": "Reticulate splines",
      "on_fail": "./spline_reticulation_failure_handler.sh"
    }
  },
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write", "mcp__serena__replace_symbol_body"],
      "gates": ["check"]
    },
    "SubagentStop": {
      "enabled_agents": ["rust-engineer", "code-reviewer", "ultrathink-debugger"],
      "gates": ["check", "test"]
    }
  }
}
```

#### Gate Definition

A gate is a command with optional on_pass and on_fail actions:

- **command**: Resolved from CLAUDE.md frontmatter (e.g., "check" → "cargo clippy && cargo check")
- **on_pass**: Action when gate passes (default: "CONTINUE")
- **on_fail**: Action when gate fails (default: "BLOCK")

#### Actions

- **CONTINUE**: Hook returns success, execution continues normally
- **BLOCK**: Hook returns `decision: "block"`, prevents agent from proceeding
- **STOP**: Hook returns `continue: false`, stops Claude entirely
- **{command}**: Execute shell command (e.g., `./fix-formatting.sh`)

#### Default Behavior

If `on_pass` or `on_fail` are omitted:
- `on_pass`: "CONTINUE"
- `on_fail`: "BLOCK"

### Hook Script Implementation

**PostToolUse Hook** (`plugin/hooks/post-tool-use.sh`):

```bash
#!/usr/bin/env bash
set -euo pipefail

# Source shared functions
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/shared-functions.sh"

# Parse input
INPUT=$(cat)
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name')
CWD=$(echo "$INPUT" | jq -r '.cwd')

# Load gates config
CONFIG="${CLAUDE_PLUGIN_ROOT}/hooks/gates.json"
ENABLED_TOOLS=$(jq -r '.hooks.PostToolUse.enabled_tools[]' "$CONFIG")

# Check if this tool should trigger gates
if ! echo "$ENABLED_TOOLS" | grep -q "$TOOL_NAME"; then
  exit 0  # No output = continue normally
fi

# Get gates to run
GATES=$(jq -r '.hooks.PostToolUse.gates[]' "$CONFIG")

# Run each gate
for gate in $GATES; do
  run_gate "$gate" "$CONFIG"
done
```

**SubagentStop Hook** (`plugin/hooks/subagent-stop.sh`):

```bash
#!/usr/bin/env bash
set -euo pipefail

# Source shared functions
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/shared-functions.sh"

# Parse input
INPUT=$(cat)
AGENT_NAME=$(echo "$INPUT" | jq -r '.agent_name // .subagent_name // "unknown"')
CWD=$(echo "$INPUT" | jq -r '.cwd')

# Load gates config
CONFIG="${CLAUDE_PLUGIN_ROOT}/hooks/gates.json"
ENABLED_AGENTS=$(jq -r '.hooks.SubagentStop.enabled_agents[]' "$CONFIG")

# Check if this agent should trigger gates
if ! echo "$ENABLED_AGENTS" | grep -q "$AGENT_NAME"; then
  exit 0  # No output = continue normally
fi

# Get gates to run
GATES=$(jq -r '.hooks.SubagentStop.gates[]' "$CONFIG")

# Run each gate
for gate in $GATES; do
  run_gate "$gate" "$CONFIG"
done
```

**Shared Functions** (`plugin/hooks/shared-functions.sh`):

```bash
#!/usr/bin/env bash

# Extract command from CLAUDE.md frontmatter
get_command() {
  local cmd_type="$1"

  awk -v cmd="$cmd_type" '
    BEGIN { in_frontmatter=0; in_commands=0 }
    /^---$/ { in_frontmatter = !in_frontmatter; next }
    in_frontmatter && /^commands:/ { in_commands=1; next }
    in_frontmatter && in_commands && /^[^ ]/ { in_commands=0 }
    in_frontmatter && in_commands && $0 ~ "^  " cmd ":" {
      gsub(/.*: "/, "")
      gsub(/".*/, "")
      print
      exit
    }
  ' CLAUDE.md
}

# Execute a gate and handle its result
run_gate() {
  local gate_name="$1"
  local config="$2"

  # Get gate command from CLAUDE.md
  local gate_cmd=$(get_command "$gate_name")

  if [ -z "$gate_cmd" ]; then
    # Gate command not found in CLAUDE.md, skip
    return 0
  fi

  # Get gate configuration
  local on_pass=$(jq -r ".gates.$gate_name.on_pass // \"CONTINUE\"" "$config")
  local on_fail=$(jq -r ".gates.$gate_name.on_fail // \"BLOCK\"" "$config")
  local description=$(jq -r ".gates.$gate_name.description // \"\"" "$config")

  # Run the gate
  local output
  local exit_code=0
  output=$(eval "$gate_cmd" 2>&1) || exit_code=$?

  if [ $exit_code -eq 0 ]; then
    # Gate passed
    handle_action "$on_pass" "$gate_name" "passed" "$output"
  else
    # Gate failed
    handle_action "$on_fail" "$gate_name" "failed" "$output"
  fi
}

# Handle gate action (CONTINUE, BLOCK, STOP, or custom command)
handle_action() {
  local action="$1"
  local gate_name="$2"
  local status="$3"
  local output="$4"

  case "$action" in
    CONTINUE)
      # Continue normally, optionally add context
      if [ "$status" = "failed" ]; then
        jq -n --arg msg "⚠️ Gate '$gate_name' failed but continuing:\n$output" '{
          additionalContext: $msg
        }'
      fi
      ;;

    BLOCK)
      # Block with reason
      jq -n --arg reason "Gate '$gate_name' $status. Output:\n$output" '{
        decision: "block",
        reason: $reason
      }'
      ;;

    STOP)
      # Stop Claude entirely
      jq -n --arg msg "Gate '$gate_name' $status. Stopping Claude.\n$output" '{
        continue: false,
        message: $msg
      }'
      ;;

    *)
      # Custom command - execute it
      if [ -f "$action" ] || command -v "$action" &> /dev/null; then
        local custom_output
        custom_output=$(eval "$action" 2>&1) || true
        jq -n --arg msg "Gate '$gate_name' $status. Ran custom action '$action':\n$custom_output" '{
          additionalContext: $msg
        }'
      else
        # Command not found, treat as CONTINUE with warning
        jq -n --arg msg "⚠️ Gate '$gate_name' $status. Custom action '$action' not found." '{
          additionalContext: $msg
        }'
      fi
      ;;
  esac
}
```

### Command Discovery Pattern

Reuses existing `get_command()` function pattern from `user-prompt-submit.sh`:

```bash
# Extract command from CLAUDE.md frontmatter
get_command() {
  local cmd_type="$1"

  awk -v cmd="$cmd_type" '
    BEGIN { in_frontmatter=0; in_commands=0 }
    /^---$/ { in_frontmatter = !in_frontmatter; next }
    in_frontmatter && /^commands:/ { in_commands=1; next }
    in_frontmatter && in_commands && /^[^ ]/ { in_commands=0 }
    in_frontmatter && in_commands && $0 ~ "^  " cmd ":" {
      gsub(/.*: "/, "")
      gsub(/".*/, "")
      print
      exit
    }
  ' CLAUDE.md
}
```

## Error Handling

### Action-Based Behavior

**CONTINUE**:
- Hook outputs optional additionalContext with information
- Agent sees message but continues normally
- Example: Non-critical warnings

**BLOCK**:
- Hook outputs `decision: "block"` with reason
- Agent cannot proceed until issue is fixed
- Example: Test failures that must be resolved

**STOP**:
- Hook outputs `continue: false`
- Stops Claude entirely (equivalent to Ctrl+C)
- Example: Critical system failures

**Custom Command**:
- Hook executes specified shell command
- Outputs additionalContext with command result
- Agent sees output and continues
- Example: Auto-formatting fixes

### Hook Output Patterns

1. **CONTINUE (success)** - No output, hook exits 0
   ```bash
   # Gate passed, continue normally
   exit 0
   ```

2. **CONTINUE (failure with warning)**
   ```json
   {
     "additionalContext": "⚠️ Gate 'check' failed but continuing:\n<output>"
   }
   ```

3. **BLOCK**
   ```json
   {
     "decision": "block",
     "reason": "Gate 'test' failed. Output:\n<output>"
   }
   ```

4. **STOP**
   ```json
   {
     "continue": false,
     "message": "Gate 'check' failed. Stopping Claude.\n<output>"
   }
   ```

5. **Custom Command**
   ```json
   {
     "additionalContext": "Gate 'check' failed. Ran custom action './fix-formatting.sh':\n<output>"
   }
   ```

### Failure Recovery

- Agent sees error output in context
- Can fix issues and try again
- Hook re-runs automatically on next completion attempt
- Provides clear feedback about what failed and why
- Custom commands can auto-fix certain issues (e.g., formatting)

### Hook Script Safety

- Scripts use `set -euo pipefail` for safety
- Malformed JSON = hook fails safely (no block)
- Missing config = hook exits cleanly
- Command not found in CLAUDE.md = skip that gate
- Custom command not found = treat as CONTINUE with warning

## Agent Integration

Agents are made aware of quality gates via their prompt references:

```markdown
# Agent Prompt
You enforce strict quality standards.

Quality gates are configured in ${CLAUDE_PLUGIN_ROOT}/hooks/gates.json

When you complete work:
- SubagentStop hook will run project gates (check, test, etc.)
- Gate failures may BLOCK, STOP, or run custom recovery commands
- You'll see results in additionalContext and must respond appropriately
```

## Project Setup

Projects opt into quality gates by:

1. **Adding hooks.json** to their plugin:
   ```
   plugin/hooks/hooks.json (registers PostToolUse, SubagentStop)
   plugin/hooks/gates.json (configures what runs when)
   ```

2. **Defining commands in CLAUDE.md frontmatter**:
   ```yaml
   ---
   commands:
     test: "cargo test"
     check: "cargo clippy && cargo check"
     build: "cargo build"
   ---
   ```

3. **Implementing hook scripts**:
   ```
   plugin/hooks/post-tool-use.sh
   plugin/hooks/subagent-stop.sh
   plugin/hooks/shared-functions.sh
   ```

## Customization

### Configuration Points

- **Which agents use hooks**: Edit `enabled_agents` in gates.json
- **Which tools trigger hooks**: Edit `enabled_tools` in gates.json
- **What gates run when**: Edit `gates` array per hook event
- **Pass/fail behavior**: Set `on_pass`/`on_fail` actions per gate
- **Project commands**: Update CLAUDE.md frontmatter

### Example Configurations

**Strict mode** (block on all failures):
```json
{
  "gates": {
    "check": {
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

**Permissive mode** (warn only):
```json
{
  "gates": {
    "check": {
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    },
    "test": {
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    }
  }
}
```

**Auto-fix mode** (run formatters on failure):
```json
{
  "gates": {
    "check": {
      "on_pass": "CONTINUE",
      "on_fail": "./scripts/auto-fix.sh"
    },
    "test": {
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

**Critical gates** (STOP on failure):
```json
{
  "gates": {
    "security-scan": {
      "description": "Run security vulnerability scan",
      "on_pass": "CONTINUE",
      "on_fail": "STOP"
    }
  }
}
```

## User Experience

- **Transparent**: Agents see gate results in their context
- **Automatic**: No manual `/check` commands needed
- **Configurable**: Projects control which gates run and their actions via gates.json
- **Tool-agnostic**: Uses canonical command vocabulary from CLAUDE.md
- **Flexible**: Actions range from warnings to full stops, with custom command support

## Benefits

1. **Consistent Quality**: All agent work validated automatically
2. **Early Detection**: Issues caught at edit time (PostToolUse) or completion (SubagentStop)
3. **Flexible Enforcement**: Configure pass/fail actions per gate (CONTINUE, BLOCK, STOP, custom)
4. **Tool-Agnostic**: Works with any project using canonical commands
5. **Existing Patterns**: Reuses command discovery from user-prompt-submit.sh
6. **Auto-Remediation**: Custom commands can fix issues automatically (e.g., formatting)

## Implementation Files

- `plugin/hooks/hooks.json` - Hook registration
- `plugin/hooks/gates.json` - Gate configuration
- `plugin/hooks/post-tool-use.sh` - PostToolUse hook script
- `plugin/hooks/subagent-stop.sh` - SubagentStop hook script
- `plugin/hooks/shared-functions.sh` - Shared helper functions (get_command, run_gate, handle_action)
