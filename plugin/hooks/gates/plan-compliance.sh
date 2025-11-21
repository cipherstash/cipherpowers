#!/usr/bin/env bash
set -euo pipefail

# Plan Compliance Gate
# Checks that agents provide STATUS and handles BLOCKED reports
#
# Invoked automatically by dispatcher.sh for all SubagentStop events.
# See dispatcher.sh lines 153-156 for automatic integration.
#
# This gate runs automatically on ALL SubagentStop events to ensure:
# 1. Agent provided STATUS (OK or BLOCKED)
# 2. If BLOCKED, block execution for dispatcher to handle
# 3. If OK, allow execution to continue
#
# No configuration needed - built-in gate that always runs first

# Get agent output from hook input
AGENT_OUTPUT=$(echo "$HOOK_INPUT" | jq -r '.output // ""')

# If no output, just continue (agent may not be doing plan execution)
if [ -z "$AGENT_OUTPUT" ]; then
  jq -n '{additionalContext: ""}'
  exit 0
fi

# Check if STATUS is present
if ! echo "$AGENT_OUTPUT" | grep -q "STATUS:"; then
  # No STATUS found - block execution
  jq -n '{
    decision: "block",
    reason: "Agent must provide STATUS in completion report. Use STATUS: OK when task complete or STATUS: BLOCKED when plan cannot be followed."
  }'
  exit 0
fi

# Check if STATUS: BLOCKED
if echo "$AGENT_OUTPUT" | grep -q "STATUS: BLOCKED"; then
  # Extract BLOCKED reason if possible
  REASON=$(echo "$AGENT_OUTPUT" | grep -A 10 "STATUS: BLOCKED" | grep "REASON:" | head -1 || echo "No reason provided")

  # Block execution - dispatcher will handle escalation to user
  jq -n --arg reason "$REASON" '{
    decision: "block",
    reason: ("Agent reported BLOCKED. Review required before proceeding.\n\n" + $reason)
  }'
  exit 0
fi

# STATUS: OK - continue execution
jq -n '{
  additionalContext: "Task completed successfully (STATUS: OK)"
}'
