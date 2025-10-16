#!/usr/bin/env bash
# SessionStart hook for cipherpowers plugin

set -euo pipefail

# Debug logging to verify hook execution
echo "[DEBUG] SessionStart hook triggered at $(date)" >> /tmp/cipherpowers-hook.log

# Determine plugin root
# Use CLAUDE_PLUGIN_ROOT if set, otherwise compute from script location
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLAUDE_PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-$(cd "$SCRIPT_DIR/.." && pwd)}"

# Log the computed path
echo "[DEBUG] CLAUDE_PLUGIN_ROOT=${CLAUDE_PLUGIN_ROOT}" >> /tmp/cipherpowers-hook.log

# Read skill files
using_skills_content=$(cat "${CLAUDE_PLUGIN_ROOT}/skills/using-skills/SKILL.md" 2>&1 || echo "Error reading using-skills")
selecting_agents_content=$(cat "${CLAUDE_PLUGIN_ROOT}/skills/selecting-agents/SKILL.md" 2>&1 || echo "Error reading selecting-agents")

# Run find-practices to show all available practices
find_practices_output=$("${CLAUDE_PLUGIN_ROOT}/tools/find-practices" 2>&1 || echo "Error running find-practices")

# Output context directly (not as JSON)
cat <<'EOF'
<EXTREMELY_IMPORTANT>

**CipherPowers Skills:**

**The content below is from skills/using-skills/SKILL.md:**

EOF

echo "$using_skills_content"

cat <<'EOF'

---

**The content below is from skills/selecting-agents/SKILL.md:**

EOF

echo "$selecting_agents_content"

cat <<'EOF'

---

**Available practices (output of find-practices):**

EOF

echo "$find_practices_output"

cat <<EOF

**Tool paths:**
- find-skills: ${CLAUDE_PLUGIN_ROOT}/tools/find-skills
- find-practices: ${CLAUDE_PLUGIN_ROOT}/tools/find-practices

</EXTREMELY_IMPORTANT>
EOF

exit 0
