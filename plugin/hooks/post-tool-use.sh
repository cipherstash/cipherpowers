#!/usr/bin/env bash
set -euo pipefail

# Source shared functions
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ ! -f "${SCRIPT_DIR}/shared-functions.sh" ]; then
  echo '{"continue": false, "message": "shared-functions.sh not found"}' | jq .
  exit 1
fi
source "${SCRIPT_DIR}/shared-functions.sh"

# Parse input
INPUT=$(cat)
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name')
CWD=$(echo "$INPUT" | jq -r '.cwd')  # Used for project-level config resolution

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
