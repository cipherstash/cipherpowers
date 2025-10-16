# SessionStart Hook Refactoring Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Replace individual markdown file commands in hooks.json with a unified session-start.sh script that outputs JSON context

**Architecture:** Create a bash script following the superpowers pattern that reads skill files, calls find-practices, and outputs formatted JSON to inject context at session start

**Tech Stack:** Bash, JSON, Claude Code hooks API

---

## Task 1: Create session-start.sh script

**Files:**
- Create: `plugin/hooks/session-start.sh`

**Step 1: Write the script header and setup**

Create the file with shebang, error handling, and environment variable setup:

```bash
#!/usr/bin/env bash
# SessionStart hook for cipherpowers plugin

set -euo pipefail

# CLAUDE_PLUGIN_ROOT is already set by the plugin loader
# Use it to reference all plugin paths
```

**Step 2: Add skill file reading section**

Read both skill files with error handling:

```bash
# Read skill files
using_skills_content=$(cat "${CLAUDE_PLUGIN_ROOT}skills/using-skills/SKILL.md" 2>&1 || echo "Error reading using-skills")
selecting_agents_content=$(cat "${CLAUDE_PLUGIN_ROOT}skills/selecting-agents/SKILL.md" 2>&1 || echo "Error reading selecting-agents")
```

**Step 3: Add find-practices call**

Call the find-practices tool to list available practices:

```bash
# Run find-practices to show all available practices
find_practices_output=$("${CLAUDE_PLUGIN_ROOT}tools/find-practices" 2>&1 || echo "Error running find-practices")
```

**Step 4: Add JSON escaping**

Escape all content for JSON output:

```bash
# Escape outputs for JSON (replace backslashes, quotes, add newlines)
using_skills_escaped=$(echo "$using_skills_content" | sed 's/\\/\\\\/g' | sed 's/"/\\"/g' | awk '{printf "%s\\n", $0}')
selecting_agents_escaped=$(echo "$selecting_agents_content" | sed 's/\\/\\\\/g' | sed 's/"/\\"/g' | awk '{printf "%s\\n", $0}')
find_practices_escaped=$(echo "$find_practices_output" | sed 's/\\/\\\\/g' | sed 's/"/\\"/g' | awk '{printf "%s\\n", $0}')
```

**Step 5: Write JSON output section**

Output the formatted JSON with all context:

```bash
# Output context injection as JSON
cat <<EOF
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "<EXTREMELY_IMPORTANT>\n\n**CipherPowers Skills:**\n\n**The content below is from skills/using-skills/SKILL.md:**\n\n${using_skills_escaped}\n\n---\n\n**The content below is from skills/selecting-agents/SKILL.md:**\n\n${selecting_agents_escaped}\n\n---\n\n**Available practices (output of find-practices):**\n\n${find_practices_escaped}\n\n**Tool paths:**\n- find-skills: ${CLAUDE_PLUGIN_ROOT}tools/find-skills\n- find-practices: ${CLAUDE_PLUGIN_ROOT}tools/find-practices\n\n</EXTREMELY_IMPORTANT>"
  }
}
EOF

exit 0
```

**Step 6: Make script executable**

Run: `chmod +x plugin/hooks/session-start.sh`

Expected: File permissions updated to executable

**Step 7: Test script manually**

Run: `./plugin/hooks/session-start.sh`

Expected: Valid JSON output with all three content sections escaped properly

**Step 8: Commit script creation**

```bash
git add plugin/hooks/session-start.sh
git commit -m "feat(hooks): create session-start.sh script for unified context injection

Replace individual skill file commands with single script that:
- Reads using-skills and selecting-agents skill content
- Calls find-practices to list available practices
- Outputs JSON-formatted context for SessionStart hook

Follows superpowers session-start.sh pattern with simplified structure."
```

---

## Task 2: Update hooks.json configuration

**Files:**
- Modify: `plugin/hooks/hooks.json:1-19`

**Step 1: Replace dual commands with single script reference**

Replace the hooks array containing two commands with single script command:

```json
{
    "hooks": {
        "SessionStart": [
            {
                "matcher": "startup|resume|clear|compact",
                "hooks": [
                    {
                        "type": "command",
                        "command": "${CLAUDE_PLUGIN_ROOT}hooks/session-start.sh"
                    }
                ]
            }
        ]
    }
}
```

**Step 2: Test hook in new Claude session**

Run: Start a new Claude Code session or `/clear`

Expected: SessionStart hook executes without "Permission denied" errors

Expected: Context shows both skill contents and practices list

**Step 3: Verify JSON parsing**

Check that Claude receives the context properly formatted:
- using-skills content appears
- selecting-agents content appears
- find-practices output appears
- No JSON parse errors in hook output

**Step 4: Commit hooks.json update**

```bash
git add plugin/hooks/hooks.json
git commit -m "refactor(hooks): use session-start.sh script in SessionStart hook

Replace individual skill file commands with unified script reference.
Fixes 'Permission denied' errors from trying to execute .md files.

The script handles all context injection: skills + practices discovery."
```

---

## Task 3: Verify complete integration

**Step 1: Test fresh session startup**

Run: `/clear` to trigger SessionStart hook

Expected: No errors in hook output

Expected: Skills content and practices list appear in context

**Step 2: Verify find-practices output format**

Check that practices list is properly formatted with:
- Practice names
- Descriptions from YAML frontmatter
- Paths or usage hints

**Step 3: Test with actual practice discovery**

Ask Claude: "What practices are available?"

Expected: Claude can see and list practices from the hook context

**Step 4: Final commit if any fixes needed**

If adjustments were required, commit them with descriptive message

---

## Remember

- Use exact file paths: `plugin/hooks/session-start.sh`, `plugin/hooks/hooks.json`
- Script must be executable (`chmod +x`)
- JSON escaping is critical (backslashes, quotes, newlines)
- Test in actual Claude session, not just script execution
- Follow superpowers pattern but keep it simpler (no git operations, just content loading)
- The `${CLAUDE_PLUGIN_ROOT}` variable is already set by plugin loader
