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

# Run find-practices to show all available practices
find_practices_output=$("${CLAUDE_PLUGIN_ROOT}/tools/find-practices" 2>&1 || echo "Error running find-practices")

# Build the additionalContext content
additional_context="<EXTREMELY_IMPORTANT>

**CipherPowers Skills:**

**The content below is from skills/selecting-agents/SKILL.md:**

${selecting_agents_content}

---

**The content below is from skills/collaboration/emergency-stop/SKILL.md:**

${emergency_stop_content}

---

**Available practices (output of find-practices):**

${find_practices_output}

**Tool paths:**
- find-practices: ${CLAUDE_PLUGIN_ROOT}/tools/find-practices

</EXTREMELY_IMPORTANT>"

# Output as JSON using jq to properly escape
jq -n --arg context "$additional_context" '{
  hookSpecificOutput: {
    hookEventName: "SessionStart",
    additionalContext: $context
  }
}'

exit 0
