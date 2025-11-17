#!/usr/bin/env bash

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

  # Check if gate is defined in gates.json
  if ! gate_exists "$gate_name" "$config"; then
    # Missing gate definition - STOP
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

  # Run the gate
  local output
  local exit_code=0
  output=$(eval "$gate_cmd" 2>&1) || exit_code=$?

  if [ $exit_code -eq 0 ]; then
    # Gate passed
    handle_action "$on_pass" "$gate_name" "passed" "$output" "$config"
  else
    # Gate failed
    handle_action "$on_fail" "$gate_name" "failed" "$output" "$config"
  fi
}

# Handle gate action (CONTINUE, BLOCK, STOP, or gate chaining)
# Returns: exit code (0 = continue, 1 = stop execution)
handle_action() {
  local action="$1"
  local gate_name="$2"
  local status="$3"
  local output="$4"
  local config="$5"

  case "$action" in
    CONTINUE)
      # Continue to next gate in sequence
      if [ "$status" = "failed" ]; then
        jq -n --arg msg "⚠️ Gate '\''$gate_name'\'' failed but continuing:\n$output" '{
          additionalContext: $msg
        }'
      fi
      return 0
      ;;

    BLOCK)
      # Block execution, prevent subsequent gates
      jq -n --arg reason "Gate '\''$gate_name'\'' $status. Output:\n$output" '{
        decision: "block",
        reason: $reason
      }'
      return 1
      ;;

    STOP)
      # Stop Claude entirely
      jq -n --arg msg "Gate '\''$gate_name'\'' $status. Stopping Claude.\n$output" '{
        continue: false,
        message: $msg
      }'
      return 1
      ;;

    *)
      # Gate chaining - run the referenced gate
      if gate_exists "$action" "$config"; then
        # Chain to another gate
        run_gate "$action" "$config"
        return $?
      else
        # Referenced gate doesn't exist - STOP
        jq -n --arg gate "$gate_name" --arg ref "$action" '{
          continue: false,
          message: ("Gate '\''\($gate)'\'''' references undefined gate '\''\($ref)'\''")
        }'
        return 1
      fi
      ;;
  esac
}
