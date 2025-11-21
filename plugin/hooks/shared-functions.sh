#!/usr/bin/env bash
set -euo pipefail

# Logging configuration
LOG_FILE="${TMPDIR:-/tmp}/cipherpowers-hooks-$(date +%Y%m%d).log"
LOG_ENABLED="${CIPHERPOWERS_HOOK_DEBUG:-true}"

# Log helper - writes to stderr and log file
log_debug() {
  if [ "$LOG_ENABLED" = "true" ]; then
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local message="[$timestamp] $*"
    echo "$message" >&2
    echo "$message" >> "$LOG_FILE"
  fi
}

# Check if gate exists in configuration
gate_exists() {
  local gate_name="$1"
  local config="$2"

  local exists=$(jq -r ".gates | has(\"$gate_name\")" "$config")
  [ "$exists" = "true" ]
}

# Execute a gate and handle its result
# Returns: exit code (0 = continue, 1 = stop execution)
run_gate() {
  local gate_name="$1"
  local config="$2"

  log_debug "run_gate: Starting gate '$gate_name'"

  # Check if gate is defined in gates.json
  if ! gate_exists "$gate_name" "$config"; then
    # Missing gate definition - STOP
    log_debug "run_gate: Gate '$gate_name' not found in config, stopping"
    jq -n --arg gate "$gate_name" '{
      continue: false,
      message: ("Gate '\''\($gate)'\'''' referenced but not defined in gates.json")
    }'
    return 1
  fi

  # Get gate command from gates.json
  local gate_cmd=$(jq -r ".gates[\"$gate_name\"].command // empty" "$config")

  if [ -z "$gate_cmd" ]; then
    # Missing command field - STOP
    log_debug "run_gate: Gate '$gate_name' missing command field, stopping"
    jq -n --arg gate "$gate_name" '{
      continue: false,
      message: ("Gate '\''\($gate)'\'''' is missing required '\''command'\'' field")
    }'
    return 1
  fi

  # Get gate configuration
  local on_pass=$(jq -r ".gates[\"$gate_name\"].on_pass // \"CONTINUE\"" "$config")
  local on_fail=$(jq -r ".gates[\"$gate_name\"].on_fail // \"BLOCK\"" "$config")
  local description=$(jq -r ".gates[\"$gate_name\"].description // \"\"" "$config")

  log_debug "run_gate: Running command for gate '$gate_name': $gate_cmd"
  log_debug "run_gate: on_pass=$on_pass, on_fail=$on_fail"

  # Run the gate
  local output
  local exit_code=0
  output=$(eval "$gate_cmd" 2>&1) || exit_code=$?

  log_debug "run_gate: Gate '$gate_name' exited with code $exit_code"
  log_debug "run_gate: Gate '$gate_name' output: $output"

  if [ $exit_code -eq 0 ]; then
    # Gate passed
    log_debug "run_gate: Gate '$gate_name' passed, handling action '$on_pass'"
    handle_action "$on_pass" "$gate_name" "passed" "$output" "$config"
  else
    # Gate failed
    log_debug "run_gate: Gate '$gate_name' failed, handling action '$on_fail'"
    handle_action "$on_fail" "$gate_name" "failed" "$output" "$config"
  fi
}

# Handle gate action (CONTINUE, BLOCK, STOP, or gate chaining)
# Returns: exit code (0 = continue, 1 = stop execution)
handle_action() {
  local action="$1"
  local gate_name="$2"
  local gate_status="$3"
  local output="$4"
  local config="$5"

  log_debug "handle_action: Processing action '$action' for gate '$gate_name' (status: $gate_status)"

  case "$action" in
    CONTINUE)
      # Continue to next gate in sequence
      # Uses additionalContext field per Claude Code hook spec
      log_debug "handle_action: Action CONTINUE - proceeding to next gate"
      if [ "$gate_status" = "failed" ]; then
        log_debug "handle_action: Injecting warning context for failed gate"
        jq -n --arg msg "⚠️ Gate '\''$gate_name'\'' failed but continuing:\n$output" '{
          additionalContext: $msg
        }'
      fi
      return 0
      ;;

    BLOCK)
      # Block execution, prevent subsequent gates
      # Uses decision/reason fields per Claude Code hook spec
      log_debug "handle_action: Action BLOCK - blocking agent execution"
      jq -n --arg reason "Gate '\''$gate_name'\'' $gate_status. Output:\n$output" '{
        decision: "block",
        reason: $reason
      }'
      return 1
      ;;

    STOP)
      # Stop Claude entirely
      # Uses continue/message fields per Claude Code hook spec
      log_debug "handle_action: Action STOP - stopping Claude entirely"
      jq -n --arg msg "Gate '\''$gate_name'\'' $gate_status. Stopping Claude.\n$output" '{
        continue: false,
        message: $msg
      }'
      return 1
      ;;

    *)
      # Gate chaining - run the referenced gate
      log_debug "handle_action: Attempting to chain to gate '$action'"
      if gate_exists "$action" "$config"; then
        # Chain to another gate
        log_debug "handle_action: Chaining to gate '$action'"
        run_gate "$action" "$config"
        return $?
      else
        # Referenced gate doesn't exist - STOP
        log_debug "handle_action: Chain target '$action' not found, stopping"
        jq -n --arg gate "$gate_name" --arg ref "$action" '{
          continue: false,
          message: ("Gate '\''\($gate)'\'''' references undefined gate '\''\($ref)'\''")
        }'
        return 1
      fi
      ;;
  esac
}

# Discover context file using convention-based naming
# Args: cwd, name (command/skill without prefix), stage (start/end)
# Returns: path to context file if exists, empty if not found
# Usage: result=$(discover_context_file "$cwd" "$name" "$stage")
#        [ -n "$result" ] && echo "Found: $result"
discover_context_file() {
  local cwd="$1"
  local name="$2"
  local stage="$3"

  # Try discovery paths in priority order
  local paths=(
    "${cwd}/.claude/context/${name}-${stage}.md"                    # Flat
    "${cwd}/.claude/context/slash-command/${name}-${stage}.md"      # Organized
    "${cwd}/.claude/context/slash-command/${name}/${stage}.md"      # Hierarchical
    "${cwd}/.claude/context/skill/${name}-${stage}.md"              # Skill organized
    "${cwd}/.claude/context/skill/${name}/${stage}.md"              # Skill hierarchical
  )

  for path in "${paths[@]}"; do
    if [ -f "$path" ]; then
      echo "$path"
      return 0
    fi
  done

  # Not found - return success (empty string) so set -e doesn't abort
  return 0
}

# Inject context file content into conversation
# Args: file_path
# Output: JSON with additionalContext field (to stdout)
inject_context_file() {
  local file="$1"

  if [ ! -f "$file" ]; then
    log_debug "inject_context_file: File not found: $file"
    return 1
  fi

  # Check file size (100KB = 102400 bytes, 1MB = 1048576 bytes)
  local file_size=$(wc -c < "$file" | tr -d ' ')
  if [ "$file_size" -gt 1048576 ]; then
    log_debug "inject_context_file: File too large ($file_size bytes, max 1MB): $file"
    return 1
  elif [ "$file_size" -gt 102400 ]; then
    log_debug "inject_context_file: WARNING: Large file ($file_size bytes, >100KB): $file"
  fi

  local content=$(cat "$file")

  log_debug "inject_context_file: Injecting content from $file (${#content} chars)"

  # Output as JSON additionalContext (per Claude Code hook spec)
  jq -n --arg content "$content" '{
    additionalContext: $content
  }'
}
