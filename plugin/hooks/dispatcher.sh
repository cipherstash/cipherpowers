#!/usr/bin/env bash
set -euo pipefail

# Universal hook dispatcher for all hook events
# Routes hook events to configured gates based on gates.json

# Source shared functions
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ ! -f "${SCRIPT_DIR}/shared-functions.sh" ]; then
  echo '{"continue": false, "message": "shared-functions.sh not found"}' | jq .
  exit 1
fi
source "${SCRIPT_DIR}/shared-functions.sh"

# Parse input - hook events send different fields based on type
log_debug "dispatcher: ========== HOOK EVENT START =========="
log_debug "dispatcher: About to read INPUT from stdin"

INPUT=$(cat)

log_debug "dispatcher: INPUT length: ${#INPUT}"
log_debug "dispatcher: Raw INPUT: ${INPUT:0:500}"  # First 500 chars to avoid huge logs

# Parse hook event - Claude Code uses 'hook_event_name' not 'hook_event'
HOOK_EVENT=$(echo "$INPUT" | jq -r '.hook_event_name // .hook_event // .event // .type // "unknown"')
CWD=$(echo "$INPUT" | jq -r '.cwd // .workingDirectory // .pwd // "."')

# Also log all top-level keys
KEYS=$(echo "$INPUT" | jq -r 'keys // [] | join(", ")' 2>/dev/null || echo "parse error")

log_debug "dispatcher: Hook event: $HOOK_EVENT"
log_debug "dispatcher: Working directory: $CWD"
log_debug "dispatcher: Available keys in INPUT: $KEYS"

# Extract context based on hook type
case "$HOOK_EVENT" in
  PostToolUse)
    CONTEXT_KEY="tool_name"
    CONTEXT_VALUE=$(echo "$INPUT" | jq -r '.tool_name // "unknown"')
    ENABLED_LIST_KEY="enabled_tools"
    log_debug "dispatcher: Tool: $CONTEXT_VALUE"
    ;;
  SubagentStop)
    CONTEXT_KEY="agent_name"
    CONTEXT_VALUE=$(echo "$INPUT" | jq -r '.agent_name // .subagent_name // "unknown"')
    ENABLED_LIST_KEY="enabled_agents"
    log_debug "dispatcher: Agent: $CONTEXT_VALUE"
    ;;
  UserPromptSubmit)
    CONTEXT_KEY="user_message"
    CONTEXT_VALUE=$(echo "$INPUT" | jq -r '.user_message // ""')
    ENABLED_LIST_KEY="enabled"
    log_debug "dispatcher: User message (truncated): ${CONTEXT_VALUE:0:100}..."
    ;;
  SlashCommandStart|SlashCommandEnd)
    COMMAND=$(echo "$INPUT" | jq -r '.command // ""')
    COMMAND_NAME="${COMMAND#/}"  # Remove leading /
    STAGE="${HOOK_EVENT#SlashCommand}"  # "Start" or "End"
    STAGE_LOWER=$(echo "$STAGE" | tr '[:upper:]' '[:lower:]')
    CONTEXT_FILE=$(discover_context_file "$CWD" "$COMMAND_NAME" "$STAGE_LOWER")
    CONTEXT_KEY="command"
    CONTEXT_VALUE="$COMMAND"
    ENABLED_LIST_KEY="enabled_commands"
    log_debug "dispatcher: Command: $COMMAND, Stage: $STAGE_LOWER"
    [ -n "$CONTEXT_FILE" ] && log_debug "dispatcher: Context file: $CONTEXT_FILE"
    ;;
  SkillStart|SkillEnd)
    SKILL=$(echo "$INPUT" | jq -r '.skill // ""')
    STAGE="${HOOK_EVENT#Skill}"  # "Start" or "End"
    STAGE_LOWER=$(echo "$STAGE" | tr '[:upper:]' '[:lower:]')
    CONTEXT_FILE=$(discover_context_file "$CWD" "$SKILL" "$STAGE_LOWER")
    CONTEXT_KEY="skill"
    CONTEXT_VALUE="$SKILL"
    ENABLED_LIST_KEY="enabled_skills"
    log_debug "dispatcher: Skill: $SKILL, Stage: $STAGE_LOWER"
    [ -n "$CONTEXT_FILE" ] && log_debug "dispatcher: Context file: $CONTEXT_FILE"
    ;;
  *)
    # Unknown hook event - exit cleanly
    log_debug "dispatcher: Unknown hook event '$HOOK_EVENT', exiting"
    exit 0
    ;;
esac

# Convention-based injection (if context file exists)
if [ -n "${CONTEXT_FILE:-}" ] && [ -f "$CONTEXT_FILE" ]; then
  log_debug "dispatcher: Auto-injecting context from $CONTEXT_FILE"
  inject_context_file "$CONTEXT_FILE"
fi

# Load gates config - check project directory first, then plugin default
if [ -f "${CWD}/.claude/gates.json" ]; then
  CONFIG="${CWD}/.claude/gates.json"
  log_debug "dispatcher: Using config: ${CWD}/.claude/gates.json"
elif [ -f "${CWD}/gates.json" ]; then
  CONFIG="${CWD}/gates.json"
  log_debug "dispatcher: Using config: ${CWD}/gates.json"
elif [ -f "${CLAUDE_PLUGIN_ROOT}/hooks/gates.json" ]; then
  CONFIG="${CLAUDE_PLUGIN_ROOT}/hooks/gates.json"
  log_debug "dispatcher: Using config: ${CLAUDE_PLUGIN_ROOT}/hooks/gates.json"
else
  # No config found - exit cleanly
  log_debug "dispatcher: No gates.json config found, exiting"
  exit 0
fi

# Check if hook is configured
HOOK_CONFIG=$(jq -r ".hooks.${HOOK_EVENT} // null" "$CONFIG")
if [ "$HOOK_CONFIG" = "null" ]; then
  # Hook not configured - exit cleanly
  log_debug "dispatcher: Hook '$HOOK_EVENT' not configured in gates.json, exiting"
  exit 0
fi

log_debug "dispatcher: Hook '$HOOK_EVENT' is configured"

# For PostToolUse and SubagentStop: Check if context value is in enabled list
if [ "$HOOK_EVENT" = "PostToolUse" ] || [ "$HOOK_EVENT" = "SubagentStop" ]; then
  ENABLED_LIST=$(jq -r ".hooks.${HOOK_EVENT}.${ENABLED_LIST_KEY}[]" "$CONFIG" 2>/dev/null || echo "")

  log_debug "dispatcher: Checking if '$CONTEXT_VALUE' is in enabled list"
  log_debug "dispatcher: Enabled list: $(echo $ENABLED_LIST | tr '\n' ' ')"

  if ! echo "$ENABLED_LIST" | grep -q "^${CONTEXT_VALUE}$"; then
    # Not enabled for this tool/agent - exit cleanly
    log_debug "dispatcher: '$CONTEXT_VALUE' not in enabled list, exiting"
    exit 0
  fi

  log_debug "dispatcher: '$CONTEXT_VALUE' is enabled, proceeding"
fi

# Special handling for UserPromptSubmit: Always run commands gate first
if [ "$HOOK_EVENT" = "UserPromptSubmit" ]; then
  # Export INPUT for commands.sh
  export HOOK_INPUT="$INPUT"

  log_debug "dispatcher: Running built-in commands gate for UserPromptSubmit"
  # Run built-in commands gate (context injection, always runs first)
  "${CLAUDE_PLUGIN_ROOT}/hooks/gates/commands.sh" || true

  # Then check if additional gates are configured
  ENABLED=$(jq -r ".hooks.${HOOK_EVENT}.enabled // false" "$CONFIG")
  if [ "$ENABLED" != "true" ]; then
    # No additional gates configured - exit after commands
    log_debug "dispatcher: No additional gates enabled for UserPromptSubmit, exiting"
    exit 0
  fi

  # Get additional gates to run (if any)
  GATES=$(jq -r ".hooks.${HOOK_EVENT}.gates[]" "$CONFIG" 2>/dev/null || echo "")
  log_debug "dispatcher: Additional gates for UserPromptSubmit: $GATES"
else
  # For PostToolUse and SubagentStop: Get gates to run
  GATES=$(jq -r ".hooks.${HOOK_EVENT}.gates[]" "$CONFIG")
  log_debug "dispatcher: Gates to run: $GATES"

  # Export INPUT for gates that need access to full hook context
  export HOOK_INPUT="$INPUT"
fi

# Run each gate in sequence
log_debug "dispatcher: Starting gate sequence execution"
for gate in $GATES; do
  log_debug "dispatcher: Running gate '$gate'"
  run_gate "$gate" "$CONFIG"
  GATE_EXIT_CODE=$?
  log_debug "dispatcher: Gate '$gate' returned exit code $GATE_EXIT_CODE"
  # If gate returns non-zero, stop execution (BLOCK or STOP action)
  if [ $GATE_EXIT_CODE -ne 0 ]; then
    log_debug "dispatcher: Gate '$gate' stopped execution, breaking loop"
    break
  fi
done

log_debug "dispatcher: ========== HOOK EVENT END =========="
