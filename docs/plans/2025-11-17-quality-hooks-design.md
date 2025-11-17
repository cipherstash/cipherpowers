# Quality Hooks Design

**Date**: 2025-11-17
**Status**: Approved
**Author**: Design Session

## Overview

Automated quality enforcement via Claude Code's hook system. Runs project test and check commands automatically when agents modify code or complete work.

## Requirements

- **Event-specific triggers**: Different checks at different hook points
- **Context-dependent handling**: Auto-fix formatting, warn on lint, block on type/test errors
- **Project-level config**: Configurable which agents use hooks
- **Tool-agnostic**: Uses canonical command vocabulary (test, check, build, run)
- **Integration**: Extends Claude Code's existing hook system

## Architecture

### Hook Registration

**File**: `plugin/hooks/hooks.json`

Registers quality hooks with Claude Code:

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

### Quality Checks Configuration

**File**: `plugin/hooks/quality-checks.json`

Defines what checks run when:

```json
{
  "checks": {
    "check": {
      "description": "Run project quality checks (formatting, linting, types)",
      "blocking": false,
      "on_failure": "warn"
    },
    "test": {
      "description": "Run project test suite",
      "blocking": true,
      "on_failure": "block"
    },
    "build": {
      "description": "Run project build",
      "blocking": false,
      "on_failure": "warn"
    }
  },
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write", "mcp__serena__replace_symbol_body"],
      "checks": ["check"]
    },
    "SubagentStop": {
      "enabled_agents": ["rust-engineer", "code-reviewer", "ultrathink-debugger"],
      "checks": ["check", "test"]
    }
  }
}
```

### Hook Script Implementation

**PostToolUse Hook** (`plugin/hooks/post-tool-use.sh`):

```bash
#!/usr/bin/env bash
# Receives JSON via stdin with tool execution details

# Parse input
INPUT=$(cat)
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name')
CWD=$(echo "$INPUT" | jq -r '.cwd')

# Load quality-checks config
CONFIG="${CLAUDE_PLUGIN_ROOT}/hooks/quality-checks.json"
ENABLED_TOOLS=$(jq -r '.hooks.PostToolUse.enabled_tools[]' "$CONFIG")

# Check if this tool should trigger checks
if ! echo "$ENABLED_TOOLS" | grep -q "$TOOL_NAME"; then
  exit 0  # No output = continue normally
fi

# Get checks to run
CHECKS=$(jq -r '.hooks.PostToolUse.checks[]' "$CONFIG")

# Run each check
for check in $CHECKS; do
  # Get check command from CLAUDE.md (same pattern as user-prompt-submit.sh)
  CHECK_CMD=$(get_command_from_claudemd "$check")

  # Run the check
  if ! eval "$CHECK_CMD" 2>&1; then
    # Check failed - read blocking/on_failure config
    BLOCKING=$(jq -r ".checks.$check.blocking" "$CONFIG")

    if [ "$BLOCKING" = "true" ]; then
      # Block with reason
      jq -n --arg reason "Project $check command failed" '{
        decision: "block",
        reason: $reason
      }'
      exit 0
    else
      # Warn via additionalContext
      jq -n --arg msg "Warning: $check failed (non-blocking)" '{
        additionalContext: $msg
      }'
    fi
  fi
done
```

**SubagentStop Hook** (`plugin/hooks/subagent-stop.sh`):
- Similar structure but checks `enabled_agents` instead of `enabled_tools`
- Runs more comprehensive checks (check + test)
- Always includes results in additionalContext for agent to see

### Command Discovery Pattern

Reuses existing `get_command()` function from `user-prompt-submit.sh`:

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

### Blocking vs Non-Blocking Behavior

**Non-blocking checks** (blocking: false):
- Hook outputs additionalContext with warning
- Agent sees warning but can continue
- Example: `check` command failures

**Blocking checks** (blocking: true):
- Hook outputs `decision: "block"` with reason
- Agent cannot proceed until fixed
- Example: `test` command failures

### Hook Output Patterns

1. **Success** - No output, hook exits 0
   - Check passed, continue normally

2. **Non-blocking failure** - additionalContext only
   ```json
   {
     "additionalContext": "⚠️ Project check command failed (non-blocking):\n<output>"
   }
   ```
   - Agent sees warning, can choose to fix or continue

3. **Blocking failure** - decision: block
   ```json
   {
     "decision": "block",
     "reason": "Project test command failed. Fix errors before completing."
   }
   ```
   - Agent cannot proceed until fixed
   - SubagentStop blocked = agent stays active

### Failure Recovery

- Agent sees error output in context
- Can fix issues and try again
- Hook re-runs automatically on next completion attempt
- Provides clear feedback about what failed and why

### Hook Script Safety

- Scripts use `set -euo pipefail` for safety
- Malformed JSON = hook fails safely (no block)
- Missing config = hook exits cleanly
- Command not found in CLAUDE.md = skip that check

## Agent Integration

Agents are made aware of quality hooks via their prompt references:

```markdown
# Agent Prompt
You enforce strict quality standards.

Quality hooks are configured in ${CLAUDE_PLUGIN_ROOT}/hooks/quality-checks.json

When you complete work:
- SubagentStop hook will run project check and test commands
- Blocking failures will prevent you from returning to main context
- You'll see results in additionalContext and must fix issues
```

## Project Setup

Projects opt into quality hooks by:

1. **Adding hooks.json** to their plugin:
   ```
   plugin/hooks/hooks.json (registers PostToolUse, SubagentStop)
   plugin/hooks/quality-checks.json (configures what runs when)
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
   ```

## Customization

### Configuration Points

- **Which agents use hooks**: Edit `enabled_agents` in quality-checks.json
- **Which tools trigger hooks**: Edit `enabled_tools` in quality-checks.json
- **What checks run when**: Edit `checks` array per hook event
- **Blocking behavior**: Set `blocking: true/false` per check
- **Project commands**: Update CLAUDE.md frontmatter

### Example Configurations

**Strict mode** (block on everything):
```json
{
  "checks": {
    "check": { "blocking": true },
    "test": { "blocking": true }
  }
}
```

**Permissive mode** (warn only):
```json
{
  "checks": {
    "check": { "blocking": false },
    "test": { "blocking": false }
  }
}
```

## User Experience

- **Transparent**: Agents see check results in their context
- **Automatic**: No manual `/check` commands needed
- **Configurable**: Projects control which checks run via quality-checks.json
- **Tool-agnostic**: Uses canonical command vocabulary from CLAUDE.md

## Benefits

1. **Consistent Quality**: All agent work validated automatically
2. **Early Detection**: Issues caught at edit time (PostToolUse) or completion (SubagentStop)
3. **Flexible Enforcement**: Configure blocking vs warning per check type
4. **Tool-Agnostic**: Works with any project using canonical commands
5. **Existing Patterns**: Reuses command discovery from user-prompt-submit.sh

## Implementation Files

- `plugin/hooks/hooks.json` - Hook registration
- `plugin/hooks/quality-checks.json` - Check configuration
- `plugin/hooks/post-tool-use.sh` - PostToolUse hook script
- `plugin/hooks/subagent-stop.sh` - SubagentStop hook script
- `plugin/hooks/shared-functions.sh` - Shared helper functions (get_command, etc.)
