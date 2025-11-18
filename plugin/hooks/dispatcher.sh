#!/usr/bin/env bash
set -euo pipefail

# Universal hook dispatcher for all hook events
# Routes hook events to configured gates based on gates.json

# Source shared functions
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ ! -f "${SCRIPT_DIR}/shared-functions.sh" ]; then
  echo '{"continue": false, "message": "shared-functions.sh not found"}' | jq .
  exit 1
fi
source "${SCRIPT_DIR}/shared-functions.sh"

# Parse input - hook events send different fields based on type
INPUT=$(cat)
HOOK_EVENT=$(echo "$INPUT" | jq -r '.hook_event // "unknown"')
CWD=$(echo "$INPUT" | jq -r '.cwd // "."')

# Extract context based on hook type
case "$HOOK_EVENT" in
  PostToolUse)
    CONTEXT_KEY="tool_name"
    CONTEXT_VALUE=$(echo "$INPUT" | jq -r '.tool_name // "unknown"')
    ENABLED_LIST_KEY="enabled_tools"
    ;;
  SubagentStop)
    CONTEXT_KEY="agent_name"
    CONTEXT_VALUE=$(echo "$INPUT" | jq -r '.agent_name // .subagent_name // "unknown"')
    ENABLED_LIST_KEY="enabled_agents"
    ;;
  UserPromptSubmit)
    CONTEXT_KEY="user_message"
    CONTEXT_VALUE=$(echo "$INPUT" | jq -r '.user_message // ""')
    ENABLED_LIST_KEY="enabled"
    ;;
  *)
    # Unknown hook event - exit cleanly
    exit 0
    ;;
esac

# Load gates config - check project directory first, then plugin default
if [ -f "${CWD}/.claude/gates.json" ]; then
  CONFIG="${CWD}/.claude/gates.json"
elif [ -f "${CWD}/gates.json" ]; then
  CONFIG="${CWD}/gates.json"
elif [ -f "${CLAUDE_PLUGIN_ROOT}/hooks/gates.json" ]; then
  CONFIG="${CLAUDE_PLUGIN_ROOT}/hooks/gates.json"
else
  # No config found - exit cleanly
  exit 0
fi

# Check if hook is configured
HOOK_CONFIG=$(jq -r ".hooks.${HOOK_EVENT} // null" "$CONFIG")
if [ "$HOOK_CONFIG" = "null" ]; then
  # Hook not configured - exit cleanly
  exit 0
fi

# For PostToolUse and SubagentStop: Check if context value is in enabled list
if [ "$HOOK_EVENT" = "PostToolUse" ] || [ "$HOOK_EVENT" = "SubagentStop" ]; then
  ENABLED_LIST=$(jq -r ".hooks.${HOOK_EVENT}.${ENABLED_LIST_KEY}[]" "$CONFIG" 2>/dev/null || echo "")

  if ! echo "$ENABLED_LIST" | grep -q "^${CONTEXT_VALUE}$"; then
    # Not enabled for this tool/agent - exit cleanly
    exit 0
  fi
fi

# Special handling for UserPromptSubmit: Always run commands gate first
if [ "$HOOK_EVENT" = "UserPromptSubmit" ]; then
  # Export INPUT for commands.sh
  export HOOK_INPUT="$INPUT"

  # Run built-in commands gate (context injection, always runs first)
  "${CLAUDE_PLUGIN_ROOT}/hooks/gates/commands.sh" || true

  # Then check if additional gates are configured
  ENABLED=$(jq -r ".hooks.${HOOK_EVENT}.enabled // false" "$CONFIG")
  if [ "$ENABLED" != "true" ]; then
    # No additional gates configured - exit after commands
    exit 0
  fi

  # Get additional gates to run (if any)
  GATES=$(jq -r ".hooks.${HOOK_EVENT}.gates[]" "$CONFIG" 2>/dev/null || echo "")
else
  # For PostToolUse and SubagentStop: Get gates to run
  GATES=$(jq -r ".hooks.${HOOK_EVENT}.gates[]" "$CONFIG")

  # Export INPUT for gates that need access to full hook context
  export HOOK_INPUT="$INPUT"
fi

# Run each gate in sequence
for gate in $GATES; do
  run_gate "$gate" "$CONFIG"
  # If gate returns non-zero, stop execution (BLOCK or STOP action)
  if [ $? -ne 0 ]; then
    break
  fi
done
