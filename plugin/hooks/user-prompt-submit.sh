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

# Detect which commands are needed based on context
detect_needed_commands() {
  local needs=""

  # Command-specific detection (slash commands)
  case "$COMMAND" in
    /commit)
      # Commits need all quality gates
      needs="test check build"
      ;;
    /execute)
      # Plan execution needs all verification
      needs="test check build"
      ;;
    /code-review)
      # Reviews need test and check
      needs="test check"
      ;;
    /brainstorm|/plan)
      # Planning doesn't need commands
      needs=""
      ;;
  esac

  # Message content analysis (if no specific command matched)
  if [ -z "$needs" ] && [ -n "$USER_MESSAGE" ]; then
    # Check for test-related keywords
    if echo "$USER_MESSAGE" | grep -qiE "\b(test|tests|testing|spec|verify|verification)\b"; then
      needs="$needs test"
    fi

    # Check for check-related keywords
    if echo "$USER_MESSAGE" | grep -qiE "\b(lint|check|checks|format|quality|clippy|typecheck)\b"; then
      needs="$needs check"
    fi

    # Check for build-related keywords
    if echo "$USER_MESSAGE" | grep -qiE "\b(build|compile|package)\b"; then
      needs="$needs build"
    fi

    # Check for run-related keywords
    if echo "$USER_MESSAGE" | grep -qiE "\b(run|start|execute the application)\b"; then
      needs="$needs run"
    fi
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
