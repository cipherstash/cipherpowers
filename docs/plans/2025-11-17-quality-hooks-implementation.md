# Quality Hooks Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement automated quality gates via Claude Code's hook system that run project commands (test, check, build) when agents modify code or complete work.

**Architecture:** Three-component system: (1) Hook scripts (post-tool-use.sh, subagent-stop.sh) that respond to Claude Code events, (2) Shared gate execution logic (shared-functions.sh) that reads gates.json configuration, (3) Self-contained configuration (gates.json) that defines gates, commands, and pass/fail actions.

**Tech Stack:** Bash scripts, jq for JSON processing, Claude Code hook system, mise for task orchestration

---

## Task 1: Create Gates Configuration

**Files:**
- Create: `plugin/hooks/gates.json`

**Step 1: Write gates.json with initial configuration**

Create the gates configuration file with example gates and hook configurations:

```json
{
  "gates": {
    "check": {
      "description": "Run project quality checks (formatting, linting, types)",
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "description": "Run project test suite",
      "command": "mise run test",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "build": {
      "description": "Run project build",
      "command": "mise run build",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
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

**Step 2: Verify JSON is valid**

Run: `jq . plugin/hooks/gates.json`
Expected: Pretty-printed JSON (no errors)

**Step 3: Commit**

```bash
git add plugin/hooks/gates.json
git commit -m "feat: add quality gates configuration"
```

---

## Task 2: Create Shared Functions Library

**Files:**
- Create: `plugin/hooks/shared-functions.sh`

**Step 1: Write shared-functions.sh with gate execution logic**

Create the shared functions library:

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
      message: ("Gate '\''\\($gate)'\'''' referenced but not defined in gates.json")
    }'
    return 1
  fi

  # Get gate command from gates.json
  local gate_cmd=$(jq -r ".gates.$gate_name.command // empty" "$config")

  if [ -z "$gate_cmd" ]; then
    # Missing command field - STOP
    jq -n --arg gate "$gate_name" '{
      continue: false,
      message: ("Gate '\''\\($gate)'\'''' is missing required '\''command'\'' field")
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
        jq -n --arg msg "⚠️ Gate '\''$gate_name'\'' failed but continuing:\\n$output" '{
          additionalContext: $msg
        }'
      fi
      return 0
      ;;

    BLOCK)
      # Block execution, prevent subsequent gates
      jq -n --arg reason "Gate '\''$gate_name'\'' $status. Output:\\n$output" '{
        decision: "block",
        reason: $reason
      }'
      return 1
      ;;

    STOP)
      # Stop Claude entirely
      jq -n --arg msg "Gate '\''$gate_name'\'' $status. Stopping Claude.\\n$output" '{
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
          message: ("Gate '\''\\($gate)'\'''' references undefined gate '\''\\($ref)'\''")
        }'
        return 1
      fi
      ;;
  esac
}
```

**Step 2: Make script executable**

Run: `chmod +x plugin/hooks/shared-functions.sh`
Expected: File is executable

**Step 3: Test gate_exists function**

Run:
```bash
source plugin/hooks/shared-functions.sh
gate_exists "check" "plugin/hooks/gates.json" && echo "check exists" || echo "check missing"
```
Expected: `check exists`

**Step 4: Test gate_exists with missing gate**

Run:
```bash
source plugin/hooks/shared-functions.sh
gate_exists "nonexistent" "plugin/hooks/gates.json" && echo "exists" || echo "missing"
```
Expected: `missing`

**Step 5: Commit**

```bash
git add plugin/hooks/shared-functions.sh
git commit -m "feat: add shared gate execution functions"
```

---

## Task 3: Create PostToolUse Hook Script

**Files:**
- Create: `plugin/hooks/post-tool-use.sh`

**Step 1: Write post-tool-use.sh**

Create the PostToolUse hook script:

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

# Exit cleanly if config doesn't exist
if [ ! -f "$CONFIG" ]; then
  exit 0
fi

ENABLED_TOOLS=$(jq -r '.hooks.PostToolUse.enabled_tools[]' "$CONFIG" 2>/dev/null || echo "")

# Check if this tool should trigger gates
if ! echo "$ENABLED_TOOLS" | grep -q "^${TOOL_NAME}$"; then
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

**Step 2: Make script executable**

Run: `chmod +x plugin/hooks/post-tool-use.sh`
Expected: File is executable

**Step 3: Test hook with mock input (Edit tool)**

Create test input:
```bash
echo '{"tool_name": "Edit", "cwd": "/test"}' | plugin/hooks/post-tool-use.sh
```
Expected: Hook runs check gate (command will fail but should see gate execution attempt)

**Step 4: Test hook with non-enabled tool**

Run:
```bash
echo '{"tool_name": "Read", "cwd": "/test"}' | plugin/hooks/post-tool-use.sh
```
Expected: No output (hook exits early)

**Step 5: Commit**

```bash
git add plugin/hooks/post-tool-use.sh
git commit -m "feat: add PostToolUse hook script"
```

---

## Task 4: Create SubagentStop Hook Script

**Files:**
- Create: `plugin/hooks/subagent-stop.sh`

**Step 1: Write subagent-stop.sh**

Create the SubagentStop hook script:

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

# Exit cleanly if config doesn't exist
if [ ! -f "$CONFIG" ]; then
  exit 0
fi

ENABLED_AGENTS=$(jq -r '.hooks.SubagentStop.enabled_agents[]' "$CONFIG" 2>/dev/null || echo "")

# Check if this agent should trigger gates
if ! echo "$ENABLED_AGENTS" | grep -q "^${AGENT_NAME}$"; then
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

**Step 2: Make script executable**

Run: `chmod +x plugin/hooks/subagent-stop.sh`
Expected: File is executable

**Step 3: Test hook with mock input (rust-engineer agent)**

Run:
```bash
echo '{"agent_name": "rust-engineer", "cwd": "/test"}' | plugin/hooks/subagent-stop.sh
```
Expected: Hook runs check and test gates (commands will fail but should see gate execution attempts)

**Step 4: Test hook with non-enabled agent**

Run:
```bash
echo '{"agent_name": "general-purpose", "cwd": "/test"}' | plugin/hooks/subagent-stop.sh
```
Expected: No output (hook exits early)

**Step 5: Commit**

```bash
git add plugin/hooks/subagent-stop.sh
git commit -m "feat: add SubagentStop hook script"
```

---

## Task 5: Register Hooks with Claude Code

**Files:**
- Modify: `plugin/hooks/hooks.json`

**Step 1: Read current hooks.json**

Run: `cat plugin/hooks/hooks.json`
Expected: View current hook configuration

**Step 2: Update hooks.json to include quality gate hooks**

Add PostToolUse and SubagentStop hooks to the configuration:

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": ".*",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/session-start.sh"
          }
        ]
      }
    ],
    "UserPromptSubmit": [
      {
        "matcher": ".*",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/user-prompt-submit.sh"
          }
        ]
      }
    ],
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

**Step 3: Verify JSON is valid**

Run: `jq . plugin/hooks/hooks.json`
Expected: Pretty-printed JSON (no errors)

**Step 4: Commit**

```bash
git add plugin/hooks/hooks.json
git commit -m "feat: register quality gate hooks with Claude Code"
```

---

## Task 6: Create Test Configuration for Verification

**Files:**
- Create: `plugin/hooks/gates.test.json`

**Step 1: Write test configuration with simple gates**

Create a test configuration for manual verification:

```json
{
  "gates": {
    "always-pass": {
      "description": "Gate that always passes",
      "command": "echo 'Gate passed'",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "always-fail": {
      "description": "Gate that always fails",
      "command": "exit 1",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "warn-only": {
      "description": "Gate that fails but continues",
      "command": "exit 1",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    },
    "chain-test": {
      "description": "Gate that chains to another",
      "command": "echo 'Chaining'",
      "on_pass": "always-pass",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit"],
      "gates": ["always-pass"]
    },
    "SubagentStop": {
      "enabled_agents": ["test-agent"],
      "gates": ["always-pass", "warn-only"]
    }
  }
}
```

**Step 2: Test always-pass gate**

Run:
```bash
source plugin/hooks/shared-functions.sh
run_gate "always-pass" "plugin/hooks/gates.test.json"
echo "Exit code: $?"
```
Expected: Exit code 0 (CONTINUE)

**Step 3: Test always-fail gate**

Run:
```bash
source plugin/hooks/shared-functions.sh
run_gate "always-fail" "plugin/hooks/gates.test.json"
echo "Exit code: $?"
```
Expected: Exit code 1 (BLOCK), JSON output with decision: "block"

**Step 4: Test warn-only gate**

Run:
```bash
source plugin/hooks/shared-functions.sh
run_gate "warn-only" "plugin/hooks/gates.test.json"
echo "Exit code: $?"
```
Expected: Exit code 0 (CONTINUE), JSON output with additionalContext warning

**Step 5: Test chain-test gate**

Run:
```bash
source plugin/hooks/shared-functions.sh
run_gate "chain-test" "plugin/hooks/gates.test.json"
echo "Exit code: $?"
```
Expected: Exit code 0, executes chain-test then always-pass

**Step 6: Test missing gate error**

Run:
```bash
source plugin/hooks/shared-functions.sh
run_gate "nonexistent" "plugin/hooks/gates.test.json"
echo "Exit code: $?"
```
Expected: Exit code 1 (STOP), JSON output with continue: false

**Step 7: Commit**

```bash
git add plugin/hooks/gates.test.json
git commit -m "test: add gate verification configuration"
```

---

## Task 7: Create Documentation

**Files:**
- Create: `plugin/hooks/README.md`

**Step 1: Write comprehensive README**

Create documentation explaining the quality hooks system:

```markdown
# Quality Hooks

Automated quality enforcement via Claude Code's hook system. Runs project test and check commands automatically when agents modify code or complete work.

## Overview

Quality hooks integrate with Claude Code's event system to automatically run quality gates (test, check, build commands) at strategic points:

- **PostToolUse**: Runs after code modifications (Edit, Write tools)
- **SubagentStop**: Runs when specialized agents complete their work

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
```

**Step 2: Commit**

```bash
git add plugin/hooks/README.md
git commit -m "docs: add quality hooks documentation"
```

---

## Task 8: Update Agent Prompts

**Files:**
- Modify: `plugin/agents/rust-engineer.md` (example)
- Modify: `plugin/agents/code-reviewer.md` (example)
- Modify: `plugin/agents/ultrathink-debugger.md` (example)

**Step 1: Add quality gates section to rust-engineer.md**

Find the section about quality standards and add:

```markdown
## Quality Gates

Quality gates are configured in ${CLAUDE_PLUGIN_ROOT}/hooks/gates.json

When you complete work:
- SubagentStop hook will run project gates (check, test, etc.)
- Gate actions: CONTINUE (proceed), BLOCK (fix required), STOP (critical error)
- Gates can chain to other gates for complex workflows
- You'll see results in additionalContext and must respond appropriately

If a gate blocks:
1. Review the error output in the block reason
2. Fix the issues
3. Try again (hook re-runs automatically)
```

**Step 2: Add to code-reviewer.md**

Same content as Step 1, add to quality standards section.

**Step 3: Add to ultrathink-debugger.md**

Same content as Step 1, add to quality standards section.

**Step 4: Commit**

```bash
git add plugin/agents/rust-engineer.md plugin/agents/code-reviewer.md plugin/agents/ultrathink-debugger.md
git commit -m "docs: add quality gates documentation to agent prompts"
```

---

## Task 9: Integration Test with Real Agent

**Files:**
- Test in live environment

**Step 1: Create test scenario**

Create a simple test file to trigger PostToolUse hook:

```bash
echo "# Test file for quality hooks" > /tmp/test-quality-hooks.md
```

**Step 2: Dispatch rust-engineer agent to edit file**

Use Claude Code to dispatch rust-engineer agent:
- Ask agent to add a line to `/tmp/test-quality-hooks.md`
- Agent should use Edit tool
- PostToolUse hook should run check gate
- Observe hook output in agent context

**Step 3: Verify SubagentStop hook**

When rust-engineer completes:
- SubagentStop hook should run check and test gates
- Observe gate results
- Verify appropriate action (CONTINUE/BLOCK/STOP)

**Step 4: Test gate chaining**

Modify gates.json to add gate chaining:

```json
{
  "gates": {
    "format": {
      "command": "echo 'Formatting...'",
      "on_pass": "check"
    },
    "check": {
      "command": "echo 'Checking...'",
      "on_pass": "CONTINUE"
    }
  },
  "hooks": {
    "SubagentStop": {
      "enabled_agents": ["rust-engineer"],
      "gates": ["format"]
    }
  }
}
```

Verify chaining works: format → check → continue

**Step 5: Test BLOCK action**

Modify test gate to fail:

```json
{
  "gates": {
    "check": {
      "command": "exit 1",
      "on_fail": "BLOCK"
    }
  }
}
```

Verify agent is blocked with appropriate message.

**Step 6: Document results**

Create brief test report:
- Which scenarios tested
- Results observed
- Any issues found

---

## Task 10: Create Example Configurations

**Files:**
- Create: `plugin/hooks/examples/strict.json`
- Create: `plugin/hooks/examples/permissive.json`
- Create: `plugin/hooks/examples/pipeline.json`

**Step 1: Create examples directory**

Run: `mkdir -p plugin/hooks/examples`
Expected: Directory created

**Step 2: Write strict.json**

```json
{
  "gates": {
    "check": {
      "description": "Quality checks must pass",
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "description": "All tests must pass",
      "command": "mise run test",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "build": {
      "description": "Build must succeed",
      "command": "mise run build",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write", "mcp__serena__replace_symbol_body"],
      "gates": ["check"]
    },
    "SubagentStop": {
      "enabled_agents": ["rust-engineer", "code-reviewer", "ultrathink-debugger"],
      "gates": ["check", "test", "build"]
    }
  }
}
```

**Step 3: Write permissive.json**

```json
{
  "gates": {
    "check": {
      "description": "Quality checks (warn only)",
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    },
    "test": {
      "description": "Tests (warn only)",
      "command": "mise run test",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
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

**Step 4: Write pipeline.json**

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
      "description": "Run tests",
      "command": "mise run test",
      "on_pass": "build",
      "on_fail": "BLOCK"
    },
    "build": {
      "description": "Build project",
      "command": "mise run build",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "SubagentStop": {
      "enabled_agents": ["rust-engineer", "code-reviewer"],
      "gates": ["format"]
    }
  }
}
```

**Step 5: Create examples README**

Create `plugin/hooks/examples/README.md`:

```markdown
# Example Gate Configurations

## strict.json

Block on all quality gate failures. Use when:
- Working on production code
- Quality standards are non-negotiable
- Team is experienced with tooling

Copy to use:
```bash
cp plugin/hooks/examples/strict.json plugin/hooks/gates.json
```

## permissive.json

Warn on failures but continue. Use when:
- Prototyping or experimenting
- Learning new tooling
- Quality checks are aspirational

Copy to use:
```bash
cp plugin/hooks/examples/permissive.json plugin/hooks/gates.json
```

## pipeline.json

Chain gates in sequence (format → check → test → build). Use when:
- Want auto-formatting before checks
- Need ordered quality verification
- Building complex workflows

Copy to use:
```bash
cp plugin/hooks/examples/pipeline.json plugin/hooks/gates.json
```

## Customization

Mix and match concepts:
- Use strict mode for some gates, permissive for others
- Chain critical gates, run others independently
- Enable different gates for different agents/tools
```

**Step 6: Commit**

```bash
git add plugin/hooks/examples/
git commit -m "docs: add example gate configurations"
```

---

## Verification Checklist

After implementing all tasks:

- [ ] `plugin/hooks/gates.json` exists and is valid JSON
- [ ] `plugin/hooks/shared-functions.sh` is executable
- [ ] `plugin/hooks/post-tool-use.sh` is executable
- [ ] `plugin/hooks/subagent-stop.sh` is executable
- [ ] `plugin/hooks/hooks.json` includes PostToolUse and SubagentStop hooks
- [ ] Test gates execute successfully (gates.test.json scenarios)
- [ ] PostToolUse hook triggers on Edit/Write tools
- [ ] SubagentStop hook triggers for configured agents
- [ ] Gate chaining works correctly
- [ ] BLOCK action prevents agent continuation
- [ ] STOP action halts Claude
- [ ] CONTINUE action proceeds with warning
- [ ] Missing gate configuration produces STOP with error
- [ ] Documentation is complete (README.md, agent prompts)
- [ ] Example configurations are provided

## Testing Strategy

**Unit Testing:**
- Test individual functions (gate_exists, run_gate, handle_action)
- Test with gates.test.json configuration
- Verify all action types (CONTINUE, BLOCK, STOP, chaining)

**Integration Testing:**
- Test PostToolUse hook with real Edit/Write operations
- Test SubagentStop hook with real agent completions
- Test gate chaining workflows
- Test error scenarios (missing gates, missing commands)

**Manual Verification:**
- Run hooks with mock input
- Verify hook output format (JSON)
- Check exit codes match expected behavior
- Confirm agent prompts reference quality gates

## Success Criteria

- All gate types execute correctly
- All action types work as designed
- Hooks integrate with Claude Code events
- Configuration is self-contained in gates.json
- Error handling is robust (missing config, invalid gates)
- Documentation enables users to customize gates
- Example configurations demonstrate common patterns
