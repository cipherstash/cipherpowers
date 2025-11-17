# Quality Gates Design

**Date**: 2025-11-17
**Status**: Approved
**Author**: Design Session

## Overview

Automated quality enforcement via Claude Code's hook system. Runs project test and check commands automatically when agents modify code or complete work.

## Requirements

- **Event-specific triggers**: Different gates at different hook points
- **Action-based handling**: Configurable actions on pass/fail (CONTINUE, BLOCK, STOP, or gate chaining)
- **Project-level config**: Configurable which agents use hooks
- **Tool-agnostic**: Uses canonical command vocabulary (test, check, build, run)
- **Integration**: Extends Claude Code's existing hook system
- **Gate chaining**: Actions can reference other gates for complex workflows

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
      "command": "mise run check",
      "on_pass": "BLOCK",
      "on_fail": "STOP"
    },
    "test": {
      "description": "Run project test suite",
      "command": "mise run test"
    },
    "build": {
      "description": "Run project build",
      "command": "mise run build",
      "on_fail": "CONTINUE"
    },
    "format": {
      "description": "Auto-format code",
      "command": "mise run format",
      "on_pass": "check",
      "on_fail": "STOP"
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

A gate is a **named configuration** that defines a command to run, with optional on_pass and on_fail actions:

- **Gate name**: Unique identifier for the gate (e.g., "check")
- **command**: Shell command to execute (e.g., "mise run check")
- **description**: Human-readable description (optional)
- **on_pass**: Action when gate passes (default: "CONTINUE")
- **on_fail**: Action when gate fails (default: "BLOCK")

Example:
```json
{
  "check": {
    "description": "Run project quality checks",
    "command": "mise run check",
    "on_pass": "CONTINUE",
    "on_fail": "BLOCK"
  }
}
```

Gates are self-contained in gates.json - all configuration in one place.

#### Actions

Actions determine what happens after a gate executes. An action can be:

- **CONTINUE**: Proceed to next gate in sequence (or complete if last gate)
- **BLOCK**: Stop execution, prevent agent from proceeding (returns `decision: "block"`)
- **STOP**: Stop Claude entirely (returns `continue: false`)
- **{gate_name}**: Chain to another gate (e.g., `"check"` → executes the "check" gate)

#### Gate Chaining

Actions can reference other gates, calling them like subroutines. After the chained gate completes, the original sequence resumes (if the chained gate returns CONTINUE).

```json
{
  "gates": {
    "format": {
      "command": "mise run format",
      "on_pass": "check",
      "on_fail": "STOP"
    },
    "check": {
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "command": "mise run test"
    }
  },
  "hooks": {
    "SubagentStop": {
      "gates": ["format", "test"]
    }
  }
}
```

Execution with gate chaining:
- `format` passes → calls `check` gate (subroutine)
- `check` passes → returns CONTINUE → sequence resumes → runs `test`
- If `format` fails → STOP immediately (no test)
- If `check` fails → BLOCK (halts sequence, no test)

#### Execution Order

Gate execution follows these rules:

1. **Sequential by default**: Gates listed in `"gates": ["check", "test"]` run in order
2. **Action is immediate**:
   - BLOCK prevents subsequent gates in the list from running
   - STOP halts everything immediately
   - CONTINUE proceeds to next gate in the list
   - Gate reference calls that gate like a subroutine
3. **Chaining is a subroutine call**: When action references a gate, that gate executes and then the original sequence resumes

Gate chaining behavior:
- Chained gate executes immediately
- Chained gate's action rules apply (CONTINUE/BLOCK/STOP/another chain)
- If chained gate returns CONTINUE (default on_pass), original sequence resumes
- If chained gate returns BLOCK/STOP, original sequence halts

Example execution:
```json
{
  "gates": {
    "check": {
      "command": "mise run check",
      "on_pass": "reticulate"
    },
    "reticulate": {
      "description": "Reticulate splines",
      "command": "mise run reticulate"
    },
    "test": {
      "command": "mise run test"
    }
  },
  "hooks": {
    "SubagentStop": {
      "gates": ["check", "test"]
    }
  }
}
```

Execution flow:
1. Run `check` gate (first in sequence)
2. Check passes → `on_pass: "reticulate"` calls reticulate gate
3. Reticulate executes:
   - If reticulate passes → CONTINUE (default) → sequence resumes
   - If reticulate fails → BLOCK (default) → sequence halts
4. If sequence resumed, run `test` gate (next in original sequence)

#### Default Behavior

If `on_pass` or `on_fail` are omitted:
- `on_pass`: "CONTINUE"
- `on_fail`: "BLOCK"

#### Missing Gate Handling

- **Gate referenced but not defined** in gates.json: STOP with error
- **Gate defined but missing command field**: STOP with error
- Rationale: Missing configuration is a critical error, not a silent skip

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

# Run each gate in sequence
for gate in $GATES; do
  run_gate "$gate" "$CONFIG"
  # If gate returns non-zero, stop execution (BLOCK or STOP action)
  if [ $? -ne 0 ]; then
    break
  fi
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

# Run each gate in sequence
for gate in $GATES; do
  run_gate "$gate" "$CONFIG"
  # If gate returns non-zero, stop execution (BLOCK or STOP action)
  if [ $? -ne 0 ]; then
    break
  fi
done
```

**Shared Functions** (`plugin/hooks/shared-functions.sh`):

```bash
#!/usr/bin/env bash

# Check if gate exists in configuration
gate_exists() {
  local gate_name="$1"
  local config="$2"

  local exists=$(jq -r ".gates | has(\"$gate_name\")" "$config")
  [ "$exists" = "true" ]
}

# Execute a gate and handle its result
# Returns: exit code (0 = continue, 1 = stop execution)
run_gate() {
  local gate_name="$1"
  local config="$2"

  # Check if gate is defined in gates.json
  if ! gate_exists "$gate_name" "$config"; then
    # Missing gate definition - STOP
    jq -n --arg gate "$gate_name" '{
      continue: false,
      message: ("Gate '\($gate)' referenced but not defined in gates.json")
    }'
    return 1
  fi

  # Get gate command from gates.json
  local gate_cmd=$(jq -r ".gates.$gate_name.command // empty" "$config")

  if [ -z "$gate_cmd" ]; then
    # Missing command field - STOP
    jq -n --arg gate "$gate_name" '{
      continue: false,
      message: ("Gate '\($gate)' is missing required 'command' field")
    }'
    return 1
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
    handle_action "$on_pass" "$gate_name" "passed" "$output" "$config"
  else
    # Gate failed
    handle_action "$on_fail" "$gate_name" "failed" "$output" "$config"
  fi
}

# Handle gate action (CONTINUE, BLOCK, STOP, or gate chaining)
# Returns: exit code (0 = continue, 1 = stop execution)
handle_action() {
  local action="$1"
  local gate_name="$2"
  local status="$3"
  local output="$4"
  local config="$5"

  case "$action" in
    CONTINUE)
      # Continue to next gate in sequence
      if [ "$status" = "failed" ]; then
        jq -n --arg msg "⚠️ Gate '$gate_name' failed but continuing:\n$output" '{
          additionalContext: $msg
        }'
      fi
      return 0
      ;;

    BLOCK)
      # Block execution, prevent subsequent gates
      jq -n --arg reason "Gate '$gate_name' $status. Output:\n$output" '{
        decision: "block",
        reason: $reason
      }'
      return 1
      ;;

    STOP)
      # Stop Claude entirely
      jq -n --arg msg "Gate '$gate_name' $status. Stopping Claude.\n$output" '{
        continue: false,
        message: $msg
      }'
      return 1
      ;;

    *)
      # Gate chaining - run the referenced gate
      if gate_exists "$action" "$config"; then
        # Chain to another gate
        run_gate "$action" "$config"
        return $?
      else
        # Referenced gate doesn't exist - STOP
        jq -n --arg gate "$gate_name" --arg ref "$action" '{
          continue: false,
          message: ("Gate '\($gate)' references undefined gate '\($ref)'")
        }'
        return 1
      fi
      ;;
  esac
}
```

## Error Handling

### Action-Based Behavior

**CONTINUE**:
- Hook outputs optional additionalContext with information
- Proceeds to next gate in sequence (or completes if last gate)
- Example: Non-critical warnings

**BLOCK**:
- Hook outputs `decision: "block"` with reason
- Prevents subsequent gates from running
- Agent cannot proceed until issue is fixed
- Example: Test failures that must be resolved

**STOP**:
- Hook outputs `continue: false`
- Stops Claude entirely (equivalent to Ctrl+C)
- Halts all gate execution immediately
- Example: Critical system failures or configuration errors

**Gate Chaining** (action = gate name):
- Executes the referenced gate immediately (like a subroutine call)
- Original sequence resumes after chained gate completes
- If chained gate returns CONTINUE, next gate in original sequence runs
- If chained gate returns BLOCK/STOP, original sequence halts
- Example: `gates: ["check", "test"]` with `check.on_pass: "reticulate"` → check passes → reticulate runs → if reticulate passes → test runs

### Hook Output Patterns

1. **CONTINUE (success)** - No output, function returns 0
   ```bash
   # Gate passed, continue to next gate in sequence
   return 0
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

5. **Gate Chaining** - Recursive call to run_gate
   ```bash
   # Action references another gate
   run_gate "check" "$config"
   return $?  # Pass through chained gate's result
   ```

6. **Missing Gate Error** - STOP
   ```json
   {
     "continue": false,
     "message": "Gate 'check' referenced but not defined in gates.json"
   }
   ```

7. **Missing Command Error** - STOP
   ```json
   {
     "continue": false,
     "message": "Gate 'check' is missing required 'command' field"
   }
   ```

### Failure Recovery

- Agent sees error output in context
- Can fix issues and try again
- Hook re-runs automatically on next completion attempt
- Provides clear feedback about what failed and why
- Gate chaining enables complex workflows (e.g., format → check → test)

### Hook Script Safety

- Scripts use `set -euo pipefail` for safety
- Malformed JSON = hook fails safely (no block)
- Missing config = hook exits cleanly (no gates run)
- **Gate not defined in gates.json = STOP** (configuration error)
- **Gate missing command field = STOP** (configuration error)
- **Referenced gate doesn't exist = STOP** (chaining error)
- All configuration errors halt execution to prevent silent failures

## Agent Integration

Agents are made aware of quality gates via their prompt references:

```markdown
# Agent Prompt
You enforce strict quality standards.

Quality gates are configured in ${CLAUDE_PLUGIN_ROOT}/hooks/gates.json

When you complete work:
- SubagentStop hook will run project gates (check, test, etc.)
- Gate actions: CONTINUE (proceed), BLOCK (fix required), STOP (critical error)
- Gates can chain to other gates for complex workflows
- You'll see results in additionalContext and must respond appropriately
```

## Project Setup

Projects opt into quality gates by:

1. **Configuring gates in gates.json**:
   - Define each gate with name, command, and actions
   - Configure which hooks trigger which gates
   - Example: `plugin/hooks/gates.json`

2. **Registering hooks with Claude Code**:
   - Register PostToolUse and SubagentStop hooks
   - Point to hook scripts
   - Example: `plugin/hooks/hooks.json`

3. **Implementing hook scripts**:
   ```
   plugin/hooks/post-tool-use.sh
   plugin/hooks/subagent-stop.sh
   plugin/hooks/shared-functions.sh
   ```

All gate configuration is self-contained in `gates.json` - no external dependencies.

## Customization

### Configuration Points

- **Which agents use hooks**: Edit `enabled_agents` in gates.json
- **Which tools trigger hooks**: Edit `enabled_tools` in gates.json
- **What gates run when**: Edit `gates` array per hook event
- **Pass/fail behavior**: Set `on_pass`/`on_fail` actions per gate
- **Gate commands**: Set `command` field per gate in gates.json

### Example Configurations

**Strict mode** (block on all failures):
```json
{
  "gates": {
    "check": {
      "description": "Quality checks",
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "description": "Test suite",
      "command": "mise run test",
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
      "description": "Quality checks",
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    },
    "test": {
      "description": "Test suite",
      "command": "mise run test",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    }
  }
}
```

**Gate chaining as pipeline** (format → check → test):
```json
{
  "gates": {
    "format": {
      "description": "Auto-format code",
      "command": "mise run format",
      "on_pass": "check",
      "on_fail": "STOP"
    },
    "check": {
      "description": "Quality checks",
      "command": "mise run check",
      "on_pass": "test",
      "on_fail": "BLOCK"
    },
    "test": {
      "description": "Test suite",
      "command": "mise run test",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "SubagentStop": {
      "gates": ["format"]
    }
  }
}
```
In this example, only "format" is listed in the gates array. If format passes, it chains to check. If check passes, it chains to test. This creates a pure pipeline without listing all gates explicitly.

**Gate chaining as subroutine** (check → reticulate → test):
```json
{
  "gates": {
    "check": {
      "description": "Quality checks",
      "command": "mise run check",
      "on_pass": "reticulate"
    },
    "reticulate": {
      "description": "Reticulate splines",
      "command": "mise run reticulate"
    },
    "test": {
      "description": "Test suite",
      "command": "mise run test"
    }
  },
  "hooks": {
    "SubagentStop": {
      "gates": ["check", "test"]
    }
  }
}
```
Execution flow:
1. Run `check` (first in sequence)
2. If check passes → calls `reticulate` (subroutine)
3. If reticulate passes → CONTINUE (default) → sequence resumes → runs `test`
4. If reticulate fails → BLOCK (default) → sequence halts (test skipped)

**Critical gates** (STOP on failure):
```json
{
  "gates": {
    "security-scan": {
      "description": "Run security vulnerability scan",
      "command": "mise run security-scan",
      "on_pass": "CONTINUE",
      "on_fail": "STOP"
    }
  }
}
```

**Unusual configuration** (inverted logic for testing):
```json
{
  "gates": {
    "check": {
      "description": "Quality checks that should fail",
      "command": "mise run check",
      "on_pass": "BLOCK",
      "on_fail": "STOP"
    }
  }
}
```
This example demonstrates flexibility: blocks when check passes, stops when it fails. Useful for testing gate behavior or enforcing that certain checks should fail in specific contexts.

## User Experience

- **Transparent**: Agents see gate results in their context
- **Automatic**: No manual `/check` commands needed
- **Configurable**: Projects control which gates run and their actions via gates.json
- **Self-contained**: All configuration in one file (gates.json)
- **Flexible**: Actions range from warnings to full stops, with gate chaining support
- **Composable**: Build complex workflows by chaining gates together

## Benefits

1. **Consistent Quality**: All agent work validated automatically
2. **Early Detection**: Issues caught at edit time (PostToolUse) or completion (SubagentStop)
3. **Flexible Enforcement**: Configure pass/fail actions per gate (CONTINUE, BLOCK, STOP, chaining)
4. **Self-Contained**: All gate configuration in gates.json - no external dependencies
5. **Tool-Agnostic**: Works with any command-line tool (mise, cargo, npm, make, etc.)
6. **Gate Chaining**: Build complex workflows (format → check → test) declaratively
7. **Safe Defaults**: Missing configuration = STOP (fail-safe behavior)

## Implementation Files

- `plugin/hooks/hooks.json` - Hook registration
- `plugin/hooks/gates.json` - Gate configuration (self-contained, includes all commands)
- `plugin/hooks/post-tool-use.sh` - PostToolUse hook script
- `plugin/hooks/subagent-stop.sh` - SubagentStop hook script
- `plugin/hooks/shared-functions.sh` - Shared helper functions:
  - `gate_exists()` - Check if gate is defined
  - `run_gate()` - Execute gate and handle result (reads command from gates.json)
  - `handle_action()` - Process CONTINUE/BLOCK/STOP/chaining
