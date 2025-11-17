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
