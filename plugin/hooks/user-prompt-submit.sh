#!/bin/bash
# user-prompt-submit.sh - Context-aware command injection
#
# Injects project commands from CLAUDE.md frontmatter based on the user's
# message content and slash command. Only injects commands that are relevant
# to the current interaction, saving tokens.

set -euo pipefail

# Get hook arguments
USER_MESSAGE="${1:-}"
COMMAND="${2:-}"

# Parse CLAUDE.md frontmatter for a specific command
get_command() {
  local cmd_type="$1"

  # Check if CLAUDE.md exists
  if [ ! -f "CLAUDE.md" ]; then
    return
  fi

  # Extract command from YAML frontmatter
  # Format: "  test: \"mise run test\""
  awk -v cmd="$cmd_type" '
    BEGIN { in_frontmatter=0; in_commands=0 }
    /^---$/ {
      in_frontmatter = !in_frontmatter
      next
    }
    in_frontmatter && /^commands:/ {
      in_commands=1
      next
    }
    in_frontmatter && in_commands && /^[^ ]/ {
      in_commands=0
    }
    in_frontmatter && in_commands && $0 ~ "^  " cmd ":" {
      # Extract value between quotes
      gsub(/.*: "/, "")
      gsub(/".*/, "")
      print
      exit
    }
  ' CLAUDE.md
}

# Detect which commands are needed based on canonical phrases
detect_needed_commands() {
  local needs=""

  if [ -z "$USER_MESSAGE" ]; then
    return
  fi

  # Detect canonical command phrases
  # Pattern: "Run project <type> command"

  if echo "$USER_MESSAGE" | grep -qiE "run project test command"; then
    needs="$needs test"
  fi

  if echo "$USER_MESSAGE" | grep -qiE "run project check command"; then
    needs="$needs check"
  fi

  if echo "$USER_MESSAGE" | grep -qiE "run project build command"; then
    needs="$needs build"
  fi

  if echo "$USER_MESSAGE" | grep -qiE "run project run command"; then
    needs="$needs run"
  fi

  # Remove duplicates and trim
  echo "$needs" | tr ' ' '\n' | sort -u | tr '\n' ' ' | sed 's/ $//'
}

# Inject commands as XML additional context
inject_commands() {
  local needed="$1"

  if [ -z "$needed" ]; then
    return
  fi

  echo "<project_commands>"
  echo "Commands from CLAUDE.md frontmatter for this project:"
  echo ""

  for cmd in $needed; do
    local cmd_value=$(get_command "$cmd")
    if [ -n "$cmd_value" ]; then
      echo "  <$cmd>$cmd_value</$cmd>"
    fi
  done

  echo "</project_commands>"
}

# Main execution
commands_needed=$(detect_needed_commands)

# Only inject if commands are needed
if [ -n "$commands_needed" ]; then
  additional_context=$(inject_commands "$commands_needed")

  # Output as JSON for Claude Code
  jq -n --arg content "$additional_context" '{
    additionalContext: $content
  }'
fi
