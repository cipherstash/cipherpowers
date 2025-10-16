#!/usr/bin/env bash
# SessionStart hook for cipherpowers plugin

set -euo pipefail

# Determine plugin root
# Use CLAUDE_PLUGIN_ROOT if set, otherwise compute from script location
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLAUDE_PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-$(cd "$SCRIPT_DIR/.." && pwd)}"

# Read skill files
using_skills_content=$(cat "${CLAUDE_PLUGIN_ROOT}/skills/using-skills/SKILL.md" 2>&1 || echo "Error reading using-skills")
selecting_agents_content=$(cat "${CLAUDE_PLUGIN_ROOT}/skills/selecting-agents/SKILL.md" 2>&1 || echo "Error reading selecting-agents")

# Run find-practices to show all available practices
find_practices_output=$("${CLAUDE_PLUGIN_ROOT}/tools/find-practices" 2>&1 || echo "Error running find-practices")

# Escape outputs for JSON (replace backslashes, quotes, add newlines)
using_skills_escaped=$(echo "$using_skills_content" | sed 's/\\/\\\\/g' | sed 's/"/\\"/g' | awk '{printf "%s\\n", $0}')
selecting_agents_escaped=$(echo "$selecting_agents_content" | sed 's/\\/\\\\/g' | sed 's/"/\\"/g' | awk '{printf "%s\\n", $0}')
find_practices_escaped=$(echo "$find_practices_output" | sed 's/\\/\\\\/g' | sed 's/"/\\"/g' | awk '{printf "%s\\n", $0}')

# Output context injection as JSON
cat <<EOF
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "<EXTREMELY_IMPORTANT>\n\n**CipherPowers Skills:**\n\n**The content below is from skills/using-skills/SKILL.md:**\n\n${using_skills_escaped}\n\n---\n\n**The content below is from skills/selecting-agents/SKILL.md:**\n\n${selecting_agents_escaped}\n\n---\n\n**Available practices (output of find-practices):**\n\n${find_practices_escaped}\n\n**Tool paths:**\n- find-skills: ${CLAUDE_PLUGIN_ROOT}/tools/find-skills\n- find-practices: ${CLAUDE_PLUGIN_ROOT}/tools/find-practices\n\n</EXTREMELY_IMPORTANT>"
  }
}
EOF

exit 0
