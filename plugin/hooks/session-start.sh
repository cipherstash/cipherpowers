#!/usr/bin/env bash
# SessionStart hook for cipherpowers plugin

set -euo pipefail

# Determine plugin root
# Use CLAUDE_PLUGIN_ROOT if set, otherwise compute from script location
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLAUDE_PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-$(cd "$SCRIPT_DIR/.." && pwd)}"

# Read skill files
selecting_agents_content=$(cat "${CLAUDE_PLUGIN_ROOT}/skills/selecting-agents/SKILL.md" 2>&1 || echo "Error reading selecting-agents")
emergency_stop_content=$(cat "${CLAUDE_PLUGIN_ROOT}/skills/collaboration/emergency-stop/SKILL.md" 2>&1 || echo "Error reading emergency-stop")


# Build the additionalContext content
additional_context="<EXTREMELY_IMPORTANT>

**CipherPowers Skills:**

**The content below is from skills/selecting-agents/SKILL.md:**

${selecting_agents_content}




</EXTREMELY_IMPORTANT>"

# Output as JSON using jq to properly escape
jq -n --arg context "$additional_context" '{
  hookSpecificOutput: {
    hookEventName: "SessionStart",
    additionalContext: $context
  }
}'

exit 0
